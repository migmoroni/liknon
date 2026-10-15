# Phase B03.2: Publish The Rust AI Package

[Back to the ecosystem AI packages plan](README.md)

## Purpose

Publish crates.io package `liknon` as an importable Rust resource library
without binary targets.

## Package Contract

- Expose shared and Rust-specific resource manifests and exact bytes.
- Provide typed inventory, retrieval, compatibility, and explicit
  materialization APIs.
- Preserve the workspace MSRV and documented feature policy.
- Contain no binary named `liknon`, `liknon-cli`, `liknon-knowledge`, or
  `liknon-k`.
- Build from published contents without unpublished path dependencies.
- Reserve coherent public namespace for future B04 Rust APIs.

Rust additions cover Cargo-aware agent instructions and Rust ecosystem
integration, not validation knowledge copied from B02.

## Verification

- `cargo package --list` matches the approved inventory.
- Isolated consumers import the resource API at the MSRV.
- No `cargo install liknon` command is advertised or produced.
- Materialization safety fixtures pass.
- Published readback reproduces the retained manifest and resource hashes.

## Exit Criterion

Rust projects can depend on `liknon` for AI resources without receiving a CLI,
knowledge corpus, or placeholder validation binding.
