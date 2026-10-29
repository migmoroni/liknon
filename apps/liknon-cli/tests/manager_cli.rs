#[path = "manager_common/mod.rs"]
mod common;

use std::{fs, process::Command};

use liknon_resource_contracts::{
    ResourceDependency, ResourceIdentity, ResourceKind, TrustClass, WorkflowMetadata, WorkflowNode,
};

use common::{
    assert_success, resource, run_manager, run_manager_from, run_routed, write_local_resource,
    Fixture,
};

#[test]
fn direct_and_routed_entry_points_expose_the_same_suite_version() {
    let primary = run_manager(&["--version".to_owned()]);
    let routed = run_routed(&["--version"]);
    assert_success(&primary);
    assert_success(&routed);
    assert_eq!(primary.stdout, routed.stdout);
}

#[test]
fn shared_visual_contract_styles_help_and_rejects_machine_output() {
    let plain = run_manager(&["--help".to_owned()]);
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
        let colored = Command::new(env!("CARGO_BIN_EXE_liknon-manager"))
            .args([format!("--color={palette}"), "--help".to_owned()])
            .env("NO_COLOR", "1")
            .output()
            .unwrap();
        assert_success(&colored);
        assert!(colored.stdout.contains(&0x1b), "{palette}");
        assert_eq!(
            console::strip_ansi_codes(&String::from_utf8(colored.stdout).unwrap()),
            plain_text,
            "{palette}"
        );
    }

    let colored = Command::new(env!("CARGO_BIN_EXE_liknon-manager"))
        .args(["--color=standard", "--help"])
        .env("NO_COLOR", "1")
        .output()
        .unwrap();

    let low_vision = run_manager(&["--presentation=low-vision".to_owned(), "--help".to_owned()]);
    assert_success(&low_vision);
    assert!(low_vision.stdout.contains(&0x1b));

    let focused_plain = run_manager(&["init".to_owned(), "--help".to_owned()]);
    let focused_colored = run_manager(&[
        "init".to_owned(),
        "--help".to_owned(),
        "--color=standard".to_owned(),
    ]);
    assert_success(&focused_plain);
    assert_success(&focused_colored);
    assert!(focused_colored.stdout.contains(&0x1b));
    assert_eq!(
        console::strip_ansi_codes(&String::from_utf8(focused_colored.stdout).unwrap()),
        String::from_utf8(focused_plain.stdout).unwrap()
    );

    let malformed = run_manager(&["--color=unknown".to_owned(), "--help".to_owned()]);
    assert_eq!(malformed.status.code(), Some(2));
    assert!(!malformed.stderr.contains(&0x1b));

    let incompatible = run_manager(&[
        "schema".to_owned(),
        "resource".to_owned(),
        "--format=json".to_owned(),
        "--color=standard".to_owned(),
    ]);
    assert_eq!(incompatible.status.code(), Some(3));
    assert!(!incompatible.stdout.contains(&0x1b));
    let error: serde_json::Value = serde_json::from_slice(&incompatible.stdout).unwrap();
    assert_eq!(error["error"]["class"], "usage");

    let routed = run_routed(&["--color=standard", "--help"]);
    assert_success(&routed);
    assert_eq!(routed.stdout, colored.stdout);

    let fixture = Fixture::new();
    let mut colored_init = vec!["init".to_owned(), "--color=standard".to_owned()];
    colored_init.extend(fixture.workspace_arguments());
    let colored_init = run_manager(&colored_init);
    assert_success(&colored_init);
    assert!(colored_init.stdout.contains(&0x1b));
    assert!(
        !console::strip_ansi_codes(&String::from_utf8(colored_init.stdout).unwrap()).is_empty()
    );
}

