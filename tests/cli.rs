mod common;
use serde_json::Value;
#[cfg(unix)]
use std::process::Stdio;
use std::{fs, process::Command};
use tempfile::TempDir;

fn init_candidate(root: &std::path::Path) -> (std::path::PathBuf, Vec<u8>) {
    let mut value = common::base_config("definitely-missing");
    value["workspaceRoot"] = "..".into();
    let bytes = serde_json::to_vec_pretty(&value).unwrap();
    let path = root.join("candidate.json");
    fs::write(&path, &bytes).unwrap();
    (path, bytes)
}

#[test]
fn init_requires_a_capability_flag() {
    let temp = TempDir::new().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_workspace-validator"))
        .arg("init")
        .current_dir(temp.path())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(3));
    assert!(!temp.path().join(".validation").exists());
}

#[test]
fn init_installs_exact_validated_bytes_and_reuses_them() {
    let temp = TempDir::new().unwrap();
    let (candidate, bytes) = init_candidate(temp.path());
    let bin = env!("CARGO_BIN_EXE_workspace-validator");

    let created = Command::new(bin)
        .args(["init", "--config"])
        .arg(&candidate)
        .arg("--format=json")
        .current_dir(temp.path())
        .output()
        .unwrap();
    assert!(
        created.status.success(),
        "{}",
        String::from_utf8_lossy(&created.stderr)
    );
    assert!(created.stderr.is_empty());
    let document: Value = serde_json::from_slice(&created.stdout).unwrap();
    assert_eq!(document["schemaVersion"], 1);
    assert_eq!(document["status"], "success");
    assert_eq!(document["resources"][0]["kind"], "config");
    assert_eq!(document["resources"][0]["status"], "created");
    assert!(document["resources"][0]["digest"].as_str().unwrap().len() == 64);
    assert_eq!(
        fs::read(temp.path().join(".validation/config.json")).unwrap(),
        bytes
    );

    let reused = Command::new(bin)
        .args(["init", "--config"])
        .arg(&candidate)
        .arg("--format=json")
        .current_dir(temp.path())
        .output()
        .unwrap();
    assert!(reused.status.success());
    let document: Value = serde_json::from_slice(&reused.stdout).unwrap();
    assert_eq!(document["resources"][0]["status"], "reused");
}

#[test]
fn init_reports_conflicts_and_never_overwrites() {
    let temp = TempDir::new().unwrap();
    let (candidate, _) = init_candidate(temp.path());
    fs::create_dir(temp.path().join(".validation")).unwrap();
    fs::write(temp.path().join(".validation/config.json"), b"different").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_workspace-validator"))
        .args(["init", "--config"])
        .arg(candidate)
        .arg("--format=json")
        .current_dir(temp.path())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty());
    let document: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(document["status"], "conflict");
    assert_eq!(document["resources"][0]["status"], "conflict");
    assert_eq!(
        fs::read(temp.path().join(".validation/config.json")).unwrap(),
        b"different"
    );
}

#[test]
fn init_uses_canonical_destination_semantics_and_starts_no_tool() {
    let temp = TempDir::new().unwrap();
    let marker = temp.path().join("preflight-ran");
    let mut value = common::base_config(common::process_fixture());
    value["workspaceRoot"] = "..".into();
    value["tools"][0]["versionArgs"] = serde_json::json!(["mark-version", marker]);
    let candidate = common::write_config(temp.path(), &value);
    let output = Command::new(env!("CARGO_BIN_EXE_workspace-validator"))
        .args(["init", "--config"])
        .arg(candidate)
        .arg("--workspace")
        .arg(temp.path())
        .current_dir(std::env::temp_dir())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!marker.exists());
}

