use std::{
    fs,
    io::{Read as _, Write as _},
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use liknon_resource_contracts::{
    sha256_bytes, validate_catalog, validate_relative_path, validate_sha256, validate_trust_root,
    verify_detached, CatalogPointer, CatalogSnapshot, DetachedSignature, PayloadMetadata,
    TrustRoot, ValidationLimits, CATALOG_POINTER_SCHEMA_VERSION,
};
use reqwest::{
    blocking::{Client, Response},
    redirect::{Action, Attempt, Policy},
    Url,
};
use serde::{Deserialize, Serialize};

use crate::{
    cli::HubArgs,
    error::{IoResultExt as _, ManagerError},
    workspace,
};

const MAX_METADATA_BYTES: u64 = 32 * 1024 * 1024;

#[derive(Debug, Clone)]
enum HubOrigin {
    Local(PathBuf),
    Https(Url),
}

impl HubOrigin {
    fn parse(value: &str) -> Result<Self, ManagerError> {
        if value.starts_with("https://")
            || value.starts_with("http://")
            || value.starts_with("file://")
        {
            let url = Url::parse(value)
                .map_err(|error| ManagerError::Usage(format!("invalid Hub origin: {error}")))?;
            return match url.scheme() {
                "https" => Ok(Self::Https(with_trailing_slash(url)?)),
                "file" => url
                    .to_file_path()
                    .map(Self::Local)
                    .map_err(|_| ManagerError::Usage("invalid file:// Hub path".to_owned())),
                "http" => Err(ManagerError::Usage(
                    "Hub network origins must use HTTPS; use file:// for local tests".to_owned(),
                )),
                scheme => Err(ManagerError::Usage(format!(
                    "unsupported Hub origin scheme `{scheme}`"
                ))),
            };
        }
        Ok(Self::Local(PathBuf::from(value)))
    }

    fn identity(&self) -> String {
        match self {
            Self::Local(path) => format!("file://{}", path.display()),
            Self::Https(url) => url.as_str().to_owned(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct HubSnapshot {
    pub(crate) catalog: CatalogSnapshot,
    pub(crate) origin: String,
    pub(crate) catalog_sha256: String,
    pub(crate) trust_authority: String,
    pub(crate) signing_key_id: String,
    pub(crate) evidence: AuthenticatedCatalogEvidence,
    client: HubClient,
}

#[derive(Debug, Clone)]
pub(crate) struct AuthenticatedCatalogEvidence {
    pub(crate) catalog_bytes: Vec<u8>,
    pub(crate) catalog_signature: DetachedSignature,
    pub(crate) trust_root: TrustRoot,
}

impl HubSnapshot {
    pub(crate) fn payload(&self, payload: &PayloadMetadata) -> Result<Vec<u8>, ManagerError> {
        self.client.fetch_payload(payload)
    }
}

#[derive(Debug)]
struct HubClient {
    origin: HubOrigin,
    cache: PathBuf,
    http: Option<Client>,
}

impl HubClient {
    fn new(origin: HubOrigin, cache: PathBuf, timeout: Duration) -> Result<Self, ManagerError> {
        let http = match &origin {
            HubOrigin::Local(_) => None,
            HubOrigin::Https(base) => {
                let expected_origin = base.origin().ascii_serialization();
                let policy = Policy::custom(move |attempt: Attempt<'_>| {
                    redirect_action(attempt, &expected_origin)
                });
                Some(
                    Client::builder()
                        .connect_timeout(timeout.min(Duration::from_secs(10)))
                        .timeout(timeout)
                        .redirect(policy)
                        .user_agent(concat!("liknon-manager/", env!("CARGO_PKG_VERSION")))
                        .build()
                        .map_err(|error| ManagerError::Network(error.to_string()))?,
                )
            }
        };
        Ok(Self {
            origin,
            cache: secure_cache_root(&cache)?,
            http,
        })
    }

    fn fetch_metadata(&self, path: &str) -> Result<Vec<u8>, ManagerError> {
        self.fetch(path, MAX_METADATA_BYTES)
    }

    fn fetch_payload(&self, payload: &PayloadMetadata) -> Result<Vec<u8>, ManagerError> {
        validate_relative_path(&payload.object)?;
        validate_sha256(&payload.sha256)?;
        let cache_path = self
            .cache
            .join("objects/sha256")
            .join(&payload.sha256[..2])
            .join(&payload.sha256);
        if let Some(bytes) = read_cache_file(&self.cache, &cache_path)? {
            if bytes.len() as u64 == payload.length && sha256_bytes(&bytes) == payload.sha256 {
                return Ok(bytes);
            }
        }
        let bytes = self.fetch(
            &payload.object,
            payload
                .length
                .checked_add(1)
                .ok_or_else(|| ManagerError::Integrity("payload length overflow".to_owned()))?,
        )?;
        if bytes.len() as u64 != payload.length || sha256_bytes(&bytes) != payload.sha256 {
            return Err(ManagerError::Integrity(format!(
                "Hub payload `{}` does not match authenticated length and digest",
                payload.object
            )));
        }
        atomic_cache_write(&self.cache, &cache_path, &bytes)?;
        Ok(bytes)
    }

    fn fetch(&self, path: &str, maximum: u64) -> Result<Vec<u8>, ManagerError> {
        validate_relative_path(path)?;
        match &self.origin {
            HubOrigin::Local(root) => read_local(root, path, maximum),
            HubOrigin::Https(base) => {
                let url = base
                    .join(path)
                    .map_err(|error| ManagerError::Network(error.to_string()))?;
                if url.origin() != base.origin() {
                    return Err(ManagerError::Integrity(format!(
                        "Hub object `{path}` changes origin"
                    )));
                }
                self.fetch_https(url, maximum)
            }
        }
    }

    fn fetch_https(&self, url: Url, maximum: u64) -> Result<Vec<u8>, ManagerError> {
        let client = self
            .http
            .as_ref()
            .expect("HTTPS origin always has a client");
        let mut last_error = None;
        for attempt in 0..3 {
            match client.get(url.clone()).send() {
                Ok(response) if response.status().is_server_error() && attempt < 2 => {
                    last_error = Some(ManagerError::Network(format!(
                        "Hub returned transient HTTP {}",
                        response.status()
                    )));
                }
                Ok(response) => match read_response(response, maximum) {
                    Ok(bytes) => return Ok(bytes),
                    Err(error @ ManagerError::Network(_)) if attempt < 2 => {
                        last_error = Some(error);
                    }
                    Err(error) => return Err(error),
                },
                Err(error) if attempt < 2 => {
                    last_error = Some(ManagerError::Network(error.to_string()));
                }
                Err(error) => return Err(ManagerError::Network(error.to_string())),
            }
            std::thread::sleep(Duration::from_millis(100 * (attempt + 1) as u64));
        }
        Err(last_error.unwrap_or_else(|| {
            ManagerError::Network("Hub request exhausted its retry budget".to_owned())
        }))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct RollbackState {
    schema_version: u32,
    sequence: u64,
    snapshot: String,
}

pub(crate) fn load(args: &HubArgs) -> Result<HubSnapshot, ManagerError> {
    let origin_value = args.hub.as_deref().ok_or_else(|| {
        ManagerError::Usage(
            "this operation requires --hub or the LIKNON_HUB_URL environment variable".to_owned(),
        )
    })?;
    let trust_path = args.trust_root.as_deref().ok_or_else(|| {
        ManagerError::Usage(
            "this operation requires --trust-root or LIKNON_HUB_TRUST_ROOT".to_owned(),
        )
    })?;
    let origin = HubOrigin::parse(origin_value)?;
    let origin_identity = origin.identity();
    let cache = workspace::cache_root(args.cache.as_deref())?;
    let client = HubClient::new(origin, cache.clone(), Duration::from_secs(args.timeout))?;
    let trust_root: TrustRoot = read_integrity_json_file(trust_path, "Hub trust root")?;
    validate_trust_root(&trust_root)?;

    let pointer_bytes = client.fetch_metadata("stable.json")?;
    let pointer_signature: DetachedSignature = parse_hub_json(
        "stable.json.sig.json",
        &client.fetch_metadata("stable.json.sig.json")?,
    )?;
    verify_detached(&pointer_bytes, &pointer_signature, &trust_root)?;
    let pointer: CatalogPointer = parse_hub_json("stable.json", &pointer_bytes)?;
    if pointer.schema_version != CATALOG_POINTER_SCHEMA_VERSION {
        return Err(ManagerError::Integrity(format!(
            "unsupported catalog pointer schema {}",
            pointer.schema_version
        )));
    }
    let now = unix_now()?;
    if pointer.snapshot.published_at_unix > now.saturating_add(300) {
        return Err(ManagerError::Integrity(
            "catalog publication time is implausibly in the future".to_owned(),
        ));
    }
    if pointer.snapshot.expires_at_unix < now {
        return Err(ManagerError::Integrity(format!(
            "catalog snapshot `{}` expired",
            pointer.snapshot.id
        )));
    }
    if pointer_signature.signed_at_unix != pointer.snapshot.published_at_unix {
        return Err(ManagerError::Integrity(
            "pointer signature time differs from catalog publication time".to_owned(),
        ));
    }
    enforce_rollback(&cache, &origin_identity, &pointer)?;

    validate_relative_path(&pointer.catalog_path)?;
    validate_relative_path(&pointer.catalog_signature_path)?;
    validate_sha256(&pointer.catalog_sha256)?;
    let catalog_bytes = client.fetch_metadata(&pointer.catalog_path)?;
    if catalog_bytes.len() as u64 != pointer.catalog_length
        || sha256_bytes(&catalog_bytes) != pointer.catalog_sha256
    {
        return Err(ManagerError::Integrity(
            "catalog object differs from its authenticated stable pointer".to_owned(),
        ));
    }
    let catalog_signature: DetachedSignature = parse_hub_json(
        &pointer.catalog_signature_path,
        &client.fetch_metadata(&pointer.catalog_signature_path)?,
    )?;
    verify_detached(&catalog_bytes, &catalog_signature, &trust_root)?;
    if catalog_signature.signed_at_unix != pointer.snapshot.published_at_unix {
        return Err(ManagerError::Integrity(
            "catalog signature time differs from catalog publication time".to_owned(),
        ));
    }
    let catalog: CatalogSnapshot = parse_hub_json(&pointer.catalog_path, &catalog_bytes)?;
    validate_catalog(&catalog, ValidationLimits::default())?;
    if catalog.snapshot != pointer.snapshot {
        return Err(ManagerError::Integrity(
            "catalog snapshot identity differs from the stable pointer".to_owned(),
        ));
    }
    record_rollback(&cache, &origin_identity, &pointer)?;
    cache_catalog(&cache, &origin_identity, &catalog, &catalog_bytes)?;

    Ok(HubSnapshot {
        catalog,
        origin: origin_identity,
        catalog_sha256: pointer.catalog_sha256,
        trust_authority: trust_root.authority.clone(),
        signing_key_id: catalog_signature.key_id.clone(),
        evidence: AuthenticatedCatalogEvidence {
            catalog_bytes,
            catalog_signature,
            trust_root,
        },
        client,
    })
}

fn redirect_action(attempt: Attempt<'_>, expected_origin: &str) -> Action {
    if attempt.previous().len() >= 3 {
        return attempt.error("too many Hub redirects");
    }
    if attempt.url().origin().ascii_serialization() != expected_origin {
        return attempt.error("Hub redirect changes origin");
    }
    attempt.follow()
}

fn read_response(mut response: Response, maximum: u64) -> Result<Vec<u8>, ManagerError> {
    if !response.status().is_success() {
        return Err(ManagerError::Network(format!(
            "Hub returned HTTP {}",
            response.status()
        )));
    }
    if response
        .content_length()
        .is_some_and(|length| length > maximum)
    {
        return Err(ManagerError::Integrity(format!(
            "Hub response exceeds the {maximum}-byte limit"
        )));
    }
    let mut bytes = Vec::new();
    response
        .by_ref()
        .take(maximum.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(|error| ManagerError::Network(error.to_string()))?;
    if bytes.len() as u64 > maximum {
        return Err(ManagerError::Integrity(format!(
            "Hub response exceeds the {maximum}-byte limit"
        )));
    }
    Ok(bytes)
}

fn read_local(root: &Path, relative: &str, maximum: u64) -> Result<Vec<u8>, ManagerError> {
    let supplied_metadata = fs::symlink_metadata(root).at(root)?;
    if supplied_metadata.file_type().is_symlink() || !supplied_metadata.is_dir() {
        return Err(ManagerError::Usage(format!(
            "local Hub `{}` must be a non-symlink directory",
            root.display()
        )));
    }
    let root = fs::canonicalize(root).at(root)?;
    let mut current = root.clone();
    for component in Path::new(relative).components() {
        current.push(component);
        let metadata = fs::symlink_metadata(&current).at(&current)?;
        if metadata.file_type().is_symlink() {
            return Err(ManagerError::Integrity(format!(
                "local Hub object `{relative}` traverses a symlink"
            )));
        }
    }
    let canonical = fs::canonicalize(&current).at(&current)?;
    if !canonical.starts_with(&root) || !canonical.is_file() {
        return Err(ManagerError::Integrity(format!(
            "local Hub object `{relative}` escapes its origin"
        )));
    }
    let metadata = fs::metadata(&canonical).at(&canonical)?;
    if metadata.len() > maximum {
        return Err(ManagerError::Integrity(format!(
            "local Hub object `{relative}` exceeds the {maximum}-byte limit"
        )));
    }
    fs::read(&canonical).at(&canonical)
}

fn with_trailing_slash(mut url: Url) -> Result<Url, ManagerError> {
    if url.query().is_some() || url.fragment().is_some() {
        return Err(ManagerError::Usage(
            "Hub origin cannot contain a query or fragment".to_owned(),
        ));
    }
    if !url.path().ends_with('/') {
        let path = format!("{}/", url.path());
        url.set_path(&path);
    }
    Ok(url)
}

fn unix_now() -> Result<u64, ManagerError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|error| {
            ManagerError::Internal(format!("system clock precedes Unix epoch: {error}"))
        })
}

fn read_integrity_json_file<T: serde::de::DeserializeOwned>(
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

fn parse_hub_json<T: serde::de::DeserializeOwned>(
    path: &str,
    bytes: &[u8],
) -> Result<T, ManagerError> {
    serde_json::from_slice(bytes).map_err(|error| {
        ManagerError::Integrity(format!("Hub metadata `{path}` is malformed JSON: {error}"))
    })
}

fn rollback_path(cache: &Path, origin: &str) -> PathBuf {
    cache
        .join("origins")
        .join(format!("{}.json", sha256_bytes(origin.as_bytes())))
}

fn enforce_rollback(
    cache: &Path,
    origin: &str,
    pointer: &CatalogPointer,
) -> Result<(), ManagerError> {
    let path = rollback_path(cache, origin);
    let Some(bytes) = read_cache_file(cache, &path)? else {
        return Ok(());
    };
    let state: RollbackState = serde_json::from_slice(&bytes).map_err(|error| {
        ManagerError::Integrity(format!(
            "Hub rollback state `{}` is malformed JSON: {error}",
            path.display()
        ))
    })?;
    if state.schema_version != 1 {
        return Err(ManagerError::Integrity(
            "unsupported local Hub rollback-state schema".to_owned(),
        ));
    }
    if pointer.snapshot.sequence < state.sequence {
        return Err(ManagerError::Integrity(format!(
            "Hub snapshot sequence {} rolls back previously observed sequence {}",
            pointer.snapshot.sequence, state.sequence
        )));
    }
    if pointer.snapshot.sequence == state.sequence && pointer.snapshot.id != state.snapshot {
        return Err(ManagerError::Integrity(
            "Hub reused a snapshot sequence for different metadata".to_owned(),
        ));
    }
    Ok(())
}

fn record_rollback(
    cache: &Path,
    origin: &str,
    pointer: &CatalogPointer,
) -> Result<(), ManagerError> {
    let state = RollbackState {
        schema_version: 1,
        sequence: pointer.snapshot.sequence,
        snapshot: pointer.snapshot.id.clone(),
    };
    atomic_cache_write(cache, &rollback_path(cache, origin), &json_bytes(&state)?)
}

fn cache_catalog(
    cache: &Path,
    origin: &str,
    catalog: &CatalogSnapshot,
    bytes: &[u8],
) -> Result<(), ManagerError> {
    let path = cache
        .join("catalogs")
        .join(sha256_bytes(origin.as_bytes()))
        .join(format!("{}.json", catalog.snapshot.id));
    atomic_cache_write(cache, &path, bytes)
}

fn json_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, ManagerError> {
    let mut bytes = serde_json::to_vec_pretty(value)
        .map_err(|error| ManagerError::Internal(error.to_string()))?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn secure_cache_root(path: &Path) -> Result<PathBuf, ManagerError> {
    fs::create_dir_all(path).at(path)?;
    let metadata = fs::symlink_metadata(path).at(path)?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(ManagerError::Integrity(format!(
            "cache root `{}` must be a non-symlink directory",
            path.display()
        )));
    }
    fs::canonicalize(path).at(path)
}

fn read_cache_file(root: &Path, path: &Path) -> Result<Option<Vec<u8>>, ManagerError> {
    validate_cache_path(root, path, false)?;
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() || !metadata.is_file() {
                return Err(ManagerError::Integrity(format!(
                    "cache object `{}` is not a regular file",
                    path.display()
                )));
            }
            Ok(Some(fs::read(path).at(path)?))
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(source) => Err(ManagerError::Io {
            path: path.to_path_buf(),
            source,
        }),
    }
}

fn atomic_cache_write(root: &Path, path: &Path, bytes: &[u8]) -> Result<(), ManagerError> {
    validate_cache_path(root, path, true)?;
    if let Some(current) = read_cache_file(root, path)? {
        if current == bytes {
            return Ok(());
        }
    }
    let temporary = cache_temporary_path(path)?;
    let write_result = (|| -> Result<(), ManagerError> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .at(&temporary)?;
        file.write_all(bytes).at(&temporary)?;
        file.sync_all().at(&temporary)?;
        Ok(())
    })();
    if let Err(error) = write_result {
        let _ = fs::remove_file(&temporary);
        return Err(error);
    }
    if fs::symlink_metadata(path).is_ok() {
        fs::remove_file(path).at(path)?;
    }
    fs::rename(&temporary, path).at(path)?;
    Ok(())
}

fn validate_cache_path(root: &Path, path: &Path, create_parents: bool) -> Result<(), ManagerError> {
    let relative = path.strip_prefix(root).map_err(|_| {
        ManagerError::Integrity(format!(
            "cache object `{}` escapes its root",
            path.display()
        ))
    })?;
    let parent = relative
        .parent()
        .ok_or_else(|| ManagerError::Integrity("cache object has no relative parent".to_owned()))?;
    let mut current = root.to_path_buf();
    for component in parent.components() {
        current.push(component);
        match fs::symlink_metadata(&current) {
            Ok(metadata) => {
                if metadata.file_type().is_symlink() || !metadata.is_dir() {
                    return Err(ManagerError::Integrity(format!(
                        "cache path component `{}` is unsafe",
                        current.display()
                    )));
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound && create_parents => {
                match fs::create_dir(&current) {
                    Ok(()) => {}
                    Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                        let metadata = fs::symlink_metadata(&current).at(&current)?;
                        if metadata.file_type().is_symlink() || !metadata.is_dir() {
                            return Err(ManagerError::Integrity(format!(
                                "cache path component `{}` is unsafe",
                                current.display()
                            )));
                        }
                    }
                    Err(source) => {
                        return Err(ManagerError::Io {
                            path: current,
                            source,
                        });
                    }
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(source) => {
                return Err(ManagerError::Io {
                    path: current,
                    source,
                });
            }
        }
    }
    Ok(())
}

fn cache_temporary_path(path: &Path) -> Result<PathBuf, ManagerError> {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| ManagerError::Integrity("cache object has no portable name".to_owned()))?;
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| ManagerError::Internal(error.to_string()))?
        .as_nanos();
    Ok(path.with_file_name(format!(".{name}.tmp-{}-{nonce}", std::process::id())))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use reqwest::{redirect::Policy, Url};
    use tempfile::TempDir;

    use super::{
        atomic_cache_write, read_local, redirect_action, secure_cache_root, with_trailing_slash,
    };

    #[test]
    fn normalizes_https_origin_path() {
        let url = with_trailing_slash(Url::parse("https://example.test/hub").unwrap()).unwrap();
        assert_eq!(url.as_str(), "https://example.test/hub/");
        let _ = Policy::custom(move |attempt| redirect_action(attempt, "https://example.test"));
    }

    #[cfg(unix)]
    #[test]
    fn rejects_symlinks_inside_the_untrusted_cache() {
        use std::os::unix::fs::symlink;

        let temporary = TempDir::new().unwrap();
        let cache = temporary.path().join("cache");
        let outside = temporary.path().join("outside");
        fs::create_dir(&cache).unwrap();
        fs::create_dir(&outside).unwrap();
        symlink(&outside, cache.join("objects")).unwrap();
        let cache = secure_cache_root(&cache).unwrap();
        let destination = cache.join("objects/sha256/aa/payload");

        let error = atomic_cache_write(&cache, &destination, b"payload").unwrap_err();

        assert!(error.to_string().contains("unsafe"));
        assert!(fs::read_dir(outside).unwrap().next().is_none());
    }

    #[cfg(unix)]
    #[test]
    fn rejects_a_symlinked_local_hub_root() {
        use std::os::unix::fs::symlink;

        let temporary = TempDir::new().unwrap();
        let target = temporary.path().join("target");
        let origin = temporary.path().join("origin");
        fs::create_dir(&target).unwrap();
        fs::write(target.join("stable.json"), b"{}\n").unwrap();
        symlink(&target, &origin).unwrap();

        assert!(read_local(&origin, "stable.json", 1024).is_err());
    }
}