#[test]
fn every_operation_has_human_and_json_output_contracts() {
    let fixture = Fixture::new();
    fixture.publish(
        1,
        vec![resource(
            ResourceKind::Knowledge,
            "evidence",
            "1.0.0",
            b"evidence version one\n",
        )],
    );

    let mut init = vec!["init".to_owned()];
    init.extend(fixture.workspace_arguments());
    assert_human_and_json(&init);

    let mut search = vec!["search".to_owned(), "evidence".to_owned()];
    search.extend(fixture.hub_arguments());
    assert_human_and_json(&search);

    let mut info = vec!["info".to_owned(), "knowledge:official/evidence".to_owned()];
    info.extend(fixture.hub_arguments());
    assert_human_and_json(&info);

    let mut install = vec![
        "install".to_owned(),
        "knowledge:official/evidence".to_owned(),
        "--dry-run".to_owned(),
    ];
    install.extend(fixture.workspace_arguments());
    install.extend(fixture.hub_arguments());
    assert_human_and_json(&install);

    let mut commit = vec![
        "install".to_owned(),
        "knowledge:official/evidence".to_owned(),
        "--yes".to_owned(),
    ];
    commit.extend(fixture.workspace_arguments());
    commit.extend(fixture.hub_arguments());
    assert_success(&run_manager(&commit));

    let mut installed_info = vec![
        "info".to_owned(),
        "knowledge:official/evidence".to_owned(),
        "--installed".to_owned(),
    ];
    installed_info.extend(fixture.workspace_arguments());
    assert_human_and_json(&installed_info);

    let mut update = vec!["update".to_owned(), "--dry-run".to_owned()];
    update.extend(fixture.workspace_arguments());
    update.extend(fixture.hub_arguments());
    assert_human_and_json(&update);

    let mut list = vec!["list".to_owned()];
    list.extend(fixture.workspace_arguments());
    assert_human_and_json(&list);

    let mut verify = vec!["verify".to_owned(), "--offline".to_owned()];
    verify.extend(fixture.workspace_arguments());
    assert_human_and_json(&verify);

    let mut materialize = vec![
        "materialize".to_owned(),
        "knowledge:official/evidence".to_owned(),
        "--payload".to_owned(),
        "content.md".to_owned(),
        "--destination".to_owned(),
        "docs/evidence.md".to_owned(),
        "--dry-run".to_owned(),
    ];
    materialize.extend(fixture.workspace_arguments());
    assert_human_and_json(&materialize);

    let mut remove = vec![
        "remove".to_owned(),
        "knowledge:official/evidence".to_owned(),
        "--dry-run".to_owned(),
    ];
    remove.extend(fixture.workspace_arguments());
    assert_human_and_json(&remove);

    assert_human_and_json(&["schema".to_owned(), "resource".to_owned()]);
}

#[test]
fn discovery_selects_the_nearest_initialized_workspace() {
    let fixture = Fixture::new();
    let nested = fixture.workspace.join("packages/application");
    let deep = nested.join("src/domain");
    fs::create_dir_all(&deep).unwrap();

    for root in [&fixture.workspace, &nested] {
        assert_success(&run_manager(&[
            "init".to_owned(),
            "--workspace".to_owned(),
            root.display().to_string(),
        ]));
    }

    let output = run_manager_from(&deep, &["list".to_owned(), "--format=json".to_owned()]);
    assert_success(&output);
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        report["workspace"],
        fs::canonicalize(&nested).unwrap().display().to_string()
    );

    let missing = Fixture::new();
    assert_eq!(
        run_manager_from(&missing.workspace, &["list".to_owned()])
            .status
            .code(),
        Some(3)
    );
}

fn assert_human_and_json(arguments: &[String]) {
    let human = run_manager(arguments);
    assert_success(&human);
    assert!(!human.stdout.is_empty());
    assert!(!human.stdout.contains(&0x1b));

    let mut json_arguments = arguments.to_vec();
    json_arguments.push("--format=json".to_owned());
    let json = run_manager(&json_arguments);
    assert_success(&json);
    assert!(!json.stdout.contains(&0x1b));
    assert!(!json.stderr.contains(&0x1b));
    let value: serde_json::Value = serde_json::from_slice(&json.stdout).unwrap();
    assert_eq!(value["schemaVersion"], 1);
}

