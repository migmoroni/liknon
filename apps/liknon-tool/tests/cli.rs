mod common;

use std::{fs, process::Command};

use common::{
    assert_success, json_output, recompute_manifest_digest, run_alias, run_tool, Fixture,
    COLLECTION_ID, EVIDENCE_ID, EVIDENCE_TEXT, EXTERNAL_WORKFLOW_ID, PROJECT_WORKFLOW_ID,
    REVIEW_BINARY, REVIEW_ID,
};

#[test]
fn direct_and_compact_entry_points_share_version_and_visual_contract() {
    let primary_version = run_tool(&["--version".to_owned()]);
    let alias_version = run_alias(&["--version"]);
    assert_success(&primary_version);
    assert_success(&alias_version);
    assert_eq!(primary_version.stdout, alias_version.stdout);

    let plain = run_tool(&["--help".to_owned()]);
    assert_success(&plain);
    assert!(!plain.stdout.contains(&0x1b));
    let plain_text = String::from_utf8(plain.stdout.clone()).unwrap();
    for palette in [
        "standard",
        "high-contrast",
        "protanopia",
        "deuteranopia",
        "tritanopia",
        "achromatopsia",
    ] {
        let colored = run_tool(&[format!("--color={palette}"), "--help".to_owned()]);
        assert_success(&colored);
        assert!(colored.stdout.contains(&0x1b), "{palette}");
        assert_eq!(
            console::strip_ansi_codes(&String::from_utf8(colored.stdout).unwrap()),
            plain_text,
            "{palette}"
        );
    }

    let low_vision = run_tool(&["--presentation=low-vision".to_owned(), "--help".to_owned()]);
    assert_success(&low_vision);
    assert!(low_vision.stdout.contains(&0x1b));

    let malformed = run_tool(&["--color=unknown".to_owned(), "--help".to_owned()]);
    assert_eq!(malformed.status.code(), Some(2));
    assert!(!malformed.stderr.contains(&0x1b));
}

