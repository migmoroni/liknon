//! Immutable local resource-store loading, indexing, and payload verification.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::ErrorKind,
    path::Path,
};

use liknon_resource_contracts::{
    canonical_json_sha256, sha256_bytes, validate_catalog, validate_relative_path,
    validate_store_state, validate_trust_root, verify_detached, CatalogSnapshot, DetachedSignature,
    LockedResource, LockedResourceOrigin, PayloadMetadata, ResourceIdentity, ResourceLock,
    ResourceManifest, TrustRoot, ValidationLimits,
};
use semver::{Version, VersionReq};
use serde::de::DeserializeOwned;

use crate::{
    error::{IoResultExt as _, ToolError},
    workspace::{self, Workspace},
};

const MANIFEST_FILE: &str = "manifest.json";
const LOCK_FILE: &str = "lock.json";
const MAX_CONTRACT_BYTES: u64 = 32 * 1024 * 1024;
const MAX_CLOSURE_RESOURCES: usize = 10_000;
const MAX_CLOSURE_DEPTH: usize = 256;
/// Maximum bytes returned by one direct payload operation.
pub(crate) const MAX_SHOW_BYTES: u64 = 16 * 1024 * 1024;

#[derive(Debug)]
/// Validated immutable index for one local Tool invocation.
pub(crate) struct Store {
    workspace: Workspace,
    lock: ResourceLock,
    by_identity: BTreeMap<ResourceIdentity, usize>,
}

impl Store {
    /// Loads and authenticates one complete local resource-store contract.
    pub(crate) fn load(workspace: Workspace) -> Result<Self, ToolError> {
        let manifest_path = workspace.resources.join(MANIFEST_FILE);
        let lock_path = workspace.resources.join(LOCK_FILE);
        let manifest: ResourceManifest = read_json(&workspace.resources, &manifest_path)?;
        let lock: ResourceLock = read_json(&workspace.resources, &lock_path)?;
        validate_store_state(&manifest, &lock, ValidationLimits::default())?;
        verify_catalog_evidence(&workspace, &lock)?;
        let by_identity = lock
            .resources
            .iter()
            .enumerate()
            .map(|(index, locked)| (locked.resource.identity.clone(), index))
            .collect();
        Ok(Self {
            workspace,
            lock,
            by_identity,
        })
    }

    /// Returns the selected workspace boundary.
    pub(crate) const fn workspace(&self) -> &Workspace {
        &self.workspace
    }

    /// Returns every locked resource in deterministic identity order.
    pub(crate) fn resources(&self) -> &[LockedResource] {
        &self.lock.resources
    }

    /// Returns one exact locked identity.
    pub(crate) fn resource(
        &self,
        identity: &ResourceIdentity,
    ) -> Result<&LockedResource, ToolError> {
        self.by_identity
            .get(identity)
            .map(|index| &self.lock.resources[*index])
            .ok_or_else(|| ToolError::ResourceNotFound(identity.to_string()))
    }

    /// Returns a deterministic dependencies-first closure for exact roots.
    pub(crate) fn closure(
        &self,
        roots: &[ResourceIdentity],
    ) -> Result<Vec<&LockedResource>, ToolError> {
        let mut visiting = BTreeSet::new();
        let mut visited = BTreeSet::new();
        let mut ordered = Vec::new();
        for root in roots {
            self.visit(root, 0, &mut visiting, &mut visited, &mut ordered)?;
        }
        Ok(ordered)
    }

    /// Verifies and reads one exact declared payload under an output bound.
    pub(crate) fn payload(
        &self,
        locked: &LockedResource,
        payload: &PayloadMetadata,
        maximum: u64,
    ) -> Result<Vec<u8>, ToolError> {
        ensure_consumable(locked)?;
        if payload.length > maximum {
            return Err(ToolError::Limit(format!(
                "payload `{}` declares {} bytes, above the limit of {maximum}",
                payload.path, payload.length
            )));
        }
        validate_relative_path(&payload.path)?;
        let directory = self.workspace.resources.join(&locked.install_path);
        require_directory(&self.workspace.resources, &directory)?;
        let path = directory.join(&payload.path);
        let bytes = read_bounded(&self.workspace.resources, &path, maximum)?;
        if bytes.len() as u64 != payload.length {
            return Err(ToolError::Integrity(format!(
                "payload `{}` for {} is missing, unsupported, or has an unexpected length",
                payload.path, locked.resource.identity
            )));
        }
        if sha256_bytes(&bytes) != payload.sha256 {
            return Err(ToolError::Integrity(format!(
                "payload `{}` for {} differs from the lock",
                payload.path, locked.resource.identity
            )));
        }
        Ok(bytes)
    }

