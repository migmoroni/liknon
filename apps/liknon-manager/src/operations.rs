use std::{
    collections::{BTreeMap, BTreeSet},
    io::{self, IsTerminal as _, Write as _},
    path::{Path, PathBuf},
};

use liknon_resource_contracts::{
    CatalogSnapshot, LockedCatalog, LockedResource, LockedResourceOrigin, RequestedSource,
    ResourceIdentity, ResourceLock, ResourceManifest, ResourceRequest, ResourceSpec,
    ResourceVersion, RESOURCE_LOCK_SCHEMA_VERSION, RETAINED_CATALOG_PATH,
    RETAINED_CATALOG_SIGNATURE_PATH, RETAINED_TRUST_ROOT_PATH,
};
use semver::{Version, VersionReq};

use crate::{
    cli::{Cli, Command, HubArgs, MutationArgs, OutputFormat, SchemaContract, WorkspaceArgs},
    error::ManagerError,
    hub::{self, HubSnapshot},
    output::{
        ChangeAction, ExactResource, FileAction, ManagerReport, MutationPlan, PlannedChange,
        PlannedDependency, PlannedEffect, PlannedFile, PlannedRequest, ReportPayload,
    },
    resolver::{self, AvailableResource},
    store::{self, PayloadBytes, StoreState},
    workspace::{self, Workspace},
};

pub(crate) fn execute(cli: Cli) -> Result<ManagerReport, ManagerError> {
    let format = cli.format();
    match cli.command {
        Command::Init { workspace } => initialize(workspace),
        Command::Search {
            query,
            kind,
            namespace,
            trust,
            hub,
        } => search(query, kind, namespace, trust, hub),
        Command::Info {
            resource,
            installed,
            workspace,
            hub,
        } => info(&resource, installed, workspace, hub),
        Command::Install {
            resources,
            local,
            allow_community,
            workspace,
            hub,
            mutation,
        } => install(
            resources,
            local,
            allow_community,
            workspace,
            hub,
            mutation,
            format,
        ),
        Command::Update {
            resources,
            allow_community,
            workspace,
            hub,
            mutation,
        } => update(resources, allow_community, workspace, hub, mutation, format),
        Command::Remove {
            resources,
            workspace,
            mutation,
        } => remove(resources, workspace, mutation, format),
        Command::List { workspace } => list(workspace),
        Command::Verify {
            offline,
            workspace,
            hub,
        } => verify(offline, workspace, hub),
        Command::Materialize {
            resource,
            payload,
            destination,
            workspace,
            mutation,
        } => materialize(
            &resource,
            &payload,
            &destination,
            workspace,
            mutation,
            format,
        ),
        Command::Schema { contract } => schema(contract),
    }
}

fn initialize(args: WorkspaceArgs) -> Result<ManagerReport, ManagerError> {
    let workspace = workspace::for_init(args.workspace.as_deref())?;
    store::initialize(&workspace)?;
    Ok(ManagerReport::completed(
        "init",
        Some(workspace.display()),
        ReportPayload::Initialized,
    ))
}

fn search(
    query: String,
    kind: Option<liknon_resource_contracts::ResourceKind>,
    namespace: Option<String>,
    trust: Option<liknon_resource_contracts::TrustClass>,
    args: HubArgs,
) -> Result<ManagerReport, ManagerError> {
    let hub = hub::load(&args)?;
    let query = query.to_ascii_lowercase();
    let mut results = hub
        .catalog
        .resources
        .iter()
        .filter(|resource| {
            let text = format!(
                "{} {} {}",
                resource.identity, resource.title, resource.summary
            )
            .to_ascii_lowercase();
            (query.is_empty() || text.contains(&query))
                && kind.is_none_or(|kind| resource.identity.kind == kind)
                && namespace
                    .as_deref()
                    .is_none_or(|namespace| resource.identity.namespace == namespace)
                && trust.is_none_or(|trust| resource.trust.class == trust)
        })
        .cloned()
        .collect::<Vec<_>>();
    results.sort_by(resource_order);
    Ok(ManagerReport::completed(
        "search",
        None,
        ReportPayload::Search {
            snapshot: hub.catalog.snapshot,
            results,
        },
    ))
}

