//! Capability-free launcher for the complete Liknon CLI suite.

use std::{
    ffi::{OsStr, OsString},
    io::Write as _,
    path::{Path, PathBuf},
    process::{Command, ExitCode, ExitStatus, Stdio},
};

use clap::{CommandFactory as _, Parser, Subcommand};
use liknon_cli_ui::{
    clap as clap_theme,
    help::{self as shared_help, RootHelp},
    RootVisualOptions, VisualSelection,
};

const HELP: &str = "Print help for the suite launcher";
const HELP_LONG: &str = "Prints help for the suite launcher.\n\nThe -h flag prints a compact summary; --help prints complete launcher details. Capability-specific help belongs to liknon-validator, liknon-manager, and liknon-tool.";
const SUITE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Parser)]
#[command(
    name = "liknon",
    disable_version_flag = true,
    about = "Routes one explicit Liknon capability namespace",
    arg_required_else_help = true,
    disable_help_subcommand = true
)]
struct Cli {
    #[command(flatten)]
    _visual: RootVisualOptions,
    #[command(subcommand)]
    _namespace: Namespace,
}

#[derive(Debug, Subcommand)]
enum Namespace {
    /// Routes arguments to liknon-validator.
    #[command(visible_alias = "v")]
    Validator,
    /// Routes arguments to liknon-manager.
    #[command(visible_alias = "m")]
    Manager,
    /// Routes arguments to liknon-tool.
    #[command(visible_alias = "t")]
    Tool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Capability {
    Validator,
    Manager,
    Tool,
}

impl Capability {
    const ALL: [Self; 3] = [Self::Validator, Self::Manager, Self::Tool];

    const fn executable(self) -> &'static str {
        match self {
            Self::Validator => "liknon-validator",
            Self::Manager => "liknon-manager",
            Self::Tool => "liknon-tool",
        }
    }

