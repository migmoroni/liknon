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
        Some(workspace_root) => validation::validate_for_initialization(
            parsed,
            workspace_root.path,
            workspace_root.is_virtual,
        ),
        None => validation::validate(parsed),
    }
}

struct InitializationWorkspaceRoot {
    path: std::path::PathBuf,
    is_virtual: bool,
}

enum InitializationLocation {
    Existing(std::path::PathBuf),
    Future,
}

const MAX_INITIALIZATION_SYMLINKS: usize = 40;

fn resolve_virtual_workspace_root(
    declared: &Path,
    future_directory: &Path,
    canonical_workspace: &Path,
    configuration_path: &Path,
) -> Result<InitializationWorkspaceRoot, ValidatorError> {
    let location = if declared.is_absolute() {
        resolve_initialization_absolute(
            declared,
            future_directory,
            canonical_workspace,
            configuration_path,
            0,
        )?
    } else {
        resolve_initialization_components(
            InitializationLocation::Future,
            declared,
            future_directory,
            canonical_workspace,
            configuration_path,
            0,
        )?
    };

    match location {
        InitializationLocation::Future => {
            finish_initialization_root(future_directory.to_path_buf(), true, configuration_path)
        }
        InitializationLocation::Existing(path) => {
            finish_initialization_root(path, false, configuration_path)
        }
    }
}

fn resolve_initialization_absolute(
    declared: &Path,
    future_directory: &Path,
    canonical_workspace: &Path,
    configuration_path: &Path,
    followed_symlinks: usize,
) -> Result<InitializationLocation, ValidatorError> {
    if let Ok(resolved) = declared.canonicalize() {
        return Ok(InitializationLocation::Existing(resolved));
    }
    let relative = declared.strip_prefix(canonical_workspace).map_err(|_| {
        ValidatorError::invalid(
            configuration_path,
            format!(
                "invalid workspaceRoot: {} does not exist",
                declared.display()
            ),
        )
    })?;
    resolve_initialization_components(
        InitializationLocation::Existing(canonical_workspace.to_path_buf()),
        relative,
        future_directory,
        canonical_workspace,
        configuration_path,
        followed_symlinks,
    )
}

fn resolve_initialization_components(
    mut location: InitializationLocation,
    relative: &Path,
    future_directory: &Path,
    canonical_workspace: &Path,
    configuration_path: &Path,
    followed_symlinks: usize,
) -> Result<InitializationLocation, ValidatorError> {
    use std::path::Component;

    for component in relative.components() {
        location = match (location, component) {
            (location, Component::CurDir) => location,
            (InitializationLocation::Future, Component::ParentDir) => {
                InitializationLocation::Existing(canonical_workspace.to_path_buf())
            }
            (InitializationLocation::Existing(path), Component::ParentDir) => {
                InitializationLocation::Existing(path.join("..").canonicalize().map_err(
                    |error| {
                        ValidatorError::invalid(
                            configuration_path,
                            format!("invalid workspaceRoot: {error}"),
                        )
                    },
                )?)
            }
            (InitializationLocation::Future, Component::Normal(_)) => {
                return Err(ValidatorError::invalid(
                    configuration_path,
                    format!(
                        "invalid workspaceRoot: {} will not exist after initialization",
                        future_directory.join(relative).display()
                    ),
                ));
            }
            (InitializationLocation::Existing(path), Component::Normal(name)) => {
                let candidate = path.join(name);
                match candidate.canonicalize() {
                    Ok(resolved) => InitializationLocation::Existing(resolved),
                    Err(_) if candidate == future_directory => InitializationLocation::Future,
                    Err(_) if candidate.is_symlink() => {
                        if followed_symlinks >= MAX_INITIALIZATION_SYMLINKS {
                            return Err(ValidatorError::invalid(
                                configuration_path,
                                "invalid workspaceRoot: too many symbolic links",
                            ));
                        }
                        let target = std::fs::read_link(&candidate).map_err(|error| {
                            ValidatorError::invalid(
                                configuration_path,
                                format!("invalid workspaceRoot: {error}"),
                            )
                        })?;
                        if target.is_absolute() {
                            resolve_initialization_absolute(
                                &target,
                                future_directory,
                                canonical_workspace,
                                configuration_path,
                                followed_symlinks + 1,
                            )?
                        } else {
                            resolve_initialization_components(
                                InitializationLocation::Existing(path),
                                &target,
                                future_directory,
                                canonical_workspace,
                                configuration_path,
                                followed_symlinks + 1,
                            )?
                        }
                    }
                    Err(error) => {
                        return Err(ValidatorError::invalid(
                            configuration_path,
                            format!("invalid workspaceRoot: {error}"),
                        ));
                    }
                }
            }
            (_, Component::RootDir | Component::Prefix(_)) => {
                return Err(ValidatorError::invalid(
                    configuration_path,
                    "invalid workspaceRoot component",
                ));
            }
        };
    }
    Ok(location)
}

