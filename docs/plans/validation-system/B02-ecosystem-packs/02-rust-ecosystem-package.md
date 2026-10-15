# Phase B02.2: Build The Rust Ecosystem Package

[Back to the ecosystem packs plan](README.md)

## Purpose

Publish an ecosystem-native Rust package that exposes exact shared and Rust
pack resources without installing the Liknon CLI or exposing the validation
engine.

## Package Contract

The package contains:

- the exact approved shared pack;
- the Rust pack;
- a minimal documented Rust API for package version, manifest inspection, and
  exact sealed pack bytes or explicit caller-directed materialization;
- no binary target named `liknon` and no hidden subprocess adapter.

The Rust pack owns Rust language guidance, Cargo and tool references,
Rust-specific recipes, templates, and skill extensions. It does not duplicate
shared foundations.

Do not return `CARGO_MANIFEST_DIR` or a Cargo registry-cache location as a
durable runtime contract. If the package embeds sealed pack bytes, extraction
occurs only through an explicit API into a caller-selected destination and
verifies the resulting manifest and hashes.

The final crate name, public module name, SemVer relationship, MSRV, and
features are approved only after B02.0 prototypes the consumption model. The
crate must build from its published contents without unpublished path
dependencies.

## Verification

- `cargo package` contains only the locator library and declared pack bytes.
- Isolated consumers obtain both exact pack payloads through typed APIs and can
  explicitly make them available to the CLI.
- Installing the crate creates no executable and mutates no workspace.
- Exact packaged bytes pass generic pack validation.
- One representative Rust workspace activates the package and retrieves
  Rust-specific guidance through the CLI.
- MSRV, Rustdoc, package inspection, and registry readback pass before release.

## Exit Criterion

A Rust project can depend on the package, access immutable shared and Rust pack
payloads, and explicitly supply them to Liknon without receiving a second CLI
or validation implementation.
