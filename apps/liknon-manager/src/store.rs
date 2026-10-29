use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{ErrorKind, Write as _},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use fs2::FileExt as _;
use liknon_resource_contracts::{
    canonical_json_sha256, sha256_bytes, validate_catalog, validate_relative_path,
    validate_resource, validate_sha256, validate_trust_root, verify_detached, CatalogSnapshot,
    DetachedSignature, InstalledState, LockedResource, Materialization, MaterializationState,
    MaterializationStatus, RequestedSource, ResourceIdentity, ResourceLock, ResourceManifest,
    ResourceState, TrustClass, TrustRoot, ValidationLimits, RESOURCE_LOCK_SCHEMA_VERSION,
    RESOURCE_MANIFEST_SCHEMA_VERSION, RETAINED_CATALOG_PATH, RETAINED_CATALOG_SIGNATURE_PATH,
    RETAINED_TRUST_ROOT_PATH,
};
use semver::VersionReq;
use serde::{Deserialize, Serialize};

use crate::{
    error::{IoResultExt as _, ManagerError},
    hub::AuthenticatedCatalogEvidence,
    resolver::AvailableResource,
    workspace::{self, Workspace},
};

const MANIFEST_FILE: &str = "manifest.json";
const LOCK_FILE: &str = "lock.json";

#[derive(Debug, Clone)]
pub(crate) struct StoreState {
    pub(crate) manifest: ResourceManifest,
    pub(crate) lock: ResourceLock,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Verification {
    pub(crate) resources: usize,
    pub(crate) payloads: usize,
    pub(crate) warnings: Vec<String>,
}

pub(crate) type PayloadBytes = BTreeMap<(ResourceIdentity, String, String), Vec<u8>>;

#[derive(Debug, Clone)]
pub(crate) struct ProjectionMutation {
    destination: PathBuf,
    expected_sha256: Option<String>,
    action: ProjectionAction,
}

#[derive(Debug, Clone)]
enum ProjectionAction {
    Write(Vec<u8>),
    Remove,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct TransactionJournal {
    schema_version: u32,
    phase: TransactionPhase,
    staging: String,
    backup: String,
    #[serde(default)]
    projections: Vec<JournalProjection>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct JournalProjection {
    destination: String,
    temporary: Option<String>,
    backup: String,
    expected_sha256: Option<String>,
    resulting_sha256: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum TransactionPhase {
    Prepared,
    BackedUp,
    Committed,
}

pub(crate) fn initialize(workspace: &Workspace) -> Result<(), ManagerError> {
    match fs::symlink_metadata(&workspace.liknon_root) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            return Err(ManagerError::Conflict(format!(
                "`{}` is not a regular directory",
                workspace.liknon_root.display()
            )));
        }
        Ok(_) => {}
        Err(error) if error.kind() == ErrorKind::NotFound => {
            fs::create_dir(&workspace.liknon_root).at(&workspace.liknon_root)?;
        }
        Err(source) => {
            return Err(ManagerError::Io {
                path: workspace.liknon_root.clone(),
                source,
            });
        }
    }
    let replace_empty = match fs::symlink_metadata(&workspace.resources) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            return Err(ManagerError::Conflict(format!(
                "resource root `{}` is not a regular directory",
                workspace.resources.display()
            )));
        }
        Ok(_) => {
            if load(workspace).is_ok() {
                return Ok(());
            }
            let mut entries = fs::read_dir(&workspace.resources).at(&workspace.resources)?;
            if entries.next().is_some() {
                return Err(ManagerError::Conflict(format!(
                    "resource root `{}` contains conflicting state",
                    workspace.resources.display()
                )));
            }
            true
        }
        Err(error) if error.kind() == ErrorKind::NotFound => false,
        Err(source) => {
            return Err(ManagerError::Io {
                path: workspace.resources.clone(),
                source,
            });
        }
    };

    let staging = workspace
        .liknon_root
        .join(format!(".resources-init-{}", transaction_id()?));
    fs::create_dir(&staging).at(&staging)?;
    let result = initialize_staging(&staging);
    if let Err(error) = result {
        let _ = fs::remove_dir_all(&staging);
        return Err(error);
    }
    if replace_empty {
        fs::remove_dir(&workspace.resources).at(&workspace.resources)?;
    }
    if let Err(source) = fs::rename(&staging, &workspace.resources) {
        let _ = fs::remove_dir_all(&staging);
        if replace_empty {
            let _ = fs::create_dir(&workspace.resources);
        }
        return Err(ManagerError::Io {
            path: staging,
            source,
        });
    }
    #[cfg(unix)]
    fs::File::open(&workspace.liknon_root)
        .at(&workspace.liknon_root)?
        .sync_all()
        .at(&workspace.liknon_root)?;
    Ok(())
}

fn initialize_staging(staging: &Path) -> Result<(), ManagerError> {
    for directory in [
        "knowledge",
        "skills",
        "workflows",
        "agent-profiles",
        "tools",
        "prompts",
        "guides",
        "templates",
        "collections",
    ] {
        let path = staging.join(directory);
        fs::create_dir_all(&path).at(&path)?;
    }
    let manifest = ResourceManifest {
        schema_version: RESOURCE_MANIFEST_SCHEMA_VERSION,
        resources: Vec::new(),
    };
    let lock = ResourceLock {
        schema_version: RESOURCE_LOCK_SCHEMA_VERSION,
        catalog: None,
        resources: Vec::new(),
        materializations: Vec::new(),
    };
    write_json(&staging.join(MANIFEST_FILE), &manifest)?;
    write_json(&staging.join(LOCK_FILE), &lock)?;
    sync_tree(staging)
}

pub(crate) fn load(workspace: &Workspace) -> Result<StoreState, ManagerError> {
    validate_resource_root(workspace)?;
    let manifest_path = workspace.resources.join(MANIFEST_FILE);
    let lock_path = workspace.resources.join(LOCK_FILE);
    let manifest: ResourceManifest = read_manifest_json(&manifest_path)?;
    let lock: ResourceLock = read_integrity_json(&lock_path, "resource lock")?;
    validate_state(&manifest, &lock)?;
    Ok(StoreState { manifest, lock })
}

pub(crate) fn state_rows(
    workspace: &Workspace,
    state: &StoreState,
) -> Result<Vec<ResourceState>, ManagerError> {
    let desired = state
        .manifest
        .resources
        .iter()
        .map(|request| (request.identity.clone(), request.requirement.clone()))
        .collect::<BTreeMap<_, _>>();
    let locked = state
        .lock
        .resources
        .iter()
        .map(|resource| (resource.resource.identity.clone(), resource))
        .collect::<BTreeMap<_, _>>();
    let identities = desired
        .keys()
        .chain(locked.keys())
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut rows = Vec::new();
    for identity in identities {
        let desired_requirement = desired.get(&identity).cloned();
        let locked_resource = locked.get(&identity).copied();
        let installed_state = match (desired_requirement.is_some(), locked_resource) {
            (true, None) => InstalledState::Unlocked,
            (desired, Some(resource)) => match installed_state(workspace, resource)? {
                InstalledState::Installed if !desired => InstalledState::Transitive,
                state => state,
            },
            (false, None) => continue,
        };
        rows.push(ResourceState {
            identity,
            desired: desired_requirement,
            locked: locked_resource.map(|resource| resource.resource.version.clone()),
            trust: locked_resource.map(|resource| resource.resource.trust.class),
            state: installed_state,
            yanked: locked_resource.is_some_and(|resource| resource.resource.yanked),
            revoked: locked_resource.is_some_and(|resource| resource.resource.revoked),
            activatable: locked_resource.is_some_and(|resource| resource.compatibility.activatable),
        });
    }
    Ok(rows)
}

