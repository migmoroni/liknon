//! Schemas and serialized contracts consumed by the release validator.

use super::integrity::read_json;
use serde::Deserialize;
use serde_json::Value;
use std::path::Path;

pub struct CompiledSchemas {
    catalog: jsonschema::Validator,
    sources: jsonschema::Validator,
    config: jsonschema::Validator,
}

#[derive(Clone, Copy)]
pub enum SchemaKind {
    Catalog,
    Sources,
    Config,
}

impl CompiledSchemas {
    pub fn compile(repository: &Path) -> Result<Self, String> {
        Ok(Self {
            catalog: compile_schema(repository, "schemas/knowledge-catalog.schema.json")?,
            sources: compile_schema(repository, "schemas/knowledge-sources.schema.json")?,
            config: compile_schema(repository, "schemas/config.schema.json")?,
        })
    }

    pub fn validate(&self, kind: SchemaKind, name: &str, instance: &Value) -> Result<(), String> {
        let validator = match kind {
            SchemaKind::Catalog => &self.catalog,
            SchemaKind::Sources => &self.sources,
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
