//! Workspace discovery and read-only resource-store boundary checks.

use std::{env, fs, io::ErrorKind, path::Path};
use std::{path::Component, path::PathBuf};

use liknon_resource_contracts::RESOURCE_ROOT_RELATIVE;

use crate::error::{IoResultExt as _, ToolError};

#[derive(Debug, Clone)]
/// Canonical paths selected for one Tool invocation.
pub(crate) struct Workspace {
    /// Canonical consumer workspace boundary.
    pub(crate) root: PathBuf,
    /// Canonical resource-store boundary.
    pub(crate) resources: PathBuf,
}

impl Workspace {
    /// Returns the workspace boundary in a human-readable form.
    pub(crate) fn display(&self) -> String {
        self.root.display().to_string()
    }
}

/// Discovers the nearest resource workspace or validates an explicit boundary.
pub(crate) fn discover(explicit: Option<&Path>) -> Result<Workspace, ToolError> {
    let start =
        canonical_directory(explicit.unwrap_or(&env::current_dir().at("current directory")?))?;
    if explicit.is_some() {
        return require_store(start);
    }

    let mut candidate = start.as_path();
    loop {
        if candidate.join(RESOURCE_ROOT_RELATIVE).exists() {
            return require_store(candidate.to_path_buf());
        }
        if candidate.join(".liknon").exists() {
            return Err(ToolError::PartialWorkspace(candidate.to_path_buf()));
        }
        candidate = candidate
            .parent()
            .ok_or_else(|| ToolError::WorkspaceNotFound(start.clone()))?;
    }
}

/// Verifies that every existing component stays below a canonical boundary.
pub(crate) fn validate_existing_path(boundary: &Path, path: &Path) -> Result<(), ToolError> {
    let canonical_boundary = fs::canonicalize(boundary).at(boundary)?;
    let relative = path.strip_prefix(boundary).map_err(|_| {
        ToolError::Integrity(format!(
            "path `{}` escapes `{}`",
            path.display(),
            boundary.display()
        ))
    })?;
    let mut current = canonical_boundary.clone();
    for component in relative.components() {
        if !matches!(component, Component::Normal(_)) {
            return Err(ToolError::Integrity(format!(
                "path `{}` contains a non-normal component",
                path.display()
            )));
        }
        current.push(component);
        let metadata = fs::symlink_metadata(&current).at(&current)?;
        if metadata.file_type().is_symlink() {
            return Err(ToolError::Integrity(format!(
                "path component `{}` is a symlink",
                current.display()
            )));
        }
    }
    let canonical = fs::canonicalize(path).at(path)?;
    if !canonical.starts_with(&canonical_boundary) {
        return Err(ToolError::Integrity(format!(
            "path `{}` escapes the resource-store boundary",
            path.display()
        )));
    }
    Ok(())
}

fn require_store(root: PathBuf) -> Result<Workspace, ToolError> {
    let marker = root.join(".liknon");
    let marker_metadata = match fs::symlink_metadata(&marker) {
        Ok(metadata) => metadata,
        Err(source) if source.kind() == ErrorKind::NotFound => {
            return Err(ToolError::WorkspaceNotFound(root));
        }
        Err(source) => {
            return Err(ToolError::Io {
                path: marker,
                source,
            })
        }
    };
    if marker_metadata.file_type().is_symlink() || !marker_metadata.is_dir() {
        return Err(ToolError::Integrity(format!(
            "workspace marker `{}` must be a non-symlink directory",
            marker.display()
        )));
    }

    let resources = root.join(RESOURCE_ROOT_RELATIVE);
    let metadata = match fs::symlink_metadata(&resources) {
        Ok(metadata) => metadata,
        Err(source) if source.kind() == ErrorKind::NotFound => {
            return Err(ToolError::PartialWorkspace(root));
        }
        Err(source) => {
            return Err(ToolError::Io {
                path: resources,
                source,
            });
        }
    };
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(ToolError::Integrity(format!(
            "resource root `{}` must be a non-symlink directory",
            resources.display()
        )));
    }
    validate_existing_path(&root, &resources)?;
    Ok(Workspace { root, resources })
}

fn canonical_directory(path: &Path) -> Result<PathBuf, ToolError> {
    let canonical = fs::canonicalize(path).at(path)?;
    if !canonical.is_dir() {
        return Err(ToolError::Usage(format!(
            "workspace `{}` is not a directory",
            path.display()
        )));
    }
    Ok(canonical)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::discover;

    #[test]
    fn explicit_workspace_requires_a_complete_resource_store() {
        let temporary = TempDir::new().unwrap();
        fs::create_dir(temporary.path().join(".liknon")).unwrap();
        assert!(discover(Some(temporary.path())).is_err());
        fs::create_dir_all(temporary.path().join(".liknon/resources")).unwrap();
        assert_eq!(
            discover(Some(temporary.path())).unwrap().root,
            fs::canonicalize(temporary.path()).unwrap()
        );
    }
}
