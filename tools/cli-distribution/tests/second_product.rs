#![cfg(target_os = "linux")]

use std::{
    fs,
    net::{TcpListener, TcpStream},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread,
    time::Duration,
};

use serde_json::Value;
use tempfile::TempDir;

const TARGET: &str = "x86_64-unknown-linux-gnu";

fn binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_liknon-cli-distribution"))
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .unwrap()
        .to_path_buf()
}

fn run(arguments: &[&str]) {
    let output = Command::new(binary()).args(arguments).output().unwrap();
    assert!(
        output.status.success(),
        "command failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn run_external(program: &str, arguments: &[&str]) {
    let output = Command::new(program).args(arguments).output().unwrap();
    assert!(
        output.status.success(),
        "{program} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn shared_foundation_assembles_and_installs_an_isolated_product() {
    for required in ["curl", "jq", "openssl", "python3", "tar", "touch"] {
        if Command::new(required).arg("--version").output().is_err() {
            eprintln!("skipping second-product test because {required} is unavailable");
            return;
        }
    }

    let root = repository_root();
    let temporary = TempDir::new().unwrap();
    let descriptor = temporary.path().join("product.json");
    let identity = temporary.path().join("identity.json");
    fs::write(
        &descriptor,
        serde_json::to_vec_pretty(&serde_json::json!({
            "schemaVersion": 1,
            "product": {
                "id": "fixture",
                "ownerPlan": "TEST",
                "cargoPackage": "fixture-cli",
                "cargoManifest": "fixtures/fixture-cli/Cargo.toml",
                "canonicalCommand": "fixture-cli",
                "aliases": ["fix"],
                "tagPrefix": "fixture-cli-v",
                "releaseRustToolchain": "1.87.0",
                "repository": "example/fixture",
                "licenseFiles": ["LICENSE"]
            },
            "distribution": {
                "cratesRegistry": "crates-io",
                "releasePrefix": "releases/fixture-cli",
                "stablePointer": "channels/fixture-cli/stable.json",
                "manifestSigningKeyId": "fixture-release-v1",
                "environments": {
                    "approval": "fixture-approval",
                    "crates": "fixture-crates",
                    "detachedSigning": "fixture-signing",
                    "windowsSigning": "fixture-windows",
                    "github": "fixture-github",
                    "cloudflare": "fixture-cloudflare"
                }
            },
            "targets": [{
                "rustTarget": TARGET,
                "runner": "ubuntu-24.04",
                "operatingSystem": "linux",
                "architecture": "x86-64",
                "libc": "gnu",
                "archiveFormat": "tar-gz",
                "executableSuffix": "",
                "authenticity": "metadata-signature",
                "runtimeBaseline": "fixture GNU baseline"
            }]
        }))
        .unwrap(),
    )
    .unwrap();
    fs::write(
        &identity,
        serde_json::to_vec_pretty(&serde_json::json!({
            "schemaVersion": 1,
            "product": "fixture",
            "package": "fixture-cli",
            "packagePath": "fixtures/fixture-cli/Cargo.toml",
            "version": "1.2.3",
            "tag": "fixture-cli-v1.2.3",
            "commit": "1111111111111111111111111111111111111111",
            "cargoLockSha256": "2222222222222222222222222222222222222222222222222222222222222222",
            "binaryTargets": ["fixture-cli", "fix"]
        }))
        .unwrap(),
    )
    .unwrap();

    let payload_root = temporary.path().join("payloads");
    let payload = payload_root.join(TARGET).join("fixture-cli");
    fs::create_dir_all(payload.parent().unwrap()).unwrap();
    fs::write(&payload, b"#!/bin/sh\nprintf '%s\\n' 'fixture 1.2.3'\n").unwrap();
    let mut permissions = fs::metadata(&payload).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&payload, permissions).unwrap();

    let authenticity = temporary.path().join("authenticity.json");
    fs::write(
        &authenticity,
        br#"{"mode":"metadata-signature","verified":true,"method":"fixture signed metadata"}"#,
    )
    .unwrap();
    let records = temporary.path().join("records");
    fs::create_dir_all(&records).unwrap();
    run(&[
        "native-record",
        "--descriptor",
        descriptor.to_str().unwrap(),
        "--target",
        TARGET,
        "--payload",
        payload.to_str().unwrap(),
        "--payload-path",
        "x86_64-unknown-linux-gnu/fixture-cli",
        "--authenticity",
        authenticity.to_str().unwrap(),
        "--output",
        records.join(format!("{TARGET}.json")).to_str().unwrap(),
    ]);

    let build_manifest = temporary.path().join("build-manifest.json");
    run(&[
        "build-manifest",
        "--descriptor",
        descriptor.to_str().unwrap(),
        "--identity",
        identity.to_str().unwrap(),
        "--workspace",
        root.to_str().unwrap(),
        "--records-dir",
        records.to_str().unwrap(),
        "--release-toolchain",
        "rustc fixture",
        "--output",
        build_manifest.to_str().unwrap(),
    ]);

    let direct = temporary.path().join("direct");
    run(&[
        "assemble-archives",
        "--descriptor",
        descriptor.to_str().unwrap(),
        "--identity",
        identity.to_str().unwrap(),
        "--workspace",
        root.to_str().unwrap(),
        "--build-manifest",
        build_manifest.to_str().unwrap(),
        "--payload-root",
        payload_root.to_str().unwrap(),
        "--output-dir",
        direct.to_str().unwrap(),
        "--source-date-epoch",
        "1700000000",
    ]);

    let private_key = temporary.path().join("private.pem");
    let public_key = temporary.path().join("public.pem");
    run_external(
        "openssl",
        &[
            "genpkey",
            "-algorithm",
            "RSA",
            "-pkeyopt",
            "rsa_keygen_bits:2048",
            "-out",
            private_key.to_str().unwrap(),
        ],
    );
    run_external(
        "openssl",
        &[
            "pkey",
            "-in",
            private_key.to_str().unwrap(),
            "-pubout",
            "-out",
            public_key.to_str().unwrap(),
        ],
    );

    let port = TcpListener::bind(("127.0.0.1", 0))
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let base_url = format!("http://127.0.0.1:{port}");
    run(&[
        "render-installers",
        "--descriptor",
        descriptor.to_str().unwrap(),
        "--identity",
        identity.to_str().unwrap(),
        "--workspace",
        root.to_str().unwrap(),
        "--base-url",
        &base_url,
        "--public-key",
        public_key.to_str().unwrap(),
        "--posix-template",
        root.join("distribution/cli/common/installers/install.sh.in")
            .to_str()
            .unwrap(),
        "--powershell-template",
        root.join("distribution/cli/common/installers/install.ps1.in")
            .to_str()
            .unwrap(),
        "--output-dir",
        direct.to_str().unwrap(),
        "--allow-http",
    ]);
    run(&[
        "checksums",
        "--descriptor",
        descriptor.to_str().unwrap(),
        "--identity",
        identity.to_str().unwrap(),
        "--workspace",
        root.to_str().unwrap(),
        "--root",
        direct.to_str().unwrap(),
    ]);
    let manifest = direct.join("manifest.json");
    run(&[
        "direct-manifest",
        "--descriptor",
        descriptor.to_str().unwrap(),
        "--identity",
        identity.to_str().unwrap(),
        "--workspace",
        root.to_str().unwrap(),
        "--root",
        direct.to_str().unwrap(),
        "--output",
        manifest.to_str().unwrap(),
    ]);
    run_external(
        root.join("distribution/cli/common/scripts/sign-direct.sh")
            .to_str()
            .unwrap(),
        &[direct.to_str().unwrap(), private_key.to_str().unwrap()],
    );
    run(&[
        "verify-direct",
        "--descriptor",
        descriptor.to_str().unwrap(),
        "--identity",
        identity.to_str().unwrap(),
        "--workspace",
        root.to_str().unwrap(),
        "--manifest",
        manifest.to_str().unwrap(),
        "--root",
        direct.to_str().unwrap(),
        "--public-key",
        public_key.to_str().unwrap(),
    ]);

    let manifest_value: Value = serde_json::from_slice(&fs::read(&manifest).unwrap()).unwrap();
    assert_eq!(manifest_value["identity"]["product"], "fixture");
    assert!(manifest_value["artifacts"][0]["archive"]["path"]
        .as_str()
        .unwrap()
        .starts_with("fixture-cli-v1.2.3-"));
    assert!(!fs::read_to_string(direct.join("install.sh"))
        .unwrap()
        .contains("liknon-validator"));

    let release = temporary.path().join("web/releases/fixture-cli/v1.2.3");
    fs::create_dir_all(&release).unwrap();
    for entry in fs::read_dir(&direct).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            fs::copy(entry.path(), release.join(entry.file_name())).unwrap();
        }
    }
    let mut server = ChildGuard(
        Command::new("python3")
            .args([
                "-m",
                "http.server",
                &port.to_string(),
                "--bind",
                "127.0.0.1",
                "--directory",
                temporary.path().join("web").to_str().unwrap(),
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap(),
    );
    for _ in 0..50 {
        if TcpStream::connect(("127.0.0.1", port)).is_ok() {
            break;
        }
        thread::sleep(Duration::from_millis(20));
    }
    let install = temporary.path().join("fixture install");
    let status = Command::new("sh")
        .arg(direct.join("install.sh"))
        .args(["--install-dir", install.to_str().unwrap()])
        .status()
        .unwrap();
    assert!(status.success());
    assert_eq!(
        fs::read(install.join("fixture-cli")).unwrap(),
        fs::read(install.join("fix")).unwrap()
    );
    assert!(!install.join("liknon-validator").exists());
    let output = Command::new(install.join("fixture-cli")).output().unwrap();
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "fixture 1.2.3\n");

    let stable = temporary.path().join("stable.json");
    run(&[
        "stable-pointer",
        "--descriptor",
        descriptor.to_str().unwrap(),
        "--identity",
        identity.to_str().unwrap(),
        "--workspace",
        root.to_str().unwrap(),
        "--manifest",
        manifest.to_str().unwrap(),
        "--output",
        stable.to_str().unwrap(),
    ]);
    let stable: Value = serde_json::from_slice(&fs::read(stable).unwrap()).unwrap();
    assert_eq!(
        stable["manifestPath"],
        "releases/fixture-cli/v1.2.3/manifest.json"
    );

    server.0.kill().unwrap();
    server.0.wait().unwrap();
}

struct ChildGuard(Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
