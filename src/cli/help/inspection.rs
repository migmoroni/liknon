//! Help text for configuration inspection and schema output.

pub(in crate::cli) const CONFIG_ABOUT: &str =
    "Inspects and validates configuration without executing configured programs";

pub(in crate::cli) const CONFIG_VALIDATE_ABOUT: &str =
    "Validates configuration and filesystem semantics without starting configured tools";

pub(in crate::cli) const LIST_ABOUT: &str =
    "Lists configured groups, suites, checks, and tools without executing them";

pub(in crate::cli) const TREE: &str =
    "Also prints the configured default group's reachable execution hierarchy";

pub(in crate::cli) const TREE_LONG: &str = "Also prints the configured default group's reachable execution hierarchy.\n\nWithout this flag, the command lists the available groups, suites, checks, and tools without expanding their relationships.";

pub(in crate::cli) const EXPLAIN_ABOUT: &str =
    "Explains one configured group, suite, or check without executing it";

pub(in crate::cli) const GROUP_ABOUT: &str =
    "Shows membership, hierarchy, resolved invocations, and required tools for a group";

pub(in crate::cli) const GROUP_ID: &str = "ID of the configured group to explain";

pub(in crate::cli) const SUITE_ABOUT: &str =
    "Shows directory, membership, resolved invocations, and required tools for a suite";

pub(in crate::cli) const SUITE_ID: &str = "ID of the configured suite to explain";

pub(in crate::cli) const CHECK_ABOUT: &str =
    "Shows the template and resolved direct and suite-bound invocations for a check";

pub(in crate::cli) const CHECK_ID: &str = "ID of the configured check to explain";

pub(in crate::cli) const SCHEMA_ABOUT: &str =
    "Prints the generated JSON Schema for a public CLI contract";

pub(in crate::cli) const CONTRACT: &str =
    "Public contract whose JSON Schema is written to standard output";
