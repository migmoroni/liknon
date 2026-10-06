//! Command-line help text grouped by user-facing context.

pub(in crate::cli) mod execution;
pub(in crate::cli) mod initialization;
pub(in crate::cli) mod inspection;
pub(in crate::cli) mod knowledge;
pub(in crate::cli) mod root;
pub(in crate::cli) mod values;

pub(in crate::cli) const CONFIG: &str = "Uses an explicit configuration file instead of discovery";

pub(in crate::cli) const CONFIG_LONG: &str = "Uses an explicit configuration file instead of discovery.\n\nWhen omitted, discovery walks upward from the process current directory and selects the nearest .validation/config.json.";
