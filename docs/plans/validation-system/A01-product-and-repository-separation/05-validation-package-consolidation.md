# Phase A01.5: Consolidate The Validation Package

## Status

`Implemented; hosted verification pending`. All locally executable gates passed
on 2026-10-26. B01.0 may begin after the consolidated commit also passes the
hosted macOS and Windows jobs.

[Back to Plan A01](README.md)

## Decision

Liknon validation is one product and one Cargo package: `liknon-validator` at
`apps/liknon-validator`.

The package owns both:

- the terminal-independent validation implementation and structured contracts;
- the CLI adaptation, human presentation, and binary targets
  `liknon-validator` and `likv`.

The implementation remains internally modular, but package boundaries are not
used merely to separate engine modules from their only product. The package
`liknon-validation` and directory `crates/liknon-validation` are removed.

## Reason

The current split is mechanically valid but does not represent two independent
products, release units, or supported APIs. It adds a package identity,
publication order, path dependency, schema location, and documentation surface
without an independent consumer that justifies those costs.

One package provides the same separation through Rust modules while making the
source, installation, release, and ownership model match the product boundary.

## Starting State

At the start of this unit:

- `crates/liknon-validation` owns configuration, contracts, planning,
  execution, repository evidence, initialization, and schemas;
- `apps/liknon-validator` owns CLI parsing, reporting, themes, and the two
  validation binaries;
- `apps/liknon-validator` depends on `liknon-validation`;
- `tools/knowledge` depends on `liknon-validation` only to validate the
  configuration examples embedded in authored recipes with the real loader;
- `apps/liknon-tool` remains an identity-only shell whose resource-consumption
  behavior belongs to B03.

This unit changes the validation package boundary. It does not implement Tool
behavior or move runtime knowledge access into the Validation CLI.

## Target Layout

```text
apps/liknon-validator/
|-- Cargo.toml
|-- assets/
|   `-- branding/
|-- examples/
|-- schemas/
|   |-- config.schema.json
|   `-- report.schema.json
|-- src/
|   |-- bin/
|   |   |-- liknon-validator.rs
|   |   `-- likv.rs
|   |-- cli/
|   |-- config/
|   |-- contracts/
|   |-- execution/
|   |-- planning/
|   |-- reporting/
|   |-- repository/
|   |-- theme/
|   |-- dag.rs
|   |-- error.rs
|   |-- initialization.rs
|   |-- lib.rs
|   `-- process.rs
`-- tests/

tools/knowledge/                 # private repository maintenance only
```

The exact module nesting may follow the existing source with minimal movement.
It must retain clear internal boundaries between validation semantics and
terminal presentation without creating another package.

## Ownership After Consolidation

| Surface | Owner |
| --- | --- |
| Configuration discovery, parsing, parameters, and semantic validation | `liknon-validator` library |
| Structured configuration, initialization, and report contracts | `liknon-validator` library |
| DAG construction, planning, execution, cancellation, and repository evidence | `liknon-validator` library |
| Config and report schemas | `apps/liknon-validator/schemas` |
| Clap grammar, streams, signals, exit mapping, reporting, and themes | `liknon-validator` CLI modules |
| Binaries `liknon-validator` and `likv` | package `liknon-validator` |
| Authored knowledge source and editorial contracts | `docs/validation` |
| Authored-knowledge conformance checks | private package `liknon-knowledge-maintenance` under `tools/knowledge` |
| Future installed-resource retrieval | `liknon-tool`, defined and implemented by B03 |

## Dependency Graph

The target dependency direction is:

```text
liknon-manager
liknon-tool

liknon-validator <--- liknon-cli
       ^
       |
