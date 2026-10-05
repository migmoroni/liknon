use super::{contracts::*, integrity::*, recipes::*, *};
use std::{fs, path::Path};

fn catalog(document_path: &str) -> Catalog {
    Catalog {
        schema: "../../../schemas/knowledge-catalog.schema.json".into(),
        schema_version: 2,
        vocabularies: Vocabularies {
            applicability_tags: vec![VocabularyEntry {
                id: "general".into(),
                label: "General".into(),
            }],
            evidence_dimensions: vec![VocabularyEntry {
                id: "strategy".into(),
                label: "Strategy".into(),
            }],
        },
        navigation: vec!["README.md".into()],
        documents: vec![Document {
            id: "foundation.fixture".into(),
            title: "Fixture".into(),
            kind: "foundation".into(),
            path: document_path.into(),
            summary: "Fixture summary".into(),
            questions: vec!["What is the fixture?".into()],
            applicability: vec!["general".into()],
            evidence_dimensions: vec!["strategy".into()],
            related: Vec::new(),
            sources: vec!["source.fixture".into()],
            status: "draft".into(),
            last_reviewed: "2026-10-01".into(),
        }],
    }
}

fn sources() -> SourceRegister {
    SourceRegister {
        schema: "../../../schemas/knowledge-sources.schema.json".into(),
        schema_version: 1,
        sources: vec![Source {
            id: "source.fixture".into(),
            authority: "Fixture".into(),
            title: "Fixture".into(),
            revision: "Current".into(),
            uri: "https://example.com".into(),
            scope: "Fixture".into(),
            kind: "standard".into(),
            publication_status: "living".into(),
            last_reviewed: "2026-10-01".into(),
            locations: vec![SourceLocation {
                id: "fixture".into(),
                locator: "Fixture".into(),
                uri: "https://example.com#fixture".into(),
                supports: "Fixture".into(),
            }],
        }],
    }
}

fn fixture_repository(document: &str) -> tempfile::TempDir {
    let repository = tempfile::tempdir().unwrap();
    let root = repository.path().join(CANONICAL_ROOT);
    fs::create_dir_all(root.join("drafts")).unwrap();
    fs::write(root.join("README.md"), "Draft index.\n").unwrap();
    fs::write(root.join("drafts/fixture.md"), document).unwrap();
    repository
}

#[test]
fn integrity_accepts_incomplete_draft_prose_without_editorial_structure() {
    let repository = fixture_repository(
        "Work in progress with [an unresolved link](missing.md#section).\n\n```text\nunfinished\n",
    );
    validate_catalog_integrity(repository.path(), &catalog("drafts/fixture.md"), &sources())
        .unwrap();
}

#[test]
fn recipe_loader_accepts_confined_relative_workspace_roots() {
    let authored = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/validation/examples/single-crate.json"),
    )
    .unwrap();
    let mut value: serde_json::Value = serde_json::from_str(&authored).unwrap();
    value["workspaceRoot"] = "../nested/project".into();
    let authored = serde_json::to_string_pretty(&value).unwrap();
    let document = catalog("drafts/fixture.md").documents.remove(0);

    validate_recipe_with_ordinary_loader(&document, &authored, &value).unwrap();
}

#[test]
fn recipe_loader_rejects_workspace_roots_outside_its_sandbox() {
    let authored = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/validation/examples/single-crate.json"),
    )
    .unwrap();
    let document = catalog("drafts/fixture.md").documents.remove(0);

    for workspace_root in ["../../..", "/outside"] {
        let mut value: serde_json::Value = serde_json::from_str(&authored).unwrap();
        value["workspaceRoot"] = workspace_root.into();
        let authored = serde_json::to_string_pretty(&value).unwrap();
        assert!(
            validate_recipe_with_ordinary_loader(&document, &authored, &value).is_err(),
            "accepted unsafe workspaceRoot {workspace_root}"
        );
    }
}

#[test]
fn integrity_rejects_unsafe_missing_duplicate_and_unknown_references() {
    let repository = fixture_repository("Draft.\n");

    let mut value = catalog("../escape.md");
    assert!(validate_catalog_integrity(repository.path(), &value, &sources()).is_err());

    value = catalog("drafts/missing.md");
    assert!(validate_catalog_integrity(repository.path(), &value, &sources()).is_err());

    value = catalog("drafts/fixture.md");
    value.documents.push(Document {
        id: value.documents[0].id.clone(),
        title: "Duplicate".into(),
        kind: "foundation".into(),
        path: "drafts/duplicate.md".into(),
        summary: "Duplicate".into(),
        questions: vec!["Duplicate?".into()],
        applicability: vec!["general".into()],
        evidence_dimensions: vec!["strategy".into()],
        related: Vec::new(),
        sources: vec!["source.fixture".into()],
        status: "draft".into(),
        last_reviewed: "2026-10-01".into(),
    });
    assert!(validate_catalog_integrity(repository.path(), &value, &sources()).is_err());

    value = catalog("drafts/fixture.md");
    value.documents[0].related.push("missing.document".into());
    assert!(validate_catalog_integrity(repository.path(), &value, &sources()).is_err());

    value = catalog("drafts/fixture.md");
    value.documents[0].sources[0] = "missing.source".into();
    assert!(validate_catalog_integrity(repository.path(), &value, &sources()).is_err());

    value = catalog("drafts/fixture.md");
    value.documents[0].applicability[0] = "missing-tag".into();
    assert!(validate_catalog_integrity(repository.path(), &value, &sources()).is_err());
}

#[test]
fn unsafe_relative_paths_are_rejected() {
    for path in [
        "",
        "/absolute",
        "../escape",
        "./file.md",
        "nested//file.md",
        "nested/../escape",
        "nested\\file",
    ] {
        assert!(safe_relative(path).is_err(), "accepted unsafe path {path}");
    }
    assert_eq!(
        safe_relative("nested/file.md").unwrap(),
        Path::new("nested/file.md")
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

    let mut assets = embedded_source_assets(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
    assets.push(("invalid-utf8.bin".into(), vec![0xff]));
    assert!(validate_embedded_asset_inventory(&catalog, &assets).is_err());
}

#[test]
fn recipe_parser_uses_commonmark_and_requires_one_valid_json_example() {
    assert_eq!(
        extract_recipe_json("> ```json\n> {\"value\": true}\n> ```\n").unwrap()["value"],
        true
    );
    assert!(extract_recipe_json("# Recipe\n").is_err());
    assert!(extract_recipe_json("```json\n{}\n```\n```json\n{}\n```\n").is_err());
    assert!(extract_recipe_json("```json\n{broken}\n```\n").is_err());
}

#[test]
fn tree_digest_is_deterministic() {
    let repository = fixture_repository("Draft.\n");
    let root = repository.path().join(CANONICAL_ROOT);
    assert_eq!(tree_digest(&root).unwrap(), tree_digest(&root).unwrap());
}
