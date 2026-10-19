# A00: Validation-System Plan Coherence And Reorganization

## 1. Status

`Historical`. This documentation-only meta-plan has been executed and is
retained as the record of the reorganization. It did not authorize any
product-code, schema, CLI, resource, packaging, or publication change.

Its completion means that the implementation plans describe one coherent route
from the product that exists today to the approved target. It does not mean that
the target itself has been implemented.

## 2. Objective

Rebuild the validation-system plan set so that:

- current implementation facts and future proposals are never mixed;
- every target capability has one owning plan;
- open decisions remain explicit decision gates;
- the former standalone plans `03` through `07` are decomposed into named,
  lettered plan tracks instead of preserving the obsolete linear sequence;
- generic platform contracts remain separate from official resources;
- product implementation, distribution, cross-product workflows, and system
  evaluation have separate ownership;
- historical plans remain available as evidence without acting as normative
  dependencies;
- the root README becomes a concise end-to-end roadmap rather than a duplicate
  specification.

## 3. Exclusions

This work must not:

- change Rust code, Cargo manifests, schemas, tests, CI, or shipped assets;
- create the three-CLI architecture;
- define unresolved Manager or Tool command grammars by assumption;
- implement policy, persistence, workflows, resources, the Hub, or publishing;
- preserve superseded proposals as compatibility requirements;
- create removal tasks for aliases, products, paths, or behaviors that were
  never implemented;
- rewrite historical plans to make them appear current;
- use the reorganization itself as evidence that a product decision has been
  approved.

## 4. Dependencies And Authoritative Inputs

The refinement must use the following precedence.

1. The current code, schemas, tests, manifests, and CI describe implemented
   behavior.
2. `../avaliacao-estado-atual.md` summarizes that implemented state.
3. `../resumo-estado-alvo.md` records approved target direction and explicitly
   open decisions.
4. This meta-plan controls how the remaining plans are reorganized.
5. Subordinate plans may add scoped detail only when their prerequisites and
   decision gates permit it.
6. Files under `docs/plans/old/` are historical evidence only.

When a lower-precedence document conflicts with a higher-precedence source, the
lower-precedence document must be corrected. It must not silently redefine the
source above it.

## 5. Deliverables

- a reconciled current-state record and approved target summary;
- a concise root roadmap with ownership, dependencies, and decision gates;
- evidence-based A01 and normalized B product tracks;
- separate C01, C02, and C03 governance and workflow tracks;
- a D01 integrated conformance and agent-evaluation track;
- a source-to-destination ledger and a non-normative historical archive;
- structural, semantic, link, and formatting verification evidence.

## 6. Required Plan Status Vocabulary

Every active plan README and every standalone contract retained by this work
must declare exactly one of these statuses near its title:

| Status | Meaning |
| --- | --- |
| `Current-state record` | Describes behavior verified in the repository today |
| `Approved target` | Describes a target decision already accepted by the user |
| `Decision gate` | Must resolve named questions before dependent implementation |
| `Implementation-ready` | Has approved scope, ownership, dependencies, and acceptance criteria |
| `Provisional` | Useful design material that is blocked by a decision gate |
| `Future` | Deliberately outside the active implementation sequence |
| `Historical` | Retained only as evidence and never used as a normative dependency |

Words such as `required`, `must`, `canonical`, and `approved` may only be used
for facts or decisions supported by an authoritative input. Candidate designs
must be labeled as candidates.

## 7. Target Plan Topology

The refined plan set should use these tracks:

```text
validation-system/
|-- README.md
|-- avaliacao-estado-atual.md
|-- resumo-estado-alvo.md
|-- A00-plan-coherence-and-reorganization/
|   `-- README.md
|-- A01-product-and-repository-separation/
|   `-- ...
|-- B01-cli-distribution/
|   `-- ...
|-- B02-resource-hub-and-manager/
|   `-- ...
|-- B03-tool-cli/
|   `-- ...
|-- B04-programmatic-api-and-language-bindings/
|   `-- ...
|-- C01-ai-governance-and-workflow-foundation/
|   `-- ...
|-- C02-official-action-workflow/
|   `-- ...
|-- C03-official-validation-engineering-workflow/
|   `-- ...
`-- D01-system-conformance-and-agent-evaluation/
    `-- ...
