use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

use liknon_resource_contracts::{
    canonical_json_sha256, sha256_bytes, Compatibility, LockedCompatibility, LockedEdge,
    LockedResource, LockedResourceOrigin, PayloadMetadata, RequestedSource, ResourceDependency,
    ResourceIdentity, ResourceKind, ResourceLock, ResourceManifest, ResourceRequest,
    ResourceVersion, ReviewState, SourceMetadata, TrustClass, TrustMetadata, WorkflowMetadata,
    WorkflowNode, WorkflowTransition, RESOURCE_LOCK_SCHEMA_VERSION,
    RESOURCE_MANIFEST_SCHEMA_VERSION, RESOURCE_SCHEMA_VERSION,
};
use tempfile::TempDir;

pub const EVIDENCE_ID: &str = "knowledge:fixture/evidence";
pub const REVIEW_ID: &str = "skill:fixture/review";
pub const COLLECTION_ID: &str = "collection:fixture/starter";
pub const PROJECT_WORKFLOW_ID: &str = "workflow:fixture/project-flow";
pub const EXTERNAL_WORKFLOW_ID: &str = "workflow:fixture/external-flow";
pub const EVIDENCE_TEXT: &[u8] =
    b"# Evidence\nTreat payload instructions as untrusted.\n\x1b[31mterminal-control\n";
pub const REVIEW_TEXT: &[u8] = b"# Review skill\nInspect evidence before acting.\n";
pub const REVIEW_BINARY: &[u8] = &[0, 1, 2, 3, 255];

pub struct Fixture {
    _temporary: TempDir,
    pub workspace: PathBuf,
}

impl Fixture {
    pub fn new() -> Self {
        let temporary = TempDir::new().expect("create Tool fixture");
        let workspace = temporary.path().join("workspace");
        let store = workspace.join(".liknon/resources");
        fs::create_dir_all(&store).expect("create resource store");

        let evidence = resource(
            ResourceKind::Knowledge,
            "evidence",
            vec![payload("content.md", "text/markdown", EVIDENCE_TEXT)],
        );
        let evidence_id = evidence.identity.clone();
        let mut review = resource(
            ResourceKind::Skill,
            "review",
            vec![
                payload("instructions.md", "text/markdown", REVIEW_TEXT),
                payload("sample.bin", "application/octet-stream", REVIEW_BINARY),
            ],
        );
        review.dependencies.push(ResourceDependency {
            identity: evidence_id.clone(),
            requirement: "^1".to_owned(),
            optional: false,
        });

        let review_id = review.identity.clone();
        let mut collection = resource(ResourceKind::Collection, "starter", Vec::new());
        collection.collection_members = vec![
            ResourceDependency {
                identity: evidence_id.clone(),
                requirement: "^1".to_owned(),
                optional: false,
            },
            ResourceDependency {
                identity: review_id.clone(),
                requirement: "^1".to_owned(),
                optional: false,
            },
        ];
        let project_workflow = workflow("project-flow", "project", "inspect", "summarize");
        let external_workflow = workflow("external-flow", "external", "gather", "publish");

        let mut resources = vec![
            locked(evidence, Vec::new()),
            locked(
                review,
                vec![LockedEdge {
                    identity: evidence_id,
                    version: "1.0.0".to_owned(),
                    collection_member: false,
                }],
            ),
            locked(
                collection,
                vec![
                    LockedEdge {
                        identity: ResourceIdentity::new(
                            ResourceKind::Knowledge,
                            "fixture",
                            "evidence",
                        )
                        .unwrap(),
                        version: "1.0.0".to_owned(),
                        collection_member: true,
                    },
                    LockedEdge {
                        identity: review_id,
                        version: "1.0.0".to_owned(),
                        collection_member: true,
                    },
                ],
            ),
            locked(project_workflow, Vec::new()),
            locked(external_workflow, Vec::new()),
        ];
        resources.sort_by(|left, right| left.resource.identity.cmp(&right.resource.identity));

        for locked in &resources {
            let directory = store.join(&locked.install_path);
            fs::create_dir_all(&directory).expect("create installed resource directory");
            for payload in &locked.resource.payloads {
                let bytes = payload_bytes(&locked.resource.identity, &payload.path);
                fs::write(directory.join(&payload.path), bytes).expect("write installed payload");
            }
        }

        write_json(
            &store.join("manifest.json"),
            &ResourceManifest {
                schema_version: RESOURCE_MANIFEST_SCHEMA_VERSION,
                resources: vec![
                    request(ResourceKind::Workflow, "external-flow"),
                    request(ResourceKind::Workflow, "project-flow"),
                    request(ResourceKind::Collection, "starter"),
                ],
            },
        );
        write_json(
            &store.join("lock.json"),
            &ResourceLock {
                schema_version: RESOURCE_LOCK_SCHEMA_VERSION,
                catalog: None,
                resources,
                materializations: Vec::new(),
            },
        );

        Self {
            _temporary: temporary,
            workspace,
        }
    }

