//! Presentation modes independent from concrete terminal hues.

use crate::style::{SemanticRole, StyleModifier, BOLD, BRIGHT, DIM, UNDERLINE};

const ROLE_COUNT: usize = SemanticRole::COUNT;

/// Layout and emphasis selected independently from color.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PresentationProfile {
    /// Regular density and emphasis.
    Standard,
    /// Expanded spacing and stronger non-color emphasis.
    LowVision,
}

impl PresentationProfile {
    /// Every supported presentation profile.
    pub const ALL: [Self; 2] = [Self::Standard, Self::LowVision];

    /// Generic spacing metrics for human terminal sections and items.
    pub const fn metrics(self) -> PresentationMetrics {
        match self {
            Self::Standard => PresentationMetrics {
                section_gap: 0,
                item_gap: 0,
            },
            Self::LowVision => PresentationMetrics {
                section_gap: 2,
                item_gap: 1,
            },
        }
    }
}

/// Generic spacing owned by a presentation profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PresentationMetrics {
    section_gap: usize,
    item_gap: usize,
}

impl PresentationMetrics {
    /// Blank rows separating major terminal sections.
    pub const fn section_gap(self) -> usize {
        self.section_gap
    }

    /// Blank rows separating neighboring terminal items.
    pub const fn item_gap(self) -> usize {
        self.item_gap
    }
}

pub(crate) struct PresentationSpec {
    pub(crate) modifiers: [StyleModifier; ROLE_COUNT],
}

pub(crate) fn specification(profile: PresentationProfile) -> &'static PresentationSpec {
    match profile {
        PresentationProfile::Standard => &STANDARD,
        PresentationProfile::LowVision => &LOW_VISION,
    }
}

const N: StyleModifier = StyleModifier::none();
const fn m(remove: u8, add: u8) -> StyleModifier {
    StyleModifier::new(remove, add)
}

const STANDARD: PresentationSpec = PresentationSpec {
    modifiers: [N; ROLE_COUNT],
};

const LOW_VISION: PresentationSpec = PresentationSpec {
    modifiers: [
        m(DIM, BOLD | BRIGHT),
        m(DIM, BOLD | UNDERLINE | BRIGHT),
        m(DIM, BOLD),
        m(DIM, BOLD),
        m(DIM, BRIGHT),
        m(DIM, UNDERLINE | BRIGHT),
        m(DIM, BOLD | UNDERLINE),
        m(DIM, BOLD | BRIGHT),
        m(DIM, BOLD | BRIGHT),
        m(DIM, BOLD | BRIGHT),
        m(DIM, BOLD | BRIGHT),
        m(DIM, BOLD | BRIGHT),
        m(DIM, BOLD | BRIGHT),
        m(DIM, BOLD | UNDERLINE | BRIGHT),
        m(DIM, BOLD | UNDERLINE | BRIGHT),
        m(DIM, BOLD | UNDERLINE),
        m(DIM, BOLD | UNDERLINE),
        m(DIM, BRIGHT),
        m(DIM, BOLD | BRIGHT),
        m(DIM, BOLD | BRIGHT),
        m(DIM, BRIGHT),
    ],
};
