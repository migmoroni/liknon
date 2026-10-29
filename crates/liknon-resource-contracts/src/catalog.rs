use std::{fmt, str::FromStr};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ResourceIdentity, ResourceKind};

/// Publisher trust classification exposed without granting instruction authority.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "kebab-case")]
pub enum TrustClass {
    /// Published by the Liknon project.
    Official,
    /// Publisher identity and review evidence satisfy the Hub process.
    Verified,
    /// Listed without Liknon endorsement.
    Community,
    /// Supplied explicitly outside the public Hub.
    Local,
}

impl fmt::Display for TrustClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Official => "official",
            Self::Verified => "verified",
            Self::Community => "community",
            Self::Local => "local",
        })
    }
}

impl FromStr for TrustClass {
    type Err = crate::ContractError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "official" => Ok(Self::Official),
            "verified" => Ok(Self::Verified),
            "community" => Ok(Self::Community),
            "local" => Ok(Self::Local),
            _ => Err(crate::ContractError::InvalidContract(format!(
                "unknown trust class `{value}`"
            ))),
        }
    }
}

/// Review state attached to one immutable resource version.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "kebab-case")]
pub enum ReviewState {
    /// The resource has not completed a review process.
    Unreviewed,
    /// Review is actively in progress.
    Pending,
    /// Review evidence has been accepted.
    Reviewed,
    /// Previously accepted review evidence has been withdrawn.
    Withdrawn,
}

/// Authorship and source provenance for one resource version.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SourceMetadata {
    /// Canonical source repository or publication location.
    pub repository: String,
    /// Exact source revision when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    /// Publisher-controlled path within the source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// Trust and review facts for one resource version.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustMetadata {
    /// Public trust classification.
    pub class: TrustClass,
    /// Publisher identity shown to consumers.
    pub publisher: String,
    /// Review state for this exact version.
    pub review: ReviewState,
    /// Stable review-evidence reference when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence: Option<String>,
}

/// Compatibility requirements declared by one resource version.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Compatibility {
    /// Accepted resource-contract schema versions.
    pub resource_schema: String,
    /// Accepted Manager versions.
    pub manager: String,
    /// Accepted Tool versions when the resource is consumed by Tool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool: Option<String>,
    /// Accepted Validation CLI versions when relevant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validator: Option<String>,
    /// Accepted program-owned AI bootstrap contract versions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ai_bootstrap: Option<String>,
    /// Accepted human-owned AI decision-policy contract versions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decision_policy: Option<String>,
}

/// Required or optional edge in the resource dependency graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceDependency {
    /// Stable dependency identity.
    pub identity: ResourceIdentity,
    /// Accepted dependency versions.
    pub requirement: String,
    /// Whether omission is permitted when no caller selects the dependency.
    #[serde(default)]
    pub optional: bool,
}

/// One payload file declared by immutable metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PayloadMetadata {
    /// Relative installed path below the resource version directory.
    pub path: String,
    /// Immutable object path relative to the authenticated Hub origin.
    pub object: String,
    /// Declared media type.
    pub media_type: String,
    /// Exact byte length.
    pub length: u64,
    /// Lower-case hexadecimal SHA-256 digest.
    pub sha256: String,
    /// Whether the payload represents executable content as data.
    #[serde(default)]
    pub executable: bool,
}

/// Generic workflow node declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkflowNode {
    /// Stable node identifier scoped to the workflow.
    pub id: String,
    /// Stable action identifier scoped to the workflow.
    pub action: String,
    /// Referenced agent-profile resource identities.
    #[serde(default)]
    pub agent_profiles: Vec<ResourceIdentity>,
    /// Referenced operation IDs declared by tool resources.
    #[serde(default)]
    pub tool_operations: Vec<String>,
    /// Potential externally observable effects.
    #[serde(default)]
    pub effects: Vec<String>,
    /// Required host capabilities.
    #[serde(default)]
    pub required_capabilities: Vec<String>,
}

/// Generic directed workflow transition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkflowTransition {
    /// Stable transition identifier scoped to the workflow.
    pub id: String,
    /// Source node identifier.
    pub from: String,
    /// Destination node identifier.
    pub to: String,
    /// Declarative precondition label interpreted by the external host.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precondition: Option<String>,
    /// Whether the decision-policy continuation setting applies to this edge.
    #[serde(default)]
    pub continuation_eligible: bool,
}

/// Declared artifact output from one workflow action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ArtifactDeclaration {
    /// Stable declaration identifier.
    pub id: String,
    /// Producing workflow node.
    pub node: String,
    /// Producing action identifier.
    pub action: String,
    /// Stable payload-schema identifier or URI.
    pub schema: String,
    /// Bounded relative storage template.
    pub storage: String,
    /// Sensitivity label interpreted by the external host policy.
    pub sensitivity: String,
    /// Maximum serialized size.
    pub max_bytes: u64,
    /// Repository treatment for the produced artifact.
    pub repository_treatment: RepositoryTreatment,
}

