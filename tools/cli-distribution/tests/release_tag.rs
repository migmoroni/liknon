#![cfg(unix)]

use std::{fs, path::Path, process::Command};

use tempfile::TempDir;

fn run(directory: &Path, program: &str, arguments: &[&str]) {
    let output = Command::new(program)
        .args(arguments)
        .current_dir(directory)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{program} failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn verifies_only_annotated_tags_under_one_explicit_trust_root() {
    if Command::new("ssh-keygen").arg("-V").output().is_err() {
        eprintln!("skipping signed-tag fixture because ssh-keygen is unavailable");
        return;
    }

    let temporary = TempDir::new().unwrap();
    let repository = temporary.path().join("repository");
    let trust = temporary.path().join("trust");
    let key = temporary.path().join("release-key");
    fs::create_dir_all(&repository).unwrap();
    fs::create_dir_all(&trust).unwrap();
    run(
        temporary.path(),
        "ssh-keygen",
        &["-q", "-t", "ed25519", "-N", "", "-f", key.to_str().unwrap()],
    );

    run(&repository, "git", &["init", "--quiet"]);
    run(
        &repository,
        "git",
        &["config", "user.name", "Release Fixture"],
    );
    run(
        &repository,
        "git",
        &["config", "user.email", "release@example.invalid"],
    );
    run(&repository, "git", &["config", "gpg.format", "ssh"]);
    run(
        &repository,
        "git",
        &["config", "user.signingkey", key.to_str().unwrap()],
    );
    fs::write(repository.join("payload"), "release fixture\n").unwrap();
    run(&repository, "git", &["add", "payload"]);
    run(&repository, "git", &["commit", "--quiet", "-m", "fixture"]);
    run(
        &repository,
        "git",
        &["tag", "-s", "liknon-validator-v0.5.0", "-m", "release"],
    );
    run(&repository, "git", &["tag", "lightweight"]);

    let public_key = fs::read_to_string(key.with_extension("pub")).unwrap();
    fs::write(
        trust.join("tag-signing-allowed-signers"),
        format!("release@example.invalid namespaces=\"git\" {public_key}"),
    )
    .unwrap();
    let script = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .unwrap()
        .join("distribution/cli/common/scripts/verify-release-tag.sh");

    run(
        &repository,
        script.to_str().unwrap(),
        &["liknon-validator-v0.5.0", trust.to_str().unwrap()],
    );

    let lightweight = Command::new(&script)
        .args(["lightweight", trust.to_str().unwrap()])
        .current_dir(&repository)
        .output()
        .unwrap();
    assert!(!lightweight.status.success());

    fs::write(trust.join("tag-signing-public-key.asc"), "ambiguous\n").unwrap();
    let ambiguous = Command::new(&script)
        .args(["liknon-validator-v0.5.0", trust.to_str().unwrap()])
        .current_dir(&repository)
        .output()
        .unwrap();
    assert!(!ambiguous.status.success());
}
