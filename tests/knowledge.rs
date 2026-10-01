#[path = "../tools/knowledge.rs"]
mod knowledge;

use std::{fs, path::Path};
use workspace_validator::contracts::config::Config;

fn repository() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn canonical_knowledge_and_generated_projection_are_current() {
    let first = knowledge::verify(repository()).expect("knowledge contract must be valid");
    let second = knowledge::verify(repository()).expect("repeated verification must be valid");
    assert_eq!(first, second, "tree digest must be deterministic");
    assert_eq!(first.len(), 64);
}

#[test]
fn every_recipe_contains_one_complete_configuration_contract() {
    let catalog = knowledge::load_catalog(repository()).unwrap();
    let recipes = catalog
        .documents
        .iter()
        .filter(|document| document.kind == "recipe")
        .collect::<Vec<_>>();
    assert!(!recipes.is_empty());
    for recipe in recipes {
        let markdown = fs::read_to_string(
            repository()
                .join(knowledge::CANONICAL_ROOT)
                .join(&recipe.path),
        )
        .unwrap();
        let example = knowledge::extract_recipe_json(&markdown).unwrap();
        let parsed: Config = serde_json::from_value(example).unwrap_or_else(|error| {
            panic!("recipe {} is not a complete config: {error}", recipe.id)
        });
        assert!(!parsed.tools.is_empty());
        assert!(!parsed.checks.is_empty());
        assert!(!parsed.suites.is_empty());
        assert!(!parsed.groups.is_empty());
    }
}

#[test]
fn source_package_manifest_includes_canonical_projected_and_trial_assets() {
    let manifest = fs::read_to_string(repository().join("Cargo.toml")).unwrap();
    for path in [
        "\"/docs/validation/**\"",
        "\"/skills/**\"",
        "\"/schemas/**\"",
        "\"/evals/**\"",
        "\"/fixtures/knowledge/**\"",
        "\"/tools/knowledge.rs\"",
    ] {
        assert!(manifest.contains(path), "package include omits {path}");
    }
}

#[test]
fn registry_contracts_reject_missing_and_unknown_fields() {
    let catalog_path = repository()
        .join(knowledge::CANONICAL_ROOT)
        .join("catalog.json");
    let sources_path = repository()
        .join(knowledge::CANONICAL_ROOT)
        .join("sources.json");

    let mut catalog: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(catalog_path).unwrap()).unwrap();
    catalog["unknown"] = true.into();
    assert!(serde_json::from_value::<knowledge::Catalog>(catalog).is_err());

    let mut sources: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(sources_path).unwrap()).unwrap();
    sources.as_object_mut().unwrap().remove("sources");
    assert!(serde_json::from_value::<knowledge::SourceRegister>(sources).is_err());
}
