//! Capability-free launcher shell for the Liknon CLI suite.

use clap::{Parser, Subcommand};
use std::process::ExitCode;

#[derive(Debug, Parser)]
#[command(
    name = "liknon",
    version,
    about = "Selects a Liknon capability namespace",
    arg_required_else_help = true
)]
struct Cli {
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
    let _ = Cli::parse();
    eprintln!(
        "liknon: capability dispatch is not available yet; invoke the capability executable directly"
    );
    ExitCode::from(2)
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::{error::ErrorKind, Parser};

    #[test]
    fn launcher_exposes_only_explicit_capability_namespaces() {
        for namespace in ["validate", "manager", "tool"] {
            assert!(Cli::try_parse_from(["liknon", namespace]).is_ok());
        }
        assert_eq!(
            Cli::try_parse_from(["liknon", "check"]).unwrap_err().kind(),
            ErrorKind::InvalidSubcommand
        );
        assert_eq!(
            Cli::try_parse_from(["liknon", "--help"])
                .unwrap_err()
                .kind(),
            ErrorKind::DisplayHelp
        );
        assert_eq!(
            Cli::try_parse_from(["liknon", "--version"])
                .unwrap_err()
                .kind(),
            ErrorKind::DisplayVersion
        );
    }
}
