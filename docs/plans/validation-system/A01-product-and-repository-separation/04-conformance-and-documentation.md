# Phase A01.4: Prove Conformance And Reconcile Documentation

[Back to the product and repository separation plan](README.md)

## Purpose

Prove that the repository boundaries preserve validation behavior, establish
safe resource interoperability, and leave every maintained document and test
aligned with the current product model.

## Conformance Matrix

Verify independently:

- validation-core configuration, planning, execution, cancellation, timeout,
  repository, and report semantics;
- Validation CLI arguments, help, streams, progress, themes, signals, aliases,
  initialization, and exit statuses;
- Manager catalog access, resolution, locking, installation, update, removal,
  verification, and materialization;
- Tool offline catalog, search, lookup, resolution, context output, aliases,
  and provenance;
- exact program-owned AI bootstrap output, version, and digest without a
  workspace or resource store;
- source installation of all three applications;
- package inspection of all supporting crates and registry-only resolution by
  each application candidate;
- validation artifacts excluding authored resources;
- Manager and Tool artifacts excluding resource payloads and the validation
  engine;
- one selected knowledge resource, one skill, one collection, and one
  dependency graph through Manager-to-Tool interoperability;
- one official workflow and one external workflow with different graph shapes,
  profile dependencies, and continuation-eligible edges through the same
  Manager-to-Tool contracts;
- source and binary inspection proving that no application branches on an
  official workflow, skill, tool, process, node, action, transition, or
  agent-profile ID;
- repository inventory proving each authored resource has one owner.

Existing outcome fixtures compare structured validation meaning before and
after extraction. Resource fixtures compare catalog identity, locked metadata,
payload hashes, and exact Tool output.

## Documentation Reconciliation

Update maintained documentation and plans so they consistently state:

- `contracts/ai/ENTRYPOINT.md` is the program-owned first instruction for every
  governed AI operation and `likt bootstrap` exposes its exact bytes;
- `liknon-cli` is the Cargo application package whose commands are `liknon`
  and `likx`;
- `liknon-manager` provides `liknon-manager` and `likm`;
- `liknon-tool` provides `liknon-tool` and `likt`;
- `.liknon/` is the only consumer workspace root;
- B01 distributes validation, B02 establishes Hub and Manager, B03 distributes
  Tool, and B04 governs future programmatic APIs;
- knowledge is a selectable resource installed by Manager and consumed by Tool;
- AI-Tool is the official default skill of the initial workflows but remains an
  ordinary replaceable resource with no application privilege;
- workflow orchestration, agent profiles, skills, tool descriptors, and
  knowledge are distinct resource contracts organized as
  `resources/<kind>/<namespace>/`;
- package registries never distribute authored resource payloads or copied CLI
  launchers; crates.io application packages contain source and resolve only
  published supporting crates.

Search the repository for obsolete command names, `.validation/`, embedded
corpus claims, package-distributed resource claims, stale track links, old
include rules, outdated installation commands, AI instructions that bypass the
bootstrap entry point, and base documents that still present an official
workflow as a platform root.

Use the A01.0 replacement ledger to update or remove each superseded maintained
document in its owning phase. Do not preserve the pre-separation wording as a
fallback or describe an expected intermediate mismatch as a second supported
contract.

## Required Gates

- `cargo fmt --all -- --check`;
- `cargo clippy --workspace --all-targets --locked -- -D warnings`;
- `cargo test --workspace --all-targets --locked`;
- `cargo test --workspace --doc --locked`;
- ignored outcome fixtures;
- exact bootstrap source-versus-Tool output and digest checks;
- knowledge editorial and resource exact-byte checks;
- MSRV validation;
- isolated installation smoke tests for all three applications and six command
  names;
- isolated `cargo package` and registry-resolution checks for every publishable
  application and supporting crate;
- offline Tool tests;
- Manager-to-Tool interoperability, artifact-content, and local-link checks.

## Exit Criterion

All boundary and behavior evidence passes, maintained documentation has no
stale product, bootstrap, policy, workflow-root, path, or ownership claim, and
B01 through B03 can begin without moving responsibilities again.