    fn visit<'a>(
        &'a self,
        identity: &ResourceIdentity,
        depth: usize,
        visiting: &mut BTreeSet<ResourceIdentity>,
        visited: &mut BTreeSet<ResourceIdentity>,
        ordered: &mut Vec<&'a LockedResource>,
    ) -> Result<(), ToolError> {
        if visited.contains(identity) {
            return Ok(());
        }
        if depth > MAX_CLOSURE_DEPTH {
            return Err(ToolError::Limit(format!(
                "dependency depth exceeds {MAX_CLOSURE_DEPTH} at `{identity}`"
            )));
        }
        if visiting.len() + visited.len() >= MAX_CLOSURE_RESOURCES {
            return Err(ToolError::Limit(format!(
                "dependency closure exceeds {MAX_CLOSURE_RESOURCES} resources"
            )));
        }
        if !visiting.insert(identity.clone()) {
            return Err(ToolError::Integrity(format!(
                "locked dependency graph contains a cycle at `{identity}`"
            )));
        }
        let resource = self.resource(identity)?;
        for edge in &resource.dependencies {
            self.visit(&edge.identity, depth + 1, visiting, visited, ordered)?;
        }
        visiting.remove(identity);
        visited.insert(identity.clone());
        ordered.push(resource);
        Ok(())
    }
}

/// Rejects security-revoked or version-incompatible resource content.
pub(crate) fn ensure_consumable(locked: &LockedResource) -> Result<(), ToolError> {
    let identity = locked.resource.identity.to_string();
    if locked.resource.revoked {
        return Err(ToolError::Revoked(identity));
    }
    let schema_version = Version::new(
        liknon_resource_contracts::RESOURCE_SCHEMA_VERSION.into(),
        0,
        0,
    );
    let schema_requirement = VersionReq::parse(&locked.resource.compatibility.resource_schema)
        .map_err(|error| {
            ToolError::Integrity(format!("invalid resource-schema requirement: {error}"))
        })?;
    if !schema_requirement.matches(&schema_version) {
        return Err(ToolError::Incompatible {
            resource: identity,
            reason: format!(
                "resource schema requirement `{}` excludes {}",
                locked.resource.compatibility.resource_schema,
                liknon_resource_contracts::RESOURCE_SCHEMA_VERSION
            ),
        });
    }
    if let Some(requirement) = &locked.resource.compatibility.tool {
        let requirement = VersionReq::parse(requirement).map_err(|error| {
            ToolError::Integrity(format!("invalid Tool compatibility requirement: {error}"))
        })?;
        let version = Version::parse(env!("CARGO_PKG_VERSION")).map_err(|error| {
            ToolError::Internal(format!("package version is not semantic: {error}"))
        })?;
        if !requirement.matches(&version) {
            return Err(ToolError::Incompatible {
                resource: identity,
                reason: format!("Tool requirement `{requirement}` excludes {version}"),
            });
        }
    }
    Ok(())
}

fn verify_catalog_evidence(workspace: &Workspace, lock: &ResourceLock) -> Result<(), ToolError> {
    let hub_resources = lock
        .resources
        .iter()
        .filter(|resource| matches!(resource.origin, LockedResourceOrigin::Hub { .. }))
        .collect::<Vec<_>>();
    let Some(locked_catalog) = &lock.catalog else {
        if hub_resources.is_empty() {
            return Ok(());
        }
        return Err(ToolError::Integrity(
            "Hub-backed resources have no authenticated catalog evidence".to_owned(),
        ));
    };
    if hub_resources.is_empty() {
        return Err(ToolError::Integrity(
            "lock retains a catalog without any Hub-backed resource".to_owned(),
        ));
    }

    let catalog_path = workspace.resources.join(&locked_catalog.catalog_path);
    let signature_path = workspace
        .resources
        .join(&locked_catalog.catalog_signature_path);
    let trust_root_path = workspace.resources.join(&locked_catalog.trust_root_path);
    let catalog_bytes = read_bounded(&workspace.resources, &catalog_path, MAX_CONTRACT_BYTES)?;
    if sha256_bytes(&catalog_bytes) != locked_catalog.sha256 {
        return Err(ToolError::Integrity(
            "retained catalog digest differs from the lock".to_owned(),
        ));
    }
    let signature: DetachedSignature = read_json(&workspace.resources, &signature_path)?;
    let trust_root: TrustRoot = read_json(&workspace.resources, &trust_root_path)?;
    validate_trust_root(&trust_root)?;
    verify_detached(&catalog_bytes, &signature, &trust_root)?;
    if trust_root.authority != locked_catalog.trust_authority
        || signature.key_id != locked_catalog.signing_key_id
    {
        return Err(ToolError::Integrity(
            "retained catalog trust identity differs from the lock".to_owned(),
        ));
    }
    let catalog: CatalogSnapshot =
        serde_json::from_slice(&catalog_bytes).map_err(|source| ToolError::Json {
            path: catalog_path.clone(),
            source,
        })?;
    validate_catalog(&catalog, ValidationLimits::default())?;
    if catalog.snapshot != locked_catalog.snapshot {
        return Err(ToolError::Integrity(
            "retained catalog snapshot differs from the lock".to_owned(),
        ));
    }
    for locked in hub_resources {
        let authenticated = catalog.resources.iter().any(|resource| {
            resource.identity == locked.resource.identity
                && resource.version == locked.resource.version
                && canonical_json_sha256(resource).ok().as_deref()
                    == Some(locked.manifest_sha256.as_str())
        });
        if !authenticated {
            return Err(ToolError::Integrity(format!(
                "locked resource {}@{} is absent from retained authenticated metadata",
                locked.resource.identity, locked.resource.version
            )));
        }
    }
    Ok(())
}

