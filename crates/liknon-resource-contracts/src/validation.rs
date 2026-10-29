use std::collections::BTreeSet;

use semver::{Version, VersionReq};

use crate::{
    validate_relative_path, validate_sha256, CatalogSnapshot, ContractError, ResourceIdentity,
    ResourceKind, ResourceVersion, TrustKeyStatus, TrustRoot, CATALOG_SCHEMA_VERSION,
    RESOURCE_SCHEMA_VERSION, TRUST_ROOT_SCHEMA_VERSION,
};

/// Resource payload bounds enforced before allocation or installation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidationLimits {
    /// Maximum payload count in one resource version.
    pub max_payload_files: usize,
    /// Maximum bytes in one payload.
    pub max_payload_bytes: u64,
    /// Maximum aggregate bytes in one resource version.
    pub max_resource_bytes: u64,
}

impl Default for ValidationLimits {
    fn default() -> Self {
        Self {
            max_payload_files: 10_000,
            max_payload_bytes: 256 * 1024 * 1024,
            max_resource_bytes: 1024 * 1024 * 1024,
        }
    }
}

/// Validates one immutable resource version independently from transport.
pub fn validate_resource(
    resource: &ResourceVersion,
    limits: ValidationLimits,
) -> Result<(), ContractError> {
    if resource.schema_version != RESOURCE_SCHEMA_VERSION {
        return Err(ContractError::UnsupportedSchema {
            document: "resource",
            expected: RESOURCE_SCHEMA_VERSION,
            actual: resource.schema_version,
        });
    }
    resource.identity.validate()?;
    Version::parse(&resource.version).map_err(|error| {
        ContractError::InvalidVersion(format!(
            "{} version `{}`: {error}",
            resource.identity, resource.version
        ))
    })?;
    require_text(&resource.title, "resource title")?;
    require_text(&resource.summary, "resource summary")?;
    if resource.authors.is_empty() {
        return Err(ContractError::InvalidContract(format!(
            "{} has no author",
            resource.identity
        )));
    }
    for author in &resource.authors {
        require_text(author, "resource author")?;
    }
    require_text(&resource.license, "resource license")?;
    require_text(&resource.source.repository, "source repository")?;
    require_text(&resource.trust.publisher, "trust publisher")?;
    parse_requirement(
        &resource.compatibility.resource_schema,
        "resource schema compatibility",
    )?;
    parse_requirement(&resource.compatibility.manager, "Manager compatibility")?;
    for (label, requirement) in [
        ("Tool compatibility", &resource.compatibility.tool),
        (
            "Validation compatibility",
            &resource.compatibility.validator,
        ),
        (
            "AI bootstrap compatibility",
            &resource.compatibility.ai_bootstrap,
        ),
        (
            "decision-policy compatibility",
            &resource.compatibility.decision_policy,
        ),
    ] {
        if let Some(requirement) = requirement {
            parse_requirement(requirement, label)?;
        }
    }
    if !resource.has_kind_specific_shape() {
        return Err(ContractError::InvalidContract(format!(
            "{} has metadata incompatible with kind {}",
            resource.identity, resource.identity.kind
        )));
    }

    let mut edge_ids = BTreeSet::new();
    for edge in resource
        .dependencies
        .iter()
        .chain(&resource.collection_members)
    {
        edge.identity.validate()?;
        parse_requirement(&edge.requirement, "dependency requirement")?;
        if !edge_ids.insert(edge.identity.clone()) {
            return Err(ContractError::InvalidContract(format!(
                "{} declares duplicate edge {}",
                resource.identity, edge.identity
            )));
        }
        if edge.identity == resource.identity {
            return Err(ContractError::InvalidContract(format!(
                "{} depends on itself",
                resource.identity
            )));
        }
    }
    validate_edge_order(&resource.identity, &resource.dependencies, "dependencies")?;
    validate_edge_order(
        &resource.identity,
        &resource.collection_members,
        "collection members",
    )?;
    let mut prior_capability: Option<&str> = None;
    for capability in &resource.capabilities {
        require_stable_id(capability, "resource capability")?;
        if prior_capability.is_some_and(|prior| prior >= capability.as_str()) {
            return Err(ContractError::InvalidContract(format!(
                "{} capabilities must be sorted and unique",
                resource.identity
            )));
        }
        prior_capability = Some(capability);
    }

    if resource.payloads.len() > limits.max_payload_files {
        return Err(ContractError::InvalidContract(format!(
            "{} declares {} payloads, above the limit of {}",
            resource.identity,
            resource.payloads.len(),
            limits.max_payload_files
        )));
    }
    let mut payload_paths = BTreeSet::new();
    let mut total_bytes = 0u64;
    let mut prior_payload: Option<&str> = None;
    for payload in &resource.payloads {
        validate_relative_path(&payload.path)?;
        validate_relative_path(&payload.object)?;
        validate_sha256(&payload.sha256)?;
        require_text(&payload.media_type, "payload media type")?;
        if payload.length > limits.max_payload_bytes {
            return Err(ContractError::InvalidContract(format!(
                "payload `{}` exceeds the per-file byte limit",
                payload.path
            )));
        }
        total_bytes = total_bytes.checked_add(payload.length).ok_or_else(|| {
            ContractError::InvalidContract("payload byte total overflowed".to_owned())
        })?;
        if total_bytes > limits.max_resource_bytes {
            return Err(ContractError::InvalidContract(format!(
                "{} exceeds the aggregate payload byte limit",
                resource.identity
            )));
        }
        if !payload_paths.insert(&payload.path) {
            return Err(ContractError::InvalidContract(format!(
                "{} declares duplicate payload path `{}`",
                resource.identity, payload.path
            )));
        }
        if prior_payload.is_some_and(|prior| prior >= payload.path.as_str()) {
            return Err(ContractError::InvalidContract(format!(
                "{} payloads must be sorted by unique installed path",
                resource.identity
            )));
        }
        prior_payload = Some(&payload.path);
    }
    if resource.payloads.iter().any(|payload| payload.executable)
        && !resource.contains_executable_content
    {
        return Err(ContractError::InvalidContract(format!(
            "{} contains an executable payload without declaring executable content",
            resource.identity
        )));
    }

    if let Some(workflow) = &resource.workflow {
        validate_workflow(resource, workflow)?;
    }
    Ok(())
}

