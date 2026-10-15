# Phase A01.1: Create The Cargo Workspace And Extract The Core

[Back to the product and repository separation plan](README.md)

## Purpose

Create the virtual Cargo workspace and move deterministic validation behavior
behind a terminal-independent internal core API while preserving one
implementation and the current observable validation behavior.

## Workspace Contract

- The root `Cargo.toml` is virtual and owns shared package metadata,
  dependencies, lints, release profile, and version policy.
- The root `Cargo.lock` remains committed because the repository ships
  applications.
- `crates/liknon-core` owns validation contracts and behavior.
- `apps/liknon-cli` owns binary targets `liknon` and `liknon-cli`.
- `apps/liknon-knowledge` owns binary targets `liknon-knowledge` and
  `liknon-k`.
- Both names in each application invoke the same implementation.
- Dependency edges point from applications to approved internal crates, never
  from the core to an application.
- The workspace MSRV is verified for every affected Rust member.

## Core Extraction

Move configuration, contract types, DAG construction, planning, shell-free
process execution, repository inspection, structured outcomes, and
machine-readable report construction into `liknon-core`.

The core API uses explicit values and paths. It does not:

- parse process arguments;
- infer behavior from terminal capabilities;
- write help or progress output;
- choose ANSI palettes or presentation modes;
- terminate the host process;
- read a process-wide current directory when an explicit root is required;
- embed, discover, catalog, or render authored knowledge or agent resources.

Move Clap definitions, validation help, themes, human report rendering,
progress, Ctrl+C adaptation, stream selection, and exit-code mapping into
`apps/liknon-cli`.

Keep knowledge command parsing, catalog lookup, document lookup, and knowledge
rendering in `apps/liknon-knowledge`. Do not make either application a hidden
subprocess dependency of the other.

## Reporting Boundary

Structured report types and deterministic JSON serialization belong to core.
Human report layout, hierarchy lines, terminal width, color, presentation, and
live execution rendering belong to the validation CLI. Tests follow the same
boundary.

## Verification

- Existing configuration, planning, execution, cancellation, timeout, and JSON
  report fixtures pass through core APIs.
- Existing validation-CLI fixtures preserve arguments, diagnostics, streams,
  rendering, and stable exit statuses.
- Knowledge-CLI tests require no validation configuration or process runner.
- Dependency inspection proves core has no Clap, terminal, or authored-content
  dependency.
- `cargo test --workspace --all-targets --locked` exercises all members.
- Isolated installation proves both command names of each application.

## Exit Criterion

The virtual workspace builds with one validation implementation, one validation
terminal adapter, one independent knowledge application, and no circular,
presentation, knowledge, or agent-resource dependency in the core.