```

Track meanings:

- `A`: establishes product and repository boundaries;
- `B`: defines, implements, distributes, and verifies individual products;
- `C`: defines cross-product AI governance and installs official workflow
  resources on top of generic contracts;
- `D`: verifies the integrated system and evaluates agent behavior;
- `B04` remains future until real programmatic consumers justify it.

The letters organize ownership. They do not imply that every `B` task must
finish before every `C` task. The root dependency graph must state the actual
prerequisites.

## 8. End-To-End Dependency Model

The refined root roadmap must express at least this ordering:

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

## 9. Workstream 0: Reconcile The Foundation Documents

### 9.1 Current-State Assessment

Review `../avaliacao-estado-atual.md` only for factual drift discovered while
performing this work. Do not expand it with target architecture.

### 9.2 Target Summary

Update `../resumo-estado-alvo.md` before rewriting subordinate plans:

- record the approved decision that Validation CLI, Manager, and Tool use the
  same channel categories: exact GitHub source, crates.io source packages, and
  native binaries delivered through the project site/Cloudflare;
- retain independent package identities, artifacts, release readiness, and
  publication controls for each CLI;
- remove Manager and Tool channel selection from the open-question list;
- keep their complete command grammars as open decision gates;
- add the unresolved ownership boundary for policy evaluation, policy
  initialization, workflow persistence, and artifact persistence;
- distinguish workspace-owned contracts from the CLI that validates or
  manipulates them;
- ensure that no target statement claims those facilities are implemented.

### 9.3 Foundation Checkpoint

Do not rewrite dependent plans until the revised target summary clearly
separates:

- approved product boundaries;
- approved distribution direction;
- unresolved command surfaces;
- unresolved cross-product ownership.

## 10. Workstream 1: Replace The Root Roadmap

Rewrite `../README.md` as a concise index and dependency map.

It must contain:

- links to the current-state assessment and target summary before any
  implementation track;
- a short description and status for every `A`, `B`, `C`, and `D` folder;
- the end-to-end dependency graph;
- explicit decision gates and the plans they block;
- the distinction between product-specific release gates and integrated-system
  evaluation;
- a link to historical plans as non-normative evidence.

It must not contain:

- complete policy category definitions;
- workflow node definitions;
- full command grammars;
- resource schemas;
- detailed release procedures;
- duplicate acceptance criteria from subordinate plans;
- a numbered invariant catalogue that acts as a second specification.

The root roadmap should answer four questions only:

1. What exists now?
2. What is the approved target?
3. In what order are decisions and implementations made?
4. Which plan owns each part?

## 11. Workstream 2: Reconcile A01 With The Real Repository

### 11.1 Evidence-Based Inventory

Rebuild the A01 inventory from `../avaliacao-estado-atual.md` and the repository
tree. Every entry must identify:

- the current path or public surface;
- its implemented responsibility;
- its target owner;
- the intended operation: preserve, move, split, change, or remove;
- the target plan that performs that operation;
- evidence supporting the classification.

Only implemented items may receive a removal task. Proposals that never
existed must disappear from active task lists rather than being described as
things to remove.

### 11.2 Decision Order

A01.0 must produce the reviewed inventory and boundary ledger. Later A01 files
may consume that ledger, but they must not predeclare speculative crate splits,
public APIs, or final destinations as already approved.

Names such as `liknon-core` and `liknon-workspace` may remain candidates until
the ledger proves that the split is useful and identifies actual consumers.

### 11.3 A01 Completion Boundary

A01 should define and implement repository/product separation. It should not
also define Manager commands, Tool query semantics, official workflows, Hub
publication, or the integrated agent evaluation program.

## 12. Workstream 3: Normalize The B Product Tracks

### 12.1 B01 Validation CLI Distribution

Keep B01 focused on the Validation CLI:

- distinguish current binary/package identity from target identities;
- consume the final A01 ledger instead of dictating internal crate structure;
- retain GitHub source, crates.io source package, native builds, Cloudflare/site
  delivery, provenance, recovery, and public verification;
- remove integrated AI-system release responsibilities;
- depend only on capabilities actually needed to release the Validation CLI.

### 12.2 B02 Resource Hub And Manager

Make `B02.0` a genuine decision gate. It must approve, before implementation:

- the initial Hub contract;
- the Manager's exact mutation boundary;
- command grammar and machine-readable outputs;
- manifest and lock ownership;
- trust, provenance, rollback, and recovery behavior;
- how the shared distribution channel model applies to Manager.

Existing command lists are design candidates until that gate is accepted.
Mark downstream B02 files `Provisional` and explicitly blocked by B02.0.

### 12.3 B03 Tool CLI

Make `B03.0` a genuine decision gate. It must approve, before implementation:

- local discovery and read-only boundaries;
- exact command grammar and machine-readable outputs;
- integrity and compatibility behavior;
- the interface exposed to humans, agents, and editor integrations;
- whether policy evaluation or persistence inspection belongs to Tool;
- how the shared distribution channel model applies to Tool.

Existing command lists are design candidates until that gate is accepted.
Mark downstream B03 files `Provisional` and explicitly blocked by B03.0.

### 12.4 B04 Future APIs

Keep B04 marked `Future`. It may describe activation criteria and potential
consumer ecosystems, but it must not prescribe bindings, package layouts, or
publication work before a real programmatic API and consumer are approved.

## 13. Workstream 4: Create The C Cross-Product Tracks

The former plans `03` through `06` must be classified section by section. They
must not be moved intact under new names.

### 13.1 C01 AI Governance And Workflow Foundation

Create a product-neutral foundation with a structure similar to:

```text
C01-ai-governance-and-workflow-foundation/
|-- README.md
|-- 00-boundary-and-ownership.md
|-- 01-bootstrap-and-conformance.md
|-- 02-decision-policy.md
|-- 03-workflow-agent-and-skill-contracts.md
|-- 04-persistence-and-artifact-contracts.md
`-- 05-conformance.md
```