/// Repository treatment declared for a workflow artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum RepositoryTreatment {
    /// The artifact is private local state expected to be ignored.
    LocalIgnored,
    /// The consumer explicitly decides whether and where it is tracked.
    ConsumerSelected,
}

/// Generic workflow graph metadata carried as untrusted resource data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkflowMetadata {
    /// Entry node identifiers.
    pub entry_nodes: Vec<String>,
    /// Terminal node identifiers.
    pub terminal_nodes: Vec<String>,
    /// Declared nodes.
    pub nodes: Vec<WorkflowNode>,
    /// Declared transitions.
    pub transitions: Vec<WorkflowTransition>,
    /// Declared artifacts.
    #[serde(default)]
    pub artifacts: Vec<ArtifactDeclaration>,
}

/// Complete immutable metadata for one resource version.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceVersion {
    /// Exact resource-manifest schema version.
    pub schema_version: u32,
    /// Stable resource identity.
    pub identity: ResourceIdentity,
    /// Exact semantic resource version.
    pub version: String,
    /// Human-facing title.
    pub title: String,
    /// Concise discovery summary.
    pub summary: String,
    /// Human or organizational authors.
    pub authors: Vec<String>,
    /// SPDX license expression.
    pub license: String,
    /// Authored-source provenance.
    pub source: SourceMetadata,
    /// Trust and review facts.
    pub trust: TrustMetadata,
    /// Consumer compatibility requirements.
    pub compatibility: Compatibility,
    /// Required and optional resource edges.
    #[serde(default)]
    pub dependencies: Vec<ResourceDependency>,
    /// Collection membership represented as graph edges without copied payloads.
    #[serde(default)]
    pub collection_members: Vec<ResourceDependency>,
    /// Declared generic capabilities.
    #[serde(default)]
    pub capabilities: Vec<String>,
    /// Whether any content may represent executable instructions or bytes.
    #[serde(default)]
    pub contains_executable_content: bool,
    /// Complete immutable payload inventory.
    #[serde(default)]
    pub payloads: Vec<PayloadMetadata>,
    /// Generic workflow graph, present only for workflow resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow: Option<WorkflowMetadata>,
    /// Whether new resolution should avoid this version.
    #[serde(default)]
    pub yanked: bool,
    /// Whether a newer replacement is recommended.
    #[serde(default)]
    pub deprecated: bool,
    /// Whether this version is security-revoked and cannot be resolved.
    #[serde(default)]
    pub revoked: bool,
}

impl ResourceVersion {
    /// Returns whether the metadata shape matches the declared kind.
    #[must_use]
    pub fn has_kind_specific_shape(&self) -> bool {
        match self.identity.kind {
            ResourceKind::Workflow => self.workflow.is_some(),
            ResourceKind::Collection => self.workflow.is_none() && self.payloads.is_empty(),
            _ => self.workflow.is_none() && self.collection_members.is_empty(),
        }
    }
}

/// Stable identity and monotonic order of one catalog snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CatalogSnapshotIdentity {
    /// Immutable snapshot identifier.
    pub id: String,
    /// Monotonic sequence used for rollback detection.
    pub sequence: u64,
    /// Authenticated Unix timestamp when the snapshot was published.
    pub published_at_unix: u64,
    /// Authenticated Unix timestamp after which online resolution fails closed.
    pub expires_at_unix: u64,
}

/// Security advisory associated with resource versions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecurityAdvisory {
    /// Stable advisory identifier.
    pub id: String,
    /// Affected resource identity.
    pub identity: ResourceIdentity,
    /// Affected semantic version requirement.
    pub affected: String,
    /// Human-readable summary.
    pub summary: String,
    /// Whether affected versions are security-revoked.
    pub revoked: bool,
}

/// Complete immutable Hub catalog snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CatalogSnapshot {
    /// Exact catalog schema version.
    pub schema_version: u32,
    /// Stable snapshot identity.
    pub snapshot: CatalogSnapshotIdentity,
    /// Resource versions sorted by identity and descending semantic version.
    pub resources: Vec<ResourceVersion>,
    /// Security advisories sorted by stable identifier.
    #[serde(default)]
    pub advisories: Vec<SecurityAdvisory>,
}

/// Authenticated mutable pointer to one immutable catalog snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CatalogPointer {
    /// Exact pointer schema version.
    pub schema_version: u32,
    /// Snapshot identity selected as current.
    pub snapshot: CatalogSnapshotIdentity,
    /// Relative immutable catalog-object path.
    pub catalog_path: String,
    /// Exact catalog-object byte length.
    pub catalog_length: u64,
    /// Exact catalog-object SHA-256 digest.
    pub catalog_sha256: String,
    /// Relative detached-signature path for the catalog object.
    pub catalog_signature_path: String,
}
