#![allow(dead_code)]

use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Component, Path, PathBuf},
};

pub const CANONICAL_ROOT: &str = "docs/validation/knowledge";
const ROUTING_PATH: &str = "fixtures/knowledge/routing.json";
const FORWARD_TRIALS_PATH: &str = "evals/scenarios/shared-knowledge/scenarios.json";
const RUBRIC_PATH: &str = "evals/rubrics/shared-knowledge.json";
const REQUIRED_BEHAVIORS: [&str; 7] = [
    "minimal-selection",
    "synthesis-boundary",
    "evidence-before-tool",
    "bounded-proof",
    "precise-source-location",
    "offline-depth",
    "missing-topic",
];

pub struct CompiledSchemas {
    catalog: jsonschema::Validator,
    sources: jsonschema::Validator,
    routing: jsonschema::Validator,
    forward_trials: jsonschema::Validator,
    rubric: jsonschema::Validator,
    config: jsonschema::Validator,
}

#[derive(Clone, Copy)]
pub enum SchemaKind {
    Catalog,
    Sources,
    Routing,
    ForwardTrials,
    Rubric,
    Config,
}

impl CompiledSchemas {
    pub fn compile(repository: &Path) -> Result<Self, String> {
        Ok(Self {
            catalog: compile_schema(repository, "schemas/knowledge-catalog.schema.json")?,
            sources: compile_schema(repository, "schemas/knowledge-sources.schema.json")?,
            routing: compile_schema(repository, "schemas/knowledge-routing.schema.json")?,
            forward_trials: compile_schema(
                repository,
                "schemas/knowledge-forward-trials.schema.json",
            )?,
            rubric: compile_schema(repository, "schemas/knowledge-rubric.schema.json")?,
            config: compile_schema(repository, "schemas/config.schema.json")?,
        })
    }

    pub fn validate(&self, kind: SchemaKind, name: &str, instance: &Value) -> Result<(), String> {
        let validator = match kind {
            SchemaKind::Catalog => &self.catalog,
            SchemaKind::Sources => &self.sources,
            SchemaKind::Routing => &self.routing,
            SchemaKind::ForwardTrials => &self.forward_trials,
            SchemaKind::Rubric => &self.rubric,
            SchemaKind::Config => &self.config,
        };
        let diagnostics = validator
            .iter_errors(instance)
            .map(|error| {
                format!(
                    "instance {} schema {}: {}",
                    error.instance_path, error.schema_path, error
                )
            })
            .collect::<Vec<_>>();
        if diagnostics.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "{name} does not satisfy its schema: {}",
                diagnostics.join("; ")
            ))
        }
    }
}

