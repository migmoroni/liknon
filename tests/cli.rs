mod common;
use serde_json::Value;
use std::{fs, process::Command};
#[cfg(unix)]
use std::{os::fd::OwnedFd, os::unix::net::UnixStream, process::Stdio};
use tempfile::TempDir;

const ASCII_LOGO: &str = include_str!("../assets/branding/liknon-ascii.txt");

fn ascii_logo() -> &'static str {
    ASCII_LOGO.trim_end_matches(['\r', '\n'])
}

fn command_help(arguments: &[&str]) -> String {
    command_help_with_environment(arguments, &[])
}

fn command_help_with_environment(arguments: &[&str], environment: &[(&str, &str)]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_liknon"))
        .args(arguments)
        .envs(environment.iter().copied())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{} returned {:?}: {}",
        arguments.join(" "),
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
    String::from_utf8(output.stdout).unwrap()
}

fn init_candidate(root: &std::path::Path) -> (std::path::PathBuf, Vec<u8>) {
    let mut value = common::base_config("definitely-missing");
    value["workspaceRoot"] = "..".into();
    let bytes = serde_json::to_vec_pretty(&value).unwrap();
    let path = root.join("candidate.json");
    fs::write(&path, &bytes).unwrap();
    (path, bytes)
}

#[cfg(unix)]
fn unwritable_stream() -> Stdio {
    let (reader, writer) = UnixStream::pair().unwrap();
    drop(reader);
    Stdio::from(OwnedFd::from(writer))
}

#[cfg(unix)]
fn output_with_unwritable_stdout(command: &mut Command) -> std::process::Output {
    command
        .stdout(unwritable_stream())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap()
}

#[test]
fn init_requires_a_capability_flag() {
    let temp = TempDir::new().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_liknon"))
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
    let workspace = common::canonical_path(temp.path());
    let (candidate, bytes) = init_candidate(&workspace);
    let bin = env!("CARGO_BIN_EXE_liknon");

    let created = Command::new(bin)
        .args(["init", "--config"])
        .arg(&candidate)
        .arg("--format=json")
        .current_dir(&workspace)
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
        fs::read(workspace.join(".validation/config.json")).unwrap(),
        bytes
    );

    let reused = Command::new(bin)
        .args(["init", "--config"])
        .arg(&candidate)
        .arg("--format=json")
        .current_dir(&workspace)
        .output()
        .unwrap();
    assert!(reused.status.success());
    let document: Value = serde_json::from_slice(&reused.stdout).unwrap();
    assert_eq!(document["resources"][0]["status"], "reused");
}

#[test]
fn init_reports_conflicts_and_never_overwrites() {
    let temp = TempDir::new().unwrap();
    let workspace = common::canonical_path(temp.path());
    let (candidate, _) = init_candidate(&workspace);
    fs::create_dir(workspace.join(".validation")).unwrap();
    fs::write(workspace.join(".validation/config.json"), b"different").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_liknon"))
        .args(["init", "--config"])
        .arg(candidate)
        .arg("--format=json")
        .current_dir(&workspace)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty());
    let document: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(document["status"], "conflict");
    assert_eq!(document["resources"][0]["status"], "conflict");
    assert_eq!(
        fs::read(workspace.join(".validation/config.json")).unwrap(),
        b"different"
    );
}

#[test]
fn init_uses_canonical_destination_semantics_and_starts_no_tool() {
    let temp = TempDir::new().unwrap();
    let workspace = common::canonical_path(temp.path());
    let marker = workspace.join("preflight-ran");
    let mut value = common::base_config(common::process_fixture());
    value["workspaceRoot"] = "..".into();
    value["tools"][0]["versionArgs"] = serde_json::json!(["mark-version", marker]);
    let candidate = common::write_config(&workspace, &value);
    let output = Command::new(env!("CARGO_BIN_EXE_liknon"))
        .args(["init", "--config"])
        .arg(candidate)
        .arg("--workspace")
        .arg(&workspace)
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
    let workspace_root = common::canonical_path(workspace.path());
    let outside_root = common::canonical_path(outside.path());
    let (outside_candidate, _) = init_candidate(&outside_root);
    let link = workspace_root.join("candidate-link.json");
    symlink(&outside_candidate, &link).unwrap();
    let bin = env!("CARGO_BIN_EXE_liknon");
    for candidate in [&link, &outside_candidate] {
        let output = Command::new(bin)
            .args(["init", "--config"])
            .arg(candidate)
            .arg("--workspace")
            .arg(&workspace_root)
            .arg("--format=json")
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(3));
        let document: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(document["status"], "failed");
    }
    assert!(!workspace_root.join(".validation").exists());
}

