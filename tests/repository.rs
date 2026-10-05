mod common;
use serde_json::json;
use std::{
    fs,
    process::Command,
    sync::{atomic::AtomicBool, Arc},
};
use tempfile::TempDir;
use workspace_validator::{
    config, contracts::report::Status, execution, planning, reporting, theme::Theme,
};

#[test]
fn repository_integrity_is_typed_and_counted_once_outside_checks() {
    let temp = TempDir::new().unwrap();
    assert!(Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(temp.path())
        .status()
        .unwrap()
        .success());
    let tool = common::process_fixture();
    let mut value = common::base_config(tool);
    value["checks"][0]["args"] = json!(["touch", "introduced"]);
    value["repository"] = json!({"provider":"git","toolId":"git","detectMutations":true});
    value["tools"].as_array_mut().unwrap().push(json!({"id":"git","program":"git","requiresTools":[],"versionArgs":["--version"],"versionParser":"firstSemver"}));
    let validated = config::load(
        Some(&common::write_config(temp.path(), &value)),
        temp.path(),
    )
    .unwrap();
    let plan = planning::target(&validated, None).unwrap();
    let outcome = execution::run(&validated, &plan, Arc::new(AtomicBool::new(false)));
    let repo = outcome.report.repository.as_ref().unwrap();
    assert_eq!(
        repo.integrity.status,
        Status::Fail,
        "repository integrity was blocked: {:?}",
        repo.integrity.reason
    );
    assert_eq!(repo.introduced.as_ref().unwrap(), &vec!["introduced"]);
    let human = reporting::result::human::render(&outcome.report, &Theme::plain());
    assert!(human.contains("GATE"));
    assert!(human.contains("Introduced paths:\n│         introduced"));
    assert!(outcome
        .report
        .checks
        .iter()
        .all(|v| v.check_id != "repository.integrity"));
    assert_eq!(
        outcome.report.summary.pass
            + outcome.report.summary.fail
            + outcome.report.summary.blocked
            + outcome.report.summary.skipped,
        outcome.report.checks.len() + 1
    );
}

#[test]
fn unavailable_repository_tool_still_produces_blocked_report() {
    let temp = TempDir::new().unwrap();
    let mut value = common::base_config("rustc");
    value["repository"] = json!({"provider":"git","toolId":"missing","detectMutations":true});
    value["tools"].as_array_mut().unwrap().push(json!({"id":"missing","program":"definitely-missing","requiresTools":[],"versionArgs":["--version"],"versionParser":"firstSemver"}));
    let validated = config::load(
        Some(&common::write_config(temp.path(), &value)),
        temp.path(),
    )
    .unwrap();
    let plan = planning::target(&validated, None).unwrap();
    let outcome = execution::run(&validated, &plan, Arc::new(AtomicBool::new(false)));
    let repo = outcome.report.repository.as_ref().unwrap();
    assert_eq!(repo.integrity.status, Status::Blocked);
    assert!(repo.before.is_none());
    assert!(repo.integrity.reason.is_some());
    let human = reporting::result::human::render(&outcome.report, &Theme::plain());
    assert!(human.contains("GATE  BLOCKED"));
    assert!(human.contains(repo.integrity.reason.as_deref().unwrap()));
}

#[test]
fn repository_evidence_separates_preexisting_state_from_run_mutations() {
    let temp = TempDir::new().unwrap();
    assert!(Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(temp.path())
        .status()
        .unwrap()
        .success());
    fs::write(temp.path().join(".git/info/exclude"), "ignored\n").unwrap();
    for (path, contents) in [
        ("pre-removed", "before"),
        ("pre-changed", "before"),
        ("pre-renamed", "before"),
        ("pre-copied", "before"),
    ] {
        fs::write(temp.path().join(path), contents).unwrap();
    }

    let tool = common::process_fixture();
    let mut value = common::base_config(tool);
    value["checks"][0]["args"] = json!(["repository-mutations"]);
    value["repository"] = json!({"provider":"git","toolId":"git","detectMutations":false});
    value["tools"].as_array_mut().unwrap().push(json!({"id":"git","program":"git","requiresTools":[],"versionArgs":["--version"],"versionParser":"firstSemver"}));
    let validated = config::load(
        Some(&common::write_config(temp.path(), &value)),
        temp.path(),
    )
    .unwrap();
    let plan = planning::target(&validated, None).unwrap();
    let outcome = execution::run(&validated, &plan, Arc::new(AtomicBool::new(false)));
    let human = reporting::result::human::render(&outcome.report, &Theme::plain());
    let json = reporting::result::json::render(&outcome.report).unwrap();
    let json: serde_json::Value = serde_json::from_str(&json).unwrap();
    let report = outcome.report.repository.unwrap();

    assert!(
        report.before.is_some(),
        "initial repository snapshot was unavailable: {:?}",
        report.integrity.reason
    );
    let before = report.before.unwrap();
    assert!(before.iter().any(|entry| entry.ends_with("pre-changed")));
    assert!(!before.iter().any(|entry| entry.ends_with("introduced")));
    assert_eq!(
        report.introduced.unwrap(),
        vec!["copied", "introduced", "renamed"]
    );
    assert_eq!(report.removed.unwrap(), vec!["pre-removed", "pre-renamed"]);
    assert_eq!(report.changed.unwrap(), vec!["pre-changed"]);
    assert!(!report
        .after
        .unwrap()
        .iter()
        .any(|entry| entry.ends_with("ignored")));
    assert_eq!(report.integrity.status, Status::Pass);
    for (heading, field) in [
        ("Introduced paths:", "introduced"),
        ("Removed paths:", "removed"),
        ("Changed paths:", "changed"),
    ] {
        assert!(human.contains(heading));
        for path in json["repository"][field].as_array().unwrap() {
            assert!(human.contains(path.as_str().unwrap()));
        }
    }
}