fn info(
    value: &str,
    installed: bool,
    workspace_args: WorkspaceArgs,
    hub_args: HubArgs,
) -> Result<ManagerReport, ManagerError> {
    let spec: ResourceSpec = value.parse()?;
    let (resource, workspace) = if installed {
        let workspace = workspace::discover(workspace_args.workspace.as_deref())?;
        store::recover(&workspace)?;
        let state = store::load(&workspace)?;
        let resource = newest_matching(
            state.lock.resources.iter().map(|locked| &locked.resource),
            &spec,
        )?
        .clone();
        (resource, Some(workspace.display()))
    } else {
        let hub = hub::load(&hub_args)?;
        let resource = newest_matching(hub.catalog.resources.iter(), &spec)?.clone();
        (resource, None)
    };
    Ok(ManagerReport::completed(
        "info",
        workspace,
        ReportPayload::Info {
            resource: Box::new(resource),
            installed,
        },
    ))
}

fn install(
    resource_specs: Vec<String>,
    local_paths: Vec<PathBuf>,
    allow_community: bool,
    workspace_args: WorkspaceArgs,
    hub_args: HubArgs,
    mutation: MutationArgs,
    format: OutputFormat,
) -> Result<ManagerReport, ManagerError> {
    if resource_specs.is_empty() && local_paths.is_empty() {
        return Err(ManagerError::Usage(
            "install requires at least one Hub resource or --local directory".to_owned(),
        ));
    }
    let workspace = existing_workspace(&workspace_args)?;
    let current = checked_state(&workspace)?;
    let mut manifest = current.manifest.clone();
    let mut update = BTreeSet::new();
    for value in resource_specs {
        let spec: ResourceSpec = value.parse()?;
        update.insert(spec.identity.clone());
        upsert_request(
            &mut manifest,
            ResourceRequest {
                identity: spec.identity,
                requirement: spec.requirement,
                source: RequestedSource::Hub,
            },
        );
    }
    for path in local_paths {
        let available = store::load_local_resource(&path)?;
        let LockedResourceOrigin::Local { path } = &available.origin else {
            unreachable!("local loader always returns a local origin");
        };
        update.insert(available.resource.identity.clone());
        upsert_request(
            &mut manifest,
            ResourceRequest {
                identity: available.resource.identity.clone(),
                requirement: format!("={}", available.resource.version),
                source: RequestedSource::Local { path: path.clone() },
            },
        );
    }
    let hub_required = manifest.resources.iter().any(|request| {
        update.contains(&request.identity) && matches!(request.source, RequestedSource::Hub)
    });
    let hub = load_optional_hub(&hub_args, hub_required)?;
    mutate_resolved(
        "install",
        workspace,
        current,
        manifest,
        hub,
        update,
        allow_community,
        mutation,
        format,
    )
}

fn update(
    resources: Vec<String>,
    allow_community: bool,
    workspace_args: WorkspaceArgs,
    hub_args: HubArgs,
    mutation: MutationArgs,
    format: OutputFormat,
) -> Result<ManagerReport, ManagerError> {
    let workspace = existing_workspace(&workspace_args)?;
    let current = checked_state(&workspace)?;
    let update = if resources.is_empty() {
        current
            .manifest
            .resources
            .iter()
            .map(|request| request.identity.clone())
            .collect::<BTreeSet<_>>()
    } else {
        resources
            .iter()
            .map(|value| parse_identity(value))
            .collect::<Result<BTreeSet<_>, _>>()?
    };
    for identity in &update {
        if !current
            .manifest
            .resources
            .iter()
            .any(|request| &request.identity == identity)
        {
            return Err(ManagerError::Usage(format!(
                "resource `{identity}` is not a desired root"
            )));
        }
    }
    let hub_required = current.manifest.resources.iter().any(|request| {
        matches!(request.source, RequestedSource::Hub) && update.contains(&request.identity)
    });
    let hub = load_optional_hub(&hub_args, hub_required)?;
    mutate_resolved(
        "update",
        workspace,
        current.clone(),
        current.manifest,
        hub,
        update,
        allow_community,
        mutation,
        format,
    )
}

fn remove(
    resources: Vec<String>,
    workspace_args: WorkspaceArgs,
    mutation: MutationArgs,
    format: OutputFormat,
) -> Result<ManagerReport, ManagerError> {
    let workspace = existing_workspace(&workspace_args)?;
    let current = checked_state(&workspace)?;
    let identities = resources
        .iter()
        .map(|value| parse_identity(value))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let mut manifest = current.manifest.clone();
    let prior = manifest.resources.len();
    manifest
        .resources
        .retain(|request| !identities.contains(&request.identity));
    if prior - manifest.resources.len() != identities.len() {
        return Err(ManagerError::Usage(
            "remove accepts only identities that are current desired roots".to_owned(),
        ));
    }
    mutate_resolved(
        "remove",
        workspace,
        current,
        manifest,
        None,
        BTreeSet::new(),
        true,
        mutation,
        format,
    )
}

