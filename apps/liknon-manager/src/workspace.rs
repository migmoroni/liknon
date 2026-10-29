//! Workspace discovery and path-containment enforcement.

use std::{
    env, fs,
    path::{Path, PathBuf},
};

use liknon_resource_contracts::RESOURCE_ROOT_RELATIVE;

use crate::error::{IoResultExt as _, ManagerError};

#[derive(Debug, Clone)]
/// Canonical paths owned by one initialized Liknon consumer workspace.
pub(crate) struct Workspace {
    /// Canonical consumer workspace boundary.
    pub(crate) root: PathBuf,
    /// Workspace-local Liknon control directory.
    pub(crate) liknon_root: PathBuf,
    /// Managed resource store below the control directory.
    pub(crate) resources: PathBuf,
    /// Private transaction and locking state.
    pub(crate) manager_state: PathBuf,
}

impl Workspace {
    fn from_root(root: PathBuf) -> Self {
        let liknon_root = root.join(".liknon");
        Self {
            resources: root.join(RESOURCE_ROOT_RELATIVE),
            manager_state: liknon_root.join(".manager"),
            root,
            liknon_root,
        }
    }

    /// Returns the workspace boundary in a human-readable form.
    pub(crate) fn display(&self) -> String {
        self.root.display().to_string()
    }
}

/// Discovers the nearest initialized workspace or validates an explicit boundary.
pub(crate) fn discover(explicit: Option<&Path>) -> Result<Workspace, ManagerError> {
    let start =
        canonical_directory(explicit.unwrap_or(&env::current_dir().at("current directory")?))?;
    if explicit.is_some() {
        return require_existing_root(start);
    }
    let mut candidate = start.as_path();
    loop {
        if candidate.join(".liknon").is_dir() {
            return require_existing_root(candidate.to_path_buf());
        }
        candidate = candidate
            .parent()
            .ok_or_else(|| ManagerError::WorkspaceNotFound(start.clone()))?;
    }
}

/// Selects the boundary used by initialization, including an uninitialized root.
pub(crate) fn for_init(explicit: Option<&Path>) -> Result<Workspace, ManagerError> {
    let start =
        canonical_directory(explicit.unwrap_or(&env::current_dir().at("current directory")?))?;
    if explicit.is_some() {
        return Ok(Workspace::from_root(start));
    }
    let mut candidate = Some(start.as_path());
    while let Some(path) = candidate {
        if path.join(".liknon").is_dir() {
            return require_existing_root(path.to_path_buf());
        }
        candidate = path.parent();
    }
    Ok(Workspace::from_root(start))
}

/// Resolves the explicit or platform-appropriate Manager cache directory.
pub(crate) fn cache_root(explicit: Option<&Path>) -> Result<PathBuf, ManagerError> {
    if let Some(path) = explicit {
        return absolute_or_current(path);
    }
    if let Some(value) = env::var_os("XDG_CACHE_HOME") {
        return Ok(PathBuf::from(value).join("liknon/manager"));
    }
    if cfg!(windows) {
        if let Some(value) = env::var_os("LOCALAPPDATA") {
            return Ok(PathBuf::from(value).join("Liknon/Manager/Cache"));
        }
    }
    if let Some(value) = env::var_os("HOME").or_else(|| env::var_os("USERPROFILE")) {
        return Ok(PathBuf::from(value).join(".cache/liknon/manager"));
    }
    Ok(env::temp_dir().join("liknon-manager-cache"))
}

/// Resolves a portable materialization destination without permitting control-dir access.
pub(crate) fn contained_destination(
    workspace: &Workspace,
    relative: &Path,
) -> Result<PathBuf, ManagerError> {
    if relative.as_os_str().is_empty() || relative.is_absolute() {
        return Err(ManagerError::Usage(format!(
            "destination `{}` must be a non-empty workspace-relative path",
            relative.display()
        )));
    }
    let text = relative.to_string_lossy();
    liknon_resource_contracts::validate_relative_path(&text)?;
    if relative
        .components()
        .next()
        .is_some_and(|component| component.as_os_str() == ".liknon")
    {
        return Err(ManagerError::Usage(
            "materialization destinations cannot enter the `.liknon` control directory".to_owned(),
        ));
    }
    let destination = workspace.root.join(relative);
    validate_existing_ancestors(&workspace.root, &destination)?;
    Ok(destination)
}

/// Rejects existing symlink components and escapes below a filesystem boundary.
pub(crate) fn validate_existing_ancestors(
    boundary: &Path,
    destination: &Path,
) -> Result<(), ManagerError> {
    let canonical_boundary = fs::canonicalize(boundary).at(boundary)?;
    let relative = destination.strip_prefix(boundary).map_err(|_| {
        ManagerError::Integrity(format!(
            "destination `{}` escapes `{}`",
            destination.display(),
            boundary.display()
        ))
    })?;
    let mut current = canonical_boundary.clone();
    for component in relative.components() {
        current.push(component);
        if !current.exists() {
            // A later atomic write creates missing components below the last
            // verified ancestor; only existing nodes can conceal a symlink.
            continue;
        }
        let metadata = fs::symlink_metadata(&current).at(&current)?;
        if metadata.file_type().is_symlink() {
            return Err(ManagerError::Integrity(format!(
                "path component `{}` is a symlink",
                current.display()
            )));
        }
        let canonical = fs::canonicalize(&current).at(&current)?;
        if !canonical.starts_with(&canonical_boundary) {
            return Err(ManagerError::Integrity(format!(
                "path component `{}` escapes workspace boundary",
                current.display()
            )));
        }
    }
    Ok(())
}

fn require_existing_root(root: PathBuf) -> Result<Workspace, ManagerError> {
    let liknon = root.join(".liknon");
    let metadata = fs::symlink_metadata(&liknon).at(&liknon)?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(ManagerError::Integrity(format!(
            "workspace marker `{}` must be a non-symlink directory",
            liknon.display()
        )));
    }
    Ok(Workspace::from_root(root))
}

fn canonical_directory(path: &Path) -> Result<PathBuf, ManagerError> {
    let canonical = fs::canonicalize(path).at(path)?;
    if !canonical.is_dir() {
        return Err(ManagerError::Usage(format!(
            "workspace `{}` is not a directory",
            path.display()
        )));
    }
    Ok(canonical)
}

fn absolute_or_current(path: &Path) -> Result<PathBuf, ManagerError> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(env::current_dir().at("current directory")?.join(path))
    }
}

#[cfg(test)]
mod tests {
    use std::{env, fs};
    use tempfile::TempDir;

    use super::{contained_destination, Workspace};

    #[test]
    fn destination_stays_below_workspace() {
        let temporary = TempDir::new().unwrap();
        fs::create_dir(temporary.path().join(".liknon")).unwrap();
        let root = fs::canonicalize(temporary.path()).unwrap();
        let workspace = Workspace::from_root(root.clone());
        assert_eq!(
            contained_destination(&workspace, std::path::Path::new("docs/context.md")).unwrap(),
            root.join("docs/context.md")
        );
        assert!(contained_destination(&workspace, std::path::Path::new("../outside")).is_err());
        let _ = env::current_dir();
    }
}
