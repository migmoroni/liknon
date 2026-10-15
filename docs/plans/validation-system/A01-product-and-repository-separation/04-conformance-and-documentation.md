# Phase A01.4: Prove Conformance And Reconcile Documentation

[Back to the product and repository separation plan](README.md)

## Purpose

Prove that the new repository boundaries preserve validation behavior, expose
the complete knowledge corpus independently, and leave every maintained
document and test aligned with the current product model.

## Conformance Matrix

Verify independently:

- core configuration, planning, execution, cancellation, timeout, and report
  semantics;
- validation CLI arguments, help, streams, progress, themes, signals, aliases,
  and exit statuses;
- knowledge CLI catalog, lookup, exact bytes, formats, aliases, and provenance;
- source installation of each application;
- validation artifacts excluding knowledge and AI resources;
- knowledge artifacts excluding validation execution and AI resources;
- Rust, JavaScript/TypeScript, Ruby, Python, and .NET AI-resource inventories;
- repository inventory proving each authored resource has one owner.

Existing outcome fixtures compare the pre-extraction structured meaning with
the new core and validation-CLI paths. Knowledge fixtures compare catalog IDs
and returned bytes with canonical files.

## Documentation Reconciliation

Update maintained documentation and plans so they consistently state:

- `liknon-cli` is the validation application with commands `liknon` and
  `liknon-cli`;
- `liknon-knowledge` is the complete, independent knowledge application with
  commands `liknon-knowledge` and `liknon-k`;
- `liknon knowledge` does not exist;
- B01 distributes the validation CLI;
- B02 distributes the knowledge CLI and corpus;
- B03 distributes non-CLI `liknon` ecosystem packages with AI resources;
- B04 governs future in-process APIs and bindings in those packages;
- knowledge retrieval has no hidden package discovery or workspace activation
  state;
- package registries distribute AI resources, not either CLI.

Search the entire repository for obsolete root-package paths, old include
rules, embedded-knowledge claims, old track links, and stale install commands.

## Required Gates

- `cargo fmt --all -- --check`;
- `cargo clippy --workspace --all-targets --locked -- -D warnings`;
- `cargo test --workspace --all-targets --locked`;
- `cargo test --workspace --doc --locked`;
- ignored outcome fixtures;
- knowledge editorial and exact-byte checks;
- MSRV validation;
- isolated installation smoke tests for both applications and all four command
  names;
- artifact-content and local-link checks.

## Exit Criterion

All product-boundary and behavior evidence passes, maintained documentation has
no contradictory ownership claim, and B01 through B03 can begin without moving
responsibilities again.