C01 owns generic contracts only:

- bootstrap and loading order;
- authority hierarchy;
- workflow, agent-profile, skill, tool-descriptor, and resource distinctions;
- policy semantics and workflow overrides;
- generic persistence and artifact envelopes;
- ownership boundaries among workspace files, CLIs, the AI host, and humans;
- conformance requirements for external workflows.

C01 must resolve the current policy/persistence ownership conflict before it
becomes `Implementation-ready`. It must not define the official workflows'
domain-specific node graphs.

Move and refine the substance of:

- `ai-bootstrap-and-conformance-contract.md` into C01.1;
- `ai-decision-policy.md` into C01.2;
- generic policy portions of `03-ai-tool-flow.md` into C01;
- generic persistence, safety, trust, and artifact portions of
  `04-ai-engineering-persistence-and-safety-foundation.md` into C01.

### 13.2 C02 Official Action Workflow

Create a folder similar to:

```text
C02-official-action-workflow/
|-- README.md
|-- 00-workflow-contract.md
|-- 01-resources-and-operations.md
|-- 02-policy-and-safety-application.md
`-- 03-verification.md
```

C02 owns only the replaceable official action workflow and the official
resources it references. It may define operations such as policy guidance,
configuration assistance, run, triage, and audit as resource behavior, but it
must not hard-code those operations into generic CLIs merely because the
official workflow uses them.

The workflow must consume C01 contracts and approved B02/B03 capabilities. It
must remain replaceable by an external conforming workflow.

### 13.3 C03 Official Validation-Engineering Workflow

Create a folder similar to:

```text
C03-official-validation-engineering-workflow/
|-- README.md
|-- 00-workflow-contract.md
|-- 01-domain-and-coverage-setup.md
|-- 02-triage-and-drift-loop.md
|-- 03-human-work-context-guidance.md
`-- 04-verification.md
```

C03 owns:

- the official workflow graph;
- its Domain and Sensorium processes;
- its Incident and Drift processes;
- its use of profiles, skills, tools, knowledge, policy, and persistence;
- human guidance for setup and operational work contexts;
- transition and continuation behavior defined by that workflow.

The process numbers and transitions belong to this official resource. They
must not appear as platform constants in A01, B02, B03, or C01.

Move and refine the substance of:

- official validation-engineering portions of
  `04-ai-engineering-persistence-and-safety-foundation.md`;
- `05-ai-engineering-domain-and-coverage-setup.md`;
- `06-ai-engineering-triage-and-drift-loop.md`;
- `ai-work-contexts.md`.

## 14. Workstream 5: Create D01 Integrated Verification

Create a folder similar to:

```text
D01-system-conformance-and-agent-evaluation/
|-- README.md
|-- 00-end-to-end-scenario-matrix.md
|-- 01-contract-isolation-and-security.md
|-- 02-agent-evaluation-program.md
`-- 03-documentation-and-readiness.md
```

D01 owns:

- direct human Validation CLI scenarios;
- CLI-only agent discovery;
- documentation-only agent use;
- official action-workflow evaluation;
- official validation-engineering evaluation;
- external workflow conformance;
- contract-isolation and abuse tests;
- model/scenario matrices, evidence capture, scoring, and interpretation;
- integrated documentation and readiness evidence.

Move and refine the substance of:

- the integration, security, documentation, and evaluation portions of
  `07-end-to-end-verification-and-release.md`;
- `agent-evaluation-program.md`.

Move product publication and package verification requirements from the former
plan `07` to the relevant `B01`, `B02`, or `B03` owner. Do not duplicate them in
D01.

## 15. Source-To-Destination Classification Ledger

The completed review record is
[Source-To-Destination Ledger](source-destination-ledger.md).

Before relocating any standalone plan, create a temporary review ledger with
one row per meaningful section:

| Source | Section | Classification | Destination | Required rewrite | Blocking decision |
| --- | --- | --- | --- | --- | --- |
| `03-ai-tool-flow.md` | Generic policy | Generic contract | C01 | Remove official-workflow assumptions | Policy owner |
| `03-ai-tool-flow.md` | Official action operations | Official resource | C02 | Reference C01 and B03 | Tool contract |
| `04-...foundation.md` | Artifact envelope | Generic contract | C01 | Make workflow-neutral | Persistence owner |
| `04-...foundation.md` | Engineering root | Official resource | C03 | Remove platform-global process assumptions | C01 |
| `05-...setup.md` | Domain/Sensorium | Official resource | C03 | Preserve as resource-local nodes | C03 contract |
| `06-...loop.md` | Incident/Drift | Official resource | C03 | Preserve as resource-local nodes | C03 contract |
| `07-...release.md` | Cross-system scenarios | System verification | D01 | Reference product owners | B/C implementations |
| `07-...release.md` | Product publication | Product delivery | Relevant B track | Remove integrated-release ownership | Product readiness |

The completed ledger is review evidence. It may live inside this meta-plan's
folder while the reorganization is active. Every source section must be marked
as retained, rewritten, archived, or removed as superseded before the original
standalone file is archived.

## 16. Canonical Source Rules

The refinement must identify one editable canonical source for each contract or
resource.

- Product behavior belongs to the owning A or B plan.
- Generic AI governance belongs to C01.
- Official workflow definitions belong to C02 or C03.
- Cross-system evaluation belongs to D01.
- Public explanatory documentation may summarize these sources but must not
  independently redefine them.
- Generated documentation must identify its generator and source.
- A workflow, policy category, skill, or profile must not have two independently
  editable normative definitions under both `docs/` and `resources/`.

The plans may defer the final runtime source path until A01 establishes the
repository layout, but they must still identify the conceptual owner.

## 17. Archive And Link Treatment

After all content has a reviewed destination:

- move the superseded standalone plans `03` through `07` to
  `docs/plans/old/`;
- move superseded standalone support plans there only after their active
  content has been incorporated into C or D;
- retain their original text as historical evidence rather than rewriting it;
- add a short archive index that states their historical status;
- remove active-plan dependencies on archived files;
- repair all relative links after moves;
- ensure the root roadmap links to the archive only as history.

Already archived plans `01`, `02`, and `02.1` must not be restored as active
phases. Active plans should cite the current-state assessment for implemented
behavior and use the archive only when historical rationale is genuinely
useful.

## 18. Verification Procedure

### 18.1 Structural Verification

Confirm that:

- no standalone numbered phase remains at the validation-system root;
- every active plan belongs to a lettered folder;
- every folder has a README with status, objective, dependencies, exclusions,
  deliverables, and acceptance criteria;
- every subordinate file is linked from exactly one owning README;
- archived files are not normative dependencies.

### 18.2 Semantic Verification

Search the active plan set for:

- claims that unimplemented CLIs or aliases currently exist;
- removal tasks for unimplemented proposals;
- `approved` or `canonical` claims unsupported by the target summary;
- Manager or Tool command lists presented as final before B02.0/B03.0;
- policy or persistence responsibilities assigned to multiple products;
- process `0`, `1`, `2`, or `3` treated as platform constants;
- release tasks owned by D01;
- duplicate normative workflow or resource definitions;
- references to the old `.validation/` path presented as the target layout;
- broken links to plans moved under `docs/plans/old/`.

### 18.3 Link And Formatting Verification

Run the repository's existing Markdown and documentation checks when available.
At minimum:

- inspect all relative Markdown links in the changed plan tree;
- run `git diff --check`;
- confirm that no product source or generated artifact changed;
- review `git status --short` so user-owned moves under `docs/plans/old/` are
  preserved.

This documentation-only reorganization does not require the Rust validation
gate unless an implementation file is changed accidentally.

## 19. Execution Checkpoints

### Checkpoint 1: Foundation Approval

Present the corrected current/target foundation and root dependency graph.
Do not refine implementation tasks until their open ownership decisions are
visible.

### Checkpoint 2: Track Skeleton Approval

Present the proposed C01, C02, C03, and D01 folder READMEs plus the completed
source-to-destination ledger. Do not archive the standalone plans yet.

### Checkpoint 3: Detailed Plan Review

Refine A01 and the B/C/D subordinate files. Confirm that provisional plans do
not masquerade as implementation-ready work.

### Checkpoint 4: Archive And Final Audit

Archive superseded standalone plans, repair links, perform structural and
semantic verification, and present the resulting execution order.

## 20. Acceptance Criteria

This meta-plan is complete because:

1. [x] the current-state assessment and target summary are mutually consistent;
2. [x] the target summary records the shared distribution-channel decision for all
   three CLIs;
3. [x] unresolved policy and persistence ownership is either decided or remains an
   explicit blocking gate;
4. [x] the root README is a concise roadmap rather than a parallel specification;
5. [x] A01 contains no removal tasks for products or aliases that never existed;
6. [x] A01 decisions flow from a reviewed current-to-target inventory;
7. [x] B02.0 and B03.0 are explicit decision gates and downstream work is marked
   provisional until they pass;
8. [x] the generic AI foundation is separated from both official workflows;
9. [x] the former plans `03` through `07` have reviewed destinations in C, D, or
   the appropriate B track;
10. [x] official workflow process numbers and transitions remain resource-local;
11. [x] D01 owns integrated conformance and agent evaluation, not product
    publication;
12. [x] each contract and resource has one conceptual and editable owner;
13. [x] active plans do not normatively depend on `docs/plans/old/`;
14. [x] all active plan links resolve;
15. [x] no application code, schema, package, generated asset, or CI behavior was
    changed by this documentation-only effort;
16. [x] the final roadmap makes the next executable decision or implementation
    step unambiguous.

## 21. Result

The repository now has a coherent planning system rather than a mixture of
historical phases, product tracks, and cross-cutting specifications. The plans
show what exists, what has been approved, what remains undecided, which product
owns each capability, and the safe order in which the Liknon architecture can
be implemented.
