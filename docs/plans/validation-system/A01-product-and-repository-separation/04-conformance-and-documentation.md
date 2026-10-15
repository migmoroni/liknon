# Phase A01.4: Prove Conformance And Reconcile Documentation

[Back to the product and repository separation plan](README.md)

## Purpose

Prove that the new repository boundaries preserve validation behavior, enforce
content separation, and leave every maintained document and test aligned with
the current product model.

## Conformance Matrix

Verify independently:

- core configuration, planning, execution, cancellation, timeout, and report
  semantics;
- CLI arguments, help, streams, progress, themes, signals, and exit statuses;
- generic pack validation, compatibility, composition, and materialization;
- shared, Rust, JavaScript/TypeScript, and polyglot pack fixtures;
- source installation from `apps/liknon-cli`;
- package inventory proving the CLI excludes authored pack content;
- repository inventory proving each authored resource has one owner.

Existing outcome fixtures compare the pre-extraction structured meaning with
the new core and CLI paths. Text snapshots may change only where the approved
pack-selection contract requires a new explicit diagnostic.

## Documentation Reconciliation

Update maintained documentation and plans so they consistently state:

- the CLI is one application, not the public language library;
- GitHub source installation targets `apps/liknon-cli`;
- the CLI contains no embedded validation knowledge or skill bundle;
- ecosystem content is obtained through B02 packages and consumed through the
  generic pack contract;
- `docs/validation/knowledge` and root `skills/liknon` are not canonical target
  paths after extraction;
- B01 distributes only the CLI, B02 distributes packs, and B03 governs true
  programmatic APIs and language bindings.

Search the entire repository for obsolete root-package paths, package include
rules, embedded-knowledge claims, old install commands, and cross-track links.

## Required Gates

- `cargo fmt --all -- --check`;
- `cargo clippy --workspace --all-targets --locked -- -D warnings`;
- `cargo test --workspace --all-targets --locked`;
- `cargo test --workspace --doc --locked`;
- ignored outcome fixtures;
- MSRV validation;
- isolated `cargo install --path apps/liknon-cli --locked` smoke test;
- package-content and local-link checks.

## Exit Criterion

All product-boundary and behavior evidence passes, maintained documentation has
no contradictory ownership claim, and B01 and B02 can begin without moving
runtime or content responsibilities again.
