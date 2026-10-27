//! Help text for accepted command-line values.

pub(in crate::cli) const FORMAT_HUMAN: &str = "Human-readable terminal output";
pub(in crate::cli) const FORMAT_JSON: &str = "Machine-readable JSON without terminal styling";

pub(in crate::cli) const COLOR_STANDARD: &str = "Standard semantic terminal palette";
pub(in crate::cli) const COLOR_HIGH_CONTRAST: &str = "Higher-contrast semantic terminal palette";
pub(in crate::cli) const COLOR_PROTANOPIA: &str = "Semantic palette adapted for protanopia";
pub(in crate::cli) const COLOR_DEUTERANOPIA: &str = "Semantic palette adapted for deuteranopia";
pub(in crate::cli) const COLOR_TRITANOPIA: &str = "Semantic palette adapted for tritanopia";
pub(in crate::cli) const COLOR_ACHROMATOPSIA: &str =
    "Semantic palette that does not rely on hue distinctions";

pub(in crate::cli) const PRESENTATION_STANDARD: &str =
    "Compact terminal layout and standard emphasis";
pub(in crate::cli) const PRESENTATION_LOW_VISION: &str =
    "Expanded spacing and stronger emphasis for low-vision readability";

pub(in crate::cli) const CONTRACT_CONFIG: &str = "Declarative workspace configuration contract";
pub(in crate::cli) const CONTRACT_REPORT: &str = "Machine-readable validation report contract";
