//! Capability-free launcher shell for the Liknon CLI suite.

use std::{io::Write as _, process::ExitCode};

use clap::{error::ErrorKind, CommandFactory as _, FromArgMatches as _, Parser, Subcommand};
use liknon_cli_ui::{
    clap as clap_theme,
    help::{self as shared_help, RootHelp},
    RootVisualOptions, VisualSelection,
};

const HELP: &str = "Print help at the launcher root";
const HELP_LONG: &str = "Prints launcher help.\n\nThe -h flag prints a compact summary; --help prints complete launcher details. Capability-specific help belongs to the selected capability executable.";

#[derive(Debug, Parser)]
#[command(
    name = "liknon",
    version,
    about = "Selects a Liknon capability namespace",
    arg_required_else_help = true,
    disable_help_subcommand = true
)]
struct Cli {
    #[command(flatten)]
    _visual: RootVisualOptions,
    #[command(subcommand)]
    namespace: Namespace,
}

#[derive(Debug, Subcommand)]
enum Namespace {
    /// Selects the validation capability.
    Validate,
    /// Selects the resource-management capability.
    Manager,
    /// Selects the offline resource-tool capability.
    Tool,
}

/// Parses the launcher namespace without dispatching a capability process.
pub fn run_launcher() -> ExitCode {
    let arguments = std::env::args_os().collect::<Vec<_>>();
    let selection = VisualSelection::from_root_args(&arguments);
    let command = clap_theme::apply(
        shared_help::disable_help_subcommands(Cli::command()),
        selection.palette(),
        selection.presentation(),
    );

    if shared_help::is_complete_root_request(&arguments) {
        return write_help(shared_help::render_complete_root(
            command,
            selection.palette(),
            selection.presentation(),
            RootHelp::liknon(HELP, HELP_LONG),
        ));
    }

    let cli = match command.try_get_matches_from(arguments) {
        Ok(matches) => match Cli::from_arg_matches(&matches) {
            Ok(cli) => cli,
            Err(error) => {
                eprintln!("liknon: {error}");
                return ExitCode::from(4);
            }
        },
        Err(error) => {
            let kind = error.kind();
            if matches!(
                kind,
                ErrorKind::DisplayHelp | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
            ) {
                return write_help(clap_theme::present_help(
                    error.render().ansi().to_string(),
                    selection.presentation(),
                ));
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

    let _ = cli.namespace;
    eprintln!(
        "liknon: capability dispatch is not available yet; invoke the capability executable directly"
    );
    ExitCode::from(2)
}

fn write_help(rendered: String) -> ExitCode {
    let mut stdout = std::io::stdout().lock();
    if writeln!(stdout, "{}", rendered.trim_end())
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
    use super::Cli;
    use clap::{error::ErrorKind, Parser};

    #[test]
    fn launcher_keeps_visual_options_at_its_own_root() {
        for namespace in ["validate", "manager", "tool"] {
            assert!(Cli::try_parse_from(["liknon", namespace]).is_ok());
        }
        assert!(Cli::try_parse_from(["liknon", "--color=standard", "validate"]).is_ok());
        assert_eq!(
            Cli::try_parse_from(["liknon", "validate", "--color=standard"])
                .unwrap_err()
                .kind(),
            ErrorKind::UnknownArgument
        );
    }
}
