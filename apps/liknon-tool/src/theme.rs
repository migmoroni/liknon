//! Tool-owned mappings from resource concepts to shared semantic styles.

use liknon_cli_ui::{SemanticRole, Theme};

#[derive(Clone)]
/// Tool adapter over the product-neutral semantic CLI theme.
pub(crate) struct ToolTheme {
    shared: Theme,
}

impl ToolTheme {
    /// Creates a Tool theme from the resolved shared theme.
    pub(crate) const fn new(shared: Theme) -> Self {
        Self { shared }
    }

    /// Paints one resource-domain value with a shared semantic role.
    pub(crate) fn paint(&self, role: SemanticRole, text: impl std::fmt::Display) -> String {
        self.shared.paint(role, text)
    }

    /// Returns the presentation-dependent spacing between report sections.
    pub(crate) fn section_gap(&self) -> usize {
        self.shared.metrics().section_gap()
    }

    /// Returns the presentation-dependent spacing between repeated items.
    pub(crate) fn item_gap(&self) -> usize {
        self.shared.metrics().item_gap()
    }
}
