# Phase A01.0: Approve Product Boundaries And Inventory

[Back to the product and repository separation plan](README.md)

## Purpose

Classify every current source, schema, document, test, fixture, tool, resource,
and asset before moving files. The result is a complete replacement ledger with
one owner and one intended distribution surface per item.

## Required Inventory Records

- Record the already approved dependency direction among `liknon-core`, `liknon-workspace`,
  `liknon-cli`, `liknon-manager`, and `liknon-tool`.
- Record `liknon-core` and `liknon-workspace` as publishable Cargo support
  dependencies without representing them as the future B04 public API.
- Classify each schema as a validation contract, workspace/resource contract,
  application contract, or repository-only contract.
- Classify every knowledge, skill, workflow, agent profile, tool descriptor, prompt, guide,
  template, and collection as one authored resource.
- Inventory every current AI-facing root instruction, policy guide, workflow
  entry document, and host assumption. Classify the new
  `crates/liknon-workspace/contracts/ai/ENTRYPOINT.md` and its adjacent metadata
  manifest as a program contract rather than a Hub resource.
- Record the one bootstrap metadata source containing schema version, SemVer
  contract version, media type, canonical entry path, and SHA-256 over the exact
  UTF-8 entry-point bytes without newline normalization.
- Identify which files are compiled application assets, published Hub
  resources, public documentation, or maintainer-only material.
- Assign every current `.validation/` path to its canonical destination below
  `.liknon/validation/` or `.liknon/ai/`.
- Identify every embedded-corpus input and every resource copy that must be
  replaced by Hub publication and workspace installation.
- Identify every maintained base document whose current official-workflow,
  policy, persistence, or resource assumption must be replaced by the generic
  bootstrap and resource contracts.
- Distinguish existing authored AI material that A01 relocates from official
  workflows, profiles, guides, or skills that Phases 3-6 create later. Do not
  create placeholder product resources to satisfy the target tree.
- Separate structured report construction from human rendering and live
  progress.
- Record every path whose command, installation path, package ownership,
  release identity, or test expectation changes.

## Replacement Ledger

For every current path, record:

- current responsibility;
- target owner and target path;
- runtime, resource, and publication consumers;
- contract and tests that protect it;
- whether it moves, splits, or is removed;
- whether moving it changes public behavior or repository structure only.

No path may have multiple editable owners. An authored file may enter an
immutable resource artifact only through deterministic assembly.

The ledger explicitly covers removal of:

- the `liknon-cli` command alias;
- `liknon-knowledge`, `liknon-k`, and any `liknon knowledge` route;
- embedded complete-corpus behavior;
- ecosystem-package resource distribution;
- the standalone `.validation/` root.

The ledger also identifies which implementation phase updates or removes each
superseded base document. A temporary difference between current documentation
and the approved target is not a compatibility requirement and must not produce
a fallback reader or duplicate contract.

## Required Baseline Ledger

The implementation expands this table to individual files before moving them.
The destination classes and replacement decisions below are already fixed:

