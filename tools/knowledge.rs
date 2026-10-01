#![allow(dead_code)]

use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Parser, Tag, TagEnd};
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    ops::Range,
    path::{Component, Path, PathBuf},
};

pub const CANONICAL_ROOT: &str = "docs/validation/knowledge";
const ROUTING_PATH: &str = "fixtures/knowledge/routing.json";
const FORWARD_TRIALS_PATH: &str = "evals/scenarios/shared-knowledge/scenarios.json";
const RUBRIC_PATH: &str = "evals/rubrics/shared-knowledge.json";
const PROFILES_PATH: &str = "docs/validation/authoring/editorial-profiles.json";
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
    profiles: jsonschema::Validator,
    routing: jsonschema::Validator,
    forward_trials: jsonschema::Validator,
    rubric: jsonschema::Validator,
    config: jsonschema::Validator,
}

#[derive(Clone, Copy)]
pub enum SchemaKind {
    Catalog,
    Sources,
    Profiles,
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
            profiles: compile_schema(
                repository,
                "schemas/knowledge-editorial-profiles.schema.json",
            )?,
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
            SchemaKind::Profiles => &self.profiles,
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
    pub title: String,
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
pub struct EditorialRegistry {
    #[serde(rename = "$schema")]
    schema: String,
    schema_version: u32,
    families: Vec<EditorialFamily>,
    basis_ids: Vec<String>,
    profiles: Vec<EditorialProfile>,
    navigation_profile: NavigationProfile,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct EditorialFamily {
    id: String,
    reader_intent: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EditorialProfile {
    pub kind: String,
    family: String,
    path_prefix: String,
    profile_document: String,
    sections: Vec<EditorialSection>,
    editorial_bases: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct EditorialSection {
    title: String,
    required: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct NavigationProfile {
    profile_document: String,
    editorial_bases: Vec<String>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownHeading {
    pub level: u8,
    pub text: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MarkdownLink {
    label: String,
    target: String,
    line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MarkdownCodeBlock {
    language: Option<String>,
    content: String,
    line: usize,
    closed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedMarkdown {
    headings: Vec<MarkdownHeading>,
    links: Vec<MarkdownLink>,
    code_blocks: Vec<MarkdownCodeBlock>,
    text: String,
}

#[derive(Debug)]
struct OpenHeading {
    level: u8,
    text: String,
    line: usize,
}

#[derive(Debug)]
struct OpenLink {
    label: String,
    target: String,
    line: usize,
}

#[derive(Debug)]
struct OpenCodeBlock {
    language: Option<String>,
    content: String,
    line: usize,
    range: Range<usize>,
}

impl ParsedMarkdown {
    pub fn parse(markdown: &str) -> Result<Self, String> {
        let mut headings = Vec::new();
        let mut links = Vec::new();
        let mut code_blocks = Vec::new();
        let mut plain_text = String::new();
        let mut heading = None;
        let mut link = None;
        let mut code_block = None;

        for (event, range) in Parser::new(markdown).into_offset_iter() {
            let line = line_number(markdown, range.start);
            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    heading = Some(OpenHeading {
                        level: heading_level(level),
                        text: String::new(),
                        line,
                    });
                }
                Event::End(TagEnd::Heading(_)) => {
                    let value = heading.take().ok_or("unbalanced Markdown heading")?;
                    headings.push(MarkdownHeading {
                        level: value.level,
                        text: value.text.trim().to_owned(),
                        line: value.line,
                    });
                }
                Event::Start(Tag::Link { dest_url, .. }) => {
                    link = Some(OpenLink {
                        label: String::new(),
                        target: dest_url.into_string(),
                        line,
                    });
                }
                Event::End(TagEnd::Link) => {
                    let value = link.take().ok_or("unbalanced Markdown link")?;
                    links.push(MarkdownLink {
                        label: value.label.trim().to_owned(),
                        target: value.target,
                        line: value.line,
                    });
                }
                Event::Start(Tag::CodeBlock(kind)) => {
                    let language = match kind {
                        CodeBlockKind::Fenced(info) => info
                            .split_ascii_whitespace()
                            .next()
                            .filter(|value| !value.is_empty())
                            .map(str::to_owned),
                        CodeBlockKind::Indented => None,
                    };
                    code_block = Some(OpenCodeBlock {
                        language,
                        content: String::new(),
                        line,
                        range,
                    });
                }
                Event::End(TagEnd::CodeBlock) => {
                    let mut value = code_block.take().ok_or("unbalanced Markdown code block")?;
                    value.range.end = range.end;
                    code_blocks.push(MarkdownCodeBlock {
                        language: value.language,
                        content: value.content,
                        line: value.line,
                        closed: fenced_block_is_closed(markdown, &value.range),
                    });
                }
                Event::Text(text) | Event::Code(text) => {
                    if let Some(value) = code_block.as_mut() {
                        value.content.push_str(&text);
                    } else {
                        plain_text.push_str(&text);
                        plain_text.push('\n');
                        if let Some(value) = heading.as_mut() {
                            value.text.push_str(&text);
                        }
                        if let Some(value) = link.as_mut() {
                            value.label.push_str(&text);
                        }
                    }
                }
                Event::SoftBreak | Event::HardBreak => {
                    if let Some(value) = heading.as_mut() {
                        value.text.push(' ');
                    }
                    if let Some(value) = link.as_mut() {
                        value.label.push(' ');
                    }
                    if let Some(value) = code_block.as_mut() {
                        value.content.push('\n');
                    }
                }
                _ => {}
            }
        }
        if heading.is_some() || link.is_some() || code_block.is_some() {
            return Err("unbalanced Markdown structure".into());
        }
        Ok(Self {
            headings,
            links,
            code_blocks,
            text: plain_text,
        })
    }
}

fn heading_level(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

fn line_number(markdown: &str, offset: usize) -> usize {
    markdown.as_bytes()[..offset]
        .iter()
        .filter(|byte| **byte == b'\n')
        .count()
        + 1
}

fn fenced_block_is_closed(markdown: &str, range: &Range<usize>) -> bool {
    let source = &markdown[range.clone()];
    let Some(first) = source.lines().next() else {
        return false;
    };
    let marker = first.trim_start().chars().next().unwrap_or(' ');
    if !matches!(marker, '`' | '~') {
        return true;
    }
    source
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .is_some_and(|line| line.trim_start().starts_with(&marker.to_string().repeat(3)))
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

pub fn load_editorial_registry(repository: &Path) -> Result<EditorialRegistry, String> {
    read_json(&repository.join(PROFILES_PATH))
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
    let profiles_value = read_value(&repository.join(PROFILES_PATH))?;
    schemas.validate(SchemaKind::Profiles, PROFILES_PATH, &profiles_value)?;
    let profiles: EditorialRegistry = serde_json::from_value(profiles_value)
        .map_err(|error| format!("cannot deserialize {PROFILES_PATH}: {error}"))?;
    validate_editorial_registry(repository, &profiles)?;
    validate_registers(repository, &catalog, &sources, &profiles)?;
    Ok((catalog, sources))
}

fn validate_editorial_registry(
    repository: &Path,
    registry: &EditorialRegistry,
) -> Result<(), String> {
    if registry.schema_version != 1
        || registry.schema != "../../../schemas/knowledge-editorial-profiles.schema.json"
    {
        return Err("editorial profile schema contract is unsupported".into());
    }
    let expected_families = ["explanation", "how-to", "reference"]
        .into_iter()
        .collect::<BTreeSet<_>>();
    let families = registry
        .families
        .iter()
        .map(|family| family.id.as_str())
        .collect::<BTreeSet<_>>();
    if families != expected_families
        || registry
            .families
            .iter()
            .any(|family| family.reader_intent.trim().is_empty())
    {
        return Err("editorial families must define the three supported reader intents".into());
    }
    let basis_ids = registry
        .basis_ids
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    if basis_ids.len() != registry.basis_ids.len() {
        return Err("editorial basis IDs must be unique".into());
    }

    let mut kinds = BTreeSet::new();
    let mut documents = BTreeSet::new();
    for profile in &registry.profiles {
        if !kinds.insert(profile.kind.as_str())
            || !families.contains(profile.family.as_str())
            || !documents.insert(profile.profile_document.as_str())
            || profile.sections.is_empty()
            || profile
                .sections
                .iter()
                .map(|section| section.title.as_str())
                .collect::<BTreeSet<_>>()
                .len()
                != profile.sections.len()
            || profile
                .editorial_bases
                .iter()
                .any(|basis| !basis_ids.contains(basis.as_str()))
        {
            return Err(format!(
                "editorial profile {} is inconsistent",
                profile.kind
            ));
        }
        validate_profile_document(repository, profile)?;
    }
    let expected_kinds = [
        "foundation",
        "pattern",
        "concern",
        "language",
        "technology",
        "framework",
        "recipe",
        "tool",
        "standard",
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    if kinds != expected_kinds {
        return Err("editorial registry does not map every supported catalog kind once".into());
    }
    if registry
        .navigation_profile
        .editorial_bases
        .iter()
        .any(|basis| !basis_ids.contains(basis.as_str()))
    {
        return Err("navigation profile uses an unknown editorial basis".into());
    }
    let navigation = EditorialProfile {
        kind: "navigation".into(),
        family: "navigation".into(),
        path_prefix: String::new(),
        profile_document: registry.navigation_profile.profile_document.clone(),
        sections: Vec::new(),
        editorial_bases: registry.navigation_profile.editorial_bases.clone(),
    };
    validate_profile_document(repository, &navigation)?;

    let profile_root = repository.join("docs/validation/authoring/profiles");
    let actual = collect_files(&profile_root)?
        .into_iter()
        .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("md"))
        .map(|path| format!("profiles/{}", slash_path(&path)))
        .collect::<BTreeSet<_>>();
    let mut declared = documents
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    declared.insert(registry.navigation_profile.profile_document.clone());
    if actual != declared {
        return Err(format!(
            "profile Markdown inventory differs from registry: actual {actual:?}, declared {declared:?}"
        ));
    }
    Ok(())
}

fn validate_profile_document(repository: &Path, profile: &EditorialProfile) -> Result<(), String> {
    let path = repository
        .join("docs/validation/authoring")
        .join(safe_relative(&profile.profile_document)?);
    let markdown = fs::read_to_string(&path)
        .map_err(|error| format!("cannot read profile {}: {error}", path.display()))?;
    let parsed = ParsedMarkdown::parse(&markdown)
        .map_err(|error| format!("cannot parse profile {}: {error}", path.display()))?;
    let named_bases = profile
        .editorial_bases
        .iter()
        .filter(|basis| parsed.text.lines().any(|line| line == basis.as_str()))
        .count();
    if !parsed.text.lines().any(|line| line == profile.kind)
        || !parsed.text.lines().any(|line| line == profile.family)
        || named_bases != profile.editorial_bases.len()
    {
        return Err(format!(
            "profile Markdown {} disagrees with kind {}, family {}, or editorial bases",
            profile.profile_document, profile.kind, profile.family
        ));
    }
    Ok(())
}

fn validate_registers(
    repository: &Path,
    catalog: &Catalog,
    register: &SourceRegister,
    profiles: &EditorialRegistry,
) -> Result<(), String> {
    if catalog.schema_version != 2
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
    let mut previous_source_id = None;
    for source in &register.sources {
        if previous_source_id.is_some_and(|previous| previous >= source.id.as_str()) {
            return Err("source records must be ordered by stable ID".into());
        }
        previous_source_id = Some(source.id.as_str());
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
    let profiles_by_kind = profiles
        .profiles
        .iter()
        .map(|profile| (profile.kind.as_str(), profile))
        .collect::<BTreeMap<_, _>>();
    let mut document_ids = BTreeSet::new();
    let mut document_paths = BTreeSet::new();
    let mut previous_document_id = None;
    for document in &catalog.documents {
        if previous_document_id.is_some_and(|previous| previous >= document.id.as_str()) {
            return Err("catalog documents must be ordered by stable ID".into());
        }
        previous_document_id = Some(document.id.as_str());
        let profile = profiles_by_kind
            .get(document.kind.as_str())
            .ok_or_else(|| format!("document {} has no editorial profile", document.id))?;
        let expected_prefix = format!("{}.", document.kind);
        if !valid_dot_id(&document.id)
            || !document.id.starts_with(&expected_prefix)
            || !document_ids.insert(document.id.as_str())
            || !document_paths.insert(document.path.as_str())
        {
            return Err(format!("invalid or duplicate document {}", document.id));
        }
        if !document.path.starts_with(&profile.path_prefix) {
            return Err(format!(
                "document {} path {} violates {} profile prefix {}",
                document.id, document.path, profile.kind, profile.path_prefix
            ));
        }
        if document.kind == "tool"
            && (contains_patch_version(&document.title)
                || contains_patch_version(&document.summary))
        {
            return Err(format!(
                "tool {} uses a patch-only public title or summary",
                document.id
            ));
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
            || document.title.trim().is_empty()
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
        let parsed = ParsedMarkdown::parse(&markdown)
            .map_err(|error| document_diagnostic(document, profile, None, &error))?;
        validate_document_structure(document, profile, &parsed)?;
        let cited = validate_citations(document, &parsed, &locations)?;
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
        validate_links(&root, &relative, &parsed)?;
        let index = relative.parent().unwrap_or(Path::new("")).join("README.md");
        let index_text = fs::read_to_string(root.join(&index))
            .map_err(|error| format!("cannot read category index: {error}"))?;
        let index_document = ParsedMarkdown::parse(&index_text)
            .map_err(|error| format!("cannot parse navigation {}: {error}", index.display()))?;
        let file_name = relative.file_name().unwrap().to_string_lossy();
        if !index_document
            .links
            .iter()
            .any(|link| link.target == file_name)
        {
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

    validate_navigation_indexes(&root, &catalog.navigation)?;

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
            let parsed = ParsedMarkdown::parse(&markdown)
                .map_err(|error| format!("cannot parse {}: {error}", path.display()))?;
            validate_links(&root, &path, &parsed)?;
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

fn validate_document_structure(
    document: &Document,
    profile: &EditorialProfile,
    markdown: &ParsedMarkdown,
) -> Result<(), String> {
    if let Some(block) = markdown.code_blocks.iter().find(|block| !block.closed) {
        return Err(document_diagnostic(
            document,
            profile,
            None,
            &format!("unclosed fenced code block at line {}", block.line),
        ));
    }
    if let Some(block) = markdown
        .code_blocks
        .iter()
        .find(|block| block.language.is_none())
    {
        return Err(document_diagnostic(
            document,
            profile,
            None,
            &format!("code block at line {} has no language", block.line),
        ));
    }
    let h1 = markdown
        .headings
        .iter()
        .filter(|heading| heading.level == 1)
        .collect::<Vec<_>>();
    if h1.len() != 1 || h1[0].text != document.title {
        return Err(document_diagnostic(
            document,
            profile,
            None,
            &format!(
                "expected exactly one H1 titled {:?}; found {:?}",
                document.title,
                h1.iter().map(|heading| &heading.text).collect::<Vec<_>>()
            ),
        ));
    }
    let mut previous_level = 0;
    for heading in &markdown.headings {
        if heading.level > previous_level + 1 {
            return Err(document_diagnostic(
                document,
                profile,
                Some(&heading.text),
                &format!(
                    "heading level skips from H{previous_level} to H{} at line {}",
                    heading.level, heading.line
                ),
            ));
        }
        previous_level = heading.level;
    }

    let actual = markdown
        .headings
        .iter()
        .filter(|heading| heading.level == 2)
        .collect::<Vec<_>>();
    let mut seen = BTreeSet::new();
    let mut cursor = 0;
    for heading in actual {
        if !seen.insert(heading.text.as_str()) {
            return Err(document_diagnostic(
                document,
                profile,
                Some(&heading.text),
                "duplicate H2 section",
            ));
        }
        let Some(position) = profile.sections[cursor..]
            .iter()
            .position(|section| section.title == heading.text)
            .map(|position| position + cursor)
        else {
            return Err(document_diagnostic(
                document,
                profile,
                Some(&heading.text),
                "unknown or out-of-order H2 section",
            ));
        };
        if profile.sections[cursor..position]
            .iter()
            .any(|section| section.required)
        {
            return Err(document_diagnostic(
                document,
                profile,
                Some(&heading.text),
                "a required earlier H2 section is missing",
            ));
        }
        cursor = position + 1;
    }
    if let Some(missing) = profile.sections[cursor..]
        .iter()
        .find(|section| section.required)
    {
        return Err(document_diagnostic(
            document,
            profile,
            Some(&missing.title),
            "required H2 section is missing",
        ));
    }
    Ok(())
}

fn document_diagnostic(
    document: &Document,
    profile: &EditorialProfile,
    section: Option<&str>,
    violation: &str,
) -> String {
    let expected = profile
        .sections
        .iter()
        .map(|section| {
            if section.required {
                section.title.clone()
            } else {
                format!("{} (optional)", section.title)
            }
        })
        .collect::<Vec<_>>()
        .join(" -> ");
    format!(
        "document {} at {} violates profile {}{}: {}; expected H2 structure: {}",
        document.id,
        document.path,
        profile.kind,
        section
            .map(|value| format!(" section {value:?}"))
            .unwrap_or_default(),
        violation,
        expected
    )
}

fn validate_citations<'a>(
    document: &Document,
    markdown: &'a ParsedMarkdown,
    locations: &BTreeMap<String, &str>,
) -> Result<BTreeSet<&'a str>, String> {
    let mut cited = BTreeSet::new();
    for link in markdown
        .links
        .iter()
        .filter(|link| link.label.starts_with("Source: "))
    {
        let label = link.label.trim_start_matches("Source: ");
        let expected = locations.get(label).ok_or_else(|| {
            format!(
                "unknown source location {label} in {} at line {}",
                document.path, link.line
            )
        })?;
        if link.target != *expected {
            return Err(format!(
                "citation {label} in {} at line {} does not use its registered URI",
                document.path, link.line
            ));
        }
        cited.insert(label.split_once('#').unwrap().0);
    }
    if cited.is_empty() {
        return Err(format!(
            "document {} has no source-location citation",
            document.id
        ));
    }
    Ok(cited)
}

fn validate_links(root: &Path, relative: &Path, markdown: &ParsedMarkdown) -> Result<(), String> {
    for link in &markdown.links {
        let target = link.target.as_str();
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
            return Err(format!(
                "broken link {target} in {} at line {}",
                relative.display(),
                link.line
            ));
        }
    }
    Ok(())
}

fn validate_navigation_indexes(root: &Path, navigation: &[String]) -> Result<(), String> {
    for value in navigation
        .iter()
        .filter(|value| value.ends_with("README.md"))
    {
        let relative = safe_relative(value)?;
        let markdown = fs::read_to_string(root.join(&relative))
            .map_err(|error| format!("cannot read navigation {value}: {error}"))?;
        let parsed = ParsedMarkdown::parse(&markdown)
            .map_err(|error| format!("cannot parse navigation {value}: {error}"))?;
        let h1_count = parsed
            .headings
            .iter()
            .filter(|heading| heading.level == 1)
            .count();
        if h1_count != 1 {
            return Err(format!("navigation {value} must contain exactly one H1"));
        }
        let parent = relative.parent().unwrap_or(Path::new(""));
        let mut expected = Vec::new();
        let mut entries = fs::read_dir(root.join(parent))
            .map_err(|error| format!("cannot enumerate navigation {value}: {error}"))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("cannot enumerate navigation {value}: {error}"))?;
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let name = entry.file_name().to_string_lossy().into_owned();
            let kind = entry
                .file_type()
                .map_err(|error| format!("cannot inspect navigation child: {error}"))?;
            if kind.is_dir() && entry.path().join("README.md").is_file() {
                expected.push(format!("{name}/README.md"));
            } else if kind.is_file()
                && name.ends_with(".md")
                && name != "README.md"
                && name != "SOURCES.md"
            {
                expected.push(name);
            }
        }
        let listed = parsed
            .links
            .iter()
            .filter_map(|link| {
                expected
                    .contains(&link.target)
                    .then_some(link.target.clone())
            })
            .collect::<Vec<_>>();
        if listed != expected {
            return Err(format!(
                "navigation {value} must list direct children in order: {expected:?}; found {listed:?}"
            ));
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
        let value: Value = serde_json::from_str(&authored)
            .map_err(|error| format!("invalid recipe JSON in {}: {error}", recipe.path))?;
        schemas.validate(SchemaKind::Config, &recipe.path, &value)?;
        validate_recipe_with_ordinary_loader(recipe, &authored, &value)?;
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

pub fn extract_recipe_json_text(markdown: &str) -> Result<String, String> {
    let parsed = ParsedMarkdown::parse(markdown)?;
    let json = parsed
        .code_blocks
        .iter()
        .filter(|block| block.language.as_deref() == Some("json"))
        .collect::<Vec<_>>();
    if json.is_empty() {
        return Err("recipe has no JSON example".into());
    }
    if json.len() != 1 {
        return Err("recipe must contain exactly one complete JSON example".into());
    }
    if !json[0].closed {
        return Err(format!(
            "recipe JSON fence at line {} is not closed",
            json[0].line
        ));
    }
    Ok(json[0].content.trim_end_matches('\n').to_owned())
}

pub fn extract_recipe_json(markdown: &str) -> Result<Value, String> {
    serde_json::from_str(&extract_recipe_json_text(markdown)?)
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

fn contains_patch_version(value: &str) -> bool {
    value
        .split(|character: char| !(character.is_ascii_digit() || character == '.'))
        .any(|token| {
            let parts = token.split('.').collect::<Vec<_>>();
            parts.len() == 3
                && parts
                    .iter()
                    .all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit()))
        })
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

    fn document() -> Document {
        Document {
            id: "foundation.fixture".into(),
            title: "Fixture".into(),
            kind: "foundation".into(),
            path: "explanations/foundations/fixture.md".into(),
            summary: "Fixture summary".into(),
            questions: vec!["What is the fixture?".into()],
            applicability: vec!["general".into()],
            evidence_dimensions: vec!["strategy".into()],
            related: Vec::new(),
            sources: vec!["source.fixture".into()],
            status: "reviewed".into(),
            last_reviewed: "2026-10-01".into(),
        }
    }

    fn profile() -> EditorialProfile {
        EditorialProfile {
            kind: "foundation".into(),
            family: "explanation".into(),
            path_prefix: "explanations/foundations/".into(),
            profile_document: "profiles/foundation.md".into(),
            sections: vec![
                EditorialSection {
                    title: "Question".into(),
                    required: true,
                },
                EditorialSection {
                    title: "Optional Context".into(),
                    required: false,
                },
                EditorialSection {
                    title: "Evidence And Limitations".into(),
                    required: true,
                },
            ],
            editorial_bases: vec!["diataxis".into()],
        }
    }

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

    #[test]
    fn parsed_markdown_uses_commonmark_structure_offsets_and_exact_fences() {
        let markdown = "# Fixture\n\nText with `## inline` and [nested](guide_(one).md).\n\n```text\n## not a heading\n```\n\n## Question\n\n[Source: source.fixture#claim](https://example.com/a_(b)).\n\n```json\n{\"value\":\"✓\"}\n```\n\n## Evidence And Limitations\n";
        let parsed = ParsedMarkdown::parse(markdown).unwrap();
        assert_eq!(
            parsed
                .headings
                .iter()
                .map(|heading| (heading.level, heading.text.as_str(), heading.line))
                .collect::<Vec<_>>(),
            vec![
                (1, "Fixture", 1),
                (2, "Question", 9),
                (2, "Evidence And Limitations", 17),
            ]
        );
        assert_eq!(parsed.links[0].target, "guide_(one).md");
        assert_eq!(parsed.links[1].label, "Source: source.fixture#claim");
        assert_eq!(parsed.links[1].target, "https://example.com/a_(b)");
        assert_eq!(parsed.code_blocks.len(), 2);
        assert_eq!(parsed.code_blocks[0].language.as_deref(), Some("text"));
        assert_eq!(parsed.code_blocks[1].language.as_deref(), Some("json"));
        assert_eq!(parsed.code_blocks[1].content, "{\"value\":\"✓\"}\n");
        assert!(parsed.code_blocks.iter().all(|block| block.closed));
    }

    #[test]
    fn recipe_parser_rejects_missing_duplicate_malformed_and_unclosed_json() {
        assert!(extract_recipe_json("# Recipe\n").is_err());
        assert!(extract_recipe_json("```json\n{}\n```\n```json\n{}\n```\n").is_err());
        assert!(extract_recipe_json("```json\n{broken}\n```\n").is_err());
        assert!(extract_recipe_json("```json\n{}\n").is_err());
    }

    #[test]
    fn profile_structure_allows_optional_omission_and_rejects_failure_classes() {
        let document = document();
        let profile = profile();
        let valid = ParsedMarkdown::parse(
            "# Fixture\n\n## Question\n\nPurpose.\n\n## Evidence And Limitations\n\nBounded.\n",
        )
        .unwrap();
        validate_document_structure(&document, &profile, &valid).unwrap();

        for markdown in [
            "# Fixture\n\n## Evidence And Limitations\n",
            "# Fixture\n\n## Question\n\n## Unknown\n\n## Evidence And Limitations\n",
            "# Fixture\n\n## Question\n\n## Question\n\n## Evidence And Limitations\n",
            "# Fixture\n\n# Duplicate\n\n## Question\n\n## Evidence And Limitations\n",
            "# Fixture\n\n### Skipped\n\n## Question\n\n## Evidence And Limitations\n",
            "# Wrong Title\n\n## Question\n\n## Evidence And Limitations\n",
            "# Fixture\n\n## Evidence And Limitations\n\n## Question\n",
        ] {
            let parsed = ParsedMarkdown::parse(markdown).unwrap();
            assert!(
                validate_document_structure(&document, &profile, &parsed).is_err(),
                "accepted invalid Markdown:\n{markdown}"
            );
        }
    }

    #[test]
    fn version_identity_detection_rejects_only_patch_shapes() {
        assert!(contains_patch_version("pnpm 12.8.1 reference"));
        assert!(!contains_patch_version("pnpm major lines 11 and 12"));
        assert!(!contains_patch_version("WCAG 2.2"));
    }

    #[test]
    fn canonical_profile_markdown_and_registry_agree() {
        let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
        let registry = load_editorial_registry(repository).unwrap();
        validate_editorial_registry(repository, &registry).unwrap();

        let mut value = read_value(&repository.join(PROFILES_PATH)).unwrap();
        value["profiles"][0]["family"] = "reference".into();
        let registry: EditorialRegistry = serde_json::from_value(value).unwrap();
        assert!(validate_editorial_registry(repository, &registry).is_err());
    }

    #[test]
    fn every_profile_accepts_its_sequence_and_rejects_a_missing_required_section() {
        let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
        let registry = load_editorial_registry(repository).unwrap();
        for profile in &registry.profiles {
            let mut document = document();
            document.id = format!("{}.fixture", profile.kind);
            document.kind = profile.kind.clone();
            document.path = format!("{}fixture.md", profile.path_prefix);

            let mut markdown = String::from("# Fixture\n");
            for section in &profile.sections {
                markdown.push_str(&format!("\n## {}\n\nContent.\n", section.title));
            }
            let parsed = ParsedMarkdown::parse(&markdown).unwrap();
            validate_document_structure(&document, profile, &parsed).unwrap_or_else(|error| {
                panic!("{} positive fixture failed: {error}", profile.kind)
            });

            let first_required = profile
                .sections
                .iter()
                .position(|section| section.required)
                .unwrap();
            let mut missing = String::from("# Fixture\n");
            for (index, section) in profile.sections.iter().enumerate() {
                if index != first_required {
                    missing.push_str(&format!("\n## {}\n\nContent.\n", section.title));
                }
            }
            let parsed = ParsedMarkdown::parse(&missing).unwrap();
            assert!(
                validate_document_structure(&document, profile, &parsed).is_err(),
                "{} accepted a missing required section",
                profile.kind
            );
        }
    }

    #[test]
    fn navigation_validation_rejects_missing_and_out_of_order_children() {
        let temporary = tempfile::tempdir().unwrap();
        let root = temporary.path();
        fs::write(root.join("a.md"), "# A\n").unwrap();
        fs::write(root.join("b.md"), "# B\n").unwrap();
        let navigation = vec!["README.md".to_owned()];

        fs::write(root.join("README.md"), "# Index\n\n1. [B](b.md)\n").unwrap();
        assert!(validate_navigation_indexes(root, &navigation).is_err());

        fs::write(
            root.join("README.md"),
            "# Index\n\n1. [B](b.md)\n2. [A](a.md)\n",
        )
        .unwrap();
        assert!(validate_navigation_indexes(root, &navigation).is_err());

        fs::write(
            root.join("README.md"),
            "# Index\n\n1. [A](a.md)\n2. [B](b.md)\n",
        )
        .unwrap();
        validate_navigation_indexes(root, &navigation).unwrap();
    }
}