fn read_json<T: DeserializeOwned>(boundary: &Path, path: &Path) -> Result<T, ToolError> {
    let bytes = read_bounded(boundary, path, MAX_CONTRACT_BYTES)?;
    serde_json::from_slice(&bytes).map_err(|source| ToolError::Json {
        path: path.to_path_buf(),
        source,
    })
}

fn read_bounded(boundary: &Path, path: &Path, maximum: u64) -> Result<Vec<u8>, ToolError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            return Err(ToolError::Integrity(format!(
                "`{}` is a symlink",
                path.display()
            )));
        }
        Ok(_) => {}
        Err(source) if source.kind() == ErrorKind::NotFound => {
            return Err(ToolError::Integrity(format!(
                "required file `{}` is missing",
                path.display()
            )));
        }
        Err(source) => {
            return Err(ToolError::Io {
                path: path.to_path_buf(),
                source,
            });
        }
    }
    workspace::validate_existing_path(boundary, path)?;
    let metadata = fs::metadata(path).at(path)?;
    if !metadata.is_file() {
        return Err(ToolError::Integrity(format!(
            "`{}` is not a regular file",
            path.display()
        )));
    }
    if metadata.len() > maximum {
        return Err(ToolError::Limit(format!(
            "`{}` has {} bytes, above the limit of {maximum}",
            path.display(),
            metadata.len()
        )));
    }
    fs::read(path).at(path)
}

fn require_directory(boundary: &Path, path: &Path) -> Result<(), ToolError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() || !metadata.is_dir() {
                return Err(ToolError::Integrity(format!(
                    "resource directory `{}` is missing or unsafe",
                    path.display()
                )));
            }
        }
        Err(source) if source.kind() == ErrorKind::NotFound => {
            return Err(ToolError::Integrity(format!(
                "resource directory `{}` is missing",
                path.display()
            )));
        }
        Err(source) => {
            return Err(ToolError::Io {
                path: path.to_path_buf(),
                source,
            });
        }
    }
    workspace::validate_existing_path(boundary, path)
}

#[cfg(test)]
mod tests {
    use liknon_resource_contracts::{
        Compatibility, LockedCompatibility, LockedResource, LockedResourceOrigin, ResourceIdentity,
        ResourceKind, ResourceVersion, ReviewState, SourceMetadata, TrustClass, TrustMetadata,
        RESOURCE_SCHEMA_VERSION,
    };

    use super::ensure_consumable;

    fn locked() -> LockedResource {
        let identity = ResourceIdentity::new(ResourceKind::Knowledge, "fixture", "guide").unwrap();
        LockedResource {
            resource: ResourceVersion {
                schema_version: RESOURCE_SCHEMA_VERSION,
                identity: identity.clone(),
                version: "1.0.0".to_owned(),
                title: "Guide".to_owned(),
                summary: "Fixture".to_owned(),
                authors: vec!["Fixture".to_owned()],
                license: "MIT".to_owned(),
                source: SourceMetadata {
                    repository: "https://example.invalid/fixture".to_owned(),
                    revision: None,
                    path: None,
                },
                trust: TrustMetadata {
                    class: TrustClass::Local,
                    publisher: "Fixture".to_owned(),
                    review: ReviewState::Unreviewed,
                    evidence: None,
                },
                compatibility: Compatibility {
                    resource_schema: "^1".to_owned(),
                    manager: "*".to_owned(),
                    tool: Some("^0.5".to_owned()),
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
            },
            manifest_sha256: String::new(),
            origin: LockedResourceOrigin::Local {
                path: "/fixture".to_owned(),
            },
            install_path: identity.version_path("1.0.0").unwrap(),
            dependencies: Vec::new(),
            compatibility: LockedCompatibility {
                manager: true,
                ai_bootstrap_declared: false,
                decision_policy_declared: false,
                activatable: false,
                activation_blockers: Vec::new(),
            },
        }
    }

    #[test]
    fn rejects_revocation_and_incompatible_tool_versions() {
        let mut resource = locked();
        assert!(ensure_consumable(&resource).is_ok());
        resource.resource.revoked = true;
        assert!(ensure_consumable(&resource).is_err());
        resource.resource.revoked = false;
        resource.resource.compatibility.tool = Some(">=9".to_owned());
        assert!(ensure_consumable(&resource).is_err());
    }
}
