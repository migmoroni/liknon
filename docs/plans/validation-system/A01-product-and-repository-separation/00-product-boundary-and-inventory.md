# Phase A01.0: Approve Product Boundaries And Inventory

## Status

`Decision gate`. This phase changes no product code or runtime path. Its only
exit artifact is a reviewed current-to-target ledger.

[Back to Plan A01](README.md)

## Purpose

Inspect the repository as it exists and classify every maintained input before
moving, splitting, or deleting anything.

## Ledger Schema

Every row must record:

| Field | Requirement |
| --- | --- |
| Current path or surface | Exact implemented file, directory, command, export, or package input |
| Current responsibility | Behavior proved by code, tests, schemas, or packaging |
| Current consumers | Runtime, build, test, documentation, or maintenance consumers |
| Protecting evidence | Tests, schemas, CI jobs, or documented contracts |
| Target responsibility | Approved role from the target summary |
| Candidate owner | Product, shared crate, resource source, or repository-only area |
| Operation | Preserve, move, split, change, or remove |
| Behavior impact | Structural only or intentional contract change |
| Owning unit | A01.1-A01.4 or a later B/C/D plan |
| Blocking decision | Any unresolved choice that prevents execution |

No row may have two editable owners. A generated or published artifact must
identify the authored source from which it is assembled.

## Required Current Inventory

The ledger must expand these observed current surfaces to individual files:

| Current surface | Current fact to preserve in the ledger |
| --- | --- |
| Root `Cargo.toml`, `Cargo.lock`, `src/lib.rs`, and `src/main.rs` | One Cargo package currently provides both library and `liknon` binary |
| `src/config`, `src/contracts`, `src/planning`, `src/execution`, `src/repository`, `src/reporting`, `src/dag.rs`, and `src/process.rs` | Validation contracts and runtime behavior |
| `src/cli` and `src/theme` | Arguments, help, streams, exit behavior, themes, and presentation |
| `src/initialization.rs` | Current config initialization behavior and path safety |
| `build.rs`, `src/knowledge`, and current knowledge CLI routes | Existing embedded knowledge implementation |
| `docs/validation/knowledge` and `docs/validation/authoring` | Current authored knowledge and editorial guidance |
| `schemas/` | Current validation, report, knowledge, and evaluation contracts |
| `tools/knowledge` | Current repository maintenance and editorial validation code |
| `skills/liknon` and its tests | Current static skill source distributed with the source package |
| `assets/branding` | Runtime and public branding inputs |
| `tests`, `fixtures`, and `examples` | Current behavioral, outcome, editorial, process, and API evidence |
| `.validation/` references | Current consumer workspace path |
| `.github/workflows`, `deny.toml`, `.gitignore`, and package include rules | Current build, policy, and packaging assumptions |
| README, `CONFIG_DESIGN.md`, validation references, changelog, and security docs | Current maintained behavior claims |

The inventory must also locate every:

- public Rust re-export;
- `include_bytes!`, `include_str!`, `OUT_DIR`, and build-script input;
- schema generator and checked-in generated contract;
- binary name and command route that actually exists;
- package-owned asset and documentation file;
- path-discovery and symlink-safety implementation;
- test that crosses a proposed product boundary;
- repository-only tool accidentally included in a publishable package.

## Classification Rules

1. Start from repository evidence, not the proposed folder tree.
2. A target name does not prove a current item exists.
3. Do not create ledger rows that remove `likx`, Manager, Tool, Hub resources,
   policy, persistence, or another proposal unless repository inspection finds
   an implemented current surface with that identity.
4. Current embedded knowledge and `liknon knowledge` may receive an intentional
   removal or extraction decision because they exist.
5. The current skill may be preserved, transformed, or retired only after its
   useful source content and consumers are identified. C02 owns any future
   official operational skill.
6. Current public Rust exports are not automatically a promised B05 API, but
   each export still requires an explicit preserve, internalize, or remove
   decision.
7. A proposed destination remains a candidate until the ledger identifies its
   responsibility and consumers.
8. Any policy, persistence, Hub, workflow, or resource-schema decision outside
   structural ownership is assigned to its later plan rather than solved here.

## Decisions Required

A01.0 must decide:

- the exact Cargo workspace root form and its initial members;
- the minimum shared-crate boundaries and their names;
- which crates must eventually be publishable to support CLI source packages;
- how package `liknon-cli` owns the approved eight binary targets while the
  standalone packages reuse the same capability implementations;
- where launcher-only dispatch code lives without importing a capability API;
- where each current validation schema and application asset belongs;
- how current knowledge embedding is removed without losing authored sources;
- how the current static skill is handed to later resource work;
- which current Rust exports remain internal implementation contracts;
- the exact atomic boundary for `.validation/` to `.liknon/validation/`;
- which tests move with semantics and which remain repository-level;
- which documentation changes in each A01 unit.

It does not decide Manager or Tool command grammar, Hub protocol, policy
operations, workflow persistence, official workflows, B04 launcher resolution
and release contracts, or B05 language bindings.

## Required Evidence

- current `cargo metadata` package and target inventory;
- package file inventory and build-script input inventory;
- `rg` results for commands, workspace paths, embedded content, public exports,
  and generated schemas;
- test ownership map;
- dependency-direction proposal justified by actual consumers;
- proposed move order with a green handoff after every unit.

## Verification

- Every current file included by the package or CI has a ledger row.
- Every current command and public export has a decision.
- Every schema, fixture, test, tool, and asset has one owner.
- Every removal row points to an implemented current surface.
- Every later-plan item is assigned without being prematurely specified.
- No target directory is created solely to make the proposed tree look
  complete.

## Exit Criterion

The user has reviewed and approved a complete ledger and dependency proposal.
A01.1-A01.4 can then replace their candidate names and destinations with the
approved values and execute without rediscovering ownership during file moves.
