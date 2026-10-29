use std::collections::{BTreeMap, BTreeSet};

use liknon_resource_contracts::{
    canonical_json_sha256, LockedCompatibility, LockedEdge, LockedResource, LockedResourceOrigin,
    RequestedSource, ResourceIdentity, ResourceKind, ResourceLock, ResourceManifest,
    ResourceVersion, TrustClass,
};
use semver::{Version, VersionReq};

use crate::error::ManagerError;

#[derive(Debug, Clone)]
pub(crate) struct AvailableResource {
    pub(crate) resource: ResourceVersion,
    pub(crate) origin: LockedResourceOrigin,
}

#[derive(Debug)]
pub(crate) struct Resolution {
    pub(crate) resources: Vec<LockedResource>,
}

#[derive(Debug, Clone, Default)]
struct SolveState {
    selected: BTreeMap<ResourceIdentity, AvailableResource>,
    constraints: BTreeMap<ResourceIdentity, Vec<String>>,
    edges: BTreeMap<ResourceIdentity, Vec<(ResourceIdentity, bool)>>,
}

pub(crate) fn resolve(
    manifest: &ResourceManifest,
    available: Vec<AvailableResource>,
    existing: Option<&ResourceLock>,
    update: &BTreeSet<ResourceIdentity>,
    allow_community: bool,
) -> Result<Resolution, ManagerError> {
    if manifest.resources.is_empty() {
        return Ok(Resolution {
            resources: Vec::new(),
        });
    }
    let mut pool: BTreeMap<ResourceIdentity, Vec<AvailableResource>> = BTreeMap::new();
    for candidate in available {
        insert_candidate(&mut pool, candidate);
    }
    if let Some(existing) = existing {
        for locked in &existing.resources {
            insert_candidate(
                &mut pool,
                AvailableResource {
                    resource: locked.resource.clone(),
                    origin: locked.origin.clone(),
                },
            );
        }
    }
    for candidates in pool.values_mut() {
        candidates.sort_by(|left, right| {
            let left = Version::parse(&left.resource.version).expect("validated resource version");
            let right =
                Version::parse(&right.resource.version).expect("validated resource version");
            right.cmp(&left)
        });
    }

    let mut state = SolveState::default();
    for request in &manifest.resources {
        state
            .constraints
            .entry(request.identity.clone())
            .or_default()
            .push(request.requirement.clone());
    }
    let preferences = existing
        .map(|lock| {
            lock.resources
                .iter()
                .map(|locked| {
                    (
                        locked.resource.identity.clone(),
                        locked.resource.version.clone(),
                    )
                })
                .collect::<BTreeMap<_, _>>()
        })
        .unwrap_or_default();
    let requested_sources = manifest
        .resources
        .iter()
        .map(|request| (request.identity.clone(), request.source.clone()))
        .collect::<BTreeMap<_, _>>();
    let state = solve(
        state,
        &pool,
        &preferences,
        &requested_sources,
        update,
        allow_community,
    )
    .ok_or_else(|| {
        ManagerError::Resolution(format_constraints(manifest, &pool, allow_community))
    })?;
    reject_cycles(&state.edges)?;

    let selected_versions = state
        .selected
        .iter()
        .map(|(identity, candidate)| (identity.clone(), candidate.resource.version.clone()))
        .collect::<BTreeMap<_, _>>();
    let mut resources = Vec::with_capacity(state.selected.len());
    for (identity, candidate) in state.selected {
        let mut dependencies = Vec::new();
        for (dependency, collection_member) in state.edges.get(&identity).into_iter().flatten() {
            dependencies.push(LockedEdge {
                identity: dependency.clone(),
                version: selected_versions
                    .get(dependency)
                    .expect("resolved edge target exists")
                    .clone(),
                collection_member: *collection_member,
            });
        }
        dependencies.sort_by(|left, right| left.identity.cmp(&right.identity));
        let compatibility = locked_compatibility(&candidate.resource);
        resources.push(LockedResource {
            manifest_sha256: canonical_json_sha256(&candidate.resource)?,
            install_path: candidate
                .resource
                .identity
                .version_path(&candidate.resource.version)?,
            origin: candidate.origin,
            resource: candidate.resource,
            dependencies,
            compatibility,
        });
    }
    resources.sort_by(|left, right| left.resource.identity.cmp(&right.resource.identity));
    Ok(Resolution { resources })
}

