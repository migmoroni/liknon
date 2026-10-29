//! Validation-specific roles and layout mapped to the shared terminal theme.

pub(crate) use liknon_cli_ui::clap;
pub(crate) use liknon_cli_ui::{PaletteProfile, PresentationProfile};
use liknon_cli_ui::{SemanticRole, Theme as SharedTheme};

const FRAME_WIDTH: usize = 78;

/// Validation-local presentation over the shared semantic terminal theme.
#[derive(Clone)]
pub struct Theme {
    shared: SharedTheme,
    progress_width: usize,
    execution_gap: usize,
}

impl Theme {
    /// Creates the default ANSI-free validation theme.
    pub fn plain() -> Self {
        Self::resolve(PaletteProfile::Plain, PresentationProfile::Standard)
    }

    /// Resolves shared styles and validation-only execution metrics.
    pub fn resolve(palette: PaletteProfile, presentation: PresentationProfile) -> Self {
        let (progress_width, execution_gap) = match presentation {
            PresentationProfile::Standard => (14, 0),
            PresentationProfile::LowVision => (22, 2),
        };
        Self {
            shared: SharedTheme::resolve(palette, presentation),
            progress_width,
            execution_gap,
        }
    }

    pub(crate) fn paint(&self, role: Role, text: impl std::fmt::Display) -> String {
        self.shared.paint(role.semantic(), text)
    }

    pub(crate) fn indicatif_modifier(&self, role: Role) -> String {
        self.shared.indicatif_modifier(role.semantic())
    }

    pub(crate) const fn progress_width(&self) -> usize {
        self.progress_width
    }

    pub(crate) fn section_gap(&self) -> usize {
        self.shared.metrics().section_gap()
    }

    pub(crate) fn item_gap(&self) -> usize {
        self.shared.metrics().item_gap()
    }

    pub(crate) const fn execution_gap(&self) -> usize {
        self.execution_gap
    }

    pub(crate) fn frame_rule(&self, edge: char, label: &str, label_role: Role) -> String {
        let leading = format!("{edge}─ ");
        let occupied = leading.chars().count() + label.chars().count() + 1;
        let trailing = format!(" {}", "─".repeat(FRAME_WIDTH.saturating_sub(occupied)));
        format!(
            "{}{}{}",
            self.paint(Role::Border, leading),
            self.paint(label_role, label),
            self.paint(Role::Border, trailing)
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Role {
    Heading,
    Section,
    Border,
    Metadata,
    Duration,
    Path,
    Shared,
    Tool,
    Group,
    Suite,
    Check,
    Gate,
    Success,
    Failure,
    Blocked,
    Skipped,
    DiagnosticLabel,
    DiagnosticContent,
    ActiveSpinner,
    ProgressComplete,
    ProgressPending,
}

impl Role {
    #[cfg(test)]
    const ALL: [Self; 21] = [
        Self::Heading,
        Self::Section,
        Self::Border,
        Self::Metadata,
        Self::Duration,
        Self::Path,
        Self::Shared,
        Self::Tool,
        Self::Group,
        Self::Suite,
        Self::Check,
        Self::Gate,
        Self::Success,
        Self::Failure,
        Self::Blocked,
        Self::Skipped,
        Self::DiagnosticLabel,
        Self::DiagnosticContent,
        Self::ActiveSpinner,
        Self::ProgressComplete,
        Self::ProgressPending,
    ];

    const fn semantic(self) -> SemanticRole {
        match self {
            Self::Heading => SemanticRole::Heading,
            Self::Section => SemanticRole::Section,
            Self::Border => SemanticRole::Border,
            Self::Metadata => SemanticRole::Metadata,
            Self::Duration => SemanticRole::Subdued,
            Self::Path => SemanticRole::Path,
            Self::Shared => SemanticRole::Reference,
            Self::Tool => SemanticRole::AccentPrimary,
            Self::Group => SemanticRole::AccentSecondary,
            Self::Suite => SemanticRole::AccentTertiary,
            Self::Check => SemanticRole::AccentQuaternary,
            Self::Gate => SemanticRole::Warning,
            Self::Success => SemanticRole::Success,
            Self::Failure => SemanticRole::Failure,
            Self::Blocked => SemanticRole::Blocked,
            Self::Skipped => SemanticRole::Skipped,
            Self::DiagnosticLabel => SemanticRole::DiagnosticLabel,
            Self::DiagnosticContent => SemanticRole::DiagnosticContent,
            Self::ActiveSpinner => SemanticRole::Active,
            Self::ProgressComplete => SemanticRole::Complete,
            Self::ProgressPending => SemanticRole::Pending,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use console::{measure_text_width, strip_ansi_codes};

    #[test]
    fn every_validation_role_maps_to_shared_semantics() {
        for palette in PaletteProfile::ALL {
            for presentation in PresentationProfile::ALL {
                let theme = Theme::resolve(palette, presentation);
                for role in Role::ALL {
                    assert_eq!(strip_ansi_codes(&theme.paint(role, "token")), "token");
                }
            }
        }
    }

    #[test]
    fn validation_metrics_remain_local_and_stable() {
        let regular = Theme::plain();
        assert_eq!(regular.progress_width(), 14);
        assert_eq!(regular.section_gap(), 0);
        assert_eq!(regular.item_gap(), 0);
        assert_eq!(regular.execution_gap(), 0);

        let low_vision = Theme::resolve(PaletteProfile::Plain, PresentationProfile::LowVision);
        assert_eq!(low_vision.progress_width(), 22);
        assert_eq!(low_vision.section_gap(), 2);
        assert_eq!(low_vision.item_gap(), 1);
        assert_eq!(low_vision.execution_gap(), 2);
    }

    #[test]
    fn frame_width_is_stable_for_all_profiles() {
        for palette in PaletteProfile::ALL {
            for presentation in PresentationProfile::ALL {
                let theme = Theme::resolve(palette, presentation);
                let line = theme.frame_rule('├', "Tools", Role::Section);
                assert_eq!(measure_text_width(&line), FRAME_WIDTH);
                assert_eq!(strip_ansi_codes(&line).chars().count(), FRAME_WIDTH);
            }
        }
    }
}
