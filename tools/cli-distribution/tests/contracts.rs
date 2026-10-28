use std::{fs, path::PathBuf, process::Command};

use serde_json::Value;
use tempfile::TempDir;

fn binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_liknon-cli-distribution"))
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("tool must remain below tools/cli-distribution")
        .to_path_buf()
}

fn descriptor() -> PathBuf {
    repository_root().join("distribution/cli/validation/product.json")
}

fn run(arguments: &[&str]) -> std::process::Output {
    Command::new(binary())
        .args(arguments)
        .output()
        .expect("distribution command should start")
}

fn write_descriptor(temporary: &TempDir, value: &Value) -> PathBuf {
    let path = temporary.path().join("product.json");
    fs::write(&path, serde_json::to_vec(value).unwrap()).unwrap();
    path
}

#[test]
fn checked_in_schemas_are_canonical() {
    let temporary = TempDir::new().unwrap();
    let output = run(&[
        "schemas",
        "--output-dir",
        temporary.path().to_str().unwrap(),
    ]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let expected = repository_root().join("distribution/cli/common/schemas");
    let mut names = fs::read_dir(&expected)
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect::<Vec<_>>();
    names.sort();
    assert!(!names.is_empty());
    for name in names {
        assert_eq!(
            fs::read(expected.join(&name)).unwrap(),
            fs::read(temporary.path().join(&name)).unwrap(),
            "schema {} drifted",
            name.to_string_lossy()
        );
    }
}

#[test]
fn cargo_inspection_derives_validation_identity() {
    let temporary = TempDir::new().unwrap();
    let identity = temporary.path().join("identity.json");
    let output = run(&[
        "inspect",
        "--descriptor",
        descriptor().to_str().unwrap(),
        "--workspace",
        repository_root().to_str().unwrap(),
        "--output",
        identity.to_str().unwrap(),
    ]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: Value = serde_json::from_slice(&fs::read(identity).unwrap()).unwrap();
    assert_eq!(value["product"], "validation");
    assert_eq!(value["package"], "liknon-validator");
    assert_eq!(value["version"], env!("CARGO_PKG_VERSION"));
    assert_eq!(
        value["tag"],
        format!("liknon-validator-v{}", env!("CARGO_PKG_VERSION"))
    );
    assert_eq!(
        value["binaryTargets"],
        serde_json::json!(["liknon-validator", "likv"])
    );
}

#[test]
fn descriptor_rejects_cross_product_namespace() {
    let temporary = TempDir::new().unwrap();
    let mut value: Value = serde_json::from_slice(&fs::read(descriptor()).unwrap()).unwrap();
    value["distribution"]["releasePrefix"] = "releases/liknon-manager".into();
    let invalid = write_descriptor(&temporary, &value);
    let output = run(&[
        "validate-descriptor",
        "--descriptor",
        invalid.to_str().unwrap(),
    ]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("releasePrefix"));
}

#[test]
fn descriptor_rejects_missing_publication_authority() {
    let temporary = TempDir::new().unwrap();
    let mut value: Value = serde_json::from_slice(&fs::read(descriptor()).unwrap()).unwrap();
    value["distribution"]["environments"]["approval"] = "".into();
    let invalid = write_descriptor(&temporary, &value);
    let output = run(&[
        "validate-descriptor",
        "--descriptor",
        invalid.to_str().unwrap(),
    ]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("distribution.environments"));
}

#[test]
fn descriptor_rejects_unsafe_repository_path() {
    let temporary = TempDir::new().unwrap();
    let mut value: Value = serde_json::from_slice(&fs::read(descriptor()).unwrap()).unwrap();
    value["product"]["cargoManifest"] = "../Cargo.toml".into();
    let invalid = write_descriptor(&temporary, &value);
    let output = run(&[
        "validate-descriptor",
        "--descriptor",
        invalid.to_str().unwrap(),
    ]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("product.cargoManifest"));
}

#[test]
fn descriptor_rejects_product_target_mismatch() {
    let temporary = TempDir::new().unwrap();
    let mut value: Value = serde_json::from_slice(&fs::read(descriptor()).unwrap()).unwrap();
    value["targets"][0]["rustTarget"] = "x86_64-unknown-linux-gnux".into();
    let invalid = write_descriptor(&temporary, &value);
    let output = run(&[
        "validate-descriptor",
        "--descriptor",
        invalid.to_str().unwrap(),
    ]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("target coordinates require"));
}

#[test]
fn descriptor_rejects_more_than_one_compact_alias() {
    let temporary = TempDir::new().unwrap();
    let mut value: Value = serde_json::from_slice(&fs::read(descriptor()).unwrap()).unwrap();
    value["product"]["aliases"] = serde_json::json!(["likv", "validate"]);
    let invalid = write_descriptor(&temporary, &value);
    let output = run(&[
        "validate-descriptor",
        "--descriptor",
        invalid.to_str().unwrap(),
    ]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("exactly one compact command alias"));
}

#[test]
fn reusable_workflows_contain_no_validation_product_identity() {
    for workflow in [
        "_cli-native-candidate.yml",
        "_cli-direct-assembly.yml",
        "_cli-public-verification.yml",
    ] {
        let content =
            fs::read_to_string(repository_root().join(".github/workflows").join(workflow)).unwrap();
        for product_value in [
            "liknon-validator",
            "validation-native",
            "distribution/cli/validation",
            "apps/liknon-validator",
        ] {
            assert!(
                !content.contains(product_value),
                "reusable workflow {workflow} contains product value {product_value}"
            );
        }
    }
}

#[test]
fn unknown_product_cannot_activate_a_repository_package() {
    let temporary = TempDir::new().unwrap();
    let mut value: Value = serde_json::from_slice(&fs::read(descriptor()).unwrap()).unwrap();
    value["product"]["id"] = "missing-product".into();
    value["product"]["cargoPackage"] = "missing-product-cli".into();
    value["product"]["cargoManifest"] = "apps/missing-product/Cargo.toml".into();
    value["product"]["canonicalCommand"] = "missing-product-cli".into();
    value["product"]["aliases"] = serde_json::json!(["mpc"]);
    value["product"]["tagPrefix"] = "missing-product-cli-v".into();
    value["distribution"]["releasePrefix"] = "releases/missing-product-cli".into();
    value["distribution"]["stablePointer"] = "channels/missing-product-cli/stable.json".into();
    value["distribution"]["manifestSigningKeyId"] = "missing-product-release-v1".into();
    let unknown = write_descriptor(&temporary, &value);

    let output = run(&[
        "validate-descriptor",
        "--descriptor",
        unknown.to_str().unwrap(),
    ]);
    assert!(
        output.status.success(),
        "a product-neutral descriptor should remain valid: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let output = run(&[
        "inspect",
        "--descriptor",
        unknown.to_str().unwrap(),
        "--workspace",
        repository_root().to_str().unwrap(),
        "--output",
        temporary.path().join("identity.json").to_str().unwrap(),
    ]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("was not found"));
}

#[test]
fn target_selection_covers_linux_precedence_and_failure() {
    let temporary = TempDir::new().unwrap();
    let selected = temporary.path().join("selected.json");
    let output = run(&[
        "select-target",
        "--descriptor",
        descriptor().to_str().unwrap(),
        "--operating-system",
        "linux",
        "--architecture",
        "x86-64",
        "--gnu-detected",
        "--musl-detected",
        "--output",
        selected.to_str().unwrap(),
    ]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: Value = serde_json::from_slice(&fs::read(&selected).unwrap()).unwrap();
    assert_eq!(value["selectedTarget"], "x86_64-unknown-linux-gnu");
    assert_eq!(value["decisionSource"], "automatic-gnu-preferred");

    let output = run(&[
        "select-target",
        "--descriptor",
        descriptor().to_str().unwrap(),
        "--operating-system",
        "linux",
        "--architecture",
        "aarch64",
        "--output",
        selected.to_str().unwrap(),
    ]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("neither a compatible GNU libc"));
}

#[test]
fn explicit_libc_must_be_detected() {
    let temporary = TempDir::new().unwrap();
    let output = run(&[
        "select-target",
        "--descriptor",
        descriptor().to_str().unwrap(),
        "--operating-system",
        "linux",
        "--architecture",
        "x86-64",
        "--libc",
        "musl",
        "--gnu-detected",
        "--output",
        temporary.path().join("selected.json").to_str().unwrap(),
    ]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("requested Linux libc"));
}