fn solve(
    state: SolveState,
    pool: &BTreeMap<ResourceIdentity, Vec<AvailableResource>>,
    preferences: &BTreeMap<ResourceIdentity, String>,
    requested_sources: &BTreeMap<ResourceIdentity, RequestedSource>,
    update: &BTreeSet<ResourceIdentity>,
    allow_community: bool,
) -> Option<SolveState> {
    for (identity, selected) in &state.selected {
        if !requirements_match(
            &selected.resource.version,
            state
                .constraints
                .get(identity)
                .map(Vec::as_slice)
                .unwrap_or(&[]),
        ) {
            return None;
        }
    }
    let identity = state
        .constraints
        .keys()
        .find(|identity| !state.selected.contains_key(*identity))?
        .clone();
    let mut candidates = pool.get(&identity)?.clone();
    if !update.contains(&identity) {
        if let Some(preferred) = preferences.get(&identity) {
            candidates.sort_by_key(|candidate| candidate.resource.version != *preferred);
        }
    }
    let requirements = state
        .constraints
        .get(&identity)
        .cloned()
        .unwrap_or_default();
    for candidate in candidates {
        if !candidate_allowed(
            &candidate,
            &requirements,
            requested_sources.get(&identity),
            preferences.get(&identity),
            update.contains(&identity),
            allow_community,
        ) {
            continue;
        }
        let mut next = state.clone();
        next.selected.insert(identity.clone(), candidate.clone());
        let mut edges = Vec::new();
        for dependency in &candidate.resource.dependencies {
            if dependency.optional {
                continue;
            }
            next.constraints
                .entry(dependency.identity.clone())
                .or_default()
                .push(dependency.requirement.clone());
            edges.push((dependency.identity.clone(), false));
        }
        for member in &candidate.resource.collection_members {
            if member.optional {
                continue;
            }
            next.constraints
                .entry(member.identity.clone())
                .or_default()
                .push(member.requirement.clone());
            edges.push((member.identity.clone(), true));
        }
        edges.sort_by(|left, right| left.0.cmp(&right.0));
        next.edges.insert(identity.clone(), edges);

        if next.selected.len() == next.constraints.len() {
            if next.selected.iter().all(|(identity, selected)| {
                requirements_match(
                    &selected.resource.version,
                    next.constraints
                        .get(identity)
                        .map(Vec::as_slice)
                        .unwrap_or(&[]),
                )
            }) {
                return Some(next);
            }
        } else if let Some(solution) = solve(
            next,
            pool,
            preferences,
            requested_sources,
            update,
            allow_community,
        ) {
            return Some(solution);
        }
    }
    None
}

fn insert_candidate(
    pool: &mut BTreeMap<ResourceIdentity, Vec<AvailableResource>>,
    candidate: AvailableResource,
) {
    let candidates = pool.entry(candidate.resource.identity.clone()).or_default();
    if !candidates.iter().any(|existing| {
        existing.resource.version == candidate.resource.version
            && existing.origin == candidate.origin
    }) {
        candidates.push(candidate);
    }
}

fn candidate_allowed(
    candidate: &AvailableResource,
    requirements: &[String],
    requested_source: Option<&RequestedSource>,
    preferred: Option<&String>,
    updating: bool,
    allow_community: bool,
) -> bool {
    let resource = &candidate.resource;
    if resource.revoked
        || !requirements_match(&resource.version, requirements)
        || !manager_compatible(resource)
    {
        return false;
    }
    let source_allowed = match (requested_source, &candidate.origin) {
        (Some(RequestedSource::Hub), LockedResourceOrigin::Hub { .. }) => true,
        (
            Some(RequestedSource::Local { path: requested }),
            LockedResourceOrigin::Local { path },
        ) => path == requested,
        (Some(_), _) => false,
        (None, LockedResourceOrigin::Hub { .. }) => resource.trust.class != TrustClass::Local,
        (None, LockedResourceOrigin::Local { .. }) => resource.trust.class == TrustClass::Local,
    };
    if !source_allowed {
        return false;
    }
    if resource.yanked && (updating || preferred.is_none_or(|value| value != &resource.version)) {
        return false;
    }
    match resource.trust.class {
        TrustClass::Official | TrustClass::Verified | TrustClass::Local => true,
        TrustClass::Community => allow_community,
    }
}

fn requirements_match(version: &str, requirements: &[String]) -> bool {
    let Ok(version) = Version::parse(version) else {
        return false;
    };
    requirements.iter().all(|requirement| {
        VersionReq::parse(requirement).is_ok_and(|requirement| requirement.matches(&version))
    })
}