#[cfg(unix)]
#[test]
fn init_rejects_symlinked_candidates_and_workspace_escape() {
    use std::os::unix::fs::symlink;
    let workspace = TempDir::new().unwrap();
    let outside = TempDir::new().unwrap();
    let (outside_candidate, _) = init_candidate(outside.path());
    let link = workspace.path().join("candidate-link.json");
    symlink(&outside_candidate, &link).unwrap();
    let bin = env!("CARGO_BIN_EXE_workspace-validator");
    for candidate in [&link, &outside_candidate] {
        let output = Command::new(bin)
            .args(["init", "--config"])
            .arg(candidate)
            .arg("--workspace")
            .arg(workspace.path())
            .arg("--format=json")
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(3));
        let document: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(document["status"], "failed");
    }
    assert!(!workspace.path().join(".validation").exists());
}

#[cfg(unix)]
#[test]
fn init_rejects_a_symlinked_destination_directory() {
    use std::os::unix::fs::symlink;
    let workspace = TempDir::new().unwrap();
    let outside = TempDir::new().unwrap();
    let (candidate, _) = init_candidate(workspace.path());
    symlink(outside.path(), workspace.path().join(".validation")).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_workspace-validator"))
        .args(["init", "--config"])
        .arg(candidate)
        .arg("--workspace")
        .arg(workspace.path())
        .arg("--format=json")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let document: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(document["resources"][0]["status"], "conflict");
    assert!(!outside.path().join("config.json").exists());
}

#[test]
fn discovers_current_config_and_emits_portable_json() {
    let temp = TempDir::new().unwrap();
    let validation = temp.path().join(".validation");
    let child = temp.path().join("nested");
    fs::create_dir_all(&validation).unwrap();
    fs::create_dir_all(&child).unwrap();
    let mut value = common::base_config("rustc");
    value["workspaceRoot"] = "..".into();
    value["checks"][0]["args"] = serde_json::json!(["--version"]);
    fs::write(
        validation.join("config.json"),
        serde_json::to_vec(&value).unwrap(),
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_workspace-validator"))
        .args(["validate", "--format", "json"])
        .current_dir(child)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
    assert!(!output.stdout.contains(&0x1b));
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["schemaVersion"], 4);
    assert_eq!(report["workspaceRoot"], ".");
    assert_eq!(report["selection"]["kind"], "group");
    assert!(report["checks"][0].get("workingDirectory").is_none());
    assert_eq!(report["groups"][0]["id"], "all");
    assert_eq!(report["suites"][0]["workingDirectory"], ".");
}

#[test]
fn human_report_uses_group_suite_and_check_vocabulary() {
    let temp = TempDir::new().unwrap();
    let mut value = common::base_config("rustc");
    value["checks"][0]["args"] = serde_json::json!(["--version"]);
    let path = common::write_config(temp.path(), &value);
    let output = Command::new(env!("CARGO_BIN_EXE_workspace-validator"))
        .args(["validate", "--config"])
        .arg(path)
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(!stdout.contains('\u{1b}'));
    assert!(stdout.starts_with("╭─ Workspace Validator · Result"));
    for expected in ["GROUP", "SUITE", "CHECK", "Counted outcomes"] {
        assert!(
            stdout.contains(expected),
            "missing {expected:?} in:\n{stdout}"
        );
    }
}