| Current source | A01 destination or decision | Owning unit |
| --- | --- | --- |
| Root package `Cargo.toml` and `Cargo.lock` | Virtual workspace manifest plus one committed workspace lockfile | A01.1 |
| Root `README.md`, `CHANGELOG.md`, `SECURITY.md`, `LICENSE`, `deny.toml`, and agent/maintainer instructions | Remain repository-level governance and product documentation; package-specific readmes are added only where a Cargo package requires one | A01.1-A01.4 |
| `.github/workflows`, repository automation instructions, and `.gitignore` | Update for virtual-workspace gates, member/package paths, generated-output exclusions, and removal of root-package assumptions | A01.1-A01.4 |
| `src/config`, `src/contracts`, `src/dag.rs`, `src/planning`, shell-free `src/execution`, `src/process.rs`, `src/repository`, structured report types, and terminal-independent progress events | `crates/liknon-core` | A01.1 |
| `src/main.rs`, `src/cli` except knowledge routes, `src/theme`, signal/process adaptation, live progress, human/JSON output adapters, and initialization UI | `apps/liknon-cli` | A01.1 |
| Current path discovery mixed into validation loading | Split: `.liknon/` boundary and containment in `liknon-workspace`; exact config parsing and validation in `liknon-core`; command selection in `liknon-cli` | A01.1 and A01.3 |
| `src/initialization.rs` and init contracts | Split: candidate/config semantics in `liknon-core`; path safety, no-overwrite preflight, and atomic placement in `liknon-workspace`; arguments, presentation, and exit behavior in `liknon-cli` | A01.1 and A01.3 |
| `build.rs`, `src/knowledge`, `src/cli/knowledge.rs`, `src/cli/help/knowledge.rs`, and the `liknon knowledge` grammar | Remove from all runtime packages | A01.1 |
| `schemas/config.schema.json` and `schemas/report.schema.json` | `crates/liknon-core/schemas/`, generated or checked against the owning Rust types | A01.1 |
| `schemas/knowledge-catalog.schema.json`, `knowledge-sources.schema.json`, and `knowledge-editorial-profiles.schema.json` | `docs/validation/authoring/schemas/`, owned by editorial maintenance | A01.2 |
| `schemas/knowledge-routing.schema.json`, `knowledge-forward-trials.schema.json`, and `knowledge-rubric.schema.json` | `evals/schemas/`, owned by evaluation fixtures and harnesses | A01.2 |
| `tools/knowledge/*.rs` and knowledge contract tests | Repository-only `tools/knowledge` Cargo package named `liknon-knowledge-maintenance`, with `publish = false` | A01.1-A01.2 |
| `docs/validation/knowledge` and `docs/validation/authoring` | Remain canonical authored and maintainer sources; never enter an application package | A01.2 |
| `skills/liknon` and `tests/skills.rs` | Remove atomically with the knowledge command they require; Phase 3 authors the final resource under the approved public contract | A01.1 |
| `assets/branding/liknon-ascii.txt` | `apps/liknon-cli/assets/branding/liknon-ascii.txt` | A01.1 |
| `assets/branding/liknon.svg` | Remain repository/public branding and outside runtime package logic | A01.1 |
| `examples/inspect.rs` and old root-library examples | Remove; B04 owns any future supported programmatic API and examples | A01.1 |
| Validation unit and integration tests | Split beside `liknon-core` and `liknon-cli` according to behavior; no test reaches through one application to another | A01.1 |
| `fixtures/outcomes` | Remain repository fixtures; their config path changes exactly once in A01.3 | A01.3 |
| `fixtures/knowledge` and knowledge evaluations | Remain repository-only editorial fixtures and evaluations | A01.2 |
| `.validation/` references in code, docs, tests, and fixtures | Replace atomically with `.liknon/validation/`; remove the old root entirely | A01.3 |
| Root README, `CONFIG_DESIGN.md`, validation reference docs, and package metadata | Reconcile with only the behavior that exists at A01 exit | A01.4 |

The ledger must additionally name every current `src/lib.rs` export. An export
either becomes an intentionally internal `liknon-core` API used by the
Validation CLI or is removed. A01 does not preserve the former root crate's
public Rust surface merely because it was public in source.

## Phase Handoff Rule

A01.0 changes no runtime path. Its sole exit artifact is the reviewed ledger.
Each later A01 unit consumes that ledger, records every completed move or
removal, and ends with its own tests green. A later unit may not be used to
repair a knowingly broken handoff from an earlier one.

## Verification

- Every file included by the current root package appears in the ledger.
- Every source module and public re-export has one target or is removed.
- Every `include_bytes!`, `include_str!`, `OUT_DIR`, and build-script
  input has an approved owner.
- Every current test has a destination aligned with the behavior it verifies.
- Every current schema and package-owned asset has a destination inside the
  package that must publish or compile it, or an explicit repository-only
  owner.
- Every public Rust example and export is either deliberately owned by an A01
  support crate or explicitly removed pending B04.
- Every command and path replacement is represented in help, docs, fixtures,
  and distribution plans.
- No proposed target directory exists solely as a placeholder.

## Exit Criterion

The reviewed ledger covers the repository and leaves no unresolved ownership,
dependency direction, command identity, workspace path, bootstrap identity,
resource-source, or structural packaging question needed by A01.1-A01.4. Public
resource schemas, complete Manager and Tool behavior, official workflows, and
publication remain assigned to their later owning units.
