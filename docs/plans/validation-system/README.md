# Liknon Validation-System Roadmap

## Status

`Approved target`.

This file is the concise end-to-end index for the active plan set. It defines
plan ownership, status, dependencies, and decision gates. Detailed product,
workflow, policy, resource, distribution, and evaluation contracts belong to
their owning folders.

The roadmap describes a target under development. Only the repository code,
schemas, tests, and the current-state record prove implemented behavior.

## Foundation

Read these documents in order before using an implementation plan:

1. [Current-State Assessment](avaliacao-estado-atual.md) - `Current-state record`
2. [Target-State Summary](resumo-estado-alvo.md) - `Approved target` with
   explicit open decisions
3. [Plan Coherence And Reorganization](A00-plan-coherence-and-reorganization/README.md)
   - `Historical` completed documentation meta-plan
4. [Unified Launcher, CLI Suite, And Command Plan Realignment](A00.1-unified-launcher-plan-realignment/README.md)
   - `Implementation-ready` documentation meta-plan that precedes A01

The current-state assessment answers what exists. The target summary answers
what has been approved and what remains undecided. A subordinate plan may add
detail, but it may not silently contradict either document.

Until A00.1 is completed, its approved command identities, aggregate-package
model, and launcher boundary take precedence over conflicting target
identities still present in the target summary or subordinate active plans.
All other ownership and target decisions retain their existing precedence.

## Plan Tracks

### A: Product And Repository Boundaries

| Plan | Status | Owns |
| --- | --- | --- |
| [A01 Product And Repository Separation](A01-product-and-repository-separation/README.md) | `Decision gate` until A01.0 is approved | Current-to-target inventory, package boundaries, source ownership, repository layout, and structural cutover |

### B: Product Implementation And Distribution

| Plan | Status | Owns |
| --- | --- | --- |
| [B01 Validation CLI Distribution](B01-cli-distribution/README.md) | `Provisional`, blocked by A01 | Source-package and native distribution of `liknon` / `likx` |
| [B02 Resource Hub And Manager](B02-resource-hub-and-manager/README.md) | `Decision gate` at B02.0 | Resource protocol, Hub, Manager behavior, and Manager distribution |
| [B03 Tool CLI](B03-tool-cli/README.md) | `Decision gate` at B03.0 | Offline resource consumption, agent-facing retrieval, and Tool distribution |
| [B04 Programmatic APIs And Language Bindings](B04-programmatic-api-and-language-bindings/README.md) | `Future` | Deliberate in-process APIs with demonstrated consumers |

All three CLIs use the same categories of distribution channel: exact GitHub
source, crates.io source packages, and project-hosted native binaries through
Cloudflare/site delivery. They retain independent package identities, release
artifacts, readiness gates, credentials, and publication authority.

### C: AI Governance And Official Workflows

| Plan | Status | Owns |
| --- | --- | --- |
| [C01 AI Governance And Workflow Foundation](C01-ai-governance-and-workflow-foundation/README.md) | `Decision gate` | Generic bootstrap, authority, policy, workflow/resource distinctions, persistence envelopes, and cross-product ownership |
| [C02 Official Action Workflow](C02-official-action-workflow/README.md) | `Provisional`, blocked by C01 and B02/B03 contracts | Replaceable official workflow for bounded assisted operations |
| [C03 Official Validation-Engineering Workflow](C03-official-validation-engineering-workflow/README.md) | `Provisional`, blocked by C01 and B02/B03 contracts | Replaceable official workflow for domain, coverage, incident, and drift work |

C01 defines platform-neutral contracts. C02 and C03 define ordinary official
resources that consume those contracts. Process numbers, node names, selected
profiles, and selected skills remain local to their workflow resources and are
not platform constants.

### D: Integrated Conformance And Evaluation

| Plan | Status | Owns |
| --- | --- | --- |
| [D01 System Conformance And Agent Evaluation](D01-system-conformance-and-agent-evaluation/README.md) | `Provisional`, blocked by implemented B/C capabilities | Cross-product scenarios, isolation, abuse review, agent evaluation, and integrated readiness evidence |

