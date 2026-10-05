//! Clap adapter for the shared semantic theme.

use super::{
    style::{TerminalColor, BOLD, BRIGHT, DIM, REVERSE, UNDERLINE},
    PaletteProfile, PresentationProfile, Role, Theme,
};
use ::clap::{
    builder::styling::{AnsiColor, Color, Style, Styles},
    ColorChoice, Command,
};

/// Applies one resolved visual theme to help, usage, and parser diagnostics.
pub(crate) fn apply(
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
pub(crate) fn present_help(rendered: String, presentation: PresentationProfile) -> String {
    match presentation {
        PresentationProfile::Standard => rendered,
        PresentationProfile::LowVision => expand_paragraph_gaps(&rendered),
    }
}

fn apply_presentation(command: Command, presentation: PresentationProfile) -> Command {
    match presentation {
        PresentationProfile::Standard => command,
        PresentationProfile::LowVision => command
            .next_line_help(true)
            .mut_subcommands(|subcommand| apply_presentation(subcommand, presentation)),
    }
}

fn expand_paragraph_gaps(rendered: &str) -> String {
    let mut expanded = String::with_capacity(rendered.len());
    let mut in_blank_run = false;

    for line in rendered.split_inclusive('\n') {
        let content = line.strip_suffix('\n').unwrap_or(line);
        if content.is_empty() {
            if !in_blank_run {
                expanded.push_str("\n\n");
                in_blank_run = true;
            }
        } else {
            expanded.push_str(line);
            in_blank_run = false;
        }
    }

    expanded
}

fn styles(theme: &Theme) -> Styles {
    Styles::plain()
        .header(resolve(theme, Role::Section))
        .error(resolve(theme, Role::Failure))
        .usage(resolve(theme, Role::Heading))
        .literal(resolve(theme, Role::Tool))
        .placeholder(resolve(theme, Role::Path))
        .valid(resolve(theme, Role::Success))
        .invalid(resolve(theme, Role::Blocked))
        .context(resolve(theme, Role::Metadata))
        .context_value(resolve(theme, Role::Path))
}

fn resolve(theme: &Theme, role: Role) -> Style {
    let specification = theme.styles[role as usize];
    let mut style = Style::new();

    if let Some(color) = specification.foreground {
        style = style.fg_color(Some(Color::Ansi(ansi_color(
            color,
            specification.attributes & BRIGHT != 0,
        ))));
    }
    if specification.attributes & BOLD != 0 {
        style = style.bold();
    }
    if specification.attributes & DIM != 0 {
        style = style.dimmed();
    }
    if specification.attributes & UNDERLINE != 0 {
        style = style.underline();
    }
    if specification.attributes & REVERSE != 0 {
        style = style.invert();
    }
    if specification.foreground.is_none() && specification.attributes & BRIGHT != 0 {
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

    fn render(profile: PaletteProfile, presentation: PresentationProfile) -> String {
        let mut command = apply(
            Command::new("validator")
                .about("Validates a workspace")
                .arg(
                    Arg::new("target")
                        .long("target")
                        .value_name("TARGET")
                        .help("Selects the validation target"),
                ),
            profile,
            presentation,
        );
        command.render_help().ansi().to_string()
    }

    #[test]
    fn plain_help_is_ansi_free() {
        assert!(!render(PaletteProfile::Plain, PresentationProfile::Standard).contains('\u{1b}'));
    }

    #[test]
    fn every_explicit_palette_preserves_help_content() {
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
    fn low_vision_help_expands_layout_and_adds_typographic_emphasis() {
        let standard = render(PaletteProfile::Plain, PresentationProfile::Standard);
        let low_vision = render(PaletteProfile::Plain, PresentationProfile::LowVision);
        let low_vision_plain = console::strip_ansi_codes(&low_vision);

        assert!(low_vision.contains('\u{1b}'));
        assert_ne!(low_vision_plain, standard);
        assert!(low_vision_plain.lines().count() > standard.lines().count());
    }

    #[test]
    fn custom_help_spacing_changes_only_low_vision_paragraph_gaps() {
        let rendered = "first\n\nsecond\n".to_owned();
        assert_eq!(
            present_help(rendered.clone(), PresentationProfile::Standard),
            rendered
        );
        assert_eq!(
            present_help(rendered, PresentationProfile::LowVision),
            "first\n\n\nsecond\n"
        );
    }
}
