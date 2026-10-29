use std::{
    fs,
    io::Write as _,
    path::Path,
    process::{Command, Stdio},
};

use tempfile::TempDir;

const COMPONENTS: [(&str, &str, &str); 3] = [
    ("validator", "v", "liknon-validator"),
    ("manager", "m", "liknon-manager"),
    ("tool", "t", "liknon-tool"),
];

fn run(binary: &str, arguments: &[&str]) -> std::process::Output {
    Command::new(binary).args(arguments).output().unwrap()
}

#[test]
fn launcher_help_owns_only_suite_namespaces_and_visual_controls() {
    let launcher = env!("CARGO_BIN_EXE_liknon");
    let plain = run(launcher, &["--help"]);
    assert!(plain.status.success());
    assert!(plain.stderr.is_empty());
    assert!(!plain.stdout.contains(&0x1b));

    let text = String::from_utf8(plain.stdout.clone()).unwrap();
    for expected in ["validator", "manager", "tool", "--color", "--presentation"] {
        assert!(text.contains(expected), "missing {expected:?} in:\n{text}");
    }
    for absent in [" validate ", " install ", " context "] {
        assert!(!text.contains(absent), "unexpected {absent:?} in:\n{text}");
    }

    for palette in [
        "standard",
        "high-contrast",
        "protanopia",
        "deuteranopia",
        "tritanopia",
        "achromatopsia",
    ] {
        let colored = run(launcher, &[&format!("--color={palette}"), "--help"]);
        assert!(colored.status.success(), "{palette}");
        assert!(colored.stdout.contains(&0x1b), "{palette}");
        assert_eq!(
            console::strip_ansi_codes(&String::from_utf8(colored.stdout).unwrap()),
            text,
            "{palette}"
        );
    }
}

#[test]
fn launcher_reports_one_verified_suite_version() {
    let output = run(env!("CARGO_BIN_EXE_liknon"), &["--version"]);
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let version = env!("CARGO_PKG_VERSION");
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        format!(
            "liknon-cli {version}\nliknon {version}\nliknon-validator {version}\nliknon-manager {version}\nliknon-tool {version}\n"
        )
    );
}

#[test]
fn long_and_short_namespaces_route_to_the_same_sibling() {
    let launcher = env!("CARGO_BIN_EXE_liknon");
    for (long, short, executable) in COMPONENTS {
        let direct = run(component_binary(executable), &["--version"]);
        let long_output = run(launcher, &[long, "--version"]);
        let short_output = run(launcher, &[short, "--version"]);
        assert!(direct.status.success(), "{executable}");
        assert_eq!(long_output.status.code(), direct.status.code(), "{long}");
        assert_eq!(short_output.status.code(), direct.status.code(), "{short}");
        assert_eq!(long_output.stdout, direct.stdout, "{long}");
        assert_eq!(short_output.stdout, direct.stdout, "{short}");
        assert_eq!(long_output.stderr, direct.stderr, "{long}");
        assert_eq!(short_output.stderr, direct.stderr, "{short}");
    }
}

#[test]
fn capability_arguments_remain_owned_by_the_child() {
    let launcher = env!("CARGO_BIN_EXE_liknon");
    let direct = run(
        env!("CARGO_BIN_EXE_liknon-validator"),
        &["--color=standard", "--help"],
    );
    let routed = run(launcher, &["validator", "--color=standard", "--help"]);
    assert_eq!(routed.status.code(), direct.status.code());
    assert_eq!(routed.stdout, direct.stdout);
    assert_eq!(routed.stderr, direct.stderr);

    let misplaced = run(launcher, &["--color=standard", "validator", "--help"]);
    assert_eq!(misplaced.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&misplaced.stderr)
        .contains("place them after the capability namespace"));
}

#[test]
fn rejects_missing_and_unknown_namespaces() {
    let launcher = env!("CARGO_BIN_EXE_liknon");
    for arguments in [&[][..], &["unknown"][..]] {
        let output = run(launcher, arguments);
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr).contains("capability namespace"));
    }
}

#[test]
fn rejects_a_missing_sibling_without_falling_back_to_path() {
    let temporary = TempDir::new().unwrap();
    let launcher = temporary
        .path()
        .join(format!("liknon{}", std::env::consts::EXE_SUFFIX));
    fs::copy(env!("CARGO_BIN_EXE_liknon"), &launcher).unwrap();

    let output = Command::new(&launcher)
        .args(["validator", "--version"])
        .env("PATH", parent_dir(env!("CARGO_BIN_EXE_liknon-validator")))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(4));
    assert!(String::from_utf8_lossy(&output.stderr).contains("is missing beside the launcher"));
}

