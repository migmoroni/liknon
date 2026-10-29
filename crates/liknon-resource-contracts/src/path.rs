use std::path::{Component, Path};

use crate::ContractError;

/// Canonical resource-store location below one consumer workspace.
pub const RESOURCE_ROOT_RELATIVE: &str = ".liknon/resources";

/// Validates a portable, non-empty relative resource path.
///
/// Backslashes and colons are rejected even on Unix so authored metadata
/// cannot change meaning when consumed on Windows.
pub fn validate_relative_path(value: &str) -> Result<(), ContractError> {
    let reject = |reason: &str| ContractError::UnsafePath {
        path: value.to_owned(),
        reason: reason.to_owned(),
    };

    if value.is_empty() {
        return Err(reject("the path is empty"));
    }
    if value.contains('\0') {
        return Err(reject("the path contains a NUL byte"));
    }
    if value.contains('\\') {
        return Err(reject("backslash separators are not portable"));
    }
    if value.contains(':') {
        return Err(reject("colon may introduce a platform-specific prefix"));
    }
    if value.len() > 1024 {
        return Err(reject("the path exceeds the portable length limit"));
    }
    for component in value.split('/') {
        if component.is_empty() {
            return Err(reject("empty path components are forbidden"));
        }
        if component.len() > 255 {
            return Err(reject("a path component exceeds the portable length limit"));
        }
        if component.ends_with([' ', '.'])
            || component.chars().any(|character| {
                character.is_control() || matches!(character, '<' | '>' | '"' | '|' | '?' | '*')
            })
            || is_windows_reserved(component)
        {
            return Err(reject(
                "a path component is not portable across supported systems",
            ));
        }
    }
    let path = Path::new(value);
    if path.is_absolute() {
        return Err(reject("absolute paths are forbidden"));
    }
    let mut count = 0usize;
    for component in path.components() {
        match component {
            Component::Normal(part) if !part.is_empty() => count += 1,
            Component::Normal(_) => return Err(reject("empty path components are forbidden")),
            Component::CurDir => return Err(reject("current-directory components are forbidden")),
            Component::ParentDir => return Err(reject("parent traversal is forbidden")),
            Component::RootDir | Component::Prefix(_) => {
                return Err(reject("platform path prefixes are forbidden"));
            }
        }
    }
    if count == 0 {
        return Err(reject("the path has no file component"));
    }
    Ok(())
}

fn is_windows_reserved(component: &str) -> bool {
    let stem = component
        .split_once('.')
        .map_or(component, |(stem, _)| stem)
        .to_ascii_uppercase();
    matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || stem
            .strip_prefix("COM")
            .or_else(|| stem.strip_prefix("LPT"))
            .is_some_and(|suffix| suffix.len() == 1 && matches!(suffix.as_bytes()[0], b'1'..=b'9'))
}

#[cfg(test)]
mod tests {
    use super::validate_relative_path;

    #[test]
    fn accepts_portable_paths_and_rejects_platform_escapes() {
        validate_relative_path("docs/evidence.md").unwrap();
        for rejected in [
            "",
            ".",
            "../outside",
            "/root",
            "C:/root",
            r"a\b",
            "a//b",
            "a/CON.txt",
            "a/trailing.",
            "a/question?",
        ] {
            assert!(validate_relative_path(rejected).is_err(), "{rejected}");
        }
    }
}
