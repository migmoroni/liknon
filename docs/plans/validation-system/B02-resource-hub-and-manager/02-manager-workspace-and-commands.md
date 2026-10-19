# Phase B02.2: Implement Manager Workspace And Command Semantics

## Status

`Provisional`. Blocked by the approved B02.0 command and state contracts.

Operation labels in this document are placeholders for the exact grammar
selected by B02.0; they are not independently approved commands.

[Back to the Resource Hub and Manager plan](README.md)

## Purpose

Implement the Manager's stable CLI surface, workspace discovery, state
inspection, and mutation planning before enabling complete installation.

## Workspace Discovery

Manager discovers the nearest ancestor containing `.liknon/` or accepts an
explicit workspace boundary. It never infers a different root from Git,
language package files, environment-manager state, or an agent directory.

Existing-state commands fail when no root is found. `init` creates only the
requested resource foundation at the explicit workspace, the nearest existing
root, or the exact current directory when no root exists. It preflights all
destinations and never overwrites conflicting bytes.

## Read Operations

- `search` queries authenticated Hub metadata and identifies the catalog
  snapshot used.
- `info` reports administrative metadata for one available or installed
  resource.
- `list` compares desired, locked, installed, materialized, missing, and
  drifted state.
- `verify` checks schemas, metadata authentication, payload inventory, and
  exact installed bytes.

These operations report trust, compatibility, yanking, revocation, and
integrity separately. No aggregate word hides a security-relevant state.
For workflows they report graph and dependency metadata as data only; listing
or inspecting a workflow never selects an entry point, binds an agent profile,
or activates execution.

## Mutation Planning

Before `install`, `update`, `remove`, or `materialize` writes anything,
Manager produces a complete plan containing:

- selected catalog snapshot;
- requested specs and constraints;
- exact dependency graph;
- files to add, replace, retain, or remove;
- manifest and lock changes;
- downloads and verified byte counts;
- conflicts, policy-relevant effects, and required human decisions;
- rollback boundary.

Planning performs no partial workspace mutation.

## Output Contract

Human output explains the requested operation and exact resulting state.
Versioned JSON output exposes typed records suitable for agents and automation.
JSON uses stdout, diagnostics use stderr, and neither contains terminal styling
unless the human format explicitly requests it.

Exit statuses distinguish usage/configuration failure, blocked or incompatible
resolution, integrity/security failure, and completed mutation.

## Verification

- Discovery, explicit roots, nested workspaces, and missing roots are covered.
- Every operation has human and JSON contract tests.
- Read operations make no workspace mutation.
- Dry-run and commit use the same resolved plan.
- Interrupted output does not leave a partial manifest, lock, or payload.
- Command aliases remain equivalent.
- Official and external workflow IDs follow identical command paths, and no
  command assumes a process number, transition, or profile identity.

## Exit Criterion

Manager has a deterministic, inspectable command surface and can describe every
intended state transition before any resource bytes are committed.
