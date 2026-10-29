//! Shared visual command-line values and deterministic pre-parse resolution.

use std::ffi::{OsStr, OsString};

use clap::{Args, ValueEnum};

use crate::{PaletteProfile, PresentationProfile};

/// Standard help text for the global color option.
pub const COLOR_HELP: &str = "Enables ANSI color in human CLI output";

/// Complete help text for the global color option.
pub const COLOR_LONG_HELP: &str = "Enables ANSI color in human CLI output.\n\nWith -h or --help, colors the requested help. During supported human-output operations, colors semantic terminal roles. Without this flag, output contains no ANSI color unless an explicit presentation adds typographic emphasis. Bare --color selects the standard palette; named palettes require --color=<PALETTE>. An explicit command-line palette overrides NO_COLOR. Status and hierarchy must never rely on color alone.";

/// Standard help text for the global presentation option.
pub const PRESENTATION_HELP: &str = "Selects the layout and emphasis of human CLI output";

/// Complete help text for the global presentation option.
pub const PRESENTATION_LONG_HELP: &str = "Selects the layout and emphasis of human CLI output independently from color.\n\nWith -h or --help, changes the requested help presentation. During supported human-output operations, changes terminal layout and emphasis. Standard uses the compact layout. Low-vision expands spacing and removes dim styling. Named modes require --presentation=<MODE>.";

/// Palette value accepted by the shared `--color` option.
#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum PaletteArgument {
    /// Familiar semantic terminal colors.
    #[value(help = "Standard semantic terminal palette")]
    Standard,
    /// Bright colors and stronger typographic separation.
    #[value(help = "Higher-contrast semantic terminal palette")]
    HighContrast,
    /// A palette that avoids red-green distinctions for reduced red perception.
    #[value(help = "Semantic palette adapted for protanopia")]
    Protanopia,
    /// A palette that avoids red-green distinctions for reduced green perception.
    #[value(help = "Semantic palette adapted for deuteranopia")]
    Deuteranopia,
    /// A palette that avoids documented tritanopia-confusing color pairs.
    #[value(help = "Semantic palette adapted for tritanopia")]
    Tritanopia,
    /// A non-chromatic palette based on typographic distinctions.
    #[value(help = "Semantic palette that does not rely on hue distinctions")]
    Achromatopsia,
}

/// Presentation value accepted by the shared `--presentation` option.
#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum PresentationArgument {
    /// Compact terminal layout and regular emphasis.
    #[value(help = "Compact terminal layout and standard emphasis")]
    Standard,
    /// Expanded spacing and stronger non-color emphasis.
    #[value(help = "Expanded spacing and stronger emphasis for low-vision readability")]
    LowVision,
}

/// Reusable global visual arguments for a human-facing CLI.
#[derive(Args, Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VisualOptions {
    /// Explicit semantic color palette.
    #[arg(
        long,
        global = true,
        value_enum,
        num_args = 0..=1,
        default_missing_value = "standard",
        require_equals = true,
        value_name = "PALETTE",
        help = COLOR_HELP,
        long_help = COLOR_LONG_HELP
    )]
    pub color: Option<PaletteArgument>,

    /// Explicit layout and emphasis profile.
    #[arg(
        long,
        global = true,
        value_enum,
        require_equals = true,
        value_name = "MODE",
        help = PRESENTATION_HELP,
        long_help = PRESENTATION_LONG_HELP
    )]
    pub presentation: Option<PresentationArgument>,
}

impl VisualOptions {
    /// Returns the resolved terminal selection represented by parsed options.
    pub fn selection(self) -> VisualSelection {
        VisualSelection::new(self.color, self.presentation)
    }

    /// Returns whether either visual execution option was explicitly supplied.
    pub const fn is_explicit(self) -> bool {
        self.color.is_some() || self.presentation.is_some()
    }
}

/// Visual arguments restricted to a root command rather than delegated
/// subcommands.
#[derive(Args, Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RootVisualOptions {
    /// Explicit semantic color palette for the root-owned surface.
    #[arg(
        long,
        value_enum,
        num_args = 0..=1,
        default_missing_value = "standard",
        require_equals = true,
        value_name = "PALETTE",
        help = COLOR_HELP,
        long_help = COLOR_LONG_HELP
    )]
    pub color: Option<PaletteArgument>,

    /// Explicit root-owned layout and emphasis profile.
    #[arg(
        long,
        value_enum,
        require_equals = true,
        value_name = "MODE",
        help = PRESENTATION_HELP,
        long_help = PRESENTATION_LONG_HELP
    )]
    pub presentation: Option<PresentationArgument>,
}

impl RootVisualOptions {
    /// Returns the resolved terminal selection represented by parsed options.
    pub fn selection(self) -> VisualSelection {
        VisualSelection::new(self.color, self.presentation)
    }
}

/// A resolved palette and presentation plus their explicitness.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VisualSelection {
    explicit_palette: Option<PaletteArgument>,
    explicit_presentation: Option<PresentationArgument>,
    palette: PaletteProfile,
    presentation: PresentationProfile,
}