#[test]
fn palettes_and_presentations_compose_locally_and_preserve_semantics() {
    let temp = TempDir::new().unwrap();
    let mut value = common::base_config("rustc");
    value["checks"][0]["args"] = serde_json::json!(["--version"]);
    let path = common::write_config(temp.path(), &value);
    let bin = env!("CARGO_BIN_EXE_workspace-validator");

    let plain = Command::new(bin)
        .args(["validate", "fixture", "--config"])
        .arg(&path)
        .env("NO_COLOR", "1")
        .output()
        .unwrap();
    assert!(plain.status.success());
    assert!(!plain.stdout.contains(&0x1b));
    let plain_text = String::from_utf8(plain.stdout).unwrap();

    for palette in [
        "standard",
        "high-contrast",
        "protanopia",
        "deuteranopia",
        "tritanopia",
        "achromatopsia",
    ] {
        let output = Command::new(bin)
            .args(["validate", "fixture"])
            .arg(format!("--color={palette}"))
            .arg("--config")
            .arg(&path)
            .env("NO_COLOR", "1")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "palette {palette}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(output.stdout.contains(&0x1b), "palette {palette}");
        let styled = String::from_utf8(output.stdout).unwrap();
        let stripped = console::strip_ansi_codes(&styled);
        for token in [
            "Selection: suite fixture",
            "SUITE PASS  fixture",
            "CHECK PASS  fixture.check",
            "Validation complete",
        ] {
            assert!(stripped.contains(token), "palette {palette}: {stripped}");
            assert!(plain_text.contains(token));
        }
    }

    for presentation in ["standard", "low-vision"] {
        let output = Command::new(bin)
            .args(["validate", "fixture"])
            .arg(format!("--presentation={presentation}"))
            .arg("--config")
            .arg(&path)
            .env("NO_COLOR", "1")
            .output()
            .unwrap();
        assert!(output.status.success(), "presentation {presentation}");
        if presentation == "standard" {
            assert!(!output.stdout.contains(&0x1b));
        } else {
            assert!(output.stdout.contains(&0x1b));
        }
        let styled = String::from_utf8(output.stdout).unwrap();
        let stripped = console::strip_ansi_codes(&styled);
        assert!(stripped.contains("CHECK PASS  fixture.check"));
    }

    let composed = Command::new(bin)
        .args([
            "validate",
            "fixture",
            "--color=deuteranopia",
            "--presentation=low-vision",
            "--config",
        ])
        .arg(&path)
        .env("NO_COLOR", "1")
        .output()
        .unwrap();
    assert!(composed.status.success());
    assert!(composed.stdout.contains(&0x1b));

    let bare = Command::new(bin)
        .args(["validate", "fixture", "--color", "--config"])
        .arg(&path)
        .output()
        .unwrap();
    assert!(bare.status.success());
    assert!(bare.stdout.contains(&0x1b));

    let check = Command::new(bin)
        .args([
            "check",
            "fixture.check",
            "--color=high-contrast",
            "--presentation=low-vision",
            "--config",
        ])
        .arg(&path)
        .output()
        .unwrap();
    assert!(check.status.success());
    assert!(check.stdout.contains(&0x1b));
}

