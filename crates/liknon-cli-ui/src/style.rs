//! Styling primitives and product-neutral semantic roles.

use console::{Attribute, Color, Style};

use crate::{palette, presentation, PaletteProfile, PresentationProfile};

pub(crate) const BOLD: u8 = 1 << 0;
pub(crate) const DIM: u8 = 1 << 1;
pub(crate) const UNDERLINE: u8 = 1 << 2;
pub(crate) const REVERSE: u8 = 1 << 3;
pub(crate) const BRIGHT: u8 = 1 << 4;

/// Product-neutral semantic role resolved by every shared palette.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum SemanticRole {
    /// Primary page or report heading.
    Heading,
    /// Section heading.
    Section,
    /// Borders, tree branches, and separators.
    Border,
    /// Labels and secondary metadata.
    Metadata,
    /// De-emphasized values such as durations.
    Subdued,
    /// File-system paths and command placeholders.
    Path,
    /// References, reuse markers, and related links.
    Reference,
    /// Primary product-selected accent.
    AccentPrimary,
    /// Secondary product-selected accent.
    AccentSecondary,
    /// Tertiary product-selected accent.
    AccentTertiary,
    /// Quaternary product-selected accent.
    AccentQuaternary,
    /// Warning or caution state.
    Warning,
    /// Successful state.
    Success,
    /// Failed state.
    Failure,
    /// Blocked state.
    Blocked,
    /// Skipped state.
    Skipped,
    /// Diagnostic label.
    DiagnosticLabel,
    /// Diagnostic body content.
    DiagnosticContent,
    /// Active or in-progress state.
    Active,
    /// Completed progress state.
    Complete,
    /// Pending progress state.
    Pending,
}

impl SemanticRole {
    pub(crate) const COUNT: usize = 21;

    /// Every semantic role in stable palette-table order.
    pub const ALL: [Self; Self::COUNT] = [
        Self::Heading,
        Self::Section,
        Self::Border,
        Self::Metadata,
        Self::Subdued,
        Self::Path,
        Self::Reference,
        Self::AccentPrimary,
        Self::AccentSecondary,
        Self::AccentTertiary,
        Self::AccentQuaternary,
        Self::Warning,
        Self::Success,
        Self::Failure,
        Self::Blocked,
        Self::Skipped,
        Self::DiagnosticLabel,
        Self::DiagnosticContent,
        Self::Active,
        Self::Complete,
        Self::Pending,
    ];
}

/// Terminal foreground colors used by palette specifications.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalColor {
    /// Red terminal foreground.
    Red,
    /// Green terminal foreground.
    Green,
    /// Yellow terminal foreground.
    Yellow,
    /// Blue terminal foreground.
    Blue,
    /// Magenta terminal foreground.
    Magenta,
    /// Cyan terminal foreground.
    Cyan,
    /// White terminal foreground.
    White,
}

impl TerminalColor {
    const fn console(self) -> Color {
        match self {
            Self::Red => Color::Red,
            Self::Green => Color::Green,
            Self::Yellow => Color::Yellow,
            Self::Blue => Color::Blue,
            Self::Magenta => Color::Magenta,
            Self::Cyan => Color::Cyan,
            Self::White => Color::White,
        }
    }

    const fn indicatif(self) -> &'static str {
        match self {
            Self::Red => "red",
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Blue => "blue",
            Self::Magenta => "magenta",
            Self::Cyan => "cyan",
            Self::White => "white",
        }
    }
}

/// Terminal attribute queried by conformance tests and local adapters.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StyleAttribute {
    /// Bold emphasis.
    Bold,
    /// Dim emphasis.
    Dim,
    /// Underlined emphasis.
    Underline,
    /// Reversed foreground and background.
    Reverse,
    /// Bright foreground intensity.
    Bright,
}

impl StyleAttribute {
    const fn mask(self) -> u8 {
        match self {
            Self::Bold => BOLD,
            Self::Dim => DIM,
            Self::Underline => UNDERLINE,
            Self::Reverse => REVERSE,
            Self::Bright => BRIGHT,
        }
    }
}

/// Resolved ANSI-independent style specification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StyleSpec {
    foreground: Option<TerminalColor>,
    attributes: u8,
}

impl StyleSpec {
    pub(crate) const fn plain() -> Self {
        Self {
            foreground: None,
            attributes: 0,
        }
    }

    pub(crate) const fn color(foreground: TerminalColor, attributes: u8) -> Self {
        Self {
            foreground: Some(foreground),
            attributes,
        }
    }

    pub(crate) const fn attributes(attributes: u8) -> Self {
        Self {
            foreground: None,
            attributes,
        }
    }

    /// Returns the resolved terminal foreground, if any.
    pub const fn foreground(self) -> Option<TerminalColor> {
        self.foreground
    }

