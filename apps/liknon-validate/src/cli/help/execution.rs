//! Help text for validation execution.

pub(in crate::cli) const VALIDATE_ABOUT: &str =
    "Runs a configured group or suite after configuration and tool preflight";

pub(in crate::cli) const VALIDATE_ROOT_REFERENCE: &str = "Runs a configured group or suite after configuration and tool preflight. Supports --color and --presentation for human validation output; see root Options.";

pub(in crate::cli) const VALIDATE_LONG_ABOUT: &str = "Runs a configured group or suite after configuration and tool preflight.\n\nWithout a target, runs the configured default group.";

pub(in crate::cli) const TARGET: &str =
    "Group or suite ID; defaults to the configured default group";

pub(in crate::cli) const CHECK_ABOUT: &str =
    "Runs one reusable check directly at the workspace root";

pub(in crate::cli) const CHECK_ROOT_REFERENCE: &str = "Runs one reusable check directly at the workspace root. Supports --color and --presentation for human validation output; see root Options.";

pub(in crate::cli) const CHECK_LONG_ABOUT: &str = "Runs one reusable check directly at the workspace root.\n\nDirect execution omits unbound placeholders and does not apply suite parameters or directories.";

pub(in crate::cli) const CHECK_ID: &str = "ID of the configured check to run";

pub(in crate::cli) const FORMAT: &str = "Selects the validation report format";

pub(in crate::cli) const FORMAT_LONG: &str = "Selects the validation report format.\n\nHuman output is the default and includes live execution progress followed by the completed report. JSON emits only the versioned ValidationReport and cannot be combined with --color or --presentation.";

pub(in crate::cli) const COLOR: &str = "Styles human validation output with ANSI color";

pub(in crate::cli) const COLOR_LONG: &str =
    "Styles human validation output with ANSI color. See root --help for palettes and rendering behavior.";

pub(in crate::cli) const PRESENTATION: &str =
    "Selects the layout and emphasis of human validation output";

pub(in crate::cli) const PRESENTATION_LONG: &str = "Selects the layout and emphasis of human validation output. See root --help for presentation modes and rendering behavior.";