#[test]
fn installs_updates_verifies_and_removes_authenticated_resources() {
    let fixture = Fixture::new();
    fixture.publish(
        1,
        vec![resource(
            ResourceKind::Knowledge,
            "evidence",
            "1.0.0",
            b"evidence version one\n",
        )],
    );

    let mut init = vec!["init".to_owned()];
    init.extend(fixture.workspace_arguments());
    assert_success(&run_manager(&init));

    let mut install = vec![
        "install".to_owned(),
        "knowledge:official/evidence@^1".to_owned(),
        "--yes".to_owned(),
        "--format=json".to_owned(),
    ];
    install.extend(fixture.workspace_arguments());
    install.extend(fixture.hub_arguments());
    let installed = run_manager(&install);
    assert_success(&installed);
    let installed_json: serde_json::Value =
        serde_json::from_slice(&installed.stdout).expect("valid install JSON");
    assert_eq!(installed_json["outcome"], "completed");
    assert_eq!(installed_json["plan"]["committed"], true);

    let retained = fixture
        .workspace
        .join(".liknon/resources/.metadata/catalog.json");
    assert!(retained.is_file());
    let payload = fixture
        .workspace
        .join(".liknon/resources/knowledge/official/evidence/1.0.0/content.md");
    assert_eq!(fs::read(&payload).unwrap(), b"evidence version one\n");

    let mut offline = vec![
        "verify".to_owned(),
        "--offline".to_owned(),
        "--format=json".to_owned(),
    ];
    offline.extend(fixture.workspace_arguments());
    let verified = run_manager(&offline);
    assert_success(&verified);
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&verified.stdout).unwrap()["online"],
        false
    );

    fixture.publish(
        2,
        vec![
            resource(
                ResourceKind::Knowledge,
                "evidence",
                "1.1.0",
                b"evidence version two\n",
            ),
            resource(
                ResourceKind::Knowledge,
                "evidence",
                "1.0.0",
                b"evidence version one\n",
            ),
        ],
    );
    let mut update = vec!["update".to_owned(), "--yes".to_owned()];
    update.extend(fixture.workspace_arguments());
    update.extend(fixture.hub_arguments());
    assert_success(&run_manager(&update));
    assert!(fixture
        .workspace
        .join(".liknon/resources/knowledge/official/evidence/1.1.0/content.md")
        .is_file());

    let mut list = vec!["list".to_owned(), "--format=json".to_owned()];
    list.extend(fixture.workspace_arguments());
    let listed = run_manager(&list);
    assert_success(&listed);
    let listed: serde_json::Value = serde_json::from_slice(&listed.stdout).unwrap();
    assert_eq!(listed["resources"][0]["locked"], "1.1.0");

    let projection = fixture.workspace.join("docs/evidence.md");
    let mut materialize = vec![
        "materialize".to_owned(),
        "knowledge:official/evidence".to_owned(),
        "--payload".to_owned(),
        "content.md".to_owned(),
        "--destination".to_owned(),
        "docs/evidence.md".to_owned(),
        "--yes".to_owned(),
    ];
    materialize.extend(fixture.workspace_arguments());
    assert_success(&run_manager(&materialize));
    assert_eq!(fs::read(&projection).unwrap(), b"evidence version two\n");

    let mut remove = vec![
        "remove".to_owned(),
        "knowledge:official/evidence".to_owned(),
        "--yes".to_owned(),
    ];
    remove.extend(fixture.workspace_arguments());
    assert_success(&run_manager(&remove));
    assert!(!fixture
        .workspace
        .join(".liknon/resources/knowledge/official/evidence/1.1.0")
        .exists());
    assert!(!projection.exists());
}

