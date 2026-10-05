//! Deterministic provisioning of consumer-owned validator resources.

use crate::{
    config,
    contracts::init::{
        InitResourceKind, InitResourceResult, InitResourceStatus, InitResult, InitStatus,
        INIT_RESULT_SCHEMA_VERSION,
    },
};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Component, Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

const DIAGNOSTIC_LIMIT: usize = 2048;
static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// CLI exit-code category associated with a typed initialization result.
pub(crate) enum InitFailureClass {
    /// No failure occurred, including an explicit destination conflict.
    None,
    /// Candidate input or a provisioning precondition was rejected.
    Rejected,
    /// Filesystem publication or result construction could not complete.
    Internal,
}

#[derive(Clone, Copy, Debug)]
enum PreparedDirectory {
    Existing,
    Created,
}

#[derive(Debug)]
enum DirectoryPreparationError {
    Conflict(String),
    Internal(String),
}

/// Provisions a complete candidate as `.validation/config.json`.
///
/// The workspace must already be canonical. Candidate bytes are validated as
/// if they were at the destination and are published atomically without ever
/// replacing an existing destination.
pub fn provision_config(workspace: &Path, candidate: &Path) -> InitResult {
    provision_config_classified(workspace, candidate).0
}

pub(crate) fn provision_config_classified(
    workspace: &Path,
    candidate: &Path,
) -> (InitResult, InitFailureClass) {
    let destination = workspace.join(".validation/config.json");
    let mut resource = InitResourceResult {
        kind: InitResourceKind::Config,
        path: portable_absolute(&destination),
        status: InitResourceStatus::Failed,
        digest: None,
        diagnostics: Vec::new(),
    };

    let bytes = match read_candidate(workspace, candidate) {
        Ok(bytes) => bytes,
        Err(error) => {
            resource.diagnostics.push(bounded(error));
            return (result(workspace, resource), InitFailureClass::Rejected);
        }
    };
    resource.digest = Some(format!("{:x}", Sha256::digest(&bytes)));

    let parsed = match config::parse_at(&bytes, &destination) {
        Ok(parsed) => parsed,
        Err(error) => {
            resource.diagnostics.push(bounded(error.to_string()));
            return (result(workspace, resource), InitFailureClass::Rejected);
        }
    };

    let parent = destination.parent().expect("fixed destination has parent");
    let prepared = match prepare_destination_directory(parent) {
        Ok(prepared) => prepared,
        Err(DirectoryPreparationError::Conflict(error)) => {
            resource.status = InitResourceStatus::Conflict;
            resource.diagnostics.push(bounded(error));
            return (result(workspace, resource), InitFailureClass::None);
        }
        Err(DirectoryPreparationError::Internal(error)) => {
            resource.diagnostics.push(bounded(error));
            return (result(workspace, resource), InitFailureClass::Internal);
        }
    };

    if let Err(error) = config::validate_parsed(parsed) {
        resource.diagnostics.push(bounded(error.to_string()));
        cleanup_created_directory(parent, prepared);
        return (result(workspace, resource), InitFailureClass::Rejected);
    }

    let failure_class = match inspect_destination(&destination, &bytes) {
        Ok(Some(status)) => {
            resource.status = status;
            if status == InitResourceStatus::Conflict {
                resource.diagnostics.push(
                    "destination already exists with different bytes or an unsafe file type".into(),
                );
            }
            InitFailureClass::None
        }
        Ok(None) => match publish(&destination, &bytes) {
            Ok(status) => {
                resource.status = status;
                InitFailureClass::None
            }
            Err(error) => {
                resource.diagnostics.push(bounded(error));
                InitFailureClass::Internal
            }
        },
        Err(error) => {
            resource.diagnostics.push(bounded(error));
            InitFailureClass::Internal
        }
    };
    if !matches!(
        resource.status,
        InitResourceStatus::Created | InitResourceStatus::Reused
    ) {
        cleanup_created_directory(parent, prepared);
    }
    (result(workspace, resource), failure_class)
}

fn prepare_destination_directory(
    directory: &Path,
) -> Result<PreparedDirectory, DirectoryPreparationError> {
    match fs::symlink_metadata(directory) {
        Ok(metadata) => inspect_destination_directory(metadata),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            match fs::create_dir(directory) {
                Ok(()) => Ok(PreparedDirectory::Created),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                    let metadata = fs::symlink_metadata(directory).map_err(|error| {
                        DirectoryPreparationError::Internal(format!(
                            "cannot inspect concurrently created destination directory: {error}"
                        ))
                    })?;
                    inspect_destination_directory(metadata)
                }
                Err(error) => Err(DirectoryPreparationError::Internal(format!(
                    "cannot create destination directory: {error}"
                ))),
            }
        }
        Err(error) => Err(DirectoryPreparationError::Internal(format!(
            "cannot inspect destination directory: {error}"
        ))),
    }
}

