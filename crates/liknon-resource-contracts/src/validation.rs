use std::collections::{BTreeMap, BTreeSet};

use semver::{Version, VersionReq};

use crate::{
    canonical_json_sha256, validate_relative_path, validate_sha256, CatalogSnapshot, ContractError,
    LockedResourceOrigin, RequestedSource, ResourceIdentity, ResourceKind, ResourceLock,
    ResourceManifest, ResourceVersion, TrustKeyStatus, TrustRoot, CATALOG_SCHEMA_VERSION,
    RESOURCE_LOCK_SCHEMA_VERSION, RESOURCE_MANIFEST_SCHEMA_VERSION, RESOURCE_SCHEMA_VERSION,
    RETAINED_CATALOG_PATH, RETAINED_CATALOG_SIGNATURE_PATH, RETAINED_TRUST_ROOT_PATH,
    TRUST_ROOT_SCHEMA_VERSION,
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

/// Validates the complete desired-state and exact-lock relationship.
///
/// This operation is transport- and filesystem-neutral. Applications remain
/// responsible for authenticating retained catalog bytes and checking that
/// installed payload files match the validated lock.
pub fn validate_store_state(
    manifest: &ResourceManifest,
    lock: &ResourceLock,
    limits: ValidationLimits,
) -> Result<(), ContractError> {
    if manifest.schema_version != RESOURCE_MANIFEST_SCHEMA_VERSION {
        return Err(ContractError::UnsupportedSchema {
            document: "resource manifest",
            expected: RESOURCE_MANIFEST_SCHEMA_VERSION,
            actual: manifest.schema_version,
        });
    }
    if lock.schema_version != RESOURCE_LOCK_SCHEMA_VERSION {
        return Err(ContractError::UnsupportedSchema {
            document: "resource lock",
            expected: RESOURCE_LOCK_SCHEMA_VERSION,
            actual: lock.schema_version,
        });
    }

    if let Some(catalog) = &lock.catalog {
        validate_sha256(&catalog.sha256)?;
        for (value, label) in [
            (&catalog.origin, "locked catalog origin"),
            (&catalog.trust_authority, "locked catalog trust authority"),
            (&catalog.signing_key_id, "locked catalog signing key ID"),
        ] {
            require_text(value, label)?;
        }
        for (actual, expected) in [
            (&catalog.catalog_path, RETAINED_CATALOG_PATH),
            (
                &catalog.catalog_signature_path,
                RETAINED_CATALOG_SIGNATURE_PATH,
            ),
            (&catalog.trust_root_path, RETAINED_TRUST_ROOT_PATH),
        ] {
            validate_relative_path(actual)?;
            if actual != expected {
                return Err(ContractError::InvalidContract(format!(
                    "noncanonical retained catalog evidence path `{actual}`"
                )));
            }
        }
    }

    let mut prior_request: Option<&ResourceIdentity> = None;
    for request in &manifest.resources {
        request.identity.validate()?;
        parse_requirement(&request.requirement, "resource request")?;
        if prior_request.is_some_and(|prior| prior >= &request.identity) {
            return Err(ContractError::InvalidContract(
                "manifest resources must be sorted by unique identity".to_owned(),
            ));
        }
        if let RequestedSource::Local { path } = &request.source {
            require_text(path, "local resource source path")?;
        }
        prior_request = Some(&request.identity);
    }

    let mut prior_locked: Option<&ResourceIdentity> = None;
    for locked in &lock.resources {
        validate_resource(&locked.resource, limits)?;
        if locked.manifest_sha256 != canonical_json_sha256(&locked.resource)? {
            return Err(ContractError::InvalidContract(format!(
                "locked metadata digest differs for {}",
                locked.resource.identity
            )));
        }
        validate_relative_path(&locked.install_path)?;
        if locked.install_path
            != locked
                .resource
                .identity
                .version_path(&locked.resource.version)?
        {
            return Err(ContractError::InvalidContract(format!(
                "noncanonical install path for {}",
                locked.resource.identity
            )));
        }
        if prior_locked.is_some_and(|prior| prior >= &locked.resource.identity) {
            return Err(ContractError::InvalidContract(
                "locked resources must be sorted by unique identity".to_owned(),
            ));
        }
        match &locked.origin {
            LockedResourceOrigin::Hub { origin } => {
                let catalog = lock.catalog.as_ref().ok_or_else(|| {
                    ContractError::InvalidContract(
                        "Hub-backed resource has no locked catalog".to_owned(),
                    )
                })?;
                if origin != &catalog.origin {
                    return Err(ContractError::InvalidContract(format!(
                        "Hub origin for {} differs from the locked catalog",
                        locked.resource.identity
                    )));
                }
            }
            LockedResourceOrigin::Local { path } => {
                require_text(path, "local locked-resource origin")?;
            }
        }
        validate_locked_edges(locked)?;
        prior_locked = Some(&locked.resource.identity);
    }

    let locked = lock
        .resources
        .iter()
        .map(|resource| (&resource.resource.identity, resource))
        .collect::<BTreeMap<_, _>>();
    for request in &manifest.resources {
        let resolved = locked.get(&request.identity).ok_or_else(|| {
            ContractError::InvalidContract(format!(
                "requested resource {} is absent from the lock",
                request.identity
            ))
        })?;
        let requirement = VersionReq::parse(&request.requirement).map_err(|error| {
            ContractError::InvalidVersion(format!(
                "resource request `{}`: {error}",
                request.requirement
            ))
        })?;
        let version = Version::parse(&resolved.resource.version).map_err(|error| {
            ContractError::InvalidVersion(format!(
                "locked resource version `{}`: {error}",
                resolved.resource.version
            ))
        })?;
        if !requirement.matches(&version) {
            return Err(ContractError::InvalidContract(format!(
                "locked resource {}@{} does not satisfy request `{}`",
                request.identity, resolved.resource.version, request.requirement
            )));
        }
        let source_matches = matches!(
            (&request.source, &resolved.origin),
            (RequestedSource::Hub, LockedResourceOrigin::Hub { .. })
        ) || matches!(
            (&request.source, &resolved.origin),
            (
                RequestedSource::Local { path: requested },
                LockedResourceOrigin::Local { path: locked }
            ) if requested == locked
        );
        if !source_matches {
            return Err(ContractError::InvalidContract(format!(
                "locked origin for {} differs from its requested source",
                request.identity
            )));
        }
    }
    for resource in &lock.resources {
        for edge in &resource.dependencies {
            if locked
                .get(&edge.identity)
                .is_none_or(|dependency| dependency.resource.version != edge.version)
            {
                return Err(ContractError::InvalidContract(format!(
                    "locked dependency {}@{} is absent",
                    edge.identity, edge.version
                )));
            }
        }
    }

    let mut prior_destination: Option<&str> = None;
    for materialization in &lock.materializations {
        materialization.identity.validate()?;
        Version::parse(&materialization.version).map_err(|error| {
            ContractError::InvalidVersion(format!(
                "materialized version `{}`: {error}",
                materialization.version
            ))
        })?;
        validate_relative_path(&materialization.payload_path)?;
        validate_relative_path(&materialization.destination)?;
        validate_sha256(&materialization.sha256)?;
        if prior_destination.is_some_and(|prior| prior >= materialization.destination.as_str()) {
            return Err(ContractError::InvalidContract(
                "materializations must be sorted by unique destination".to_owned(),
            ));
        }
        let resource = lock
            .resources
            .iter()
            .find(|resource| {
                resource.resource.identity == materialization.identity
                    && resource.resource.version == materialization.version
            })
            .ok_or_else(|| {
                ContractError::InvalidContract(format!(
                    "materialization `{}` references an absent resource",
                    materialization.destination
                ))
            })?;
        if !resource.resource.payloads.iter().any(|payload| {
            payload.path == materialization.payload_path && payload.sha256 == materialization.sha256
        }) {
            return Err(ContractError::InvalidContract(format!(
                "materialization `{}` references an absent payload",
                materialization.destination
            )));
        }
        prior_destination = Some(&materialization.destination);
    }
    Ok(())
}

fn validate_locked_edges(locked: &crate::LockedResource) -> Result<(), ContractError> {
    let mut prior: Option<&ResourceIdentity> = None;
    for edge in &locked.dependencies {
        edge.identity.validate()?;
        Version::parse(&edge.version).map_err(|error| {
            ContractError::InvalidVersion(format!(
                "locked dependency version `{}`: {error}",
                edge.version
            ))
        })?;
        if prior.is_some_and(|prior| prior >= &edge.identity) {
            return Err(ContractError::InvalidContract(format!(
                "locked dependencies for {} must be sorted by unique identity",
                locked.resource.identity
            )));
        }
        let declared = locked
            .resource
            .dependencies
            .iter()
            .map(|dependency| (dependency, false))
            .chain(
                locked
                    .resource
                    .collection_members
                    .iter()
                    .map(|dependency| (dependency, true)),
            )
            .find(|(dependency, _)| dependency.identity == edge.identity);
        let Some((declared, collection_member)) = declared else {
            return Err(ContractError::InvalidContract(format!(
                "locked dependency {} is not declared by {}",
                edge.identity, locked.resource.identity
            )));
        };
        if collection_member != edge.collection_member {
            return Err(ContractError::InvalidContract(format!(
                "locked dependency {} has the wrong edge kind for {}",
                edge.identity, locked.resource.identity
            )));
        }
        let requirement = VersionReq::parse(&declared.requirement).map_err(|error| {
            ContractError::InvalidVersion(format!(
                "dependency requirement `{}`: {error}",
                declared.requirement
            ))
        })?;
        let version = Version::parse(&edge.version).map_err(|error| {
            ContractError::InvalidVersion(format!(
                "locked dependency version `{}`: {error}",
                edge.version
            ))
        })?;
        if !requirement.matches(&version) {
            return Err(ContractError::InvalidContract(format!(
                "locked dependency {}@{} does not satisfy `{}`",
                edge.identity, edge.version, declared.requirement
            )));
        }
        prior = Some(&edge.identity);
    }
    for (declared, collection_member) in locked
        .resource
        .dependencies
        .iter()
        .map(|dependency| (dependency, false))
        .chain(
            locked
                .resource
                .collection_members
                .iter()
                .map(|dependency| (dependency, true)),
        )
    {
        if !declared.optional
            && !locked.dependencies.iter().any(|edge| {
                edge.identity == declared.identity && edge.collection_member == collection_member
            })
        {
            return Err(ContractError::InvalidContract(format!(
                "required dependency {} is absent from the lock for {}",
                declared.identity, locked.resource.identity
            )));
        }
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
