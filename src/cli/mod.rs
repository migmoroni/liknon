//! CLI parsing, command dispatch, cancellation, and exit-code translation.

mod arguments;
mod help;
mod inspection;
mod knowledge;

use arguments::{
    requested_palette, requested_presentation, Cli, ColorPaletteArgument, Command, ConfigCommand,
    Contract, ExplainTarget, Format, KnowledgeCommand, PresentationArgument,
};

use crate::{
    config::{self, ValidatedConfig},
    contracts::{
        config::Config,
        init::{InitResult, InitStatus},
        report::ValidationReport,
    },
    error::ValidatorError,
    execution, initialization,
    planning::{self, ValidationPlan},
    reporting,
    theme::{self, PaletteProfile, PresentationProfile},
};
use clap::{error::ErrorKind, CommandFactory, FromArgMatches, ValueEnum};
use schemars::schema_for;
use std::{
    fmt::Write as _,
    io::Write,
    process::ExitCode,
    sync::{atomic::AtomicBool, Arc},
};

/// Parses process arguments, executes the requested command, and returns its exit status.
///
/// This entry point reads the current process arguments and working directory,
/// writes command output to standard output or standard error, and installs a
/// cooperative `Ctrl+C` handler for validation commands. Returned codes follow
/// the stable CLI contract documented in the crate README: `0` for success,
/// `1` for a failed result, `2` for a blocked or skipped result, `3` for usage
/// or configuration errors, `4` for internal failures, and `130` for an
/// interrupted validation.
pub fn run_cli() -> ExitCode {
    let arguments = std::env::args_os().collect::<Vec<_>>();
    let palette = requested_palette(&arguments);
    let presentation = requested_presentation(&arguments);
    let command = theme::clap::apply(help::root::configure(Cli::command()), palette, presentation);
    if help::root::requested(&arguments) {
        let mut stdout = std::io::stdout().lock();
        let rendered = help::root::render(command, presentation);
        return match write_output(&mut stdout, rendered.trim_end()) {
            Ok(()) => ExitCode::from(0),
            Err((error, code)) => {
                write_stderr(&format!("workspace-validator: {error}"));
                ExitCode::from(code)
            }
        };
    }
    if color_value_without_equals(&arguments) {
        write_stderr("error: named --color palettes require --color=<PALETTE>");
        return ExitCode::from(3);
    }
    if presentation_value_without_equals(&arguments) {
        write_stderr("error: named presentation modes require --presentation=<MODE>");
        return ExitCode::from(3);
    }
    let cli = match command.try_get_matches_from(arguments) {
        Ok(matches) => match Cli::from_arg_matches(&matches) {
            Ok(value) => value,
            Err(error) => {
                write_stderr(&format!("workspace-validator: {error}"));
                return ExitCode::from(4);
            }
        },
        Err(error) => {
            let kind = error.kind();
            if kind == ErrorKind::DisplayHelp {
                let styled = error.render();
                let rendered = theme::clap::present_help(styled.ansi().to_string(), presentation);
                let mut stdout = std::io::stdout().lock();
                return match write_output(&mut stdout, rendered.trim_end()) {
                    Ok(()) => ExitCode::from(0),
                    Err((error, code)) => {
                        write_stderr(&format!("workspace-validator: {error}"));
                        ExitCode::from(code)
                    }
                };
            }
            let code = if kind == ErrorKind::DisplayVersion {
                0
            } else {
                3
            };
            let _ = error.print();
            return ExitCode::from(code);
        }
    };
    let mut stdout = std::io::stdout().lock();
    let mut stderr = std::io::stderr().lock();
    match execute(cli, &mut stdout, &mut stderr) {
        Ok(code) => ExitCode::from(code as u8),
        Err((error, code)) => {
            let _ = writeln!(stderr, "workspace-validator: {error}");
            let _ = stderr.flush();
            ExitCode::from(code)
        }
    }
}