D01 does not publish products. Publication and recovery remain in B01, B02,
and B03.

## Dependency Graph

```text
current-state assessment + approved target summary
                         |
                         v
          A01 product/repository boundaries
              |                    |
              |                    +--> B01 Validation CLI delivery
              |
              +--> C01 generic AI/workflow governance contracts
                         |
                         +--> B02.0 Manager/Hub decision gate
                         |
                         +--> B03.0 Tool decision gate
                                  |
                 approved product contracts and implementations
                                  |
                         +--------+--------+
                         |                 |
                         v                 v
                C02 official       C03 official validation-
                action workflow    engineering workflow
                         \                 /
                          +-------+-------+
                                  |
                                  v
                  D01 integrated conformance and evaluation

B04 programmatic APIs and language bindings: future, independently activated
```

Distribution readiness remains product-specific. `D01` may verify an
integrated candidate, but it must not own publication already assigned to
`B01`, `B02`, or `B03`.

## Active Decision Gates

### A01.0: Structural Inventory

Before repository restructuring, approve an evidence-based ledger that maps
every implemented source, package surface, schema, test, asset, fixture, and
maintained document to one target owner and one operation. An unimplemented
proposal cannot appear as a removal task.

### C01: Cross-Product Ownership

Before AI-governance implementation, decide:

- which product validates and initializes the human-owned policy;
- which product provides deterministic persistence operations;
- which responsibilities remain with the external AI host;
- how these operations remain generic and independent from official workflow
  identities.

### B02.0: Manager And Hub Contract

Before Manager implementation, approve resource identity, Hub behavior,
manifest/lock ownership, command grammar, machine outputs, trust, recovery, and
publication boundaries. Existing command names in B02 are candidates until
this gate passes.

### B03.0: Tool Contract

Before Tool implementation, approve local discovery, read-only behavior,
command grammar, machine outputs, integrity handling, the bootstrap projection
interface, and the policy/persistence inspection boundary. Existing command
names in B03 are candidates until this gate passes.

### B04.0: Programmatic API Activation

Do not activate B04 until at least one real in-process consumer justifies a
deliberate API. CLI syntax and internal Rust modules are not sufficient API
design evidence.

## Execution Order

1. Execute A00.1 to realign the active plans with the approved launcher and
   command identities.
2. Approve A01.0, then perform the A01 structural cutover in buildable units.
3. Begin B01 delivery work and C01 contract work from the stable A01 boundary.
4. Resolve B02.0 and B03.0; only then implement their provisional phases.
5. Implement C02 and C03 as resources over approved generic contracts.
6. Execute D01 integrated conformance and agent evaluation.
7. Activate the future programmatic-API plan only through its independent
   consumer-driven gate.

Each plan owns its own detailed handoff and acceptance criteria. Completion of
one plan never implicitly marks another plan complete.

## Canonical Ownership Rules

- Current implementation facts belong to code, schemas, tests, and the
  current-state assessment.
- Approved target direction belongs to the target summary.
- Product behavior and distribution belong to A/B plans.
- Generic AI governance belongs to C01.
- Official workflow definitions belong to C02 or C03.
- Integrated conformance and agent evaluation belong to D01.
- Public documentation may summarize these contracts but must not redefine
  them independently.
- Generated documentation must identify its canonical source.
- Archived plans are evidence, never normative dependencies.

## Historical Plans

Completed and superseded plans are retained under
[docs/plans/old](../old/README.md). They explain how the current implementation
was reached but do not override the current-state assessment, target summary,
or active plans.

## Roadmap Completion

This roadmap is coherent when:

- every active capability has one owning plan;
- every open design question is visible as a gate;
- no active plan depends normatively on the historical archive;
- no proposal that was never implemented is described as something to remove;
- generic platform contracts remain separate from official workflow content;
- product release work remains separate from integrated-system evaluation;
- all active links resolve and the next executable step is unambiguous.