#[test]
fn installs_one_exact_transitive_graph() {
    let fixture = Fixture::new();
    let evidence = resource(
        ResourceKind::Knowledge,
        "evidence",
        "1.0.0",
        b"evidence version one\n",
    );
    let mut review = resource(ResourceKind::Skill, "review", "1.0.0", b"review skill\n");
    review.dependencies.push(ResourceDependency {
        identity: evidence.identity.clone(),
        requirement: "^1".to_owned(),
        optional: false,
    });
    fixture.publish(1, vec![review, evidence]);
    let mut init = vec!["init".to_owned()];
    init.extend(fixture.workspace_arguments());
    assert_success(&run_manager(&init));
    let mut install = vec![
        "install".to_owned(),
        "skill:official/review@^1".to_owned(),
        "--yes".to_owned(),
        "--format=json".to_owned(),
    ];
    install.extend(fixture.workspace_arguments());
    install.extend(fixture.hub_arguments());

    let output = run_manager(&install);
    assert_success(&output);
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        report["plan"]["resultingResources"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(report["plan"]["dependencies"].as_array().unwrap().len(), 1);
    assert_eq!(report["plan"]["dependencies"][0]["to"]["kind"], "knowledge");
    assert_eq!(
        report["plan"]["dependencies"][0]["to"]["namespace"],
        "official"
    );
    assert_eq!(report["plan"]["dependencies"][0]["to"]["name"], "evidence");
}

#[test]
fn installs_explicit_local_resources_without_contacting_a_hub() {
    let fixture = Fixture::new();
    let source = fixture.workspace.join("local-resource");
    let mut local = resource(
        ResourceKind::Knowledge,
        "local-guide",
        "1.0.0",
        b"local guidance\n",
    );
    local.identity =
        ResourceIdentity::new(ResourceKind::Knowledge, "local", "local-guide").unwrap();
    local.trust.class = TrustClass::Local;
    local.trust.publisher = "Local workspace owner".to_owned();
    write_local_resource(&source, &local, b"local guidance\n");

    let mut init = vec!["init".to_owned()];
    init.extend(fixture.workspace_arguments());
    assert_success(&run_manager(&init));
    let mut install = vec![
        "install".to_owned(),
        "--local".to_owned(),
        source.display().to_string(),
        "--yes".to_owned(),
    ];
    install.extend(fixture.workspace_arguments());
    assert_success(&run_manager(&install));

    assert_eq!(
        fs::read(
            fixture
                .workspace
                .join(".liknon/resources/knowledge/local/local-guide/1.0.0/content.md")
        )
        .unwrap(),
        b"local guidance\n"
    );
}

#[test]
fn community_resources_require_explicit_consent() {
    let fixture = Fixture::new();
    let mut community = resource(ResourceKind::Skill, "review", "1.0.0", b"review skill\n");
    community.identity = ResourceIdentity::new(ResourceKind::Skill, "community", "review").unwrap();
    community.trust.class = TrustClass::Community;
    fixture.publish(1, vec![community]);
    let mut init = vec!["init".to_owned()];
    init.extend(fixture.workspace_arguments());
    assert_success(&run_manager(&init));
    let mut install = vec![
        "install".to_owned(),
        "skill:community/review".to_owned(),
        "--yes".to_owned(),
    ];
    install.extend(fixture.workspace_arguments());
    install.extend(fixture.hub_arguments());
    assert_eq!(run_manager(&install).status.code(), Some(2));
    install.push("--allow-community".to_owned());
    assert_success(&run_manager(&install));
}

#[test]
fn installs_unrelated_workflow_shapes_without_activation_or_namespace_privilege() {
    let fixture = Fixture::new();
    let mut official_profile = resource(ResourceKind::AgentProfile, "reviewer", "1.0.0", b"");
    official_profile.payloads.clear();
    let mut external_profile = resource(ResourceKind::AgentProfile, "auditor", "1.0.0", b"");
    external_profile.identity =
        ResourceIdentity::new(ResourceKind::AgentProfile, "external", "auditor").unwrap();
    external_profile.payloads.clear();
    external_profile.trust.class = TrustClass::Verified;

    let mut official = resource(ResourceKind::Workflow, "irregular", "1.0.0", b"");
    official.payloads.clear();
    official.dependencies = vec![ResourceDependency {
        identity: official_profile.identity.clone(),
        requirement: "^1".to_owned(),
        optional: false,
    }];
    official.compatibility.ai_bootstrap = Some("^1".to_owned());
    official.compatibility.decision_policy = Some("^1".to_owned());
    official.workflow = Some(WorkflowMetadata {
        entry_nodes: vec!["prepare".to_owned()],
        terminal_nodes: vec!["finish".to_owned()],
        nodes: vec![
            workflow_node("prepare", "collect", &official_profile.identity),
            workflow_node("finish", "report", &official_profile.identity),
        ],
        transitions: vec![liknon_resource_contracts::WorkflowTransition {
            id: "after-evidence".to_owned(),
            from: "prepare".to_owned(),
            to: "finish".to_owned(),
            precondition: Some("evidence-ready".to_owned()),
            continuation_eligible: true,
        }],
        artifacts: Vec::new(),
    });

    let mut external = resource(ResourceKind::Workflow, "alternate", "1.0.0", b"");
    external.identity =
        ResourceIdentity::new(ResourceKind::Workflow, "external", "alternate").unwrap();
    external.payloads.clear();
    external.trust.class = TrustClass::Verified;
    external.dependencies = vec![ResourceDependency {
        identity: external_profile.identity.clone(),
        requirement: "=1.0.0".to_owned(),
        optional: false,
    }];
    external.compatibility.ai_bootstrap = Some("^1".to_owned());
    external.compatibility.decision_policy = Some("^1".to_owned());
    external.workflow = Some(WorkflowMetadata {
        entry_nodes: vec!["opening".to_owned()],
        terminal_nodes: vec!["opening".to_owned()],
        nodes: vec![workflow_node(
            "opening",
            "inspect",
            &external_profile.identity,
        )],
        transitions: Vec::new(),
        artifacts: Vec::new(),
    });

    fixture.publish(
        1,
        vec![official_profile, external_profile, official, external],
    );
    let mut init = vec!["init".to_owned()];
    init.extend(fixture.workspace_arguments());
    assert_success(&run_manager(&init));
    let mut install = vec![
        "install".to_owned(),
        "workflow:official/irregular".to_owned(),
        "workflow:external/alternate".to_owned(),
        "--yes".to_owned(),
    ];
    install.extend(fixture.workspace_arguments());
    install.extend(fixture.hub_arguments());
    assert_success(&run_manager(&install));

    let lock: serde_json::Value = serde_json::from_slice(
        &fs::read(fixture.workspace.join(".liknon/resources/lock.json")).unwrap(),
    )
    .unwrap();
    let workflows = lock["resources"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|resource| resource["resource"]["identity"]["kind"] == "workflow")
        .collect::<Vec<_>>();
    assert_eq!(workflows.len(), 2);
    assert!(workflows
        .iter()
        .all(|resource| resource["compatibility"]["activatable"] == false));
    assert!(workflows.iter().all(|resource| {
        resource["compatibility"]["activationBlockers"]
            .as_array()
            .unwrap()
            .iter()
            .any(|value| value == "host-capabilities-not-evaluated")
    }));
}

fn workflow_node(id: &str, action: &str, profile: &ResourceIdentity) -> WorkflowNode {
    WorkflowNode {
        id: id.to_owned(),
        action: action.to_owned(),
        agent_profiles: vec![profile.clone()],
        tool_operations: Vec::new(),
        effects: vec![format!("{action}-effect")],
        required_capabilities: Vec::new(),
    }
}

#[test]
fn dry_run_and_search_are_workspace_read_only() {
    let fixture = Fixture::new();
    fixture.publish(
        1,
        vec![resource(
            ResourceKind::Skill,
            "review",
            "1.0.0",
            b"review skill\n",
        )],
    );
    let mut init = vec!["init".to_owned()];
    init.extend(fixture.workspace_arguments());
    assert_success(&run_manager(&init));
    let manifest = fixture.workspace.join(".liknon/resources/manifest.json");
    let before = fs::read(&manifest).unwrap();

    let mut search = vec!["search".to_owned(), "review".to_owned()];
    search.extend(fixture.hub_arguments());
    assert_success(&run_manager(&search));

    let mut install = vec![
        "install".to_owned(),
        "skill:official/review".to_owned(),
        "--dry-run".to_owned(),
        "--format=json".to_owned(),
    ];
    install.extend(fixture.workspace_arguments());
    install.extend(fixture.hub_arguments());
    let planned = run_manager(&install);
    assert_success(&planned);
    let planned: serde_json::Value = serde_json::from_slice(&planned.stdout).unwrap();
    assert_eq!(planned["outcome"], "planned");
    assert_eq!(planned["plan"]["requests"][0]["requirement"], "*");
    assert!(planned["plan"]["dependencies"].is_array());
    assert!(planned["plan"]["files"]
        .as_array()
        .unwrap()
        .iter()
        .any(|file| file["path"]
            .as_str()
            .unwrap()
            .ends_with("skills/official/review/1.0.0/content.md")));
    assert_eq!(planned["plan"]["verifiedBytes"], 13);
    assert!(planned["plan"]["decisions"]
        .as_array()
        .unwrap()
        .iter()
        .any(|decision| decision == "approve-complete-plan"));
    assert_eq!(fs::read(&manifest).unwrap(), before);
}

#[test]
fn list_distinguishes_missing_and_drifted_state_including_materializations() {
    let fixture = Fixture::new();
    fixture.publish(
        1,
        vec![resource(
            ResourceKind::Knowledge,
            "evidence",
            "1.0.0",
            b"evidence version one\n",
        )],
    );
    let mut init = vec!["init".to_owned()];
    init.extend(fixture.workspace_arguments());
    assert_success(&run_manager(&init));
    let mut install = vec![
        "install".to_owned(),
        "knowledge:official/evidence".to_owned(),
        "--yes".to_owned(),
    ];
    install.extend(fixture.workspace_arguments());
    install.extend(fixture.hub_arguments());
    assert_success(&run_manager(&install));

    let mut materialize = vec![
        "materialize".to_owned(),
        "knowledge:official/evidence".to_owned(),
        "--payload".to_owned(),
        "content.md".to_owned(),
        "--destination".to_owned(),
        "docs/evidence.md".to_owned(),
        "--yes".to_owned(),
    ];
    materialize.extend(fixture.workspace_arguments());
    assert_success(&run_manager(&materialize));

    let mut list = vec!["list".to_owned(), "--format=json".to_owned()];
    list.extend(fixture.workspace_arguments());
    let listed = run_manager(&list);
    assert_success(&listed);
    let listed: serde_json::Value = serde_json::from_slice(&listed.stdout).unwrap();
    assert_eq!(listed["resources"][0]["state"], "installed");
    assert_eq!(listed["materializations"][0]["state"], "materialized");

    fs::remove_file(
        fixture
            .workspace
            .join(".liknon/resources/knowledge/official/evidence/1.0.0/content.md"),
    )
    .unwrap();
    fs::write(fixture.workspace.join("docs/evidence.md"), b"changed\n").unwrap();
    let listed = run_manager(&list);
    assert_success(&listed);
    let listed: serde_json::Value = serde_json::from_slice(&listed.stdout).unwrap();
    assert_eq!(listed["resources"][0]["state"], "missing");
    assert_eq!(listed["materializations"][0]["state"], "drifted");
}

#[test]
fn drift_and_forged_metadata_fail_closed() {
    let fixture = Fixture::new();
    fixture.publish(
        1,
        vec![resource(
            ResourceKind::Knowledge,
            "evidence",
            "1.0.0",
            b"evidence version one\n",
        )],
    );
    let mut init = vec!["init".to_owned()];
    init.extend(fixture.workspace_arguments());
    assert_success(&run_manager(&init));
    let mut install = vec![
        "install".to_owned(),
        "knowledge:official/evidence".to_owned(),
        "--yes".to_owned(),
    ];
    install.extend(fixture.workspace_arguments());
    install.extend(fixture.hub_arguments());
    assert_success(&run_manager(&install));

    let payload = fixture
        .workspace
        .join(".liknon/resources/knowledge/official/evidence/1.0.0/content.md");
    fs::write(&payload, b"tampered\n").unwrap();
    let mut verify = vec!["verify".to_owned(), "--offline".to_owned()];
    verify.extend(fixture.workspace_arguments());
    assert_eq!(run_manager(&verify).status.code(), Some(4));

    let signature_path = fixture.hub.join("stable.json.sig.json");
    let mut signature: serde_json::Value =
        serde_json::from_slice(&fs::read(&signature_path).unwrap()).unwrap();
    let encoded = signature["signature"].as_str().unwrap();
    let replacement = if encoded.starts_with('A') { "B" } else { "A" };
    signature["signature"] = format!("{replacement}{}", &encoded[1..]).into();
    fs::write(
        &signature_path,
        serde_json::to_vec_pretty(&signature).unwrap(),
    )
    .unwrap();
    let mut search = vec!["search".to_owned()];
    search.extend(fixture.hub_arguments());
    assert_eq!(run_manager(&search).status.code(), Some(4));
}

#[test]
fn rejects_catalog_rollback_without_discarding_installed_state() {
    let fixture = Fixture::new();
    fixture.publish(
        2,
        vec![resource(
            ResourceKind::Knowledge,
            "evidence",
            "1.0.0",
            b"evidence version one\n",
        )],
    );
    let mut init = vec!["init".to_owned()];
    init.extend(fixture.workspace_arguments());
    assert_success(&run_manager(&init));
    let mut install = vec![
        "install".to_owned(),
        "knowledge:official/evidence".to_owned(),
        "--yes".to_owned(),
    ];
    install.extend(fixture.workspace_arguments());
    install.extend(fixture.hub_arguments());
    assert_success(&run_manager(&install));

    fixture.publish(
        1,
        vec![resource(
            ResourceKind::Knowledge,
            "evidence",
            "1.0.0",
            b"evidence version one\n",
        )],
    );
    let mut search = vec!["search".to_owned()];
    search.extend(fixture.hub_arguments());
    assert_eq!(run_manager(&search).status.code(), Some(4));

    let mut offline = vec!["verify".to_owned(), "--offline".to_owned()];
    offline.extend(fixture.workspace_arguments());
    assert_success(&run_manager(&offline));
}

#[test]
fn rejects_changed_metadata_for_an_existing_identity_and_version() {
    let fixture = Fixture::new();
    let original = resource(
        ResourceKind::Knowledge,
        "evidence",
        "1.0.0",
        b"evidence version one\n",
    );
    fixture.publish(1, vec![original.clone()]);
    let mut init = vec!["init".to_owned()];
    init.extend(fixture.workspace_arguments());
    assert_success(&run_manager(&init));
    let mut install = vec![
        "install".to_owned(),
        "knowledge:official/evidence".to_owned(),
        "--yes".to_owned(),
    ];
    install.extend(fixture.workspace_arguments());
    install.extend(fixture.hub_arguments());
    assert_success(&run_manager(&install));

    let mut replaced = original;
    replaced.summary = "Changed metadata under an immutable version".to_owned();
    fixture.publish(2, vec![replaced]);
    let mut update = vec!["update".to_owned(), "--yes".to_owned()];
    update.extend(fixture.workspace_arguments());
    update.extend(fixture.hub_arguments());
    assert_eq!(run_manager(&update).status.code(), Some(4));

    let mut offline = vec!["verify".to_owned(), "--offline".to_owned()];
    offline.extend(fixture.workspace_arguments());
    assert_success(&run_manager(&offline));
}

#[test]
fn malformed_hub_metadata_is_an_integrity_failure() {
    let fixture = Fixture::new();
    fixture.publish(
        1,
        vec![resource(
            ResourceKind::Knowledge,
            "evidence",
            "1.0.0",
            b"evidence version one\n",
        )],
    );
    fs::write(fixture.hub.join("stable.json.sig.json"), b"not-json\n").unwrap();

    let mut search = vec!["search".to_owned()];
    search.extend(fixture.hub_arguments());
    let output = run_manager(&search);
    assert_eq!(output.status.code(), Some(4));
    assert!(String::from_utf8_lossy(&output.stderr).contains("malformed JSON"));
}

#[test]
fn malformed_trust_and_workspace_documents_use_stable_error_classes() {
    let fixture = Fixture::new();
    fixture.publish(
        1,
        vec![resource(
            ResourceKind::Knowledge,
            "evidence",
            "1.0.0",
            b"evidence version one\n",
        )],
    );
    fs::write(&fixture.trust_root, b"not-json\n").unwrap();
    let mut search = vec!["search".to_owned()];
    search.extend(fixture.hub_arguments());
    assert_eq!(run_manager(&search).status.code(), Some(4));

    let mut init = vec!["init".to_owned()];
    init.extend(fixture.workspace_arguments());
    assert_success(&run_manager(&init));
    let manifest = fixture.workspace.join(".liknon/resources/manifest.json");
    fs::write(&manifest, b"not-json\n").unwrap();
    let mut list = vec!["list".to_owned()];
    list.extend(fixture.workspace_arguments());
    assert_eq!(run_manager(&list).status.code(), Some(3));

    fs::write(&manifest, b"{\"schemaVersion\":1,\"resources\":[]}\n").unwrap();
    fs::write(
        fixture.workspace.join(".liknon/resources/lock.json"),
        b"not-json\n",
    )
    .unwrap();
    assert_eq!(run_manager(&list).status.code(), Some(4));
}