fn list(args: WorkspaceArgs) -> Result<ManagerReport, ManagerError> {
    let workspace = existing_workspace(&args)?;
    store::recover(&workspace)?;
    let state = store::load(&workspace)?;
    let resources = store::state_rows(&workspace, &state)?;
    let materializations = store::materialization_rows(&workspace, &state)?;
    Ok(ManagerReport::completed(
        "list",
        Some(workspace.display()),
        ReportPayload::State {
            resources,
            materializations,
        },
    ))
}

fn verify(
    offline: bool,
    workspace_args: WorkspaceArgs,
    hub_args: HubArgs,
) -> Result<ManagerReport, ManagerError> {
    let workspace = existing_workspace(&workspace_args)?;
    store::recover(&workspace)?;
    let state = store::load(&workspace)?;
    let mut verification = store::verify(&workspace, &state)?;
    if !offline {
        let hub = hub::load(&hub_args)?;
        verify_current_catalog(&state.lock, &hub, &mut verification.warnings)?;
    }
    Ok(ManagerReport::completed(
        "verify",
        Some(workspace.display()),
        ReportPayload::Verification {
            verified_resources: verification.resources,
            verified_payloads: verification.payloads,
            online: !offline,
            warnings: verification.warnings,
        },
    ))
}

fn materialize(
    resource: &str,
    payload: &str,
    destination: &Path,
    workspace_args: WorkspaceArgs,
    mutation: MutationArgs,
    format: OutputFormat,
) -> Result<ManagerReport, ManagerError> {
    let identity = parse_identity(resource)?;
    let workspace = existing_workspace(&workspace_args)?;
    let current = checked_state(&workspace)?;
    let (proposed_lock, projection) =
        store::prepare_materialization(&workspace, &current, &identity, payload, destination)?;
    let plan = mutation_plan(
        "materialize",
        &current.manifest,
        &current.manifest,
        &current.lock,
        &proposed_lock,
    )?;
    if mutation.dry_run {
        return Ok(planned_report("materialize", &workspace, plan));
    }
    confirm(&plan, mutation.yes, format)?;
    let payloads = collect_payloads(&workspace, &current, &proposed_lock, None)?;
    store::apply(
        &workspace,
        &current.manifest,
        &proposed_lock,
        &payloads,
        None,
        &[projection],
    )?;
    let mut plan = plan;
    plan.committed = true;
    Ok(completed_mutation("materialize", &workspace, plan))
}

fn schema(contract: SchemaContract) -> Result<ManagerReport, ManagerError> {
    let value = match contract {
        SchemaContract::Resource => schema_value(schemars::schema_for!(ResourceVersion))?,
        SchemaContract::Catalog => schema_value(schemars::schema_for!(CatalogSnapshot))?,
        SchemaContract::Manifest => schema_value(schemars::schema_for!(ResourceManifest))?,
        SchemaContract::Lock => schema_value(schemars::schema_for!(ResourceLock))?,
        SchemaContract::TrustRoot => {
            schema_value(schemars::schema_for!(liknon_resource_contracts::TrustRoot))?
        }
        SchemaContract::Signature => schema_value(schemars::schema_for!(
            liknon_resource_contracts::DetachedSignature
        ))?,
    };
    Ok(ManagerReport::completed(
        "schema",
        None,
        ReportPayload::Schema { schema: value },
    ))
}

