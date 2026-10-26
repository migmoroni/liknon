# A01.0 Current-To-Target Ledger

## Status

`Implemented`. The user approved this ledger on 2026-10-26, and A01.1-A01.5
implemented its package boundaries, ownership decisions, operations, and move
order.

This is the implementation ledger of A01.0. It records the observed entry
surfaces, approved target ownership, and buildable move order used by the
completed structural cutover.

[Back to Plan A01](README.md)

## Evidence Snapshot

The inventory was produced from the clean repository at A01.0 entry with:

- `cargo metadata --no-deps --format-version 1`: one package, `liknon 0.5.0`,
  containing one library, one `liknon` binary, two examples, eight integration
  test targets, and one build script;
- `cargo package --list --allow-dirty`: the package includes application
  source, schemas, branding, the complete knowledge corpus, the static skill,
  evaluation material, and repository-only knowledge tooling;
- `rg` searches for public exports, commands, `.validation`, `include!`,
  `include_bytes!`, `include_str!`, `OUT_DIR`, `CARGO_MANIFEST_DIR`, generated
  schemas, and binary test variables;
- the current workflow in `.github/workflows/liknon.yml`;
- `cargo test --all-targets --locked`: 139 tests passed and the two opt-in
  outcome tests remained ignored as designed.

No current Manager, Tool, Hub, policy, persistence, launcher alias, or resource
installation implementation was found. Those proposals therefore have no
removal rows below.

## Decisions Proposed For Approval

### D1. Final Cargo Workspace

The repository becomes a virtual Cargo workspace with resolver 2 and these
final A01 members:

```text
apps/liknon-cli
apps/liknon-validate
apps/liknon-manager
apps/liknon-tool
tools/knowledge
```

`fixtures/outcomes` and `fixtures/process-tool` remain isolated nested
workspaces and are explicitly excluded from root membership. The root owns the
single committed `Cargo.lock`, shared package metadata, MSRV 1.87, dependency
versions, CI, deny policy, license, and repository documentation.

### D2. Single Validation Package Boundary

Create one validation package named `liknon-validate` at
`apps/liknon-validate`. Its library owns configuration, structured contracts,
planning, shell-free execution, process-tree termination, repository evidence,
initialization semantics, structured outcomes, CLI composition, and terminal
presentation. Its two binary targets are thin entry points over that library.

Do not create `liknon-workspace` in A01. Current code proves only one
implemented consumer of workspace discovery and containment. The approved
`.liknon/` root is represented by narrowly scoped validation primitives; a shared
workspace crate is extracted later only when an implemented second product
requires the same behavior.

The library target is an implementation boundary for the binaries, aggregate
launcher package, and private repository checks. It is not the B05 package
`liknon` and does not declare its Rust modules to be a supported public
programmatic API.

### D3. Application Packages And Targets

The application packages are:

| Package | Library responsibility | Binary targets at A01 exit |
| --- | --- | --- |
| `liknon-validate` | Validation implementation and CLI composition | `liknon-validate`, `likv` |
| `liknon-manager` | Identity-only structural shell | `liknon-manager`, `likm` |
| `liknon-tool` | Identity-only structural shell | `liknon-tool`, `likt` |
| `liknon-cli` | Aggregate wrappers and capability-free launcher shell | `liknon`, `likx`, plus the six capability targets |

The aggregate capability binaries call the same application-library entry
points as the standalone packages. They do not copy capability behavior.
Launcher targets expose only identity, version, concise help, and the three
approved namespaces in A01; executable discovery and process forwarding remain
blocked by B04.0. Manager and Tool shells expose no feature commands before
B02.0 and B03.0.

All five workspace packages are `publish = false` at A01 exit. Later publication
owners deliberately enable only their packages:

- B01: `liknon-validate`;
- B02: `liknon-manager` and any dependency its approved contract proves;
- B03: `liknon-tool` and any dependency its approved contract proves;
- B04: `liknon-cli` after all component contracts are stable;
- never: `liknon-knowledge-maintenance`.

The package identity `liknon` is absent after A01 and remains reserved for B05.

### D4. Dependency Direction

```text
liknon-validate          liknon-manager          liknon-tool
        ^                       ^                     ^
        +-----------------------+---------------------+
                                |
                           liknon-cli

liknon-knowledge-maintenance --> liknon-validate
```

Capability application crates never depend on `liknon-cli` or on one another.
Only the aggregate package depends on their library entry points. The
repository tool may use the validation engine to validate authored
configuration recipes, but no product depends on repository tooling.

### D5. Current Public Rust Surface

The current package has not been publicly released and does not define B05.
Its exports receive these explicit dispositions:

| Current export | Decision |
| --- | --- |
| `run_cli` | Move to the small `liknon-validate` application-library entry point used by both binary targets. |
| `ValidatorError` | Move to `liknon-validate`; retain as an engine implementation contract. |
| `config::{load, ValidatedConfig}` | Move to `liknon-validate`; retain for application and private maintenance consumers. |
| `contracts::config::*` | Move with schema version 6 to `liknon-validate`. |
| `contracts::init::*` | Move with init-result version 1 to `liknon-validate`. |
| `contracts::report::*` | Move with report schema version 4 to `liknon-validate`. |
| `planning::*` | Move to `liknon-validate`; retain only the surface required by application composition and engine tests. |
| `execution::{run, run_with_progress, RunOutcome}` and `execution::progress::*` | Move to `liknon-validate`; retain the reporter trait as the CLI adaptation boundary. |
| `initialization::{provision_config, resolve_workspace}` | Move to `liknon-validate`; change only the canonical destination in A01.3. |
| `reporting::result::{human, json}` | Move into `liknon-validate` and internalize; these are presentation adapters, not engine API. |
| `theme::{PaletteProfile, PresentationProfile, Theme}` | Move into `liknon-validate` and internalize. |

Public items currently hidden behind private modules (`dag`, `process`, and
`repository`) remain private implementation. The library exposes only the
narrow modules required by current workspace consumers; A01 does not present
them as the consumer-facing `liknon` API.

### D6. Current Command Surface

| Current command | A01 decision |
| --- | --- |
| `liknon init` | Preserve behavior as `liknon-validate init` and `likv init`; change destination atomically in A01.3. |
| `liknon validate`, `check`, `config validate`, `list`, `explain *`, `schema *` | Preserve semantics under `liknon-validate` and `likv`. |
| global color, presentation, help, version, stream, signal, and exit behavior | Preserve in `liknon-validate`; adapt displayed executable names. |
| `liknon knowledge catalog` and `liknon knowledge show` | Remove from the Validation CLI in A01.2 after preserving authored sources and maintenance evidence. No compatibility route is created. |
| current validation behavior reached through executable `liknon` | Replace in A01.3. `liknon` becomes only the B04-owned launcher shell and does not inherit validation subcommands implicitly. |

### D7. Knowledge, Skill, And Repository Tooling

`docs/validation/knowledge` remains the sole editable knowledge corpus.
`docs/validation/authoring` remains its editorial source and guidance area.
They are repository content, not application package payloads.

The current `build.rs`, generated `OUT_DIR/knowledge_assets.rs`,
`src/knowledge`, knowledge CLI adapters, and knowledge help are removed in
A01.2. There is no generated replacement inside an application.

`tools/knowledge` becomes the private workspace package
`liknon-knowledge-maintenance`, with its own dependencies and tests. It resolves
the repository root explicitly instead of treating its package directory as
the repository root. It validates source content but is never packaged with a
user-facing CLI.

`skills/liknon` is preserved byte-for-byte as non-active C02 design input until
a handoff record identifies useful content and stale command assumptions. It
is removed from all application package inventories. A01 does not rename it
into a future official resource or define a Hub artifact.

### D8. Schema Ownership

