//! Loading and semantic validation for workspace-validator configuration.

mod directories;
mod loader;
pub(crate) mod parameters;
mod validation;

pub use validation::ValidatedConfig;

use crate::error::ValidatorError;
use std::path::Path;

/// Loads and fully validates a configuration before any subprocess starts.
///
/// # Errors
///
/// Returns [`ValidatorError`] when discovery, file access, JSON deserialization,
/// or semantic validation fails. No declared tool or check is executed before
/// this function returns successfully.
pub fn load(explicit: Option<&Path>, current: &Path) -> Result<ValidatedConfig, ValidatorError> {
    validation::validate(loader::load(explicit, current)?)
}

/// Validates configuration bytes using `canonical_path` as their eventual location.
///
/// This is used by deterministic provisioning so relative paths have exactly
/// the same meaning before and after the document is installed.
pub(crate) fn validate_at(
    contents: &[u8],
    canonical_path: &Path,
    canonical_workspace: &Path,
) -> Result<ValidatedConfig, ValidatorError> {
    let config: crate::contracts::config::Config = serde_json::from_slice(contents)
        .map_err(|error| ValidatorError::invalid(canonical_path, error.to_string()))?;
    let future_directory = canonical_path
        .parent()
        .ok_or_else(|| ValidatorError::invalid(canonical_path, "configuration has no parent"))?;
    let virtual_workspace_root = if !future_directory.exists() {
        Some(resolve_virtual_workspace_root(
            &config.workspace_root,
            future_directory,
            canonical_workspace,
            canonical_path,
        )?)
    } else {
        None
    };
    let parsed = loader::ParsedConfig {
        config,
        path: canonical_path.to_path_buf(),
    };
    match virtual_workspace_root {
        Some(workspace_root) => validation::validate_for_initialization(parsed, workspace_root),
        None => validation::validate(parsed),
    }
}

fn resolve_virtual_workspace_root(
    declared: &Path,
    future_directory: &Path,
    canonical_workspace: &Path,
    configuration_path: &Path,
) -> Result<std::path::PathBuf, ValidatorError> {
    use std::path::Component;

    if declared.is_absolute() {
        return declared.canonicalize().map_err(|error| {
            ValidatorError::invalid(
                configuration_path,
                format!("invalid workspaceRoot: {error}"),
            )
        });
    }
    let mut components = declared.components();
    while let Some(component) = components.next() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                let remainder = components.as_path();
                let resolved =
                    canonical_workspace
                        .join(remainder)
                        .canonicalize()
                        .map_err(|error| {
                            ValidatorError::invalid(
                                configuration_path,
                                format!("invalid workspaceRoot: {error}"),
                            )
                        })?;
                if !resolved.is_dir() {
                    return Err(ValidatorError::invalid(
                        configuration_path,
                        "workspaceRoot is not a directory",
                    ));
                }
                return Ok(resolved);
            }
            Component::Normal(_) | Component::RootDir | Component::Prefix(_) => {
                return Err(ValidatorError::invalid(
                    configuration_path,
                    format!(
                        "invalid workspaceRoot: {} does not exist yet",
                        future_directory.join(declared).display()
                    ),
                ));
            }
        }
    }
    Err(ValidatorError::invalid(
        configuration_path,
        format!(
            "invalid workspaceRoot: {} does not exist yet",
            future_directory.display()
        ),
    ))
}

#[cfg(all(test, unix))]
mod tests {
    use crate::{config, initialization};
    use serde_json::json;
    use std::{fs, os::unix::fs::symlink};
    use tempfile::TempDir;

    #[test]
    fn future_destination_and_installed_loading_resolve_identically() {
        let workspace = TempDir::new().unwrap();
        let target = TempDir::new().unwrap();
        fs::create_dir_all(target.path().join("nested")).unwrap();
        fs::create_dir(target.path().join("suite")).unwrap();
        symlink(target.path().join("nested"), workspace.path().join("link")).unwrap();
        let candidate = workspace.path().join("candidate.json");
        let document = json!({
            "schemaVersion": 6,
            "workspaceRoot": "../link/..",
            "defaultGroup": "all",
            "outputLimitBytes": 4096,
            "tools": [{"id":"tool","program":"tool","requiresTools":[],"versionArgs":["--version"],"versionParser":"firstSemver"}],
            "checks": [{"id":"check","label":"Check","description":"Check.","toolId":"tool","args":[],"requiresTools":[],"timeoutSeconds":1}],
            "suites": [{"id":"suite","label":"Suite","description":"Suite.","workingDirectory":"suite","checks":[{"checkId":"check","dependsOn":[]}]}],
            "groups": [{"id":"all","label":"All","description":"All.","members":[{"kind":"suite","id":"suite"}]}]
        });
        fs::write(&candidate, serde_json::to_vec(&document).unwrap()).unwrap();

        let future = config::validate_at(
            &fs::read(&candidate).unwrap(),
            &workspace.path().join(".validation/config.json"),
            &workspace.path().canonicalize().unwrap(),
        )
        .unwrap();
        let result =
            initialization::provision_config(&workspace.path().canonicalize().unwrap(), &candidate);
        assert_eq!(result.status, crate::contracts::init::InitStatus::Success);
        let installed = config::load(None, workspace.path()).unwrap();

        assert_eq!(future.workspace_root, installed.workspace_root);
        assert_eq!(
            future.suite_directories["suite"].absolute,
            installed.suite_directories["suite"].absolute
        );
        assert_eq!(
            installed.workspace_root,
            target.path().canonicalize().unwrap()
        );
    }
}