pub(crate) fn materialization_rows(
    workspace: &Workspace,
    state: &StoreState,
) -> Result<Vec<MaterializationState>, ManagerError> {
    state
        .lock
        .materializations
        .iter()
        .map(|record| {
            let state = match verify_materialization(workspace, record) {
                Ok(()) => MaterializationStatus::Materialized,
                Err(ManagerError::Io { source, .. }) if source.kind() == ErrorKind::NotFound => {
                    MaterializationStatus::Missing
                }
                Err(ManagerError::Integrity(_)) => MaterializationStatus::Drifted,
                Err(error) => return Err(error),
            };
            Ok(MaterializationState {
                identity: record.identity.clone(),
                version: record.version.clone(),
                payload_path: record.payload_path.clone(),
                destination: record.destination.clone(),
                state,
            })
        })
        .collect()
}

pub(crate) fn verify(
    workspace: &Workspace,
    state: &StoreState,
) -> Result<Verification, ManagerError> {
    validate_state(&state.manifest, &state.lock)?;
    let mut result = Verification::default();
    verify_catalog_evidence(workspace, &state.lock)?;
    for resource in &state.lock.resources {
        verify_locked_resource(workspace, resource)?;
        result.resources += 1;
        result.payloads += resource.resource.payloads.len();
    }
    for materialization in &state.lock.materializations {
        verify_materialization(workspace, materialization)?;
    }
    Ok(result)
}

fn verify_catalog_evidence(workspace: &Workspace, lock: &ResourceLock) -> Result<(), ManagerError> {
    let hub_resources = lock
        .resources
        .iter()
        .filter(|resource| {
            matches!(
                resource.origin,
                liknon_resource_contracts::LockedResourceOrigin::Hub { .. }
            )
        })
        .collect::<Vec<_>>();
    let Some(locked_catalog) = &lock.catalog else {
        if hub_resources.is_empty() {
            return Ok(());
        }
        return Err(ManagerError::Integrity(
            "Hub-backed resources have no authenticated catalog evidence".to_owned(),
        ));
    };
    if hub_resources.is_empty() {
        return Err(ManagerError::Integrity(
            "lock retains a catalog without any Hub-backed resource".to_owned(),
        ));
    }
    for (actual, expected, label) in [
        (
            &locked_catalog.catalog_path,
            RETAINED_CATALOG_PATH,
            "catalog",
        ),
        (
            &locked_catalog.catalog_signature_path,
            RETAINED_CATALOG_SIGNATURE_PATH,
            "catalog signature",
        ),
        (
            &locked_catalog.trust_root_path,
            RETAINED_TRUST_ROOT_PATH,
            "trust root",
        ),
    ] {
        if actual != expected {
            return Err(ManagerError::Integrity(format!(
                "noncanonical retained {label} path `{actual}`"
            )));
        }
        validate_relative_path(actual)?;
    }
    let catalog_path = workspace.resources.join(&locked_catalog.catalog_path);
    let signature_path = workspace
        .resources
        .join(&locked_catalog.catalog_signature_path);
    let trust_root_path = workspace.resources.join(&locked_catalog.trust_root_path);
    let catalog_bytes = fs::read(&catalog_path).at(&catalog_path)?;
    if sha256_bytes(&catalog_bytes) != locked_catalog.sha256 {
        return Err(ManagerError::Integrity(
            "retained catalog digest differs from the lock".to_owned(),
        ));
    }
    let signature: DetachedSignature =
        read_integrity_json(&signature_path, "retained catalog signature")?;
    let trust_root: TrustRoot = read_integrity_json(&trust_root_path, "retained trust root")?;
    validate_trust_root(&trust_root)?;
    verify_detached(&catalog_bytes, &signature, &trust_root)?;
    if trust_root.authority != locked_catalog.trust_authority
        || signature.key_id != locked_catalog.signing_key_id
    {
        return Err(ManagerError::Integrity(
            "retained catalog trust identity differs from the lock".to_owned(),
        ));
    }
    let catalog: CatalogSnapshot = serde_json::from_slice(&catalog_bytes).map_err(|error| {
        ManagerError::Integrity(format!(
            "retained catalog `{}` is malformed JSON: {error}",
            catalog_path.display()
        ))
    })?;
    validate_catalog(&catalog, ValidationLimits::default())?;
    if catalog.snapshot != locked_catalog.snapshot {
        return Err(ManagerError::Integrity(
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
            return Err(ManagerError::Integrity(format!(
                "locked resource {}@{} is absent from retained authenticated metadata",
                locked.resource.identity, locked.resource.version
            )));
        }
    }
    Ok(())
}

pub(crate) fn apply(
    workspace: &Workspace,
    manifest: &ResourceManifest,
    lock: &ResourceLock,
    payloads: &PayloadBytes,
    evidence: Option<&AuthenticatedCatalogEvidence>,
    projections: &[ProjectionMutation],
) -> Result<(), ManagerError> {
    validate_state(manifest, lock)?;
    fs::create_dir_all(&workspace.manager_state).at(&workspace.manager_state)?;
    let lock_file_path = workspace.manager_state.join("manager.lock");
    let lock_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_file_path)
        .at(&lock_file_path)?;
    lock_file.try_lock_exclusive().map_err(|_| {
        ManagerError::Blocked(
            "another Manager process holds the workspace mutation lock".to_owned(),
        )
    })?;
    recover_locked(workspace)?;

    let transaction_id = transaction_id()?;
    let transactions = workspace.manager_state.join("transactions");
    fs::create_dir_all(&transactions).at(&transactions)?;
    let staging = transactions.join(format!("{transaction_id}-resources"));
    let backup = transactions.join(format!("{transaction_id}-backup"));
    copy_tree(&workspace.resources, &staging)?;
    remove_owned_payloads(&staging, &load(workspace)?.lock)?;
    install_payloads(&staging, lock, payloads)?;
    install_catalog_evidence(&staging, lock, evidence)?;
    write_json(&staging.join(MANIFEST_FILE), manifest)?;
    write_json(&staging.join(LOCK_FILE), lock)?;
    sync_tree(&staging)?;

    let journal_projections = match prepare_projections(workspace, projections, &transaction_id) {
        Ok(prepared) => prepared,
        Err(error) => {
            fs::remove_dir_all(&staging).at(&staging)?;
            return Err(error);
        }
    };

    let mut journal = TransactionJournal {
        schema_version: 1,
        phase: TransactionPhase::Prepared,
        staging: staging.display().to_string(),
        backup: backup.display().to_string(),
        projections: journal_projections,
    };
    if let Err(error) = write_journal(workspace, &journal) {
        cleanup_projection_backups(&journal.projections)?;
        fs::remove_dir_all(&staging).at(&staging)?;
        return Err(error);
    }
    fs::rename(&workspace.resources, &backup).at(&workspace.resources)?;
    journal.phase = TransactionPhase::BackedUp;
    write_journal(workspace, &journal)?;
    if let Err(error) = fs::rename(&staging, &workspace.resources) {
        let _ = fs::rename(&backup, &workspace.resources);
        return Err(ManagerError::Io {
            path: staging,
            source: error,
        });
    }
    if let Err(error) = commit_projections(&journal.projections) {
        rollback_projections(workspace, &journal.projections)?;
        fs::remove_dir_all(&workspace.resources).at(&workspace.resources)?;
        fs::rename(&backup, &workspace.resources).at(&backup)?;
        if staging.exists() {
            fs::remove_dir_all(&staging).at(&staging)?;
        }
        fs::remove_file(journal_path(workspace)).at(journal_path(workspace))?;
        return Err(error);
    }
    journal.phase = TransactionPhase::Committed;
    write_journal(workspace, &journal)?;
    fs::remove_dir_all(&backup).at(&backup)?;
    cleanup_projection_backups(&journal.projections)?;
    fs::remove_file(journal_path(workspace)).at(journal_path(workspace))?;
    fs2::FileExt::unlock(&lock_file).at(&lock_file_path)?;
    Ok(())
}