fn inspect_destination_directory(
    metadata: fs::Metadata,
) -> Result<PreparedDirectory, DirectoryPreparationError> {
    if metadata.file_type().is_dir() {
        Ok(PreparedDirectory::Existing)
    } else {
        Err(DirectoryPreparationError::Conflict(
            "destination directory exists with an unsafe file type".into(),
        ))
    }
}

fn cleanup_created_directory(directory: &Path, prepared: PreparedDirectory) {
    if matches!(prepared, PreparedDirectory::Created) {
        let _ = fs::remove_dir(directory);
    }
}

fn read_candidate(workspace: &Path, candidate: &Path) -> Result<Vec<u8>, String> {
    if candidate.as_os_str().is_empty()
        || candidate
            .components()
            .any(|component| matches!(component, Component::ParentDir))
    {
        return Err("config candidate path must not be empty or contain parent traversal".into());
    }
    let metadata = fs::symlink_metadata(candidate)
        .map_err(|error| format!("cannot inspect config candidate: {error}"))?;
    if !metadata.file_type().is_file() {
        return Err("config candidate must be a regular file and not a symlink".into());
    }
    let canonical = candidate
        .canonicalize()
        .map_err(|error| format!("cannot resolve config candidate: {error}"))?;
    let presented = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|error| format!("cannot resolve current directory: {error}"))?
            .join(candidate)
    };
    if normalize_candidate_path(&presented) != canonical {
        return Err("config candidate path must not contain symlinks".into());
    }
    if !canonical.starts_with(workspace) {
        return Err("config candidate escapes the selected workspace".into());
    }
    fs::read(&canonical).map_err(|error| format!("cannot read config candidate: {error}"))
}

fn normalize_candidate_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        if !matches!(component, Component::CurDir) {
            normalized.push(component.as_os_str());
        }
    }
    normalized
}

fn inspect_destination(
    destination: &Path,
    bytes: &[u8],
) -> Result<Option<InitResourceStatus>, String> {
    match fs::symlink_metadata(destination) {
        Ok(metadata) if !metadata.file_type().is_file() => Ok(Some(InitResourceStatus::Conflict)),
        Ok(_) => fs::read(destination)
            .map(|existing| {
                Some(if existing == bytes {
                    InitResourceStatus::Reused
                } else {
                    InitResourceStatus::Conflict
                })
            })
            .map_err(|error| format!("cannot read existing destination: {error}")),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(format!("cannot inspect destination: {error}")),
    }
}

fn publish(destination: &Path, bytes: &[u8]) -> Result<InitResourceStatus, String> {
    publish_with(destination, bytes, |_| Ok(()))
}

fn publish_with(
    destination: &Path,
    bytes: &[u8],
    before_publication: impl FnOnce(&Path) -> Result<(), String>,
) -> Result<InitResourceStatus, String> {
    publish_with_allocator(
        destination,
        bytes,
        || TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed),
        before_publication,
    )
}

fn publish_with_allocator(
    destination: &Path,
    bytes: &[u8],
    mut next_sequence: impl FnMut() -> u64,
    before_publication: impl FnOnce(&Path) -> Result<(), String>,
) -> Result<InitResourceStatus, String> {
    let parent = destination.parent().expect("fixed destination has parent");
    let (temporary, mut file) = loop {
        let temporary = temporary_path(parent, next_sequence());
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
        {
            Ok(file) => break (temporary, file),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => {
                return Err(format!("cannot create atomic temporary file: {error}"));
            }
        }
    };
    let write_result = file
        .write_all(bytes)
        .and_then(|()| file.sync_all())
        .map_err(|error| format!("cannot write atomic temporary file: {error}"));
    drop(file);
    if let Err(error) = write_result {
        let _ = fs::remove_file(&temporary);
        return Err(error);
    }

    if let Err(error) = before_publication(&temporary) {
        let _ = fs::remove_file(&temporary);
        return Err(error);
    }

    let published = match fs::hard_link(&temporary, destination) {
        Ok(()) => Ok(InitResourceStatus::Created),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            inspect_destination(destination, bytes)?.ok_or_else(|| {
                "destination disappeared while resolving an atomic write conflict".into()
            })
        }
        Err(error) => Err(format!("cannot publish config atomically: {error}")),
    };
    let _ = fs::remove_file(&temporary);
    published
}

