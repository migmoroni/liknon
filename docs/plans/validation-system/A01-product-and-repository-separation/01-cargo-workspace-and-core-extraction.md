# Phase A01.1: Create The Cargo Workspace And Extract Shared Boundaries

## Status

`Complete`. The virtual Cargo workspace, `liknon-validation` engine, Validation
CLI package, owner-local tests, dependency graph, and root lockfile implement
the approved A01.0 ledger.

[Back to Plan A01](README.md)

## Purpose

Create the approved Cargo workspace and separate terminal-independent
validation behavior from the Validation CLI while preserving all current
behavior during this unit.

## Inputs

- the approved A01.0 ledger;
- current configuration, report, and initialization schema versions;
- current validation and CLI tests;
- the current MSRV and dependency policy.

Candidate names shown in earlier design material are replaced by the exact
names approved in the ledger. This phase does not independently choose them.

## Implementation Boundary

Perform the workspace conversion in a buildable sequence:

1. create the virtual workspace and approved initial members;
2. move validation semantics and structured contracts to the approved internal
   validation crate;
3. move CLI arguments, terminal presentation, streams, signals, and exit-code
   adaptation to the Validation CLI package;
4. move shared workspace-discovery or path-containment behavior only when the
   ledger proves more than one product needs it;
5. move tests with the behavior they protect;
6. update CI, dependency policy, package metadata, and include paths;
7. preserve current runtime behavior that A01.2 intentionally separates next;
8. finish with the complete workspace green.

Do not leave a virtual root manifest pointing at sources, build scripts, tests,
or generated files that still assume the former package root.

## Validation Boundary

The terminal-independent validation owner receives only responsibilities
approved by the ledger, expected to include:

- explicit-path configuration parsing and semantic validation;
- parameter resolution, DAG construction, and planning;
- shell-free process execution, timeout, cancellation, and structured events;
- repository evidence and mutation detection;
- structured outcomes and versioned machine reports;
- generation of validation-owned schemas.

The Validation CLI retains:

- Clap command construction and help;
- stdin, stdout, stderr, signals, and process exit mapping;
- human progress and report rendering;
- color and presentation themes;
- selection of human or JSON output;
- user-facing initialization interaction.

No shared crate may depend on an application crate. A dependency declared at
workspace level is used by a member only when its code requires it.

## Transitional Knowledge Handling

This unit must remain behavior-preserving. If the current build script,
embedded catalog, knowledge commands, or skill packaging cannot be separated
atomically without mixing concerns, move them temporarily with the Validation
CLI package and mark them as A01.2 inputs. Do not redesign them here and do not
claim their temporary package location as target ownership.

## Exclusions

This phase does not:

- create Manager or Tool feature commands;
- create the Hub or public resource schemas;
- create bootstrap, policy, persistence, workflow, or agent contracts;
- author official resources;
- publish a package;
- implement B04 launcher routing or aggregate distribution;
- declare an internal crate to be the future B05 public API.

## Verification

- `cargo metadata` matches the approved member and dependency graph.
- The root lockfile remains committed.
- Validation semantics pass through the internal owner without CLI argument
  construction or terminal rendering.
- CLI tests retain current help, output, accessibility, stream, signal, and
  exit behavior.
- Current knowledge behavior remains intact until A01.2, if it was explicitly
  retained as a transitional package input.
- Nested fixture workspaces are not accidentally absorbed as product members.
- Package inventories contain only files recorded by the ledger.
- Formatting, Clippy, all-target tests, doctests, MSRV, and relevant fixtures
  pass for the complete workspace.

## Exit Criterion

The repository is a buildable Cargo workspace with one validation
implementation and explicit CLI adaptation. Every transitional current feature
has a named owner and next unit; no later product contract has been invented.