fn execute(
    cli: Cli,
    stdout: &mut impl Write,
    stderr: &mut impl Write,
) -> Result<i32, (ValidatorError, u8)> {
    let color = cli.color;
    let presentation = cli.presentation;
    let supports_visual_output = matches!(
        &cli.command,
        Command::Validate { .. } | Command::Check { .. }
    );
    if !supports_visual_output && (color.is_some() || presentation.is_some()) {
        return Err((
            ValidatorError::Usage(
                "--color and --presentation apply to command execution only for validate and check; combine them with -h or --help to style help"
                    .into(),
            ),
            3,
        ));
    }
    if let Command::Schema { contract } = cli.command {
        let schema = match contract {
            Contract::Config => schema_for!(Config),
            Contract::Report => schema_for!(ValidationReport),
        };
        let rendered = serde_json::to_string_pretty(&schema)
            .map_err(|error| (ValidatorError::Internal(error.to_string()), 4))?;
        write_output(stdout, &rendered)?;
        return Ok(0);
    }
    if let Command::Knowledge { command } = &cli.command {
        let result = match command {
            KnowledgeCommand::Catalog { format } => {
                knowledge::catalog(matches!(format, Format::Json), stdout)
            }
            KnowledgeCommand::Show { document_id } => knowledge::show(document_id, stdout),
        };
        return match result {
            Ok(()) => Ok(0),
            Err(knowledge::KnowledgeError::Selection(details)) => {
                Err((ValidatorError::Usage(details), 3))
            }
            Err(knowledge::KnowledgeError::Internal(details)) => {
                Err((ValidatorError::Internal(details), 4))
            }
        };
    }
    let visual_json = match &cli.command {
        Command::Validate {
            format: Format::Json,
            ..
        }
        | Command::Check {
            format: Format::Json,
            ..
        } => color.is_some() || presentation.is_some(),
        _ => false,
    };
    if visual_json {
        return Err((
            ValidatorError::Usage(
                "--color and --presentation cannot be combined with --format=json".into(),
            ),
            3,
        ));
    }
    let current = std::env::current_dir()
        .map_err(|error| (ValidatorError::Internal(error.to_string()), 4))?;
    if let Command::Init {
        config: candidate,
        workspace,
        format,
    } = &cli.command
    {
        let candidate = candidate.as_deref().ok_or_else(|| {
            (
                ValidatorError::Usage("init requires at least one provisioning flag".into()),
                3,
            )
        })?;
        let workspace =
            initialization::resolve_workspace(workspace.clone().unwrap_or_else(|| current.clone()))
                .map_err(|error| (ValidatorError::Usage(error), 3))?;
        let (result, failure_class) =
            initialization::provision_config_classified(&workspace, candidate);
        render_init(&result, *format, stdout)?;
        return Ok(match result.status {
            InitStatus::Success => 0,
            InitStatus::Conflict | InitStatus::Partial => 1,
            InitStatus::Failed => match failure_class {
                initialization::InitFailureClass::Internal => 4,
                initialization::InitFailureClass::Rejected
                | initialization::InitFailureClass::None => 3,
            },
        });
    }
    let explicit = match &cli.command {
        Command::Init { .. } => unreachable!(),
        Command::Validate { config, .. }
        | Command::Check { config, .. }
        | Command::List { config, .. } => config.as_deref(),
        Command::Config {
            command: ConfigCommand::Validate { config },
        } => config.as_deref(),
        Command::Explain { target } => match target {
            ExplainTarget::Group { config, .. }
            | ExplainTarget::Suite { config, .. }
            | ExplainTarget::Check { config, .. } => config.as_deref(),
        },
        Command::Schema { .. } => unreachable!(),
        Command::Knowledge { .. } => unreachable!(),
    };
    let validated = config::load(explicit, &current).map_err(|error| (error, 3))?;
    match cli.command {
        Command::Config { .. } => {
            inspection::config_validate(&validated, stdout)
                .map_err(|error| (ValidatorError::Internal(error), 4))?;
            Ok(0)
        }
        Command::List { tree, .. } => {
            inspection::list(&validated, tree, stdout)
                .map_err(|error| (ValidatorError::Internal(error), 4))?;
            Ok(0)
        }
        Command::Explain { target } => {
            match target {
                ExplainTarget::Group { group_id, .. } => {
                    inspection::explain_group(&validated, &group_id, stdout)
                        .map_err(|error| classify_explain_error(&validated, error))?
                }
                ExplainTarget::Suite { suite_id, .. } => {
                    inspection::explain_suite(&validated, &suite_id, stdout)
                        .map_err(|error| classify_explain_error(&validated, error))?
                }
                ExplainTarget::Check { check_id, .. } => {
                    inspection::explain_check(&validated, &check_id, stdout)
                        .map_err(|error| classify_explain_error(&validated, error))?
                }
            };
            Ok(0)
        }
        Command::Validate { target, format, .. } => {
            let plan = planning::target(&validated, target.as_deref()).map_err(|details| {
                (
                    ValidatorError::invalid(validated.configuration_path(), details.to_string()),
                    3,
                )
            })?;
            execute_run(
                &validated,
                &plan,
                format,
                color,
                presentation,
                stdout,
                stderr,
            )
        }
        Command::Check {
            check_id, format, ..
        } => {
            let plan = planning::check(&validated, &check_id).map_err(|details| {
                (
                    ValidatorError::invalid(validated.configuration_path(), details.to_string()),
                    3,
                )
            })?;
            execute_run(
                &validated,
                &plan,
                format,
                color,
                presentation,
                stdout,
                stderr,
            )
        }
        Command::Schema { .. } => unreachable!(),
        Command::Knowledge { .. } => unreachable!(),
        Command::Init { .. } => unreachable!(),
    }
}

