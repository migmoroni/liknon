//! Validation-owned help text over the shared command-tree renderer.

use super::execution;
use crate::theme::{PaletteProfile, PresentationProfile};
use clap::Arg;
use liknon_cli_ui::help::{self as shared_help, RootHelp};
use std::ffi::OsString;

pub(in crate::cli) const ABOUT: &str = "Runs declarative workspace validation pipelines";

const HELP: &str = "Prints help at the root or after any command";

const HELP_LONG: &str = "Prints help at the root or after any command.\n\nThe -h flag prints a compact summary; --help prints complete details.";

pub(in crate::cli) const COLOR: &str = "Enables ANSI color in human CLI output";

pub(in crate::cli) const COLOR_LONG: &str = "Enables ANSI color in human CLI output.\n\nWith -h or --help, colors the requested help. During validate and check, colors the human validation report. Without this flag, output contains no ANSI color unless an explicit presentation adds typographic emphasis. Bare --color selects the standard palette; named palettes require --color=<PALETTE>. An explicit command-line palette overrides NO_COLOR. Status and hierarchy never rely on color alone.";

pub(in crate::cli) const PRESENTATION: &str = "Selects the layout and emphasis of human CLI output";

pub(in crate::cli) const PRESENTATION_LONG: &str = "Selects the layout and emphasis of human CLI output independently from color.\n\nWith -h or --help, changes the requested help presentation. During validate and check, changes the human validation report. Standard uses the compact layout. Low-vision expands spacing and removes dim styling. Named modes require --presentation=<MODE>.";

/// Configures the command tree for focused, execution-aware help rendering.
pub(in crate::cli) fn configure(command: clap::Command) -> clap::Command {
    let root_name = command.get_name().to_owned();
    let command = command
        .mut_arg("color", |argument| {
            argument.help(COLOR).long_help(COLOR_LONG)
        })
        .mut_arg("presentation", |argument| {
            argument.help(PRESENTATION).long_help(PRESENTATION_LONG)
        });
    let visual_arguments = visual_arguments(&command);
    shared_help::disable_help_subcommands(command)
        .override_usage(format!("{root_name} <COMMAND> [OPTIONS]"))
        .mut_subcommands(|subcommand| {
            let supports_visual_execution = matches!(subcommand.get_name(), "validate" | "check");
            visual_arguments
                .iter()
                .cloned()
                .fold(subcommand, |subcommand, argument| {
                    if supports_visual_execution {
                        subcommand.arg(execution_argument(argument))
                    } else {
                        subcommand.arg(argument.hide(true))
                    }
                })
        })
}

/// Renders the complete root command reference through the shared help shell.
pub(in crate::cli) fn render(
    command: clap::Command,
    palette: PaletteProfile,
    presentation: PresentationProfile,
) -> String {
    shared_help::render_complete_root_with_details(
        command.clone(),
        annotate_execution_commands(command),
        palette,
        presentation,
        RootHelp::liknon(HELP, HELP_LONG),
    )
}

/// Identifies the complete root reference and its visual rendering options.
pub(in crate::cli) fn requested(arguments: &[OsString]) -> bool {
    shared_help::is_complete_root_request(arguments)
}

fn visual_arguments(command: &clap::Command) -> [Arg; 2] {
    ["color", "presentation"].map(|id| {
        command
            .get_arguments()
            .find(|argument| argument.get_id() == id)
            .unwrap_or_else(|| panic!("root visual argument `{id}` is undefined"))
            .clone()
    })
}

fn execution_argument(argument: Arg) -> Arg {
    match argument.get_id().as_str() {
        "color" => argument
            .help(execution::COLOR)
            .long_help(execution::COLOR_LONG)
            .hide_possible_values(true),
        "presentation" => argument
            .help(execution::PRESENTATION)
            .long_help(execution::PRESENTATION_LONG)
            .hide_possible_values(true),
        id => panic!("unsupported visual argument `{id}`"),
    }
}

fn annotate_execution_commands(command: clap::Command) -> clap::Command {
    command.mut_subcommands(|subcommand| match subcommand.get_name() {
        "validate" => subcommand.about(execution::VALIDATE_ROOT_REFERENCE),
        "check" => subcommand.about(execution::CHECK_ROOT_REFERENCE),
        _ => subcommand,
    })
}