#[cfg(unix)]
#[test]
fn init_rejects_a_symlinked_destination_directory() {
    use std::os::unix::fs::symlink;
    let workspace = TempDir::new().unwrap();
    let outside = TempDir::new().unwrap();
    let workspace_root = common::canonical_path(workspace.path());
    let outside_root = common::canonical_path(outside.path());
    let (candidate, _) = init_candidate(&workspace_root);
    symlink(&outside_root, workspace_root.join(".validation")).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_liknon"))
        .args(["init", "--config"])
        .arg(candidate)
        .arg("--workspace")
        .arg(&workspace_root)
        .arg("--format=json")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let document: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(document["resources"][0]["status"], "conflict");
    assert!(!outside_root.join("config.json").exists());
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
    let output = Command::new(env!("CARGO_BIN_EXE_liknon"))
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
    let output = Command::new(env!("CARGO_BIN_EXE_liknon"))
        .args(["validate", "--config"])
        .arg(path)
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(!stdout.contains('\u{1b}'));
    assert!(stdout.starts_with("╭─ Liknon · Result"));
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
    let bin = env!("CARGO_BIN_EXE_liknon");

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
    let bin = env!("CARGO_BIN_EXE_liknon");
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
fn visual_execution_options_are_rejected_by_unsupported_commands() {
    let binary = env!("CARGO_BIN_EXE_liknon");
    for arguments in [
        &["init", "--color"][..],
        &["config", "validate", "--color=standard"][..],
        &["list", "--presentation=low-vision"][..],
        &["knowledge", "catalog", "--presentation=standard"][..],
    ] {
        let output = Command::new(binary).args(arguments).output().unwrap();
        assert_eq!(output.status.code(), Some(3), "{}", arguments.join(" "));
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(
            stderr.contains("apply to command execution only for validate and check"),
            "{}:\n{stderr}",
            arguments.join(" ")
        );
    }
}

#[test]
fn help_lists_palettes_and_presentations_as_distinct_options() {
    let stdout = command_help(&["--help"]);
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
    for description in [
        "Standard semantic terminal palette",
        "Higher-contrast semantic terminal palette",
        "Semantic palette adapted for protanopia",
        "Semantic palette adapted for deuteranopia",
        "Semantic palette adapted for tritanopia",
        "Semantic palette that does not rely on hue distinctions",
        "Compact terminal layout and standard emphasis",
        "Expanded spacing and stronger emphasis for low-vision readability",
    ] {
        assert!(
            stdout.contains(description),
            "missing {description:?} in:\n{stdout}"
        );
    }
    assert!(!stdout.contains("- plain:"));
}

#[test]
fn visual_help_controls_are_documented_only_for_supported_execution() {
    for arguments in [["-h"].as_slice(), ["--help"].as_slice()] {
        let help = command_help(arguments);
        assert!(help.contains("--color[=<PALETTE>]"), "{help}");
        assert!(help.contains("--presentation=<MODE>"), "{help}");
    }

    for arguments in [&["validate", "--help"][..], &["check", "--help"][..]] {
        let help = command_help(arguments);
        assert!(help.contains("--color[=<PALETTE>]"), "{help}");
        assert!(help.contains("--presentation=<MODE>"), "{help}");
        assert!(help.contains("See root --help"), "{help}");
        assert!(
            !help.contains("Standard semantic terminal palette"),
            "{help}"
        );
    }

    for arguments in [
        &["init", "--help"][..],
        &["config", "--help"][..],
        &["config", "validate", "--help"][..],
        &["list", "--help"][..],
        &["explain", "--help"][..],
        &["explain", "group", "--help"][..],
        &["explain", "suite", "--help"][..],
        &["explain", "check", "--help"][..],
        &["schema", "--help"][..],
        &["knowledge", "--help"][..],
        &["knowledge", "catalog", "--help"][..],
        &["knowledge", "show", "--help"][..],
    ] {
        let help = command_help(arguments);
        assert!(
            !help.contains("--color"),
            "{}:\n{help}",
            arguments.join(" ")
        );
        assert!(
            !help.contains("--presentation"),
            "{}:\n{help}",
            arguments.join(" ")
        );
    }
}

#[test]
fn hidden_visual_controls_still_style_focused_help() {
    let plain = command_help_with_environment(&["config", "--help"], &[("NO_COLOR", "1")]);
    let colored = command_help_with_environment(
        &["config", "--color=standard", "--help"],
        &[("NO_COLOR", "1")],
    );
    assert!(colored.contains('\u{1b}'));
    assert_eq!(console::strip_ansi_codes(&colored), plain);

    let low_vision = command_help_with_environment(
        &["config", "--presentation=low-vision", "--help"],
        &[("NO_COLOR", "1")],
    );
    let low_vision_plain = console::strip_ansi_codes(&low_vision);
    assert!(low_vision.contains('\u{1b}'));
    assert!(low_vision_plain.lines().count() > plain.lines().count());
    assert!(!low_vision_plain.contains("--presentation"));
    assert!(!low_vision_plain.contains("--color"));

    let root_standard = command_help_with_environment(&["--help"], &[("NO_COLOR", "1")]);
    let root_low_vision = command_help_with_environment(
        &["--presentation=low-vision", "--help"],
        &[("NO_COLOR", "1")],
    );
    assert!(root_low_vision.contains('\u{1b}'));
    assert!(
        console::strip_ansi_codes(&root_low_vision).lines().count() > root_standard.lines().count()
    );
}

#[test]
fn low_vision_short_help_separates_neighboring_commands_and_options() {
    let rendered =
        command_help_with_environment(&["-h", "--presentation=low-vision"], &[("NO_COLOR", "1")]);
    let plain = console::strip_ansi_codes(&rendered);

    assert!(
        plain.contains(
            "init       Provisions validated consumer-owned resources without executing checks\n\n  validate"
        ),
        "{plain}"
    );
    assert!(
        plain.contains("Selects the layout and emphasis of human CLI output [possible values: standard, low-vision]\n\n  -h, --help"),
        "{plain}"
    );
}

#[test]
fn root_help_describes_every_command() {
    let stdout = command_help(&["--help"]);
    for description in [
        "Provisions validated consumer-owned resources without executing checks",
        "Runs a configured group or suite after configuration and tool preflight",
        "Runs one reusable check directly at the workspace root",
        "Inspects and validates configuration without executing configured programs",
        "Lists configured groups, suites, checks, and tools without executing them",
        "Explains one configured group, suite, or check without executing it",
        "Prints the generated JSON Schema for a public CLI contract",
        "Reads validation guidance embedded in this binary without loading configuration",
        "Prints help at the root or after any command",
        "The -h flag prints a compact summary; --help prints complete details",
    ] {
        assert!(
            stdout.contains(description),
            "missing {description:?} in:\n{stdout}"
        );
    }
    assert_eq!(
        stdout
            .matches(
                "Supports --color and --presentation for human validation output; see root Options"
            )
            .count(),
        2,
        "{stdout}"
    );
}

#[test]
fn help_option_is_documented_once_in_each_requested_help_page() {
    let root = command_help(&["--help"]);
    assert_eq!(root.matches("-h, --help").count(), 1, "{root}");

    for arguments in [
        &["init", "--help"][..],
        &["validate", "--help"][..],
        &["check", "--help"][..],
        &["config", "--help"][..],
        &["config", "validate", "--help"][..],
        &["list", "--help"][..],
        &["explain", "--help"][..],
        &["explain", "group", "--help"][..],
        &["explain", "suite", "--help"][..],
        &["explain", "check", "--help"][..],
        &["schema", "--help"][..],
        &["knowledge", "--help"][..],
        &["knowledge", "catalog", "--help"][..],
        &["knowledge", "show", "--help"][..],
    ] {
        let focused = command_help(arguments);
        assert_eq!(
            focused.matches("-h, --help").count(),
            1,
            "{}:\n{focused}",
            arguments.join(" ")
        );
    }
}

#[test]
fn root_short_help_stays_compact_while_long_help_flattens_the_command_tree() {
    let logo = ascii_logo();
    let short = command_help(&["-h"]);
    assert!(short.contains("Commands:"), "{short}");
    assert!(short.contains("validate"), "{short}");
    assert!(!short.contains("liknon validate:"), "{short}");
    assert!(
        short.contains("Usage: liknon <COMMAND> [OPTIONS]"),
        "{short}"
    );
    assert!(short.contains("--color[=<PALETTE>]"), "{short}");
    assert!(short.contains("--presentation=<MODE>"), "{short}");
    assert!(!short.contains(logo), "{short}");

    let long = command_help(&["--help"]);
    assert!(
        long.starts_with(&format!("{logo}\n\nRuns declarative")),
        "{long}"
    );
    let commands_position = long.find("Commands:").expect("root command index");
    let options_position = long.find("Options:").expect("root options");
    let first_separator_position = long.find(&"─".repeat(78)).expect("first command section");
    assert!(commands_position < options_position, "{long}");
    assert!(options_position < first_separator_position, "{long}");
    let command_index = &long[commands_position..options_position];
    for command in [
        "init",
        "validate",
        "check",
        "config",
        "list",
        "explain",
        "schema",
        "knowledge",
    ] {
        assert!(
            command_index.lines().any(|line| {
                line.trim_start()
                    .strip_prefix(command)
                    .and_then(|remainder| remainder.chars().next())
                    .is_some_and(char::is_whitespace)
            }),
            "missing {command:?} from command index:\n{command_index}"
        );
    }
    let top_level_commands = [
        "init:",
        "validate:",
        "check:",
        "config:",
        "list:",
        "explain:",
        "schema:",
        "knowledge:",
    ];
    let nested_commands = [
        "config validate:",
        "explain group:",
        "explain suite:",
        "explain check:",
        "knowledge catalog:",
        "knowledge show:",
    ];
    let separator = "─".repeat(78);
    let nested_separator = "─".repeat(39);
    assert_eq!(
        long.lines().filter(|line| *line == separator).count(),
        top_level_commands.len(),
        "{long}"
    );
    assert_eq!(
        long.lines()
            .filter(|line| *line == nested_separator)
            .count(),
        nested_commands.len(),
        "{long}"
    );
    for command in top_level_commands {
        assert!(long.contains(command), "missing {command:?} in:\n{long}");
        assert!(
            long.contains(&format!("{separator}\n{command}")),
            "missing separator before {command:?} in:\n{long}"
        );
    }
    for command in nested_commands {
        assert!(long.contains(command), "missing {command:?} in:\n{long}");
        assert!(
            long.contains(&format!("{nested_separator}\n{command}")),
            "missing nested separator before {command:?} in:\n{long}"
        );
    }
    assert!(long.contains("Usage: liknon <COMMAND> [OPTIONS]"), "{long}");
    for usage in [
        "Usage: liknon init [OPTIONS]",
        "Usage: liknon validate [OPTIONS] [TARGET]",
        "Usage: liknon check [OPTIONS] <CHECK_ID>",
        "Usage: liknon config <COMMAND>",
        "Usage: liknon config validate [OPTIONS]",
        "Usage: liknon list [OPTIONS]",
        "Usage: liknon explain <COMMAND>",
        "Usage: liknon explain group [OPTIONS] <GROUP_ID>",
        "Usage: liknon explain suite [OPTIONS] <SUITE_ID>",
        "Usage: liknon explain check [OPTIONS] <CHECK_ID>",
        "Usage: liknon schema <CONTRACT>",
        "Usage: liknon knowledge <COMMAND>",
        "Usage: liknon knowledge catalog [OPTIONS]",
        "Usage: liknon knowledge show <DOCUMENT_ID>",
    ] {
        assert!(long.contains(usage), "missing {usage:?} in:\n{long}");
    }
    for old_heading in [
        "liknon init:",
        "liknon config validate:",
        "liknon explain suite:",
        "liknon knowledge show:",
    ] {
        assert!(
            !long.contains(old_heading),
            "unexpected heading {old_heading:?} in:\n{long}"
        );
    }
    for generated in [
        "liknon help:",
        "liknon config help:",
        "liknon explain help:",
        "liknon knowledge help:",
    ] {
        assert!(
            !long.contains(generated),
            "unexpected {generated:?} in:\n{long}"
        );
    }
    for option in [
        "--config <CONFIG>",
        "--workspace <WORKSPACE>",
        "--format <FORMAT>",
        "--color[=<PALETTE>]",
        "--presentation=<MODE>",
        "--tree",
    ] {
        assert!(long.contains(option), "missing {option:?} in:\n{long}");
    }
    for argument in [
        "[TARGET]",
        "<CHECK_ID>",
        "<GROUP_ID>",
        "<SUITE_ID>",
        "<CONTRACT>",
        "<DOCUMENT_ID>",
    ] {
        assert!(long.contains(argument), "missing {argument:?} in:\n{long}");
    }
    assert!(
        long.contains("Without this flag, output contains no ANSI color"),
        "{long}"
    );

    let focused = command_help(&["validate", "--help"]);
    assert!(!focused.contains(&separator), "{focused}");
    assert!(!focused.contains(logo), "{focused}");
}

#[test]
fn root_low_vision_help_adds_space_after_hierarchical_separators() {
    let rendered = command_help_with_environment(
        &["--presentation=low-vision", "--help"],
        &[("NO_COLOR", "1")],
    );
    let plain = console::strip_ansi_codes(&rendered);

    assert!(
        plain.starts_with(&format!("{}\n\n\nRuns declarative", ascii_logo())),
        "{plain}"
    );

    let full_separator = "─".repeat(78);
    let nested_separator = "─".repeat(39);
    let lines = plain.lines().collect::<Vec<_>>();
    let mut separator_count = 0;
    for (index, line) in lines.iter().enumerate() {
        if *line == full_separator || *line == nested_separator {
            separator_count += 1;
            assert_eq!(
                lines.get(index + 1),
                Some(&""),
                "separator lacks its low-vision gap:\n{plain}"
            );
        }
    }
    assert_eq!(separator_count, 14, "{plain}");
}

#[test]
fn root_help_guidance_lives_in_the_top_options_block() {
    let rendered = command_help(&["--help"]);
    let guidance = "Prints help at the root or after any command";
    let guidance_position = rendered.find(guidance).expect("root help guidance");
    let first_command_separator = rendered
        .find(&"─".repeat(78))
        .expect("first command separator");

    assert!(guidance_position < first_command_separator, "{rendered}");
    assert!(
        !rendered.trim_end().ends_with("complete details."),
        "{rendered}"
    );
}

#[test]
fn generated_help_subcommands_are_disabled() {
    let binary = env!("CARGO_BIN_EXE_liknon");
    for arguments in [
        &["help"][..],
        &["config", "help"][..],
        &["explain", "help"][..],
        &["knowledge", "help"][..],
    ] {
        let output = Command::new(binary).args(arguments).output().unwrap();
        assert_eq!(output.status.code(), Some(3), "{}", arguments.join(" "));
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(
            stderr.contains("unrecognized subcommand 'help'"),
            "{}:\n{stderr}",
            arguments.join(" ")
        );
    }
}

#[test]
fn short_and_long_help_share_accessible_palette_semantics() {
    let cases: &[(&[&str], &[&str])] = &[
        (&["-h"], &["--color=standard", "-h"]),
        (&["--help"], &["--color=standard", "--help"]),
        (&["validate", "-h"], &["validate", "--color=standard", "-h"]),
        (
            &["validate", "--help"],
            &["validate", "--color=standard", "--help"],
        ),
    ];

    for (plain_arguments, colored_arguments) in cases {
        let plain = command_help_with_environment(plain_arguments, &[("NO_COLOR", "1")]);
        assert!(!plain.contains('\u{1b}'), "{}", plain_arguments.join(" "));

        let colored = command_help_with_environment(colored_arguments, &[("NO_COLOR", "1")]);
        assert!(
            colored.contains('\u{1b}'),
            "{}",
            colored_arguments.join(" ")
        );
        assert_eq!(
            console::strip_ansi_codes(&colored),
            plain,
            "{}",
            colored_arguments.join(" ")
        );
    }

    let plain = command_help(&["--help"]);
    for palette in [
        "standard",
        "high-contrast",
        "protanopia",
        "deuteranopia",
        "tritanopia",
        "achromatopsia",
    ] {
        let option = format!("--color={palette}");
        let colored = command_help(&[&option, "--help"]);
        assert!(colored.contains('\u{1b}'), "palette {palette}");
        assert_eq!(
            console::strip_ansi_codes(&colored),
            plain,
            "palette {palette}"
        );
    }
}

#[test]
fn parser_diagnostics_follow_the_global_palette() {
    let binary = env!("CARGO_BIN_EXE_liknon");
    let plain = Command::new(binary)
        .arg("unknown")
        .env("NO_COLOR", "1")
        .output()
        .unwrap();
    let colored = Command::new(binary)
        .args(["--color=high-contrast", "unknown"])
        .env("NO_COLOR", "1")
        .output()
        .unwrap();

    assert_eq!(plain.status.code(), Some(3));
    assert_eq!(colored.status.code(), Some(3));
    assert!(plain.stdout.is_empty());
    assert!(colored.stdout.is_empty());
    assert!(!plain.stderr.contains(&0x1b));
    assert!(colored.stderr.contains(&0x1b));
    assert_eq!(
        console::strip_ansi_codes(&String::from_utf8_lossy(&colored.stderr)),
        String::from_utf8_lossy(&plain.stderr)
    );
}

#[test]
fn command_help_describes_every_argument_and_option() {
    let cases: &[(&[&str], &[&str])] = &[
        (
            &["init", "--help"],
            &[
                "Path to the complete configuration candidate to install",
                "does not discover an existing workspace configuration",
                "Sets the destination workspace boundary",
                "must remain within this boundary",
                "Selects the initialization result format",
                "machine-readable initialization result without terminal styling",
            ],
        ),
        (
            &["validate", "--help"],
            &[
                "Group or suite ID; defaults to the configured default group",
                "Uses an explicit configuration file instead of discovery",
                "discovery walks upward from the process current directory",
                "Selects the validation report format",
                "JSON emits only the versioned ValidationReport",
                "Styles human validation output with ANSI color",
                "See root --help for palettes and rendering behavior",
                "Selects the layout and emphasis of human validation output",
                "See root --help for presentation modes and rendering behavior",
            ],
        ),
        (
            &["check", "--help"],
            &[
                "ID of the configured check to run",
                "Uses an explicit configuration file instead of discovery",
                "discovery walks upward from the process current directory",
                "Selects the validation report format",
                "JSON emits only the versioned ValidationReport",
                "Styles human validation output with ANSI color",
                "See root --help for palettes and rendering behavior",
                "Selects the layout and emphasis of human validation output",
                "See root --help for presentation modes and rendering behavior",
            ],
        ),
        (
            &["config", "--help"],
            &["Validates configuration and filesystem semantics without starting configured tools"],
        ),
        (
            &["config", "validate", "--help"],
            &[
                "Uses an explicit configuration file instead of discovery",
                "discovery walks upward from the process current directory",
            ],
        ),
        (
            &["list", "--help"],
            &[
                "Also prints the configured default group's reachable execution hierarchy",
                "Without this flag, the command lists the available groups",
                "Uses an explicit configuration file instead of discovery",
            ],
        ),
        (
            &["explain", "--help"],
            &[
                "Shows membership, hierarchy, resolved invocations, and required tools for a group",
                "Shows directory, membership, resolved invocations, and required tools for a suite",
                "Shows the template and resolved direct and suite-bound invocations for a check",
            ],
        ),
        (
            &["explain", "group", "--help"],
            &[
                "ID of the configured group to explain",
                "Uses an explicit configuration file instead of discovery",
            ],
        ),
        (
            &["explain", "suite", "--help"],
            &[
                "ID of the configured suite to explain",
                "Uses an explicit configuration file instead of discovery",
            ],
        ),
        (
            &["explain", "check", "--help"],
            &[
                "ID of the configured check to explain",
                "Uses an explicit configuration file instead of discovery",
            ],
        ),
        (
            &["schema", "--help"],
            &["Public contract whose JSON Schema is written to standard output"],
        ),
        (
            &["knowledge", "--help"],
            &[
                "Lists the embedded catalog used for progressive discovery",
                "Writes one canonical Markdown document selected by stable ID",
            ],
        ),
        (
            &["knowledge", "catalog", "--help"],
            &[
                "Selects the embedded catalog format",
                "exact embedded canonical catalog for machine consumption",
            ],
        ),
        (
            &["knowledge", "show", "--help"],
            &["Stable document ID returned by the embedded knowledge catalog"],
        ),
    ];

    for (arguments, descriptions) in cases {
        let stdout = command_help(arguments);
        for description in *descriptions {
            assert!(
                stdout.contains(description),
                "missing {description:?} from `{}` help:\n{stdout}",
                arguments.join(" ")
            );
        }
    }
}

#[test]
fn root_version_is_a_successful_cli_outcome() {
    let binary = env!("CARGO_BIN_EXE_liknon");
    let output = Command::new(binary).arg("--version").output().unwrap();
    assert!(
        output.status.success(),
        "--version returned {:?}: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!output.stdout.is_empty());
}

#[test]
fn embedded_knowledge_is_exact_read_only_and_independent_from_configuration() {
    let temp = TempDir::new().unwrap();
    let binary = env!("CARGO_BIN_EXE_liknon");
    let canonical_root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/validation/knowledge");
    let canonical_catalog = fs::read(canonical_root.join("catalog.json")).unwrap();

    let json = Command::new(binary)
        .args(["knowledge", "catalog", "--format=json"])
        .current_dir(temp.path())
        .output()
        .unwrap();
    assert!(json.status.success());
    assert!(json.stderr.is_empty());
    assert_eq!(json.stdout, canonical_catalog);

    let human = Command::new(binary)
        .args(["knowledge", "catalog"])
        .current_dir(temp.path())
        .output()
        .unwrap();
    assert!(human.status.success());
    let human = String::from_utf8(human.stdout).unwrap();
    assert!(human.starts_with("Embedded validation knowledge ("));

    let catalog: Value = serde_json::from_slice(&canonical_catalog).unwrap();
    for document in catalog["documents"].as_array().unwrap() {
        let id = document["id"].as_str().unwrap();
        let path = document["path"].as_str().unwrap();
        let output = Command::new(binary)
            .args(["knowledge", "show", id])
            .current_dir(temp.path())
            .output()
            .unwrap();
        assert!(output.status.success(), "{id}");
        assert!(output.stderr.is_empty(), "{id}");
        assert_eq!(
            output.stdout,
            fs::read(canonical_root.join(path)).unwrap(),
            "{id}"
        );
    }

    assert!(!temp.path().join(".validation").exists());
    assert_eq!(fs::read_dir(temp.path()).unwrap().count(), 0);
}

#[test]
fn knowledge_show_rejects_non_ids_paths_and_unknown_documents() {
    let temp = TempDir::new().unwrap();
    let binary = env!("CARGO_BIN_EXE_liknon");
    for selector in [
        "../escape",
        "/absolute",
        "reference/tools/cargo.md",
        "tool.not-present",
    ] {
        let output = Command::new(binary)
            .args(["knowledge", "show", selector])
            .current_dir(temp.path())
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(3), "{selector}");
        assert!(output.stdout.is_empty(), "{selector}");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("knowledge document ID"),
            "{selector}"
        );
    }
    assert_eq!(fs::read_dir(temp.path()).unwrap().count(), 0);
}

