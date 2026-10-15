# Phase B04.1: Publish The Rust Library Crate

[Back to the programmatic API and bindings plan](README.md)

## Purpose

Extend the existing crates.io `liknon` resource library with the public Rust
API approved by B04.0. The package remains a library and is not a CLI
installation channel.

## Dependencies

- [Phase B04.0](00-programmatic-api-contract.md) is complete.
- The existing B03 package ownership, version relationship, MSRV, feature
  policy, and public resource namespace are approved for extension.
- A01 has separated the core from CLI argument parsing, terminal rendering,
  process exit, and authored ecosystem content.

## Contract To Define

- public modules, types, traits, and construction patterns;
- explicit workspace and side-effect capabilities;
- async/runtime neutrality where practical;
- typed errors, reports, progress, cancellation, and timeouts;
- feature flags and dependency surface;
- schema and embedded-resource ownership;
- SemVer and MSRV policies;
- Rustdoc examples and security guidance.

The crate package must build from its own published contents, preserve the B03
resource API, and avoid unpublished path crates or repository-only files.

## Verification

- `cargo package` inspection and source-only build in isolation;
- tests against the approved programmatic conformance fixtures;
- Rustdoc, examples, feature combinations, and MSRV checks;
- absence of CLI-only assets and implicit process-global behavior;
- continued B03 resource inventory, retrieval, and materialization behavior;
- publication readback and checksum verification when this phase reaches
  release.

## Exit Criterion

Rust applications can depend on the published crate and use supported Liknon
capabilities in process without invoking the CLI or parsing terminal output.
