#![allow(dead_code)]

use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Component, Path, PathBuf},
};

pub const CANONICAL_ROOT: &str = "docs/validation/knowledge";
pub const PROJECTED_ROOT: &str = "skills/workspace-validator/references/knowledge";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Catalog {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub schema_version: u32,
    pub vocabularies: Vocabularies,
    pub navigation: Vec<String>,
    pub documents: Vec<Document>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Vocabularies {
    pub applicability_tags: Vec<VocabularyEntry>,
    pub evidence_dimensions: Vec<VocabularyEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VocabularyEntry {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Document {
    pub id: String,
    pub kind: String,
    pub path: String,
    pub summary: String,
    pub questions: Vec<String>,
    pub applicability: Vec<String>,
    pub evidence_dimensions: Vec<String>,
    pub related: Vec<String>,
    pub sources: Vec<String>,
    pub status: String,
    pub last_reviewed: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SourceRegister {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub schema_version: u32,
    pub sources: Vec<Source>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Source {
    pub id: String,
    pub authority: String,
    pub title: String,
    pub revision: String,
    pub uri: String,
    pub scope: String,
    pub kind: String,
    pub publication_status: String,
    pub last_reviewed: String,
    pub locations: Vec<SourceLocation>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceLocation {
    pub id: String,
    pub locator: String,
    pub uri: String,
    pub supports: String,
}

pub fn load_catalog(repository: &Path) -> Result<Catalog, String> {
    read_json(&repository.join(CANONICAL_ROOT).join("catalog.json"))
}

pub fn load_sources(repository: &Path) -> Result<SourceRegister, String> {
    read_json(&repository.join(CANONICAL_ROOT).join("sources.json"))
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, String> {
    let bytes =
        fs::read(path).map_err(|error| format!("cannot read {}: {error}", path.display()))?;
    serde_json::from_slice(&bytes).map_err(|error| format!("invalid {}: {error}", path.display()))
}

pub fn render_sources(register: &SourceRegister) -> String {
    let mut rendered = String::from(
        "# Validation Knowledge Sources\n\nThis file is generated from `sources.json`. Do not edit it directly.\n\n",
    );
    for source in &register.sources {
        rendered.push_str(&format!(
            "## {}\n\n- ID: `{}`\n- Authority: {}\n- Revision: {}\n- Status: `{}`\n- Kind: `{}`\n- Scope: {}\n- Primary URI: <{}>\n- Last reviewed: `{}`\n\n### Locations\n\n",
            source.title,
            source.id,
            source.authority,
            source.revision,
            source.publication_status,
            source.kind,
            source.scope,
            source.uri,
            source.last_reviewed
        ));
        for location in &source.locations {
            rendered.push_str(&format!(
                "- `{id}` — {locator}. Supports: {supports} [Open location]({uri})\n",
                id = location.id,
                locator = location.locator,
                supports = location.supports,
                uri = location.uri
            ));
        }
        rendered.push('\n');
    }
    rendered
}

pub fn write_generated(repository: &Path) -> Result<String, String> {
    let canonical = repository.join(CANONICAL_ROOT);
    let sources = load_sources(repository)?;
    fs::write(canonical.join("SOURCES.md"), render_sources(&sources))
        .map_err(|error| format!("cannot generate SOURCES.md: {error}"))?;

    let projected = repository.join(PROJECTED_ROOT);
    if projected.exists() {
        fs::remove_dir_all(&projected)
            .map_err(|error| format!("cannot replace {}: {error}", projected.display()))?;
    }
    copy_tree(&canonical, &projected)?;
    verify(repository)
}

pub fn verify(repository: &Path) -> Result<String, String> {
    validate_schemas(repository)?;
    let catalog = load_catalog(repository)?;
    let sources = load_sources(repository)?;
    validate_registers(repository, &catalog, &sources)?;

    let expected_sources = render_sources(&sources);
    let actual_sources = fs::read_to_string(repository.join(CANONICAL_ROOT).join("SOURCES.md"))
        .map_err(|error| format!("cannot read generated SOURCES.md: {error}"))?;
    if actual_sources != expected_sources {
        return Err("SOURCES.md is stale; run the knowledge release task with --write".into());
    }

    compare_trees(
        &repository.join(CANONICAL_ROOT),
        &repository.join(PROJECTED_ROOT),
    )?;
    validate_routing_fixture(repository, &catalog)?;
    validate_forward_trials(repository, &catalog)?;
    tree_digest(&repository.join(CANONICAL_ROOT))
}

fn validate_schemas(repository: &Path) -> Result<(), String> {
    for name in [
        "schemas/knowledge-catalog.schema.json",
        "schemas/knowledge-sources.schema.json",
    ] {
        let value: serde_json::Value = read_json(&repository.join(name))?;
        if value["$schema"] != "https://json-schema.org/draft/2020-12/schema"
            || value["type"] != "object"
            || value["additionalProperties"] != false
        {
            return Err(format!(
                "{name} must be a closed Draft 2020-12 object schema"
            ));
        }
    }
    Ok(())
}

fn validate_registers(
    repository: &Path,
    catalog: &Catalog,
    register: &SourceRegister,
) -> Result<(), String> {
    if catalog.schema_version != 1
        || catalog.schema != "../../../schemas/knowledge-catalog.schema.json"
    {
        return Err("catalog schema contract is unsupported".into());
    }
    if register.schema_version != 1
        || register.schema != "../../../schemas/knowledge-sources.schema.json"
    {
        return Err("source-register schema contract is unsupported".into());
    }

    let applicability = validate_vocabulary(&catalog.vocabularies.applicability_tags)?;
    let dimensions = validate_vocabulary(&catalog.vocabularies.evidence_dimensions)?;
    let mut source_ids = BTreeSet::new();
    let mut locations = BTreeMap::new();
    for source in &register.sources {
        if !valid_dot_id(&source.id) || !source_ids.insert(source.id.as_str()) {
            return Err(format!("invalid or duplicate source ID {}", source.id));
        }
        if !source.uri.starts_with("https://") || source.locations.is_empty() {
            return Err(format!(
                "source {} has no HTTPS URI or locations",
                source.id
            ));
        }
        if !matches!(
            source.kind.as_str(),
            "specification" | "standard" | "maintainer-documentation" | "primary-research"
        ) || !matches!(
            source.publication_status.as_str(),
            "final" | "draft" | "living"
        ) || !valid_date(&source.last_reviewed)
            || [
                &source.authority,
                &source.title,
                &source.revision,
                &source.scope,
            ]
            .iter()
            .any(|value| value.trim().is_empty())
        {
            return Err(format!(
                "source {} violates its schema vocabulary",
                source.id
            ));
        }
        let mut local_ids = BTreeSet::new();
        for location in &source.locations {
            if !valid_kebab_id(&location.id)
                || !local_ids.insert(location.id.as_str())
                || !location.uri.starts_with("https://")
                || location.locator.trim().is_empty()
                || location.supports.trim().is_empty()
            {
                return Err(format!("invalid location {}#{}", source.id, location.id));
            }
            locations.insert(
                format!("{}#{}", source.id, location.id),
                location.uri.as_str(),
            );
        }
    }

    let root = repository.join(CANONICAL_ROOT);
    let mut document_ids = BTreeSet::new();
    let mut document_paths = BTreeSet::new();
    for document in &catalog.documents {
        let expected_prefix = format!("{}.", document.kind);
        if !valid_dot_id(&document.id)
            || !document.id.starts_with(&expected_prefix)
            || !document_ids.insert(document.id.as_str())
            || !document_paths.insert(document.path.as_str())
        {
            return Err(format!("invalid or duplicate document {}", document.id));
        }
        if !matches!(
            document.kind.as_str(),
            "foundation"
                | "pattern"
                | "language"
                | "framework"
                | "technology"
                | "tool"
                | "concern"
                | "standard"
                | "recipe"
        ) || !matches!(document.status.as_str(), "draft" | "reviewed")
            || !valid_date(&document.last_reviewed)
            || document.summary.is_empty()
            || document.summary.len() > 240
            || document.questions.is_empty()
            || document
                .questions
                .iter()
                .any(|value| value.trim().is_empty())
            || document.applicability.is_empty()
            || document.evidence_dimensions.is_empty()
            || document.sources.is_empty()
        {
            return Err(format!(
                "document {} violates its schema vocabulary",
                document.id
            ));
        }
        let relative = safe_relative(&document.path)?;
        if relative.extension().and_then(|value| value.to_str()) != Some("md")
            || relative.file_name().and_then(|value| value.to_str()) == Some("README.md")
            || !root.join(&relative).is_file()
        {
            return Err(format!(
                "document path {} is not a substantive guide",
                document.path
            ));
        }
        for tag in &document.applicability {
            if !applicability.contains(tag.as_str()) {
                return Err(format!(
                    "document {} uses unknown applicability {tag}",
                    document.id
                ));
            }
        }
        for dimension in &document.evidence_dimensions {
            if !dimensions.contains(dimension.as_str()) {
                return Err(format!(
                    "document {} uses unknown dimension {dimension}",
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
        let markdown = fs::read_to_string(root.join(&relative))
            .map_err(|error| format!("cannot read {}: {error}", document.path))?;
        let cited = validate_citations(document, &markdown, &locations)?;
        let declared = document
            .sources
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if cited != declared {
            return Err(format!(
                "document {} citation sources {:?} do not equal declared sources {:?}",
                document.id, cited, declared
            ));
        }
        validate_links(&root, &relative, &markdown)?;
        let index = relative.parent().unwrap_or(Path::new("")).join("README.md");
        let index_text = fs::read_to_string(root.join(index))
            .map_err(|error| format!("cannot read category index: {error}"))?;
        let file_name = relative.file_name().unwrap().to_string_lossy();
        if !index_text.contains(&format!("]({file_name})")) {
            return Err(format!("category index does not list {}", document.path));
        }
    }
    for document in &catalog.documents {
        for related in &document.related {
            if !document_ids.contains(related.as_str()) || related == &document.id {
                return Err(format!(
                    "document {} has invalid related ID {related}",
                    document.id
                ));
            }
        }
    }

    let actual_guides = collect_files(&root)?
        .into_iter()
        .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("md"))
        .filter(|path| path.file_name().and_then(|value| value.to_str()) != Some("README.md"))
        .filter(|path| path != Path::new("SOURCES.md"))
        .map(|path| slash_path(&path))
        .collect::<BTreeSet<_>>();
    let declared_paths = document_paths
        .iter()
        .map(|path| (*path).to_owned())
        .collect();
    if actual_guides != declared_paths {
        return Err(format!(
            "catalog guide inventory mismatch: {actual_guides:?}"
        ));
    }

    let expected_navigation = collect_files(&root)?
        .into_iter()
        .filter(|path| {
            path.file_name().and_then(|value| value.to_str()) == Some("README.md")
                || path == Path::new("SOURCES.md")
        })
        .map(|path| slash_path(&path))
        .collect::<BTreeSet<_>>();
    let navigation = catalog
        .navigation
        .iter()
        .map(|path| safe_relative(path).map(|value| slash_path(&value)))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if navigation != expected_navigation {
        return Err("catalog navigation assets do not match current indexes".into());
    }

    for path in collect_files(&root)? {
        if path.extension().and_then(|value| value.to_str()) == Some("md") {
            let markdown = fs::read_to_string(root.join(&path))
                .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
            if markdown.trim().is_empty() || markdown.contains("TODO") {
                return Err(format!(
                    "empty or placeholder knowledge file {}",
                    path.display()
                ));
            }
            validate_links(&root, &path, &markdown)?;
        }
    }
    Ok(())
}

fn validate_vocabulary(entries: &[VocabularyEntry]) -> Result<BTreeSet<&str>, String> {
    let mut ids = BTreeSet::new();
    for entry in entries {
        if !valid_dot_id(&entry.id)
            || entry.label.trim().is_empty()
            || !ids.insert(entry.id.as_str())
        {
            return Err(format!("invalid or duplicate vocabulary ID {}", entry.id));
        }
    }
    if ids.is_empty() {
        return Err("vocabulary cannot be empty".into());
    }
    Ok(ids)
}

fn validate_citations<'a>(
    document: &Document,
    markdown: &'a str,
    locations: &BTreeMap<String, &str>,
) -> Result<BTreeSet<&'a str>, String> {
    let mut remaining = markdown;
    let mut cited = BTreeSet::new();
    while let Some(start) = remaining.find("[Source: ") {
        remaining = &remaining[start + 9..];
        let label_end = remaining
            .find("](")
            .ok_or_else(|| format!("malformed citation in {}", document.path))?;
        let label = &remaining[..label_end];
        remaining = &remaining[label_end + 2..];
        let uri_end = remaining
            .find(')')
            .ok_or_else(|| format!("malformed citation URI in {}", document.path))?;
        let uri = &remaining[..uri_end];
        let expected = locations
            .get(label)
            .ok_or_else(|| format!("unknown source location {label} in {}", document.path))?;
        if uri != *expected {
            return Err(format!(
                "citation {label} in {} does not use its registered URI",
                document.path
            ));
        }
        cited.insert(label.split_once('#').unwrap().0);
        remaining = &remaining[uri_end + 1..];
    }
    if cited.is_empty() {
        return Err(format!(
            "document {} has no source-location citation",
            document.id
        ));
    }
    Ok(cited)
}

fn validate_links(root: &Path, relative: &Path, markdown: &str) -> Result<(), String> {
    let mut remaining = markdown;
    while let Some(start) = remaining.find("](") {
        remaining = &remaining[start + 2..];
        let Some(end) = remaining.find(')') else {
            break;
        };
        let target = &remaining[..end];
        remaining = &remaining[end + 1..];
        if target.is_empty()
            || target.starts_with('#')
            || target.starts_with("https://")
            || target.starts_with("mailto:")
        {
            continue;
        }
        let path = target.split('#').next().unwrap();
        let parent = relative.parent().unwrap_or(Path::new(""));
        let resolved = normalize_relative(&parent.join(path))?;
        if !root.join(&resolved).exists() {
            return Err(format!("broken link {target} in {}", relative.display()));
        }
    }
    Ok(())
}

fn validate_routing_fixture(repository: &Path, catalog: &Catalog) -> Result<(), String> {
    let value: serde_json::Value = read_json(&repository.join("fixtures/knowledge/routing.json"))?;
    if value["schemaVersion"] != 1 {
        return Err("unsupported routing fixture schemaVersion".into());
    }
    let ids = catalog
        .documents
        .iter()
        .map(|document| document.id.as_str())
        .collect::<BTreeSet<_>>();
    let cases = value["cases"]
        .as_array()
        .ok_or("routing cases must be an array")?;
    if cases.len() < 5 {
        return Err("routing fixture must cover at least five representative cases".into());
    }
    let mut case_ids = BTreeSet::new();
    for case in cases {
        let id = case["id"].as_str().ok_or("routing case needs an ID")?;
        if !case_ids.insert(id) {
            return Err(format!("duplicate routing case {id}"));
        }
        for field in ["expectedDocumentIds", "rejectedDocumentIds"] {
            for document in case[field]
                .as_array()
                .ok_or("routing expectations must be arrays")?
            {
                let document = document
                    .as_str()
                    .ok_or("routing document ID must be a string")?;
                if !ids.contains(document) {
                    return Err(format!(
                        "routing case {id} uses unknown document {document}"
                    ));
                }
            }
        }
    }
    Ok(())
}

fn validate_forward_trials(repository: &Path, catalog: &Catalog) -> Result<(), String> {
    let value: serde_json::Value =
        read_json(&repository.join("evals/scenarios/shared-knowledge/scenarios.json"))?;
    if value["schemaVersion"] != 1 {
        return Err("unsupported forward-trial schemaVersion".into());
    }
    let ids = catalog
        .documents
        .iter()
        .map(|document| document.id.as_str())
        .collect::<BTreeSet<_>>();
    let scenarios = value["scenarios"]
        .as_array()
        .ok_or("forward trials must be an array")?;
    if scenarios.len() < 7 {
        return Err("forward trials must cover all seven required behaviors".into());
    }
    let mut scenario_ids = BTreeSet::new();
    for scenario in scenarios {
        let id = scenario["id"].as_str().ok_or("scenario needs an ID")?;
        if !scenario_ids.insert(id) {
            return Err(format!("duplicate forward-trial scenario {id}"));
        }
        let prompt = safe_relative(
            scenario["prompt"]
                .as_str()
                .ok_or("scenario needs a prompt")?,
        )?;
        if !repository
            .join("evals/scenarios/shared-knowledge")
            .join(prompt)
            .is_file()
        {
            return Err(format!("scenario {id} prompt is missing"));
        }
        for document in scenario["expectedDocumentIds"]
            .as_array()
            .ok_or("scenario expectations must be an array")?
        {
            let document = document
                .as_str()
                .ok_or("scenario document ID must be a string")?;
            if !ids.contains(document) {
                return Err(format!("scenario {id} uses unknown document {document}"));
            }
        }
        if scenario["assertions"].as_array().is_none_or(Vec::is_empty) {
            return Err(format!("scenario {id} needs deterministic assertions"));
        }
    }
    let rubric: serde_json::Value =
        read_json(&repository.join("evals/rubrics/shared-knowledge.json"))?;
    if rubric["schemaVersion"] != 1 || rubric["criteria"].as_array().is_none_or(Vec::is_empty) {
        return Err("shared-knowledge rubric is incomplete".into());
    }
    Ok(())
}

pub fn extract_recipe_json(markdown: &str) -> Result<serde_json::Value, String> {
    let start = markdown
        .find("```json\n")
        .ok_or("recipe has no JSON example")?
        + 8;
    let end = markdown[start..]
        .find("\n```")
        .ok_or("recipe JSON fence is not closed")?
        + start;
    if markdown[end + 4..].contains("```json\n") {
        return Err("recipe must contain exactly one complete JSON example".into());
    }
    serde_json::from_str(&markdown[start..end])
        .map_err(|error| format!("invalid recipe JSON: {error}"))
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

fn compare_trees(canonical: &Path, projected: &Path) -> Result<(), String> {
    let canonical_files = collect_files(canonical)?;
    let projected_files = collect_files(projected)?;
    if canonical_files != projected_files {
        return Err("projected knowledge has missing, extra, or stale paths".into());
    }
    for relative in canonical_files {
        let left = fs::read(canonical.join(&relative)).map_err(|error| error.to_string())?;
        let right = fs::read(projected.join(&relative)).map_err(|error| error.to_string())?;
        if left != right {
            return Err(format!(
                "projected knowledge differs at {}",
                relative.display()
            ));
        }
    }
    Ok(())
}

fn copy_tree(source: &Path, destination: &Path) -> Result<(), String> {
    fs::create_dir_all(destination)
        .map_err(|error| format!("cannot create {}: {error}", destination.display()))?;
    for relative in collect_files(source)? {
        let target = destination.join(&relative);
        fs::create_dir_all(target.parent().unwrap())
            .map_err(|error| format!("cannot create parent for {}: {error}", target.display()))?;
        fs::copy(source.join(&relative), &target)
            .map_err(|error| format!("cannot copy {}: {error}", relative.display()))?;
    }
    Ok(())
}

fn collect_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    if !root.is_dir() {
        return Err(format!("{} is not a directory", root.display()));
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
                "generated knowledge cannot contain symlink {}",
                child.display()
            ));
        } else if kind.is_dir() {
            collect_files_at(root, &child, files)?;
        } else if kind.is_file() {
            files.push(child);
        } else {
            return Err(format!("unsupported knowledge entry {}", child.display()));
        }
    }
    Ok(())
}

fn safe_relative(value: &str) -> Result<PathBuf, String> {
    if value.is_empty() {
        return Err("path cannot be empty".into());
    }
    let path = Path::new(value);
    if path.is_absolute()
        || !path
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
    {
        return Err(format!("unsafe relative path {value}"));
    }
    Ok(path.to_path_buf())
}

fn normalize_relative(path: &Path) -> Result<PathBuf, String> {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(value) => normalized.push(value),
            Component::CurDir => {}
            Component::ParentDir => {
                if !normalized.pop() {
                    return Err(format!("link escapes knowledge root: {}", path.display()));
                }
            }
            _ => return Err(format!("unsafe link path {}", path.display())),
        }
    }
    Ok(normalized)
}

fn valid_dot_id(value: &str) -> bool {
    valid_delimited_id(value, |character| character == '.' || character == '-')
}

fn valid_kebab_id(value: &str) -> bool {
    valid_delimited_id(value, |character| character == '-')
}

fn valid_delimited_id(value: &str, delimiter: impl Fn(char) -> bool) -> bool {
    let mut previous_delimiter = false;
    for (index, character) in value.chars().enumerate() {
        if delimiter(character) {
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

fn valid_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| index == 4 || index == 7 || byte.is_ascii_digit())
}

fn slash_path(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsafe_relative_paths_and_escaping_links_are_rejected() {
        for path in ["", "/absolute", "../escape", "nested/../escape"] {
            assert!(safe_relative(path).is_err(), "accepted unsafe path {path}");
        }
        assert!(normalize_relative(Path::new("../../escape")).is_err());
        assert_eq!(
            normalize_relative(Path::new("tools/../patterns/evidence.md")).unwrap(),
            Path::new("patterns/evidence.md")
        );
    }

    #[test]
    fn projection_comparison_rejects_missing_extra_and_modified_files() {
        let temporary = tempfile::tempdir().unwrap();
        let canonical = temporary.path().join("canonical");
        let projected = temporary.path().join("projected");
        fs::create_dir_all(&canonical).unwrap();
        fs::create_dir_all(&projected).unwrap();
        fs::write(canonical.join("guide.md"), "canonical").unwrap();

        assert!(compare_trees(&canonical, &projected).is_err());
        fs::write(projected.join("guide.md"), "modified").unwrap();
        assert!(compare_trees(&canonical, &projected).is_err());
        fs::write(projected.join("guide.md"), "canonical").unwrap();
        fs::write(projected.join("extra.md"), "extra").unwrap();
        assert!(compare_trees(&canonical, &projected).is_err());
        fs::remove_file(projected.join("extra.md")).unwrap();
        assert!(compare_trees(&canonical, &projected).is_ok());
    }
}
