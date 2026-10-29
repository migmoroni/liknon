use std::{fs, path::PathBuf};

use liknon_resource_contracts::{
    CatalogSnapshot, DetachedSignature, ResourceLock, ResourceManifest, ResourceVersion, TrustRoot,
};

#[test]
fn checked_in_public_schemas_match_the_rust_contracts() {
    for (name, schema) in [
        (
            "resource.schema.json",
            serde_json::to_value(schemars::schema_for!(ResourceVersion)).unwrap(),
        ),
        (
            "catalog.schema.json",
            serde_json::to_value(schemars::schema_for!(CatalogSnapshot)).unwrap(),
        ),
        (
            "manifest.schema.json",
            serde_json::to_value(schemars::schema_for!(ResourceManifest)).unwrap(),
        ),
        (
            "lock.schema.json",
            serde_json::to_value(schemars::schema_for!(ResourceLock)).unwrap(),
        ),
        (
            "trust-root.schema.json",
            serde_json::to_value(schemars::schema_for!(TrustRoot)).unwrap(),
        ),
        (
            "signature.schema.json",
            serde_json::to_value(schemars::schema_for!(DetachedSignature)).unwrap(),
        ),
    ] {
        jsonschema::validator_for(&schema).expect("generated schema is valid");
        let path = schema_directory().join(name);
        let mut expected = serde_json::to_vec_pretty(&schema).unwrap();
        expected.push(b'\n');
        assert_eq!(
            fs::read(&path).unwrap(),
            expected,
            "{} is stale",
            path.display()
        );
    }
}

fn schema_directory() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("schemas")
}