fn validate_edge_order(
    owner: &ResourceIdentity,
    edges: &[crate::ResourceDependency],
    label: &str,
) -> Result<(), ContractError> {
    let mut prior: Option<&ResourceIdentity> = None;
    for edge in edges {
        if prior.is_some_and(|prior| prior >= &edge.identity) {
            return Err(ContractError::InvalidContract(format!(
                "{owner} {label} must be sorted by unique identity"
            )));
        }
        prior = Some(&edge.identity);
    }
    Ok(())
}

/// Validates one complete catalog and its deterministic ordering.
pub fn validate_catalog(
    catalog: &CatalogSnapshot,
    limits: ValidationLimits,
) -> Result<(), ContractError> {
    if catalog.schema_version != CATALOG_SCHEMA_VERSION {
        return Err(ContractError::UnsupportedSchema {
            document: "catalog",
            expected: CATALOG_SCHEMA_VERSION,
            actual: catalog.schema_version,
        });
    }
    validate_snapshot_id(&catalog.snapshot.id)?;
    if catalog.snapshot.sequence == 0 {
        return Err(ContractError::InvalidContract(
            "catalog sequence must be positive".to_owned(),
        ));
    }
    if catalog.snapshot.expires_at_unix <= catalog.snapshot.published_at_unix {
        return Err(ContractError::InvalidContract(
            "catalog expiry must follow publication".to_owned(),
        ));
    }

    let mut previous: Option<(&ResourceIdentity, Version)> = None;
    for resource in &catalog.resources {
        validate_resource(resource, limits)?;
        let version = Version::parse(&resource.version)
            .map_err(|error| ContractError::InvalidVersion(format!("resource version: {error}")))?;
        if let Some((previous_identity, previous_version)) = &previous {
            if *previous_identity > &resource.identity
                || (*previous_identity == &resource.identity && previous_version <= &version)
            {
                return Err(ContractError::InvalidContract(
                    "catalog resources must be ordered by identity and descending unique version"
                        .to_owned(),
                ));
            }
        }
        previous = Some((&resource.identity, version));
    }

    let mut advisory_ids = BTreeSet::new();
    let mut prior_advisory: Option<&str> = None;
    for advisory in &catalog.advisories {
        require_stable_id(&advisory.id, "advisory ID")?;
        advisory.identity.validate()?;
        parse_requirement(&advisory.affected, "advisory affected versions")?;
        require_text(&advisory.summary, "advisory summary")?;
        if prior_advisory.is_some_and(|prior| prior >= advisory.id.as_str())
            || !advisory_ids.insert(&advisory.id)
        {
            return Err(ContractError::InvalidContract(
                "catalog advisories must be sorted by unique ID".to_owned(),
            ));
        }
        prior_advisory = Some(&advisory.id);
    }
    for resource in &catalog.resources {
        let version = Version::parse(&resource.version)
            .map_err(|error| ContractError::InvalidVersion(format!("resource version: {error}")))?;
        let advisory_revoked = catalog.advisories.iter().any(|advisory| {
            advisory.revoked
                && advisory.identity == resource.identity
                && VersionReq::parse(&advisory.affected)
                    .is_ok_and(|requirement| requirement.matches(&version))
        });
        if resource.revoked != advisory_revoked {
            return Err(ContractError::InvalidContract(format!(
                "revocation state for {}@{} differs from its advisories",
                resource.identity, resource.version
            )));
        }
    }
    Ok(())
}