| Current schema | Canonical owner after A01 |
| --- | --- |
| `schemas/config.schema.json` | `apps/liknon-validate/schemas/config.schema.json` |
| `schemas/report.schema.json` | `apps/liknon-validate/schemas/report.schema.json` |
| `knowledge-catalog`, `knowledge-sources`, `knowledge-editorial-profiles` | `docs/validation/authoring/schemas/`; repository editorial contracts only |
| `knowledge-forward-trials`, `knowledge-routing`, `knowledge-rubric` | `evals/schemas/`; evaluation-only contracts |

Generated validation schemas remain derived from Rust contract types and are
checked against their single checked-in copies. Existing knowledge schemas are
not promoted into the unapproved B02 resource protocol.

### D9. Workspace Path Cutover

A01.3 replaces `.validation/config.json` with
`.liknon/validation/config.json` in one atomic behavioral unit covering loader,
initialization, structured init results, tests, fixtures, recipes, examples,
help, security guidance, and maintained public documentation. Schema versions
do not change because the JSON shapes do not change.

No fallback, migration, dual read, dual write, deprecated alias, or empty
future resource/AI directory is introduced. Historical plans and the
current-state assessment may continue to mention the old path as historical
evidence.

### D10. Branding

`assets/branding/liknon-ascii.txt` moves with the Validation CLI because its
bytes are compiled into validation help. `assets/branding/liknon.svg` remains a
repository-level Liknon brand source. B04 may choose launcher presentation only
through its own approved contract; A01 does not duplicate either asset.

### D11. Test Ownership

| Current test surface | Target owner |
| --- | --- |
| `tests/config.rs`, `tests/execution.rs`, semantic portions of `tests/repository.rs`, `tests/schemas.rs` | `liknon-validate` integration tests |
| `tests/cli.rs`, human-rendering portions of `tests/repository.rs`, `tests/outcome_fixtures.rs` | `liknon-validate` integration tests |
| `tests/knowledge.rs` and `tools/knowledge/tests.rs` | `liknon-knowledge-maintenance` tests |
| `tests/skills.rs` | repository conformance test for the non-active C02 handoff, outside application packages |
| `tests/common/mod.rs` | `liknon-validate` integration-test support; process fixture remains repository-level |
| `fixtures/outcomes` and `fixtures/process-tool` | isolated repository fixtures, never root workspace members |

Tests that use `CARGO_BIN_EXE_liknon` switch to the directly owned validation
binary. Aggregate/standalone parity is new B04 evidence and is not inferred
from current tests.

### D12. Maintained Documentation And CI

Root README, configuration design, security policy, changelog, validation
reference, CI, deny policy, and package checks are updated to describe only the
A01 exit. Knowledge and skill packaging claims are removed. Manager, Tool, and
launcher routing remain visibly incomplete rather than documented as working.

Historical plans remain untouched. Active plans remain control documents and
are updated only where A01 status or an accepted exact name must be recorded.

## Ledger Key

The individual-file ledger below uses compact codes while retaining every
field required by A01.0.

### Current Responsibility

- `R1`: repository, package, build, dependency, or CI governance;
- `R2`: terminal-independent validation semantics or structured contracts;
- `R3`: Validation CLI parsing, inspection, rendering, theme, streams, or
  branding;
- `R4`: embedded knowledge runtime or its generated build input;
- `R5`: authored knowledge or editorial source;
- `R6`: static AI skill source;
- `R7`: repository maintenance or evaluation input;
- `R8`: test, example, or execution fixture;
- `R9`: maintained product documentation or repository policy.

### Current Consumers And Protecting Evidence

- `E1`: Cargo metadata/package inventory, lockfile, CI, or dependency policy;
- `E2`: configuration and schema tests;
- `E3`: planning, execution, process, repository, and outcome tests;
- `E4`: CLI/help/accessibility/stream/signal tests;
- `E5`: knowledge maintenance, catalog, recipe, and embedded-runtime tests;
- `E6`: skill conformance tests;
- `E7`: maintained documentation and manual source consumers;
- `E8`: evaluation schemas, fixtures, scenarios, or rubrics.

### Target Owners

- `T0`: virtual workspace root and repository governance;
- `T1`: `apps/liknon-validate`;
- `T3`: `apps/liknon-manager` structural shell;
- `T4`: `apps/liknon-tool` structural shell;
- `T5`: `apps/liknon-cli` aggregate/launcher shell;
- `T6`: private package `tools/knowledge`;
- `T7`: canonical authored/editorial source under `docs/validation`;
- `T8`: repository evaluation area under `evals`;
- `T9`: isolated repository fixture;
- `T10`: maintained repository documentation;
- `T11`: non-active source handed to C02.

`Structural` means ownership or location changes without intended validation
semantic change. `Contract` identifies an intentional current-contract change.
Every row is blocked by user approval of A01.0; additional future gates are
named explicitly.

## Individual-File Disposition

