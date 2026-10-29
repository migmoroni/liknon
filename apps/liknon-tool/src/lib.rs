//! Identity-only shell for the future Liknon offline resource tool.

use std::{io::Write as _, process::ExitCode};

use clap::{error::ErrorKind, CommandFactory as _, FromArgMatches as _, Parser};
use liknon_cli_ui::{
    clap as clap_theme,
    help::{self as shared_help, RootHelp},
    VisualOptions, VisualSelection,
};

const HELP: &str = "Print help at the root or after any command";
const HELP_LONG: &str = "Prints help at the root or after any command.\n\nThe -h flag prints a compact summary; --help prints complete details.";

#[derive(Debug, Parser)]
#[command(
    name = "liknon-tool",
    version,
    about = "Reads installed Liknon resources offline (commands are not available yet)",
    arg_required_else_help = true,
    disable_help_subcommand = true
)]
struct Cli {
    #[command(flatten)]
    _visual: VisualOptions,
}

/// Parses the identity-only Tool shell through the shared terminal foundation.
pub fn run_cli() -> ExitCode {
    let arguments = std::env::args_os().collect::<Vec<_>>();
    let selection = VisualSelection::from_args(&arguments);
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

    match command.try_get_matches_from(arguments) {
        Ok(matches) => match Cli::from_arg_matches(&matches) {
            Ok(_) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("liknon-tool: {error}");
                ExitCode::from(4)
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
            ExitCode::from(code)
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
        ExitCode::from(4)
    }
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::{error::ErrorKind, Parser};

    #[test]
    fn shell_exposes_only_identity_and_shared_visual_options() {
        assert_eq!(
            Cli::try_parse_from(["liknon-tool"]).unwrap_err().kind(),
            ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        );
        assert!(Cli::try_parse_from([
            "liknon-tool",
            "--color=standard",
            "--presentation=low-vision"
        ])
        .is_ok());
        assert_eq!(
            Cli::try_parse_from(["liknon-tool", "show"])
                .unwrap_err()
                .kind(),
            ErrorKind::UnknownArgument
        );
    }
}
