//! Clap adapter for the shared semantic theme.

use ::clap::{
    builder::styling::{AnsiColor, Color, Style, Styles},
    ColorChoice, Command,
};

use crate::{
    PaletteProfile, PresentationProfile, SemanticRole, StyleAttribute, TerminalColor, Theme,
};

/// Applies one resolved visual theme to help, usage, and parser diagnostics.
pub fn apply(
    command: Command,
    palette: PaletteProfile,
    presentation: PresentationProfile,
) -> Command {
    let theme = Theme::resolve(palette, presentation);
    let color_choice =
        if palette == PaletteProfile::Plain && presentation == PresentationProfile::Standard {
            ColorChoice::Never
        } else {
            ColorChoice::Always
        };

    apply_presentation(command, presentation)
        .color(color_choice)
        .styles(styles(&theme))
}

/// Applies presentation-only spacing after custom help composition.
pub fn present_help(rendered: String, presentation: PresentationProfile) -> String {
    match presentation {
        PresentationProfile::Standard => rendered,
        PresentationProfile::LowVision => expand_help_spacing(&rendered),
    }
}

fn apply_presentation(command: Command, presentation: PresentationProfile) -> Command {
    match presentation {
        PresentationProfile::Standard => command,
        PresentationProfile::LowVision => command
            .mut_args(|argument| argument.next_line_help(true))
            .mut_subcommands(|subcommand| apply_presentation(subcommand, presentation)),
    }
}

fn expand_help_spacing(rendered: &str) -> String {
    let mut expanded = String::with_capacity(rendered.len());
    let mut in_blank_run = false;
    let mut in_item_section = false;
    let mut has_item = false;

    for line in rendered.split_inclusive('\n') {
        let content = line.strip_suffix('\n').unwrap_or(line);
        let plain = console::strip_ansi_codes(content);
        if plain.is_empty() {
            if !in_blank_run {
                expanded.push_str("\n\n");
                in_blank_run = true;
            }
        } else {
            let indentation = plain
                .chars()
                .take_while(|character| *character == ' ')
                .count();
            let trimmed = plain.trim();
            if matches!(trimmed, "Commands:" | "Arguments:" | "Options:") {
                in_item_section = true;
                has_item = false;
            } else if indentation == 0 {
                in_item_section = false;
                has_item = false;
            } else if in_item_section && indentation < 10 {
                if has_item && !expanded.ends_with("\n\n") {
                    expanded.push('\n');
                }
                has_item = true;
            }

            expanded.push_str(line);
            in_blank_run = false;
        }
    }

    expanded
}

fn styles(theme: &Theme) -> Styles {
    Styles::plain()
        .header(resolve(theme, SemanticRole::Section))
        .error(resolve(theme, SemanticRole::Failure))
        .usage(resolve(theme, SemanticRole::Heading))
        .literal(resolve(theme, SemanticRole::AccentPrimary))
        .placeholder(resolve(theme, SemanticRole::Path))
        .valid(resolve(theme, SemanticRole::Success))
        .invalid(resolve(theme, SemanticRole::Blocked))
        .context(resolve(theme, SemanticRole::Metadata))
        .context_value(resolve(theme, SemanticRole::Path))
}

fn resolve(theme: &Theme, role: SemanticRole) -> Style {
    let specification = theme.style(role);
    let mut style = Style::new();

    if let Some(color) = specification.foreground() {
        style = style.fg_color(Some(Color::Ansi(ansi_color(
            color,
            specification.has_attribute(StyleAttribute::Bright),
        ))));
    }
    if specification.has_attribute(StyleAttribute::Bold) {
        style = style.bold();
    }
    if specification.has_attribute(StyleAttribute::Dim) {
        style = style.dimmed();
    }
    if specification.has_attribute(StyleAttribute::Underline) {
        style = style.underline();
    }
    if specification.has_attribute(StyleAttribute::Reverse) {
        style = style.invert();
    }
    if specification.foreground().is_none() && specification.has_attribute(StyleAttribute::Bright) {
        style = style.bold();
    }

    style
}

fn ansi_color(color: TerminalColor, bright: bool) -> AnsiColor {
    match (color, bright) {
        (TerminalColor::Red, false) => AnsiColor::Red,
        (TerminalColor::Red, true) => AnsiColor::BrightRed,
        (TerminalColor::Green, false) => AnsiColor::Green,
        (TerminalColor::Green, true) => AnsiColor::BrightGreen,
        (TerminalColor::Yellow, false) => AnsiColor::Yellow,
        (TerminalColor::Yellow, true) => AnsiColor::BrightYellow,
        (TerminalColor::Blue, false) => AnsiColor::Blue,
        (TerminalColor::Blue, true) => AnsiColor::BrightBlue,
        (TerminalColor::Magenta, false) => AnsiColor::Magenta,
        (TerminalColor::Magenta, true) => AnsiColor::BrightMagenta,
        (TerminalColor::Cyan, false) => AnsiColor::Cyan,
        (TerminalColor::Cyan, true) => AnsiColor::BrightCyan,
        (TerminalColor::White, false) => AnsiColor::White,
        (TerminalColor::White, true) => AnsiColor::BrightWhite,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::clap::Arg;

    fn render(palette: PaletteProfile, presentation: PresentationProfile) -> String {
        let mut command = apply(
            Command::new("example")
                .about("Examines a workspace")
                .arg(Arg::new("target").long("target").value_name("TARGET")),
            palette,
            presentation,
        );
        command.render_help().ansi().to_string()
    }

    #[test]
    fn palettes_change_only_ansi_styling() {
        let plain = render(PaletteProfile::Plain, PresentationProfile::Standard);
        for palette in PaletteProfile::ALL
            .into_iter()
            .filter(|palette| *palette != PaletteProfile::Plain)
        {
            let rendered = render(palette, PresentationProfile::Standard);
            assert!(rendered.contains('\u{1b}'), "{palette:?}");
            assert_eq!(console::strip_ansi_codes(&rendered), plain, "{palette:?}");
        }
    }

    #[test]
    fn low_vision_expands_help_without_losing_content() {
        let standard = render(PaletteProfile::Plain, PresentationProfile::Standard);
        let low_vision = render(PaletteProfile::Plain, PresentationProfile::LowVision);
        assert!(low_vision.contains('\u{1b}'));
        assert!(console::strip_ansi_codes(&low_vision).lines().count() > standard.lines().count());
    }
}
