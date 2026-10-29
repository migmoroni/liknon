//! Terminal-output policy independent from application error taxonomies.

use crate::VisualSelection;

/// Output channel semantics relevant to terminal styling.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutputKind {
    /// Human-readable terminal output may use an explicit visual profile.
    Human,
    /// Machine-readable output must remain ANSI-free and layout-independent.
    Machine,
}

/// Returns whether a visual selection is valid for one output kind.
///
/// Applications own the user-facing error and exit code when this returns
/// `false`.
pub const fn visual_options_are_compatible(selection: VisualSelection, output: OutputKind) -> bool {
    matches!(output, OutputKind::Human) || !selection.is_explicit()
}

/// Returns whether the conventional `NO_COLOR` variable is present and non-empty.
pub fn no_color_is_set() -> bool {
    std::env::var_os("NO_COLOR").is_some_and(|value| !value.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PaletteArgument, VisualSelection};

    #[test]
    fn machine_output_rejects_only_explicit_visual_controls() {
        assert!(visual_options_are_compatible(
            VisualSelection::default(),
            OutputKind::Machine
        ));
        assert!(!visual_options_are_compatible(
            VisualSelection::new(Some(PaletteArgument::Standard), None),
            OutputKind::Machine
        ));
    }
}