#[cfg(unix)]
#[test]
fn knowledge_output_failure_returns_four_without_panicking() {
    let mut command = Command::new(env!("CARGO_BIN_EXE_liknon"));
    command.args(["knowledge", "show", "tool.cargo"]);
    let output = output_with_unwritable_stdout(&mut command);
    assert_eq!(output.status.code(), Some(4));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("cannot write standard output"), "{stderr}");
    assert!(!stderr.contains("panicked"), "{stderr}");
}

#[test]
fn config_validate_and_inspection_do_not_run_declared_tools() {
    let temp = TempDir::new().unwrap();
    let value = common::base_config("definitely-missing");
    let path = common::write_config(temp.path(), &value);
    let bin = env!("CARGO_BIN_EXE_liknon");
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
    let output = Command::new(env!("CARGO_BIN_EXE_liknon"))
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
    let bin = env!("CARGO_BIN_EXE_liknon");
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
    let output = Command::new(env!("CARGO_BIN_EXE_liknon"))
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
    let status = Command::new(env!("CARGO_BIN_EXE_liknon"))
        .args(["config", "validate", "--config"])
        .arg(path)
        .status()
        .unwrap();
    assert_eq!(status.code(), Some(3));
    let status = Command::new(env!("CARGO_BIN_EXE_liknon"))
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
    let mut command = Command::new(env!("CARGO_BIN_EXE_liknon"));
    command.args(["list", "--config"]).arg(path);
    let output = output_with_unwritable_stdout(&mut command);
    assert_eq!(output.status.code(), Some(4));
    assert!(String::from_utf8_lossy(&output.stderr).contains("internal executor failure"));
}