#[allow(clippy::too_many_arguments)]
fn mutate_resolved(
    operation: &str,
    workspace: Workspace,
    current: StoreState,
    manifest: ResourceManifest,
    hub: Option<HubSnapshot>,
    update: BTreeSet<ResourceIdentity>,
    allow_community: bool,
    mutation: MutationArgs,
    format: OutputFormat,
) -> Result<ManagerReport, ManagerError> {
    let available = available_resources(&workspace, &manifest, hub.as_ref())?;
    let resolution = resolver::resolve(
        &manifest,
        available,
        Some(&current.lock),
        &update,
        allow_community,
    )?;
    let lock = resolved_lock(&current.lock, resolution.resources, hub.as_ref())?;
    let plan = mutation_plan(
        operation,
        &current.manifest,
        &manifest,
        &current.lock,
        &lock,
    )?;
    if mutation.dry_run {
        return Ok(planned_report(operation, &workspace, plan));
    }
    confirm(&plan, mutation.yes, format)?;
    let payloads = collect_payloads(&workspace, &current, &lock, hub.as_ref())?;
    let evidence = hub.as_ref().and_then(|hub| {
        lock.catalog
            .as_ref()
            .filter(|catalog| catalog.sha256 == hub.catalog_sha256)
            .map(|_| &hub.evidence)
    });
    let projections = store::removed_materializations(&workspace, &current.lock, &lock)?;
    store::apply(
        &workspace,
        &manifest,
        &lock,
        &payloads,
        evidence,
        &projections,
    )?;
    let mut plan = plan;
    plan.committed = true;
    Ok(completed_mutation(operation, &workspace, plan))
}

fn available_resources(
    workspace: &Workspace,
    manifest: &ResourceManifest,
    hub: Option<&HubSnapshot>,
) -> Result<Vec<AvailableResource>, ManagerError> {
    let mut available = hub
        .into_iter()
        .flat_map(|hub| {
            hub.catalog
                .resources
                .iter()
                .cloned()
                .map(|resource| AvailableResource {
                    resource,
                    origin: LockedResourceOrigin::Hub {
                        origin: hub.origin.clone(),
                    },
                })
        })
        .collect::<Vec<_>>();
    for request in &manifest.resources {
        if let RequestedSource::Local { path } = &request.source {
            let source = Path::new(path);
            let source = if source.is_absolute() {
                source.to_path_buf()
            } else {
                workspace.root.join(source)
            };
            let local = store::load_local_resource(&source)?;
            if local.resource.identity != request.identity {
                return Err(ManagerError::Integrity(format!(
                    "local source `{}` declares {}, expected {}",
                    source.display(),
                    local.resource.identity,
                    request.identity
                )));
            }
            available.push(local);
        }
    }
    Ok(available)
}

fn resolved_lock(
    current: &ResourceLock,
    resources: Vec<LockedResource>,
    hub: Option<&HubSnapshot>,
) -> Result<ResourceLock, ManagerError> {
    for resolved in &resources {
        let previous = current.resources.iter().find(|previous| {
            previous.resource.identity == resolved.resource.identity
                && previous.resource.version == resolved.resource.version
        });
        if let Some(previous) = previous {
            if previous.manifest_sha256 != resolved.manifest_sha256 {
                return Err(ManagerError::Integrity(format!(
                    "immutable resource {}@{} changed metadata without a version change",
                    resolved.resource.identity, resolved.resource.version
                )));
            }
        }
    }
    let has_hub = resources
        .iter()
        .any(|resource| matches!(resource.origin, LockedResourceOrigin::Hub { .. }));
    let catalog = if !has_hub {
        None
    } else if let Some(hub) = hub {
        for locked in resources
            .iter()
            .filter(|resource| matches!(resource.origin, LockedResourceOrigin::Hub { .. }))
        {
            let authenticated = hub.catalog.resources.iter().any(|resource| {
                resource.identity == locked.resource.identity
                    && resource.version == locked.resource.version
                    && liknon_resource_contracts::canonical_json_sha256(resource).ok()
                        == Some(locked.manifest_sha256.clone())
            });
            if !authenticated {
                return Err(ManagerError::Integrity(format!(
                    "resolved Hub resource {}@{} is absent from the selected authenticated catalog",
                    locked.resource.identity, locked.resource.version
                )));
            }
        }
        Some(LockedCatalog {
            origin: hub.origin.clone(),
            snapshot: hub.catalog.snapshot.clone(),
            sha256: hub.catalog_sha256.clone(),
            trust_authority: hub.trust_authority.clone(),
            signing_key_id: hub.signing_key_id.clone(),
            catalog_path: RETAINED_CATALOG_PATH.to_owned(),
            catalog_signature_path: RETAINED_CATALOG_SIGNATURE_PATH.to_owned(),
            trust_root_path: RETAINED_TRUST_ROOT_PATH.to_owned(),
        })
    } else {
        current
            .catalog
            .clone()
            .ok_or_else(|| {
                ManagerError::Integrity(
                    "Hub-backed resolution has no retained authenticated catalog".to_owned(),
                )
            })?
            .into()
    };
    let retained = resources
        .iter()
        .map(|resource| {
            (
                resource.resource.identity.clone(),
                resource.resource.version.clone(),
            )
        })
        .collect::<BTreeSet<_>>();
    let mut materializations = current
        .materializations
        .iter()
        .filter(|record| retained.contains(&(record.identity.clone(), record.version.clone())))
        .cloned()
        .collect::<Vec<_>>();
    materializations.sort_by(|left, right| left.destination.cmp(&right.destination));
    Ok(ResourceLock {
        schema_version: RESOURCE_LOCK_SCHEMA_VERSION,
        catalog,
        resources,
        materializations,
    })
}

