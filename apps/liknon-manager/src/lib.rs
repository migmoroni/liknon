//! Authenticated acquisition and transactional state management for Liknon resources.

mod cli;
mod error;
mod hub;
mod operations;
mod output;
mod resolver;
mod store;
mod workspace;

use std::process::ExitCode;

use clap::Parser as _;

/// Parses process arguments, executes one Manager operation, and returns its stable exit status.
pub fn run_cli() -> ExitCode {
    run(cli::Cli::parse())
}

fn run(cli: cli::Cli) -> ExitCode {
    let format = cli.format();
    match operations::execute(cli) {
        Ok(report) => {
            if let Err(error) = output::write_report(&report, format) {
                eprintln!("error: {error}");
                return ExitCode::from(error.exit_code());
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            output::write_error(&error, format);
            ExitCode::from(error.exit_code())
        }
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
