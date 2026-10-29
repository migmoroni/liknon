use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

use liknon_resource_contracts::{
    public_key_base64, sha256_bytes, sign_detached, CatalogPointer, CatalogSnapshot,
    CatalogSnapshotIdentity, Compatibility, PayloadMetadata, ResourceIdentity, ResourceKind,
    ResourceVersion, ReviewState, SignatureAlgorithm, SourceMetadata, TrustClass, TrustKey,
    TrustKeyStatus, TrustMetadata, TrustRoot, CATALOG_POINTER_SCHEMA_VERSION,
    CATALOG_SCHEMA_VERSION, RESOURCE_SCHEMA_VERSION, TRUST_ROOT_SCHEMA_VERSION,
};
use tempfile::TempDir;

const PRIVATE_KEY: [u8; 32] = [29; 32];
const KEY_ID: &str = "fixture-key";

pub struct Fixture {
    _temporary: TempDir,
    pub workspace: PathBuf,
    pub hub: PathBuf,
    pub trust_root: PathBuf,
    pub cache: PathBuf,
}

impl Fixture {
    pub fn new() -> Self {
        let temporary = TempDir::new().expect("create fixture root");
        let workspace = temporary.path().join("workspace");
        let hub = temporary.path().join("hub");
        let trust_root = temporary.path().join("trust-root.json");
        let cache = temporary.path().join("cache");
        fs::create_dir(&workspace).expect("create workspace");
        fs::create_dir(&hub).expect("create Hub");
        write_trust_root(&trust_root);
        Self {
            _temporary: temporary,
            workspace,
            hub,
            trust_root,
            cache,
        }
    }

    pub fn publish(&self, sequence: u64, resources: Vec<ResourceVersion>) {
        publish(&self.hub, sequence, resources);
    }

    pub fn hub_arguments(&self) -> Vec<String> {
        vec![
            "--hub".to_owned(),
            self.hub.display().to_string(),
            "--trust-root".to_owned(),
            self.trust_root.display().to_string(),
            "--cache".to_owned(),
            self.cache.display().to_string(),
        ]
    }

    pub fn workspace_arguments(&self) -> Vec<String> {
        vec![
            "--workspace".to_owned(),
            self.workspace.display().to_string(),
        ]
    }
}

pub fn resource(kind: ResourceKind, name: &str, version: &str, payload: &[u8]) -> ResourceVersion {
    let digest = sha256_bytes(payload);
    ResourceVersion {
        schema_version: RESOURCE_SCHEMA_VERSION,
        identity: ResourceIdentity::new(kind, "official", name).expect("fixture identity"),
        version: version.to_owned(),
        title: format!("Fixture {name}"),
        summary: format!("Deterministic fixture for {name}"),
        authors: vec!["Liknon fixture".to_owned()],
        license: "MIT".to_owned(),
        source: SourceMetadata {
            repository: "https://example.invalid/liknon-fixtures".to_owned(),
            revision: Some(format!("fixture-{version}")),
            path: Some(name.to_owned()),
        },
        trust: TrustMetadata {
            class: TrustClass::Official,
            publisher: "Liknon fixture".to_owned(),
            review: ReviewState::Reviewed,
            evidence: Some("fixture-review".to_owned()),
        },
        compatibility: Compatibility {
            resource_schema: "^1".to_owned(),
            manager: ">=0.5, <0.6".to_owned(),
            tool: None,
            validator: None,
            ai_bootstrap: None,
            decision_policy: None,
        },
        dependencies: Vec::new(),
        collection_members: Vec::new(),
        capabilities: Vec::new(),
        contains_executable_content: false,
        payloads: vec![PayloadMetadata {
            path: "content.md".to_owned(),
            object: format!("objects/sha256/{}/{digest}", &digest[..2]),
            media_type: "text/markdown".to_owned(),
            length: payload.len() as u64,
            sha256: digest,
            executable: false,
        }],
        workflow: None,
        yanked: false,
        deprecated: false,
        revoked: false,
    }
}

pub fn run_manager(arguments: &[String]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_liknon-manager"))
        .args(arguments)
        .output()
        .expect("run liknon-manager")
}

pub fn run_manager_from(directory: &Path, arguments: &[String]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_liknon-manager"))
        .current_dir(directory)
        .args(arguments)
        .output()
        .expect("run liknon-manager from directory")
}

pub fn run_routed(arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_liknon"))
        .arg("m")
        .args(arguments)
        .output()
        .expect("route liknon m")
}

