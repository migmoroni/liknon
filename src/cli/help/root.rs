//! Root help text and consolidated command-tree rendering.

use clap::CommandFactory;
use std::ffi::OsString;

pub(in crate::cli) const ABOUT: &str = "Runs declarative workspace validation pipelines";

pub(in crate::cli) const AFTER_HELP: &str = "Run 'workspace-validator <COMMAND> --help' to focus on one command. The '-h' flag prints a compact summary.";

/// Renders every nested command through Clap's own help tree.
pub(in crate::cli) fn render<T: CommandFactory>() -> String {
    flatten(T::command()).render_long_help().to_string()
}

/// Identifies the complete root reference without changing compact `-h` behavior.
pub(in crate::cli) fn requested(arguments: &[OsString]) -> bool {
    arguments.len() == 2 && arguments[1] == "--help"
}

fn flatten(command: clap::Command) -> clap::Command {
    command.flatten_help(true).mut_subcommands(flatten)
}