#[cfg(unix)]
#[test]
fn explain_output_failures_return_four_without_panicking() {
    let temp = TempDir::new().unwrap();
    let value = common::base_config("rustc");
    let path = common::write_config(temp.path(), &value);
    for (kind, id) in [
        ("group", "all"),
        ("suite", "fixture"),
        ("check", "fixture.check"),
    ] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_liknon"));
        command.args(["explain", kind, id, "--config"]).arg(&path);
        let output = output_with_unwritable_stdout(&mut command);
        assert_eq!(output.status.code(), Some(4), "{kind}");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("internal executor failure"),
            "{kind}: {stderr}"
        );
        assert!(!stderr.contains("panicked"), "{kind}: {stderr}");
    }
}

#[test]
fn unknown_explain_targets_remain_semantic_failures() {
    let temp = TempDir::new().unwrap();
    let value = common::base_config("rustc");
    let path = common::write_config(temp.path(), &value);
    for kind in ["group", "suite", "check"] {
        let output = Command::new(env!("CARGO_BIN_EXE_liknon"))
            .args(["explain", kind, "missing", "--config"])
            .arg(&path)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(3), "{kind}");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("invalid configuration"), "{kind}: {stderr}");
        assert!(
            !stderr.contains("internal executor failure"),
            "{kind}: {stderr}"
        );
    }
}