pub(crate) fn recover(workspace: &Workspace) -> Result<(), ManagerError> {
    fs::create_dir_all(&workspace.manager_state).at(&workspace.manager_state)?;
    let lock_file_path = workspace.manager_state.join("manager.lock");
    let lock_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_file_path)
        .at(&lock_file_path)?;
    lock_file.try_lock_exclusive().map_err(|_| {
        ManagerError::Blocked(
            "another Manager process holds the workspace mutation lock".to_owned(),
        )
    })?;
    let result = recover_locked(workspace);
    fs2::FileExt::unlock(&lock_file).at(&lock_file_path)?;
    result
}

pub(crate) fn load_local_resource(path: &Path) -> Result<AvailableResource, ManagerError> {
    let supplied_metadata = fs::symlink_metadata(path).at(path)?;
    if supplied_metadata.file_type().is_symlink() || !supplied_metadata.is_dir() {
        return Err(ManagerError::Usage(format!(
            "local resource `{}` must be a non-symlink directory",
            path.display()
        )));
    }
    let root = fs::canonicalize(path).at(path)?;
    let resource_path = root.join("resource.json");
    let resource: liknon_resource_contracts::ResourceVersion =
        read_integrity_json(&resource_path, "local resource metadata")?;
    validate_resource(&resource, ValidationLimits::default())?;
    if resource.trust.class != TrustClass::Local {
        return Err(ManagerError::Integrity(format!(
            "explicit local resource {} must declare trust class `local`",
            resource.identity
        )));
    }
    for payload in &resource.payloads {
        let bytes = read_local_payload(&root, &payload.object, payload.length)?;
        if sha256_bytes(&bytes) != payload.sha256 {
            return Err(ManagerError::Integrity(format!(
                "local payload `{}` does not match its declared digest",
                payload.object
            )));
        }
    }
    Ok(AvailableResource {
        resource,
        origin: liknon_resource_contracts::LockedResourceOrigin::Local {
            path: root.display().to_string(),
        },
    })
}

pub(crate) fn local_payload(
    root: &Path,
    object: &str,
    expected_length: u64,
) -> Result<Vec<u8>, ManagerError> {
    read_local_payload(root, object, expected_length)
}

pub(crate) fn installed_payload(
    workspace: &Workspace,
    locked: &LockedResource,
    payload_path: &str,
) -> Result<Vec<u8>, ManagerError> {
    let payload = locked
        .resource
        .payloads
        .iter()
        .find(|payload| payload.path == payload_path)
        .ok_or_else(|| {
            ManagerError::Internal(format!(
                "locked resource {} has no payload `{payload_path}`",
                locked.resource.identity
            ))
        })?;
    let path = workspace
        .resources
        .join(&locked.install_path)
        .join(payload_path);
    workspace::validate_existing_ancestors(&workspace.resources, &path)?;
    let bytes = fs::read(&path).at(&path)?;
    if bytes.len() as u64 != payload.length || sha256_bytes(&bytes) != payload.sha256 {
        return Err(ManagerError::Integrity(format!(
            "installed payload `{payload_path}` for {} differs from the lock",
            locked.resource.identity
        )));
    }
    Ok(bytes)
}

pub(crate) fn prepare_materialization(
    workspace: &Workspace,
    state: &StoreState,
    identity: &ResourceIdentity,
    payload_path: &str,
    destination: &Path,
) -> Result<(ResourceLock, ProjectionMutation), ManagerError> {
    validate_relative_path(payload_path)?;
    let locked = state
        .lock
        .resources
        .iter()
        .find(|resource| &resource.resource.identity == identity)
        .ok_or_else(|| ManagerError::Usage(format!("resource `{identity}` is not installed")))?;
    let payload = locked
        .resource
        .payloads
        .iter()
        .find(|payload| payload.path == payload_path)
        .ok_or_else(|| {
            ManagerError::Usage(format!(
                "resource `{identity}` has no payload `{payload_path}`"
            ))
        })?;
    let source = workspace
        .resources
        .join(&locked.install_path)
        .join(&payload.path);
    let bytes = fs::read(&source).at(&source)?;
    if bytes.len() as u64 != payload.length || sha256_bytes(&bytes) != payload.sha256 {
        return Err(ManagerError::Integrity(format!(
            "installed payload `{payload_path}` is drifted"
        )));
    }
    let destination = workspace::contained_destination(workspace, destination)?;
    if destination.exists() {
        let current = fs::read(&destination).at(&destination)?;
        let owned = state.lock.materializations.iter().find(|record| {
            Path::new(&record.destination) == destination.strip_prefix(&workspace.root).unwrap()
        });
        if owned.is_none_or(|record| sha256_bytes(&current) != record.sha256) {
            return Err(ManagerError::Conflict(format!(
                "destination `{}` contains unowned bytes",
                destination.display()
            )));
        }
    }
    let relative = destination
        .strip_prefix(&workspace.root)
        .expect("contained destination")
        .to_string_lossy()
        .replace('\\', "/");
    let expected_sha256 = state
        .lock
        .materializations
        .iter()
        .find(|record| record.destination == relative)
        .map(|record| record.sha256.clone());
    let mut lock = state.lock.clone();
    lock.materializations
        .retain(|record| record.destination != relative);
    lock.materializations.push(Materialization {
        identity: identity.clone(),
        version: locked.resource.version.clone(),
        payload_path: payload_path.to_owned(),
        destination: relative,
        sha256: payload.sha256.clone(),
    });
    lock.materializations
        .sort_by(|left, right| left.destination.cmp(&right.destination));
    Ok((
        lock,
        ProjectionMutation {
            destination,
            expected_sha256,
            action: ProjectionAction::Write(bytes),
        },
    ))
}

