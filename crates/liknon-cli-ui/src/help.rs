//! Generic compact and complete help composition.

use std::{collections::BTreeMap, ffi::OsString};

use clap::{Command, ValueEnum};
use console::strip_ansi_codes;

use crate::{
    clap as clap_theme, PaletteArgument, PaletteProfile, PresentationArgument, PresentationProfile,
    SemanticRole, Theme,
};

/// Rendering inputs for one complete root reference.
#[derive(Clone, Copy, Debug)]
pub struct RootHelp<'a> {
    /// Optional ASCII mark rendered above the root reference.
    pub logo: Option<&'a str>,
    /// Short text used for the generated `-h, --help` option.
    pub help: &'static str,
    /// Complete text used for the generated `-h, --help` option.
    pub long_help: &'static str,
    /// Width of separators before top-level commands.
    pub separator_width: usize,
}

impl<'a> RootHelp<'a> {
    /// Creates the canonical Liknon complete-help layout.
    pub const fn liknon(help: &'static str, long_help: &'static str) -> Self {
        Self {
            logo: Some(crate::LIKNON_ASCII_LOGO),
            help,
            long_help,
            separator_width: 78,
        }
    }
}

/// Renders a complete root command reference from an application-owned tree.
///
/// The caller remains responsible for command grammar, descriptions, examples,
/// and any domain annotations added before this function is called.
pub fn render_complete_root(
    command: Command,
    palette: PaletteProfile,
    presentation: PresentationProfile,
    options: RootHelp<'_>,
) -> String {
    render_complete_root_with_details(command.clone(), command, palette, presentation, options)
}

/// Renders a complete root reference while allowing detailed command blocks to
/// carry annotations that should not be repeated in the compact command index.
pub fn render_complete_root_with_details(
    index_command: Command,
    command: Command,
    palette: PaletteProfile,
    presentation: PresentationProfile,
    options: RootHelp<'_>,
) -> String {
    let root_name = command.get_name().to_owned();
    debug_assert_eq!(index_command.get_name(), command.get_name());
    let command_index = command_index(&index_command);
    let usages = command_usages(&command);
    let mut command = flatten(annotate_command_usages(command, &root_name, &usages));
    command.build();
    let rendered = command
        .mut_arg("help", |argument| {
            argument.help(options.help).long_help(options.long_help)
        })
        .render_long_help()
        .ansi()
        .to_string();
    let rendered = insert_command_index(rendered, &command_index);
    let rendered = clap_theme::present_help(rendered, presentation);
    let rendered = separate_commands(rendered, &root_name, presentation, options.separator_width);
    prepend_logo(rendered, palette, presentation, options.logo)
}

/// Returns whether arguments request only the complete root help plus valid
/// visual rendering controls.
pub fn is_complete_root_request(arguments: &[OsString]) -> bool {
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
                        PaletteArgument::from_str(palette, true).is_ok()
                    }) =>
            {
                color = true;
            }
            value
                if !presentation
                    && value
                        .strip_prefix("--presentation=")
                        .is_some_and(|mode| PresentationArgument::from_str(mode, true).is_ok()) =>
            {
                presentation = true;
            }
            _ => return false,
        }
    }

    help
}

/// Disables Clap's synthetic `help` subcommands recursively while preserving
/// `-h` and `--help` flags.
pub fn disable_help_subcommands(command: Command) -> Command {
    command
        .disable_help_subcommand(true)
        .mut_subcommands(disable_help_subcommands)
}

fn command_index(command: &Command) -> String {
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

fn command_usages(command: &Command) -> BTreeMap<String, String> {
    let mut command = command.clone();
    command.build();
    let mut usages = BTreeMap::new();
    collect_command_usages(&command, &mut usages);
    usages
}

fn collect_command_usages(command: &Command, usages: &mut BTreeMap<String, String>) {
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
    command: Command,
    parent_path: &str,
    usages: &BTreeMap<String, String>,
) -> Command {
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

fn flatten(command: Command) -> Command {
    command
        .flatten_help(true)
        .mut_subcommands(flatten_embedded_command)
}

fn flatten_embedded_command(command: Command) -> Command {
    command
        .disable_help_flag(true)
        .flatten_help(true)
        .mut_subcommands(flatten_embedded_command)
}

fn separate_commands(
    rendered: String,
    root_name: &str,
    presentation: PresentationProfile,
    separator_width: usize,
) -> String {
    let heading_prefix = format!("{root_name} ");
    let mut separated = String::with_capacity(rendered.len());

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
                    separator_width / 2
                } else {
                    separator_width
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
    logo: Option<&str>,
) -> String {
    let Some(logo) = logo else {
        return rendered;
    };
    let theme = Theme::resolve(palette, presentation);
    let logo = logo.trim_end_matches(['\r', '\n']);
    let gap = match presentation {
        PresentationProfile::Standard => "\n\n",
        PresentationProfile::LowVision => "\n\n\n",
    };
    format!(
        "{}{gap}{rendered}",
        theme.paint(SemanticRole::Heading, logo)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;

    fn arbitrary_tree() -> Command {
        Command::new("arbitrary")
            .about("Arbitrary product")
            .subcommand(Command::new("alpha").about("First operation"))
            .subcommand(
                Command::new("beta")
                    .about("Second operation")
                    .subcommand(Command::new("nested").about("Nested operation")),
            )
    }

    #[test]
    fn complete_help_uses_arbitrary_command_metadata() {
        let rendered = render_complete_root(
            arbitrary_tree(),
            PaletteProfile::Plain,
            PresentationProfile::Standard,
            RootHelp {
                logo: None,
                help: "Print help",
                long_help: "Print complete help",
                separator_width: 40,
            },
        );
        assert!(rendered.contains("Commands:"));
        assert!(rendered.contains("Usage: arbitrary alpha"));
        assert!(rendered.contains(&"─".repeat(40)));
        assert!(rendered.contains(&"─".repeat(20)));
        assert!(!rendered.contains("validator"));
        assert!(!rendered.contains("manager"));
    }
}
