//! Help text for validation execution.

pub(in crate::cli) const VALIDATE_ABOUT: &str =
    "Runs a configured group or suite after configuration and tool preflight";

pub(in crate::cli) const VALIDATE_LONG_ABOUT: &str = "Runs a configured group or suite after configuration and tool preflight.\n\nWithout a target, runs the configured default group.";

pub(in crate::cli) const TARGET: &str =
    "Group or suite ID; defaults to the configured default group";

pub(in crate::cli) const CHECK_ABOUT: &str =
    "Runs one reusable check directly at the workspace root";

pub(in crate::cli) const CHECK_LONG_ABOUT: &str = "Runs one reusable check directly at the workspace root.\n\nDirect execution omits unbound placeholders and does not apply suite parameters or directories.";

pub(in crate::cli) const CHECK_ID: &str = "ID of the configured check to run";

pub(in crate::cli) const FORMAT: &str = "Selects the validation report format";

pub(in crate::cli) const FORMAT_LONG: &str = "Selects the validation report format.\n\nHuman output is the default and includes live execution progress followed by the completed report. JSON emits only the versioned ValidationReport and cannot be combined with --color or --presentation.";

pub(in crate::cli) const COLOR: &str = "Enables ANSI color in human output";

pub(in crate::cli) const COLOR_LONG: &str = "Enables ANSI color in human output.\n\nWithout this flag, output contains no ANSI color. Bare --color selects the standard palette; named palettes require --color=<PALETTE>. Color can be combined with --presentation=<MODE>, but not with --format=json. Status and hierarchy never rely on color alone.";

pub(in crate::cli) const PRESENTATION: &str =
    "Selects the human-output presentation independently from color";

pub(in crate::cli) const PRESENTATION_LONG: &str = "Selects the human-output presentation independently from color.\n\nStandard uses the compact layout. Low-vision expands spacing and removes dim styling. Presentation works with plain output or any color palette, but not with --format=json. Named modes require --presentation=<MODE>.";