liknon-knowledge-maintenance
```

`liknon-knowledge-maintenance` may use the `liknon-validator` library solely to
prove that authored validation recipes satisfy the real configuration
semantics. This is a repository-conformance dependency, not public knowledge
retrieval and not a runtime dependency of a distributed Liknon application.

No product depends on `tools/knowledge`. `liknon-tool` remains unchanged in this
unit and receives its read-only resource behavior only in B03.

## Implementation Units

### 1. Consolidate Source Ownership

1. Move the engine source modules from `crates/liknon-validation/src` into
   `apps/liknon-validator/src`.
2. Replace the dependency re-export in `apps/liknon-validator/src/lib.rs` with
   direct module declarations and the narrow exports required by current
   workspace consumers.
3. Update internal imports from `liknon_validation` to crate-local paths.
4. Preserve shell-free execution, cancellation, timeout, process-tree,
   repository-integrity, and structured-report behavior exactly.
5. Do not create an adapter, compatibility crate, re-export package, deprecated
   alias, or forwarding facade for `liknon-validation`.

### 2. Consolidate Schemas, Examples, And Tests

1. Move config and report schemas to `apps/liknon-validator/schemas`.
2. Move the inspection example into `apps/liknon-validator/examples` and update
   its crate paths.
3. Merge engine integration tests into `apps/liknon-validator/tests` without
   weakening assertions or duplicating shared test helpers.
4. Preserve the existing schema versions and generated-schema byte checks.
5. Keep outcome fixtures isolated from the root workspace and preserve their
   explicit ignored execution.

### 3. Consolidate Cargo Metadata

1. Move the engine's runtime, target-specific, and development dependencies into
   `apps/liknon-validator/Cargo.toml`.
2. Remove `liknon-validation` from workspace members and workspace dependencies.
3. Remove `crates/liknon-validation`; remove `crates/` as well if it becomes
   empty.
4. Regenerate the committed root `Cargo.lock` only through Cargo.
5. Keep package name `liknon-validator`, library crate name `liknon_validator`,
   version `0.5.0`, MSRV 1.87, and both binary names unchanged.
6. Do not activate the future B05 programmatic API or claim stability for
   internal Rust modules.

### 4. Preserve Repository Knowledge Maintenance

1. Keep `tools/knowledge` private, unpublished, and repository-only.
2. Point its configuration-recipe validation at the `liknon-validator` library
   and the schemas owned by `apps/liknon-validator`.
3. Preserve catalog, source, integrity, recipe, generated-source, and skill
   handoff checks.
4. Do not add knowledge commands to `liknon-validator`.
5. Do not implement `liknon-tool` resource retrieval in this unit.

### 5. Reconcile CI, Documentation, And Active Plans

1. Make CI inspect and install the single `liknon-validator` package rather than
   packaging a separate engine.
2. Update README, configuration design, schema reference, security guidance,
   changelog, and maintained examples to use the consolidated owner and paths.
3. Rewrite the A01 ledger and phase descriptions so `liknon-validator` is the
   sole validation target owner.
4. Remove B01 assumptions about publishing a supporting validation crate before
   the application package.
5. Update the validation-system roadmap and target-state summary to avoid
   presenting speculative internal crates as required architecture.
6. Search active code, manifests, CI, and maintained documentation for stale
   `liknon-validation` names and old schema paths.

## Non-Goals

This unit does not:

- change CLI syntax, help, output, color, presentation, or exit behavior;
- change configuration, report, or initialization schema versions;
- change `.liknon/validation/config.json` discovery or initialization;
- implement Manager, Tool, Hub, policy, persistence, resources, or workflows;
- restore embedded knowledge or a `knowledge` command in the Validation CLI;
- define public language bindings or the future package `liknon`;
- publish any package or native artifact.

## Verification

Run and record:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --all-targets --locked
cargo test --workspace --doc --locked
cargo test -p liknon-validator --test outcome_fixtures --locked -- --ignored
cargo run -p liknon-knowledge-maintenance --locked -- --check
cargo +1.87.0 check --workspace --all-targets --locked
cargo +1.87.0 test --workspace --all-targets --locked
cargo +1.87.0 test --workspace --doc --locked
```

Also verify:

- the supported Windows cross-target check still passes;
- package inventory for `liknon-validator` contains its engine, CLI, schemas, and
  branding but no knowledge corpus, AI resources, plans, fixtures, evaluations,
  or repository tooling;
- temporary local installation exposes equivalent `liknon-validator` and `likv`
  binaries;
- `cargo metadata` contains no package or dependency named
  `liknon-validation`;
- active source, manifests, CI, maintained docs, and active target plans contain
  no stale ownership claim for `crates/liknon-validation`;
- `git diff --check` passes.

Actual Windows and macOS runtime execution remains GitHub Actions evidence when
those hosts are unavailable locally.

## Acceptance Criteria

- [x] `liknon-validator` is the only package that implements validation.
- [x] Its library and both binaries compile from one package.
- [x] `crates/liknon-validation` and its workspace identity no longer exist.
- [x] Engine and terminal responsibilities remain visibly separated by modules.
- [x] Current validation behavior and versioned contracts are unchanged.
- [x] Config and report schemas have one canonical owner inside
      `apps/liknon-validator`.
- [x] All engine, CLI, schema, fixture, and repository tests remain active.
- [x] Knowledge maintenance still validates authored recipe configurations with
      the real loader.
- [x] `liknon-tool` remains untouched and its future behavior remains owned by
      B03.
- [x] B01 describes one self-contained source application package and has no
      validation supporting-crate publication step.
- [x] All required repository, MSRV, package, installation, and locally
      executable CI gates pass.
- [ ] The consolidated commit passes the hosted macOS and Windows runtime jobs.

## Result

- The workspace now contains five packages and no package or dependency named
  `liknon-validation`.
- `liknon-validator` owns the engine modules, CLI adaptation, schemas, example,
  integration tests, and both binaries.
- Cargo packaged and verified the dirty working-tree candidate as 62 files;
  its inventory contains no authored knowledge, AI resource, plan, fixture, or
  repository tool.
- A temporary isolated installation exposed equivalent `liknon-validator` and
  `likv` version `0.5.0` binaries and both generated schemas.
- Stable and Rust 1.87 formatting, Clippy, tests, doctests, strict Rustdoc,
  Windows GNU cross-check, knowledge maintenance, outcome fixtures, dependency
  audit, source/license policy, and diff checks passed.
- Native macOS and Windows runtime execution remains evidence for the next
  hosted CI run, as required when those hosts are unavailable locally.

## Exit Criterion

The repository has one validation product, one validation package, one
implementation, and one schema owner. B01 may begin only after the consolidated
package is independently packageable, installable, fully tested, and accurately
documented.