#[test]
fn all_resource_operations_are_offline_read_only_and_machine_readable() {
    let fixture = Fixture::new();
    let before = fixture.snapshot();

    let mut catalog = vec!["catalog".to_owned()];
    catalog.extend(fixture.arguments());
    let catalog = json_output(catalog);
    assert_eq!(catalog["schemaVersion"], 1);
    assert_eq!(catalog["resultType"], "catalog");
    assert_eq!(catalog["resources"].as_array().unwrap().len(), 5);

    let mut search = vec!["search".to_owned(), "EVIDENCE".to_owned()];
    search.extend(fixture.arguments());
    let search = json_output(search);
    assert_eq!(search["resultType"], "search");
    assert_eq!(search["results"].as_array().unwrap().len(), 1);

    let mut show = vec!["show".to_owned(), EVIDENCE_ID.to_owned()];
    show.extend(fixture.arguments());
    let show = json_output(show);
    assert_eq!(show["resultType"], "show");
    assert_eq!(
        show["content"]["content"].as_str().unwrap(),
        String::from_utf8_lossy(EVIDENCE_TEXT).as_ref()
    );

    let mut resolve = vec!["resolve".to_owned(), COLLECTION_ID.to_owned()];
    resolve.extend(fixture.arguments());
    let resolve = json_output(resolve);
    let resolved = resolve["resources"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value["resource"]["identity"]["name"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(resolved, ["evidence", "review", "starter"]);

    let mut context = vec!["context".to_owned(), COLLECTION_ID.to_owned()];
    context.extend(fixture.arguments());
    let context = json_output(context);
    assert_eq!(context["resultType"], "context");
    assert_eq!(context["outcome"], "complete");
    assert_eq!(context["declaredFiles"], 3);

    assert_eq!(fixture.snapshot(), before);
}

#[test]
fn discovery_is_stable_for_unicode_duplicate_titles_limits_and_human_facts() {
    let fixture = Fixture::new();
    fixture.update_lock(|lock| {
        for resource in &mut lock.resources {
            let identity = resource.resource.identity.to_string();
            if identity == EVIDENCE_ID {
                resource.resource.title = "Evid\u{00ea}ncia t\u{00e9}cnica".to_owned();
                resource.resource.summary =
                    "Evid\u{00ea}ncia Unicode: valida\u{00e7}\u{00e3}o".to_owned();
            }
            if identity == PROJECT_WORKFLOW_ID || identity == EXTERNAL_WORKFLOW_ID {
                resource.resource.title = "Shared workflow title".to_owned();
            }
            recompute_manifest_digest(resource);
        }
    });

    let mut unicode = vec!["search".to_owned(), "VALIDA\u{00c7}\u{00c3}O".to_owned()];
    unicode.extend(fixture.arguments());
    let unicode = json_output(unicode);
    assert_eq!(unicode["total"], 1);
    assert_eq!(
        unicode["results"][0]["resource"]["identity"]["name"],
        "evidence"
    );

    let mut duplicates = vec![
        "search".to_owned(),
        "shared workflow title".to_owned(),
        "--limit=1".to_owned(),
    ];
    duplicates.extend(fixture.arguments());
    let duplicates = json_output(duplicates);
    assert_eq!(duplicates["total"], 2);
    assert_eq!(duplicates["results"].as_array().unwrap().len(), 1);

    let mut absent = vec!["search".to_owned(), "absent-query".to_owned()];
    absent.extend(fixture.arguments());
    let absent = json_output(absent);
    assert_eq!(absent["total"], 0);

    let mut human = vec!["catalog".to_owned(), "--limit=1".to_owned()];
    human.extend(fixture.arguments());
    let human = run_tool(&human);
    assert_success(&human);
    let human = String::from_utf8(human.stdout).unwrap();
    assert!(human.contains("origin="));
    assert!(human.contains("metadata-integrity=Verified"));
    assert!(human.contains("payload-integrity=NotInspected"));
    assert!(human.contains("revoked=false"));
}

#[test]
fn distinct_workflow_graphs_use_the_same_uninterpreted_contract() {
    let fixture = Fixture::new();
    let show = |identity: &str| {
        let mut arguments = vec!["show".to_owned(), identity.to_owned()];
        arguments.extend(fixture.arguments());
        json_output(arguments)
    };

    let project = show(PROJECT_WORKFLOW_ID);
    let external = show(EXTERNAL_WORKFLOW_ID);
    for report in [&project, &external] {
        assert_eq!(report["resultType"], "show");
        assert!(report["content"].is_null());
        assert_eq!(
            report["resource"]["resource"]["workflow"]["transitions"][0]["continuationEligible"],
            true
        );
    }
    assert_eq!(
        project["resource"]["resource"]["workflow"]["nodes"][0]["action"],
        "inspect"
    );
    assert_eq!(
        external["resource"]["resource"]["workflow"]["nodes"][0]["action"],
        "gather"
    );
}

#[test]
fn discovery_selects_the_nearest_store_and_stops_at_partial_state() {
    let fixture = Fixture::new();
    let nested = fixture.workspace.join("packages/client/src");
    fs::create_dir_all(&nested).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_liknon-tool"))
        .current_dir(&nested)
        .args(["catalog", "--format=json"])
        .output()
        .unwrap();
    assert_success(&output);
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        value["workspace"],
        fs::canonicalize(&fixture.workspace)
            .unwrap()
            .display()
            .to_string()
    );

    let partial = tempfile::TempDir::new().unwrap();
    fs::create_dir(partial.path().join(".liknon")).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_liknon-tool"))
        .current_dir(partial.path())
        .args(["catalog", "--format=json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(3));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["error"]["code"], "partial-workspace");

    let absent = tempfile::TempDir::new().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_liknon-tool"))
        .args([
            "catalog",
            "--workspace",
            absent.path().to_str().unwrap(),
            "--format=json",
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(3));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["error"]["code"], "workspace-not-found");
}

#[test]
fn malformed_resource_identity_is_a_usage_error() {
    let fixture = Fixture::new();
    let mut arguments = vec![
        "show".to_owned(),
        "not-an-identity".to_owned(),
        "--format=json".to_owned(),
    ];
    arguments.extend(fixture.arguments());
    let output = run_tool(&arguments);
    assert_eq!(output.status.code(), Some(3));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["error"]["class"], "usage");
    assert_eq!(value["error"]["code"], "invalid-invocation");
}

#[test]
fn discovery_does_not_read_payloads_but_exact_retrieval_verifies_them() {
    let fixture = Fixture::new();
    let payload = fixture.payload_path(EVIDENCE_ID, "content.md");
    let mut drift = EVIDENCE_TEXT.to_vec();
    *drift.last_mut().unwrap() = b'!';
    fs::write(&payload, drift).unwrap();

    for command in ["catalog", "search"] {
        let mut arguments = vec![command.to_owned()];
        if command == "search" {
            arguments.push("evidence".to_owned());
        }
        arguments.extend(fixture.arguments());
        assert_success(&run_tool(&arguments));
    }

    let mut show = vec!["show".to_owned(), EVIDENCE_ID.to_owned()];
    show.extend(fixture.arguments());
    show.push("--format=json".to_owned());
    let output = run_tool(&show);
    assert_eq!(output.status.code(), Some(4));
    let error: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(error["error"]["code"], "integrity-failure");
}

#[test]
fn missing_payload_is_reported_as_store_integrity_failure() {
    let fixture = Fixture::new();
    fs::remove_file(fixture.payload_path(EVIDENCE_ID, "content.md")).unwrap();

    let mut arguments = vec![
        "show".to_owned(),
        EVIDENCE_ID.to_owned(),
        "--format=json".to_owned(),
    ];
    arguments.extend(fixture.arguments());
    let output = run_tool(&arguments);
    assert_eq!(output.status.code(), Some(4));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["error"]["class"], "integrity");
    assert_eq!(value["error"]["code"], "integrity-failure");
}

#[test]
fn unsupported_persisted_schema_is_reported_as_contract_integrity_failure() {
    let fixture = Fixture::new();
    fixture.update_lock(|lock| lock.schema_version = u32::MAX);

    let mut arguments = vec!["catalog".to_owned(), "--format=json".to_owned()];
    arguments.extend(fixture.arguments());
    let output = run_tool(&arguments);
    assert_eq!(output.status.code(), Some(4));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["error"]["class"], "integrity");
    assert_eq!(value["error"]["code"], "invalid-contract");
}

