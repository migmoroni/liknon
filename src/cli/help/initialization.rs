//! Help text for workspace initialization.

pub(in crate::cli) const ABOUT: &str =
    "Provisions validated consumer-owned resources without executing checks";

pub(in crate::cli) const CONFIG: &str = "Path to the complete configuration candidate to install";

pub(in crate::cli) const CONFIG_LONG: &str = "Path to the complete configuration candidate to install.\n\nThis command validates the candidate but does not discover an existing workspace configuration or run configured programs.";

pub(in crate::cli) const WORKSPACE: &str = "Sets the destination workspace boundary";

pub(in crate::cli) const WORKSPACE_LONG: &str = "Sets the destination workspace boundary.\n\nThe path defaults exactly to the process current directory. The candidate and canonical .validation/config.json destination must remain within this boundary.";

pub(in crate::cli) const FORMAT: &str = "Selects the initialization result format";

pub(in crate::cli) const FORMAT_LONG: &str = "Selects the initialization result format.\n\nHuman output is the default. JSON emits the versioned, machine-readable initialization result without terminal styling.";
