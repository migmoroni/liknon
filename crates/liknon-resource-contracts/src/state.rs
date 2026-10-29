use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{CatalogSnapshotIdentity, ResourceIdentity, ResourceVersion, TrustClass};

/// Origin selected for one desired resource.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum RequestedSource {
    /// Resolve through an authenticated Hub catalog.
    #[default]
    Hub,
    /// Resolve from an explicitly supplied local resource directory.
    Local {
        /// Consumer-relative or absolute source path supplied explicitly.
        path: String,
    },
}

/// Human-maintained desired state for one resource identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceRequest {
    /// Stable requested identity.
    pub identity: ResourceIdentity,
    /// Accepted semantic versions.
    pub requirement: String,
    /// Explicit source selection.
    #[serde(default)]
    pub source: RequestedSource,
}

/// Human-readable desired resource state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceManifest {
    /// Exact desired-state schema version.
    pub schema_version: u32,
    /// Requested roots sorted by identity.
    pub resources: Vec<ResourceRequest>,
}

/// Authenticated catalog identity retained by one lock.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LockedCatalog {
    /// Hub origin used for resolution.
    pub origin: String,
    /// Exact authenticated catalog snapshot.
    pub snapshot: CatalogSnapshotIdentity,
    /// Exact catalog digest.
    pub sha256: String,
    /// Trust-root authority that authenticated the snapshot.
    pub trust_authority: String,
    /// Signing key that authenticated the snapshot.
    pub signing_key_id: String,
    /// Retained authenticated catalog bytes below `.liknon/resources/`.
    pub catalog_path: String,
    /// Detached catalog signature below `.liknon/resources/`.
    pub catalog_signature_path: String,
    /// Separately established trust root retained for offline verification.
    pub trust_root_path: String,
}

/// Locked edge to an exact dependency or collection member.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LockedEdge {
    /// Exact target identity.
    pub identity: ResourceIdentity,
    /// Exact selected semantic version.
    pub version: String,
    /// Whether the edge originates from collection membership.
    pub collection_member: bool,
}

/// Compatibility facts retained for offline inspection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LockedCompatibility {
    /// Whether the local Manager supported the resource during resolution.
    pub manager: bool,
    /// Whether a governed workflow declares an AI bootstrap contract.
    pub ai_bootstrap_declared: bool,
    /// Whether a governed workflow declares a decision-policy contract.
    pub decision_policy_declared: bool,
    /// Whether the resource may be represented as activatable by a conforming host.
    pub activatable: bool,
    /// Stable reasons preventing activation.
    #[serde(default)]
    pub activation_blockers: Vec<String>,
}

/// Acquisition origin retained for one exact locked resource.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum LockedResourceOrigin {
    /// Immutable object paths are relative to this authenticated Hub origin.
    Hub {
        /// Authenticated Hub base origin.
        origin: String,
    },
    /// Payload paths are relative to one explicitly selected local directory.
    Local {
        /// Canonical local resource directory.
        path: String,
    },
}

/// Exact installed state for one resource version.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LockedResource {
    /// Complete authenticated immutable resource metadata.
    pub resource: ResourceVersion,
    /// Digest of canonical resource metadata.
    pub manifest_sha256: String,
    /// Exact acquisition origin.
    pub origin: LockedResourceOrigin,
    /// Relative installation directory below `.liknon/resources/`.
    pub install_path: String,
    /// Exact resolved graph edges.
    #[serde(default)]
    pub dependencies: Vec<LockedEdge>,
    /// Compatibility facts evaluated during resolution.
    pub compatibility: LockedCompatibility,
}

/// Recorded projection of one locked resource file outside the resource store.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Materialization {
    /// Stable source resource identity.
    pub identity: ResourceIdentity,
    /// Exact source resource version.
    pub version: String,
    /// Payload path within the locked resource.
    pub payload_path: String,
    /// Consumer-workspace-relative destination.
    pub destination: String,
    /// Exact projected byte digest.
    pub sha256: String,
}

/// Exact complete resolution installed in one consumer workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceLock {
    /// Exact lock schema version.
    pub schema_version: u32,
    /// Authenticated catalog used for Hub-backed resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog: Option<LockedCatalog>,
    /// Exact resources sorted by identity.
    pub resources: Vec<LockedResource>,
    /// Explicit materializations sorted by destination.
    #[serde(default)]
    pub materializations: Vec<Materialization>,
}

/// Administrative comparison state reported by Manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum InstalledState {
    /// Desired, locked, and installed bytes agree.
    Installed,
    /// Desired state has no exact lock entry.
    Unlocked,
    /// Locked state has no complete payload installation.
    Missing,
    /// Installed bytes differ from the lock.
    Drifted,
    /// Locked but no longer desired as a manifest root.
    Transitive,
}

/// Integrity state of one explicit projection outside the resource store.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum MaterializationStatus {
    /// Destination bytes agree with the lock record.
    Materialized,
    /// The recorded destination does not exist.
    Missing,
    /// Destination bytes differ from the lock record or are unsafe to inspect.
    Drifted,
}

/// Administrative comparison state for one explicit materialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MaterializationState {
    /// Stable source resource identity.
    pub identity: ResourceIdentity,
    /// Exact source resource version.
    pub version: String,
    /// Payload path within the locked resource.
    pub payload_path: String,
    /// Consumer-workspace-relative destination.
    pub destination: String,
    /// Comparison with the exact bytes retained in the lock.
    pub state: MaterializationStatus,
}

/// One row in Manager's administrative resource listing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceState {
    /// Stable resource identity.
    pub identity: ResourceIdentity,
    /// Desired semantic version requirement when directly requested.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desired: Option<String>,
    /// Exact locked semantic version when resolved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<String>,
    /// Trust classification retained in the lock.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trust: Option<TrustClass>,
    /// Administrative comparison state.
    pub state: InstalledState,
    /// Whether the version is yanked.
    pub yanked: bool,
    /// Whether the version is security-revoked.
    pub revoked: bool,
    /// Whether governed activation is compatible.
    pub activatable: bool,
}