#[cfg(unix)]
#[test]
fn final_json_report_write_failure_returns_four_without_panicking() {
    let temp = TempDir::new().unwrap();
    let mut value = common::base_config("rustc");
    value["checks"][0]["args"] = serde_json::json!(["--version"]);
    let path = common::write_config(temp.path(), &value);
    let mut command = Command::new(env!("CARGO_BIN_EXE_liknon"));
    command
        .args(["validate", "--format=json", "--config"])
        .arg(path);
    let output = output_with_unwritable_stdout(&mut command);
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
    let workspace = common::canonical_path(temp.path());
    let (candidate, _) = init_candidate(&workspace);
    let mut command = Command::new(env!("CARGO_BIN_EXE_liknon"));
    command
        .args(["init", "--format=json", "--config"])
        .arg(candidate)
        .current_dir(&workspace);
    let output = output_with_unwritable_stdout(&mut command);
    assert_eq!(output.status.code(), Some(4));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("cannot write standard output"), "{stderr}");
    assert!(!stderr.contains("panicked"), "{stderr}");
    assert!(!stderr.contains("\"schemaVersion\""), "{stderr}");
    assert!(workspace.join(".validation/config.json").is_file());
}

#[cfg(unix)]
#[test]
fn failed_stdout_and_stderr_sinks_preserve_exit_four() {
    let temp = TempDir::new().unwrap();
    let mut value = common::base_config("rustc");
    value["checks"][0]["args"] = serde_json::json!(["--version"]);
    let path = common::write_config(temp.path(), &value);
    let status = Command::new(env!("CARGO_BIN_EXE_liknon"))
        .args(["validate", "--format=json", "--config"])
        .arg(path)
        .stdout(unwritable_stream())
        .stderr(unwritable_stream())
        .spawn()
        .unwrap()
        .wait()
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
    let child = Command::new(env!("CARGO_BIN_EXE_liknon"))
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
