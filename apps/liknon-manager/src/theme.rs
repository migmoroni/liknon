//! Manager-owned mappings from resource concepts to shared semantic styles.

use liknon_cli_ui::{SemanticRole, Theme};

#[derive(Clone)]
pub(crate) struct ManagerTheme {
    shared: Theme,
}

impl ManagerTheme {
    pub(crate) const fn new(shared: Theme) -> Self {
        Self { shared }
    }

    pub(crate) fn paint(&self, role: SemanticRole, text: impl std::fmt::Display) -> String {
        self.shared.paint(role, text)
    }

    pub(crate) fn section_gap(&self) -> usize {
        self.shared.metrics().section_gap()
    }

    pub(crate) fn item_gap(&self) -> usize {
        self.shared.metrics().item_gap()
    }
}