fn finish_initialization_root(
    path: std::path::PathBuf,
    is_virtual: bool,
    configuration_path: &Path,
) -> Result<InitializationWorkspaceRoot, ValidatorError> {
    if !is_virtual && !path.is_dir() {
        return Err(ValidatorError::invalid(
            configuration_path,
            "workspaceRoot is not a directory",
        ));
    }
    Ok(InitializationWorkspaceRoot { path, is_virtual })
}

#[cfg(test)]
mod tests {
    use crate::{config, initialization};
    use serde_json::json;
    use std::fs;
    #[cfg(unix)]
    use std::os::unix::fs::symlink;
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

    #[cfg(unix)]
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

    #[test]
    fn future_directory_forms_match_precreated_and_installed_loading() {
        for declared in [
            ".",
            "././",
            "",
            "../.validation",
            "../existing/../.validation",
        ] {
            for precreate in [false, true] {
                let workspace = TempDir::new().unwrap();
                fs::create_dir(workspace.path().join("existing")).unwrap();
                if precreate {
                    fs::create_dir(workspace.path().join(".validation")).unwrap();
                }
                let candidate = workspace.path().join("candidate.json");
                fs::write(&candidate, serde_json::to_vec(&document(declared)).unwrap()).unwrap();
                let canonical_workspace = workspace.path().canonicalize().unwrap();
                let destination = workspace.path().join(".validation/config.json");
                let future = config::validate_at(
                    &fs::read(&candidate).unwrap(),
                    &destination,
                    &canonical_workspace,
                )
                .unwrap_or_else(|error| {
                    panic!(
                        "future validation failed for {declared:?}, precreate={precreate}: {error}"
                    )
                });
                let result = initialization::provision_config(&canonical_workspace, &candidate);
                assert_eq!(result.status, crate::contracts::init::InitStatus::Success);
                let installed = config::load(Some(&destination), workspace.path()).unwrap();
                assert_eq!(future.workspace_root, installed.workspace_root);
                assert_eq!(
                    future.suite_directories["suite"].absolute,
                    installed.suite_directories["suite"].absolute
                );
                assert_eq!(
                    installed.workspace_root,
                    workspace.path().join(".validation")
                );
            }
        }
    }

    #[cfg(unix)]
    #[test]
    fn future_symlink_forms_match_precreated_and_installed_loading() {
        for declared in ["../future-link", "../future-link/.."] {
            for precreate in [false, true] {
                let workspace = TempDir::new().unwrap();
                if precreate {
                    fs::create_dir(workspace.path().join(".validation")).unwrap();
                }
                symlink(".validation", workspace.path().join("future-link")).unwrap();
                let candidate = workspace.path().join("candidate.json");
                let contents = serde_json::to_vec(&document(declared)).unwrap();
                fs::write(&candidate, &contents).unwrap();
                let canonical_workspace = workspace.path().canonicalize().unwrap();
                let destination = workspace.path().join(".validation/config.json");

                let before = config::validate_at(
                    &contents,
                    &destination,
                    &canonical_workspace,
                )
                .unwrap_or_else(|error| {
                    panic!(
                        "future symlink validation failed for {declared:?}, precreate={precreate}: {error}"
                    )
                });
                let result = initialization::provision_config(&canonical_workspace, &candidate);
                assert_eq!(result.status, crate::contracts::init::InitStatus::Success);
                let installed = config::load(Some(&destination), workspace.path()).unwrap();

                assert_eq!(before.workspace_root, installed.workspace_root);
                assert_eq!(
                    before.suite_directories["suite"].absolute,
                    installed.suite_directories["suite"].absolute
                );
                let expected = if declared.ends_with("/..") {
                    canonical_workspace.clone()
                } else {
                    workspace.path().join(".validation")
                };
                assert_eq!(installed.workspace_root, expected);
            }
        }
    }

    #[cfg(unix)]
    #[test]
    fn future_symlink_resolution_rejects_targets_not_created_by_initialization() {
        for (link, target) in [
            ("unrelated-link", "missing"),
            ("descendant-link", ".validation/missing"),
        ] {
            let workspace = TempDir::new().unwrap();
            symlink(target, workspace.path().join(link)).unwrap();
            let destination = workspace.path().join(".validation/config.json");
            let contents = serde_json::to_vec(&document(&format!("../{link}"))).unwrap();
            let error = config::validate_at(
                &contents,
                &destination,
                &workspace.path().canonicalize().unwrap(),
            )
            .unwrap_err();
            assert!(error.to_string().contains("invalid workspaceRoot"));
            assert!(!workspace.path().join(".validation").exists());
        }
    }

    #[test]
    fn future_directory_does_not_invent_required_descendants() {
        let workspace = TempDir::new().unwrap();
        let destination = workspace.path().join(".validation/config.json");
        let contents = serde_json::to_vec(&document("missing/..")).unwrap();
        let error = config::validate_at(
            &contents,
            &destination,
            &workspace.path().canonicalize().unwrap(),
        )
        .unwrap_err();
        assert!(error
            .to_string()
            .contains("will not exist after initialization"));
    }
}