impl VisualSelection {
    /// Resolves parsed visual options using the shared default/plain policy.
    pub fn new(
        palette: Option<PaletteArgument>,
        presentation: Option<PresentationArgument>,
    ) -> Self {
        Self {
            explicit_palette: palette,
            explicit_presentation: presentation,
            palette: palette.map(Into::into).unwrap_or(PaletteProfile::Plain),
            presentation: presentation
                .map(Into::into)
                .unwrap_or(PresentationProfile::Standard),
        }
    }

    /// Resolves valid visual values before Clap renders help or diagnostics.
    ///
    /// Invalid values intentionally remain unresolved so Clap can report them
    /// using the plain default theme.
    pub fn from_args(arguments: &[OsString]) -> Self {
        Self::from_argument_iter(arguments.iter().skip(1).take_while(|value| *value != "--"))
    }

    /// Resolves visual values only before the first non-option token.
    ///
    /// Launchers use this form so options after a capability namespace remain
    /// owned by that delegated capability.
    pub fn from_root_args(arguments: &[OsString]) -> Self {
        Self::from_argument_iter(arguments.iter().skip(1).take_while(|value| {
            value
                .to_str()
                .is_some_and(|value| value != "--" && value.starts_with('-'))
        }))
    }

    fn from_argument_iter<'a>(arguments: impl Iterator<Item = &'a OsString>) -> Self {
        let mut palette = None;
        let mut presentation = None;

        for argument in arguments {
            let Some(argument) = argument.to_str() else {
                continue;
            };
            if palette.is_none() {
                if argument == "--color" {
                    palette = Some(PaletteArgument::Standard);
                } else if let Some(value) = argument.strip_prefix("--color=") {
                    palette = PaletteArgument::from_str(value, true).ok();
                }
            }
            if presentation.is_none() {
                if let Some(value) = argument.strip_prefix("--presentation=") {
                    presentation = PresentationArgument::from_str(value, true).ok();
                }
            }
        }

        Self::new(palette, presentation)
    }

    /// Returns the selected palette, including the ANSI-free default.
    pub const fn palette(self) -> PaletteProfile {
        self.palette
    }

    /// Returns the selected presentation profile.
    pub const fn presentation(self) -> PresentationProfile {
        self.presentation
    }

    /// Returns the explicitly supplied palette, if any.
    pub const fn explicit_palette(self) -> Option<PaletteArgument> {
        self.explicit_palette
    }

    /// Returns the explicitly supplied presentation, if any.
    pub const fn explicit_presentation(self) -> Option<PresentationArgument> {
        self.explicit_presentation
    }

    /// Returns whether either visual execution option was explicitly supplied.
    pub const fn is_explicit(self) -> bool {
        self.explicit_palette.is_some() || self.explicit_presentation.is_some()
    }
}

impl Default for VisualSelection {
    fn default() -> Self {
        Self::new(None, None)
    }
}

impl From<PaletteArgument> for PaletteProfile {
    fn from(value: PaletteArgument) -> Self {
        match value {
            PaletteArgument::Standard => Self::Standard,
            PaletteArgument::HighContrast => Self::HighContrast,
            PaletteArgument::Protanopia => Self::Protanopia,
            PaletteArgument::Deuteranopia => Self::Deuteranopia,
            PaletteArgument::Tritanopia => Self::Tritanopia,
            PaletteArgument::Achromatopsia => Self::Achromatopsia,
        }
    }
}

impl From<PresentationArgument> for PresentationProfile {
    fn from(value: PresentationArgument) -> Self {
        match value {
            PresentationArgument::Standard => Self::Standard,
            PresentationArgument::LowVision => Self::LowVision,
        }
    }
}

/// Reports whether a recognized named visual value was supplied as a detached
/// argument even though the public grammar requires `=`.
pub fn has_detached_named_value(
    arguments: &[OsString],
    option: &str,
    accepts: impl Fn(&str) -> bool,
) -> bool {
    arguments
        .windows(2)
        .any(|pair| pair[0] == OsStr::new(option) && pair[1].to_str().is_some_and(&accepts))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(values: &[&str]) -> Vec<OsString> {
        values.iter().map(OsString::from).collect()
    }

    #[test]
    fn defaults_are_plain_and_standard() {
        let selection = VisualSelection::from_args(&args(&["example", "run"]));
        assert_eq!(selection.palette(), PaletteProfile::Plain);
        assert_eq!(selection.presentation(), PresentationProfile::Standard);
        assert!(!selection.is_explicit());
    }

    #[test]
    fn explicit_values_resolve_before_command_parsing() {
        let selection = VisualSelection::from_args(&args(&[
            "example",
            "run",
            "--color=deuteranopia",
            "--presentation=low-vision",
        ]));
        assert_eq!(selection.palette(), PaletteProfile::Deuteranopia);
        assert_eq!(selection.presentation(), PresentationProfile::LowVision);
        assert!(selection.is_explicit());
    }

    #[test]
    fn argument_separator_stops_visual_preparse() {
        let selection =
            VisualSelection::from_args(&args(&["example", "run", "--", "--color=standard"]));
        assert_eq!(selection, VisualSelection::default());
    }

    #[test]
    fn root_preparse_leaves_delegated_options_untouched() {
        let selection =
            VisualSelection::from_root_args(&args(&["launcher", "capability", "--color=standard"]));
        assert_eq!(selection, VisualSelection::default());
    }
}
