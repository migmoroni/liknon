//! Private deterministic Hub assembly and verification utility.

use std::{
    fs,
    io::{self, Write as _},
    path::{Path, PathBuf},
    process::ExitCode,
    time::{SystemTime, UNIX_EPOCH},
};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use clap::{Parser, Subcommand};
use liknon_resource_contracts::{
    public_key_base64, sha256_bytes, sign_detached, validate_catalog, validate_relative_path,
    validate_resource, validate_sha256, validate_trust_root, verify_detached, CatalogPointer,
    CatalogSnapshot, CatalogSnapshotIdentity, Compatibility, DetachedSignature, PayloadMetadata,
    ResourceDependency, ResourceIdentity, ResourceVersion, SecurityAdvisory, SignatureAlgorithm,
    SourceMetadata, TrustClass, TrustKey, TrustKeyStatus, TrustMetadata, TrustRoot,
    ValidationLimits, WorkflowMetadata, CATALOG_POINTER_SCHEMA_VERSION, CATALOG_SCHEMA_VERSION,
    TRUST_ROOT_SCHEMA_VERSION,
};
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Parser)]
#[command(
    name = "liknon-resource-hub-maintenance",
    about = "Assembles and verifies static Liknon Hub snapshots"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Writes the canonical public resource protocol schemas.
    Schema {
        /// Destination directory for checked-in JSON Schemas.
        #[arg(long)]
        output: PathBuf,
    },
    /// Derives a public trust root from one private signing seed.
    TrustRoot {
        /// Standard Base64 file containing exactly 32 private seed bytes.
        #[arg(long)]
        private_key: PathBuf,
        /// Stable metadata-signing key ID.
        #[arg(long)]
        key_id: String,
        /// Human-readable Hub authority.
        #[arg(long)]
        authority: String,
        /// First accepted Unix second.
        #[arg(long)]
        not_before: u64,
        /// Last accepted Unix second.
        #[arg(long)]
        not_after: u64,
        /// Output JSON path.
        #[arg(long)]
        output: PathBuf,
    },
    /// Builds one immutable snapshot and signed stable pointer.
    Assemble {
        /// Directory recursively containing authored `resource.json` files.
        #[arg(long)]
        source: PathBuf,
        /// Destination static-Hub root.
        #[arg(long)]
        output: PathBuf,
        /// Standard Base64 file containing exactly 32 private seed bytes.
        #[arg(long)]
        private_key: PathBuf,
        /// Stable signing key ID present in the separate trust root.
        #[arg(long)]
        key_id: String,
        /// Normalized immutable snapshot ID.
        #[arg(long)]
        snapshot: String,
        /// Monotonic positive snapshot sequence.
        #[arg(long)]
        sequence: u64,
        /// Reproducible authenticated publication Unix second.
        #[arg(long)]
        published_at: u64,
        /// Reproducible authenticated expiry Unix second.
        #[arg(long)]
        expires_at: u64,
    },
    /// Verifies one assembled local Hub against a separate trust root.
    Verify {
        /// Static-Hub root.
        #[arg(long)]
        hub: PathBuf,
        /// Public trust-root JSON path.
        #[arg(long)]
        trust_root: PathBuf,
        /// Unix second used for expiry validation.
        #[arg(long)]
        now: u64,
    },
}