fn temporary_path(parent: &Path, sequence: u64) -> PathBuf {
    parent.join(format!(
        ".config.json.tmp-{}-{sequence}",
        std::process::id()
    ))
}

fn result(workspace: &Path, resource: InitResourceResult) -> InitResult {
    let status = match resource.status {
        InitResourceStatus::Created | InitResourceStatus::Reused => InitStatus::Success,
        InitResourceStatus::Conflict => InitStatus::Conflict,
        InitResourceStatus::Failed | InitResourceStatus::NotWritten => InitStatus::Failed,
    };
    InitResult {
        schema_version: INIT_RESULT_SCHEMA_VERSION,
        workspace_root: portable_absolute(workspace),
        status,
        resources: vec![resource],
    }
}

fn bounded(mut diagnostic: String) -> String {
    if diagnostic.len() <= DIAGNOSTIC_LIMIT {
        return diagnostic;
    }
    let mut boundary = DIAGNOSTIC_LIMIT;
    while !diagnostic.is_char_boundary(boundary) {
        boundary -= 1;
    }
    diagnostic.truncate(boundary);
    diagnostic.push('…');
    diagnostic
}

fn portable_absolute(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

/// Resolves the selected workspace exactly once without parent discovery.
pub fn resolve_workspace(path: PathBuf) -> Result<PathBuf, String> {
    let workspace = path
        .canonicalize()
        .map_err(|error| format!("cannot resolve workspace {}: {error}", path.display()))?;
    if !workspace.is_dir() {
        return Err(format!(
            "workspace {} is not a directory",
            workspace.display()
        ));
    }
    Ok(workspace)
}

#[cfg(test)]
mod tests {
    use super::{
        prepare_destination_directory, provision_config, publish, publish_with,
        publish_with_allocator, temporary_path, InitResourceStatus,
    };
    use crate::{config, contracts::init::InitStatus};
    use serde_json::json;
    #[cfg(unix)]
    use std::os::unix::fs::symlink;
    use std::{
        fs,
        path::{Path, PathBuf},
    };
    use tempfile::TempDir;

    fn document(workspace_root: &str) -> serde_json::Value {
        json!({
            "schemaVersion": 6,
            "workspaceRoot": workspace_root,
            "defaultGroup": "all",
            "outputLimitBytes": 4096,
            "tools": [{"id":"tool","program":"tool","requiresTools":[],"versionArgs":["--version"],"versionParser":"firstSemver"}],
            "checks": [{"id":"check","label":"Check","description":"Check.","toolId":"tool","args":[],"requiresTools":[],"timeoutSeconds":1}],
            "suites": [{"id":"suite","label":"Suite","description":"Suite.","checks":[{"checkId":"check","dependsOn":[]}]}],
            "groups": [{"id":"all","label":"All","description":"All.","members":[{"kind":"suite","id":"suite"}]}]
        })
    }

    fn write_candidate(root: &Path, value: &serde_json::Value) -> std::path::PathBuf {
        let candidate = root.join("candidate.json");
        fs::write(&candidate, serde_json::to_vec(value).unwrap()).unwrap();
        candidate
    }

    fn canonical_workspace(workspace: &TempDir) -> PathBuf {
        workspace.path().canonicalize().unwrap()
    }

    #[test]
    fn malformed_candidates_do_not_prepare_the_canonical_directory() {
        for bytes in [
            b"{".to_vec(),
            serde_json::to_vec(&{
                let mut value = document("..");
                value["unknown"] = true.into();
                value
            })
            .unwrap(),
        ] {
            let workspace = TempDir::new().unwrap();
            let workspace_root = canonical_workspace(&workspace);
            let candidate = workspace_root.join("candidate.json");
            fs::write(&candidate, bytes).unwrap();

            let result = provision_config(&workspace_root, &candidate);

            assert_eq!(result.status, InitStatus::Failed);
            assert!(!workspace_root.join(".validation").exists());
        }
    }

    #[test]
    fn safe_canonical_directory_states_publish_without_touching_unrelated_entries() {
        for state in ["absent", "empty", "unrelated"] {
            let workspace = TempDir::new().unwrap();
            let workspace_root = canonical_workspace(&workspace);
            let validation = workspace_root.join(".validation");
            if state != "absent" {
                fs::create_dir(&validation).unwrap();
            }
            if state == "unrelated" {
                fs::write(validation.join("keep.txt"), b"keep").unwrap();
            }
            let candidate = write_candidate(&workspace_root, &document(".."));

            let result = provision_config(&workspace_root, &candidate);

            assert_eq!(result.status, InitStatus::Success, "{state}");
            assert!(validation.join("config.json").is_file());
            if state == "unrelated" {
                assert_eq!(fs::read(validation.join("keep.txt")).unwrap(), b"keep");
            }
        }
    }

    #[test]
    fn failed_validation_never_publishes_and_only_leaves_a_safe_empty_directory() {
        for value in [document("missing"), {
            let mut value = document("..");
            value["defaultGroup"] = "missing".into();
            value
        }] {
            let workspace = TempDir::new().unwrap();
            let workspace_root = canonical_workspace(&workspace);
            let candidate = write_candidate(&workspace_root, &value);
            let validation = workspace_root.join(".validation");

            let result = provision_config(&workspace_root, &candidate);

            assert_eq!(result.status, InitStatus::Failed);
            assert!(!validation.join("config.json").exists());
            if validation.exists() {
                assert!(fs::symlink_metadata(&validation)
                    .unwrap()
                    .file_type()
                    .is_dir());
                assert!(fs::read_dir(&validation).unwrap().next().is_none());
            }
        }
    }

    #[test]
    fn canonical_directory_forms_use_the_same_ordinary_loading_semantics() {
        for declared in [".", "", "././", "../.validation"] {
            let workspace = TempDir::new().unwrap();
            let workspace_root = canonical_workspace(&workspace);
            let candidate = write_candidate(&workspace_root, &document(declared));
            let destination = workspace_root.join(".validation/config.json");
            prepare_destination_directory(destination.parent().unwrap()).unwrap();
            let before = config::validate_parsed(
                config::parse_at(&fs::read(&candidate).unwrap(), &destination).unwrap(),
            )
            .unwrap();

            let result = provision_config(&workspace_root, &candidate);
            assert_eq!(result.status, InitStatus::Success, "{declared:?}");
            let installed = config::load(None, &workspace_root).unwrap();
            assert_eq!(before.workspace_root, installed.workspace_root);
            assert_eq!(
                before.suite_directories["suite"].absolute,
                installed.suite_directories["suite"].absolute
            );
            assert_eq!(installed.workspace_root, workspace_root.join(".validation"));
            assert_eq!(
                installed.suite_directories["suite"].absolute,
                installed.workspace_root
            );
        }
    }

    #[test]
    fn absolute_workspace_root_uses_ordinary_loading_semantics() {
        let workspace = TempDir::new().unwrap();
        let external = TempDir::new().unwrap();
        let workspace_root = canonical_workspace(&workspace);
        let external_root = canonical_workspace(&external);
        let declared = external_root.to_string_lossy();
        let candidate = write_candidate(&workspace_root, &document(&declared));

        let result = provision_config(&workspace_root, &candidate);

        assert_eq!(result.status, InitStatus::Success);
        let installed = config::load(None, &workspace_root).unwrap();
        assert_eq!(installed.workspace_root, external_root);
    }

    #[cfg(unix)]
    #[test]
    fn local_and_external_symlinks_use_ordinary_loading_semantics() {
        for mode in ["local", "parent", "external"] {
            let workspace = TempDir::new().unwrap();
            let external = TempDir::new().unwrap();
            let workspace_root = canonical_workspace(&workspace);
            let external_root = canonical_workspace(&external);
            let validation = workspace_root.join(".validation");
            let (declared, expected) = match mode {
                "local" => {
                    symlink(".validation", workspace_root.join("future-link")).unwrap();
                    ("../future-link".into(), validation.clone())
                }
                "parent" => {
                    symlink(".validation", workspace_root.join("future-link")).unwrap();
                    ("../future-link/..".into(), workspace_root.clone())
                }
                "external" => {
                    let link = external_root.join("absolute-link");
                    symlink(&validation, &link).unwrap();
                    (link.to_string_lossy().into_owned(), validation.clone())
                }
                _ => unreachable!(),
            };
            let candidate = write_candidate(&workspace_root, &document(&declared));
            let destination = validation.join("config.json");
            prepare_destination_directory(&validation).unwrap();
            let before = config::validate_parsed(
                config::parse_at(&fs::read(&candidate).unwrap(), &destination).unwrap(),
            )
            .unwrap();

            let result = provision_config(&workspace_root, &candidate);
            assert_eq!(result.status, InitStatus::Success, "{mode}");
            let installed = config::load(None, &workspace_root).unwrap();
            assert_eq!(before.workspace_root, installed.workspace_root, "{mode}");
            assert_eq!(
                before.suite_directories["suite"].absolute,
                installed.suite_directories["suite"].absolute,
                "{mode}"
            );
            assert_eq!(installed.workspace_root, expected, "{mode}");
        }
    }

    #[cfg(unix)]
    #[test]
    fn ordinary_canonicalization_rejects_dangling_missing_and_excessive_links() {
        for mode in ["dangling", "descendant", "excessive"] {
            let workspace = TempDir::new().unwrap();
            let workspace_root = canonical_workspace(&workspace);
            let declared = match mode {
                "dangling" => {
                    symlink("missing", workspace_root.join("entry")).unwrap();
                    "../entry".to_string()
                }
                "descendant" => ".validation/missing".to_string(),
                "excessive" => {
                    for index in 0..64 {
                        let target = if index == 63 {
                            ".validation".to_string()
                        } else {
                            format!("link{}", index + 1)
                        };
                        symlink(target, workspace_root.join(format!("link{index}"))).unwrap();
                    }
                    "../link0".to_string()
                }
                _ => unreachable!(),
            };
            let candidate = write_candidate(&workspace_root, &document(&declared));
            let validation = workspace_root.join(".validation");

            let result = provision_config(&workspace_root, &candidate);

            assert_eq!(result.status, InitStatus::Failed, "{mode}");
            assert!(!validation.join("config.json").exists(), "{mode}");
        }
    }

    #[test]
    fn unsafe_canonical_file_is_a_conflict_and_is_never_replaced() {
        let workspace = TempDir::new().unwrap();
        let workspace_root = canonical_workspace(&workspace);
        let validation = workspace_root.join(".validation");
        fs::write(&validation, b"unsafe").unwrap();
        let candidate = write_candidate(&workspace_root, &document(".."));

        let result = provision_config(&workspace_root, &candidate);

        assert_eq!(result.status, InitStatus::Conflict);
        assert_eq!(fs::read(validation).unwrap(), b"unsafe");
    }

    #[test]
    fn interrupted_publication_exposes_no_partial_bytes_and_allows_retry() {
        let temp = TempDir::new().unwrap();
        let destination = temp.path().join(".validation/config.json");
        fs::create_dir(destination.parent().unwrap()).unwrap();
        let interrupted = publish_with(&destination, b"complete bytes", |_| {
            Err("initialization interrupted before publication".into())
        });
        assert!(interrupted.is_err());
        assert!(!destination.exists());
        assert!(fs::read_dir(destination.parent().unwrap())
            .unwrap()
            .next()
            .is_none());

        assert_eq!(
            publish(&destination, b"complete bytes").unwrap(),
            InitResourceStatus::Created
        );
        assert_eq!(fs::read(&destination).unwrap(), b"complete bytes");
    }

    #[test]
    fn interrupted_publication_never_overwrites_an_existing_destination() {
        let temp = TempDir::new().unwrap();
        let parent = temp.path().join(".validation");
        fs::create_dir(&parent).unwrap();
        let destination = parent.join("config.json");
        fs::write(&destination, b"existing").unwrap();

        let status = publish_with(&destination, b"replacement", |_| {
            Err("initialization interrupted before publication".into())
        })
        .unwrap_err();
        assert!(status.contains("interrupted"));
        assert_eq!(fs::read(&destination).unwrap(), b"existing");
        assert_eq!(fs::read_dir(parent).unwrap().count(), 1);
    }

    #[test]
    fn abandoned_temporary_is_untouched_and_does_not_block_publication() {
        let temp = TempDir::new().unwrap();
        let parent = temp.path().join(".validation");
        fs::create_dir(&parent).unwrap();
        let destination = parent.join("config.json");
        let first_sequence = 41;
        let abandoned = temporary_path(&parent, first_sequence);
        fs::write(&abandoned, b"unknown owner").unwrap();
        let mut sequence = first_sequence;

        let status = publish_with_allocator(
            &destination,
            b"validated bytes",
            || {
                let current = sequence;
                sequence += 1;
                current
            },
            |_| Ok(()),
        )
        .unwrap();

        assert_eq!(status, InitResourceStatus::Created);
        assert_eq!(fs::read(&destination).unwrap(), b"validated bytes");
        assert_eq!(fs::read(&abandoned).unwrap(), b"unknown owner");
        assert_eq!(fs::read_dir(parent).unwrap().count(), 2);
    }
}
