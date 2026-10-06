//! Filesystem, referential-integrity, packaging, and digest checks.

use super::{
    contracts::{Catalog, SourceRegister, VocabularyEntry},
    CANONICAL_ROOT,
};
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs,
    path::{Component, Path, PathBuf},
};

pub(super) fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, String> {
    let bytes =
        fs::read(path).map_err(|error| format!("cannot read {}: {error}", path.display()))?;
    serde_json::from_slice(&bytes).map_err(|error| format!("invalid {}: {error}", path.display()))
}

pub(super) fn read_value(path: &Path) -> Result<Value, String> {
    read_json(path)
}

/// Validates the relationships needed to consume the catalog without interpreting prose.
pub fn validate_catalog_integrity(
    repository: &Path,
    catalog: &Catalog,
    sources: &SourceRegister,
) -> Result<(), String> {
    let applicability = unique_vocabulary_ids(
        &catalog.vocabularies.applicability_tags,
        "applicability tag",
    )?;
    let dimensions = unique_vocabulary_ids(
        &catalog.vocabularies.evidence_dimensions,
        "evidence dimension",
    )?;

    let mut source_ids = BTreeSet::new();
    for source in &sources.sources {
        if !source_ids.insert(source.id.as_str()) {
            return Err(format!("duplicate source ID {}", source.id));
        }
        let mut location_ids = BTreeSet::new();
        for location in &source.locations {
            if !location_ids.insert(location.id.as_str()) {
                return Err(format!(
                    "duplicate source location {}#{}",
                    source.id, location.id
                ));
            }
        }
    }

    let root = repository.join(CANONICAL_ROOT);
    let mut document_ids = BTreeSet::new();
    let mut document_paths = BTreeSet::new();
    for document in &catalog.documents {
        if !document_ids.insert(document.id.as_str()) {
            return Err(format!("duplicate document ID {}", document.id));
        }
        if !document_paths.insert(document.path.as_str()) {
            return Err(format!("duplicate document path {}", document.path));
        }
        let relative = safe_relative(&document.path)?;
        read_utf8_file(&root.join(relative), &document.path)?;
    }

    for document in &catalog.documents {
        for related in &document.related {
            if related == &document.id || !document_ids.contains(related.as_str()) {
                return Err(format!(
                    "document {} has invalid related ID {related}",
                    document.id
                ));
            }
        }
        for source in &document.sources {
            if !source_ids.contains(source.as_str()) {
                return Err(format!(
                    "document {} uses unknown source {source}",
                    document.id
                ));
            }
        }
        for tag in &document.applicability {
            if !applicability.contains(tag.as_str()) {
                return Err(format!(
                    "document {} uses unknown applicability tag {tag}",
                    document.id
                ));
            }
        }
        for dimension in &document.evidence_dimensions {
            if !dimensions.contains(dimension.as_str()) {
                return Err(format!(
                    "document {} uses unknown evidence dimension {dimension}",
                    document.id
                ));
            }
        }
    }

    let mut navigation_paths = BTreeSet::new();
    for path in &catalog.navigation {
        if !navigation_paths.insert(path.as_str()) {
            return Err(format!("duplicate navigation path {path}"));
        }
        let relative = safe_relative(path)?;
        read_utf8_file(&root.join(relative), path)?;
    }

    Ok(())
}

fn unique_vocabulary_ids<'a>(
    entries: &'a [VocabularyEntry],
    label: &str,
) -> Result<BTreeSet<&'a str>, String> {
    let mut ids = BTreeSet::new();
    for entry in entries {
        if !ids.insert(entry.id.as_str()) {
            return Err(format!("duplicate {label} ID {}", entry.id));
        }
    }
    Ok(ids)
}