/// Validates trust-root ordering, key material shape, and lifecycle intervals.
pub fn validate_trust_root(trust_root: &TrustRoot) -> Result<(), ContractError> {
    if trust_root.schema_version != TRUST_ROOT_SCHEMA_VERSION {
        return Err(ContractError::UnsupportedSchema {
            document: "trust root",
            expected: TRUST_ROOT_SCHEMA_VERSION,
            actual: trust_root.schema_version,
        });
    }
    require_text(&trust_root.authority, "trust authority")?;
    if trust_root.keys.is_empty() {
        return Err(ContractError::InvalidContract(
            "trust root must contain at least one key".to_owned(),
        ));
    }
    let mut prior: Option<&str> = None;
    for key in &trust_root.keys {
        require_stable_id(&key.key_id, "trust key ID")?;
        if prior.is_some_and(|value| value >= key.key_id.as_str()) {
            return Err(ContractError::InvalidContract(
                "trust keys must be sorted by unique key ID".to_owned(),
            ));
        }
        if key.not_after_unix <= key.not_before_unix {
            return Err(ContractError::InvalidContract(format!(
                "trust key `{}` has an invalid validity interval",
                key.key_id
            )));
        }
        if key.public_key.is_empty() {
            return Err(ContractError::InvalidContract(format!(
                "trust key `{}` has no public key",
                key.key_id
            )));
        }
        if key.status == TrustKeyStatus::Active && key.not_after_unix == u64::MAX {
            return Err(ContractError::InvalidContract(format!(
                "active trust key `{}` must have a bounded expiry",
                key.key_id
            )));
        }
        prior = Some(&key.key_id);
    }
    Ok(())
}

fn validate_workflow(
    resource: &ResourceVersion,
    workflow: &crate::WorkflowMetadata,
) -> Result<(), ContractError> {
    if workflow.entry_nodes.is_empty() || workflow.terminal_nodes.is_empty() {
        return Err(ContractError::InvalidContract(format!(
            "workflow {} requires entry and terminal nodes",
            resource.identity
        )));
    }
    let mut node_ids = BTreeSet::new();
    for node in &workflow.nodes {
        require_stable_id(&node.id, "workflow node ID")?;
        require_stable_id(&node.action, "workflow action ID")?;
        if !node_ids.insert(node.id.as_str()) {
            return Err(ContractError::InvalidContract(format!(
                "workflow {} has duplicate node `{}`",
                resource.identity, node.id
            )));
        }
        for profile in &node.agent_profiles {
            if profile.kind != ResourceKind::AgentProfile {
                return Err(ContractError::InvalidContract(format!(
                    "workflow node `{}` references non-profile {}",
                    node.id, profile
                )));
            }
            require_declared_dependency(resource, profile)?;
        }
        for operation in &node.tool_operations {
            require_stable_id(operation, "tool operation ID")?;
        }
        for effect in &node.effects {
            require_stable_id(effect, "workflow effect")?;
        }
        for capability in &node.required_capabilities {
            require_stable_id(capability, "workflow capability")?;
        }
    }
    for node in workflow.entry_nodes.iter().chain(&workflow.terminal_nodes) {
        if !node_ids.contains(node.as_str()) {
            return Err(ContractError::InvalidContract(format!(
                "workflow {} references missing node `{node}`",
                resource.identity
            )));
        }
    }

    let mut transition_ids = BTreeSet::new();
    for transition in &workflow.transitions {
        require_stable_id(&transition.id, "workflow transition ID")?;
        if !transition_ids.insert(transition.id.as_str()) {
            return Err(ContractError::InvalidContract(format!(
                "workflow {} has duplicate transition `{}`",
                resource.identity, transition.id
            )));
        }
        if !node_ids.contains(transition.from.as_str())
            || !node_ids.contains(transition.to.as_str())
        {
            return Err(ContractError::InvalidContract(format!(
                "workflow transition `{}` references a missing node",
                transition.id
            )));
        }
    }

    let mut artifact_ids = BTreeSet::new();
    for artifact in &workflow.artifacts {
        require_stable_id(&artifact.id, "artifact declaration ID")?;
        if !artifact_ids.insert(artifact.id.as_str()) {
            return Err(ContractError::InvalidContract(format!(
                "workflow {} has duplicate artifact `{}`",
                resource.identity, artifact.id
            )));
        }
        if !node_ids.contains(artifact.node.as_str()) {
            return Err(ContractError::InvalidContract(format!(
                "artifact `{}` references missing node `{}`",
                artifact.id, artifact.node
            )));
        }
        require_stable_id(&artifact.action, "artifact action ID")?;
        validate_relative_path(&artifact.storage)?;
        if artifact.max_bytes == 0 {
            return Err(ContractError::InvalidContract(format!(
                "artifact `{}` must have a positive size limit",
                artifact.id
            )));
        }
    }
    Ok(())
}