pub(crate) fn removed_materializations(
    workspace: &Workspace,
    current: &ResourceLock,
    proposed: &ResourceLock,
) -> Result<Vec<ProjectionMutation>, ManagerError> {
    let retained = proposed
        .materializations
        .iter()
        .map(|record| record.destination.as_str())
        .collect::<BTreeSet<_>>();
    current
        .materializations
        .iter()
        .filter(|record| !retained.contains(record.destination.as_str()))
        .map(|record| {
            let destination =
                workspace::contained_destination(workspace, Path::new(&record.destination))?;
            Ok(ProjectionMutation {
                destination,
                expected_sha256: Some(record.sha256.clone()),
                action: ProjectionAction::Remove,
            })
        })
        .collect()
}

fn validate_resource_root(workspace: &Workspace) -> Result<(), ManagerError> {
    let metadata = fs::symlink_metadata(&workspace.resources).at(&workspace.resources)?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(ManagerError::Integrity(format!(
            "resource root `{}` must be a non-symlink directory",
            workspace.resources.display()
        )));
    }
    workspace::validate_existing_ancestors(&workspace.root, &workspace.resources)
}

fn validate_state(manifest: &ResourceManifest, lock: &ResourceLock) -> Result<(), ManagerError> {
    if manifest.schema_version != RESOURCE_MANIFEST_SCHEMA_VERSION {
        return Err(ManagerError::Integrity(format!(
            "unsupported resource manifest schema {}",
            manifest.schema_version
        )));
    }
    if lock.schema_version != RESOURCE_LOCK_SCHEMA_VERSION {
        return Err(ManagerError::Integrity(format!(
            "unsupported resource lock schema {}",
            lock.schema_version
        )));
    }
    if let Some(catalog) = &lock.catalog {
        validate_sha256(&catalog.sha256)?;
        if catalog.origin.trim().is_empty()
            || catalog.trust_authority.trim().is_empty()
            || catalog.signing_key_id.trim().is_empty()
        {
            return Err(ManagerError::Integrity(
                "locked catalog identity contains an empty field".to_owned(),
            ));
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
                return Err(ManagerError::Integrity(format!(
                    "noncanonical retained catalog evidence path `{actual}`"
                )));
            }
        }
    }
    let mut prior_request: Option<&ResourceIdentity> = None;
    for request in &manifest.resources {
        request.identity.validate()?;
        VersionReq::parse(&request.requirement).map_err(|error| {
            ManagerError::Integrity(format!(
                "invalid requirement for {}: {error}",
                request.identity
            ))
        })?;
        if prior_request.is_some_and(|prior| prior >= &request.identity) {
            return Err(ManagerError::Integrity(
                "manifest resources must be sorted by unique identity".to_owned(),
            ));
        }
        if let RequestedSource::Local { path } = &request.source {
            if path.trim().is_empty() {
                return Err(ManagerError::Integrity(
                    "local resource request has an empty source path".to_owned(),
                ));
            }
        }
        prior_request = Some(&request.identity);
    }
    let mut prior_locked: Option<&ResourceIdentity> = None;
    for resource in &lock.resources {
        validate_resource(&resource.resource, ValidationLimits::default())?;
        if resource.manifest_sha256 != canonical_json_sha256(&resource.resource)? {
            return Err(ManagerError::Integrity(format!(
                "locked metadata digest differs for {}",
                resource.resource.identity
            )));
        }
        validate_relative_path(&resource.install_path)?;
        if resource.install_path
            != resource
                .resource
                .identity
                .version_path(&resource.resource.version)?
        {
            return Err(ManagerError::Integrity(format!(
                "noncanonical install path for {}",
                resource.resource.identity
            )));
        }
        if prior_locked.is_some_and(|prior| prior >= &resource.resource.identity) {
            return Err(ManagerError::Integrity(
                "locked resources must be sorted by unique identity".to_owned(),
            ));
        }
        match &resource.origin {
            liknon_resource_contracts::LockedResourceOrigin::Hub { origin } => {
                let catalog = lock.catalog.as_ref().ok_or_else(|| {
                    ManagerError::Integrity("Hub-backed resource has no locked catalog".to_owned())
                })?;
                if origin != &catalog.origin {
                    return Err(ManagerError::Integrity(format!(
                        "Hub origin for {} differs from the locked catalog",
                        resource.resource.identity
                    )));
                }
            }
            liknon_resource_contracts::LockedResourceOrigin::Local { path } => {
                if path.trim().is_empty() {
                    return Err(ManagerError::Integrity(format!(
                        "local origin for {} is empty",
                        resource.resource.identity
                    )));
                }
            }
        }
        prior_locked = Some(&resource.resource.identity);
    }
    let locked_ids = lock
        .resources
        .iter()
        .map(|resource| {
            (
                &resource.resource.identity,
                resource.resource.version.as_str(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    for resource in &lock.resources {
        for edge in &resource.dependencies {
            if locked_ids.get(&edge.identity).copied() != Some(edge.version.as_str()) {
                return Err(ManagerError::Integrity(format!(
                    "locked dependency {}@{} is absent",
                    edge.identity, edge.version
                )));
            }
        }
    }
    let mut prior_destination: Option<&str> = None;
    for materialization in &lock.materializations {
        materialization.identity.validate()?;
        VersionReq::parse(&format!("={}", materialization.version)).map_err(|error| {
            ManagerError::Integrity(format!(
                "invalid materialized version `{}`: {error}",
                materialization.version
            ))
        })?;
        validate_relative_path(&materialization.payload_path)?;
        validate_relative_path(&materialization.destination)?;
        validate_sha256(&materialization.sha256)?;
        if prior_destination.is_some_and(|prior| prior >= materialization.destination.as_str()) {
            return Err(ManagerError::Integrity(
                "materializations must be sorted by unique destination".to_owned(),
            ));
        }
        let Some(resource) = lock.resources.iter().find(|resource| {
            resource.resource.identity == materialization.identity
                && resource.resource.version == materialization.version
        }) else {
            return Err(ManagerError::Integrity(format!(
                "materialization `{}` references an absent resource",
                materialization.destination
            )));
        };
        let payload_matches = resource.resource.payloads.iter().any(|payload| {
            payload.path == materialization.payload_path && payload.sha256 == materialization.sha256
        });
        if !payload_matches {
            return Err(ManagerError::Integrity(format!(
                "materialization `{}` references an absent payload",
                materialization.destination
            )));
        }
        prior_destination = Some(&materialization.destination);
    }
    Ok(())
}

fn verify_locked_resource(
    workspace: &Workspace,
    locked: &LockedResource,
) -> Result<(), ManagerError> {
    let directory = workspace.resources.join(&locked.install_path);
    workspace::validate_existing_ancestors(&workspace.resources, &directory)?;
    let metadata = fs::symlink_metadata(&directory).at(&directory)?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(ManagerError::Integrity(format!(
            "resource directory `{}` is missing or unsafe",
            directory.display()
        )));
    }
    let declared = locked
        .resource
        .payloads
        .iter()
        .map(|payload| payload.path.as_str())
        .collect::<BTreeSet<_>>();
    let installed = collect_files(&directory)?;
    for relative in &installed {
        if !declared.contains(relative.as_str()) {
            return Err(ManagerError::Integrity(format!(
                "resource {} contains undeclared file `{relative}`",
                locked.resource.identity
            )));
        }
    }
    for payload in &locked.resource.payloads {
        let path = directory.join(&payload.path);
        let bytes = fs::read(&path).at(&path)?;
        if bytes.len() as u64 != payload.length || sha256_bytes(&bytes) != payload.sha256 {
            return Err(ManagerError::Integrity(format!(
                "payload `{}` for {} differs from the lock",
                payload.path, locked.resource.identity
            )));
        }
    }
    Ok(())
}

fn installed_state(
    workspace: &Workspace,
    locked: &LockedResource,
) -> Result<InstalledState, ManagerError> {
    match verify_locked_resource(workspace, locked) {
        Ok(()) => Ok(InstalledState::Installed),
        Err(ManagerError::Io { source, .. }) if source.kind() == ErrorKind::NotFound => {
            Ok(InstalledState::Missing)
        }
        Err(ManagerError::Integrity(_)) => Ok(InstalledState::Drifted),
        Err(error) => Err(error),
    }
}

fn verify_materialization(
    workspace: &Workspace,
    materialization: &Materialization,
) -> Result<(), ManagerError> {
    validate_relative_path(&materialization.destination)?;
    let destination = workspace.root.join(&materialization.destination);
    workspace::validate_existing_ancestors(&workspace.root, &destination)?;
    let bytes = fs::read(&destination).at(&destination)?;
    if sha256_bytes(&bytes) != materialization.sha256 {
        return Err(ManagerError::Integrity(format!(
            "materialized file `{}` differs from its record",
            materialization.destination
        )));
    }
    Ok(())
}

fn install_payloads(
    staging: &Path,
    lock: &ResourceLock,
    payloads: &PayloadBytes,
) -> Result<(), ManagerError> {
    for resource in &lock.resources {
        let directory = staging.join(&resource.install_path);
        fs::create_dir_all(&directory).at(&directory)?;
        for payload in &resource.resource.payloads {
            let key = (
                resource.resource.identity.clone(),
                resource.resource.version.clone(),
                payload.path.clone(),
            );
            let bytes = payloads.get(&key).ok_or_else(|| {
                ManagerError::Internal(format!(
                    "transaction is missing payload `{}` for {}",
                    payload.path, resource.resource.identity
                ))
            })?;
            if bytes.len() as u64 != payload.length || sha256_bytes(bytes) != payload.sha256 {
                return Err(ManagerError::Integrity(format!(
                    "staged payload `{}` differs from authenticated metadata",
                    payload.path
                )));
            }
            atomic_write(&directory.join(&payload.path), bytes)?;
        }
    }
    Ok(())
}

fn install_catalog_evidence(
    staging: &Path,
    lock: &ResourceLock,
    evidence: Option<&AuthenticatedCatalogEvidence>,
) -> Result<(), ManagerError> {
    let metadata = staging.join(".metadata");
    if lock.catalog.is_none() {
        if metadata.exists() {
            fs::remove_dir_all(&metadata).at(&metadata)?;
        }
        return Ok(());
    }
    let Some(evidence) = evidence else {
        for path in [
            RETAINED_CATALOG_PATH,
            RETAINED_CATALOG_SIGNATURE_PATH,
            RETAINED_TRUST_ROOT_PATH,
        ] {
            if !staging.join(path).is_file() {
                return Err(ManagerError::Integrity(format!(
                    "transaction cannot preserve missing retained evidence `{path}`"
                )));
            }
        }
        return Ok(());
    };
    if metadata.exists() {
        fs::remove_dir_all(&metadata).at(&metadata)?;
    }
    atomic_write(
        &staging.join(RETAINED_CATALOG_PATH),
        &evidence.catalog_bytes,
    )?;
    write_json(
        &staging.join(RETAINED_CATALOG_SIGNATURE_PATH),
        &evidence.catalog_signature,
    )?;
    write_json(
        &staging.join(RETAINED_TRUST_ROOT_PATH),
        &evidence.trust_root,
    )?;
    Ok(())
}

fn remove_owned_payloads(staging: &Path, lock: &ResourceLock) -> Result<(), ManagerError> {
    for resource in &lock.resources {
        let directory = staging.join(&resource.install_path);
        for payload in &resource.resource.payloads {
            let path = directory.join(&payload.path);
            if path.exists() {
                let current = fs::read(&path).at(&path)?;
                if current.len() as u64 == payload.length
                    && sha256_bytes(&current) == payload.sha256
                {
                    fs::remove_file(&path).at(&path)?;
                    remove_empty_parents(path.parent(), staging)?;
                }
            }
        }
    }
    Ok(())
}

fn remove_empty_parents(mut path: Option<&Path>, boundary: &Path) -> Result<(), ManagerError> {
    while let Some(directory) = path {
        if directory == boundary || !directory.starts_with(boundary) {
            break;
        }
        if fs::read_dir(directory).at(directory)?.next().is_some() {
            break;
        }
        fs::remove_dir(directory).at(directory)?;
        path = directory.parent();
    }
    Ok(())
}

fn copy_tree(source: &Path, destination: &Path) -> Result<(), ManagerError> {
    let metadata = fs::symlink_metadata(source).at(source)?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(ManagerError::Integrity(format!(
            "transaction source `{}` is not a safe directory",
            source.display()
        )));
    }
    fs::create_dir(destination).at(destination)?;
    let mut entries = fs::read_dir(source)
        .at(source)?
        .collect::<Result<Vec<_>, _>>()
        .at(source)?;
    entries.sort_by_key(fs::DirEntry::file_name);
    for entry in entries {
        let path = entry.path();
        let target = destination.join(entry.file_name());
        let metadata = fs::symlink_metadata(&path).at(&path)?;
        if metadata.file_type().is_symlink() {
            return Err(ManagerError::Integrity(format!(
                "resource store contains symlink `{}`",
                path.display()
            )));
        }
        if metadata.is_dir() {
            copy_tree(&path, &target)?;
        } else if metadata.is_file() {
            fs::copy(&path, &target).at(&path)?;
        } else {
            return Err(ManagerError::Integrity(format!(
                "resource store contains unsupported entry `{}`",
                path.display()
            )));
        }
    }
    Ok(())
}

fn collect_files(root: &Path) -> Result<Vec<String>, ManagerError> {
    let mut files = Vec::new();
    collect_files_inner(root, root, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_files_inner(
    root: &Path,
    directory: &Path,
    files: &mut Vec<String>,
) -> Result<(), ManagerError> {
    let mut entries = fs::read_dir(directory)
        .at(directory)?
        .collect::<Result<Vec<_>, _>>()
        .at(directory)?;
    entries.sort_by_key(fs::DirEntry::file_name);
    for entry in entries {
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path).at(&path)?;
        if metadata.file_type().is_symlink() {
            return Err(ManagerError::Integrity(format!(
                "installed resource contains symlink `{}`",
                path.display()
            )));
        }
        if metadata.is_dir() {
            collect_files_inner(root, &path, files)?;
        } else if metadata.is_file() {
            files.push(
                path.strip_prefix(root)
                    .expect("walk remains below root")
                    .to_string_lossy()
                    .replace('\\', "/"),
            );
        } else {
            return Err(ManagerError::Integrity(format!(
                "installed resource contains unsupported entry `{}`",
                path.display()
            )));
        }
    }
    Ok(())
}

fn read_local_payload(root: &Path, object: &str, maximum: u64) -> Result<Vec<u8>, ManagerError> {
    validate_relative_path(object)?;
    let mut current = root.to_path_buf();
    for component in Path::new(object).components() {
        current.push(component);
        let metadata = fs::symlink_metadata(&current).at(&current)?;
        if metadata.file_type().is_symlink() {
            return Err(ManagerError::Integrity(format!(
                "local payload `{object}` traverses a symlink"
            )));
        }
    }
    let canonical = fs::canonicalize(&current).at(&current)?;
    if !canonical.starts_with(root) || !canonical.is_file() {
        return Err(ManagerError::Integrity(format!(
            "local payload `{object}` escapes its resource directory"
        )));
    }
    let metadata = fs::metadata(&canonical).at(&canonical)?;
    if metadata.len() != maximum {
        return Err(ManagerError::Integrity(format!(
            "local payload `{object}` has length {}, expected {maximum}",
            metadata.len()
        )));
    }
    fs::read(&canonical).at(&canonical)
}

fn prepare_projections(
    workspace: &Workspace,
    projections: &[ProjectionMutation],
    transaction_id: &str,
) -> Result<Vec<JournalProjection>, ManagerError> {
    let mut destinations = BTreeSet::new();
    let mut prepared = Vec::new();
    let result = (|| -> Result<(), ManagerError> {
        for (index, projection) in projections.iter().enumerate() {
            if !projection.destination.starts_with(&workspace.root)
                || projection.destination.starts_with(&workspace.liknon_root)
                || !destinations.insert(projection.destination.clone())
            {
                cleanup_projection_backups(&prepared)?;
                return Err(ManagerError::Integrity(format!(
                    "projection destination `{}` is duplicate or outside consumer-owned paths",
                    projection.destination.display()
                )));
            }
            workspace::validate_existing_ancestors(&workspace.root, &projection.destination)?;
            match (&projection.expected_sha256, projection.destination.exists()) {
                (Some(expected), true) => {
                    let bytes = fs::read(&projection.destination).at(&projection.destination)?;
                    if sha256_bytes(&bytes) != *expected {
                        cleanup_projection_backups(&prepared)?;
                        return Err(ManagerError::Conflict(format!(
                            "owned projection `{}` is drifted",
                            projection.destination.display()
                        )));
                    }
                }
                (Some(_), false) => {
                    cleanup_projection_backups(&prepared)?;
                    return Err(ManagerError::Conflict(format!(
                        "owned projection `{}` is missing",
                        projection.destination.display()
                    )));
                }
                (None, true) => {
                    cleanup_projection_backups(&prepared)?;
                    return Err(ManagerError::Conflict(format!(
                        "projection destination `{}` contains unowned bytes",
                        projection.destination.display()
                    )));
                }
                (None, false) => {}
            }
            let parent = projection.destination.parent().ok_or_else(|| {
                ManagerError::Internal("projection destination has no parent".to_owned())
            })?;
            fs::create_dir_all(parent).at(parent)?;
            let file_name = projection
                .destination
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| {
                    ManagerError::Usage(
                        "projection destination has no portable file name".to_owned(),
                    )
                })?;
            let backup = parent.join(format!(
                ".{file_name}.liknon-backup-{transaction_id}-{index}"
            ));
            let temporary = match &projection.action {
                ProjectionAction::Write(bytes) => {
                    let path = parent.join(format!(
                        ".{file_name}.liknon-staging-{transaction_id}-{index}"
                    ));
                    if path.exists() || backup.exists() {
                        cleanup_projection_backups(&prepared)?;
                        return Err(ManagerError::Conflict(format!(
                            "projection transaction path already exists for `{}`",
                            projection.destination.display()
                        )));
                    }
                    write_new_file(&path, bytes)?;
                    Some(path)
                }
                ProjectionAction::Remove => {
                    if backup.exists() {
                        cleanup_projection_backups(&prepared)?;
                        return Err(ManagerError::Conflict(format!(
                            "projection backup already exists for `{}`",
                            projection.destination.display()
                        )));
                    }
                    None
                }
            };
            prepared.push(JournalProjection {
                destination: projection.destination.display().to_string(),
                temporary: temporary.map(|path| path.display().to_string()),
                backup: backup.display().to_string(),
                expected_sha256: projection.expected_sha256.clone(),
                resulting_sha256: match &projection.action {
                    ProjectionAction::Write(bytes) => Some(sha256_bytes(bytes)),
                    ProjectionAction::Remove => None,
                },
            });
        }
        Ok(())
    })();
    if let Err(error) = result {
        cleanup_projection_backups(&prepared)?;
        return Err(error);
    }
    Ok(prepared)
}

fn commit_projections(projections: &[JournalProjection]) -> Result<(), ManagerError> {
    for projection in projections {
        let destination = Path::new(&projection.destination);
        let backup = Path::new(&projection.backup);
        if destination.exists() {
            fs::rename(destination, backup).at(destination)?;
        }
        if let Some(temporary) = &projection.temporary {
            fs::rename(temporary, destination).at(temporary)?;
        }
    }
    Ok(())
}

fn rollback_projections(
    workspace: &Workspace,
    projections: &[JournalProjection],
) -> Result<(), ManagerError> {
    for projection in projections.iter().rev() {
        validate_journal_projection(workspace, projection)?;
        let destination = Path::new(&projection.destination);
        let backup = Path::new(&projection.backup);
        if backup.exists() {
            if destination.exists() {
                verify_resulting_projection(destination, projection.resulting_sha256.as_deref())?;
                fs::remove_file(destination).at(destination)?;
            }
            fs::rename(backup, destination).at(backup)?;
        } else if projection.expected_sha256.is_none() && destination.exists() {
            verify_resulting_projection(destination, projection.resulting_sha256.as_deref())?;
            fs::remove_file(destination).at(destination)?;
        }
        if let Some(temporary) = &projection.temporary {
            let temporary = Path::new(temporary);
            if temporary.exists() {
                fs::remove_file(temporary).at(temporary)?;
            }
        }
    }
    Ok(())
}

fn finish_committed_projections(
    workspace: &Workspace,
    projections: &[JournalProjection],
) -> Result<(), ManagerError> {
    for projection in projections {
        validate_journal_projection(workspace, projection)?;
        let destination = Path::new(&projection.destination);
        match &projection.resulting_sha256 {
            Some(expected) => {
                let bytes = fs::read(destination).at(destination)?;
                if sha256_bytes(&bytes) != *expected {
                    return Err(ManagerError::Integrity(format!(
                        "committed projection `{}` differs from its journal",
                        destination.display()
                    )));
                }
            }
            None if destination.exists() => {
                return Err(ManagerError::Integrity(format!(
                    "removed projection `{}` reappeared during recovery",
                    destination.display()
                )));
            }
            None => {}
        }
    }
    cleanup_projection_backups(projections)
}

fn cleanup_projection_backups(projections: &[JournalProjection]) -> Result<(), ManagerError> {
    for projection in projections {
        for path in [
            Some(projection.backup.as_str()),
            projection.temporary.as_deref(),
        ]
        .into_iter()
        .flatten()
        {
            let path = Path::new(path);
            if path.exists() {
                fs::remove_file(path).at(path)?;
            }
        }
    }
    Ok(())
}

fn validate_journal_projection(
    workspace: &Workspace,
    projection: &JournalProjection,
) -> Result<(), ManagerError> {
    let destination = Path::new(&projection.destination);
    let backup = Path::new(&projection.backup);
    let valid = destination.starts_with(&workspace.root)
        && !destination.starts_with(&workspace.liknon_root)
        && backup.parent() == destination.parent()
        && projection
            .temporary
            .as_deref()
            .is_none_or(|path| Path::new(path).parent() == destination.parent());
    if !valid {
        return Err(ManagerError::Integrity(
            "transaction journal contains an unsafe projection path".to_owned(),
        ));
    }
    workspace::validate_existing_ancestors(&workspace.root, destination)
}

fn verify_resulting_projection(
    destination: &Path,
    resulting_sha256: Option<&str>,
) -> Result<(), ManagerError> {
    let expected = resulting_sha256.ok_or_else(|| {
        ManagerError::Integrity(format!(
            "unexpected bytes at removed projection `{}`",
            destination.display()
        ))
    })?;
    let bytes = fs::read(destination).at(destination)?;
    if sha256_bytes(&bytes) != expected {
        return Err(ManagerError::Integrity(format!(
            "projection `{}` changed during transaction recovery",
            destination.display()
        )));
    }
    Ok(())
}

fn write_new_file(path: &Path, bytes: &[u8]) -> Result<(), ManagerError> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .at(path)?;
    file.write_all(bytes).at(path)?;
    file.sync_all().at(path)
}