fn read_utf8_file(path: &Path, declared: &str) -> Result<(), String> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| format!("cannot inspect declared asset {declared}: {error}"))?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(format!("declared asset {declared} is not a regular file"));
    }
    let bytes = fs::read(path)
        .map_err(|error| format!("cannot read declared asset {declared}: {error}"))?;
    std::str::from_utf8(&bytes)
        .map_err(|error| format!("declared asset {declared} is not UTF-8: {error}"))?;
    Ok(())
}

pub fn tree_digest(root: &Path) -> Result<String, String> {
    let files = collect_files(root)?;
    let mut digest = Sha256::new();
    for relative in files {
        let path = slash_path(&relative);
        let bytes = fs::read(root.join(&relative))
            .map_err(|error| format!("cannot read {}: {error}", relative.display()))?;
        digest.update((path.len() as u64).to_be_bytes());
        digest.update(path.as_bytes());
        digest.update((bytes.len() as u64).to_be_bytes());
        digest.update(bytes);
    }
    Ok(format!("{:x}", digest.finalize()))
}

pub fn embedded_source_assets(repository: &Path) -> Result<Vec<(String, Vec<u8>)>, String> {
    let root = repository.join(CANONICAL_ROOT);
    collect_files(&root)?
        .into_iter()
        .map(|relative| {
            let path = slash_path(&relative);
            let bytes = fs::read(root.join(&relative))
                .map_err(|error| format!("cannot read embedded source {path}: {error}"))?;
            Ok((path, bytes))
        })
        .collect()
}

pub fn validate_embedded_asset_inventory(
    catalog: &Catalog,
    assets: &[(String, Vec<u8>)],
) -> Result<(), String> {
    let mut paths = BTreeSet::new();
    for (path, bytes) in assets {
        let relative = safe_relative(path)?;
        if slash_path(&relative) != *path || !paths.insert(path.as_str()) {
            return Err(format!(
                "duplicate or non-normalized embedded asset path {path}"
            ));
        }
        std::str::from_utf8(bytes)
            .map_err(|error| format!("embedded asset {path} is not UTF-8: {error}"))?;
    }

    for required in ["catalog.json", "sources.json", "SOURCES.md"] {
        if !paths.contains(required) {
            return Err(format!("embedded asset inventory omits {required}"));
        }
    }
    for path in catalog
        .navigation
        .iter()
        .chain(catalog.documents.iter().map(|document| &document.path))
    {
        if !paths.contains(path.as_str()) {
            return Err(format!("catalog asset is not embedded at {path}"));
        }
    }
    Ok(())
}

pub(super) fn collect_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let metadata = fs::symlink_metadata(root)
        .map_err(|error| format!("cannot inspect {}: {error}", root.display()))?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(format!("{} is not a real directory", root.display()));
    }
    let mut files = Vec::new();
    collect_files_at(root, Path::new(""), &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_files_at(root: &Path, relative: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let directory = root.join(relative);
    let mut entries = fs::read_dir(&directory)
        .map_err(|error| format!("cannot read {}: {error}", directory.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("cannot enumerate {}: {error}", directory.display()))?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let child = relative.join(entry.file_name());
        let kind = entry.file_type().map_err(|error| error.to_string())?;
        if kind.is_symlink() {
            return Err(format!(
                "canonical knowledge cannot contain symlink {}",
                child.display()
            ));
        }
        if kind.is_dir() {
            collect_files_at(root, &child, files)?;
        } else if kind.is_file() {
            files.push(child);
        } else {
            return Err(format!("unsupported knowledge entry {}", child.display()));
        }
    }
    Ok(())
}

pub(super) fn safe_relative(value: &str) -> Result<PathBuf, String> {
    if value.is_empty() || value.contains('\\') {
        return Err(format!("unsafe relative path {value}"));
    }
    let path = Path::new(value);
    if path.is_absolute()
        || !path
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
        || slash_path(path) != value
    {
        return Err(format!("unsafe relative path {value}"));
    }
    Ok(path.to_path_buf())
}

fn slash_path(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}