#[cfg(unix)]
#[test]
fn rejects_a_foreign_sibling_with_the_expected_filename() {
    use std::os::unix::fs::PermissionsExt as _;

    let temporary = TempDir::new().unwrap();
    let launcher = temporary.path().join("liknon");
    fs::copy(env!("CARGO_BIN_EXE_liknon"), &launcher).unwrap();
    let sibling = temporary.path().join("liknon-validator");
    fs::write(&sibling, b"#!/bin/sh\nprintf 'liknon-validator 9.9.9\\n'\n").unwrap();
    fs::set_permissions(&sibling, fs::Permissions::from_mode(0o755)).unwrap();

    let output = Command::new(launcher)
        .args(["validator", "--version"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(4));
    assert!(String::from_utf8_lossy(&output.stderr).contains("does not match liknon-cli"));
}

#[cfg(unix)]
#[test]
fn routed_process_preserves_context_streams_arguments_and_exit_status() {
    use std::os::{unix::ffi::OsStringExt as _, unix::fs::PermissionsExt as _};

    let temporary = TempDir::new().unwrap();
    let launcher = temporary.path().join("liknon");
    fs::copy(env!("CARGO_BIN_EXE_liknon"), &launcher).unwrap();
    let sibling = temporary.path().join("liknon-validator");
    fs::write(
        &sibling,
        format!(
            "#!/bin/sh\nset -eu\nif [ \"${{1-}}\" = --version ]; then printf 'liknon-validator {}\\n'; exit 0; fi\ncase \"${{1-}}\" in\nprobe)\n  pwd | tr -d '\\n' > \"$LIKNON_PROBE/cwd\"\n  printf %s \"$LIKNON_FORWARD_ENV\" > \"$LIKNON_PROBE/environment\"\n  printf %s \"$2\" > \"$LIKNON_PROBE/argument-one\"\n  printf %s \"$3\" > \"$LIKNON_PROBE/argument-two\"\n  cat > \"$LIKNON_PROBE/stdin\"\n  printf suite-stdout\n  printf suite-stderr >&2\n  exit 37\n  ;;\nsignal) kill -TERM $$ ;;\nesac\n",
            env!("CARGO_PKG_VERSION")
        ),
    )
    .unwrap();
    fs::set_permissions(&sibling, fs::Permissions::from_mode(0o755)).unwrap();

    let direct_probe = temporary.path().join("direct-probe");
    let routed_probe = temporary.path().join("routed-probe");
    fs::create_dir(&direct_probe).unwrap();
    fs::create_dir(&routed_probe).unwrap();
    let opaque = std::ffi::OsString::from_vec(vec![b'o', b'p', b'a', b'q', b'u', b'e', 0xff]);

    let direct = run_probe(&sibling, &[], &direct_probe, &opaque);
    let routed = run_probe(&launcher, &["validator"], &routed_probe, &opaque);
    assert_eq!(direct.status.code(), Some(37));
    assert_eq!(routed.status.code(), direct.status.code());
    assert_eq!(routed.stdout, direct.stdout);
    assert_eq!(routed.stderr, direct.stderr);
    for record in [
        "cwd",
        "environment",
        "argument-one",
        "argument-two",
        "stdin",
    ] {
        assert_eq!(
            fs::read(routed_probe.join(record)).unwrap(),
            fs::read(direct_probe.join(record)).unwrap(),
            "{record}"
        );
    }
    assert_eq!(
        fs::read(routed_probe.join("cwd")).unwrap(),
        temporary.path().as_os_str().as_encoded_bytes()
    );
    assert_eq!(
        fs::read(routed_probe.join("environment")).unwrap(),
        b"preserved"
    );
    assert_eq!(
        fs::read(routed_probe.join("argument-one")).unwrap(),
        b"two words"
    );
    assert_eq!(
        fs::read(routed_probe.join("argument-two")).unwrap(),
        opaque.as_encoded_bytes()
    );
    assert_eq!(
        fs::read(routed_probe.join("stdin")).unwrap(),
        b"stream-input"
    );

    use std::os::unix::process::ExitStatusExt as _;
    let direct_signal = Command::new(&sibling).arg("signal").status().unwrap();
    let routed_signal = Command::new(&launcher)
        .args(["validator", "signal"])
        .status()
        .unwrap();
    assert_eq!(direct_signal.signal(), Some(libc::SIGTERM));
    assert_eq!(routed_signal.signal(), direct_signal.signal());
}

#[cfg(unix)]
fn run_probe(
    executable: &Path,
    prefix: &[&str],
    probe: &Path,
    opaque: &std::ffi::OsStr,
) -> std::process::Output {
    let mut command = Command::new(executable);
    command
        .args(prefix)
        .arg("probe")
        .arg("two words")
        .arg(opaque)
        .current_dir(probe.parent().unwrap())
        .env("LIKNON_PROBE", probe)
        .env("LIKNON_FORWARD_ENV", "preserved")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"stream-input")
        .unwrap();
    child.wait_with_output().unwrap()
}

fn component_binary(name: &str) -> &'static str {
    match name {
        "liknon-validator" => env!("CARGO_BIN_EXE_liknon-validator"),
        "liknon-manager" => env!("CARGO_BIN_EXE_liknon-manager"),
        "liknon-tool" => env!("CARGO_BIN_EXE_liknon-tool"),
        _ => unreachable!(),
    }
}

fn parent_dir(path: &str) -> &str {
    Path::new(path)
        .parent()
        .and_then(Path::to_str)
        .expect("binary parent path")
}