#[test]
fn incomplete_manifest_and_dependency_locks_fail_contract_validation() {
    let fixture = Fixture::new();
    fixture.update_lock(|lock| {
        lock.resources
            .retain(|resource| resource.resource.identity.to_string() != EXTERNAL_WORKFLOW_ID);
    });
    let mut arguments = vec!["catalog".to_owned(), "--format=json".to_owned()];
    arguments.extend(fixture.arguments());
    let output = run_tool(&arguments);
    assert_eq!(output.status.code(), Some(4));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["error"]["code"], "invalid-contract");

    let fixture = Fixture::new();
    fixture.update_lock(|lock| {
        let review = lock
            .resources
            .iter_mut()
            .find(|resource| resource.resource.identity.to_string() == REVIEW_ID)
            .unwrap();
        review.dependencies.clear();
    });
    let mut arguments = vec![
        "resolve".to_owned(),
        REVIEW_ID.to_owned(),
        "--format=json".to_owned(),
    ];
    arguments.extend(fixture.arguments());
    let output = run_tool(&arguments);
    assert_eq!(output.status.code(), Some(4));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["error"]["code"], "invalid-contract");
}

#[test]
fn exact_retrieval_requires_explicit_binary_and_multifile_selection() {
    let fixture = Fixture::new();

    let mut ambiguous = vec!["show".to_owned(), REVIEW_ID.to_owned()];
    ambiguous.extend(fixture.arguments());
    let output = run_tool(&ambiguous);
    assert_eq!(output.status.code(), Some(3));

    let mut binary = vec![
        "show".to_owned(),
        REVIEW_ID.to_owned(),
        "--file".to_owned(),
        "sample.bin".to_owned(),
    ];
    binary.extend(fixture.arguments());
    let output = run_tool(&binary);
    assert_eq!(output.status.code(), Some(2));

    binary.push("--encoding=base64".to_owned());
    let value = json_output(binary);
    assert_eq!(value["content"]["encoding"], "base64");
    assert_eq!(value["content"]["content"], "AAECA/8=");
    assert_eq!(value["content"]["length"], REVIEW_BINARY.len());
}