fn recover_locked(workspace: &Workspace) -> Result<(), ManagerError> {
    let journal_path = journal_path(workspace);
    if !journal_path.exists() {
        return Ok(());
    }
    let journal: TransactionJournal =
        read_integrity_json(&journal_path, "Manager transaction journal")?;
    if journal.schema_version != 1 {
        return Err(ManagerError::Integrity(
            "unsupported transaction journal schema".to_owned(),
        ));
    }
    let staging = PathBuf::from(&journal.staging);
    let backup = PathBuf::from(&journal.backup);
    if !staging.starts_with(&workspace.manager_state)
        || !backup.starts_with(&workspace.manager_state)
    {
        return Err(ManagerError::Integrity(
            "transaction journal paths escape Manager state".to_owned(),
        ));
    }
    match journal.phase {
        TransactionPhase::Prepared => {
            if backup.exists() {
                if workspace.resources.exists() {
                    return Err(ManagerError::Integrity(
                        "prepared transaction has both live and backup resource stores".to_owned(),
                    ));
                }
                fs::rename(&backup, &workspace.resources).at(&backup)?;
            }
            if staging.exists() {
                fs::remove_dir_all(&staging).at(&staging)?;
            }
            rollback_projections(workspace, &journal.projections)?;
        }
        TransactionPhase::BackedUp => {
            if backup.exists() {
                if workspace.resources.exists() {
                    fs::remove_dir_all(&workspace.resources).at(&workspace.resources)?;
                }
                fs::rename(&backup, &workspace.resources).at(&backup)?;
            } else if workspace.resources.exists() && staging.exists() {
                // The attempted store swap failed after the backup was made, and
                // the previous store was restored before the process stopped.
            } else {
                return Err(ManagerError::Integrity(
                    "backed-up transaction has no previous resource store".to_owned(),
                ));
            }
            if staging.exists() {
                fs::remove_dir_all(&staging).at(&staging)?;
            }
            rollback_projections(workspace, &journal.projections)?;
        }
        TransactionPhase::Committed => {
            if !workspace.resources.exists() {
                if staging.exists() {
                    fs::rename(&staging, &workspace.resources).at(&staging)?;
                } else {
                    return Err(ManagerError::Integrity(
                        "committed transaction has no resulting resource store".to_owned(),
                    ));
                }
            }
            finish_committed_projections(workspace, &journal.projections)?;
            if backup.exists() {
                fs::remove_dir_all(&backup).at(&backup)?;
            }
            if staging.exists() {
                fs::remove_dir_all(&staging).at(&staging)?;
            }
        }
    }
    fs::remove_file(&journal_path).at(&journal_path)?;
    Ok(())
}

