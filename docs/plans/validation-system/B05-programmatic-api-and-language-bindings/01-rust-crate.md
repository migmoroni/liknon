# Phase B05.1: Publish The Rust Library Crate

## Status

`Future`. Blocked by an approved B05.0 Rust scope.

[Back to the programmatic API and bindings plan](README.md)

## Purpose

Publish the crates.io `liknon` library only when B05.0 approves a public Rust
API and a representative consumer. The package is a library, not a CLI or
resource-distribution channel.

## Dependencies

- [Phase B05.0](00-programmatic-api-contract.md) is complete.
- Package ownership, version relationship, MSRV, feature policy, and public
  namespace are approved.
- A01 has separated the core from CLI argument parsing, terminal rendering,
  process exit, and authored ecosystem content.

## Contract To Define

- public modules, types, traits, and construction patterns;
- explicit workspace and side-effect capabilities;
- async/runtime neutrality where practical;
- typed errors, reports, progress, cancellation, and timeouts;
- feature flags and dependency surface;
- schema and installed-resource metadata ownership;
- approved Git-hook provider planning and explicit-apply APIs, when selected by
  B05.0, conforming to the shared
  [integration contract](git-hook-integration.md);
- SemVer and MSRV policies;
- Rustdoc examples and security guidance.

The crate package must build from its own published contents and avoid
unpublished path crates, repository-only files, authored resource payloads, or
CLI assets.

## Verification

- `cargo package` inspection and source-only build in isolation;
- tests against the approved programmatic conformance fixtures;
- Rustdoc, examples, feature combinations, and MSRV checks;
- absence of CLI-only assets and implicit process-global behavior;
- absence of Hub acquisition, resource mutation, and embedded authored content;
- hook-integration fixtures proving preservation, idempotency, explicit
  mutation, and no CLI installation or hidden validation execution;
- publication readback and checksum verification when this phase reaches
  release.

## Exit Criterion

Rust applications can depend on the published crate and use supported Liknon
capabilities in process without invoking the CLI or parsing terminal output.