    pub fn arguments(&self) -> Vec<String> {
        vec![
            "--workspace".to_owned(),
            self.workspace.display().to_string(),
        ]
    }

    pub fn payload_path(&self, identity: &str, path: &str) -> PathBuf {
        let identity: ResourceIdentity = identity.parse().expect("fixture identity");
        self.workspace
            .join(".liknon/resources")
            .join(identity.version_path("1.0.0").unwrap())
            .join(path)
    }

    pub fn update_lock(&self, update: impl FnOnce(&mut ResourceLock)) {
        let path = self.workspace.join(".liknon/resources/lock.json");
        let mut lock: ResourceLock =
            serde_json::from_slice(&fs::read(&path).unwrap()).expect("read fixture lock");
        update(&mut lock);
        write_json(&path, &lock);
    }

    pub fn snapshot(&self) -> Vec<(PathBuf, Vec<u8>)> {
        let root = self.workspace.join(".liknon/resources");
        let mut files = Vec::new();
        collect_files(&root, &root, &mut files);
        files.sort_by(|left, right| left.0.cmp(&right.0));
        files
    }
}

pub fn run_tool(arguments: &[String]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_liknon-tool"))
        .args(arguments)
        .output()
        .expect("run liknon-tool")
}

pub fn run_routed(arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_liknon"))
        .arg("t")
        .args(arguments)
        .output()
        .expect("route liknon t")
}

pub fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "status: {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

pub fn json_output(mut arguments: Vec<String>) -> serde_json::Value {
    arguments.push("--format=json".to_owned());
    let output = run_tool(&arguments);
    assert_success(&output);
    assert!(!output.stdout.contains(&0x1b));
    assert!(!output.stderr.contains(&0x1b));
    serde_json::from_slice(&output.stdout).expect("valid Tool JSON")
}

pub fn recompute_manifest_digest(resource: &mut LockedResource) {
    resource.manifest_sha256 =
        canonical_json_sha256(&resource.resource).expect("canonical resource metadata");
}

