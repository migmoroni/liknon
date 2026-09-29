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

    if let Some(parent) = destination.parent() {
        match fs::symlink_metadata(parent) {
            Ok(metadata) if !metadata.file_type().is_dir() => {
                resource.status = InitResourceStatus::Conflict;
                resource
                    .diagnostics
                    .push("destination directory exists with an unsafe file type".into());
                return (result(workspace, resource), InitFailureClass::None);
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                resource.diagnostics.push(bounded(format!(
                    "cannot inspect destination directory: {error}"
                )));
                return (result(workspace, resource), InitFailureClass::Internal);
            }
        }
    }

    if let Err(error) = config::validate_at(&bytes, &destination, workspace) {
        resource.diagnostics.push(bounded(error.to_string()));
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
    (result(workspace, resource), failure_class)
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
    fs::create_dir_all(parent)
        .map_err(|error| format!("cannot create destination directory: {error}"))?;
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
        publish, publish_with, publish_with_allocator, temporary_path, InitResourceStatus,
    };
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn interrupted_publication_exposes_no_partial_bytes_and_allows_retry() {
        let temp = TempDir::new().unwrap();
        let destination = temp.path().join(".validation/config.json");
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