#[derive(Debug, Error)]
enum HubError {
    #[error("I/O operation failed: {0}")]
    Io(#[from] io::Error),
    #[error("invalid JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid Hub contract: {0}")]
    Contract(#[from] liknon_resource_contracts::ContractError),
    #[error("invalid Hub source: {0}")]
    Source(String),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct AuthoredResource {
    schema_version: u32,
    identity: ResourceIdentity,
    version: String,
    title: String,
    summary: String,
    authors: Vec<String>,
    license: String,
    source: SourceMetadata,
    trust: TrustMetadata,
    compatibility: Compatibility,
    #[serde(default)]
    dependencies: Vec<ResourceDependency>,
    #[serde(default)]
    collection_members: Vec<ResourceDependency>,
    #[serde(default)]
    capabilities: Vec<String>,
    #[serde(default)]
    contains_executable_content: bool,
    #[serde(default)]
    payloads: Vec<AuthoredPayload>,
    #[serde(default)]
    workflow: Option<WorkflowMetadata>,
    #[serde(default)]
    yanked: bool,
    #[serde(default)]
    deprecated: bool,
    #[serde(default)]
    revoked: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct AuthoredPayload {
    path: String,
    source: String,
    media_type: String,
    #[serde(default)]
    executable: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct AuthoredAdvisories {
    schema_version: u32,
    advisories: Vec<SecurityAdvisory>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AssemblyResult {
    schema_version: u32,
    snapshot: CatalogSnapshotIdentity,
    catalog_sha256: String,
    resource_count: usize,
    object_count: usize,
}

fn main() -> ExitCode {
    match execute(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::from(1)
        }
    }
}

fn execute(cli: Cli) -> Result<(), HubError> {
    match cli.command {
        Command::Schema { output } => write_schemas(&output),
        Command::TrustRoot {
            private_key,
            key_id,
            authority,
            not_before,
            not_after,
            output,
        } => write_trust_root(
            &private_key,
            key_id,
            authority,
            not_before,
            not_after,
            &output,
        ),
        Command::Assemble {
            source,
            output,
            private_key,
            key_id,
            snapshot,
            sequence,
            published_at,
            expires_at,
        } => assemble(AssemblyOptions {
            source,
            output,
            private_key,
            key_id,
            snapshot,
            sequence,
            published_at,
            expires_at,
        }),
        Command::Verify {
            hub,
            trust_root,
            now,
        } => verify_hub(&hub, &trust_root, now),
    }
}

fn write_schemas(output: &Path) -> Result<(), HubError> {
    fs::create_dir_all(output)?;
    for (name, schema) in [
        (
            "resource.schema.json",
            serde_json::to_value(schemars::schema_for!(ResourceVersion))?,
        ),
        (
            "catalog.schema.json",
            serde_json::to_value(schemars::schema_for!(CatalogSnapshot))?,
        ),
        (
            "manifest.schema.json",
            serde_json::to_value(schemars::schema_for!(
                liknon_resource_contracts::ResourceManifest
            ))?,
        ),
        (
            "lock.schema.json",
            serde_json::to_value(schemars::schema_for!(
                liknon_resource_contracts::ResourceLock
            ))?,
        ),
        (
            "trust-root.schema.json",
            serde_json::to_value(schemars::schema_for!(TrustRoot))?,
        ),
        (
            "signature.schema.json",
            serde_json::to_value(schemars::schema_for!(DetachedSignature))?,
        ),
    ] {
        write_json(&output.join(name), &schema)?;
    }
    Ok(())
}

fn write_trust_root(
    private_key: &Path,
    key_id: String,
    authority: String,
    not_before: u64,
    not_after: u64,
    output: &Path,
) -> Result<(), HubError> {
    let private_key = read_private_key(private_key)?;
    let trust_root = TrustRoot {
        schema_version: TRUST_ROOT_SCHEMA_VERSION,
        authority,
        keys: vec![TrustKey {
            key_id,
            algorithm: SignatureAlgorithm::Ed25519,
            public_key: public_key_base64(&private_key),
            not_before_unix: not_before,
            not_after_unix: not_after,
            status: TrustKeyStatus::Active,
        }],
    };
    validate_trust_root(&trust_root)?;
    write_json(output, &trust_root)
}

struct AssemblyOptions {
    source: PathBuf,
    output: PathBuf,
    private_key: PathBuf,
    key_id: String,
    snapshot: String,
    sequence: u64,
    published_at: u64,
    expires_at: u64,
}

fn assemble(options: AssemblyOptions) -> Result<(), HubError> {
    if options.output.exists() {
        return Err(HubError::Source(format!(
            "output `{}` already exists; immutable Hub output is never overwritten",
            options.output.display()
        )));
    }
    let private_key = read_private_key(&options.private_key)?;
    let manifest_paths = discover_manifests(&options.source)?;
    if manifest_paths.is_empty() {
        return Err(HubError::Source(
            "source contains no resource.json files".to_owned(),
        ));
    }

    let parent = options
        .output
        .parent()
        .ok_or_else(|| HubError::Source("Hub output must have a parent directory".to_owned()))?;
    fs::create_dir_all(parent)?;
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| HubError::Source(format!("system clock precedes Unix epoch: {error}")))?
        .as_nanos();
    let staging = parent.join(format!(
        ".{}-staging-{}-{nonce}",
        options
            .output
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("hub"),
        std::process::id()
    ));
    fs::create_dir(&staging)?;

    let result = assemble_into(&options, &manifest_paths, &private_key, &staging);
    match result {
        Ok(summary) => {
            fs::rename(&staging, &options.output)?;
            serde_json::to_writer_pretty(io::stdout().lock(), &summary)?;
            println!();
            Ok(())
        }
        Err(error) => {
            let _ = fs::remove_dir_all(&staging);
            Err(error)
        }
    }
}

fn assemble_into(
    options: &AssemblyOptions,
    manifest_paths: &[PathBuf],
    private_key: &[u8; 32],
    staging: &Path,
) -> Result<AssemblyResult, HubError> {
    let mut resources = Vec::new();
    let mut object_digests = std::collections::BTreeSet::new();
    for manifest_path in manifest_paths {
        let manifest_bytes = fs::read(manifest_path)?;
        let authored: AuthoredResource = serde_json::from_slice(&manifest_bytes)?;
        let base = manifest_path.parent().ok_or_else(|| {
            HubError::Source(format!(
                "manifest `{}` has no parent",
                manifest_path.display()
            ))
        })?;
        if authored.trust.class == TrustClass::Local {
            return Err(HubError::Source(format!(
                "Hub resource {} cannot use the local trust class",
                authored.identity
            )));
        }
        let mut payloads = Vec::new();
        for payload in authored.payloads {
            validate_relative_path(&payload.path)?;
            validate_relative_path(&payload.source)?;
            let bytes = read_confined_file(base, &payload.source)?;
            let digest = sha256_bytes(&bytes);
            let object = format!("objects/sha256/{}/{digest}", &digest[..2]);
            let object_path = staging.join(&object);
            if object_digests.insert(digest.clone()) {
                write_bytes(&object_path, &bytes)?;
            }
            payloads.push(PayloadMetadata {
                path: payload.path,
                object,
                media_type: payload.media_type,
                length: u64::try_from(bytes.len())
                    .map_err(|_| HubError::Source("payload length does not fit u64".to_owned()))?,
                sha256: digest,
                executable: payload.executable,
            });
        }
        payloads.sort_by(|left, right| left.path.cmp(&right.path));
        let mut dependencies = authored.dependencies;
        dependencies.sort_by(|left, right| left.identity.cmp(&right.identity));
        let mut collection_members = authored.collection_members;
        collection_members.sort_by(|left, right| left.identity.cmp(&right.identity));
        let mut capabilities = authored.capabilities;
        capabilities.sort();
        let resource = ResourceVersion {
            schema_version: authored.schema_version,
            identity: authored.identity,
            version: authored.version,
            title: authored.title,
            summary: authored.summary,
            authors: authored.authors,
            license: authored.license,
            source: authored.source,
            trust: authored.trust,
            compatibility: authored.compatibility,
            dependencies,
            collection_members,
            capabilities,
            contains_executable_content: authored.contains_executable_content,
            payloads,
            workflow: authored.workflow,
            yanked: authored.yanked,
            deprecated: authored.deprecated,
            revoked: authored.revoked,
        };
        validate_resource(&resource, ValidationLimits::default())?;
        resources.push(resource);
    }
    resources.sort_by(|left, right| {
        left.identity.cmp(&right.identity).then_with(|| {
            let left = Version::parse(&left.version).expect("validated below");
            let right = Version::parse(&right.version).expect("validated below");
            right.cmp(&left)
        })
    });
    let snapshot = CatalogSnapshotIdentity {
        id: options.snapshot.clone(),
        sequence: options.sequence,
        published_at_unix: options.published_at,
        expires_at_unix: options.expires_at,
    };
    let catalog = CatalogSnapshot {
        schema_version: CATALOG_SCHEMA_VERSION,
        snapshot: snapshot.clone(),
        resources,
        advisories: read_advisories(&options.source)?,
    };
    validate_hub_catalog(&catalog)?;

    let catalog_path = format!("snapshots/{}/catalog.json", snapshot.id);
    let catalog_signature_path = format!("{catalog_path}.sig.json");
    let catalog_bytes = json_bytes(&catalog)?;
    write_bytes(&staging.join(&catalog_path), &catalog_bytes)?;
    let catalog_signature = sign_detached(
        &catalog_bytes,
        &options.key_id,
        options.published_at,
        private_key,
    );
    write_json(&staging.join(&catalog_signature_path), &catalog_signature)?;

    let pointer = CatalogPointer {
        schema_version: CATALOG_POINTER_SCHEMA_VERSION,
        snapshot: snapshot.clone(),
        catalog_path,
        catalog_length: u64::try_from(catalog_bytes.len())
            .map_err(|_| HubError::Source("catalog length does not fit u64".to_owned()))?,
        catalog_sha256: sha256_bytes(&catalog_bytes),
        catalog_signature_path,
    };
    let pointer_bytes = json_bytes(&pointer)?;
    write_bytes(&staging.join("stable.json"), &pointer_bytes)?;
    let pointer_signature = sign_detached(
        &pointer_bytes,
        &options.key_id,
        options.published_at,
        private_key,
    );
    write_json(&staging.join("stable.json.sig.json"), &pointer_signature)?;

    Ok(AssemblyResult {
        schema_version: 1,
        snapshot,
        catalog_sha256: pointer.catalog_sha256,
        resource_count: catalog.resources.len(),
        object_count: object_digests.len(),
    })
}

fn verify_hub(hub: &Path, trust_root_path: &Path, now: u64) -> Result<(), HubError> {
    let trust_root: TrustRoot = read_json(trust_root_path)?;
    validate_trust_root(&trust_root)?;
    let pointer_bytes = read_confined_file(hub, "stable.json")?;
    let pointer_signature: DetachedSignature =
        serde_json::from_slice(&read_confined_file(hub, "stable.json.sig.json")?)?;
    verify_detached(&pointer_bytes, &pointer_signature, &trust_root)?;
    let pointer: CatalogPointer = serde_json::from_slice(&pointer_bytes)?;
    if pointer.schema_version != CATALOG_POINTER_SCHEMA_VERSION {
        return Err(HubError::Source(format!(
            "unsupported catalog pointer schema {}",
            pointer.schema_version
        )));
    }
    if now > pointer.snapshot.expires_at_unix {
        return Err(HubError::Source("catalog pointer is expired".to_owned()));
    }
    if pointer.snapshot.published_at_unix > now.saturating_add(300)
        || pointer_signature.signed_at_unix != pointer.snapshot.published_at_unix
    {
        return Err(HubError::Source(
            "catalog pointer has an invalid authenticated time".to_owned(),
        ));
    }
    validate_relative_path(&pointer.catalog_path)?;
    validate_relative_path(&pointer.catalog_signature_path)?;
    validate_sha256(&pointer.catalog_sha256)?;
    let catalog_bytes = read_confined_file(hub, &pointer.catalog_path)?;
    if catalog_bytes.len() as u64 != pointer.catalog_length
        || sha256_bytes(&catalog_bytes) != pointer.catalog_sha256
    {
        return Err(HubError::Source(
            "catalog bytes do not match the authenticated pointer".to_owned(),
        ));
    }
    let catalog_signature: DetachedSignature =
        serde_json::from_slice(&read_confined_file(hub, &pointer.catalog_signature_path)?)?;
    verify_detached(&catalog_bytes, &catalog_signature, &trust_root)?;
    if catalog_signature.signed_at_unix != pointer.snapshot.published_at_unix {
        return Err(HubError::Source(
            "catalog signature time differs from publication time".to_owned(),
        ));
    }
    let catalog: CatalogSnapshot = serde_json::from_slice(&catalog_bytes)?;
    validate_hub_catalog(&catalog)?;
    if catalog.snapshot != pointer.snapshot {
        return Err(HubError::Source(
            "catalog identity differs from the stable pointer".to_owned(),
        ));
    }
    for resource in &catalog.resources {
        for payload in &resource.payloads {
            let bytes = read_confined_file(hub, &payload.object)?;
            if bytes.len() as u64 != payload.length || sha256_bytes(&bytes) != payload.sha256 {
                return Err(HubError::Source(format!(
                    "payload `{}` does not match {}",
                    payload.path, resource.identity
                )));
            }
        }
    }
    println!(
        "verified snapshot {} with {} resources",
        catalog.snapshot.id,
        catalog.resources.len()
    );
    Ok(())
}

fn validate_hub_catalog(catalog: &CatalogSnapshot) -> Result<(), HubError> {
    validate_catalog(catalog, ValidationLimits::default())?;
    for resource in &catalog.resources {
        if resource.trust.class == TrustClass::Local {
            return Err(HubError::Source(format!(
                "Hub resource {} uses the local trust class",
                resource.identity
            )));
        }
        for payload in &resource.payloads {
            let expected = format!("objects/sha256/{}/{}", &payload.sha256[..2], payload.sha256);
            if payload.object != expected {
                return Err(HubError::Source(format!(
                    "payload `{}` for {} is not content-addressed canonically",
                    payload.path, resource.identity
                )));
            }
        }
    }
    for resource in catalog.resources.iter().filter(|resource| resource.revoked) {
        let version = Version::parse(&resource.version)
            .map_err(|error| HubError::Source(error.to_string()))?;
        let covered = catalog.advisories.iter().any(|advisory| {
            advisory.revoked
                && advisory.identity == resource.identity
                && VersionReq::parse(&advisory.affected)
                    .is_ok_and(|requirement| requirement.matches(&version))
        });
        if !covered {
            return Err(HubError::Source(format!(
                "revoked resource {}@{} has no matching revocation advisory",
                resource.identity, resource.version
            )));
        }
    }
    Ok(())
}

fn read_advisories(source: &Path) -> Result<Vec<SecurityAdvisory>, HubError> {
    let path = source.join("advisories.json");
    if !path.exists() {
        return Ok(Vec::new());
    }
    let authored: AuthoredAdvisories =
        serde_json::from_slice(&read_confined_file(source, "advisories.json")?)?;
    if authored.schema_version != CATALOG_SCHEMA_VERSION {
        return Err(HubError::Source(format!(
            "unsupported authored advisory schema {}",
            authored.schema_version
        )));
    }
    let mut advisories = authored.advisories;
    advisories.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(advisories)
}

fn read_confined_file(root: &Path, relative: &str) -> Result<Vec<u8>, HubError> {
    validate_relative_path(relative)?;
    let root = fs::canonicalize(root)?;
    if !root.is_dir() {
        return Err(HubError::Source(format!(
            "confined root `{}` is not a directory",
            root.display()
        )));
    }
    let mut current = root.clone();
    for component in Path::new(relative).components() {
        current.push(component);
        let metadata = fs::symlink_metadata(&current)?;
        if metadata.file_type().is_symlink() {
            return Err(HubError::Source(format!(
                "confined file `{relative}` traverses a symlink"
            )));
        }
    }
    let canonical = fs::canonicalize(&current)?;
    if !canonical.starts_with(&root) || !canonical.is_file() {
        return Err(HubError::Source(format!(
            "confined file `{relative}` escapes its root"
        )));
    }
    Ok(fs::read(canonical)?)
}

fn discover_manifests(root: &Path) -> Result<Vec<PathBuf>, HubError> {
    let metadata = fs::symlink_metadata(root)?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(HubError::Source(format!(
            "source `{}` must be a non-symlink directory",
            root.display()
        )));
    }
    let mut manifests = Vec::new();
    discover_directory(root, &mut manifests)?;
    manifests.sort();
    Ok(manifests)
}

fn discover_directory(directory: &Path, manifests: &mut Vec<PathBuf>) -> Result<(), HubError> {
    let mut entries = fs::read_dir(directory)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(fs::DirEntry::file_name);
    for entry in entries {
        let metadata = fs::symlink_metadata(entry.path())?;
        if metadata.file_type().is_symlink() {
            return Err(HubError::Source(format!(
                "source tree contains symlink `{}`",
                entry.path().display()
            )));
        }
        if metadata.is_dir() {
            discover_directory(&entry.path(), manifests)?;
        } else if metadata.is_file() && entry.file_name() == "resource.json" {
            manifests.push(entry.path());
        } else if !metadata.is_file() {
            return Err(HubError::Source(format!(
                "unsupported source entry `{}`",
                entry.path().display()
            )));
        }
    }
    Ok(())
}

fn read_private_key(path: &Path) -> Result<[u8; 32], HubError> {
    let encoded = fs::read_to_string(path)?;
    let decoded = STANDARD
        .decode(encoded.trim())
        .map_err(|error| HubError::Source(format!("invalid private key Base64: {error}")))?;
    decoded.try_into().map_err(|bytes: Vec<u8>| {
        HubError::Source(format!(
            "private signing seed must contain 32 bytes, found {}",
            bytes.len()
        ))
    })
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, HubError> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

fn json_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, HubError> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), HubError> {
    write_bytes(path, &json_bytes(value)?)
}

fn write_bytes(path: &Path, bytes: &[u8]) -> Result<(), HubError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let temporary = path.with_extension(format!(
        "{}.tmp-{}",
        path.extension()
            .and_then(|value| value.to_str())
            .unwrap_or("file"),
        std::process::id()
    ));
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    fs::rename(temporary, path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        assemble, discover_manifests, read_private_key, verify_hub, write_trust_root,
        AssemblyOptions,
    };
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    use liknon_resource_contracts::{CatalogPointer, CatalogSnapshot};
    use std::{fs, path::Path};
    use tempfile::TempDir;

    #[test]
    fn reads_exact_private_seed_and_discovers_sorted_manifests() {
        let temporary = TempDir::new().unwrap();
        let key = temporary.path().join("key");
        fs::write(&key, STANDARD.encode([3u8; 32])).unwrap();
        assert_eq!(read_private_key(&key).unwrap(), [3u8; 32]);

        for name in ["b", "a"] {
            let directory = temporary.path().join(name);
            fs::create_dir(&directory).unwrap();
            fs::write(directory.join("resource.json"), "{}").unwrap();
        }
        let manifests = discover_manifests(temporary.path()).unwrap();
        assert!(manifests[0] < manifests[1]);
    }

    #[test]
    fn assembles_and_verifies_the_neutral_fixture_reproducibly() {
        let source =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/resource-hub/source");
        let temporary = TempDir::new().unwrap();
        let key = temporary.path().join("private-key.txt");
        let trust_root = temporary.path().join("trust-root.json");
        fs::write(&key, STANDARD.encode([7u8; 32])).unwrap();
        write_trust_root(
            &key,
            "fixture-hub-key".to_owned(),
            "Liknon fixture Hub".to_owned(),
            1_700_000_000,
            1_800_000_000,
            &trust_root,
        )
        .unwrap();

        let output_a = temporary.path().join("hub-a");
        let output_b = temporary.path().join("hub-b");
        for output in [&output_a, &output_b] {
            assemble(AssemblyOptions {
                source: source.clone(),
                output: output.clone(),
                private_key: key.clone(),
                key_id: "fixture-hub-key".to_owned(),
                snapshot: "fixture-2026-01".to_owned(),
                sequence: 1,
                published_at: 1_750_000_000,
                expires_at: 1_760_000_000,
            })
            .unwrap();
            verify_hub(output, &trust_root, 1_755_000_000).unwrap();
        }

        assert_eq!(read_tree(&output_a), read_tree(&output_b));
        let pointer: CatalogPointer =
            serde_json::from_slice(&fs::read(output_a.join("stable.json")).unwrap()).unwrap();
        let catalog: CatalogSnapshot =
            serde_json::from_slice(&fs::read(output_a.join(&pointer.catalog_path)).unwrap())
                .unwrap();
        assert_eq!(catalog.resources.len(), 8);
        assert_eq!(catalog.advisories.len(), 1);
        assert!(catalog.resources.iter().any(|resource| resource.revoked));
        assert!(catalog
            .resources
            .iter()
            .any(|resource| resource.workflow.is_some()));
        assert!(verify_hub(&output_a, &trust_root, 1_760_000_001).is_err());

        let payload = &catalog.resources[0].payloads[0];
        fs::write(output_b.join(&payload.object), b"tampered payload\n").unwrap();
        assert!(verify_hub(&output_b, &trust_root, 1_755_000_000).is_err());

        let error = assemble(AssemblyOptions {
            source,
            output: output_a,
            private_key: key,
            key_id: "fixture-hub-key".to_owned(),
            snapshot: "fixture-2026-01".to_owned(),
            sequence: 1,
            published_at: 1_750_000_000,
            expires_at: 1_760_000_000,
        })
        .unwrap_err();
        assert!(error.to_string().contains("never overwritten"));
    }

    #[cfg(unix)]
    #[test]
    fn source_discovery_rejects_symlinks() {
        use std::os::unix::fs::symlink;

        let temporary = TempDir::new().unwrap();
        let external = temporary.path().join("external.json");
        let source = temporary.path().join("source");
        fs::create_dir(&source).unwrap();
        fs::write(&external, "{}").unwrap();
        symlink(&external, source.join("resource.json")).unwrap();

        let error = discover_manifests(&source).unwrap_err();
        assert!(error.to_string().contains("contains symlink"));
    }

    fn read_tree(root: &Path) -> Vec<(String, Vec<u8>)> {
        fn visit(root: &Path, directory: &Path, files: &mut Vec<(String, Vec<u8>)>) {
            let mut entries = fs::read_dir(directory)
                .unwrap()
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            entries.sort_by_key(fs::DirEntry::file_name);
            for entry in entries {
                if entry.file_type().unwrap().is_dir() {
                    visit(root, &entry.path(), files);
                } else {
                    files.push((
                        entry
                            .path()
                            .strip_prefix(root)
                            .unwrap()
                            .to_string_lossy()
                            .replace('\\', "/"),
                        fs::read(entry.path()).unwrap(),
                    ));
                }
            }
        }

        let mut files = Vec::new();
        visit(root, root, &mut files);
        files
    }
}
