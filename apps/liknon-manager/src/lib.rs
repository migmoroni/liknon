//! Identity-only shell for the future Liknon resource manager.

use clap::Parser;
use std::process::ExitCode;

#[derive(Debug, Parser)]
#[command(
    name = "liknon-manager",
    version,
    about = "Manages installed Liknon resources (commands are not available yet)",
    arg_required_else_help = true
)]
struct Cli {}

/// Parses the identity-only Manager shell and returns success.
pub fn run_cli() -> ExitCode {
    let _ = Cli::parse();
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::{error::ErrorKind, Parser};

    #[test]
    fn shell_exposes_only_identity_help_and_version() {
        assert_eq!(
            Cli::try_parse_from(["liknon-manager"]).unwrap_err().kind(),
            ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        );
        assert_eq!(
            Cli::try_parse_from(["liknon-manager", "--help"])
                .unwrap_err()
                .kind(),
            ErrorKind::DisplayHelp
        );
        assert_eq!(
            Cli::try_parse_from(["liknon-manager", "--version"])
                .unwrap_err()
                .kind(),
            ErrorKind::DisplayVersion
        );
        assert_eq!(
            Cli::try_parse_from(["liknon-manager", "install"])
                .unwrap_err()
                .kind(),
            ErrorKind::UnknownArgument
        );
    }
}
