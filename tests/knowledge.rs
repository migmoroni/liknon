#[path = "../tools/knowledge.rs"]
mod knowledge;

use std::{fs, path::Path};
use workspace_validator::contracts::config::Config;

fn repository() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn value(path: &str) -> serde_json::Value {
    serde_json::from_str(&fs::read_to_string(repository().join(path)).unwrap()).unwrap()
}

#[test]
fn canonical_knowledge_and_embedded_source_inventory_are_current() {
    let first = knowledge::verify(repository()).expect("knowledge contract must be valid");
    let second = knowledge::verify(repository()).expect("repeated verification must be valid");
    assert_eq!(first, second, "tree digest must be deterministic");
    assert_eq!(first.len(), 64);
}

#[test]
fn every_recipe_contains_one_complete_configuration_validated_by_the_release_path() {
    knowledge::verify(repository()).expect("release validation includes every recipe");
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
fn source_package_manifest_includes_build_canonical_skill_and_trial_assets() {
    let manifest = fs::read_to_string(repository().join("Cargo.toml")).unwrap();
    for path in [
        "\"/build.rs\"",
        "\"/docs/validation/**\"",
        "\"/skills/**\"",
        "\"/schemas/**\"",
        "\"/evals/**\"",
        "\"/fixtures/knowledge/**\"",
        "\"/tools/knowledge.rs\"",
    ] {
        assert!(manifest.contains(path), "package include omits {path}");
    }
    let normal_dependencies = manifest
        .split("[dependencies]")
        .nth(1)
        .unwrap()
        .split("[target.")
        .next()
        .unwrap();
    let development_dependencies = manifest.split("[dev-dependencies]").nth(1).unwrap();
    assert!(!normal_dependencies.contains("jsonschema"));
    assert!(!normal_dependencies.contains("pulldown-cmark"));
    assert!(development_dependencies.contains("jsonschema"));
    assert!(development_dependencies.contains("pulldown-cmark"));
    assert!(repository()
        .join("schemas/knowledge-editorial-profiles.schema.json")
        .is_file());
    assert!(repository()
        .join("docs/validation/authoring/editorial-profiles.json")
        .is_file());
    assert!(!repository()
        .join("skills/workspace-validator/references/knowledge")
        .exists());
}

#[test]
fn registry_contracts_reject_missing_and_unknown_fields() {
    let schemas = knowledge::CompiledSchemas::compile(repository()).unwrap();
    for (kind, path) in [
        (
            knowledge::SchemaKind::Catalog,
            "docs/validation/knowledge/catalog.json",
        ),
        (
            knowledge::SchemaKind::Sources,
            "docs/validation/knowledge/sources.json",
        ),
        (
            knowledge::SchemaKind::Profiles,
            "docs/validation/authoring/editorial-profiles.json",
        ),
        (
            knowledge::SchemaKind::Routing,
            "fixtures/knowledge/routing.json",
        ),
        (
            knowledge::SchemaKind::ForwardTrials,
            "evals/scenarios/shared-knowledge/scenarios.json",
        ),
        (
            knowledge::SchemaKind::Rubric,
            "evals/rubrics/shared-knowledge.json",
        ),
    ] {
        let mut asset = value(path);
        asset["unknown"] = true.into();
        assert!(
            schemas.validate(kind, path, &asset).is_err(),
            "accepted unknown field in {path}"
        );
    }

    let mut sources = value("docs/validation/knowledge/sources.json");
    sources.as_object_mut().unwrap().remove("sources");
    assert!(schemas
        .validate(knowledge::SchemaKind::Sources, "sources", &sources)
        .is_err());

    let mut profiles = value("docs/validation/authoring/editorial-profiles.json");
    profiles.as_object_mut().unwrap().remove("profiles");
    assert!(schemas
        .validate(knowledge::SchemaKind::Profiles, "profiles", &profiles)
        .is_err());
}

#[test]
fn compiled_schemas_reject_enum_id_date_unique_path_and_config_failures() {
    let schemas = knowledge::CompiledSchemas::compile(repository()).unwrap();

    let mut catalog = value("docs/validation/knowledge/catalog.json");
    catalog["documents"][0]["kind"] = "unknown".into();
    assert!(schemas
        .validate(knowledge::SchemaKind::Catalog, "catalog enum", &catalog)
        .is_err());

    let mut catalog = value("docs/validation/knowledge/catalog.json");
    catalog["documents"][0]["id"] = "Malformed ID".into();
    assert!(schemas
        .validate(knowledge::SchemaKind::Catalog, "catalog ID", &catalog)
        .is_err());

    let mut sources = value("docs/validation/knowledge/sources.json");
    sources["sources"][0]["lastReviewed"] = "2026-02-31".into();
    assert!(schemas
        .validate(knowledge::SchemaKind::Sources, "source date", &sources)
        .is_err());

    let mut profiles = value("docs/validation/authoring/editorial-profiles.json");
    profiles["profiles"][0]["kind"] = "unknown".into();
    assert!(schemas
        .validate(knowledge::SchemaKind::Profiles, "profile kind", &profiles)
        .is_err());

    let mut profiles = value("docs/validation/authoring/editorial-profiles.json");
    let duplicate = profiles["profiles"][0].clone();
    profiles["profiles"].as_array_mut().unwrap().push(duplicate);
    assert!(schemas
        .validate(
            knowledge::SchemaKind::Profiles,
            "duplicate profile",
            &profiles
        )
        .is_err());

    let mut catalog = value("docs/validation/knowledge/catalog.json");
    let duplicate = catalog["navigation"][0].clone();
    catalog["navigation"]
        .as_array_mut()
        .unwrap()
        .push(duplicate);
    assert!(schemas
        .validate(
            knowledge::SchemaKind::Catalog,
            "catalog uniqueItems",
            &catalog
        )
        .is_err());

    let mut trials = value("evals/scenarios/shared-knowledge/scenarios.json");
    trials["scenarios"][0]["prompt"] = "../escape.md".into();
    assert!(schemas
        .validate(
            knowledge::SchemaKind::ForwardTrials,
            "unsafe prompt",
            &trials
        )
        .is_err());

    let recipe = fs::read_to_string(
        repository().join("docs/validation/knowledge/how-to/recipes/rust-cli-release-gate.md"),
    )
    .unwrap();
    let mut config = knowledge::extract_recipe_json(&recipe).unwrap();
    config.as_object_mut().unwrap().remove("groups");
    assert!(schemas
        .validate(knowledge::SchemaKind::Config, "recipe config", &config)
        .is_err());
}

#[test]
fn shared_semantic_validators_reject_relationship_overlap_and_duplicate_ids() {
    let schemas = knowledge::CompiledSchemas::compile(repository()).unwrap();

    let mut catalog = value("docs/validation/knowledge/catalog.json");
    catalog["documents"][0]["related"][0] = "missing.document".into();
    let sources = value("docs/validation/knowledge/sources.json");
    assert!(
        knowledge::validate_catalog_source_values(repository(), &schemas, catalog, sources)
            .is_err()
    );

    let catalog = knowledge::load_catalog(repository()).unwrap();
    let mut routing = value("fixtures/knowledge/routing.json");
    let overlap = routing["cases"][0]["expectedDocumentIds"][0].clone();
    routing["cases"][0]["rejectedDocumentIds"]
        .as_array_mut()
        .unwrap()
        .push(overlap);
    assert!(knowledge::validate_routing_value(&schemas, &catalog, routing).is_err());

    let mut routing = value("fixtures/knowledge/routing.json");
    routing["cases"][1]["id"] = routing["cases"][0]["id"].clone();
    assert!(knowledge::validate_routing_value(&schemas, &catalog, routing).is_err());

    let trials = value("evals/scenarios/shared-knowledge/scenarios.json");
    let mut rubric = value("evals/rubrics/shared-knowledge.json");
    rubric["criteria"][1]["id"] = rubric["criteria"][0]["id"].clone();
    assert!(knowledge::validate_forward_trial_values(
        repository(),
        &schemas,
        &catalog,
        trials,
        rubric
    )
    .is_err());
}

#[test]
fn generated_source_index_rejects_stale_content() {
    let sources = knowledge::load_sources(repository()).unwrap();
    let expected = knowledge::render_sources(&sources);
    assert!(knowledge::validate_generated_sources(&expected, &expected).is_ok());
    assert!(knowledge::validate_generated_sources(&expected, "stale").is_err());
}
