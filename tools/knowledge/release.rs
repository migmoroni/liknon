//! Release orchestration and generated source rendering.

use super::{
    contracts::{Catalog, CompiledSchemas, SchemaKind, SourceRegister},
    integrity::{
        embedded_source_assets, read_json, read_value, tree_digest, validate_catalog_integrity,
        validate_embedded_asset_inventory,
    },
    recipes::validate_recipes,
    CANONICAL_ROOT,
};
use serde_json::Value;
use std::{fs, path::Path};

pub fn load_catalog(repository: &Path) -> Result<Catalog, String> {
    read_json(&repository.join(CANONICAL_ROOT).join("catalog.json"))
}

pub fn load_sources(repository: &Path) -> Result<SourceRegister, String> {
    read_json(&repository.join(CANONICAL_ROOT).join("sources.json"))
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
    validate_recipes(repository, &schemas, &catalog)?;
    tree_digest(&repository.join(CANONICAL_ROOT))
}

pub fn validate_generated_sources(expected: &str, actual: &str) -> Result<(), String> {
    // Git may materialize tracked text with CRLF on Windows while generation
    // remains canonical LF. Only that platform representation is equivalent.
    if actual == expected || actual.replace("\r\n", "\n") == expected {
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
    validate_catalog_integrity(repository, &catalog, &sources)?;
    Ok((catalog, sources))
}
