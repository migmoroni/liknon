# Plan A01: Liknon Product And Repository Separation

## Status

`Decision gate` until A01.0 produces and the user approves the complete
current-to-target inventory. A01.1 through A01.4 are `Provisional` and must not
be implemented from candidate destinations that the ledger has not accepted.

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

The approved target has three independent applications:

| Product | Approved responsibility |
| --- | --- |
| Validation CLI | Configure, inspect, plan, and execute declarative validation |
| Manager | Acquire and mutate installed resource state through an approved Hub contract |
| Tool | Read and return installed resources offline without mutation |

Candidate internal crates may separate validation semantics from shared
workspace primitives. Their exact names, public visibility, and dependency
graph are A01.0 decisions, not prerequisites invented by later A01 documents.

## Boundary Rules

1. The inventory begins with what exists, not with an imagined destination.
2. Every current path and public surface receives one target owner or an
   explicit, evidence-backed removal decision.
3. Only implemented items may be removed. Unimplemented aliases, CLIs,
   resources, paths, or package channels simply do not enter the ledger.
4. Every later A01 unit consumes the approved ledger.
5. Validation behavior retains one implementation.
6. Applications may share libraries but do not invoke one another as hidden
   subprocesses.
7. The Validation CLI remains usable without Manager, Tool, or installed AI
   resources.
8. Manager and Tool feature grammars remain B02.0 and B03.0 decisions.
9. Generic AI governance and persistence ownership remain C01 decisions.
10. Official resources and workflow graphs remain C02/C03 responsibilities.
11. Future public in-process APIs remain B04 responsibilities.
12. Every unit ends in a buildable and testable repository; a later unit is not
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
|   |-- liknon-cli/
|   |-- liknon-manager/
|   `-- liknon-tool/
|-- crates/
|   |-- <validation-core>/
|   `-- <workspace-contracts>/
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
| A01.0 | [Product Boundary And Inventory](00-product-boundary-and-inventory.md) | `Decision gate` | Reviewed evidence-based ledger |
| A01.1 | [Cargo Workspace And Core Extraction](01-cargo-workspace-and-core-extraction.md) | `Provisional` | Buildable workspace and approved shared boundaries |
| A01.2 | [Knowledge And AI Resource Separation](02-knowledge-and-ai-resource-separation.md) | `Provisional` | Authored content separated from application runtime ownership |
| A01.3 | [Application Composition And Workspace Boundary](03-application-composition-and-workspace-resources.md) | `Provisional` | Three isolated application packages and current workspace-path cutover |
| A01.4 | [Conformance And Documentation](04-conformance-and-documentation.md) | `Provisional` | Proven boundaries and reconciled documentation |

## Cross-Track Handoff

- B01 begins only after the Validation CLI package boundary is stable.
- C01 begins from the stable A01 workspace/product boundary.
- B02.0 and B03.0 consume structural shells and shared primitives without
  inheriting speculative command grammars from A01.
- C02 and C03 begin only after C01 and the relevant B02/B03 contracts.
- D01 verifies the integrated result rather than repairing product boundaries.

## Global Acceptance Criteria

- [ ] A01.0 accounts for every current package input, module, export, command,
      schema, test, fixture, asset, tool, build input, and maintained document.
- [ ] Every ledger action is supported by current evidence and approved target
      direction.
- [ ] No task removes a product, alias, route, directory, or distribution model
      that never existed.
- [ ] The final Cargo graph has no circular application dependency.
- [ ] Validation retains one semantic implementation and its current behavior.
- [ ] Validation packaging contains no authored resource corpus unless a later
      approved contract explicitly requires one.
- [ ] Manager and Tool packages make no feature claim before B02/B03.
- [ ] `.liknon/validation/config.json` is the sole target validation path after
      cutover, with no compatibility reader for `.validation/`.
- [ ] Policy and persistence ownership is not assigned by A01.
- [ ] Official workflow identities and process numbers do not appear in
      application control flow.
- [ ] Maintained documentation describes only behavior available at the A01
      exit and labels later behavior as planned.
- [ ] Full repository formatting, lint, tests, doctests, MSRV, package-boundary,
      and relevant fixture gates pass after every implementation unit.

## Result

A01 exits with validated current behavior behind explicit product and source
boundaries. Later plans can define Manager, Tool, AI governance, official
workflows, distribution, and APIs without moving ownership again or treating a
proposal as historical implementation.