fn manager_compatible(resource: &ResourceVersion) -> bool {
    let current = Version::parse(env!("CARGO_PKG_VERSION")).expect("package version is semver");
    VersionReq::parse(&resource.compatibility.resource_schema)
        .is_ok_and(|requirement| requirement.matches(&Version::new(1, 0, 0)))
        && VersionReq::parse(&resource.compatibility.manager)
            .is_ok_and(|requirement| requirement.matches(&current))
}

fn locked_compatibility(resource: &ResourceVersion) -> LockedCompatibility {
    if resource.identity.kind != ResourceKind::Workflow {
        return LockedCompatibility {
            manager: true,
            ai_bootstrap_declared: false,
            decision_policy_declared: false,
            activatable: false,
            activation_blockers: Vec::new(),
        };
    }
    let ai_bootstrap_declared = resource.compatibility.ai_bootstrap.is_some();
    let decision_policy_declared = resource.compatibility.decision_policy.is_some();
    let mut blockers = Vec::new();
    if !ai_bootstrap_declared {
        blockers.push("ai-bootstrap-compatibility-absent".to_owned());
    }
    if !decision_policy_declared {
        blockers.push("decision-policy-compatibility-absent".to_owned());
    }
    if blockers.is_empty() {
        blockers.push("host-capabilities-not-evaluated".to_owned());
    }
    LockedCompatibility {
        manager: true,
        ai_bootstrap_declared,
        decision_policy_declared,
        activatable: false,
        activation_blockers: blockers,
    }
}

fn reject_cycles(
    edges: &BTreeMap<ResourceIdentity, Vec<(ResourceIdentity, bool)>>,
) -> Result<(), ManagerError> {
    let mut visited = BTreeSet::new();
    let mut active = BTreeSet::new();
    for identity in edges.keys() {
        visit(identity, edges, &mut visited, &mut active)?;
    }
    Ok(())
}

fn visit(
    identity: &ResourceIdentity,
    edges: &BTreeMap<ResourceIdentity, Vec<(ResourceIdentity, bool)>>,
    visited: &mut BTreeSet<ResourceIdentity>,
    active: &mut BTreeSet<ResourceIdentity>,
) -> Result<(), ManagerError> {
    if active.contains(identity) {
        return Err(ManagerError::Resolution(format!(
            "resource graph contains a cycle at {identity}"
        )));
    }
    if !visited.insert(identity.clone()) {
        return Ok(());
    }
    active.insert(identity.clone());
    for (dependency, _) in edges.get(identity).into_iter().flatten() {
        visit(dependency, edges, visited, active)?;
    }
    active.remove(identity);
    Ok(())
}