fn collect_payloads(
    workspace: &Workspace,
    current: &StoreState,
    lock: &ResourceLock,
    hub: Option<&HubSnapshot>,
) -> Result<PayloadBytes, ManagerError> {
    let existing = current
        .lock
        .resources
        .iter()
        .map(|resource| {
            (
                (
                    resource.resource.identity.clone(),
                    resource.resource.version.clone(),
                ),
                resource,
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut payloads = PayloadBytes::new();
    for locked in &lock.resources {
        for payload in &locked.resource.payloads {
            let bytes = if let Some(existing) = existing.get(&(
                locked.resource.identity.clone(),
                locked.resource.version.clone(),
            )) {
                store::installed_payload(workspace, existing, &payload.path)?
            } else {
                match &locked.origin {
                    LockedResourceOrigin::Hub { origin } => {
                        let hub = hub.ok_or_else(|| {
                            ManagerError::Blocked(format!(
                                "resource {}@{} requires authenticated Hub access",
                                locked.resource.identity, locked.resource.version
                            ))
                        })?;
                        if &hub.origin != origin {
                            return Err(ManagerError::Integrity(format!(
                                "resource {} resolved from an unexpected Hub",
                                locked.resource.identity
                            )));
                        }
                        hub.payload(payload)?
                    }
                    LockedResourceOrigin::Local { path } => {
                        store::local_payload(Path::new(path), &payload.object, payload.length)?
                    }
                }
            };
            payloads.insert(
                (
                    locked.resource.identity.clone(),
                    locked.resource.version.clone(),
                    payload.path.clone(),
                ),
                bytes,
            );
        }
    }
    Ok(payloads)
}

fn mutation_plan(
    operation: &str,
    current_manifest: &ResourceManifest,
    proposed_manifest: &ResourceManifest,
    current: &ResourceLock,
    proposed: &ResourceLock,
) -> Result<MutationPlan, ManagerError> {
    let current_map = current
        .resources
        .iter()
        .map(|resource| (&resource.resource.identity, resource))
        .collect::<BTreeMap<_, _>>();
    let proposed_map = proposed
        .resources
        .iter()
        .map(|resource| (&resource.resource.identity, resource))
        .collect::<BTreeMap<_, _>>();
    let identities = current_map
        .keys()
        .chain(proposed_map.keys())
        .copied()
        .collect::<BTreeSet<_>>();
    let newly_materialized = proposed
        .materializations
        .iter()
        .filter(|record| !current.materializations.contains(record))
        .map(|record| &record.identity)
        .collect::<BTreeSet<_>>();
    let mut changes = Vec::new();
    let mut downloads = 0;
    let mut download_bytes = 0;
    for identity in identities {
        let from = current_map.get(identity).copied();
        let to = proposed_map.get(identity).copied();
        let action = match (from, to) {
            (None, Some(_)) => ChangeAction::Add,
            (Some(_), None) => ChangeAction::Remove,
            (Some(from), Some(to)) if from.resource.version != to.resource.version => {
                ChangeAction::Update
            }
            (Some(_), Some(_)) if newly_materialized.contains(identity) => {
                ChangeAction::Materialize
            }
            (Some(_), Some(_)) => ChangeAction::Retain,
            (None, None) => unreachable!(),
        };
        if matches!(action, ChangeAction::Add | ChangeAction::Update)
            && to
                .is_some_and(|resource| matches!(resource.origin, LockedResourceOrigin::Hub { .. }))
        {
            let resource = to.expect("added or updated resource exists");
            downloads += resource.resource.payloads.len();
            download_bytes += resource
                .resource
                .payloads
                .iter()
                .map(|payload| payload.length)
                .sum::<u64>();
        }
        changes.push(PlannedChange {
            action,
            identity: identity.clone(),
            from_version: from.map(|resource| resource.resource.version.clone()),
            to_version: to.map(|resource| resource.resource.version.clone()),
        });
    }
    let requests = proposed_manifest
        .resources
        .iter()
        .map(|request| PlannedRequest {
            identity: request.identity.clone(),
            requirement: request.requirement.clone(),
            source: request.source.clone(),
        })
        .collect();
    let dependencies = proposed
        .resources
        .iter()
        .flat_map(|resource| {
            resource.dependencies.iter().map(|edge| PlannedDependency {
                from: resource.resource.identity.clone(),
                to: edge.identity.clone(),
                version: edge.version.clone(),
                collection_member: edge.collection_member,
            })
        })
        .collect();
    let effects = planned_effects(proposed);
    let decisions = planned_decisions(current, proposed);
    let verified_bytes = proposed
        .resources
        .iter()
        .flat_map(|resource| &resource.resource.payloads)
        .map(|payload| payload.length)
        .sum();
    Ok(MutationPlan {
        operation: operation.to_owned(),
        snapshot: proposed
            .catalog
            .as_ref()
            .map(|catalog| catalog.snapshot.clone()),
        changes,
        requests,
        dependencies,
        files: planned_files(current_manifest, proposed_manifest, current, proposed)?,
        effects,
        decisions,
        conflicts: Vec::new(),
        downloads,
        download_bytes,
        verified_bytes,
        resulting_resources: exact_resources(proposed),
        rollback: "the previous complete resource store remains recoverable until commit"
            .to_owned(),
        committed: false,
    })
}

#[derive(Debug, Clone)]
struct FileDescriptor {
    comparison: String,
    sha256: Option<String>,
    bytes: Option<u64>,
}

fn planned_files(
    current_manifest: &ResourceManifest,
    proposed_manifest: &ResourceManifest,
    current: &ResourceLock,
    proposed: &ResourceLock,
) -> Result<Vec<PlannedFile>, ManagerError> {
    let mut before = planned_file_map(current_manifest, current)?;
    let mut after = planned_file_map(proposed_manifest, proposed)?;
    let paths = before
        .keys()
        .chain(after.keys())
        .cloned()
        .collect::<BTreeSet<_>>();
    Ok(paths
        .into_iter()
        .map(|path| {
            let from = before.remove(&path);
            let to = after.remove(&path);
            let action = match (&from, &to) {
                (None, Some(_)) => FileAction::Add,
                (Some(_), None) => FileAction::Remove,
                (Some(from), Some(to)) if from.comparison != to.comparison => FileAction::Replace,
                (Some(_), Some(_)) => FileAction::Retain,
                (None, None) => unreachable!(),
            };
            PlannedFile {
                action,
                path,
                from_sha256: from.as_ref().and_then(|file| file.sha256.clone()),
                to_sha256: to.as_ref().and_then(|file| file.sha256.clone()),
                bytes: to
                    .as_ref()
                    .and_then(|file| file.bytes)
                    .or_else(|| from.as_ref().and_then(|file| file.bytes)),
            }
        })
        .collect())
}

fn planned_file_map(
    manifest: &ResourceManifest,
    lock: &ResourceLock,
) -> Result<BTreeMap<String, FileDescriptor>, ManagerError> {
    let manifest_sha = liknon_resource_contracts::canonical_json_sha256(manifest)?;
    let lock_sha = liknon_resource_contracts::canonical_json_sha256(lock)?;
    let mut files = BTreeMap::from([
        (
            ".liknon/resources/manifest.json".to_owned(),
            FileDescriptor {
                comparison: manifest_sha.clone(),
                sha256: Some(manifest_sha),
                bytes: None,
            },
        ),
        (
            ".liknon/resources/lock.json".to_owned(),
            FileDescriptor {
                comparison: lock_sha.clone(),
                sha256: Some(lock_sha),
                bytes: None,
            },
        ),
    ]);
    for resource in &lock.resources {
        for payload in &resource.resource.payloads {
            let path = format!(
                ".liknon/resources/{}/{}",
                resource.install_path, payload.path
            );
            files.insert(
                path,
                FileDescriptor {
                    comparison: payload.sha256.clone(),
                    sha256: Some(payload.sha256.clone()),
                    bytes: Some(payload.length),
                },
            );
        }
    }
    for materialization in &lock.materializations {
        files.insert(
            materialization.destination.clone(),
            FileDescriptor {
                comparison: materialization.sha256.clone(),
                sha256: Some(materialization.sha256.clone()),
                bytes: None,
            },
        );
    }
    if let Some(catalog) = &lock.catalog {
        let evidence_identity = liknon_resource_contracts::canonical_json_sha256(catalog)?;
        for (path, sha256) in [
            (&catalog.catalog_path, Some(catalog.sha256.clone())),
            (&catalog.catalog_signature_path, None),
            (&catalog.trust_root_path, None),
        ] {
            files.insert(
                format!(".liknon/resources/{path}"),
                FileDescriptor {
                    comparison: evidence_identity.clone(),
                    sha256,
                    bytes: None,
                },
            );
        }
    }
    Ok(files)
}

fn planned_effects(lock: &ResourceLock) -> Vec<PlannedEffect> {
    let mut effects = BTreeSet::new();
    for locked in &lock.resources {
        let identity = &locked.resource.identity;
        for capability in &locked.resource.capabilities {
            effects.insert((identity.clone(), format!("capability:{capability}")));
        }
        if locked.resource.contains_executable_content {
            effects.insert((identity.clone(), "executable-content".to_owned()));
        }
        if let Some(workflow) = &locked.resource.workflow {
            for node in &workflow.nodes {
                for effect in &node.effects {
                    effects.insert((identity.clone(), format!("{}:{effect}", node.id)));
                }
                for capability in &node.required_capabilities {
                    effects.insert((
                        identity.clone(),
                        format!("{}:requires:{capability}", node.id),
                    ));
                }
            }
        }
    }
    effects
        .into_iter()
        .map(|(identity, effect)| PlannedEffect { identity, effect })
        .collect()
}

fn planned_decisions(current: &ResourceLock, proposed: &ResourceLock) -> Vec<String> {
    let mut decisions = BTreeSet::from(["approve-complete-plan".to_owned()]);
    for locked in &proposed.resources {
        let newly_selected = current.resources.iter().all(|existing| {
            existing.resource.identity != locked.resource.identity
                || existing.resource.version != locked.resource.version
        });
        if !newly_selected {
            continue;
        }
        if locked.resource.trust.class == liknon_resource_contracts::TrustClass::Community {
            decisions.insert(format!(
                "accept-community-resource:{}",
                locked.resource.identity
            ));
        }
        if locked.resource.contains_executable_content {
            decisions.insert(format!(
                "review-executable-content:{}",
                locked.resource.identity
            ));
        }
    }
    for materialization in proposed
        .materializations
        .iter()
        .filter(|record| !current.materializations.contains(record))
    {
        decisions.insert(format!(
            "approve-materialization:{}",
            materialization.destination
        ));
    }
    decisions.into_iter().collect()
}

fn confirm(plan: &MutationPlan, approved: bool, format: OutputFormat) -> Result<(), ManagerError> {
    if approved {
        return Ok(());
    }
    if format == OutputFormat::Json || !io::stdin().is_terminal() || !io::stderr().is_terminal() {
        return Err(ManagerError::Blocked(
            "mutation requires --yes outside an interactive human terminal".to_owned(),
        ));
    }
    eprintln!(
        "Plan: {} changes, {} downloads, {} bytes; resulting resources: {}.",
        plan.changes
            .iter()
            .filter(|change| change.action != ChangeAction::Retain)
            .count(),
        plan.downloads,
        plan.download_bytes,
        plan.resulting_resources.len()
    );
    eprint!("Apply this complete plan? [y/N] ");
    io::stderr()
        .flush()
        .map_err(|error| ManagerError::Internal(error.to_string()))?;
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .map_err(|error| ManagerError::Internal(error.to_string()))?;
    if matches!(answer.trim().to_ascii_lowercase().as_str(), "y" | "yes") {
        Ok(())
    } else {
        Err(ManagerError::Blocked(
            "the user declined the complete mutation plan".to_owned(),
        ))
    }
}

fn verify_current_catalog(
    lock: &ResourceLock,
    hub: &HubSnapshot,
    warnings: &mut Vec<String>,
) -> Result<(), ManagerError> {
    let Some(locked_catalog) = &lock.catalog else {
        if lock
            .resources
            .iter()
            .any(|resource| matches!(resource.origin, LockedResourceOrigin::Hub { .. }))
        {
            return Err(ManagerError::Integrity(
                "Hub-backed resources have no locked catalog".to_owned(),
            ));
        }
        return Ok(());
    };
    if locked_catalog.origin != hub.origin || locked_catalog.trust_authority != hub.trust_authority
    {
        return Err(ManagerError::Integrity(
            "current Hub trust identity differs from the locked origin".to_owned(),
        ));
    }
    if locked_catalog.snapshot != hub.catalog.snapshot {
        warnings.push(format!(
            "installed snapshot {} differs from current snapshot {}",
            locked_catalog.snapshot.id, hub.catalog.snapshot.id
        ));
    }
    for locked in lock
        .resources
        .iter()
        .filter(|resource| matches!(resource.origin, LockedResourceOrigin::Hub { .. }))
    {
        if let Some(current) = hub.catalog.resources.iter().find(|resource| {
            resource.identity == locked.resource.identity
                && resource.version == locked.resource.version
        }) {
            if current.revoked {
                return Err(ManagerError::Integrity(format!(
                    "installed resource {}@{} is revoked by the current catalog",
                    current.identity, current.version
                )));
            }
        } else {
            warnings.push(format!(
                "installed resource {}@{} is absent from the current catalog",
                locked.resource.identity, locked.resource.version
            ));
        }
    }
    Ok(())
}

fn checked_state(workspace: &Workspace) -> Result<StoreState, ManagerError> {
    store::recover(workspace)?;
    let state = store::load(workspace)?;
    store::verify(workspace, &state)?;
    Ok(state)
}

fn existing_workspace(args: &WorkspaceArgs) -> Result<Workspace, ManagerError> {
    workspace::discover(args.workspace.as_deref())
}

fn load_optional_hub(args: &HubArgs, required: bool) -> Result<Option<HubSnapshot>, ManagerError> {
    if required || args.hub.is_some() || args.trust_root.is_some() {
        hub::load(args).map(Some)
    } else {
        Ok(None)
    }
}

fn upsert_request(manifest: &mut ResourceManifest, request: ResourceRequest) {
    manifest
        .resources
        .retain(|existing| existing.identity != request.identity);
    manifest.resources.push(request);
    manifest
        .resources
        .sort_by(|left, right| left.identity.cmp(&right.identity));
}

fn parse_identity(value: &str) -> Result<ResourceIdentity, ManagerError> {
    if value.contains('@') {
        return Err(ManagerError::Usage(format!(
            "`{value}` must be an identity without a version requirement"
        )));
    }
    value.parse().map_err(ManagerError::from)
}

fn newest_matching<'a>(
    resources: impl Iterator<Item = &'a ResourceVersion>,
    spec: &ResourceSpec,
) -> Result<&'a ResourceVersion, ManagerError> {
    let requirement = VersionReq::parse(&spec.requirement)
        .map_err(|error| ManagerError::Usage(error.to_string()))?;
    resources
        .filter(|resource| resource.identity == spec.identity)
        .filter_map(|resource| {
            Version::parse(&resource.version)
                .ok()
                .filter(|version| requirement.matches(version))
                .map(|version| (version, resource))
        })
        .max_by(|left, right| left.0.cmp(&right.0))
        .map(|(_, resource)| resource)
        .ok_or_else(|| ManagerError::Usage(format!("resource `{}` was not found", spec.identity)))
}

fn resource_order(left: &ResourceVersion, right: &ResourceVersion) -> std::cmp::Ordering {
    left.identity.cmp(&right.identity).then_with(|| {
        let left = Version::parse(&left.version).ok();
        let right = Version::parse(&right.version).ok();
        right.cmp(&left)
    })
}

fn exact_resources(lock: &ResourceLock) -> Vec<ExactResource> {
    lock.resources
        .iter()
        .map(|resource| ExactResource {
            identity: resource.resource.identity.clone(),
            version: resource.resource.version.clone(),
        })
        .collect()
}

fn planned_report(command: &str, workspace: &Workspace, plan: MutationPlan) -> ManagerReport {
    ManagerReport::planned(
        command,
        Some(workspace.display()),
        ReportPayload::Mutation {
            plan: Box::new(plan),
        },
    )
}

fn completed_mutation(command: &str, workspace: &Workspace, plan: MutationPlan) -> ManagerReport {
    ManagerReport::completed(
        command,
        Some(workspace.display()),
        ReportPayload::Mutation {
            plan: Box::new(plan),
        },
    )
}

fn schema_value<T: serde::Serialize>(schema: T) -> Result<serde_json::Value, ManagerError> {
    serde_json::to_value(schema).map_err(|error| ManagerError::Internal(error.to_string()))
}