pub fn write_local_resource(root: &Path, resource: &ResourceVersion, payload: &[u8]) {
    fs::create_dir_all(root).expect("create local resource");
    write_json(&root.join("resource.json"), resource);
    for metadata in &resource.payloads {
        assert_eq!(metadata.length, payload.len() as u64);
        assert_eq!(metadata.sha256, sha256_bytes(payload));
        write_bytes(&root.join(&metadata.object), payload);
    }
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

fn publish(root: &Path, sequence: u64, mut resources: Vec<ResourceVersion>) {
    resources.sort_by(|left, right| {
        left.identity.cmp(&right.identity).then_with(|| {
            semver::Version::parse(&right.version)
                .expect("fixture version")
                .cmp(&semver::Version::parse(&left.version).expect("fixture version"))
        })
    });
    for resource in &resources {
        for payload in &resource.payloads {
            let bytes = format!(
                "{}:{}:{}",
                resource.identity, resource.version, resource.summary
            );
            let expected = payload.sha256.clone();
            let fixture_bytes = fixture_payload(resource);
            assert_eq!(
                sha256_bytes(&fixture_bytes),
                expected,
                "fixture payload lookup"
            );
            write_bytes(&root.join(&payload.object), &fixture_bytes);
            let _ = bytes;
        }
    }
    let now = now();
    let snapshot = CatalogSnapshotIdentity {
        id: format!("fixture-{sequence}"),
        sequence,
        published_at_unix: now.saturating_sub(10),
        expires_at_unix: now + 86_400,
    };
    let catalog = CatalogSnapshot {
        schema_version: CATALOG_SCHEMA_VERSION,
        snapshot: snapshot.clone(),
        resources,
        advisories: Vec::new(),
    };
    let catalog_bytes = json_bytes(&catalog);
    let catalog_path = format!("snapshots/{}/catalog.json", snapshot.id);
    let signature_path = format!("{catalog_path}.sig.json");
    write_bytes(&root.join(&catalog_path), &catalog_bytes);
    write_json(
        &root.join(&signature_path),
        &sign_detached(
            &catalog_bytes,
            KEY_ID,
            snapshot.published_at_unix,
            &PRIVATE_KEY,
        ),
    );
    let pointer = CatalogPointer {
        schema_version: CATALOG_POINTER_SCHEMA_VERSION,
        snapshot: snapshot.clone(),
        catalog_path,
        catalog_length: catalog_bytes.len() as u64,
        catalog_sha256: sha256_bytes(&catalog_bytes),
        catalog_signature_path: signature_path,
    };
    let pointer_bytes = json_bytes(&pointer);
    write_bytes(&root.join("stable.json"), &pointer_bytes);
    write_json(
        &root.join("stable.json.sig.json"),
        &sign_detached(
            &pointer_bytes,
            KEY_ID,
            snapshot.published_at_unix,
            &PRIVATE_KEY,
        ),
    );
}

fn fixture_payload(resource: &ResourceVersion) -> Vec<u8> {
    match (resource.identity.name.as_str(), resource.version.as_str()) {
        ("evidence", "1.0.0") => b"evidence version one\n".to_vec(),
        ("evidence", "1.1.0") => b"evidence version two\n".to_vec(),
        ("review", "1.0.0") => b"review skill\n".to_vec(),
        _ => panic!(
            "fixture payload is not registered for {}@{}",
            resource.identity, resource.version
        ),
    }
}

fn write_trust_root(path: &Path) {
    let root = TrustRoot {
        schema_version: TRUST_ROOT_SCHEMA_VERSION,
        authority: "liknon-test-hub".to_owned(),
        keys: vec![TrustKey {
            key_id: KEY_ID.to_owned(),
            algorithm: SignatureAlgorithm::Ed25519,
            public_key: public_key_base64(&PRIVATE_KEY),
            not_before_unix: 1,
            not_after_unix: now() + 31_536_000,
            status: TrustKeyStatus::Active,
        }],
    };
    write_json(path, &root);
}

fn json_bytes(value: &impl serde::Serialize) -> Vec<u8> {
    let mut bytes = serde_json::to_vec_pretty(value).expect("serialize fixture JSON");
    bytes.push(b'\n');
    bytes
}

fn write_json(path: &Path, value: &impl serde::Serialize) {
    write_bytes(path, &json_bytes(value));
}

fn write_bytes(path: &Path, bytes: &[u8]) {
    fs::create_dir_all(path.parent().expect("fixture parent")).expect("create fixture parent");
    fs::write(path, bytes).expect("write fixture bytes");
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("valid system clock")
        .as_secs()
}