fn require_declared_dependency(
    resource: &ResourceVersion,
    identity: &ResourceIdentity,
) -> Result<(), ContractError> {
    if resource
        .dependencies
        .iter()
        .chain(&resource.collection_members)
        .any(|edge| &edge.identity == identity)
    {
        Ok(())
    } else {
        Err(ContractError::InvalidContract(format!(
            "workflow {} references undeclared dependency {}",
            resource.identity, identity
        )))
    }
}

fn parse_requirement(value: &str, label: &str) -> Result<(), ContractError> {
    VersionReq::parse(value)
        .map(|_| ())
        .map_err(|error| ContractError::InvalidVersion(format!("{label} `{value}`: {error}")))
}

fn require_text(value: &str, label: &str) -> Result<(), ContractError> {
    if value.trim().is_empty() || value.contains('\0') {
        Err(ContractError::InvalidContract(format!(
            "{label} must contain non-NUL text"
        )))
    } else {
        Ok(())
    }
}

fn require_stable_id(value: &str, label: &str) -> Result<(), ContractError> {
    let valid = !value.is_empty()
        && value.len() <= 128
        && value.as_bytes()[0].is_ascii_lowercase()
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'-' | b'_' | b'.')
        });
    if valid {
        Ok(())
    } else {
        Err(ContractError::InvalidContract(format!(
            "{label} `{value}` is not a normalized stable ID"
        )))
    }
}

fn validate_snapshot_id(value: &str) -> Result<(), ContractError> {
    require_stable_id(value, "catalog snapshot ID")
}

#[cfg(test)]
mod tests {
    use super::{validate_catalog, validate_resource, ValidationLimits};
    use crate::{
        CatalogSnapshot, CatalogSnapshotIdentity, Compatibility, ResourceIdentity, ResourceKind,
        ResourceVersion, ReviewState, SourceMetadata, TrustClass, TrustMetadata,
        CATALOG_SCHEMA_VERSION, RESOURCE_SCHEMA_VERSION,
    };

    fn resource(name: &str, version: &str) -> ResourceVersion {
        ResourceVersion {
            schema_version: RESOURCE_SCHEMA_VERSION,
            identity: ResourceIdentity::new(ResourceKind::Knowledge, "official", name).unwrap(),
            version: version.to_owned(),
            title: name.to_owned(),
            summary: "Fixture resource".to_owned(),
            authors: vec!["Liknon".to_owned()],
            license: "MIT".to_owned(),
            source: SourceMetadata {
                repository: "https://example.invalid/repository".to_owned(),
                revision: Some("abc".to_owned()),
                path: Some("docs.md".to_owned()),
            },
            trust: TrustMetadata {
                class: TrustClass::Official,
                publisher: "Liknon".to_owned(),
                review: ReviewState::Reviewed,
                evidence: None,
            },
            compatibility: Compatibility {
                resource_schema: "^1".to_owned(),
                manager: ">=0.5".to_owned(),
                tool: None,
                validator: None,
                ai_bootstrap: None,
                decision_policy: None,
            },
            dependencies: Vec::new(),
            collection_members: Vec::new(),
            capabilities: Vec::new(),
            contains_executable_content: false,
            payloads: Vec::new(),
            workflow: None,
            yanked: false,
            deprecated: false,
            revoked: false,
        }
    }

    #[test]
    fn validates_kind_shape_and_catalog_ordering() {
        validate_resource(&resource("alpha", "1.0.0"), ValidationLimits::default()).unwrap();
        let catalog = CatalogSnapshot {
            schema_version: CATALOG_SCHEMA_VERSION,
            snapshot: CatalogSnapshotIdentity {
                id: "snapshot-1".to_owned(),
                sequence: 1,
                published_at_unix: 10,
                expires_at_unix: 20,
            },
            resources: vec![resource("alpha", "2.0.0"), resource("alpha", "1.0.0")],
            advisories: Vec::new(),
        };
        validate_catalog(&catalog, ValidationLimits::default()).unwrap();
    }
}
