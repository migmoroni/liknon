//! Root help text and consolidated command-tree rendering.

use super::execution;
use crate::cli::arguments::{ColorPaletteArgument, PresentationArgument};
use crate::theme::{self, PaletteProfile, PresentationProfile, Role, Theme};
use clap::{Arg, ValueEnum};
use console::strip_ansi_codes;
use std::{collections::BTreeMap, ffi::OsString};

const COMMAND_SEPARATOR_WIDTH: usize = 78;
const SUBCOMMAND_SEPARATOR_WIDTH: usize = COMMAND_SEPARATOR_WIDTH / 2;

const ASCII_LOGO: &str = include_str!("../../../assets/branding/liknon-ascii.txt");

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
    let visual_arguments = visual_arguments(&command);
    disable_help_subcommands(command)
        .override_usage(format!("{root_name} <COMMAND> [OPTIONS]"))
        .mut_subcommands(|subcommand| {
            let supports_visual_execution = matches!(subcommand.get_name(), "validate" | "check");
            // Explicit child copies keep global parsing available at every depth,
            // while their visibility and wording follow command execution semantics.
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

/// Renders every nested command through Clap's own themed help tree.
pub(in crate::cli) fn render(
    command: clap::Command,
    palette: PaletteProfile,
    presentation: PresentationProfile,
) -> String {
    let root_name = command.get_name().to_owned();
    let command_index = command_index(&command);
    let command = annotate_execution_commands(command);
    let usages = command_usages(&command);
    let mut command = flatten(annotate_command_usages(command, &root_name, &usages));
    command.build();
    let rendered = command
        .mut_arg("help", |argument| argument.help(HELP).long_help(HELP_LONG))
        .render_long_help()
        .ansi()
        .to_string();
    let rendered = insert_command_index(rendered, &command_index);
    let rendered = theme::clap::present_help(rendered, presentation);
    let rendered = separate_commands(rendered, &root_name, presentation);
    prepend_logo(rendered, palette, presentation)
}

/// Identifies the complete root reference and its visual rendering options.
pub(in crate::cli) fn requested(arguments: &[OsString]) -> bool {
    let mut help = false;
    let mut color = false;
    let mut presentation = false;

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
            value
                if !presentation
                    && value
                        .strip_prefix("--presentation=")
                        .is_some_and(|mode| PresentationArgument::from_str(mode, true).is_ok()) =>
            {
                presentation = true
            }
            _ => return false,
        }
    }

    help
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

fn disable_help_subcommands(command: clap::Command) -> clap::Command {
    command
        .disable_help_subcommand(true)
        .mut_subcommands(disable_help_subcommands)
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

fn command_index(command: &clap::Command) -> String {
    let mut command = command.clone();
    let rendered = command.render_long_help().ansi().to_string();
    let mut index = String::new();
    let mut capturing = false;

    for line in rendered.split_inclusive('\n') {
        let plain = strip_ansi_codes(line.trim_end_matches('\n'));
        if plain == "Commands:" {
            capturing = true;
        } else if capturing && plain == "Options:" {
            break;
        }

        if capturing {
            index.push_str(line);
        }
    }

    index.trim_end().to_owned()
}

fn insert_command_index(rendered: String, command_index: &str) -> String {
    let mut output = String::with_capacity(rendered.len() + command_index.len() + 2);
    let mut inserted = false;

    for line in rendered.split_inclusive('\n') {
        let plain = strip_ansi_codes(line.trim_end_matches('\n'));
        if !inserted && plain == "Options:" {
            output.push_str(command_index);
            output.push_str("\n\n");
            inserted = true;
        }
        output.push_str(line);
    }

    output
}

fn command_usages(command: &clap::Command) -> BTreeMap<String, String> {
    let mut command = command.clone();
    command.build();
    let mut usages = BTreeMap::new();
    collect_command_usages(&command, &mut usages);
    usages
}

fn collect_command_usages(command: &clap::Command, usages: &mut BTreeMap<String, String>) {
    for subcommand in command.get_subcommands() {
        let command_path = subcommand
            .get_bin_name()
            .expect("built subcommand has a complete invocation path")
            .to_owned();
        let mut usage_command = subcommand.clone();
        usages.insert(command_path, usage_command.render_usage().to_string());
        collect_command_usages(subcommand, usages);
    }
}

fn annotate_command_usages(
    command: clap::Command,
    parent_path: &str,
    usages: &BTreeMap<String, String>,
) -> clap::Command {
    command.mut_subcommands(|subcommand| {
        let command_path = format!("{parent_path} {}", subcommand.get_name());
        let usage = usages
            .get(&command_path)
            .unwrap_or_else(|| panic!("usage for command `{command_path}` is undefined"));
        let about = subcommand
            .get_about()
            .map(ToString::to_string)
            .unwrap_or_default();

        annotate_command_usages(
            subcommand.about(format!("{about}\n\n{usage}")),
            &command_path,
            usages,
        )
    })
}

fn flatten(command: clap::Command) -> clap::Command {
    command
        .flatten_help(true)
        .mut_subcommands(flatten_embedded_command)
}

fn flatten_embedded_command(command: clap::Command) -> clap::Command {
    // The consolidated root reference already documents help once. Focused
    // command trees retain Clap's own help flag because this mutation is made
    // only on the temporary tree rendered by `render`.
    command
        .disable_help_flag(true)
        .flatten_help(true)
        .mut_subcommands(flatten_embedded_command)
}

fn separate_commands(
    rendered: String,
    root_name: &str,
    presentation: PresentationProfile,
) -> String {
    let heading_prefix = format!("{root_name} ");
    let mut separated = String::with_capacity(rendered.len());

    // Clap has no per-command hook in flattened help. Detecting its completed
    // heading lines avoids rebuilding the command tree solely for presentation.
    for line in rendered.split_inclusive('\n') {
        let content = line.strip_suffix('\n').unwrap_or(line);
        let plain = strip_ansi_codes(content);
        if plain.starts_with(&heading_prefix) && plain.ends_with(':') {
            if let Some(start) = content.find(plain.as_ref()) {
                let end = start + plain.len();
                let command_path = plain
                    .strip_prefix(&heading_prefix)
                    .and_then(|heading| heading.strip_suffix(':'))
                    .expect("validated command heading");
                let width = if command_path.split_whitespace().count() > 1 {
                    SUBCOMMAND_SEPARATOR_WIDTH
                } else {
                    COMMAND_SEPARATOR_WIDTH
                };
                separated.push_str(&content[..start]);
                separated.push_str(&"─".repeat(width));
                separated.push_str(&content[end..]);
                match presentation {
                    PresentationProfile::Standard => separated.push('\n'),
                    PresentationProfile::LowVision => separated.push_str("\n\n"),
                }

                separated.push_str(&content[..start]);
                separated.push_str(command_path);
                separated.push(':');
                separated.push_str(&content[end..]);
                if line.ends_with('\n') {
                    separated.push('\n');
                }
                continue;
            }
        }
        separated.push_str(line);
    }

    separated
}

fn prepend_logo(
    rendered: String,
    palette: PaletteProfile,
    presentation: PresentationProfile,
) -> String {
    let theme = Theme::resolve(palette, presentation);
    let logo = ASCII_LOGO.trim_end_matches(['\r', '\n']);
    let gap = match presentation {
        PresentationProfile::Standard => "\n\n",
        PresentationProfile::LowVision => "\n\n\n",
    };
    format!("{}{gap}{rendered}", theme.paint(Role::Heading, logo))
}