    /// Returns whether this style contains one terminal attribute.
    pub const fn has_attribute(self, attribute: StyleAttribute) -> bool {
        self.attributes & attribute.mask() != 0
    }

    pub(crate) fn merge(self, modifier: StyleModifier) -> Self {
        let attributes = (self.attributes & !modifier.remove) | modifier.add;
        Self {
            foreground: self.foreground,
            attributes,
        }
    }

    pub(crate) fn console_style(self) -> Style {
        let mut style = Style::new().force_styling(self != Self::plain());
        if let Some(color) = self.foreground {
            style = style.fg(color.console());
        }
        for (flag, attribute) in [
            (BOLD, Attribute::Bold),
            (DIM, Attribute::Dim),
            (UNDERLINE, Attribute::Underlined),
            (REVERSE, Attribute::Reverse),
        ] {
            if self.attributes & flag != 0 {
                style = style.attr(attribute);
            }
        }
        if self.attributes & BRIGHT != 0 {
            style = style.bright();
        }
        style
    }

    pub(crate) fn indicatif_modifier(self) -> String {
        let mut parts = Vec::new();
        if let Some(color) = self.foreground {
            parts.push(color.indicatif());
        }
        for (flag, name) in [
            (BOLD, "bold"),
            (DIM, "dim"),
            (UNDERLINE, "underlined"),
            (REVERSE, "reverse"),
            (BRIGHT, "bright"),
        ] {
            if self.attributes & flag != 0 {
                parts.push(name);
            }
        }
        if parts.is_empty() {
            String::new()
        } else {
            format!(".{}", parts.join("."))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct StyleModifier {
    remove: u8,
    add: u8,
}

impl StyleModifier {
    pub(crate) const fn none() -> Self {
        Self { remove: 0, add: 0 }
    }

    pub(crate) const fn new(remove: u8, add: u8) -> Self {
        Self { remove, add }
    }
}

/// Immutable combination of one palette and one presentation profile.
#[derive(Clone)]
pub struct Theme {
    styles: [StyleSpec; SemanticRole::COUNT],
    palette: PaletteProfile,
    presentation: PresentationProfile,
}

impl Theme {
    /// Creates the default theme with no ANSI output and regular density.
    pub fn plain() -> Self {
        Self::resolve(PaletteProfile::Plain, PresentationProfile::Standard)
    }

    /// Resolves palette styles and presentation modifiers deterministically.
    pub fn resolve(palette: PaletteProfile, presentation: PresentationProfile) -> Self {
        let palette_spec = palette::specification(palette);
        let presentation_spec = presentation::specification(presentation);
        let styles = std::array::from_fn(|index| {
            palette_spec.styles[index].merge(presentation_spec.modifiers[index])
        });
        Self {
            styles,
            palette,
            presentation,
        }
    }

    /// Returns the selected palette.
    pub const fn palette(&self) -> PaletteProfile {
        self.palette
    }

    /// Returns the selected presentation profile.
    pub const fn presentation(&self) -> PresentationProfile {
        self.presentation
    }

    /// Returns one resolved ANSI-independent style.
    pub const fn style(&self, role: SemanticRole) -> StyleSpec {
        self.styles[role as usize]
    }

    /// Applies one semantic role while preserving the original text.
    pub fn paint(&self, role: SemanticRole, text: impl std::fmt::Display) -> String {
        let style = self.style(role);
        if style == StyleSpec::plain() {
            text.to_string()
        } else {
            style.console_style().apply_to(text).to_string()
        }
    }

    /// Returns an Indicatif-compatible style suffix without depending on Indicatif.
    pub fn indicatif_modifier(&self, role: SemanticRole) -> String {
        self.style(role).indicatif_modifier()
    }

    /// Returns generic spacing metrics for the selected presentation.
    pub const fn metrics(&self) -> crate::PresentationMetrics {
        self.presentation.metrics()
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::plain()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use console::strip_ansi_codes;

    #[test]
    fn every_profile_combination_resolves_every_role_deterministically() {
        for palette in PaletteProfile::ALL {
            for presentation in PresentationProfile::ALL {
                let first = Theme::resolve(palette, presentation);
                let second = Theme::resolve(palette, presentation);
                for role in SemanticRole::ALL {
                    assert_eq!(first.style(role), second.style(role));
                    assert_eq!(strip_ansi_codes(&first.paint(role, "token")), "token");
                }
            }
        }
    }

    #[test]
    fn plain_standard_is_ansi_free_and_low_vision_removes_dim() {
        for role in SemanticRole::ALL {
            assert!(!Theme::plain().paint(role, "token").contains('\u{1b}'));
        }
        for palette in PaletteProfile::ALL {
            let theme = Theme::resolve(palette, PresentationProfile::LowVision);
            assert!(SemanticRole::ALL
                .into_iter()
                .all(|role| !theme.style(role).has_attribute(StyleAttribute::Dim)));
        }
    }
}
