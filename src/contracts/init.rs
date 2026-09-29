//! Version 1 initialization result types.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Initialization result schema version emitted by this crate.
pub const INIT_RESULT_SCHEMA_VERSION: u32 = 1;

/// Aggregate outcome of one initialization request.
#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InitStatus {
    /// Every requested resource was created or identically reused.
    Success,
    /// At least one destination contained different bytes.
    Conflict,
    /// At least one requested resource could not be validated or written.
    Failed,
    /// Some resources succeeded while another resource did not.
    Partial,
}

/// Outcome of provisioning one capability-owned resource.
#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InitResourceStatus {
    /// The validated bytes were installed at the canonical destination.
    Created,
    /// The destination already contained the exact validated bytes.
    Reused,
    /// The destination existed with different bytes or an unsafe file type.
    Conflict,
    /// Validation or filesystem preparation failed.
    Failed,
    /// The resource was valid but was not written because another resource failed.
    NotWritten,
}

/// Kind of resource requested from the extensible initialization command.
#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InitResourceKind {
    /// Canonical `.validation/config.json` configuration.
    Config,
}

/// Result of provisioning one requested resource.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InitResourceResult {
    /// Capability-owned resource kind.
    pub kind: InitResourceKind,
    /// Normalized absolute destination below the selected workspace.
    pub path: String,
    /// Resource-specific outcome.
    pub status: InitResourceStatus,
    /// SHA-256 digest of candidate bytes when they could be read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// Bounded diagnostics explaining a non-success outcome.
    pub diagnostics: Vec<String>,
}

/// Complete, versioned result of one initialization request.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InitResult {
    /// Version of this initialization result contract.
    #[schemars(range(min = 1, max = 1))]
    pub schema_version: u32,
    /// Canonical absolute workspace selected by the caller.
    pub workspace_root: String,
    /// Aggregate outcome across requested resources.
    pub status: InitStatus,
    /// Results in capability flag order.
    pub resources: Vec<InitResourceResult>,
}
