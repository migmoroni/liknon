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
- source installation of all three applications;
- package inspection of all supporting crates and registry-only resolution by
  each application candidate;
- validation artifacts excluding authored resources;
- Manager and Tool artifacts excluding resource payloads and the validation
  engine;
- one selected knowledge resource, one skill, one collection, and one
  dependency graph through Manager-to-Tool interoperability;
- repository inventory proving each authored resource has one owner.

Existing outcome fixtures compare structured validation meaning before and
after extraction. Resource fixtures compare catalog identity, locked metadata,
payload hashes, and exact Tool output.

## Documentation Reconciliation

Update maintained documentation and plans so they consistently state:

- `liknon-cli` is the Cargo application package whose commands are `liknon`
  and `likx`;
- `liknon-manager` provides `liknon-manager` and `likm`;
- `liknon-tool` provides `liknon-tool` and `likt`;
- `.liknon/` is the only consumer workspace root;
- B01 distributes validation, B02 establishes Hub and Manager, B03 distributes
  Tool, and B04 governs future programmatic APIs;
- knowledge is a selectable resource installed by Manager and consumed by Tool;
- package registries never distribute authored resource payloads or copied CLI
  launchers; crates.io application packages contain source and resolve only
  published supporting crates.

Search the repository for obsolete command names, `.validation/`, embedded
corpus claims, package-distributed resource claims, stale track links, old
include rules, and outdated installation commands.

## Required Gates

- `cargo fmt --all -- --check`;
- `cargo clippy --workspace --all-targets --locked -- -D warnings`;
- `cargo test --workspace --all-targets --locked`;
- `cargo test --workspace --doc --locked`;
- ignored outcome fixtures;
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
contradictory ownership claim, and B01 through B03 can begin without moving
responsibilities again.