#[test]
fn context_is_bounded_and_marks_all_payloads_as_untrusted() {
    let fixture = Fixture::new();
    let mut arguments = vec![
        "context".to_owned(),
        COLLECTION_ID.to_owned(),
        "--max-files=1".to_owned(),
    ];
    arguments.extend(fixture.arguments());
    let value = json_output(arguments);
    assert_eq!(value["outcome"], "partial");
    assert_eq!(value["includedFiles"], 1);
    assert_eq!(value["omissions"].as_array().unwrap().len(), 2);
    let content = &value["resources"][0]["payloads"][0];
    assert_eq!(
        content["instructionBoundary"],
        "untrusted-resource-content-below-system-host-repository-human-and-policy-authority"
    );
    assert!(content["content"].as_str().unwrap().contains('\u{1b}'));

    let mut human = vec!["show".to_owned(), EVIDENCE_ID.to_owned()];
    human.extend(fixture.arguments());
    let output = run_tool(&human);
    assert_success(&output);
    assert!(!output.stdout.contains(&0x1b));
    assert!(String::from_utf8(output.stdout)
        .unwrap()
        .contains("\\u{1b}[31mterminal-control"));
}

#[test]
fn revoked_and_incompatible_resources_are_visible_but_not_returned() {
    let fixture = Fixture::new();
    fixture.update_lock(|lock| {
        let resource = lock
            .resources
            .iter_mut()
            .find(|resource| resource.resource.identity.to_string() == EVIDENCE_ID)
            .unwrap();
        resource.resource.revoked = true;
        recompute_manifest_digest(resource);
    });

    let mut catalog = vec!["catalog".to_owned()];
    catalog.extend(fixture.arguments());
    let catalog = json_output(catalog);
    assert_eq!(catalog["resources"][0]["revoked"], true);

    let mut show = vec![
        "show".to_owned(),
        EVIDENCE_ID.to_owned(),
        "--format=json".to_owned(),
    ];
    show.extend(fixture.arguments());
    let revoked = run_tool(&show);
    assert_eq!(revoked.status.code(), Some(2));
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&revoked.stdout).unwrap()["error"]["code"],
        "revoked-resource"
    );

    fixture.update_lock(|lock| {
        let resource = lock
            .resources
            .iter_mut()
            .find(|resource| resource.resource.identity.to_string() == PROJECT_WORKFLOW_ID)
            .unwrap();
        resource.resource.revoked = true;
        recompute_manifest_digest(resource);
    });
    let mut context = vec![
        "context".to_owned(),
        PROJECT_WORKFLOW_ID.to_owned(),
        "--format=json".to_owned(),
    ];
    context.extend(fixture.arguments());
    let revoked_context = run_tool(&context);
    assert_eq!(revoked_context.status.code(), Some(2));
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&revoked_context.stdout).unwrap()["error"]
            ["code"],
        "revoked-resource"
    );

    fixture.update_lock(|lock| {
        let resource = lock
            .resources
            .iter_mut()
            .find(|resource| resource.resource.identity.to_string() == EVIDENCE_ID)
            .unwrap();
        resource.resource.revoked = false;
        resource.resource.compatibility.tool = Some(">=9".to_owned());
        recompute_manifest_digest(resource);
    });
    let incompatible = run_tool(&show);
    assert_eq!(incompatible.status.code(), Some(2));
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&incompatible.stdout).unwrap()["error"]["code"],
        "incompatible-resource"
    );
}

#[cfg(unix)]
#[test]
fn payload_symlinks_are_rejected_before_content_is_read() {
    use std::os::unix::fs::symlink;

    let fixture = Fixture::new();
    let payload = fixture.payload_path(EVIDENCE_ID, "content.md");
    let outside = fixture.workspace.parent().unwrap().join("outside.md");
    fs::write(&outside, EVIDENCE_TEXT).unwrap();
    fs::remove_file(&payload).unwrap();
    symlink(&outside, &payload).unwrap();

    let mut arguments = vec!["show".to_owned(), EVIDENCE_ID.to_owned()];
    arguments.extend(fixture.arguments());
    arguments.push("--format=json".to_owned());
    let output = run_tool(&arguments);
    assert_eq!(output.status.code(), Some(4));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["error"]["code"], "integrity-failure");
}
