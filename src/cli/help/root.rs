//! Root help text and consolidated command-tree rendering.

use crate::cli::arguments::ColorPaletteArgument;
use clap::ValueEnum;
use std::ffi::OsString;

pub(in crate::cli) const ABOUT: &str = "Runs declarative workspace validation pipelines";

pub(in crate::cli) const AFTER_HELP: &str = "Run 'workspace-validator <COMMAND> --help' to focus on one command. The '-h' flag prints a compact summary.";

pub(in crate::cli) const COLOR: &str = "Enables ANSI color in human CLI output";

pub(in crate::cli) const COLOR_LONG: &str = "Enables ANSI color in human CLI output.\n\nWithout this flag, output contains no ANSI color. Bare --color selects the standard palette; named palettes require --color=<PALETTE>. An explicit command-line palette overrides NO_COLOR. Status and hierarchy never rely on color alone.";

/// Renders every nested command through Clap's own themed help tree.
pub(in crate::cli) fn render(command: clap::Command) -> String {
    flatten(command).render_long_help().ansi().to_string()
}

/// Identifies the complete root reference while allowing its global color option.
pub(in crate::cli) fn requested(arguments: &[OsString]) -> bool {
    let mut help = false;
    let mut color = false;

    for argument in arguments.iter().skip(1) {
        let Some(argument) = argument.to_str() else {
            return false;
        };
        match argument {
            "--help" if !help => help = true,
            "--color" if !color => color = true,
            value
                if !color
                    && value.strip_prefix("--color=").is_some_and(|palette| {
                        ColorPaletteArgument::from_str(palette, true).is_ok()
                    }) =>
            {
                color = true
            }
            _ => return false,
        }
    }

    help
}

fn flatten(command: clap::Command) -> clap::Command {
    command.flatten_help(true).mut_subcommands(flatten)
}
