//! Declarative color palettes for neutral semantic terminal roles.

use crate::style::{SemanticRole, StyleSpec, TerminalColor, BOLD, BRIGHT, DIM, REVERSE, UNDERLINE};

const ROLE_COUNT: usize = SemanticRole::COUNT;

/// Color palette selected independently from report presentation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PaletteProfile {
    /// ANSI-free terminal output.
    Plain,
    /// Familiar semantic terminal colors.
    Standard,
    /// Bright terminal colors selected for stronger visual separation.
    HighContrast,
    /// Palette avoiding red-green distinctions for reduced red perception.
    Protanopia,
    /// Palette avoiding red-green distinctions for reduced green perception.
    Deuteranopia,
    /// Palette avoiding documented tritanopia-confusing color pairs.
    Tritanopia,
    /// Non-chromatic palette using typographic attributes.
    Achromatopsia,
}

impl PaletteProfile {
    /// Every supported palette, including the ANSI-free default.
    pub const ALL: [Self; 7] = [
        Self::Plain,
        Self::Standard,
        Self::HighContrast,
        Self::Protanopia,
        Self::Deuteranopia,
        Self::Tritanopia,
        Self::Achromatopsia,
    ];
}

pub(crate) struct PaletteSpec {
    pub(crate) styles: [StyleSpec; ROLE_COUNT],
}

pub(crate) fn specification(profile: PaletteProfile) -> &'static PaletteSpec {
    match profile {
        PaletteProfile::Plain => &PLAIN,
        PaletteProfile::Standard => &STANDARD,
        PaletteProfile::HighContrast => &HIGH_CONTRAST,
        PaletteProfile::Protanopia => &PROTANOPIA,
        PaletteProfile::Deuteranopia => &DEUTERANOPIA,
        PaletteProfile::Tritanopia => &TRITANOPIA,
        PaletteProfile::Achromatopsia => &ACHROMATOPSIA,
    }
}

const P: StyleSpec = StyleSpec::plain();
const fn c(color: TerminalColor, attributes: u8) -> StyleSpec {
    StyleSpec::color(color, attributes)
}
const fn a(attributes: u8) -> StyleSpec {
    StyleSpec::attributes(attributes)
}

const PLAIN: PaletteSpec = PaletteSpec {
    styles: [P; ROLE_COUNT],
};

// Entries follow `SemanticRole`; application vocabulary never enters a palette.
const STANDARD: PaletteSpec = PaletteSpec {
    styles: [
        c(TerminalColor::White, BOLD),
        c(TerminalColor::Cyan, BOLD),
        P,
        c(TerminalColor::Cyan, 0),
        c(TerminalColor::White, DIM),
        c(TerminalColor::Blue, 0),
        c(TerminalColor::Magenta, UNDERLINE),
        c(TerminalColor::Cyan, BOLD),
        c(TerminalColor::Blue, BOLD),
        c(TerminalColor::Magenta, BOLD),
        c(TerminalColor::Cyan, BOLD),
        c(TerminalColor::Yellow, BOLD),
        c(TerminalColor::Green, BOLD),
        c(TerminalColor::Red, BOLD | UNDERLINE),
        c(TerminalColor::Yellow, BOLD),
        c(TerminalColor::Yellow, UNDERLINE),
        c(TerminalColor::Red, BOLD),
        c(TerminalColor::White, 0),
        c(TerminalColor::Cyan, BOLD),
        c(TerminalColor::Green, BOLD),
        c(TerminalColor::Blue, DIM),
    ],
};

const HIGH_CONTRAST: PaletteSpec = PaletteSpec {
    styles: [
        c(TerminalColor::White, BOLD | BRIGHT),
        c(TerminalColor::Cyan, BOLD | BRIGHT | UNDERLINE),
        c(TerminalColor::Cyan, BRIGHT),
        c(TerminalColor::White, BOLD),
        c(TerminalColor::White, BRIGHT),
        c(TerminalColor::Cyan, BRIGHT | UNDERLINE),
        c(TerminalColor::Magenta, BOLD | UNDERLINE),
        c(TerminalColor::Cyan, BOLD | BRIGHT),
        c(TerminalColor::White, BOLD | BRIGHT),
        c(TerminalColor::Magenta, BOLD | BRIGHT),
        c(TerminalColor::Cyan, BOLD | BRIGHT),
        c(TerminalColor::Yellow, BOLD | BRIGHT),
        c(TerminalColor::White, BOLD | BRIGHT),
        c(TerminalColor::Yellow, BOLD | REVERSE),
        c(TerminalColor::Cyan, BOLD | UNDERLINE),
        c(TerminalColor::White, UNDERLINE),
        c(TerminalColor::Yellow, BOLD | UNDERLINE),
        c(TerminalColor::White, BRIGHT),
        c(TerminalColor::Cyan, BOLD | BRIGHT),
        c(TerminalColor::White, BOLD | BRIGHT),
        c(TerminalColor::White, BRIGHT),
    ],
};