#[test]
fn visual_usage_rejects_ambiguous_unknown_and_json_combinations() {
    let temp = TempDir::new().unwrap();
    let path = common::write_config(temp.path(), &common::base_config("rustc"));
    let bin = env!("CARGO_BIN_EXE_workspace-validator");
    for args in [
        vec!["validate", "fixture", "--color", "deuteranopia"],
        vec!["validate", "fixture", "--color=unknown"],
        vec!["validate", "fixture", "--color=low-vision"],
        vec!["validate", "fixture", "--presentation"],
        vec!["validate", "fixture", "--presentation", "low-vision"],
        vec!["validate", "fixture", "--presentation=high-contrast"],
        vec!["validate", "fixture", "--presentation=unknown"],
        vec!["validate", "fixture", "--format=json", "--color"],
        vec![
            "validate",
            "fixture",
            "--format=json",
            "--presentation=standard",
        ],
        vec![
            "check",
            "fixture.check",
            "--format=json",
            "--color=standard",
            "--presentation=low-vision",
        ],
    ] {
        let output = Command::new(bin)
            .args(args)
            .arg("--config")
            .arg(&path)
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(3),
            "stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn help_lists_palettes_and_presentations_as_distinct_options() {
    let output = Command::new(env!("CARGO_BIN_EXE_workspace-validator"))
        .args(["validate", "--help"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    for palette in [
        "standard",
        "high-contrast",
        "protanopia",
        "deuteranopia",
        "tritanopia",
        "achromatopsia",
    ] {
        assert!(stdout.contains(palette), "missing {palette} in:\n{stdout}");
    }
    assert!(stdout.contains("--color[=<PALETTE>]"), "{stdout}");
    assert!(stdout.contains("--presentation=<MODE>"), "{stdout}");
    assert!(
        stdout.contains("[possible values: standard, high-contrast, protanopia, deuteranopia, tritanopia, achromatopsia]"),
        "{stdout}"
    );
    assert!(
        stdout.contains("[possible values: standard, low-vision]"),
        "{stdout}"
    );
    assert!(!stdout.contains("[possible values: standard, high-contrast, low-vision]"));
    assert!(!stdout.contains("plain"));
}

#[test]
fn root_help_and_version_are_successful_cli_outcomes() {
    let binary = env!("CARGO_BIN_EXE_workspace-validator");
    for argument in ["--help", "--version"] {
        let output = Command::new(binary).arg(argument).output().unwrap();
        assert!(
            output.status.success(),
            "{argument} returned {:?}: {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(!output.stdout.is_empty());
    }
}

#[test]
fn config_validate_and_inspection_do_not_run_declared_tools() {
    let temp = TempDir::new().unwrap();
    let value = common::base_config("definitely-missing");
    let path = common::write_config(temp.path(), &value);
    let bin = env!("CARGO_BIN_EXE_workspace-validator");
    for args in [
        vec!["config", "validate", "--config"],
        vec!["list", "--tree", "--config"],
        vec!["explain", "group", "all", "--config"],
        vec!["explain", "suite", "fixture", "--config"],
        vec!["explain", "check", "fixture.check", "--config"],
    ] {
        let output = Command::new(bin).args(args).arg(&path).output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn explain_suite_reports_membership_context_commands_and_tools() {
    let temp = TempDir::new().unwrap();
    fs::create_dir(temp.path().join("nested")).unwrap();
    let mut value = common::base_config("definitely-missing");
    value["defaultGroup"] = "root".into();
    value["checks"][0]["args"] = serde_json::json!(["run", "{context}"]);
    value["suites"] = serde_json::json!([
        {"id":"shared","label":"Shared","description":"Shared suite.","workingDirectory":"nested","checks":[{"checkId":"fixture.check","parameters":{"context":"--context"},"dependsOn":[]}]}
    ]);
    value["groups"] = serde_json::json!([
        {"id":"branch.a","label":"Branch A","description":"First branch.","members":[{"kind":"suite","id":"shared"}]},
        {"id":"branch.b","label":"Branch B","description":"Second branch.","members":[{"kind":"suite","id":"shared"}]},
        {"id":"root","label":"Root","description":"Root group.","members":[{"kind":"group","id":"branch.a"},{"kind":"group","id":"branch.b"}]}
    ]);
    let path = common::write_config(temp.path(), &value);
    let output = Command::new(env!("CARGO_BIN_EXE_workspace-validator"))
        .args(["explain", "suite", "shared", "--config"])
        .arg(path)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    for expected in [
        "SUITE shared",
        "Effective directory: nested",
        "GROUP root -> GROUP branch.a -> SUITE shared",
        "GROUP root -> GROUP branch.b -> SUITE shared",
        "SUITE shared / CHECK fixture.check",
        "Directory: nested",
        "Parameters: {\"context\":\"--context\"}",
        "Depends on: <none>",
        "Prerequisites: fixture",
        "\"run\" \"--context\"",
        "Required tools:",
        "fixture: definitely-missing",
        "Version command: \"definitely-missing\" \"--version\"",
        "Requires tools: <none>",
    ] {
        assert!(
            stdout.contains(expected),
            "missing {expected:?} in:\n{stdout}"
        );
    }
    assert!(!stdout.contains("Paths: Some"));
}

#[test]
fn explain_group_and_check_share_complete_invocation_context() {
    let temp = TempDir::new().unwrap();
    fs::create_dir(temp.path().join("nested")).unwrap();
    let mut value = common::base_config("definitely-missing");
    value["checks"] = serde_json::json!([
        {"id":"base","label":"Base","description":"Base check.","toolId":"fixture","args":["base"],"requiresTools":[],"timeoutSeconds":5},
        {"id":"dependent","label":"Dependent","description":"Dependent check.","toolId":"fixture","args":["run","{target}"],"requiresTools":[],"timeoutSeconds":5}
    ]);
    value["suites"] = serde_json::json!([{
        "id":"fixture","label":"Fixture","description":"Fixture suite.","workingDirectory":"nested",
        "checks":[
            {"checkId":"base","parameters":{},"dependsOn":[]},
            {"checkId":"dependent","parameters":{"target":["--one","two words"]},"dependsOn":["base"]}
        ]
    }]);
    let path = common::write_config(temp.path(), &value);
    let bin = env!("CARGO_BIN_EXE_workspace-validator");
    let group = Command::new(bin)
        .args(["explain", "group", "all", "--config"])
        .arg(&path)
        .output()
        .unwrap();
    let check = Command::new(bin)
        .args(["explain", "check", "dependent", "--config"])
        .arg(path)
        .output()
        .unwrap();
    assert!(group.status.success());
    assert!(check.status.success());
    let group = String::from_utf8(group.stdout).unwrap();
    let check = String::from_utf8(check.stdout).unwrap();
    for output in [&group, &check] {
        for expected in [
            "SUITE fixture / CHECK dependent",
            "Directory: nested",
            "Parameters: {\"target\":[\"--one\",\"two words\"]}",
            "Depends on: base",
            "Command: \"definitely-missing\" \"run\" \"--one\" \"two words\"",
            "Prerequisites: fixture",
        ] {
            assert!(
                output.contains(expected),
                "missing {expected:?} in:\n{output}"
            );
        }
    }
    for expected in [
        "SUITE fixture / CHECK base",
        "Parameters: {}",
        "Depends on: <none>",
    ] {
        assert!(
            group.contains(expected),
            "missing {expected:?} in:\n{group}"
        );
    }
    for expected in [
        "DIRECT / CHECK dependent",
        "Directory: .",
        "Parameters: {}",
        "Depends on: <none>",
    ] {
        assert!(
            check.contains(expected),
            "missing {expected:?} in:\n{check}"
        );
    }
}

#[test]
fn invalid_configuration_starts_no_tool_preflight() {
    let temp = TempDir::new().unwrap();
    let marker = temp.path().join("preflight-ran");
    let tool = common::process_fixture();
    let mut value = common::base_config(tool);
    value["tools"][0]["versionArgs"] = serde_json::json!(["mark-version", marker]);
    value["suites"][0]["workingDirectory"] = "missing".into();
    let path = common::write_config(temp.path(), &value);
    let output = Command::new(env!("CARGO_BIN_EXE_workspace-validator"))
        .args(["validate", "--config"])
        .arg(path)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(3));
    assert!(!marker.exists());
}

#[test]
fn invalid_configuration_and_usage_return_three() {
    let temp = TempDir::new().unwrap();
    let mut value = common::base_config("rustc");
    value["schemaVersion"] = 1.into();
    let path = common::write_config(temp.path(), &value);
    let status = Command::new(env!("CARGO_BIN_EXE_workspace-validator"))
        .args(["config", "validate", "--config"])
        .arg(path)
        .status()
        .unwrap();
    assert_eq!(status.code(), Some(3));
    let status = Command::new(env!("CARGO_BIN_EXE_workspace-validator"))
        .arg("unknown")
        .status()
        .unwrap();
    assert_eq!(status.code(), Some(3));
}

#[cfg(unix)]
#[test]
fn inspection_output_failure_returns_exit_four() {
    let temp = TempDir::new().unwrap();
    let mut value = common::base_config("rustc");
    let tools = value["tools"].as_array_mut().unwrap();
    for index in 0..1000 {
        tools.push(serde_json::json!({
            "id": format!("tool{index}"),
            "program": "tool",
            "requiresTools": [],
            "versionArgs": ["--version"],
            "versionParser": "firstSemver"
        }));
    }
    let path = common::write_config(temp.path(), &value);
    let output = Command::new("sh")
        .args([
            "-c",
            "exec 1>/dev/full; exec \"$1\" list --config \"$2\"",
            "workspace-validator-test",
            env!("CARGO_BIN_EXE_workspace-validator"),
        ])
        .arg(path)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(4));
    assert!(String::from_utf8_lossy(&output.stderr).contains("internal executor failure"));
}

#[cfg(unix)]
#[test]
fn final_json_report_write_failure_returns_four_without_panicking() {
    let temp = TempDir::new().unwrap();
    let mut value = common::base_config("rustc");
    value["checks"][0]["args"] = serde_json::json!(["--version"]);
    let path = common::write_config(temp.path(), &value);
    let output = Command::new("sh")
        .args([
            "-c",
            "exec 1>/dev/full; exec \"$1\" validate --format=json --config \"$2\"",
            "workspace-validator-test",
            env!("CARGO_BIN_EXE_workspace-validator"),
        ])
        .arg(path)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(4));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("cannot write standard output"), "{stderr}");
    assert!(!stderr.contains("panicked"), "{stderr}");
    assert!(!stderr.contains("\"schemaVersion\""), "{stderr}");
}

#[cfg(unix)]
#[test]
fn final_init_json_write_failure_returns_four_without_panicking() {
    let temp = TempDir::new().unwrap();
    let (candidate, _) = init_candidate(temp.path());
    let output = Command::new("sh")
        .args([
            "-c",
            "exec 1>/dev/full; exec \"$1\" init --format=json --config \"$2\"",
            "workspace-validator-test",
            env!("CARGO_BIN_EXE_workspace-validator"),
        ])
        .arg(candidate)
        .current_dir(temp.path())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(4));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("cannot write standard output"), "{stderr}");
    assert!(!stderr.contains("panicked"), "{stderr}");
    assert!(!stderr.contains("\"schemaVersion\""), "{stderr}");
    assert!(temp.path().join(".validation/config.json").is_file());
}