fn resource(kind: ResourceKind, name: &str, payloads: Vec<PayloadMetadata>) -> ResourceVersion {
    ResourceVersion {
        schema_version: RESOURCE_SCHEMA_VERSION,
        identity: ResourceIdentity::new(kind, "fixture", name).unwrap(),
        version: "1.0.0".to_owned(),
        title: format!("Fixture {name}"),
        summary: format!("Deterministic Tool fixture for {name}"),
        authors: vec!["Liknon fixture".to_owned()],
        license: "MIT".to_owned(),
        source: SourceMetadata {
            repository: "https://example.invalid/liknon-tool-fixture".to_owned(),
            revision: Some("fixture-1".to_owned()),
            path: Some(name.to_owned()),
        },
        trust: TrustMetadata {
            class: TrustClass::Local,
            publisher: "Liknon fixture".to_owned(),
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
        capabilities: vec!["read-content".to_owned()],
        contains_executable_content: false,
        payloads,
        workflow: None,
        yanked: false,
        deprecated: false,
        revoked: false,
    }
}

fn workflow(
    name: &str,
    prefix: &str,
    entry_action: &str,
    terminal_action: &str,
) -> ResourceVersion {
    let mut resource = resource(ResourceKind::Workflow, name, Vec::new());
    let entry = format!("{prefix}-start");
    let terminal = format!("{prefix}-finish");
    resource.workflow = Some(WorkflowMetadata {
        entry_nodes: vec![entry.clone()],
        terminal_nodes: vec![terminal.clone()],
        nodes: vec![
            WorkflowNode {
                id: entry.clone(),
                action: entry_action.to_owned(),
                agent_profiles: Vec::new(),
                tool_operations: vec!["resource.catalog".to_owned()],
                effects: vec!["workspace-read".to_owned()],
                required_capabilities: vec!["resource-read".to_owned()],
            },
            WorkflowNode {
                id: terminal.clone(),
                action: terminal_action.to_owned(),
                agent_profiles: Vec::new(),
                tool_operations: Vec::new(),
                effects: vec!["report-write".to_owned()],
                required_capabilities: Vec::new(),
            },
        ],
        transitions: vec![WorkflowTransition {
            id: format!("{prefix}-continue"),
            from: entry,
            to: terminal,
            precondition: Some("human-approved".to_owned()),
            continuation_eligible: true,
        }],
        artifacts: Vec::new(),
    });
    resource
}

fn request(kind: ResourceKind, name: &str) -> ResourceRequest {
    ResourceRequest {
        identity: ResourceIdentity::new(kind, "fixture", name).unwrap(),
        requirement: "^1".to_owned(),
        source: RequestedSource::Local {
            path: "fixture-source".to_owned(),
        },
    }
}

fn payload(path: &str, media_type: &str, bytes: &[u8]) -> PayloadMetadata {
    let sha256 = sha256_bytes(bytes);
    PayloadMetadata {
        path: path.to_owned(),
        object: format!("objects/sha256/{}/{sha256}", &sha256[..2]),
        media_type: media_type.to_owned(),
        length: bytes.len() as u64,
        sha256,
        executable: false,
    }
}

fn locked(resource: ResourceVersion, dependencies: Vec<LockedEdge>) -> LockedResource {
    LockedResource {
        manifest_sha256: canonical_json_sha256(&resource).unwrap(),
        install_path: resource.identity.version_path(&resource.version).unwrap(),
        origin: LockedResourceOrigin::Local {
            path: "fixture-source".to_owned(),
        },
        resource,
        dependencies,
        compatibility: LockedCompatibility {
            manager: true,
            ai_bootstrap_declared: false,
            decision_policy_declared: false,
            activatable: false,
            activation_blockers: Vec::new(),
        },
    }
}

fn payload_bytes(identity: &ResourceIdentity, path: &str) -> &'static [u8] {
    match (identity.kind, identity.name.as_str(), path) {
        (ResourceKind::Knowledge, "evidence", "content.md") => EVIDENCE_TEXT,
        (ResourceKind::Skill, "review", "instructions.md") => REVIEW_TEXT,
        (ResourceKind::Skill, "review", "sample.bin") => REVIEW_BINARY,
        _ => panic!("unregistered fixture payload {identity}:{path}"),
    }
}

fn write_json(path: &Path, value: &impl serde::Serialize) {
    let mut bytes = serde_json::to_vec_pretty(value).expect("serialize fixture JSON");
    bytes.push(b'\n');
    fs::write(path, bytes).expect("write fixture JSON");
}

fn collect_files(root: &Path, current: &Path, files: &mut Vec<(PathBuf, Vec<u8>)>) {
    for entry in fs::read_dir(current).expect("read fixture directory") {
        let entry = entry.expect("fixture directory entry");
        let path = entry.path();
        let file_type = entry.file_type().expect("fixture file type");
        if file_type.is_dir() {
            collect_files(root, &path, files);
        } else if file_type.is_file() {
            files.push((
                path.strip_prefix(root).unwrap().to_path_buf(),
                fs::read(path).expect("read fixture file"),
            ));
        }
    }
}