fn classify_explain_error(
    validated: &ValidatedConfig,
    error: inspection::ExplainError,
) -> (ValidatorError, u8) {
    match error {
        inspection::ExplainError::Selection(details) => (
            ValidatorError::invalid(validated.configuration_path(), details),
            3,
        ),
        inspection::ExplainError::Output(details) => (ValidatorError::Internal(details), 4),
    }
}

fn render_init(
    result: &InitResult,
    format: Format,
    output: &mut impl Write,
) -> Result<(), (ValidatorError, u8)> {
    let rendered = match format {
        Format::Json => serde_json::to_string_pretty(result)
            .map_err(|error| (ValidatorError::Internal(error.to_string()), 4))?,
        Format::Human => {
            let mut rendered = String::new();
            let _ = writeln!(rendered, "Initialization: {:?}", result.status);
            let _ = writeln!(rendered, "Workspace: {}", result.workspace_root);
            for resource in &result.resources {
                let _ = writeln!(
                    rendered,
                    "  {:?} {:?}: {}",
                    resource.kind, resource.status, resource.path
                );
                if let Some(digest) = &resource.digest {
                    let _ = writeln!(rendered, "    SHA-256: {digest}");
                }
                for diagnostic in &resource.diagnostics {
                    let _ = writeln!(rendered, "    {diagnostic}");
                }
            }
            rendered.pop();
            rendered
        }
    };
    write_output(output, &rendered)
}

fn execute_run(
    validated: &ValidatedConfig,
    plan: &ValidationPlan,
    format: Format,
    color: Option<ColorPaletteArgument>,
    presentation: Option<PresentationArgument>,
    output: &mut impl Write,
    error_output: &mut impl Write,
) -> Result<i32, (ValidatorError, u8)> {
    let cancelled = Arc::new(AtomicBool::new(false));
    let signal = cancelled.clone();
    // The signal handler only flips an atomic flag; process-group termination
    // remains owned by the executor where child lifecycle is available.
    ctrlc::set_handler(move || signal.store(true, std::sync::atomic::Ordering::SeqCst)).map_err(
        |error| {
            (
                ValidatorError::Internal(format!("cannot install signal handler: {error}")),
                4,
            )
        },
    )?;
    let (outcome, theme, separate_result) = match format {
        Format::Human => {
            let theme = theme::Theme::resolve(
                color
                    .map(PaletteProfile::from)
                    .unwrap_or(PaletteProfile::Plain),
                presentation
                    .map(PresentationProfile::from)
                    .unwrap_or(PresentationProfile::Standard),
            );
            let mut reporter = reporting::execution::TerminalExecutionReporter::new(theme.clone());
            let separate_result = reporter.is_visible();
            let outcome = execution::run_with_progress(validated, plan, cancelled, &mut reporter);
            // Drop indicatif before writing the lifecycle separator so no
            // pending redraw can consume or move the blank terminal row.
            drop(reporter);
            (outcome, Some(theme), separate_result)
        }
        Format::Json => (execution::run(validated, plan, cancelled), None, false),
    };
    let rendered = match format {
        Format::Human => {
            reporting::result::human::render(&outcome.report, theme.as_ref().expect("human theme"))
        }
        Format::Json => reporting::result::json::render(&outcome.report)
            .map_err(|error| (ValidatorError::Internal(error.to_string()), 4))?,
    };
    if separate_result {
        // Indicatif leaves the cursor at the end of its completed footer. The
        // first newline closes that row; the second creates the visible gap.
        let _ = writeln!(error_output, "\n");
        let _ = error_output.flush();
    }
    write_output(output, &rendered)?;
    Ok(if outcome.interrupted {
        130
    } else {
        outcome.report.summary.exit_code()
    })
}

fn write_output(output: &mut impl Write, rendered: &str) -> Result<(), (ValidatorError, u8)> {
    writeln!(output, "{rendered}")
        .and_then(|()| output.flush())
        .map_err(|error| {
            (
                ValidatorError::Internal(format!("cannot write standard output: {error}")),
                4,
            )
        })
}

fn write_stderr(message: &str) {
    let mut stderr = std::io::stderr().lock();
    let _ = writeln!(stderr, "{message}");
    let _ = stderr.flush();
}

fn color_value_without_equals(arguments: &[std::ffi::OsString]) -> bool {
    arguments.windows(2).any(|pair| {
        pair[0] == "--color"
            && pair[1]
                .to_str()
                .is_some_and(|value| ColorPaletteArgument::from_str(value, true).is_ok())
    })
}

fn presentation_value_without_equals(arguments: &[std::ffi::OsString]) -> bool {
    arguments.windows(2).any(|pair| {
        pair[0] == "--presentation"
            && pair[1]
                .to_str()
                .is_some_and(|value| PresentationArgument::from_str(value, true).is_ok())
    })
}
