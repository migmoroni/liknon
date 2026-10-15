# Phase A01.1: Create The Cargo Workspace And Extract The Core

[Back to the product and repository separation plan](README.md)

## Purpose

Create the virtual Cargo workspace and move deterministic validation behavior
behind a terminal-independent internal core API while preserving one
implementation and the current observable CLI behavior.

## Workspace Contract

- The root `Cargo.toml` is virtual and owns shared package metadata, dependency
  policy, lint policy, release profile, and intentional version inheritance.
- The root `Cargo.lock` remains committed because the repository ships a CLI.
- `crates/liknon-core` owns domain contracts and behavior.
- `apps/liknon-cli` owns the binary target named `liknon` and is not published
  through crates.io by B01.
- Package dependency edges point from CLI to core, never from core to CLI.
- The MSRV remains the root workspace contract and is verified for every Rust
  member affected by a change.

## Core Extraction

Move configuration, contract types, DAG construction, planning, shell-free
process execution, repository inspection, structured execution outcomes, and
machine-readable report construction into `liknon-core`.

The core API uses explicit values and paths. It does not:

- parse process arguments;
- infer behavior from terminal capabilities;
- write help or progress output;
- choose ANSI palettes or presentation modes;
- terminate the host process;
- read process-wide current directory when an explicit workspace is required;
- embed or discover authored knowledge and skills.

Move Clap definitions, help, terminal themes, human report rendering, progress,
Ctrl+C adaptation, stream selection, and exit-code mapping into
`apps/liknon-cli`.

## Reporting Boundary

Structured report types and deterministic JSON serialization belong to core.
Human report layout, hierarchy lines, terminal width, color, presentation, and
live execution rendering belong to the CLI. Tests are split on the same line.

## Verification

- Existing configuration, planning, execution-state, cancellation, timeout,
  and JSON report fixtures pass through core APIs.
- Existing CLI fixtures preserve arguments, diagnostics, streams, human
  rendering, and stable exit statuses.
- Dependency inspection proves core has no Clap, console, indicatif, terminal,
  or authored-content dependency.
- `cargo test --workspace --all-targets --locked` exercises all members.
- `cargo install --path apps/liknon-cli --locked` produces one `liknon` binary.

## Exit Criterion

The virtual workspace builds with one validation implementation in
`liknon-core`, one terminal adapter in `apps/liknon-cli`, and no circular or
presentation dependency in the core.
