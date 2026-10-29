//! Authenticated acquisition and transactional state management for Liknon resources.

#![warn(missing_docs)]

mod cli;
mod error;
mod hub;
mod operations;
mod output;
mod resolver;
mod store;
mod theme;
mod workspace;

use std::{io::Write as _, process::ExitCode};

use clap::{error::ErrorKind, CommandFactory as _, FromArgMatches as _};
use liknon_cli_ui::{
    clap as clap_theme,
    help::{self as shared_help, RootHelp},
    terminal::{self, OutputKind},
    Theme, VisualSelection,
};

const HELP: &str = "Print help at the root or after any command";
const HELP_LONG: &str = "Prints help at the root or after any command.\n\nThe -h flag prints a compact summary; --help prints complete details.";

/// Parses process arguments using the suite version, executes one Manager
/// operation, and returns its stable exit status.
pub fn run_cli_with_version(version: &'static str) -> ExitCode {
    let arguments = std::env::args_os().collect::<Vec<_>>();
    let selection = VisualSelection::from_args(&arguments);
    let command = clap_theme::apply(
        shared_help::disable_help_subcommands(cli::configure(cli::Cli::command().version(version))),
        selection.palette(),
        selection.presentation(),
    );

    if shared_help::is_complete_root_request(&arguments) {
        let rendered = shared_help::render_complete_root(
            command,
            selection.palette(),
            selection.presentation(),
            RootHelp::liknon(HELP, HELP_LONG),
        );
        return write_help(rendered);
    }

    let cli = match command.try_get_matches_from(arguments) {
        Ok(matches) => match cli::Cli::from_arg_matches(&matches) {
            Ok(cli) => cli,
            Err(error) => {
                eprintln!("liknon-manager: {error}");
                return ExitCode::from(5);
            }
        },
        Err(error) => {
            let kind = error.kind();
            if matches!(
                kind,
                ErrorKind::DisplayHelp | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
            ) {
                let rendered = clap_theme::present_help(
                    error.render().ansi().to_string(),
                    selection.presentation(),
                );
                return write_help(rendered);
            }
            let code = if kind == ErrorKind::DisplayVersion {
                0
            } else {
                2
            };
            let _ = error.print();
            return ExitCode::from(code);
        }
    };

    run(cli)
}

fn run(cli: cli::Cli) -> ExitCode {
    let format = cli.format();
    let visual = cli.visual().selection();
    let machine_output =
        format == cli::OutputFormat::Json || matches!(&cli.command, cli::Command::Schema { .. });
    if !terminal::visual_options_are_compatible(
        visual,
        if machine_output {
            OutputKind::Machine
        } else {
            OutputKind::Human
        },
    ) {
        let error = error::ManagerError::Usage(
            "--color and --presentation cannot be combined with machine-readable output".to_owned(),
        );
        output::write_error(&error, format, &Theme::plain());
        return ExitCode::from(error.exit_code());
    }
    let theme = Theme::resolve(visual.palette(), visual.presentation());
    match operations::execute(cli) {
        Ok(report) => {
            if let Err(error) = output::write_report(&report, format, &theme) {
                eprintln!("error: {error}");
                return ExitCode::from(error.exit_code());
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            output::write_error(&error, format, &theme);
            ExitCode::from(error.exit_code())
        }
    }
}

fn write_help(rendered: String) -> ExitCode {
    let mut stdout = std::io::stdout().lock();
    if writeln!(stdout, "{}", rendered.trim_end())
        .and_then(|()| stdout.flush())
        .is_ok()
    {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(5)
    }
}

#[cfg(test)]
mod tests {
    use super::cli::Cli;
    use clap::{error::ErrorKind, Parser};

    #[test]
    fn exposes_complete_manager_grammar() {
        assert_eq!(
            Cli::try_parse_from(["liknon-manager"]).unwrap_err().kind(),
            ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        );
        for command in [
            "init",
            "search",
            "info",
            "install",
            "update",
            "remove",
            "list",
            "verify",
            "materialize",
            "schema",
        ] {
            let result = Cli::try_parse_from(["liknon-manager", command, "--help"]);
            assert_eq!(
                result.unwrap_err().kind(),
                ErrorKind::DisplayHelp,
                "{command}"
            );
        }
    }
}
