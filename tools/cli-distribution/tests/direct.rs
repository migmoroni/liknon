#![cfg(target_os = "linux")]

use std::{
    fs,
    net::{TcpListener, TcpStream},
    path::PathBuf,
    process::{Child, Command, Stdio},
    thread,
    time::Duration,
};

use serde_json::Value;
use tempfile::TempDir;

fn binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_liknon-cli-distribution"))
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
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
fn assembles_signs_and_verifies_complete_direct_candidate() {
    for required in ["openssl", "tar", "zip", "unzip", "touch"] {
        if Command::new(required).arg("--version").output().is_err() {
            eprintln!("skipping direct candidate test because {required} is unavailable");
            return;
        }
    }

    let root = repository_root();
    let descriptor = root.join("distribution/cli/validation/product.json");
    let temporary = TempDir::new().unwrap();
    let identity = temporary.path().join("identity.json");
    run(&[
        "inspect",
        "--descriptor",
        descriptor.to_str().unwrap(),
        "--workspace",
        root.to_str().unwrap(),
        "--output",
        identity.to_str().unwrap(),
    ]);

    let descriptor_value: Value = serde_json::from_slice(&fs::read(&descriptor).unwrap()).unwrap();
    let payload_root = temporary.path().join("payloads");
    let records = temporary.path().join("records");
    fs::create_dir_all(&records).unwrap();
    for target in descriptor_value["targets"].as_array().unwrap() {
        let target_name = target["rustTarget"].as_str().unwrap();
        let suffix = target["executableSuffix"].as_str().unwrap();
        let relative = format!("{target_name}/liknon-validator{suffix}");
        let payload = payload_root.join(&relative);
        fs::create_dir_all(payload.parent().unwrap()).unwrap();
        fs::write(&payload, format!("fixture payload for {target_name}\n")).unwrap();
        let authenticity = temporary.path().join(format!("{target_name}-auth.json"));
        let evidence = match target["authenticity"].as_str().unwrap() {
            "metadata-signature" => serde_json::json!({
                "mode": "metadata-signature",
                "verified": true,
                "method": "signed release metadata"
            }),
            "ad-hoc" => serde_json::json!({
                "mode": "ad-hoc",
                "verified": true,
                "method": "codesign --verify --strict",
                "notarized": false
            }),
            "self-signed-authenticode" => serde_json::json!({
                "mode": "self-signed-authenticode",
                "verified": true,
                "method": "Get-AuthenticodeSignature",
                "certificateFingerprint": "00112233445566778899AABBCCDDEEFF00112233",
                "timestamped": true
            }),
            value => panic!("unexpected authenticity mode {value}"),
        };
        fs::write(&authenticity, serde_json::to_vec(&evidence).unwrap()).unwrap();
        let record = records.join(format!("{target_name}.json"));
        run(&[
            "native-record",
            "--descriptor",
            descriptor.to_str().unwrap(),
            "--target",
            target_name,
            "--payload",
            payload.to_str().unwrap(),
            "--payload-path",
            &relative,
            "--authenticity",
            authenticity.to_str().unwrap(),
            "--output",
            record.to_str().unwrap(),
        ]);
    }

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
    run(&[
        "verify-build",
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
    let identity_value: Value = serde_json::from_slice(&fs::read(&identity).unwrap()).unwrap();
    assert!(direct
        .join(format!(
            "liknon-validator-v{}-x86_64-unknown-linux-gnu.tar.gz",
            identity_value["version"].as_str().unwrap()
        ))
        .is_file());

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
    run(&[
        "render-installers",
        "--descriptor",
        descriptor.to_str().unwrap(),
        "--identity",
        identity.to_str().unwrap(),
        "--workspace",
        root.to_str().unwrap(),
        "--base-url",
        "http://127.0.0.1:18080",
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
    let signing_script = root.join("distribution/cli/common/scripts/sign-direct.sh");
    run_external(
        signing_script.to_str().unwrap(),
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

    let unrelated_private_key = temporary.path().join("unrelated-private.pem");
    let unrelated_public_key = temporary.path().join("unrelated-public.pem");
    run_external(
        "openssl",
        &[
            "genpkey",
            "-algorithm",
            "RSA",
            "-pkeyopt",
            "rsa_keygen_bits:2048",
            "-out",
            unrelated_private_key.to_str().unwrap(),
        ],
    );
    run_external(
        "openssl",
        &[
            "pkey",
            "-in",
            unrelated_private_key.to_str().unwrap(),
            "-pubout",
            "-out",
            unrelated_public_key.to_str().unwrap(),
        ],
    );
    let output = Command::new(binary())
        .args([
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
            unrelated_public_key.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "unrelated signing key was accepted"
    );

    let mut wrong_key_manifest: Value =
        serde_json::from_slice(&fs::read(&manifest).unwrap()).unwrap();
    wrong_key_manifest["signature"]["signingKeyId"] = "retired-release-key".into();
    let wrong_key_manifest_path = direct.join("manifest-wrong-key.json");
    fs::write(
        &wrong_key_manifest_path,
        serde_json::to_vec_pretty(&wrong_key_manifest).unwrap(),
    )
    .unwrap();
    let output = Command::new(binary())
        .args([
            "verify-direct",
            "--descriptor",
            descriptor.to_str().unwrap(),
            "--identity",
            identity.to_str().unwrap(),
            "--workspace",
            root.to_str().unwrap(),
            "--manifest",
            wrong_key_manifest_path.to_str().unwrap(),
            "--root",
            direct.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "out-of-scope signing key ID was accepted"
    );
    fs::remove_file(wrong_key_manifest_path).unwrap();

    let installer = direct.join("install.sh");
    let installer_bytes = fs::read(&installer).unwrap();
    let mut mutated_installer = installer_bytes.clone();
    mutated_installer.push(b'\n');
    fs::write(&installer, mutated_installer).unwrap();
    let output = Command::new(binary())
        .args([
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
        ])
        .output()
        .unwrap();
    assert!(!output.status.success(), "mutated installer was accepted");
    fs::write(&installer, installer_bytes).unwrap();

    exercise_posix_installer(
        &direct,
        &descriptor_value,
        &identity_value,
        temporary.path(),
        &public_key,
    );
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
    let stable_value: Value = serde_json::from_slice(&fs::read(stable).unwrap()).unwrap();
    assert_eq!(stable_value["product"], "validation");
    assert!(stable_value["manifestPath"]
        .as_str()
        .unwrap()
        .ends_with("/manifest.json"));

    let first_archive = fs::read_dir(&direct)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| path.extension().is_some_and(|extension| extension == "gz"))
        .unwrap();
    let mut bytes = fs::read(&first_archive).unwrap();
    bytes[0] ^= 1;
    fs::write(&first_archive, bytes).unwrap();
    let output = Command::new(binary())
        .args([
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
        ])
        .output()
        .unwrap();
    assert!(!output.status.success(), "mutated archive was accepted");
}

fn exercise_posix_installer(
    direct: &std::path::Path,
    descriptor: &Value,
    identity: &Value,
    temporary: &std::path::Path,
    public_key: &std::path::Path,
) {
    if Command::new("python3").arg("--version").output().is_err()
        || Command::new("jq").arg("--version").output().is_err()
        || Command::new("curl").arg("--version").output().is_err()
    {
        eprintln!("skipping POSIX installer execution because a fixture prerequisite is missing");
        return;
    }
    let port = TcpListener::bind(("127.0.0.1", 0))
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let release_root = temporary
        .join("web")
        .join(
            descriptor["distribution"]["releasePrefix"]
                .as_str()
                .unwrap(),
        )
        .join(format!("v{}", identity["version"].as_str().unwrap()));
    fs::create_dir_all(&release_root).unwrap();
    for entry in fs::read_dir(direct).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            fs::copy(entry.path(), release_root.join(entry.file_name())).unwrap();
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
                temporary.join("web").to_str().unwrap(),
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
    let install_dir = temporary.join("installed path with spaces");
    let base_url = format!("http://127.0.0.1:{port}");
    exercise_authenticated_fetch(
        &release_root,
        descriptor,
        identity,
        temporary,
        public_key,
        &base_url,
    );
    let status = Command::new("sh")
        .arg(direct.join("install.sh"))
        .args(["--base-url", &base_url, "--install-dir"])
        .arg(&install_dir)
        .status()
        .unwrap();
    assert!(status.success());
    let canonical = install_dir.join("liknon-validator");
    let alias = install_dir.join("likv");
    assert_eq!(fs::read(&canonical).unwrap(), fs::read(&alias).unwrap());
    let previous = fs::read(&canonical).unwrap();

    let archive = descriptor["targets"]
        .as_array()
        .unwrap()
        .iter()
        .find(|target| target["rustTarget"] == "x86_64-unknown-linux-gnu")
        .map(|target| {
            format!(
                "liknon-validator-v{}-{}.tar.gz",
                identity["version"].as_str().unwrap(),
                target["rustTarget"].as_str().unwrap()
            )
        })
        .unwrap();
    let mut mutated = fs::read(release_root.join(&archive)).unwrap();
    mutated.push(0);
    fs::write(release_root.join(archive), mutated).unwrap();
    let status = Command::new("sh")
        .arg(direct.join("install.sh"))
        .args(["--base-url", &base_url, "--install-dir"])
        .arg(&install_dir)
        .status()
        .unwrap();
    assert!(!status.success());
    assert_eq!(fs::read(&canonical).unwrap(), previous);
    assert_eq!(fs::read(&alias).unwrap(), previous);

    let status = Command::new("sh")
        .arg(direct.join("install.sh"))
        .arg("--install-dir")
        .arg(&install_dir)
        .arg("--uninstall")
        .status()
        .unwrap();
    assert!(status.success());
    assert!(!canonical.exists());
    assert!(!alias.exists());
    server.0.kill().unwrap();
    server.0.wait().unwrap();
}

fn exercise_authenticated_fetch(
    release_root: &std::path::Path,
    descriptor: &Value,
    identity: &Value,
    temporary: &std::path::Path,
    public_key: &std::path::Path,
    base_url: &str,
) {
    let script = repository_root().join("distribution/cli/common/scripts/fetch-public-direct.sh");
    let prefix = descriptor["distribution"]["releasePrefix"]
        .as_str()
        .unwrap();
    let version = identity["version"].as_str().unwrap();
    let fetched = temporary.join("authenticated public fetch");
    let status = Command::new(&script)
        .args([base_url, prefix, version])
        .arg(&fetched)
        .arg(public_key)
        .status()
        .unwrap();
    assert!(status.success());
    assert!(fetched.join("manifest.json").is_file());
    assert!(fs::read_dir(&fetched).unwrap().count() > 2);

    let served_manifest = release_root.join("manifest.json");
    let original = fs::read(&served_manifest).unwrap();
    let mut changed = original.clone();
    changed.push(b'\n');
    fs::write(&served_manifest, changed).unwrap();
    let rejected = temporary.join("rejected public fetch");
    let status = Command::new(&script)
        .args([base_url, prefix, version])
        .arg(&rejected)
        .arg(public_key)
        .status()
        .unwrap();
    assert!(!status.success());
    assert_eq!(fs::read_dir(&rejected).unwrap().count(), 2);
    fs::write(served_manifest, original).unwrap();
}

struct ChildGuard(Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
