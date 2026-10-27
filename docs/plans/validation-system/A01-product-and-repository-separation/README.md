# Plan A01: Liknon Product And Repository Separation

## Status

`Implementation complete; hosted verification pending`. A01.0 through A01.5
were implemented and locally verified on 2026-10-26. The validation engine,
schemas, CLI adaptation, and both binaries now belong to one
`liknon-validator` package. The consolidated commit still requires the hosted
macOS and Windows jobs before the B01 handoff.

The [validation-system roadmap](../README.md) owns cross-track order. This plan
owns only product boundaries, repository structure, source ownership, and the
structural cutover from the current single package.

## Dependencies

- [Current-State Assessment](../avaliacao-estado-atual.md)
- [Target-State Summary](../resumo-estado-alvo.md)
- the repository code, schemas, tests, manifests, and CI
- [A00 Plan Reorganization](../A00-plan-coherence-and-reorganization/README.md)

Historical plans may explain why current behavior exists, but they are not
normative inputs to A01.

## Objective

Transform the current single Cargo package into explicit product and source
boundaries without changing validation semantics accidentally or defining
future Manager, Tool, Hub, policy, persistence, workflow, or binding contracts
inside the structural phase.

The approved target has three independent capability applications and one
capability-free launcher application:

| Product | Approved responsibility |
| --- | --- |
| Validation CLI | Configure, inspect, plan, and execute declarative validation |
| Manager | Acquire and mutate installed resource state through an approved Hub contract |
| Tool | Read and return installed resources offline without mutation |
| Unified launcher | Select one explicit capability namespace and forward the invocation without implementing it |

Validation semantics and CLI adaptation remain separate Rust modules within the
single `liknon-validator` package. A shared crate is introduced only when more
than one implemented product demonstrates that boundary; the current validation
product does not require one.

## Boundary Rules

1. The inventory begins with what exists, not with an imagined destination.
2. Every current path and public surface receives one target owner or an
   explicit, evidence-backed removal decision.
3. Only implemented items may be removed. Unimplemented aliases, CLIs,
   resources, paths, or package channels simply do not enter the ledger.
4. Every later A01 unit consumes the approved ledger.
5. Validation behavior retains one implementation.
6. Capability applications may share libraries but do not invoke one another
   as hidden subprocesses. Only the launcher may invoke the explicitly selected
   capability executable.
7. The Validation CLI remains usable without Manager, Tool, or installed AI
   resources.
8. Manager and Tool feature grammars remain B02.0 and B03.0 decisions.
9. Generic AI governance and persistence ownership remain C01 decisions.
10. Official resources and workflow graphs remain C02/C03 responsibilities.
11. Launcher routing and aggregate distribution remain B04 responsibilities.
12. Future public in-process APIs remain B05 responsibilities.
13. Every unit ends in a buildable and testable repository; a later unit is not
    a repair phase for an intentionally broken handoff.

## Exclusions

A01 does not approve Manager or Tool command grammar, Hub semantics, policy or
persistence ownership, official workflow graphs, product distribution, or
future language bindings. Their owning B/C tracks consume A01 boundaries after
the inventory gate.

## Target Repository Direction

The ledger should evaluate a shape similar to the following, without treating
candidate internal crate names as approved before A01.0:

```text
liknon/
|-- Cargo.toml
|-- Cargo.lock
|-- apps/
|   |-- liknon-cli/          # Launcher and aggregate package
|   |-- liknon-validator/
|   |-- liknon-manager/
|   `-- liknon-tool/
|-- docs/
|-- assets/
|-- evals/
|-- fixtures/
|-- resources/          # only after an owning C plan creates real resources
`-- tools/
```

Empty target directories and placeholder resources are not deliverables.

## Consumer Workspace Direction

The approved target root is `.liknon/`, with independent validation, resource,
and AI subcontracts. A01 owns only the structural replacement needed by
implemented validation behavior and shared path primitives:

