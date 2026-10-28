use std::{
    fs,
    path::{Component, Path, PathBuf},
};

use serde::{de::DeserializeOwned, Serialize};

use crate::error::{contract, Error, Result};

pub(crate) fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let bytes = fs::read(path).map_err(|source| Error::Read {
        path: path.to_path_buf(),
        source,
    })?;
    serde_json::from_slice(&bytes).map_err(|source| Error::Json {
        path: path.to_path_buf(),
        source,
    })
}

pub(crate) fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    write_bytes(path, &bytes)
}

pub(crate) fn write_bytes(path: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| Error::Write {
            path: parent.to_path_buf(),
            source,
        })?;
    }
    fs::write(path, bytes).map_err(|source| Error::Write {
        path: path.to_path_buf(),
        source,
    })
}

pub(crate) fn safe_relative_path(value: &str, field: &str) -> Result<PathBuf> {
    let path = Path::new(value);
    if value.is_empty()
        || value.contains(['\\', ':'])
        || value.split('/').any(str::is_empty)
        || path.is_absolute()
    {
        return Err(contract(format!(
            "{field} must be a canonical slash-separated relative path"
        )));
    }
    if path
        .components()
        .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(contract(format!(
            "{field} may contain only normal relative path components"
        )));
    }
    Ok(path.to_path_buf())
}

pub(crate) fn safe_identifier(value: &str, field: &str) -> Result<()> {
    let valid = !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && value
            .as_bytes()
            .first()
            .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
        && value
            .as_bytes()
            .last()
            .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit());
    if !valid {
        return Err(contract(format!(
            "{field} must contain lowercase ASCII letters, digits, and internal hyphens"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_parent_and_absolute_paths() {
        assert!(safe_relative_path("../secret", "path").is_err());
        assert!(safe_relative_path("/secret", "path").is_err());
        assert!(safe_relative_path("..\\secret", "path").is_err());
        assert!(safe_relative_path("C:\\secret", "path").is_err());
        assert!(safe_relative_path("safe//file", "path").is_err());
        assert!(safe_relative_path("safe/file", "path").is_ok());
    }

    #[test]
    fn identifiers_are_narrow() {
        assert!(safe_identifier("validation-cli", "id").is_ok());
        assert!(safe_identifier("Validation", "id").is_err());
        assert!(safe_identifier("validation/path", "id").is_err());
    }
}