const PROTANOPIA: PaletteSpec = PaletteSpec {
    styles: [
        c(TerminalColor::White, BOLD),
        c(TerminalColor::Cyan, BOLD),
        P,
        c(TerminalColor::Cyan, 0),
        c(TerminalColor::White, DIM),
        c(TerminalColor::Blue, UNDERLINE),
        c(TerminalColor::Magenta, UNDERLINE),
        c(TerminalColor::Cyan, BOLD),
        c(TerminalColor::Blue, BOLD),
        c(TerminalColor::Magenta, BOLD),
        c(TerminalColor::Cyan, BOLD),
        c(TerminalColor::Yellow, BOLD),
        c(TerminalColor::Blue, BOLD),
        c(TerminalColor::Yellow, BOLD | REVERSE | UNDERLINE),
        c(TerminalColor::Magenta, BOLD),
        c(TerminalColor::Cyan, UNDERLINE),
        c(TerminalColor::Yellow, BOLD),
        c(TerminalColor::White, 0),
        c(TerminalColor::Cyan, BOLD),
        c(TerminalColor::Blue, BOLD),
        c(TerminalColor::White, DIM),
    ],
};

const DEUTERANOPIA: PaletteSpec = PaletteSpec {
    styles: [
        c(TerminalColor::White, BOLD),
        c(TerminalColor::Blue, BOLD),
        P,
        c(TerminalColor::Cyan, 0),
        c(TerminalColor::White, DIM),
        c(TerminalColor::Blue, UNDERLINE),
        c(TerminalColor::Magenta, UNDERLINE),
        c(TerminalColor::Cyan, BOLD),
        c(TerminalColor::Blue, BOLD),
        c(TerminalColor::Magenta, BOLD),
        c(TerminalColor::Cyan, BOLD),
        c(TerminalColor::Yellow, BOLD),
        c(TerminalColor::Cyan, BOLD),
        c(TerminalColor::Magenta, BOLD | REVERSE | UNDERLINE),
        c(TerminalColor::Yellow, BOLD),
        c(TerminalColor::Blue, UNDERLINE),
        c(TerminalColor::Magenta, BOLD),
        c(TerminalColor::White, 0),
        c(TerminalColor::Cyan, BOLD),
        c(TerminalColor::Cyan, BOLD),
        c(TerminalColor::White, DIM),
    ],
};

const TRITANOPIA: PaletteSpec = PaletteSpec {
    styles: [
        c(TerminalColor::White, BOLD),
        c(TerminalColor::White, BOLD | UNDERLINE),
        P,
        c(TerminalColor::White, 0),
        c(TerminalColor::White, DIM),
        c(TerminalColor::White, UNDERLINE),
        c(TerminalColor::White, REVERSE),
        c(TerminalColor::White, BOLD),
        c(TerminalColor::White, BOLD | UNDERLINE),
        c(TerminalColor::White, BOLD | REVERSE),
        c(TerminalColor::White, UNDERLINE),
        c(TerminalColor::Red, BOLD),
        c(TerminalColor::White, BOLD),
        c(TerminalColor::Red, BOLD | REVERSE | UNDERLINE),
        c(TerminalColor::White, BOLD | UNDERLINE),
        c(TerminalColor::White, REVERSE),
        c(TerminalColor::Red, BOLD | UNDERLINE),
        c(TerminalColor::White, 0),
        c(TerminalColor::White, BOLD),
        c(TerminalColor::White, BOLD),
        c(TerminalColor::White, DIM),
    ],
};

const ACHROMATOPSIA: PaletteSpec = PaletteSpec {
    styles: [
        a(BOLD),
        a(BOLD | UNDERLINE),
        P,
        a(UNDERLINE),
        a(DIM),
        a(UNDERLINE),
        a(REVERSE),
        a(BOLD),
        a(BOLD | UNDERLINE),
        a(BOLD | REVERSE),
        a(UNDERLINE),
        a(BOLD | REVERSE),
        a(BOLD),
        a(BOLD | REVERSE | UNDERLINE),
        a(BOLD | UNDERLINE),
        a(REVERSE),
        a(BOLD | UNDERLINE),
        a(BOLD),
        a(BOLD),
        a(BOLD | REVERSE),
        a(DIM),
    ],
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accessibility_invariants_are_explicit() {
        assert!(HIGH_CONTRAST
            .styles
            .iter()
            .all(|style| !style.has_attribute(crate::StyleAttribute::Dim)));
        assert!(HIGH_CONTRAST
            .styles
            .iter()
            .any(|style| style.has_attribute(crate::StyleAttribute::Bright)));
        assert!(ACHROMATOPSIA
            .styles
            .iter()
            .all(|style| style.foreground().is_none()));
        for palette in [&PROTANOPIA, &DEUTERANOPIA] {
            assert!(palette.styles.iter().all(|style| !matches!(
                style.foreground(),
                Some(TerminalColor::Red | TerminalColor::Green)
            )));
        }
        assert!(TRITANOPIA.styles.iter().all(|style| !matches!(
            style.foreground(),
            Some(
                TerminalColor::Blue
                    | TerminalColor::Green
                    | TerminalColor::Yellow
                    | TerminalColor::Magenta
            )
        )));
    }
}