```text
.liknon/
`-- validation/
    `-- config.json
```

Resource manifest/lock files belong to B02. Policy and persistence contracts
belong to C01. A01 must not create those files or assign their command owner in
advance.

## Deliverables And Implementation Units

| Unit | Document | Status | Primary result |
| --- | --- | --- | --- |
| A01.0 | [Product Boundary And Inventory](00-product-boundary-and-inventory.md) and [approved ledger](current-to-target-ledger.md) | `Complete` | Reviewed evidence-based ledger |
| A01.1 | [Cargo Workspace And Core Extraction](01-cargo-workspace-and-core-extraction.md) | `Complete` | Buildable workspace and approved shared boundaries |
| A01.2 | [Knowledge And AI Resource Separation](02-knowledge-and-ai-resource-separation.md) | `Complete` | Authored content separated from application runtime ownership |
| A01.3 | [Application Composition And Workspace Boundary](03-application-composition-and-workspace-resources.md) | `Complete` | Three capability packages, one launcher/aggregate package, and current workspace-path cutover |
| A01.4 | [Conformance And Documentation](04-conformance-and-documentation.md) | `Complete` | Proven boundaries and reconciled documentation |
| A01.5 | [Validation Package Consolidation](05-validation-package-consolidation.md) | `Implemented; hosted verification pending` | One `liknon-validator` package owning engine modules, schemas, CLI adaptation, and binaries |

## Cross-Track Handoff

- B01 may begin after hosted CI verifies the consolidated Validation CLI
  package boundary implemented by A01.5.
- C01 begins from the stable A01 workspace/product boundary.
- B02.0 and B03.0 consume structural shells and shared primitives without
  inheriting speculative command grammars from A01.
- B04.0 consumes stable application boundaries and command contracts before
  defining launcher discovery, compatibility, and aggregate distribution.
- C02 and C03 begin only after C01 and the relevant B02/B03 contracts.
- B05 remains future until a demonstrated in-process consumer activates it.
- D01 verifies the integrated result rather than repairing product boundaries.

## Global Acceptance Criteria

- [x] A01.0 accounts for every current package input, module, export, command,
      schema, test, fixture, asset, tool, build input, and maintained document.
- [x] Every ledger action is supported by current evidence and approved target
      direction.
- [x] No task removes a product, alias, route, directory, or distribution model
      that never existed.
- [x] The final Cargo graph has no circular application dependency.
- [x] Validation retains one semantic implementation and its current behavior.
- [x] `liknon-validator` owns that implementation, its schemas, CLI adaptation,
      and both validation binaries without a separate `liknon-validation`
      package.
- [x] `liknon-validator` and `likv` expose that validation implementation
      directly, while `liknon` and `likx` are launcher-only entry points.
- [x] Package `liknon-cli` owns the approved aggregate targets without relying
      on dependency-owned executables.
- [x] Validation packaging contains no authored resource corpus unless a later
      approved contract explicitly requires one.
- [x] Manager and Tool packages make no feature claim before B02/B03.
- [x] `.liknon/validation/config.json` is the sole target validation path after
      cutover, with no compatibility reader for `.validation/`.
- [x] Policy and persistence ownership is not assigned by A01.
- [x] Official workflow identities and process numbers do not appear in
      application control flow.
- [x] Package identity `liknon` remains reserved for B05 and owns no CLI target.
- [x] Maintained documentation describes only behavior available at the A01
      exit and labels later behavior as planned.
- [ ] Full repository formatting, lint, tests, doctests, MSRV, package-boundary,
      relevant fixture, and hosted platform gates pass after every
      implementation unit. Local gates pass; the final hosted run is pending.

## Pending Verification

The virtual workspace, application shells, consolidated Validation package,
private knowledge maintenance, canonical workspace path, schemas, tests, and
documentation have their final A01 owners. A01 exits after the next hosted CI
run confirms the consolidated commit on macOS and Windows. Later plans can then
consume these boundaries without first repairing the validation package.
