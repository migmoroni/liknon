//! Recipe extraction and validation through the ordinary configuration loader.

use super::{
    contracts::{Catalog, CompiledSchemas, Document, SchemaKind},
    CANONICAL_ROOT,
};
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};
use serde_json::Value;
use std::{
    fs,
    path::{Component, Path, PathBuf},
};

pub(super) fn validate_recipes(
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
    let mut blocks = Vec::new();
    let mut current = None;

    for event in Parser::new(markdown) {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info)))
                if info.split_whitespace().next() == Some("json") =>
            {
                current = Some(String::new());
            }
            Event::End(TagEnd::CodeBlock) => {
                if let Some(block) = current.take() {
                    blocks.push(block);
                }
            }
            Event::Text(text) | Event::Code(text) if current.is_some() => {
                current.as_mut().unwrap().push_str(&text);
            }
            Event::SoftBreak | Event::HardBreak if current.is_some() => {
                current.as_mut().unwrap().push('\n');
            }
            _ => {}
        }
    }

    if blocks.is_empty() {
        return Err("recipe has no JSON example".into());
    }
    if blocks.len() != 1 {
        return Err("recipe must contain exactly one fenced JSON example".into());
    }
    Ok(blocks.pop().unwrap().trim_end_matches('\n').to_owned())
}

pub fn extract_recipe_json(markdown: &str) -> Result<Value, String> {
    serde_json::from_str(&extract_recipe_json_text(markdown)?)
        .map_err(|error| format!("invalid recipe JSON: {error}"))
}