| Current path | Responsibility | Consumers / evidence | Target responsibility / candidate owner | Operation / impact | Owning unit | Additional block |
| --- | --- | --- | --- | --- | --- | --- |
| `.github/copilot-instructions.md` | R1 | E1 | T0: repository governance | Preserve / structural | A01.1-A01.4 | none |
| `.github/workflows/liknon.yml` | R1 | E1-E6 | T0: workspace CI | Change package, target, path, and content gates / structural | A01.1-A01.4 | B01-B04 own later publication |
| `.gitignore` | R1 | E1 | T0: repository governance | Preserve / structural | A01.1-A01.4 | none |
| `AGENTS.md` | R1 | E1 | T0: repository governance | Preserve / structural | A01.1-A01.4 | none |
| `CHANGELOG.md` | R9 | E7 | T10: maintained repository documentation | Change to A01 exit contracts | A01.3-A01.4 | none |
| `CONFIG_DESIGN.md` | R9 | E7 | T10: maintained repository documentation | Change to A01 exit contracts | A01.3-A01.4 | none |
| `Cargo.lock` | R1 | E1 | T0: single workspace lockfile | Preserve and regenerate / structural | A01.1-A01.3 | none |
| `Cargo.toml` | R1 | E1 | T0: virtual workspace manifest | Split and replace / structural | A01.1 | none |
| `LICENSE` | R1 | E1 | T0: repository governance | Preserve / structural | A01.1-A01.4 | none |
| `README.md` | R9 | E7 | T10: maintained repository documentation | Change to A01 exit contracts | A01.3-A01.4 | none |
| `SECURITY.md` | R9 | E7 | T10: maintained repository documentation | Change to A01 exit contracts | A01.3-A01.4 | none |
| `assets/branding/liknon-ascii.txt` | R3 | E4 | T1: `apps/liknon-validate/assets/branding/liknon-ascii.txt` | Move / structural | A01.1 | none |
| `assets/branding/liknon.svg` | R9 | E7 | T0: repository-level brand source | Preserve / structural | A01.4 | B04 decides any launcher use |
| `build.rs` | R4 | E1, E5 | No application owner | Remove after source preservation / structural | A01.2 | none |
| `deny.toml` | R1 | E1 | T0: workspace dependency policy | Preserve and reconcile graph / structural | A01.1, A01.4 | none |
| `docs/plans/old/01-human-flow-completion.md` | R9 | E7 | T10: historical archive at `docs/plans/old/01-human-flow-completion.md` | Preserve unchanged / historical evidence | none | never normative |
| `docs/plans/old/02-shared-validation-knowledge.md` | R9 | E7 | T10: historical archive at `docs/plans/old/02-shared-validation-knowledge.md` | Preserve unchanged / historical evidence | none | never normative |
| `docs/plans/old/02.1-knowledge-editorial-refoundation.md` | R9 | E7 | T10: historical archive at `docs/plans/old/02.1-knowledge-editorial-refoundation.md` | Preserve unchanged / historical evidence | none | never normative |
| `docs/plans/old/03-ai-tool-flow.md` | R9 | E7 | T10: historical archive at `docs/plans/old/03-ai-tool-flow.md` | Preserve unchanged / historical evidence | none | never normative |
| `docs/plans/old/04-ai-engineering-persistence-and-safety-foundation.md` | R9 | E7 | T10: historical archive at `docs/plans/old/04-ai-engineering-persistence-and-safety-foundation.md` | Preserve unchanged / historical evidence | none | never normative |
| `docs/plans/old/05-ai-engineering-domain-and-coverage-setup.md` | R9 | E7 | T10: historical archive at `docs/plans/old/05-ai-engineering-domain-and-coverage-setup.md` | Preserve unchanged / historical evidence | none | never normative |
| `docs/plans/old/06-ai-engineering-triage-and-drift-loop.md` | R9 | E7 | T10: historical archive at `docs/plans/old/06-ai-engineering-triage-and-drift-loop.md` | Preserve unchanged / historical evidence | none | never normative |
| `docs/plans/old/07-end-to-end-verification-and-release.md` | R9 | E7 | T10: historical archive at `docs/plans/old/07-end-to-end-verification-and-release.md` | Preserve unchanged / historical evidence | none | never normative |
| `docs/plans/old/README.md` | R9 | E7 | T10: historical archive at `docs/plans/old/README.md` | Preserve unchanged / historical evidence | none | never normative |
| `docs/plans/old/agent-evaluation-program.md` | R9 | E7 | T10: historical archive at `docs/plans/old/agent-evaluation-program.md` | Preserve unchanged / historical evidence | none | never normative |
| `docs/plans/old/ai-bootstrap-and-conformance-contract.md` | R9 | E7 | T10: historical archive at `docs/plans/old/ai-bootstrap-and-conformance-contract.md` | Preserve unchanged / historical evidence | none | never normative |
| `docs/plans/old/ai-decision-policy.md` | R9 | E7 | T10: historical archive at `docs/plans/old/ai-decision-policy.md` | Preserve unchanged / historical evidence | none | never normative |
| `docs/plans/old/ai-work-contexts.md` | R9 | E7 | T10: historical archive at `docs/plans/old/ai-work-contexts.md` | Preserve unchanged / historical evidence | none | never normative |
| `docs/plans/validation-system/A00-plan-coherence-and-reorganization/README.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/A00-plan-coherence-and-reorganization/README.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/A00-plan-coherence-and-reorganization/source-destination-ledger.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/A00-plan-coherence-and-reorganization/source-destination-ledger.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/A00.1-unified-launcher-plan-realignment/README.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/A00.1-unified-launcher-plan-realignment/README.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/A01-product-and-repository-separation/00-product-boundary-and-inventory.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/A01-product-and-repository-separation/00-product-boundary-and-inventory.md` | Preserve; update only accepted status/names/handoffs / planning | A01.0/A01.4 | own decision gates remain |
| `docs/plans/validation-system/A01-product-and-repository-separation/01-cargo-workspace-and-core-extraction.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/A01-product-and-repository-separation/01-cargo-workspace-and-core-extraction.md` | Preserve; update only accepted status/names/handoffs / planning | A01.0/A01.4 | own decision gates remain |
| `docs/plans/validation-system/A01-product-and-repository-separation/02-knowledge-and-ai-resource-separation.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/A01-product-and-repository-separation/02-knowledge-and-ai-resource-separation.md` | Preserve; update only accepted status/names/handoffs / planning | A01.0/A01.4 | own decision gates remain |
| `docs/plans/validation-system/A01-product-and-repository-separation/03-application-composition-and-workspace-resources.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/A01-product-and-repository-separation/03-application-composition-and-workspace-resources.md` | Preserve; update only accepted status/names/handoffs / planning | A01.0/A01.4 | own decision gates remain |
| `docs/plans/validation-system/A01-product-and-repository-separation/04-conformance-and-documentation.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/A01-product-and-repository-separation/04-conformance-and-documentation.md` | Preserve; update only accepted status/names/handoffs / planning | A01.0/A01.4 | own decision gates remain |
| `docs/plans/validation-system/A01-product-and-repository-separation/README.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/A01-product-and-repository-separation/README.md` | Preserve; update only accepted status/names/handoffs / planning | A01.0/A01.4 | own decision gates remain |
| `docs/plans/validation-system/B01-validation-cli-distribution/00-public-contract.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B01-validation-cli-distribution/00-public-contract.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B01-validation-cli-distribution/01-version-and-provenance.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B01-validation-cli-distribution/01-version-and-provenance.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B01-validation-cli-distribution/02-native-builds.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B01-validation-cli-distribution/02-native-builds.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B01-validation-cli-distribution/03-cloudflare-direct.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B01-validation-cli-distribution/03-cloudflare-direct.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B01-validation-cli-distribution/04-end-to-end-verification.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B01-validation-cli-distribution/04-end-to-end-verification.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B01-validation-cli-distribution/05-publication-and-recovery.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B01-validation-cli-distribution/05-publication-and-recovery.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B01-validation-cli-distribution/06-public-verification-and-documentation.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B01-validation-cli-distribution/06-public-verification-and-documentation.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B01-validation-cli-distribution/README.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B01-validation-cli-distribution/README.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B01-validation-cli-distribution/accounts-and-permissions.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B01-validation-cli-distribution/accounts-and-permissions.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B02-resource-hub-and-manager/00-resource-and-hub-contract.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B02-resource-hub-and-manager/00-resource-and-hub-contract.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B02-resource-hub-and-manager/01-hub-catalog-and-publication.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B02-resource-hub-and-manager/01-hub-catalog-and-publication.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B02-resource-hub-and-manager/02-manager-workspace-and-commands.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B02-resource-hub-and-manager/02-manager-workspace-and-commands.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B02-resource-hub-and-manager/03-resolution-locking-and-installation.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B02-resource-hub-and-manager/03-resolution-locking-and-installation.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B02-resource-hub-and-manager/04-security-trust-and-recovery.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B02-resource-hub-and-manager/04-security-trust-and-recovery.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B02-resource-hub-and-manager/05-distribution-and-end-to-end-verification.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B02-resource-hub-and-manager/05-distribution-and-end-to-end-verification.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B02-resource-hub-and-manager/README.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B02-resource-hub-and-manager/README.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B02-resource-hub-and-manager/accounts-and-permissions.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B02-resource-hub-and-manager/accounts-and-permissions.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B03-tool-cli/00-product-and-command-contract.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B03-tool-cli/00-product-and-command-contract.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B03-tool-cli/01-local-catalog-and-retrieval.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B03-tool-cli/01-local-catalog-and-retrieval.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B03-tool-cli/02-context-and-agent-safety.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B03-tool-cli/02-context-and-agent-safety.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B03-tool-cli/03-source-and-native-distribution.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B03-tool-cli/03-source-and-native-distribution.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B03-tool-cli/04-end-to-end-verification-and-publication.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B03-tool-cli/04-end-to-end-verification-and-publication.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B03-tool-cli/README.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B03-tool-cli/README.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B03-tool-cli/accounts-and-permissions.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B03-tool-cli/accounts-and-permissions.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B04-unified-launcher-and-cli-suite/00-launcher-and-suite-contract.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B04-unified-launcher-and-cli-suite/00-launcher-and-suite-contract.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B04-unified-launcher-and-cli-suite/README.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B04-unified-launcher-and-cli-suite/README.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B05-programmatic-api-and-language-bindings/00-programmatic-api-contract.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B05-programmatic-api-and-language-bindings/00-programmatic-api-contract.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B05-programmatic-api-and-language-bindings/01-rust-crate.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B05-programmatic-api-and-language-bindings/01-rust-crate.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B05-programmatic-api-and-language-bindings/02-node-bindings.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B05-programmatic-api-and-language-bindings/02-node-bindings.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B05-programmatic-api-and-language-bindings/03-ruby-bindings.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B05-programmatic-api-and-language-bindings/03-ruby-bindings.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B05-programmatic-api-and-language-bindings/04-python-bindings.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B05-programmatic-api-and-language-bindings/04-python-bindings.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B05-programmatic-api-and-language-bindings/05-dotnet-bindings.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B05-programmatic-api-and-language-bindings/05-dotnet-bindings.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B05-programmatic-api-and-language-bindings/06-cross-language-verification-and-publication.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B05-programmatic-api-and-language-bindings/06-cross-language-verification-and-publication.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B05-programmatic-api-and-language-bindings/README.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B05-programmatic-api-and-language-bindings/README.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/B05-programmatic-api-and-language-bindings/accounts-and-permissions.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/B05-programmatic-api-and-language-bindings/accounts-and-permissions.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/C01-ai-governance-and-workflow-foundation/00-boundary-and-ownership.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/C01-ai-governance-and-workflow-foundation/00-boundary-and-ownership.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/C01-ai-governance-and-workflow-foundation/01-bootstrap-and-conformance.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/C01-ai-governance-and-workflow-foundation/01-bootstrap-and-conformance.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/C01-ai-governance-and-workflow-foundation/02-decision-policy.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/C01-ai-governance-and-workflow-foundation/02-decision-policy.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/C01-ai-governance-and-workflow-foundation/03-workflow-agent-and-skill-contracts.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/C01-ai-governance-and-workflow-foundation/03-workflow-agent-and-skill-contracts.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/C01-ai-governance-and-workflow-foundation/04-persistence-and-artifact-contracts.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/C01-ai-governance-and-workflow-foundation/04-persistence-and-artifact-contracts.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/C01-ai-governance-and-workflow-foundation/05-conformance.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/C01-ai-governance-and-workflow-foundation/05-conformance.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/C01-ai-governance-and-workflow-foundation/README.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/C01-ai-governance-and-workflow-foundation/README.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/C02-official-action-workflow/00-workflow-contract.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/C02-official-action-workflow/00-workflow-contract.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/C02-official-action-workflow/01-resources-and-operations.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/C02-official-action-workflow/01-resources-and-operations.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/C02-official-action-workflow/02-policy-and-safety-application.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/C02-official-action-workflow/02-policy-and-safety-application.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/C02-official-action-workflow/03-verification.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/C02-official-action-workflow/03-verification.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/C02-official-action-workflow/README.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/C02-official-action-workflow/README.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/C03-official-validation-engineering-workflow/00-workflow-contract.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/C03-official-validation-engineering-workflow/00-workflow-contract.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/C03-official-validation-engineering-workflow/01-domain-and-coverage-setup.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/C03-official-validation-engineering-workflow/01-domain-and-coverage-setup.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/C03-official-validation-engineering-workflow/02-triage-and-drift-loop.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/C03-official-validation-engineering-workflow/02-triage-and-drift-loop.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/C03-official-validation-engineering-workflow/03-human-work-context-guidance.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/C03-official-validation-engineering-workflow/03-human-work-context-guidance.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/C03-official-validation-engineering-workflow/04-verification.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/C03-official-validation-engineering-workflow/04-verification.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/C03-official-validation-engineering-workflow/README.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/C03-official-validation-engineering-workflow/README.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/D01-system-conformance-and-agent-evaluation/00-end-to-end-scenario-matrix.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/D01-system-conformance-and-agent-evaluation/00-end-to-end-scenario-matrix.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/D01-system-conformance-and-agent-evaluation/01-contract-isolation-and-security.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/D01-system-conformance-and-agent-evaluation/01-contract-isolation-and-security.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/D01-system-conformance-and-agent-evaluation/02-agent-evaluation-program.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/D01-system-conformance-and-agent-evaluation/02-agent-evaluation-program.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/D01-system-conformance-and-agent-evaluation/03-documentation-and-readiness.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/D01-system-conformance-and-agent-evaluation/03-documentation-and-readiness.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/D01-system-conformance-and-agent-evaluation/README.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/D01-system-conformance-and-agent-evaluation/README.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/README.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/README.md` | Preserve; update only accepted status/names/handoffs / planning | A01.0/A01.4 | own decision gates remain |
| `docs/plans/validation-system/avaliacao-estado-atual.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/avaliacao-estado-atual.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/plans/validation-system/resumo-estado-alvo.md` | R9 | E7 | T10: active planning control at `docs/plans/validation-system/resumo-estado-alvo.md` | Preserve; update only accepted status/names/handoffs / planning | owning active plan | own decision gates remain |
| `docs/strategy/competitive-positioning-and-readiness.md` | R9 | E7 | T10: maintained strategy document | Reconcile links and implemented/planned boundary / documentation | A01.4 | none |
| `docs/validation/README.md` | R9 | E7 | T10: preserve at `docs/validation/README.md` | Change commands, ownership, and canonical path to A01 exit | A01.2-A01.4 | none |
| `docs/validation/authoring/README.md` | R5, R7 | E5, E7 | T7: preserve at `docs/validation/authoring/README.md` | Preserve; update tool/schema paths where required / structural | A01.2, A01.4 | none |
| `docs/validation/authoring/corpus-review.md` | R5, R7 | E5, E7 | T7: preserve at `docs/validation/authoring/corpus-review.md` | Preserve; update tool/schema paths where required / structural | A01.2, A01.4 | none |
| `docs/validation/authoring/editorial-bases.md` | R5, R7 | E5, E7 | T7: preserve at `docs/validation/authoring/editorial-bases.md` | Preserve; update tool/schema paths where required / structural | A01.2, A01.4 | none |
| `docs/validation/authoring/editorial-profiles.json` | R5, R7 | E5, E7 | T7: preserve at `docs/validation/authoring/editorial-profiles.json` | Preserve; update tool/schema paths where required / structural | A01.2, A01.4 | none |
| `docs/validation/authoring/editorial-standard.md` | R5, R7 | E5, E7 | T7: preserve at `docs/validation/authoring/editorial-standard.md` | Preserve; update tool/schema paths where required / structural | A01.2, A01.4 | none |
| `docs/validation/authoring/profiles/concern.md` | R5, R7 | E5, E7 | T7: preserve at `docs/validation/authoring/profiles/concern.md` | Preserve; update tool/schema paths where required / structural | A01.2, A01.4 | none |
| `docs/validation/authoring/profiles/foundation.md` | R5, R7 | E5, E7 | T7: preserve at `docs/validation/authoring/profiles/foundation.md` | Preserve; update tool/schema paths where required / structural | A01.2, A01.4 | none |
| `docs/validation/authoring/profiles/framework.md` | R5, R7 | E5, E7 | T7: preserve at `docs/validation/authoring/profiles/framework.md` | Preserve; update tool/schema paths where required / structural | A01.2, A01.4 | none |
| `docs/validation/authoring/profiles/language.md` | R5, R7 | E5, E7 | T7: preserve at `docs/validation/authoring/profiles/language.md` | Preserve; update tool/schema paths where required / structural | A01.2, A01.4 | none |
| `docs/validation/authoring/profiles/navigation.md` | R5, R7 | E5, E7 | T7: preserve at `docs/validation/authoring/profiles/navigation.md` | Preserve; update tool/schema paths where required / structural | A01.2, A01.4 | none |
| `docs/validation/authoring/profiles/pattern.md` | R5, R7 | E5, E7 | T7: preserve at `docs/validation/authoring/profiles/pattern.md` | Preserve; update tool/schema paths where required / structural | A01.2, A01.4 | none |
| `docs/validation/authoring/profiles/recipe.md` | R5, R7 | E5, E7 | T7: preserve at `docs/validation/authoring/profiles/recipe.md` | Preserve; update tool/schema paths where required / structural | A01.2, A01.4 | none |
| `docs/validation/authoring/profiles/standard.md` | R5, R7 | E5, E7 | T7: preserve at `docs/validation/authoring/profiles/standard.md` | Preserve; update tool/schema paths where required / structural | A01.2, A01.4 | none |
| `docs/validation/authoring/profiles/technology.md` | R5, R7 | E5, E7 | T7: preserve at `docs/validation/authoring/profiles/technology.md` | Preserve; update tool/schema paths where required / structural | A01.2, A01.4 | none |
| `docs/validation/authoring/profiles/tool.md` | R5, R7 | E5, E7 | T7: preserve at `docs/validation/authoring/profiles/tool.md` | Preserve; update tool/schema paths where required / structural | A01.2, A01.4 | none |
| `docs/validation/authoring/review-rubric.md` | R5, R7 | E5, E7 | T7: preserve at `docs/validation/authoring/review-rubric.md` | Preserve; update tool/schema paths where required / structural | A01.2, A01.4 | none |
| `docs/validation/examples/heterogeneous-workspace.json` | R8, R9 | E2, E5, E7 | T10: preserve at `docs/validation/examples/heterogeneous-workspace.json` | Preserve and update canonical config location context / contract | A01.3-A01.4 | none |
| `docs/validation/examples/shared-suites.json` | R8, R9 | E2, E5, E7 | T10: preserve at `docs/validation/examples/shared-suites.json` | Preserve and update canonical config location context / contract | A01.3-A01.4 | none |
| `docs/validation/examples/single-crate.json` | R8, R9 | E2, E5, E7 | T10: preserve at `docs/validation/examples/single-crate.json` | Preserve and update canonical config location context / contract | A01.3-A01.4 | none |
| `docs/validation/flows/human/README.md` | R9 | E7 | T10: preserve at `docs/validation/flows/human/README.md` | Change commands, ownership, and canonical path to A01 exit | A01.2-A01.4 | none |
| `docs/validation/knowledge/README.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/README.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/SOURCES.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/SOURCES.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/catalog.json` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/catalog.json` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/explanations/README.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/explanations/README.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/explanations/concerns/README.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/explanations/concerns/README.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/explanations/concerns/reproducible-release-evidence.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/explanations/concerns/reproducible-release-evidence.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/explanations/foundations/README.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/explanations/foundations/README.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/explanations/foundations/evidence-quality.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/explanations/foundations/evidence-quality.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/explanations/foundations/source-policy.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/explanations/foundations/source-policy.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/explanations/foundations/test-types.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/explanations/foundations/test-types.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/explanations/foundations/validation-model.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/explanations/foundations/validation-model.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/explanations/foundations/validation-strategy.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/explanations/foundations/validation-strategy.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/explanations/frameworks/README.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/explanations/frameworks/README.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/explanations/languages/README.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/explanations/languages/README.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/explanations/languages/javascript.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/explanations/languages/javascript.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/explanations/languages/rust.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/explanations/languages/rust.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/explanations/languages/typescript.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/explanations/languages/typescript.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/explanations/patterns/README.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/explanations/patterns/README.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/explanations/patterns/evidence-chain.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/explanations/patterns/evidence-chain.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/explanations/patterns/isolation-and-determinism.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/explanations/patterns/isolation-and-determinism.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/explanations/patterns/oracles-and-failure-paths.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/explanations/patterns/oracles-and-failure-paths.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/explanations/technologies/README.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/explanations/technologies/README.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/explanations/technologies/git.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/explanations/technologies/git.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/explanations/technologies/nodejs.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/explanations/technologies/nodejs.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/how-to/README.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/how-to/README.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/how-to/recipes/README.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/how-to/recipes/README.md` | Preserve and update canonical config path / contract | A01.2-A01.3 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/how-to/recipes/javascript-typescript-package-gate.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/how-to/recipes/javascript-typescript-package-gate.md` | Preserve and update canonical config path / contract | A01.2-A01.3 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/how-to/recipes/rust-cli-release-gate.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/how-to/recipes/rust-cli-release-gate.md` | Preserve and update canonical config path / contract | A01.2-A01.3 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/reference/README.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/reference/README.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/reference/standards/README.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/reference/standards/README.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/reference/standards/wcag-22.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/reference/standards/wcag-22.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/reference/tools/README.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/reference/tools/README.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/reference/tools/cargo.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/reference/tools/cargo.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/reference/tools/clippy.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/reference/tools/clippy.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/reference/tools/git.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/reference/tools/git.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/reference/tools/pnpm.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/reference/tools/pnpm.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/reference/tools/rustfmt.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/reference/tools/rustfmt.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/reference/tools/typescript-compiler.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/reference/tools/typescript-compiler.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/reference/tools/vitest.md` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/reference/tools/vitest.md` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/knowledge/sources.json` | R5 | E5, E7 | T7: canonical authored source at `docs/validation/knowledge/sources.json` | Preserve outside application packages / structural | A01.2 | B02.0 owns future resource packaging |
| `docs/validation/reference/README.md` | R9 | E7 | T10: preserve at `docs/validation/reference/README.md` | Change commands, ownership, and canonical path to A01 exit | A01.2-A01.4 | none |
| `docs/validation/reference/cli.md` | R9 | E7 | T10: preserve at `docs/validation/reference/cli.md` | Change commands, ownership, and canonical path to A01 exit | A01.2-A01.4 | none |
| `docs/validation/reference/configuration.md` | R9 | E7 | T10: preserve at `docs/validation/reference/configuration.md` | Change commands, ownership, and canonical path to A01 exit | A01.2-A01.4 | none |
| `docs/validation/reference/exit-status.md` | R9 | E7 | T10: preserve at `docs/validation/reference/exit-status.md` | Change commands, ownership, and canonical path to A01 exit | A01.2-A01.4 | none |
| `docs/validation/reference/initialization.md` | R9 | E7 | T10: preserve at `docs/validation/reference/initialization.md` | Change commands, ownership, and canonical path to A01 exit | A01.2-A01.4 | none |
| `docs/validation/reference/reports.md` | R9 | E7 | T10: preserve at `docs/validation/reference/reports.md` | Change commands, ownership, and canonical path to A01 exit | A01.2-A01.4 | none |
| `docs/validation/reference/schemas.md` | R9 | E7 | T10: preserve at `docs/validation/reference/schemas.md` | Change commands, ownership, and canonical path to A01 exit | A01.2-A01.4 | none |
| `evals/README.md` | R7 | E8 | T8: preserve at `evals/README.md` | Preserve; update moved schema/source links where required / structural | A01.2, A01.4 | D01 owns future integrated evaluation |
| `evals/rubrics/shared-knowledge.json` | R7 | E8 | T8: preserve at `evals/rubrics/shared-knowledge.json` | Preserve; update moved schema/source links where required / structural | A01.2, A01.4 | D01 owns future integrated evaluation |
| `evals/scenarios/shared-knowledge/prompts/bounded-proof.md` | R7 | E8 | T8: preserve at `evals/scenarios/shared-knowledge/prompts/bounded-proof.md` | Preserve; update moved schema/source links where required / structural | A01.2, A01.4 | D01 owns future integrated evaluation |
| `evals/scenarios/shared-knowledge/prompts/evidence-before-tool.md` | R7 | E8 | T8: preserve at `evals/scenarios/shared-knowledge/prompts/evidence-before-tool.md` | Preserve; update moved schema/source links where required / structural | A01.2, A01.4 | D01 owns future integrated evaluation |
| `evals/scenarios/shared-knowledge/prompts/javascript-typescript-selection.md` | R7 | E8 | T8: preserve at `evals/scenarios/shared-knowledge/prompts/javascript-typescript-selection.md` | Preserve; update moved schema/source links where required / structural | A01.2, A01.4 | D01 owns future integrated evaluation |
| `evals/scenarios/shared-knowledge/prompts/minimal-selection.md` | R7 | E8 | T8: preserve at `evals/scenarios/shared-knowledge/prompts/minimal-selection.md` | Preserve; update moved schema/source links where required / structural | A01.2, A01.4 | D01 owns future integrated evaluation |
| `evals/scenarios/shared-knowledge/prompts/missing-topic.md` | R7 | E8 | T8: preserve at `evals/scenarios/shared-knowledge/prompts/missing-topic.md` | Preserve; update moved schema/source links where required / structural | A01.2, A01.4 | D01 owns future integrated evaluation |
| `evals/scenarios/shared-knowledge/prompts/offline-depth.md` | R7 | E8 | T8: preserve at `evals/scenarios/shared-knowledge/prompts/offline-depth.md` | Preserve; update moved schema/source links where required / structural | A01.2, A01.4 | D01 owns future integrated evaluation |
| `evals/scenarios/shared-knowledge/prompts/precise-source-location.md` | R7 | E8 | T8: preserve at `evals/scenarios/shared-knowledge/prompts/precise-source-location.md` | Preserve; update moved schema/source links where required / structural | A01.2, A01.4 | D01 owns future integrated evaluation |
| `evals/scenarios/shared-knowledge/prompts/synthesis-boundary.md` | R7 | E8 | T8: preserve at `evals/scenarios/shared-knowledge/prompts/synthesis-boundary.md` | Preserve; update moved schema/source links where required / structural | A01.2, A01.4 | D01 owns future integrated evaluation |
| `evals/scenarios/shared-knowledge/scenarios.json` | R7 | E8 | T8: preserve at `evals/scenarios/shared-knowledge/scenarios.json` | Preserve; update moved schema/source links where required / structural | A01.2, A01.4 | D01 owns future integrated evaluation |
| `examples/inspect.rs` | R8 | E2, E7 | T1: `apps/liknon-validate/examples/inspect.rs` | Move; document as internal engine example / structural | A01.1 | B05 before consumer-facing API claim |
| `fixtures/knowledge/routing.json` | R7 | E8 | T8: `evals/fixtures/knowledge/routing.json` | Move to evaluation ownership / structural | A01.2 | none |
| `fixtures/outcomes/.gitignore` | R8 | E3, E4 | T9: preserve isolated fixture at `fixtures/outcomes/.gitignore` | Preserve / structural | A01.1-A01.4 | Excluded from root workspace |
| `fixtures/outcomes/.validation/config.json` | R8 | E3, E4 | T9: `fixtures/outcomes/.liknon/validation/config.json` | Move atomically with canonical path / contract | A01.3 | none |
| `fixtures/outcomes/Cargo.lock` | R8 | E3, E4 | T9: preserve isolated fixture at `fixtures/outcomes/Cargo.lock` | Preserve / structural | A01.1-A01.4 | Excluded from root workspace |
| `fixtures/outcomes/Cargo.toml` | R8 | E3, E4 | T9: preserve isolated fixture at `fixtures/outcomes/Cargo.toml` | Preserve / structural | A01.1-A01.4 | Excluded from root workspace |
| `fixtures/outcomes/README.md` | R8 | E3, E4 | T9: preserve isolated fixture at `fixtures/outcomes/README.md` | Preserve and update commands/path / contract | A01.3-A01.4 | Excluded from root workspace |
| `fixtures/outcomes/fail/Cargo.toml` | R8 | E3, E4 | T9: preserve isolated fixture at `fixtures/outcomes/fail/Cargo.toml` | Preserve / structural | A01.1-A01.4 | Excluded from root workspace |
| `fixtures/outcomes/fail/src/lib.rs` | R8 | E3, E4 | T9: preserve isolated fixture at `fixtures/outcomes/fail/src/lib.rs` | Preserve / structural | A01.1-A01.4 | Excluded from root workspace |
| `fixtures/outcomes/pass/Cargo.toml` | R8 | E3, E4 | T9: preserve isolated fixture at `fixtures/outcomes/pass/Cargo.toml` | Preserve / structural | A01.1-A01.4 | Excluded from root workspace |
| `fixtures/outcomes/pass/src/lib.rs` | R8 | E3, E4 | T9: preserve isolated fixture at `fixtures/outcomes/pass/src/lib.rs` | Preserve / structural | A01.1-A01.4 | Excluded from root workspace |
| `fixtures/process-tool/.gitignore` | R8 | E3, E4 | T9: preserve isolated fixture at `fixtures/process-tool/.gitignore` | Preserve / structural | A01.1-A01.4 | Excluded from root workspace |
| `fixtures/process-tool/Cargo.lock` | R8 | E3, E4 | T9: preserve isolated fixture at `fixtures/process-tool/Cargo.lock` | Preserve / structural | A01.1-A01.4 | Excluded from root workspace |
| `fixtures/process-tool/Cargo.toml` | R8 | E3, E4 | T9: preserve isolated fixture at `fixtures/process-tool/Cargo.toml` | Preserve / structural | A01.1-A01.4 | Excluded from root workspace |
| `fixtures/process-tool/src/main.rs` | R8 | E3, E4 | T9: preserve isolated fixture at `fixtures/process-tool/src/main.rs` | Preserve / structural | A01.1-A01.4 | Excluded from root workspace |
| `schemas/config.schema.json` | R2 | E2 | T1: `apps/liknon-validate/schemas/config.schema.json` | Move canonical checked-in schema / structural | A01.1 | none |
| `schemas/knowledge-catalog.schema.json` | R5, R7 | E5 | T7: `docs/validation/authoring/schemas/knowledge-catalog.schema.json` | Move as repository editorial contract / structural | A01.2 | B02.0 before any resource-schema promotion |
| `schemas/knowledge-editorial-profiles.schema.json` | R5, R7 | E5 | T7: `docs/validation/authoring/schemas/knowledge-editorial-profiles.schema.json` | Move as repository editorial contract / structural | A01.2 | B02.0 before any resource-schema promotion |
| `schemas/knowledge-forward-trials.schema.json` | R7 | E8 | T8: `evals/schemas/knowledge-forward-trials.schema.json` | Move as evaluation-only contract / structural | A01.2 | none |
| `schemas/knowledge-routing.schema.json` | R7 | E8 | T8: `evals/schemas/knowledge-routing.schema.json` | Move as evaluation-only contract / structural | A01.2 | none |
| `schemas/knowledge-rubric.schema.json` | R7 | E8 | T8: `evals/schemas/knowledge-rubric.schema.json` | Move as evaluation-only contract / structural | A01.2 | none |
| `schemas/knowledge-sources.schema.json` | R5, R7 | E5 | T7: `docs/validation/authoring/schemas/knowledge-sources.schema.json` | Move as repository editorial contract / structural | A01.2 | B02.0 before any resource-schema promotion |
| `schemas/report.schema.json` | R2 | E2 | T1: `apps/liknon-validate/schemas/report.schema.json` | Move canonical checked-in schema / structural | A01.1 | none |
| `skills/liknon/SKILL.md` | R6 | E6, E7 | T11: preserve at `skills/liknon/SKILL.md` as non-active C02 input | Preserve bytes; remove package/distribution claim / contract | A01.2 | C02 defines any future official resource |
| `skills/liknon/manifest.json` | R6 | E6, E7 | T11: preserve at `skills/liknon/manifest.json` as non-active C02 input | Preserve bytes; remove package/distribution claim / contract | A01.2 | C02 defines any future official resource |
| `skills/liknon/references/audit.md` | R6 | E6, E7 | T11: preserve at `skills/liknon/references/audit.md` as non-active C02 input | Preserve bytes; remove package/distribution claim / contract | A01.2 | C02 defines any future official resource |
| `skills/liknon/references/config.md` | R6 | E6, E7 | T11: preserve at `skills/liknon/references/config.md` as non-active C02 input | Preserve bytes; remove package/distribution claim / contract | A01.2 | C02 defines any future official resource |
| `skills/liknon/references/run.md` | R6 | E6, E7 | T11: preserve at `skills/liknon/references/run.md` as non-active C02 input | Preserve bytes; remove package/distribution claim / contract | A01.2 | C02 defines any future official resource |
| `skills/liknon/references/triage.md` | R6 | E6, E7 | T11: preserve at `skills/liknon/references/triage.md` as non-active C02 input | Preserve bytes; remove package/distribution claim / contract | A01.2 | C02 defines any future official resource |
| `src/cli/arguments.rs` | R3 | E4 | T1: `apps/liknon-validate/src/cli/arguments.rs` | Move and adapt executable name / structural | A01.1, A01.3 | none |
| `src/cli/help/execution.rs` | R3 | E4 | T1: `apps/liknon-validate/src/cli/help/execution.rs` | Move and adapt executable name / structural | A01.1, A01.3 | none |
| `src/cli/help/initialization.rs` | R3 | E4 | T1: `apps/liknon-validate/src/cli/help/initialization.rs` | Move and adapt executable name / structural | A01.1, A01.3 | none |
| `src/cli/help/inspection.rs` | R3 | E4 | T1: `apps/liknon-validate/src/cli/help/inspection.rs` | Move and adapt executable name / structural | A01.1, A01.3 | none |
| `src/cli/help/knowledge.rs` | R4 | E4, E5 | No Validation CLI owner; authored bytes remain T7 | Remove runtime surface / contract | A01.2 | B03.0 owns future installed-resource retrieval |
| `src/cli/help/mod.rs` | R3 | E4 | T1: `apps/liknon-validate/src/cli/help/mod.rs` | Move and adapt executable name / structural | A01.1, A01.3 | none |
| `src/cli/help/root.rs` | R3 | E4 | T1: `apps/liknon-validate/src/cli/help/root.rs` | Move and adapt executable name / structural | A01.1, A01.3 | none |
| `src/cli/help/values.rs` | R3 | E4 | T1: `apps/liknon-validate/src/cli/help/values.rs` | Move and adapt executable name / structural | A01.1, A01.3 | none |
| `src/cli/inspection.rs` | R3 | E4 | T1: `apps/liknon-validate/src/cli/inspection.rs` | Move and adapt executable name / structural | A01.1, A01.3 | none |
| `src/cli/knowledge.rs` | R4 | E4, E5 | No Validation CLI owner; authored bytes remain T7 | Remove runtime surface / contract | A01.2 | B03.0 owns future installed-resource retrieval |
| `src/cli/mod.rs` | R3 | E4 | T1: `apps/liknon-validate/src/cli/mod.rs` | Move and adapt executable name / structural | A01.1, A01.3 | none |
| `src/config/directories.rs` | R2 | E2, E3 | T1: `apps/liknon-validate/src/config/directories.rs` | Move; expose only required application boundary / structural | A01.1 | none |
| `src/config/loader.rs` | R2 | E2, E3 | T1: `apps/liknon-validate/src/config/loader.rs` | Move; expose only required application boundary / structural | A01.1 | A01.3 changes canonical path |
| `src/config/mod.rs` | R2 | E2, E3 | T1: `apps/liknon-validate/src/config/mod.rs` | Move; expose only required application boundary / structural | A01.1 | none |
| `src/config/parameters.rs` | R2 | E2, E3 | T1: `apps/liknon-validate/src/config/parameters.rs` | Move; expose only required application boundary / structural | A01.1 | none |
| `src/config/validation.rs` | R2 | E2, E3 | T1: `apps/liknon-validate/src/config/validation.rs` | Move; expose only required application boundary / structural | A01.1 | none |
| `src/contracts/config.rs` | R2 | E2, E3 | T1: `apps/liknon-validate/src/contracts/config.rs` | Move; expose only required application boundary / structural | A01.1 | none |
| `src/contracts/init.rs` | R2 | E2, E3 | T1: `apps/liknon-validate/src/contracts/init.rs` | Move; expose only required application boundary / structural | A01.1 | A01.3 changes canonical path |
| `src/contracts/mod.rs` | R2 | E2, E3 | T1: `apps/liknon-validate/src/contracts/mod.rs` | Move; expose only required application boundary / structural | A01.1 | none |
| `src/contracts/report.rs` | R2 | E2, E3 | T1: `apps/liknon-validate/src/contracts/report.rs` | Move; expose only required application boundary / structural | A01.1 | none |
| `src/dag.rs` | R2 | E2, E3 | T1: `apps/liknon-validate/src/dag.rs` | Move; expose only required application boundary / structural | A01.1 | none |
| `src/error.rs` | R2 | E2, E3 | T1: `apps/liknon-validate/src/error.rs` | Move; expose only required application boundary / structural | A01.1 | none |
| `src/execution/checks.rs` | R2 | E2, E3 | T1: `apps/liknon-validate/src/execution/checks.rs` | Move; expose only required application boundary / structural | A01.1 | none |
| `src/execution/groups.rs` | R2 | E2, E3 | T1: `apps/liknon-validate/src/execution/groups.rs` | Move; expose only required application boundary / structural | A01.1 | none |
| `src/execution/mod.rs` | R2 | E2, E3 | T1: `apps/liknon-validate/src/execution/mod.rs` | Move; expose only required application boundary / structural | A01.1 | none |
| `src/execution/prerequisites.rs` | R2 | E2, E3 | T1: `apps/liknon-validate/src/execution/prerequisites.rs` | Move; expose only required application boundary / structural | A01.1 | none |
| `src/execution/progress.rs` | R2 | E2, E3 | T1: `apps/liknon-validate/src/execution/progress.rs` | Move; expose only required application boundary / structural | A01.1 | none |
| `src/execution/suites.rs` | R2 | E2, E3 | T1: `apps/liknon-validate/src/execution/suites.rs` | Move; expose only required application boundary / structural | A01.1 | none |
| `src/initialization.rs` | R2 | E2, E3 | T1: `apps/liknon-validate/src/initialization.rs` | Move; expose only required application boundary / structural | A01.1 | A01.3 changes canonical path |
| `src/knowledge/mod.rs` | R4 | E4, E5 | No Validation CLI owner; authored bytes remain T7 | Remove runtime surface / contract | A01.2 | B03.0 owns future installed-resource retrieval |
| `src/lib.rs` | R2 | E2, E3 | T1: validation semantics and exports in the Validation CLI library root | Move and consolidate; retire package `liknon` / structural | A01.1-A01.5 | B05 reserves package `liknon` |
| `src/lib.rs` surface `cli`, `reporting`, `theme`, and `run_cli` | R3, R4 | E4, E5 | T1: same Validation CLI library root | Move, consolidate, and remove knowledge composition / contract | A01.1-A01.5 | none |
| `src/main.rs` | R3 | E4 | T1: direct Validation CLI binary wrappers | Move and replace binary identity / contract | A01.1, A01.3 | none |
| `src/planning/mod.rs` | R2 | E2, E3 | T1: `apps/liknon-validate/src/planning/mod.rs` | Move; expose only required application boundary / structural | A01.1 | none |
| `src/process.rs` | R2 | E2, E3 | T1: `apps/liknon-validate/src/process.rs` | Move; expose only required application boundary / structural | A01.1 | none |
| `src/reporting/execution.rs` | R3 | E3, E4 | T1: `apps/liknon-validate/src/reporting/execution.rs` | Move and internalize / structural | A01.1 | none |
| `src/reporting/format.rs` | R3 | E3, E4 | T1: `apps/liknon-validate/src/reporting/format.rs` | Move and internalize / structural | A01.1 | none |
| `src/reporting/mod.rs` | R3 | E3, E4 | T1: `apps/liknon-validate/src/reporting/mod.rs` | Move and internalize / structural | A01.1 | none |
| `src/reporting/result/human.rs` | R3 | E3, E4 | T1: `apps/liknon-validate/src/reporting/result/human.rs` | Move and internalize / structural | A01.1 | none |
| `src/reporting/result/json.rs` | R3 | E3, E4 | T1: `apps/liknon-validate/src/reporting/result/json.rs` | Move and internalize / structural | A01.1 | none |
| `src/reporting/result/mod.rs` | R3 | E3, E4 | T1: `apps/liknon-validate/src/reporting/result/mod.rs` | Move and internalize / structural | A01.1 | none |
| `src/reporting/tree.rs` | R3 | E3, E4 | T1: `apps/liknon-validate/src/reporting/tree.rs` | Move and internalize / structural | A01.1 | none |
| `src/repository/fingerprint.rs` | R2 | E2, E3 | T1: `apps/liknon-validate/src/repository/fingerprint.rs` | Move; expose only required application boundary / structural | A01.1 | none |
| `src/repository/mod.rs` | R2 | E2, E3 | T1: `apps/liknon-validate/src/repository/mod.rs` | Move; expose only required application boundary / structural | A01.1 | none |
| `src/repository/status.rs` | R2 | E2, E3 | T1: `apps/liknon-validate/src/repository/status.rs` | Move; expose only required application boundary / structural | A01.1 | none |
| `src/theme/clap.rs` | R3 | E4 | T1: `apps/liknon-validate/src/theme/clap.rs` | Move and internalize / structural | A01.1 | none |
| `src/theme/mod.rs` | R3 | E4 | T1: `apps/liknon-validate/src/theme/mod.rs` | Move and internalize / structural | A01.1 | none |
| `src/theme/palette.rs` | R3 | E4 | T1: `apps/liknon-validate/src/theme/palette.rs` | Move and internalize / structural | A01.1 | none |
| `src/theme/presentation.rs` | R3 | E4 | T1: `apps/liknon-validate/src/theme/presentation.rs` | Move and internalize / structural | A01.1 | none |
| `src/theme/style.rs` | R3 | E4 | T1: `apps/liknon-validate/src/theme/style.rs` | Move and internalize / structural | A01.1 | none |
| `tests/cli.rs` | R8 | E4, E5 | T1: `apps/liknon-validate/tests/cli.rs` | Move; remove embedded-knowledge cases; rename binary / contract | A01.1-A01.3 | none |
| `tests/common/mod.rs` | R8 | E2, E3 | T1: shared Validation CLI integration-test support | Move and consolidate / structural | A01.1, A01.5 | none |
| `tests/common/mod.rs` CLI-consumed helpers | R8 | E4 | T1: same Validation CLI integration-test support | Move and consolidate / structural | A01.1, A01.5 | none |
| `tests/config.rs` | R8 | E2, E3 | T1: `apps/liknon-validate/tests/config.rs` | Move and update canonical paths / structural | A01.1, A01.3 | none |
| `tests/execution.rs` | R8 | E2, E3 | T1: `apps/liknon-validate/tests/execution.rs` | Move and update canonical paths / structural | A01.1, A01.3 | none |
| `tests/knowledge.rs` | R8 | E5 | T6: `tools/knowledge/tests/repository.rs` | Move; replace package-embedding assertions / structural | A01.2 | none |
| `tests/outcome_fixtures.rs` | R8 | E3, E4 | T1: `apps/liknon-validate/tests/outcome_fixtures.rs` | Move; rename binary and fixture config path / contract | A01.1, A01.3 | none |
| `tests/repository.rs` | R8 | E3 | T1: engine repository-evidence integration tests | Split and move semantic assertions / structural | A01.1 | none |
| `tests/repository.rs` human-rendering assertions | R8 | E4 | T1: Validation CLI rendering integration tests | Split and move / structural | A01.1 | none |
| `tests/schemas.rs` | R8 | E2, E3 | T1: `apps/liknon-validate/tests/schemas.rs` | Move and update canonical paths / structural | A01.1, A01.3 | none |
| `tests/skills.rs` | R8 | E6 | T6 repository conformance for T11 handoff | Move and narrow to source safety/inventory / structural | A01.2 | C02 defines active skill |
| `tools/knowledge/contracts.rs` | R7 | E5 | T6: `tools/knowledge/src/contracts.rs` | Move into private package; resolve repository root explicitly / structural | A01.2 | none |
| `tools/knowledge/integrity.rs` | R7 | E5 | T6: `tools/knowledge/src/integrity.rs` | Move into private package; resolve repository root explicitly / structural | A01.2 | none |
| `tools/knowledge/main.rs` | R7 | E5 | T6: `tools/knowledge/src/main.rs` | Move into private package; resolve repository root explicitly / structural | A01.2 | none |
| `tools/knowledge/recipes.rs` | R7 | E5 | T6: `tools/knowledge/src/recipes.rs` | Move into private package; resolve repository root explicitly / structural | A01.2 | none |
| `tools/knowledge/release.rs` | R7 | E5 | T6: `tools/knowledge/src/release.rs` | Move into private package; resolve repository root explicitly / structural | A01.2 | none |
| `tools/knowledge/tests.rs` | R7 | E5 | T6: `tools/knowledge/src/tests.rs` | Move into private package; resolve repository root explicitly / structural | A01.2 | none |

## Generated And Untracked Package Outputs

| Surface | Decision |
| --- | --- |
| `.cargo_vcs_info.json`, `Cargo.toml.orig` inside `cargo package` output | Regenerated by Cargo per package; never authored or moved. |
| `target/**`, generated binaries, and `.crate` archives | Remain ignored build outputs; never enter the ledger as source. |
| `OUT_DIR/knowledge_assets.rs` | Ceases to exist when the knowledge build script is removed in A01.2. |

## Approved Additions Once The Gate Passes

These rows describe new approved structural surfaces; they are not disguised
as removals of current files.

| New surface | Responsibility | Owner | Unit | Later gate |
| --- | --- | --- | --- | --- |
| virtual root `Cargo.toml` workspace tables | Shared build and package metadata | T0 | A01.1 | none |
| `apps/liknon-validate/Cargo.toml`, library, schemas, and two binary wrappers | Complete Validation CLI package | T1 | A01.1-A01.5 | B01 distribution |
| `tools/knowledge/Cargo.toml` and package layout | Private editorial maintenance package | T6 | A01.2 | none |
| C02 handoff record for `skills/liknon` | Preserve useful source without presenting it as active | T7; it records T11 source status | A01.2 | C02 defines future resource |
| Manager long/compact identity shells | No feature grammar | T3 | A01.3 | B02.0 |
| Tool long/compact identity shells | No feature grammar | T4 | A01.3 | B03.0 |
| aggregate package and eight fixed binary targets | Shared wrappers plus launcher identity shell | T5 | A01.3 | B04.0 routing/distribution |

## Buildable Move Order

1. **A01.1:** create the virtual workspace, move the existing validation
   behavior under explicit modules and package boundaries, and keep the complete
   workspace green.
2. **A01.2:** turn `tools/knowledge` into a private workspace package; preserve
   authored sources; remove build-time embedding, knowledge commands, and
   application package payloads; record the skill handoff. End with no
   application depending on authored content.
3. **A01.3:** rename the direct application package and binaries to
   `liknon-validate`/`likv`, create the approved Manager, Tool, and aggregate
   structural packages, and remove package `liknon`. No legacy validation route
   remains under `liknon`. Atomically replace the validation path throughout
   active code, tests, fixtures, recipes, examples, help, and maintained
   documentation.
4. **A01.4:** reconcile all maintained documentation and CI, inspect package
   contents/dependency direction, run local installation smoke tests for the
   applications present, and execute every required gate.
5. **A01.5:** consolidate validation semantics, schemas, tests, CLI adaptation,
   and both binaries in the single `liknon-validate` package; remove the
   unnecessary supporting package and re-run every gate.

Every numbered handoff is independently buildable and testable. A later step
does not repair an intentionally broken earlier tree.

## Decisions Deliberately Deferred

- Manager grammar, mutation behavior, Hub and resource contracts: B02.0.
- Tool grammar, retrieval, integrity, and machine output: B03.0.
- Launcher executable resolution, compatibility, forwarding, and aggregate
  release behavior: B04.0.
- Policy, persistence, and bootstrap ownership: C01.
- Future official skill/workflow identities and payload contracts: C02/C03.
- Public package `liknon` and programmatic APIs: B05.

## Approval Record

Approved by the user on 2026-10-26. A01.1-A01.5 implement the exact package
names, ownership, operations, and move order above.