fn write_journal(workspace: &Workspace, journal: &TransactionJournal) -> Result<(), ManagerError> {
    write_json(&journal_path(workspace), journal)
}

fn journal_path(workspace: &Workspace) -> PathBuf {
    workspace.manager_state.join("transaction.json")
}

fn transaction_id() -> Result<String, ManagerError> {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| ManagerError::Internal(error.to_string()))?
        .as_nanos();
    Ok(format!("{}-{nanos}", std::process::id()))
}

fn sync_tree(root: &Path) -> Result<(), ManagerError> {
    for file in collect_files(root)? {
        let path = root.join(file);
        fs::File::open(&path).at(&path)?.sync_all().at(&path)?;
    }
    #[cfg(unix)]
    fs::File::open(root).at(root)?.sync_all().at(root)?;
    Ok(())
}

fn read_manifest_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, ManagerError> {
    let bytes = fs::read(path).at(path)?;
    serde_json::from_slice(&bytes).map_err(|error| {
        ManagerError::Usage(format!(
            "resource manifest `{}` is malformed JSON: {error}",
            path.display()
        ))
    })
}

fn read_integrity_json<T: serde::de::DeserializeOwned>(
    path: &Path,
    label: &str,
) -> Result<T, ManagerError> {
    let bytes = fs::read(path).at(path)?;
    serde_json::from_slice(&bytes).map_err(|error| {
        ManagerError::Integrity(format!(
            "{label} `{}` is malformed JSON: {error}",
            path.display()
        ))
    })
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), ManagerError> {
    let mut bytes = serde_json::to_vec_pretty(value).map_err(|source| ManagerError::Json {
        path: path.to_path_buf(),
        source,
    })?;
    bytes.push(b'\n');
    atomic_write(path, &bytes)
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), ManagerError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).at(parent)?;
    }
    let replace = match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => {
            return Err(ManagerError::Integrity(format!(
                "atomic write target `{}` is not a regular file",
                path.display()
            )));
        }
        Ok(_) => true,
        Err(error) if error.kind() == ErrorKind::NotFound => false,
        Err(source) => {
            return Err(ManagerError::Io {
                path: path.to_path_buf(),
                source,
            });
        }
    };
    let temporary = path.with_extension(format!("tmp-{}", transaction_id()?));
    let result = (|| {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .at(&temporary)?;
        file.write_all(bytes).at(&temporary)?;
        file.sync_all().at(&temporary)?;
        if replace {
            fs::remove_file(path).at(path)?;
        }
        fs::rename(&temporary, path).at(&temporary)?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result
}

#[cfg(test)]
mod tests {
    use std::fs;

    use liknon_resource_contracts::sha256_bytes;
    use tempfile::TempDir;

    use crate::workspace::Workspace;

    use super::{
        copy_tree, initialize, journal_path, load, load_local_resource, recover, write_journal,
        JournalProjection, TransactionJournal, TransactionPhase,
    };

    #[test]
    fn initialization_is_idempotent_for_exact_empty_state() {
        let temporary = TempDir::new().unwrap();
        let root = fs::canonicalize(temporary.path()).unwrap();
        let workspace = Workspace {
            liknon_root: root.join(".liknon"),
            resources: root.join(".liknon/resources"),
            manager_state: root.join(".liknon/.manager"),
            root,
        };
        initialize(&workspace).unwrap();
        initialize(&workspace).unwrap();
        let state = load(&workspace).unwrap();
        assert!(state.manifest.resources.is_empty());
        assert!(state.lock.resources.is_empty());
    }

    #[test]
    fn prepared_recovery_restores_the_previous_store_after_an_early_interruption() {
        let (_temporary, workspace) = fixture_workspace();
        initialize(&workspace).unwrap();
        let transactions = workspace.manager_state.join("transactions");
        fs::create_dir_all(&transactions).unwrap();
        let staging = transactions.join("prepared-resources");
        let backup = transactions.join("prepared-backup");
        copy_tree(&workspace.resources, &staging).unwrap();
        fs::rename(&workspace.resources, &backup).unwrap();
        write_journal(
            &workspace,
            &TransactionJournal {
                schema_version: 1,
                phase: TransactionPhase::Prepared,
                staging: staging.display().to_string(),
                backup: backup.display().to_string(),
                projections: Vec::new(),
            },
        )
        .unwrap();

        recover(&workspace).unwrap();

        assert!(workspace.resources.is_dir());
        assert!(!staging.exists());
        assert!(!backup.exists());
        assert!(!journal_path(&workspace).exists());
        load(&workspace).unwrap();
    }

    #[test]
    fn backed_up_recovery_rolls_back_store_and_projection_together() {
        let (_temporary, workspace) = fixture_workspace();
        initialize(&workspace).unwrap();
        let transactions = workspace.manager_state.join("transactions");
        fs::create_dir_all(&transactions).unwrap();
        let staging = transactions.join("backed-up-resources");
        let backup = transactions.join("backed-up-backup");
        fs::write(workspace.resources.join("old-marker"), b"old").unwrap();
        fs::rename(&workspace.resources, &backup).unwrap();
        copy_tree(&backup, &workspace.resources).unwrap();
        fs::write(workspace.resources.join("old-marker"), b"new").unwrap();

        let destination = workspace.root.join("generated/context.md");
        fs::create_dir_all(destination.parent().unwrap()).unwrap();
        let projection_backup = workspace.root.join("generated/.context.md.backup");
        fs::write(&projection_backup, b"old projection").unwrap();
        fs::write(&destination, b"new projection").unwrap();
        let projection = JournalProjection {
            destination: destination.display().to_string(),
            temporary: None,
            backup: projection_backup.display().to_string(),
            expected_sha256: Some(sha256_bytes(b"old projection")),
            resulting_sha256: Some(sha256_bytes(b"new projection")),
        };
        write_journal(
            &workspace,
            &TransactionJournal {
                schema_version: 1,
                phase: TransactionPhase::BackedUp,
                staging: staging.display().to_string(),
                backup: backup.display().to_string(),
                projections: vec![projection],
            },
        )
        .unwrap();

        recover(&workspace).unwrap();

        assert_eq!(
            fs::read(workspace.resources.join("old-marker")).unwrap(),
            b"old"
        );
        assert_eq!(fs::read(destination).unwrap(), b"old projection");
        assert!(!projection_backup.exists());
        assert!(!journal_path(&workspace).exists());
    }

    #[test]
    fn backed_up_recovery_accepts_an_already_restored_previous_store() {
        let (_temporary, workspace) = fixture_workspace();
        initialize(&workspace).unwrap();
        fs::write(workspace.resources.join("old-marker"), b"old").unwrap();
        let transactions = workspace.manager_state.join("transactions");
        fs::create_dir_all(&transactions).unwrap();
        let staging = transactions.join("restored-resources");
        let backup = transactions.join("restored-backup");
        copy_tree(&workspace.resources, &staging).unwrap();
        write_journal(
            &workspace,
            &TransactionJournal {
                schema_version: 1,
                phase: TransactionPhase::BackedUp,
                staging: staging.display().to_string(),
                backup: backup.display().to_string(),
                projections: Vec::new(),
            },
        )
        .unwrap();

        recover(&workspace).unwrap();

        assert_eq!(
            fs::read(workspace.resources.join("old-marker")).unwrap(),
            b"old"
        );
        assert!(!staging.exists());
        assert!(!journal_path(&workspace).exists());
    }

    #[test]
    fn committed_recovery_keeps_new_store_and_projection() {
        let (_temporary, workspace) = fixture_workspace();
        initialize(&workspace).unwrap();
        let transactions = workspace.manager_state.join("transactions");
        fs::create_dir_all(&transactions).unwrap();
        let staging = transactions.join("committed-resources");
        let backup = transactions.join("committed-backup");
        copy_tree(&workspace.resources, &backup).unwrap();
        fs::write(workspace.resources.join("new-marker"), b"new").unwrap();

        let destination = workspace.root.join("generated/context.md");
        fs::create_dir_all(destination.parent().unwrap()).unwrap();
        let projection_backup = workspace.root.join("generated/.context.md.backup");
        fs::write(&projection_backup, b"old projection").unwrap();
        fs::write(&destination, b"new projection").unwrap();
        write_journal(
            &workspace,
            &TransactionJournal {
                schema_version: 1,
                phase: TransactionPhase::Committed,
                staging: staging.display().to_string(),
                backup: backup.display().to_string(),
                projections: vec![JournalProjection {
                    destination: destination.display().to_string(),
                    temporary: None,
                    backup: projection_backup.display().to_string(),
                    expected_sha256: Some(sha256_bytes(b"old projection")),
                    resulting_sha256: Some(sha256_bytes(b"new projection")),
                }],
            },
        )
        .unwrap();

        recover(&workspace).unwrap();

        assert_eq!(
            fs::read(workspace.resources.join("new-marker")).unwrap(),
            b"new"
        );
        assert_eq!(fs::read(destination).unwrap(), b"new projection");
        assert!(!backup.exists());
        assert!(!projection_backup.exists());
        assert!(!journal_path(&workspace).exists());
    }

    #[cfg(unix)]
    #[test]
    fn explicit_local_resource_root_cannot_be_a_symlink() {
        use std::os::unix::fs::symlink;

        let temporary = TempDir::new().unwrap();
        let target = temporary.path().join("target");
        let source = temporary.path().join("source");
        fs::create_dir(&target).unwrap();
        symlink(&target, &source).unwrap();

        assert!(load_local_resource(&source).is_err());
    }

    #[test]
    fn concurrent_manager_process_is_blocked_before_recovery() {
        let (_temporary, workspace) = fixture_workspace();
        initialize(&workspace).unwrap();
        fs::create_dir_all(&workspace.manager_state).unwrap();
        let lock_path = workspace.manager_state.join("manager.lock");
        let lock = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&lock_path)
            .unwrap();
        fs2::FileExt::lock_exclusive(&lock).unwrap();

        assert!(matches!(
            recover(&workspace),
            Err(crate::error::ManagerError::Blocked(_))
        ));

        fs2::FileExt::unlock(&lock).unwrap();
    }

    fn fixture_workspace() -> (TempDir, Workspace) {
        let temporary = TempDir::new().unwrap();
        let root = fs::canonicalize(temporary.path()).unwrap();
        let workspace = Workspace {
            liknon_root: root.join(".liknon"),
            resources: root.join(".liknon/resources"),
            manager_state: root.join(".liknon/.manager"),
            root,
        };
        (temporary, workspace)
    }
}
