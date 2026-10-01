//! Read-only access to validation knowledge embedded at build time.

use serde::Deserialize;

struct EmbeddedAsset {
    path: &'static str,
    bytes: &'static [u8],
}

include!(concat!(env!("OUT_DIR"), "/knowledge_assets.rs"));

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Catalog {
    pub(crate) documents: Vec<CatalogDocument>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CatalogDocument {
    pub(crate) id: String,
    pub(crate) kind: String,
    pub(crate) path: String,
    pub(crate) summary: String,
    pub(crate) applicability: Vec<String>,
}

pub(crate) fn catalog_bytes() -> Result<&'static [u8], String> {
    asset("catalog.json")
        .map(|asset| asset.bytes)
        .ok_or_else(|| "embedded knowledge catalog is missing".into())
}

pub(crate) fn catalog() -> Result<Catalog, String> {
    serde_json::from_slice(catalog_bytes()?)
        .map_err(|error| format!("embedded knowledge catalog is invalid: {error}"))
}

pub(crate) fn document(document_id: &str) -> Result<Option<&'static [u8]>, String> {
    let catalog = catalog()?;
    let Some(document) = catalog
        .documents
        .iter()
        .find(|document| document.id == document_id)
    else {
        return Ok(None);
    };
    let asset = asset(&document.path).ok_or_else(|| {
        format!(
            "embedded document {} is missing at {}",
            document.id, document.path
        )
    })?;
    std::str::from_utf8(asset.bytes).map_err(|error| {
        format!(
            "embedded document {} is not valid UTF-8: {error}",
            document.id
        )
    })?;
    Ok(Some(asset.bytes))
}

pub(crate) fn valid_document_id(value: &str) -> bool {
    let mut previous_delimiter = false;
    for (index, character) in value.chars().enumerate() {
        if matches!(character, '.' | '-') {
            if index == 0 || previous_delimiter {
                return false;
            }
            previous_delimiter = true;
        } else if (index == 0 && !character.is_ascii_lowercase())
            || !(character.is_ascii_lowercase() || character.is_ascii_digit())
        {
            return false;
        } else {
            previous_delimiter = false;
        }
    }
    !value.is_empty() && !previous_delimiter
}

fn asset(path: &str) -> Option<&'static EmbeddedAsset> {
    EMBEDDED_ASSETS
        .binary_search_by_key(&path, |asset| asset.path)
        .ok()
        .map(|index| &EMBEDDED_ASSETS[index])
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};
    use std::{collections::BTreeSet, fs, path::Path};

    #[test]
    fn embedded_registry_is_safe_complete_and_byte_exact() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/validation/knowledge");
        let mut previous = None;
        let mut source_paths = BTreeSet::new();
        collect_source_paths(&root, Path::new(""), &mut source_paths);
        assert_eq!(source_paths.len(), EMBEDDED_ASSETS.len());

        for asset in EMBEDDED_ASSETS {
            assert!(previous.is_none_or(|path| path < asset.path));
            previous = Some(asset.path);
            assert!(safe_asset_path(asset.path));
            assert!(std::str::from_utf8(asset.bytes).is_ok());
            assert_eq!(
                fs::read(root.join(asset.path)).unwrap(),
                asset.bytes,
                "embedded bytes differ for {}",
                asset.path
            );
            assert!(source_paths.remove(asset.path));
        }
        assert!(source_paths.is_empty());

        let catalog = catalog().unwrap();
        for document in catalog.documents {
            assert!(asset(&document.path).is_some(), "missing {}", document.id);
        }
    }

    #[test]
    fn embedded_asset_digest_is_deterministic() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/validation/knowledge");
        assert_eq!(embedded_digest(), embedded_digest());
        assert_eq!(embedded_digest(), source_digest(&root));
        assert_eq!(embedded_digest().len(), 64);
    }

    #[test]
    fn selectors_reject_paths_and_malformed_ids() {
        for value in ["", "../escape", "/absolute", "tools/cargo.md", "Bad.ID"] {
            assert!(!valid_document_id(value), "accepted {value}");
        }
        assert!(valid_document_id("tool.cargo"));
    }

    fn collect_source_paths(root: &Path, relative: &Path, paths: &mut BTreeSet<String>) {
        let mut entries = fs::read_dir(root.join(relative))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let child = relative.join(entry.file_name());
            let kind = entry.file_type().unwrap();
            assert!(!kind.is_symlink());
            if kind.is_dir() {
                collect_source_paths(root, &child, paths);
            } else {
                assert!(kind.is_file());
                paths.insert(child.to_string_lossy().replace('\\', "/"));
            }
        }
    }

    fn safe_asset_path(path: &str) -> bool {
        !path.is_empty()
            && !path.starts_with('/')
            && path
                .split('/')
                .all(|part| !part.is_empty() && part != "." && part != "..")
    }

    fn embedded_digest() -> String {
        let mut digest = Sha256::new();
        for asset in EMBEDDED_ASSETS {
            digest.update((asset.path.len() as u64).to_be_bytes());
            digest.update(asset.path.as_bytes());
            digest.update((asset.bytes.len() as u64).to_be_bytes());
            digest.update(asset.bytes);
        }
        format!("{:x}", digest.finalize())
    }

    fn source_digest(root: &Path) -> String {
        let mut paths = BTreeSet::new();
        collect_source_paths(root, Path::new(""), &mut paths);
        let mut digest = Sha256::new();
        for path in paths {
            let bytes = fs::read(root.join(&path)).unwrap();
            digest.update((path.len() as u64).to_be_bytes());
            digest.update(path.as_bytes());
            digest.update((bytes.len() as u64).to_be_bytes());
            digest.update(bytes);
        }
        format!("{:x}", digest.finalize())
    }
}