    fn from_namespace(value: &OsStr) -> Option<Self> {
        match value.to_str()? {
            "validator" | "v" => Some(Self::Validator),
            "manager" | "m" => Some(Self::Manager),
            "tool" | "t" => Some(Self::Tool),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Route {
    capability: Capability,
    arguments: Vec<OsString>,
}

/// Parses launcher arguments, routes one capability process, and returns only
/// for launcher-owned help, version, or deterministic failures.
pub fn run_launcher() -> ExitCode {
    let arguments = std::env::args_os().collect::<Vec<_>>();
    let selection = VisualSelection::from_root_args(&arguments);

    if shared_help::is_complete_root_request(&arguments) {
        return write_stdout(shared_help::render_complete_root(
            launcher_command(selection),
            selection.palette(),
            selection.presentation(),
            RootHelp::liknon(HELP, HELP_LONG),
        ));
    }

    match parse_root_request(&arguments) {
        RootRequest::ShortHelp => write_short_help(selection),
        RootRequest::Version => write_version_report(),
        RootRequest::Route(route) => dispatch(route),
        RootRequest::Error(message) => write_usage_error(&message),
    }
}

enum RootRequest {
    ShortHelp,
    Version,
    Route(Route),
    Error(String),
}

fn parse_root_request(arguments: &[OsString]) -> RootRequest {
    let mut visual = false;
    let mut index = 1;

    while let Some(argument) = arguments.get(index) {
        let Some(value) = argument.to_str() else {
            return RootRequest::Error("the capability namespace must be valid UTF-8".to_owned());
        };
        match value {
            "-h" | "--help" => {
                return if index + 1 == arguments.len() {
                    RootRequest::ShortHelp
                } else {
                    RootRequest::Error("root help does not accept trailing arguments".to_owned())
                };
            }
            "-V" | "--version" => {
                return if index + 1 == arguments.len() && !visual {
                    RootRequest::Version
                } else {
                    RootRequest::Error(
                        "root version does not accept visual or trailing arguments".to_owned(),
                    )
                };
            }
            "--color" => {
                visual = true;
                index += 1;
            }
            value if value.starts_with("--color=") => {
                visual = true;
                index += 1;
            }
            value if value.starts_with("--presentation=") => {
                visual = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return RootRequest::Error(format!("unknown launcher option `{value}`"));
            }
            _ => {
                let Some(capability) = Capability::from_namespace(argument) else {
                    return RootRequest::Error(format!("unknown capability namespace `{value}`"));
                };
                if visual {
                    return RootRequest::Error(
                        "--color and --presentation style launcher help only; place them after the capability namespace to style capability output"
                            .to_owned(),
                    );
                }
                return RootRequest::Route(Route {
                    capability,
                    arguments: arguments[index + 1..].to_vec(),
                });
            }
        }
    }

    RootRequest::Error("a capability namespace is required".to_owned())
}

fn launcher_command(selection: VisualSelection) -> clap::Command {
    clap_theme::apply(
        shared_help::disable_help_subcommands(Cli::command()),
        selection.palette(),
        selection.presentation(),
    )
}

fn write_short_help(selection: VisualSelection) -> ExitCode {
    let mut command = launcher_command(selection);
    let rendered = command.render_help().ansi().to_string();
    write_stdout(clap_theme::present_help(rendered, selection.presentation()))
}

fn write_version_report() -> ExitCode {
    let directory = match suite_directory() {
        Ok(directory) => directory,
        Err(message) => return write_internal_error(&message),
    };
    let mut report = format!("liknon-cli {SUITE_VERSION}\nliknon {SUITE_VERSION}");
    for capability in Capability::ALL {
        if let Err(message) = verify_component(&directory, capability) {
            return write_internal_error(&message);
        }
        report.push('\n');
        report.push_str(capability.executable());
        report.push(' ');
        report.push_str(SUITE_VERSION);
    }
    write_stdout(report)
}

fn dispatch(route: Route) -> ExitCode {
    let directory = match suite_directory() {
        Ok(directory) => directory,
        Err(message) => return write_internal_error(&message),
    };
    if let Err(message) = verify_component(&directory, route.capability) {
        return write_internal_error(&message);
    }
    let executable = component_path(&directory, route.capability);
    let status = match Command::new(&executable).args(route.arguments).status() {
        Ok(status) => status,
        Err(error) => {
            return write_internal_error(&format!(
                "cannot execute suite component {}: {error}",
                executable.display()
            ));
        }
    };
    exit_with_child_status(status)
}

fn suite_directory() -> Result<PathBuf, String> {
    let executable = std::env::current_exe()
        .map_err(|error| format!("cannot resolve the launcher executable: {error}"))?;
    executable
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| "the launcher executable has no parent directory".to_owned())
}

fn component_path(directory: &Path, capability: Capability) -> PathBuf {
    directory.join(format!(
        "{}{}",
        capability.executable(),
        std::env::consts::EXE_SUFFIX
    ))
}

fn verify_component(directory: &Path, capability: Capability) -> Result<(), String> {
    let executable = component_path(directory, capability);
    if !executable.is_file() {
        return Err(format!(
            "suite component {} is missing beside the launcher",
            executable.display()
        ));
    }
    let output = Command::new(&executable)
        .arg("--version")
        .stdin(Stdio::null())
        .output()
        .map_err(|error| {
            format!(
                "cannot inspect suite component {}: {error}",
                executable.display()
            )
        })?;
    let expected = format!("{} {SUITE_VERSION}\n", capability.executable());
    if !output.status.success() || output.stdout != expected.as_bytes() || !output.stderr.is_empty()
    {
        return Err(format!(
            "suite component {} does not match liknon-cli {SUITE_VERSION}",
            executable.display()
        ));
    }
    Ok(())
}

fn exit_with_child_status(status: ExitStatus) -> ExitCode {
    if let Some(code) = status.code() {
        std::process::exit(code);
    }

    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt as _;

        if let Some(signal) = status.signal() {
            // Restoring and re-raising the terminating signal preserves the
            // shell-visible process semantics of direct capability execution.
            unsafe {
                libc::signal(signal, libc::SIG_DFL);
                libc::raise(signal);
            }
            std::process::exit(128 + signal);
        }
    }

    std::process::exit(1)
}

fn write_usage_error(message: &str) -> ExitCode {
    let mut stderr = std::io::stderr().lock();
    let _ = writeln!(
        stderr,
        "liknon: {message}\n\nFor more information, try '--help'."
    );
    let _ = stderr.flush();
    ExitCode::from(2)
}

fn write_internal_error(message: &str) -> ExitCode {
    let mut stderr = std::io::stderr().lock();
    let _ = writeln!(stderr, "liknon: {message}");
    let _ = stderr.flush();
    ExitCode::from(4)
}

fn write_stdout(rendered: impl AsRef<str>) -> ExitCode {
    let mut stdout = std::io::stdout().lock();
    if writeln!(stdout, "{}", rendered.as_ref().trim_end())
        .and_then(|()| stdout.flush())
        .is_ok()
    {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_only_explicit_capability_namespaces() {
        for (namespace, capability) in [
            ("validator", Capability::Validator),
            ("v", Capability::Validator),
            ("manager", Capability::Manager),
            ("m", Capability::Manager),
            ("tool", Capability::Tool),
            ("t", Capability::Tool),
        ] {
            let arguments = vec!["liknon".into(), namespace.into(), "--help".into()];
            let RootRequest::Route(route) = parse_root_request(&arguments) else {
                panic!("expected route for {namespace}");
            };
            assert_eq!(route.capability, capability);
            assert_eq!(route.arguments, [OsString::from("--help")]);
        }
    }

    #[test]
    fn preserves_the_argument_terminator_and_following_values() {
        let arguments = vec![
            "liknon".into(),
            "validator".into(),
            "--".into(),
            "--help".into(),
        ];
        let RootRequest::Route(route) = parse_root_request(&arguments) else {
            panic!("expected route");
        };
        assert_eq!(
            route.arguments,
            [OsString::from("--"), OsString::from("--help")]
        );
    }

    #[test]
    fn visual_options_belong_to_root_help_or_delegated_arguments() {
        let root = vec![
            "liknon".into(),
            "--color=standard".into(),
            "validator".into(),
        ];
        assert!(matches!(parse_root_request(&root), RootRequest::Error(_)));

        let delegated = vec![
            "liknon".into(),
            "validator".into(),
            "--color=standard".into(),
        ];
        let RootRequest::Route(route) = parse_root_request(&delegated) else {
            panic!("expected delegated route");
        };
        assert_eq!(route.arguments, [OsString::from("--color=standard")]);
    }
}