fn compile_schema(repository: &Path, relative: &str) -> Result<jsonschema::Validator, String> {
    let schema: Value = read_json(&repository.join(relative))?;
    jsonschema::draft202012::options()
        .should_validate_formats(true)
        .build(&schema)
        .map_err(|error| format!("cannot compile {relative}: {error}"))
}

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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct RoutingRegistry {
    #[serde(rename = "$schema")]
    schema: String,
    schema_version: u32,
    cases: Vec<RoutingCase>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct RoutingCase {
    id: String,
    question: String,
    workspace_facts: Vec<String>,
    network_access: String,
    expected_document_ids: Vec<String>,
    rejected_document_ids: Vec<String>,
    expected_behavior: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ForwardTrialRegistry {
    #[serde(rename = "$schema")]
    schema: String,
    schema_version: u32,
    suite_id: String,
    suite_version: u32,
    rubric: String,
    required_behaviors: Vec<String>,
    context: TrialContext,
    scenarios: Vec<ForwardTrial>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TrialContext {
    allowed: Vec<String>,
    network: String,
    forbidden: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ForwardTrial {
    id: String,
    version: u32,
    prompt: String,
    network_access: String,
    expected_document_ids: Vec<String>,
    rejected_document_ids: Vec<String>,
    behaviors: Vec<String>,
    assertions: Vec<String>,
    rubric_criteria: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Rubric {
    #[serde(rename = "$schema")]
    schema: String,
    schema_version: u32,
    rubric_id: String,
    scale: BTreeMap<String, String>,
    criteria: Vec<RubricCriterion>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct RubricCriterion {
    id: String,
    question: String,
    failure_example: String,
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

fn read_value(path: &Path) -> Result<Value, String> {
    read_json(path)
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
    let schemas = CompiledSchemas::compile(repository)?;
    let source_value = read_value(&canonical.join("sources.json"))?;
    schemas.validate(SchemaKind::Sources, "sources.json", &source_value)?;
    let sources: SourceRegister = serde_json::from_value(source_value)
        .map_err(|error| format!("cannot deserialize sources.json: {error}"))?;
    fs::write(canonical.join("SOURCES.md"), render_sources(&sources))
        .map_err(|error| format!("cannot generate SOURCES.md: {error}"))?;
    verify(repository)
}

pub fn verify(repository: &Path) -> Result<String, String> {
    let schemas = CompiledSchemas::compile(repository)?;
    let catalog_value = read_value(&repository.join(CANONICAL_ROOT).join("catalog.json"))?;
    let sources_value = read_value(&repository.join(CANONICAL_ROOT).join("sources.json"))?;
    let (catalog, sources) =
        validate_catalog_source_values(repository, &schemas, catalog_value, sources_value)?;

    let expected_sources = render_sources(&sources);
    let actual_sources = fs::read_to_string(repository.join(CANONICAL_ROOT).join("SOURCES.md"))
        .map_err(|error| format!("cannot read generated SOURCES.md: {error}"))?;
    validate_generated_sources(&expected_sources, &actual_sources)?;

    let assets = embedded_source_assets(repository)?;
    validate_embedded_asset_inventory(&catalog, &assets)?;
    validate_structured_trials(repository, &schemas, &catalog)?;
    validate_recipes(repository, &schemas, &catalog)?;
    tree_digest(&repository.join(CANONICAL_ROOT))
}

pub fn validate_generated_sources(expected: &str, actual: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err("SOURCES.md is stale; run the knowledge release task with --write".into())
    }
}

pub fn validate_catalog_source_values(
    repository: &Path,
    schemas: &CompiledSchemas,
    catalog_value: Value,
    sources_value: Value,
) -> Result<(Catalog, SourceRegister), String> {
    schemas.validate(SchemaKind::Catalog, "catalog.json", &catalog_value)?;
    schemas.validate(SchemaKind::Sources, "sources.json", &sources_value)?;
    let catalog: Catalog = serde_json::from_value(catalog_value)
        .map_err(|error| format!("cannot deserialize catalog.json: {error}"))?;
    let sources: SourceRegister = serde_json::from_value(sources_value)
        .map_err(|error| format!("cannot deserialize sources.json: {error}"))?;
    validate_registers(repository, &catalog, &sources)?;
    Ok((catalog, sources))
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
        if matches!(document.kind.as_str(), "foundation" | "pattern")
            && document.applicability != ["general"]
        {
            return Err(format!(
                "{} must have only ecosystem-neutral general applicability",
                document.id
            ));
        }
        let stack_tags = [
            "cargo",
            "javascript",
            "nodejs",
            "pnpm",
            "rust",
            "typescript",
        ];
        if document.applicability.iter().any(|tag| tag == "general")
            && document
                .applicability
                .iter()
                .any(|tag| stack_tags.contains(&tag.as_str()))
        {
            return Err(format!(
                "{} mixes general and stack-specific applicability",
                document.id
            ));
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

fn validate_structured_trials(
    repository: &Path,
    schemas: &CompiledSchemas,
    catalog: &Catalog,
) -> Result<(), String> {
    let routing_value = read_value(&repository.join(ROUTING_PATH))?;
    let rubric_value = read_value(&repository.join(RUBRIC_PATH))?;
    let trials_value = read_value(&repository.join(FORWARD_TRIALS_PATH))?;
    validate_routing_value(schemas, catalog, routing_value)?;
    validate_forward_trial_values(repository, schemas, catalog, trials_value, rubric_value)?;
    Ok(())
}

pub fn validate_routing_value(
    schemas: &CompiledSchemas,
    catalog: &Catalog,
    value: Value,
) -> Result<(), String> {
    schemas.validate(SchemaKind::Routing, ROUTING_PATH, &value)?;
    let routing: RoutingRegistry = serde_json::from_value(value)
        .map_err(|error| format!("cannot deserialize {ROUTING_PATH}: {error}"))?;
    validate_routing_semantics(&routing, catalog)
}

pub fn validate_forward_trial_values(
    repository: &Path,
    schemas: &CompiledSchemas,
    catalog: &Catalog,
    trials_value: Value,
    rubric_value: Value,
) -> Result<(), String> {
    schemas.validate(SchemaKind::Rubric, RUBRIC_PATH, &rubric_value)?;
    let rubric: Rubric = serde_json::from_value(rubric_value)
        .map_err(|error| format!("cannot deserialize {RUBRIC_PATH}: {error}"))?;
    let mut criterion_ids = BTreeSet::new();
    for criterion in &rubric.criteria {
        if !criterion_ids.insert(criterion.id.as_str()) {
            return Err(format!("duplicate rubric criterion {}", criterion.id));
        }
    }

    schemas.validate(
        SchemaKind::ForwardTrials,
        FORWARD_TRIALS_PATH,
        &trials_value,
    )?;
    let trials: ForwardTrialRegistry = serde_json::from_value(trials_value)
        .map_err(|error| format!("cannot deserialize {FORWARD_TRIALS_PATH}: {error}"))?;
    validate_forward_trial_semantics(repository, &trials, &criterion_ids, catalog)
}

fn validate_routing_semantics(routing: &RoutingRegistry, catalog: &Catalog) -> Result<(), String> {
    let ids = catalog
        .documents
        .iter()
        .map(|document| document.id.as_str())
        .collect::<BTreeSet<_>>();
    let mut case_ids = BTreeSet::new();
    let mut rust_coverage = false;
    let mut javascript_coverage = false;
    for case in &routing.cases {
        if !case_ids.insert(case.id.as_str()) {
            return Err(format!("duplicate routing case {}", case.id));
        }
        let expected = case
            .expected_document_ids
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let rejected = case
            .rejected_document_ids
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if let Some(overlap) = expected.intersection(&rejected).next() {
            return Err(format!(
                "routing case {} both expects and rejects {overlap}",
                case.id
            ));
        }
        for document in expected.iter().chain(rejected.iter()) {
            if !ids.contains(document) {
                return Err(format!(
                    "routing case {} uses unknown document {document}",
                    case.id
                ));
            }
        }
        rust_coverage |= expected.contains("language.rust")
            && !expected.iter().any(|id| {
                id.starts_with("language.javascript") || id.starts_with("language.typescript")
            });
        javascript_coverage |= expected.contains("language.javascript")
            && expected.contains("language.typescript")
            && rejected.contains("language.rust");
    }
    if !rust_coverage || !javascript_coverage {
        return Err(
            "positive routing cases must independently cover Rust and JavaScript/TypeScript".into(),
        );
    }
    Ok(())
}

fn validate_forward_trial_semantics(
    repository: &Path,
    trials: &ForwardTrialRegistry,
    criterion_ids: &BTreeSet<&str>,
    catalog: &Catalog,
) -> Result<(), String> {
    let ids = catalog
        .documents
        .iter()
        .map(|document| document.id.as_str())
        .collect::<BTreeSet<_>>();
    let required = REQUIRED_BEHAVIORS.into_iter().collect::<BTreeSet<_>>();
    let declared = trials
        .required_behaviors
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    if declared != required {
        return Err("forward trials declare an incomplete required behavior set".into());
    }
    let rubric = safe_relative(&trials.rubric)?;
    if !repository.join("evals").join(rubric).is_file() {
        return Err("forward-trial rubric path does not resolve inside evals".into());
    }

    let mut scenario_ids = BTreeSet::new();
    let mut covered_behaviors = BTreeSet::new();
    let mut rust_coverage = false;
    let mut javascript_coverage = false;
    for scenario in &trials.scenarios {
        if !scenario_ids.insert(scenario.id.as_str()) {
            return Err(format!("duplicate forward-trial scenario {}", scenario.id));
        }
        let prompt = safe_relative(&scenario.prompt)?;
        if !repository
            .join("evals/scenarios/shared-knowledge")
            .join(prompt)
            .is_file()
        {
            return Err(format!("scenario {} prompt is missing", scenario.id));
        }
        let expected = scenario
            .expected_document_ids
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let rejected = scenario
            .rejected_document_ids
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if let Some(overlap) = expected.intersection(&rejected).next() {
            return Err(format!(
                "scenario {} both expects and rejects {overlap}",
                scenario.id
            ));
        }
        for document in expected.iter().chain(rejected.iter()) {
            if !ids.contains(document) {
                return Err(format!(
                    "scenario {} uses unknown document {document}",
                    scenario.id
                ));
            }
        }
        for behavior in &scenario.behaviors {
            covered_behaviors.insert(behavior.as_str());
        }
        for criterion in &scenario.rubric_criteria {
            if !criterion_ids.contains(criterion.as_str()) {
                return Err(format!(
                    "scenario {} uses unknown rubric criterion {criterion}",
                    scenario.id
                ));
            }
        }
        rust_coverage |=
            expected.contains("language.rust") && rejected.contains("language.javascript");
        javascript_coverage |= expected.contains("language.javascript")
            && expected.contains("language.typescript")
            && rejected.contains("language.rust");
    }
    if covered_behaviors != required {
        return Err(
            "forward-trial scenarios do not cover every required reasoning behavior".into(),
        );
    }
    if !rust_coverage || !javascript_coverage {
        return Err(
            "forward trials must independently cover Rust and JavaScript/TypeScript".into(),
        );
    }
    Ok(())
}

fn validate_recipes(
    repository: &Path,
    schemas: &CompiledSchemas,
    catalog: &Catalog,
) -> Result<(), String> {
    for recipe in catalog
        .documents
        .iter()
        .filter(|document| document.kind == "recipe")
    {
        let markdown = fs::read_to_string(repository.join(CANONICAL_ROOT).join(&recipe.path))
            .map_err(|error| format!("cannot read recipe {}: {error}", recipe.path))?;
        let authored = extract_recipe_json_text(&markdown)?;
        let value: Value = serde_json::from_str(authored)
            .map_err(|error| format!("invalid recipe JSON in {}: {error}", recipe.path))?;
        schemas.validate(SchemaKind::Config, &recipe.path, &value)?;
        validate_recipe_with_ordinary_loader(recipe, authored, &value)?;
    }
    Ok(())
}

fn validate_recipe_with_ordinary_loader(
    recipe: &Document,
    authored: &str,
    value: &Value,
) -> Result<(), String> {
    let temporary = tempfile::tempdir()
        .map_err(|error| format!("cannot create temporary recipe workspace: {error}"))?;
    let workspace = temporary.path().join("workspace");
    let config_directory = workspace.join(".validation");
    fs::create_dir_all(&config_directory)
        .map_err(|error| format!("cannot create recipe config directory: {error}"))?;
    let config_path = config_directory.join("config.json");
    fs::write(&config_path, authored.as_bytes())
        .map_err(|error| format!("cannot write authored recipe config: {error}"))?;

    let workspace_root = value["workspaceRoot"]
        .as_str()
        .ok_or_else(|| format!("recipe {} has no workspaceRoot", recipe.id))?;
    let resolved_root = match workspace_root {
        "." => config_directory.clone(),
        ".." => workspace.clone(),
        _ => {
            return Err(format!(
                "recipe {} must use a deterministic . or .. workspaceRoot",
                recipe.id
            ))
        }
    };
    for suite in value["suites"]
        .as_array()
        .ok_or("recipe suites must be an array")?
    {
        let directory = suite["workingDirectory"].as_str().unwrap_or(".");
        let relative = safe_directory_shape(directory)?;
        fs::create_dir_all(resolved_root.join(relative))
            .map_err(|error| format!("cannot create recipe suite directory: {error}"))?;
    }
    workspace_validator::config::load(Some(&config_path), &workspace).map_err(|error| {
        format!(
            "recipe {} fails ordinary configuration validation: {error}",
            recipe.id
        )
    })?;
    Ok(())
}

fn safe_directory_shape(value: &str) -> Result<PathBuf, String> {
    let path = Path::new(value);
    if path.is_absolute()
        || !path
            .components()
            .all(|component| matches!(component, Component::Normal(_) | Component::CurDir))
    {
        return Err(format!("unsafe recipe directory {value}"));
    }
    Ok(path.to_path_buf())
}

pub fn extract_recipe_json_text(markdown: &str) -> Result<&str, String> {
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
    Ok(&markdown[start..end])
}

pub fn extract_recipe_json(markdown: &str) -> Result<Value, String> {
    serde_json::from_str(extract_recipe_json_text(markdown)?)
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
    if !paths.contains("catalog.json") {
        return Err("embedded asset inventory omits catalog.json".into());
    }
    for document in &catalog.documents {
        if !paths.contains(document.path.as_str()) {
            return Err(format!(
                "catalog document {} has no embedded asset at {}",
                document.id, document.path
            ));
        }
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
                "canonical knowledge cannot contain symlink {}",
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
    fn embedded_inventory_rejects_unsafe_paths_and_missing_documents() {
        let catalog = load_catalog(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
        let mut assets = embedded_source_assets(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
        assets.push(("../escape.md".into(), b"unsafe".to_vec()));
        assert!(validate_embedded_asset_inventory(&catalog, &assets).is_err());

        let mut assets = embedded_source_assets(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
        let missing = &catalog.documents[0].path;
        assets.retain(|(path, _)| path != missing);
        assert!(validate_embedded_asset_inventory(&catalog, &assets).is_err());
    }
}