fn format_constraints(
    manifest: &ResourceManifest,
    pool: &BTreeMap<ResourceIdentity, Vec<AvailableResource>>,
    allow_community: bool,
) -> String {
    manifest
        .resources
        .iter()
        .map(|request| {
            let available = pool
                .get(&request.identity)
                .map(|values| {
                    values
                        .iter()
                        .map(|value| value.resource.version.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                })
                .unwrap_or_else(|| "none".to_owned());
            format!(
                "{} requires `{}` (available: {available}; community allowed: {allow_community})",
                request.identity, request.requirement
            )
        })
        .collect::<Vec<_>>()
        .join("; ")
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use liknon_resource_contracts::{
        Compatibility, LockedResourceOrigin, ResourceDependency, ResourceIdentity, ResourceKind,
        ResourceLock, ResourceManifest, ResourceRequest, ResourceVersion, ReviewState,
        SourceMetadata, TrustClass, TrustMetadata, RESOURCE_LOCK_SCHEMA_VERSION,
        RESOURCE_MANIFEST_SCHEMA_VERSION, RESOURCE_SCHEMA_VERSION,
    };

    use super::{resolve, AvailableResource};

    fn identity(name: &str) -> ResourceIdentity {
        ResourceIdentity::new(ResourceKind::Knowledge, "official", name).unwrap()
    }

    fn candidate(version: &str) -> AvailableResource {
        candidate_for("one", version)
    }

    fn candidate_for(name: &str, version: &str) -> AvailableResource {
        AvailableResource {
            resource: ResourceVersion {
                schema_version: RESOURCE_SCHEMA_VERSION,
                identity: identity(name),
                version: version.to_owned(),
                title: "One".to_owned(),
                summary: "Fixture".to_owned(),
                authors: vec!["Liknon".to_owned()],
                license: "MIT".to_owned(),
                source: SourceMetadata {
                    repository: "https://example.invalid".to_owned(),
                    revision: None,
                    path: None,
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
            },
            origin: LockedResourceOrigin::Hub {
                origin: "https://example.invalid/".to_owned(),
            },
        }
    }

    fn request(name: &str, requirement: &str) -> ResourceRequest {
        ResourceRequest {
            identity: identity(name),
            requirement: requirement.to_owned(),
            source: Default::default(),
        }
    }

    fn dependency(name: &str, requirement: &str) -> ResourceDependency {
        ResourceDependency {
            identity: identity(name),
            requirement: requirement.to_owned(),
            optional: false,
        }
    }

    #[test]
    fn chooses_highest_compatible_version_deterministically() {
        let identity = candidate("1.0.0").resource.identity;
        let manifest = ResourceManifest {
            schema_version: RESOURCE_MANIFEST_SCHEMA_VERSION,
            resources: vec![ResourceRequest {
                identity,
                requirement: "^1".to_owned(),
                source: Default::default(),
            }],
        };
        let resolution = resolve(
            &manifest,
            vec![candidate("1.0.0"), candidate("1.2.0"), candidate("2.0.0")],
            None,
            &BTreeSet::new(),
            false,
        )
        .unwrap();
        assert_eq!(resolution.resources[0].resource.version, "1.2.0");
    }

    #[test]
    fn rejects_resources_incompatible_with_the_manager() {
        let mut incompatible = candidate("1.0.0");
        incompatible.resource.compatibility.manager = ">=99".to_owned();
        let manifest = ResourceManifest {
            schema_version: RESOURCE_MANIFEST_SCHEMA_VERSION,
            resources: vec![request("one", "^1")],
        };

        assert!(resolve(&manifest, vec![incompatible], None, &BTreeSet::new(), false,).is_err());
    }

    #[test]
    fn resolves_transitive_graphs_independently_from_input_order() {
        let mut root = candidate_for("root", "1.0.0");
        root.resource.dependencies = vec![dependency("middle", "^1")];
        let mut middle = candidate_for("middle", "1.1.0");
        middle.resource.dependencies = vec![dependency("leaf", "=2.0.0")];
        let leaf = candidate_for("leaf", "2.0.0");
        let manifest = ResourceManifest {
            schema_version: RESOURCE_MANIFEST_SCHEMA_VERSION,
            resources: vec![request("root", "^1")],
        };

        let first = resolve(
            &manifest,
            vec![root.clone(), middle.clone(), leaf.clone()],
            None,
            &BTreeSet::new(),
            false,
        )
        .unwrap();
        let second = resolve(
            &manifest,
            vec![leaf, middle, root],
            None,
            &BTreeSet::new(),
            false,
        )
        .unwrap();

        assert_eq!(first.resources, second.resources);
        assert_eq!(
            first
                .resources
                .iter()
                .map(|resource| resource.resource.identity.name.as_str())
                .collect::<Vec<_>>(),
            vec!["leaf", "middle", "root"]
        );
    }

    #[test]
    fn resolves_collection_members_as_explicit_graph_edges() {
        let mut collection = candidate_for("starter", "1.0.0");
        collection.resource.identity =
            ResourceIdentity::new(ResourceKind::Collection, "official", "starter").unwrap();
        collection.resource.collection_members = vec![dependency("leaf", "^1")];
        let leaf = candidate_for("leaf", "1.2.0");
        let manifest = ResourceManifest {
            schema_version: RESOURCE_MANIFEST_SCHEMA_VERSION,
            resources: vec![ResourceRequest {
                identity: collection.resource.identity.clone(),
                requirement: "^1".to_owned(),
                source: Default::default(),
            }],
        };

        let resolution = resolve(
            &manifest,
            vec![collection, leaf],
            None,
            &BTreeSet::new(),
            false,
        )
        .unwrap();

        let collection = resolution
            .resources
            .iter()
            .find(|resource| resource.resource.identity.kind == ResourceKind::Collection)
            .unwrap();
        assert_eq!(collection.dependencies.len(), 1);
        assert!(collection.dependencies[0].collection_member);
        assert_eq!(collection.dependencies[0].version, "1.2.0");
    }

    #[test]
    fn rejects_conflicts_cycles_and_revoked_versions() {
        let mut root = candidate_for("root", "1.0.0");
        root.resource.dependencies = vec![dependency("shared", "^1")];
        let mut other = candidate_for("other", "1.0.0");
        other.resource.dependencies = vec![dependency("shared", "^2")];
        let manifest = ResourceManifest {
            schema_version: RESOURCE_MANIFEST_SCHEMA_VERSION,
            resources: vec![request("root", "^1"), request("other", "^1")],
        };
        assert!(resolve(
            &manifest,
            vec![
                root,
                other,
                candidate_for("shared", "1.0.0"),
                candidate_for("shared", "2.0.0"),
            ],
            None,
            &BTreeSet::new(),
            false,
        )
        .is_err());

        let mut cycle_a = candidate_for("cycle-a", "1.0.0");
        cycle_a.resource.dependencies = vec![dependency("cycle-b", "^1")];
        let mut cycle_b = candidate_for("cycle-b", "1.0.0");
        cycle_b.resource.dependencies = vec![dependency("cycle-a", "^1")];
        let cycle_manifest = ResourceManifest {
            schema_version: RESOURCE_MANIFEST_SCHEMA_VERSION,
            resources: vec![request("cycle-a", "^1")],
        };
        assert!(resolve(
            &cycle_manifest,
            vec![cycle_a, cycle_b],
            None,
            &BTreeSet::new(),
            false,
        )
        .unwrap_err()
        .to_string()
        .contains("cycle"));

        let mut revoked = candidate("1.0.0");
        revoked.resource.revoked = true;
        let revoked_manifest = ResourceManifest {
            schema_version: RESOURCE_MANIFEST_SCHEMA_VERSION,
            resources: vec![request("one", "^1")],
        };
        assert!(resolve(
            &revoked_manifest,
            vec![revoked],
            None,
            &BTreeSet::new(),
            false,
        )
        .is_err());
    }

    #[test]
    fn retains_yanked_locks_and_updates_only_the_requested_scope() {
        let manifest = ResourceManifest {
            schema_version: RESOURCE_MANIFEST_SCHEMA_VERSION,
            resources: vec![request("one", "^1"), request("two", "^1")],
        };
        let initial = resolve(
            &manifest,
            vec![candidate_for("one", "1.0.0"), candidate_for("two", "1.0.0")],
            None,
            &BTreeSet::new(),
            false,
        )
        .unwrap();
        let mut locked_one = initial.resources[0].clone();
        locked_one.resource.yanked = true;
        let existing = ResourceLock {
            schema_version: RESOURCE_LOCK_SCHEMA_VERSION,
            catalog: None,
            resources: vec![locked_one, initial.resources[1].clone()],
            materializations: Vec::new(),
        };
        let update = BTreeSet::from([identity("two")]);
        let resolved = resolve(
            &manifest,
            vec![candidate_for("one", "1.1.0"), candidate_for("two", "1.1.0")],
            Some(&existing),
            &update,
            false,
        )
        .unwrap();

        assert_eq!(resolved.resources[0].resource.version, "1.0.0");
        assert_eq!(resolved.resources[1].resource.version, "1.1.0");
    }

    #[test]
    fn requires_explicit_local_source_and_community_consent() {
        let mut local = candidate("1.0.0");
        local.resource.trust.class = TrustClass::Local;
        local.resource.trust.review = ReviewState::Unreviewed;
        local.origin = LockedResourceOrigin::Local {
            path: "/fixture/local".to_owned(),
        };
        let local_manifest = ResourceManifest {
            schema_version: RESOURCE_MANIFEST_SCHEMA_VERSION,
            resources: vec![ResourceRequest {
                identity: identity("one"),
                requirement: "^1".to_owned(),
                source: liknon_resource_contracts::RequestedSource::Local {
                    path: "/fixture/local".to_owned(),
                },
            }],
        };
        assert!(resolve(
            &local_manifest,
            vec![candidate("1.0.0"), local],
            None,
            &BTreeSet::new(),
            false,
        )
        .is_ok());

        let mut community = candidate("1.0.0");
        community.resource.trust.class = TrustClass::Community;
        community.resource.trust.review = ReviewState::Unreviewed;
        let community_manifest = ResourceManifest {
            schema_version: RESOURCE_MANIFEST_SCHEMA_VERSION,
            resources: vec![request("one", "^1")],
        };
        assert!(resolve(
            &community_manifest,
            vec![community.clone()],
            None,
            &BTreeSet::new(),
            false,
        )
        .is_err());
        assert!(resolve(
            &community_manifest,
            vec![community],
            None,
            &BTreeSet::new(),
            true,
        )
        .is_ok());
    }
}
