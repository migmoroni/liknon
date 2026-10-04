//! Clap adapter for the shared semantic theme.

use super::{
    style::{TerminalColor, BOLD, BRIGHT, DIM, REVERSE, UNDERLINE},
    PaletteProfile, PresentationProfile, Role, Theme,
};
use ::clap::{
    builder::styling::{AnsiColor, Color, Style, Styles},
    ColorChoice, Command,
};

/// Applies one resolved palette to help, usage, and parser diagnostics.
pub(crate) fn apply(command: Command, palette: PaletteProfile) -> Command {
    let theme = Theme::resolve(palette, PresentationProfile::Standard);
    let color_choice = if palette == PaletteProfile::Plain {
        ColorChoice::Never
    } else {
        ColorChoice::Always
    };

    command.color(color_choice).styles(styles(&theme))
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

    fn render(profile: PaletteProfile) -> String {
        let mut command = apply(
            Command::new("validator")
                .about("Validates a workspace")
                .arg(Arg::new("target").long("target").value_name("TARGET")),
            profile,
        );
        command.render_help().ansi().to_string()
    }

    #[test]
    fn plain_help_is_ansi_free() {
        assert!(!render(PaletteProfile::Plain).contains('\u{1b}'));
    }

    #[test]
    fn every_explicit_palette_preserves_help_content() {
        let plain = render(PaletteProfile::Plain);
        for palette in PaletteProfile::ALL
            .into_iter()
            .filter(|palette| *palette != PaletteProfile::Plain)
        {
            let rendered = render(palette);
            assert!(rendered.contains('\u{1b}'), "{palette:?}");
            assert_eq!(console::strip_ansi_codes(&rendered), plain, "{palette:?}");
        }
    }
}