#[cfg(unix)]
#[test]
fn failed_stdout_and_stderr_sinks_preserve_exit_four() {
    let temp = TempDir::new().unwrap();
    let mut value = common::base_config("rustc");
    value["checks"][0]["args"] = serde_json::json!(["--version"]);
    let path = common::write_config(temp.path(), &value);
    let status = Command::new("sh")
        .args([
            "-c",
            "exec 1>/dev/full 2>/dev/full; exec \"$1\" validate --format=json --config \"$2\"",
            "workspace-validator-test",
            env!("CARGO_BIN_EXE_workspace-validator"),
        ])
        .arg(path)
        .status()
        .unwrap();
    assert_eq!(status.code(), Some(4));
}

#[cfg(unix)]
#[test]
fn cli_translates_sigint_to_exit_one_hundred_thirty() {
    use std::time::{Duration, Instant};

    let temp = TempDir::new().unwrap();
    let tool = common::process_fixture();
    let started = temp.path().join("started");
    let descendant = temp.path().join("descendant");
    let mut value = common::base_config(tool);
    value["checks"][0]["args"] =
        serde_json::json!(["spawn-descendant", descendant, started, "0", "5000", "5000"]);
    let path = common::write_config(temp.path(), &value);
    let child = Command::new(env!("CARGO_BIN_EXE_workspace-validator"))
        .args(["validate", "--format=json", "--config"])
        .arg(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    while !started.exists() && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(10));
    }
    assert!(started.exists(), "fixture check did not start");
    assert!(Command::new("kill")
        .args(["-INT", &child.id().to_string()])
        .status()
        .unwrap()
        .success());
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(130));
    assert!(output.stderr.is_empty());
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["checks"][0]["status"], "fail");
    assert_eq!(report["checks"][0]["reason"], "check interrupted");
}
