# Unified Validation System Implementation Plan

## 1. Purpose

This plan completes direct human operation and the first two official AI
workflows around Liknon:

1. **Human Flow**, in which a person works directly with the configuration,
   CLI, and reports;
2. **Official Action Workflow**, which routes one bounded task to focused
   operational skills such as AI-Tool under a human-owned AI decision policy;
3. **Official Validation-Engineering Workflow**, which selects explicit agent
   profiles per node and composes operational skills, knowledge, tools,
   decision policy, and immutable analytical artifacts.

Human operation is the operational foundation. The action workflow is an
optional policy-governed point-in-time assistance layer over it. The
validation-engineering workflow adds governed continuity and engineering
processes while reusing ordinary skills and the same decision-policy contract.
Neither workflow replaces or extends the CLI execution contract.

The three CLIs, schemas, resource contracts, and public documentation remain
model-neutral interfaces. In process descriptions below, unqualified
**Validation CLI** means `liknon` or `likx`; **Manager** means
`liknon-manager` or `likm`; and **Tool** means `liknon-tool` or `likt`.
Humans and external agents may consume them directly without loading an
installed AI resource. Direct agent use does not claim the policy, routing,
persistence, or safety guarantees of a compatible workflow. AI-Tool is one
official skill consumed by the action workflow, the validation-engineering
workflow, or another compatible workflow.

Every governed AI operation begins with the program-owned
[AI Bootstrap And Conformance Contract](ai-bootstrap-and-conformance-contract.md).
Its canonical `contracts/ai/ENTRYPOINT.md` is the first Liknon instruction the
human supplies to an external AI host. It defines loading order, authority,
policy, configuration, resource, application-binding, CLI-use, and evidence
rules without executing a workflow or selecting a model. Tool exposes the exact
contract through `likt bootstrap`; an AI that does not load and follow it may
still use the public CLIs directly but cannot claim governed Liknon conformance.

Within this plan, an **AI operation** means an operation performed under a
workflow or bounded skill contract unless a section explicitly discusses a
direct external agent. The human-only ownership of `.liknon/ai/policy.json`
remains a public boundary for every caller. Category-based enforcement is a
contract required of every actionable workflow represented as governed by
Liknon, never an application-specific branch. Incompatible workflow content may
be installed and inspected, but a conforming AI host stops before activating it.

This plan uses **phase** only for the eight ordered implementation units. The
official validation-engineering workflow currently calls four of its nodes
**processes**. That vocabulary and graph belong to the resource; other
workflows may use arbitrary node IDs, terminology, ordering, and topology.

## 2. Architectural Model

### 2.1 Human Flow

```text
human authors or reviews config.json
                  |
                  v
liknon inspects and validates configuration
                  |
                  v
liknon executes the selected group, suite, or check
                  |
                  v
human consumes the human report or JSON report
```

A human can complete this flow without an AI agent, an installed AI resource,
`.liknon/ai/policy.json`, or any file below `.liknon/ai/persistence/`.

### 2.2 Official Action Workflow

```text
human requests one bounded validation task
                  |
                  v
human directs the agent to the Liknon AI bootstrap entry point
                  |
                  v
agent loads and verifies the Liknon AI bootstrap contract
                  |
                  v
agent loads the official action workflow
                  |
                  v
workflow selects the liknon-ai-tool skill operation
                  |
                  v
agent loads the immutable policy boundary and validates policy.json
                  |
                  +-- missing or invalid --> policy guidance; human acts manually
                  |
                  v valid
agent loads only the required AI-Tool operation
                  |
                  v
agent uses likt to retrieve only the selected installed resources
                  |
                  v
agent guides policy, inspects, configures, runs, triages, or audits as requested
                  |
                  v
liknon executes the trusted config.json when execution is needed
                  |
                  v
agent interprets the canonical report and returns the bounded result
```

The official AI-Tool skill provides `policy`, `config`, `run`, `triage`, and
`audit` operations selected by the action workflow. The workflow validates and
obeys `.liknon/ai/policy.json` before every
operation except the narrowly read-only policy-guidance path used when the
document is missing or invalid. It never creates or mutates policy, writes
`.liknon/ai/persistence/`, applies Processes 0 through 3, or implies continuity
between separate requests. It may consume structured CLI output for the current
task, but it does not automatically establish the persistent report store or
local continuity used by the official validation-engineering workflow.

AI-Tool is a reusable agent-facing skill rather than an exclusive gateway
to the executable. A human may invoke it through an ordinary task prompt, and
another agent or orchestrated flow may consume the same operation contracts.
Public CLI documentation remains sufficient for a direct caller that does not
load AI-Tool.

### 2.3 Official Validation-Engineering Workflow

```text
human requests one validation-engineering node
                  |
                  v
human directs the agent to the Liknon AI bootstrap entry point
                  |
                  v
agent loads and verifies the Liknon AI bootstrap contract
                  |
                  v
agent loads the liknon-validation-engineering workflow
                  |
                  v
workflow resolves the profile, skills, tools, and knowledge declared by its entry node
                  |
                  v
agent verifies application and installed-resource compatibility
                  |
                  v
agent validates the existing human-owned .liknon/ai/policy.json through the CLI
                  |
                  +-- missing or invalid --> read-only policy guidance; stop;
                  |                         human acts, reviews, and invokes again
                  |
                  v valid
agent loads required AI-Tool operations
                  |
                  v
agent uses likt to retrieve only relevant installed resources
                  |
                  v
agent validates every explicitly supplied artifact and report
                  |
                  v
agent persists the completed immutable process result
                  |
                  v
agent proposes a config.json change when the evidence requires one
                  |
                  v
agent classifies the proposed effects under policy.json
                  |
                  v
policy returns automatic action, a free-text human decision, or stop
                  |
                  v
liknon executes the trusted config.json
                  |
                  v
agent evaluates whether the exact canonical JSON report may be persisted
                  |
                  v
agent stores and explicitly passes permitted or human-authorized report bytes
                  |
                  v
agent applies the same decision policy to remediation and recommendations
```

The validation-engineering workflow uses the corresponding AI-Tool operation whenever `policy`,
`config`, `run`, `triage`, or `audit` covers the required work. When a required
CLI capability has no AI-Tool operation, the workflow may use that capability
directly from canonical public reference documentation under the same policy,
trust, workspace, and evidence boundaries. This exception must not duplicate,
bypass, or locally reinterpret an operation that AI-Tool already defines.

The agent creates one immutable analytical artifact as the normal output of
each invoked artifact-producing node. This does not require a separate human
approval.

Capturing the exact JSON report occurs only when
`validationReportPersistence`, `sensitiveDataPersistence` when applicable, and
every category implicated by the destination resolve to `auto` or receive exact
human authority. Consuming processes receive its exact path and digest
explicitly. A report status never grants storage authority.

The human exclusively owns and mutates the AI decision policy and remains the
final authority over product intent and risk acceptance. Every governed effect
in either official workflow follows `.liknon/ai/policy.json`: it runs automatically only
at the configured category level or when the current human instruction supplies
the exact authority required by a `human` outcome. No authority, approval,
category level, or continuation mode permits an agent to create, alter, move,
restore, or delete the policy itself.

Every governed workflow requires an existing valid policy, except that the
read-only `liknon-ai-tool/policy` operation remains available to explain how the human can create
or correct it. When policy is absent, the agent stops the requested operation and
directs the human to run `liknon init --policy` personally, review
every field, and then invoke or resume the operation.

### 2.4 Shared Boundary

The system uses distinct runtime, governance, evidence, and content contracts:

| Contract | Owner | Purpose |
| --- | --- | --- |
| `.liknon/validation/config.json` | Consumer workspace | Sole executable validation policy |
| `.liknon/ai/policy.json` | Human operator in the consumer workspace | Governs compatible AI workflows through global values and optional exact-workflow overrides without affecting validator planning |
| `contracts/ai/ENTRYPOINT.md` | This repository and the shared program contract | Canonical first instruction and conformance boundary for governed external AI use |
| `liknon-core` | This repository | Deterministic configuration, planning, execution, and structured outcomes |
| `liknon` / `likx` | `apps/liknon-cli` | Validation terminal and process adapter over the core |
| `liknon-manager` / `likm` | `apps/liknon-manager` | Hub access and explicit mutation of installed resource state |
| `liknon-tool` / `likt` | `apps/liknon-tool` | Offline discovery and consumption of installed resources |
| `.liknon/resources/manifest.json` and `lock.json` | Consumer workspace and Manager | Desired and exact installed resource state |
| Liknon Resource Hub | Project-governed resource service | Authenticated metadata and immutable resource payloads |
| Versioned validation report | `liknon-core` | Canonical evidence from one execution, whether consumed directly or stored by exact digest |

Shared validation knowledge is authored independently from every flow and
published as selectable resources. Manager obtains the resources explicitly;
Tool retrieves only installed IDs relevant to the current decision. Skills,
workflows, agent profiles, tool descriptors, prompts, guides, templates, and
knowledge retain distinct resource kinds and are never activated merely because
Manager installed them or Tool returned their content.

`.liknon/ai/persistence/` contains only artifacts declared by invoked workflows. The normal
validation planner and executor never discover, read, interpret, or execute
files from that directory. Dedicated persistence commands validate or store
only an exact path supplied by the caller. `.liknon/validation/reports/` is a local
evidence store for exact JSON reports. A consuming process receives the exact
report path from its caller or the preceding direct handoff.

Workflow continuity is explicit: each node receives exact artifact and
report paths, validates their digests and relationships, and records those
references inside its own immutable result.

`.liknon/ai/policy.json` is the human-owned decision policy obeyed by compatible
AI workflows. It is not executable validation configuration. Its complete normative
semantics and immutable human-only mutation boundary are specified by
[AI Decision Policy](ai-decision-policy.md).

Every decision category and `processContinuation` has one global value plus an
optional map keyed by canonical workflow resource ID. A matching workflow value
precedes the global value. When composed parent and child workflows both have an
override, the most restrictive matching value wins. Absolute prohibitions remain
above every global value and override. A resource can declare effects and
compatibility but can never grant itself an override.

Workflow, agent profile, skill, tool descriptor, knowledge, and prompt are
separate contracts. A workflow manifest is the orchestration index: it owns the
graph and references every profile, skill, tool operation, action, and supporting
resource by stable identity. An agent profile owns role and behavioral guidance,
not graph transitions or policy. A tool descriptor declares an operation
contract but does not install an application, execute a command, or grant
authority. Tool returns the exact locked graph and resource closure;
the consuming AI host binds profiles to available models and conducts the graph.
No Liknon CLI performs that orchestration.

The external host conducts the graph under the AI bootstrap contract. The
contract is part of the program rather than a replaceable Hub resource and has
no official-resource branch. AI-Tool remains the official default operational
skill declared by the initial workflows, while compatible workflows may select
another skill through the same resource contracts.

Policy and workflow-persistence commands operate inside one selected
workspace boundary. Omission of `--workspace` discovers the nearest ancestor
containing `.liknon/`; an explicit path selects that boundary directly. Git,
language package files, resource content, and agent directories never redefine
it. Initialization may create `.liknon/` at an explicit workspace or, when no
root exists, at the exact current directory. Equivalent internal core APIs
always receive the workspace root explicitly.

Human-only guidance is separated from shared knowledge and AI instructions. It
helps an operator select a flow, organize work contexts, and exercise policy
ownership, but it is never interpreted as an instruction to an agent.

| Property | Human operation | Official action workflow | Official validation-engineering workflow |
| --- | --- | --- | --- |
| Primary operator | Human | Agent directed for one bounded task | Agent directed through engineering processes |
| CLI and `config.json` | Direct use | Through AI-Tool operations | Through AI-Tool when covered; direct documented CLI use only for uncovered capabilities |
| Shared resources | Canonical sources or `likt` | Progressive `likt` retrieval by stable resource ID | Progressive `likt` retrieval by stable resource ID |
| `policy.json` | Not used by direct CLI operation | Required; read-only guidance may operate when missing or invalid | Required and governed |
| `.liknon/ai/persistence/` | Not used | Not used | Immutable process artifacts |
| Persisted analytical context | Not used | Not used | Explicit immutable artifacts supplied by exact path and digest |
| `.liknon/validation/reports/` persistent store | Not established automatically | Not established automatically | Exact reports stored and linked only after the applicable persistence decision |
| Resource graph | Not used | Bounded action graph selecting AI-Tool operations | Four current process nodes with independently selected agent profiles |
| Cross-request continuity | Human-managed | None | Explicit validated persistence |

### 2.5 Official Validation-Engineering Processes

The current official validation-engineering workflow has four recurring
analytical process nodes around the normal runner:

| Process | Responsibility | Declared persisted result | Human authority |
| ---: | --- | --- | --- |
| 0 | Model domain, consequences, invariants, and uncertainty | Immutable domain artifact | Provides missing domain decisions when needed |
| 1 | Design evidence coverage and reconcile it with config IDs | Immutable sensorium artifact bound to exact domain and config digests | Owns the policy and responds when its effective result is `human` |
| 2 | Diagnose a concrete non-pass report and verify remediation | Incident artifact | Owns the policy and responds when its effective result is `human` |
| 3 | Compare passing evidence with repository change and recommend recalibration | Drift artifact | Owns product decisions and any policy-required continuation decision |

The runner is the deterministic bridge between Processes 1 and 2 or 3. It
remains the same CLI used by direct human operation and the action workflow and
is not an additional cognitive process.

Process persistence and process continuation are separate decisions.
Completing a process persists its bounded redacted result without a separate
prompt; protected content that remains follows `sensitiveDataPersistence`.
Every completion creates a new immutable UUIDv7 artifact. A correction creates
another artifact; it never overwrites an earlier result.
This workflow declares `0 -> 1` and `2 -> 3` as continuation-eligible edges.
They follow the workflow's effective ternary `processContinuation` value,
resolved from its exact-ID override when present and otherwise from the global value:
`auto` starts the valid related process, `human` suggests it and waits for a
free-text decision, and `stop` ends without suggesting or asking. Every next
process still validates its own prerequisites. Ending or postponing a handoff
leaves the persisted preceding result valid. These edge IDs are resource
content, not platform constants. Other workflows may declare unrelated graphs
and continuation-eligible edges.

The processes define analytical responsibilities, not chat boundaries. Human
recommendations for organizing those processes across one or more conversations
are specified separately in [AI Work Contexts](ai-work-contexts.md). The
recommended arrangement uses one setup context for Processes 0 and 1 and one
operational context for Processes 2 and 3, while a single context remains fully
supported.

## 3. Sources Of Truth

| Concern | Source of truth |
| --- | --- |
| Governed AI loading order, authority boundary, and conformance evidence | Program-owned `contracts/ai/ENTRYPOINT.md`, projected exactly by `likt bootstrap` |
| Executable programs, arguments, directories, dependencies, and composition | `.liknon/validation/config.json` |
| Compatible AI authority and workflow continuation | Human-owned `.liknon/ai/policy.json`, using global values and optional exact-workflow overrides interpreted exclusively by the normative AI decision-policy contract |
| Concrete validation outcome | Versioned JSON report emitted by the CLI and, when persistence is permitted for a compatible workflow run, stored byte-for-byte under `.liknon/validation/reports/` |
| CLI and configuration mechanics | Runtime contracts, schemas, and canonical reference documentation |
| Validation concepts and tool-selection evidence | `docs/validation/knowledge`, published resource metadata, and exact installed documents returned by `likt` |
| Desired and exact local resources | `.liknon/resources/manifest.json` and `.liknon/resources/lock.json` |
| Hub resource identity and trust | Authenticated Hub catalog metadata selected by `likm` |
| Human selection and governance of flows | Human-only operator resources under `docs/validation/flows/human/` |
| AI-Tool operation | Exact installed `liknon-ai-tool` skill resource returned by `likt`, authored canonically under `resources/skills/official/liknon-ai-tool/` |
| Official action routing | Exact installed action workflow returned by `likt`, authored under `resources/workflows/official/liknon-action/` and declaring its AI-Tool dependency |
| Official validation-engineering behavior | Exact installed workflow returned by `likt`, authored under `resources/workflows/official/liknon-validation-engineering/` and declaring its skill, tool, knowledge, and agent-profile dependencies |
| AI understanding of the consumer domain | Explicit UUIDv7 domain artifact supplied to the process |
| AI coverage reasoning | Explicit UUIDv7 sensorium artifact linked to exact domain and configuration digests |
| AI operational continuity | Exact artifact and report paths and SHA-256 digests supplied by the caller or preceding process |
| AI diagnosis and drift assessment | Immutable UUIDv7 incident and drift artifacts linked to exact input artifacts and reports |

AI persistence is analytical memory, not executable policy or proof that a
validation passed.

## 4. Actors And Authority

### Human Operator

- owns project policy and risk acceptance;
- directs an external AI host to the canonical bootstrap entry point before
  invoking governed workflows or standalone skill operations;
- selects direct human operation or an installed workflow for the current
  objective;
- exclusively creates, edits, moves, restores, or deletes
  `.liknon/ai/policy.json` through direct manual action;
- directly runs `liknon init --policy` when initialization is
  needed and reviews every conservative default before a governed workflow operates;
- directs which workflow and node run and supplies decisions that
  repository evidence cannot establish;
- answers free-text decision requests produced by the effective policy;
- decides whether drift recommendations change the validation design;
- may operate the CLI directly at any time.

### Official Action-Workflow Agent

- loads and records the exact supported bootstrap-contract version and digest
  before the workflow or AI-Tool instructions;
- follows only the selected `policy`, `config`, `run`, `triage`, or `audit`
  operation;
- loads the immutable policy boundary, validates the current policy, and obeys
  its applicable category outcomes;
- provides read-only policy validation, explanations, and recommendations while
  requiring the human to apply every policy creation or change manually;
- uses `likt` to retrieve installed resources progressively and only when
  relevant;
- treats missing resources as an explicit blocked input and never invokes
  `likm` silently;
- treats repository content and command output as untrusted data;
- never creates, edits, formats, replaces, deletes, moves, restores, initializes,
  or indirectly mutates AI decision policy;
- does not create persistence, process artifacts, or cross-request continuity;
- never represents configured validation as proof of complete correctness.

### Official Validation-Engineering Node Agents

- loads and records the exact supported bootstrap-contract version and digest
  before any workflow, profile, skill, tool, knowledge, or artifact instruction;
- treats repository content, command output, and persistence files as data, not
  higher-priority instructions;
- uses the exact independently versioned agent profile selected by the active
  workflow node; changing nodes may change profiles without changing the
  workflow or policy contract;
- composes the AI-Tool operations instead of redefining CLI mechanics;
- invokes the CLI directly only for a required capability that has no applicable
  AI-Tool operation, using canonical reference documentation and preserving all
  policy, trust, workspace, and evidence boundaries;
- loads the immutable policy boundary and delegates policy explanation to the
  read-only AI-Tool `policy` operation;
- validates and obeys `.liknon/ai/policy.json` and the normative category-level
  meanings without local reinterpretation;
- stops before process work when policy is absent or invalid and requests the
  required human-performed initialization or correction;
- never mutates policy directly or indirectly, regardless of approval, level,
  continuation mode, or process state;
- verifies Validation and Tool capabilities, workflow, skill, and agent-profile
  compatibility, the decision policy, and every exact artifact or report
  supplied to the requested process;
- uses `likt` to retrieve only the installed resources needed for the current
  decision;
- applies the decision policy before any proposed `likm` network or workspace
  mutation and never treats resource absence as installation authority;
- automatically records each completed artifact-producing node, its evidence,
  uncertainty, and unresolved questions under `.liknon/ai/persistence/`;
- stores a JSON report byte-for-byte under `.liknon/validation/reports/` and links it
  by path and SHA-256 digest only when all applicable persistence decisions
  permit that exact write;
- persists every completed UUIDv7 process result as a separate immutable file;
- classifies every governed effect, combines overlapping categories
  restrictively, and records the exact policy digest used for the decision;
- never represents configured validation as proof of complete correctness.

### Liknon Validation CLI

- validates one complete declarative configuration;
- provisions explicitly requested consumer-owned configuration and policy files
  through the deterministic, no-overwrite `init` command;
- executes explicit program and argument vectors without an implicit shell;
- produces deterministic human and JSON reports;
- detects configured repository mutations;
- owns deterministic single-file verification and storage operations for
  explicitly supplied AI artifacts and reports; artifact payload validation
  requires one exact caller-supplied workflow contract verified against the
  resource lock;
- remains independent from AI orchestration and consumer-specific policy.

### Liknon Manager

- is the sole Liknon CLI that accesses the Hub or mutates installed resources;
- exposes complete plans before installation, update, removal, or
  materialization;
- records exact desired and locked resource state;
- verifies provenance, compatibility, inventory, and payload hashes;
- never executes, interprets, or activates resource instructions.

### Liknon Tool

- projects the exact program-owned bootstrap contract without requiring a
  workspace or installed resource store;
- reads and verifies only installed local resource state;
- exposes progressive catalog, search, lookup, resolution, and context output;
- operates offline and performs no resource or workspace mutation;
- presents resource trust as provenance, not instruction authority;
- never executes returned content or invokes another Liknon executable.

## 5. Cross-Cutting Invariants

1. `.liknon/validation/config.json` is the only executable validation policy.
2. AI artifacts never become an alternate command source for the runner.
3. When a compatible AI workflow proposes executable coverage, it resolves that proposal
   to config tool, check, suite, and group identifiers before execution.
4. Validation CLI does not depend on an AI provider, model, chat protocol, or
   persistence directory.
5. Human operation remains complete when AI resources are absent.
6. Every governed AI operation except contract-valid read-only policy guidance
   requires a valid human-owned AI decision policy before workspace work begins.
7. Every compatible workflow loads the same immutable policy boundary,
   validates the current policy, and applies the same category and override
   meanings.
8. The official action workflow never discovers validation-engineering
   persistence and never creates implicit continuity between requests.
9. Every persisting workflow node verifies application capabilities and the
   exact workflow, profile, skill, and tool resource compatibility, validates
   the policy, and validates every explicit artifact or report input.
10. Invoking a workflow node authorizes that node to create one
   bounded redacted immutable result without another approval prompt. Protected
   content that remains is governed by `sensitiveDataPersistence`.
11. Exact report storage is governed by `validationReportPersistence`,
    `sensitiveDataPersistence` when applicable, and by every category
    implicated by the destination. Report status never grants authority.
12. Every other governed effect is classified under the current AI decision
    policy before execution.
13. Decision levels use the mandatory restrictiveness order
    `4 > 3 > 2 > 1 > 0`; independently computed outcomes combine as
    `stop > human > auto`.
14. The JSON report is interpreted by schema and status, not by terminal prose
   or a binary success/failure assumption.
15. Every completed persisting workflow node produces a new immutable UUIDv7
    artifact linked to the exact workflow, node, transition, performed actions,
    agent profile, resource lock, parent paths, parent digests, config digest,
    and report evidence.
16. Passing checks establish only the evidence represented by the selected
   configuration.
17. Safety information, uncertainty, and policy-required human decisions
   override compact chat formatting.
18. Documentation, skill instructions, schemas, examples, and code comments
   are written in English.
19. Chat identity and chat history are never execution inputs or persistence
   references. A resumed process receives its artifact paths explicitly.
20. Conversational organization is human-facing quality guidance and never
   changes process semantics, authorization, executable policy, or reports.
21. Workflow continuation follows only the effective `processContinuation`
    value: `auto`, `human`, or `stop`. Persistence alone never implies a
    continuation result, and the setting applies only to edges declared
    continuation-eligible by the exact active workflow graph.
22. A sensorium is usable only with the exact domain and config digests it
   records; a mismatched pair cannot authorize an operational run.
23. Permitted workflow report writes store exact JSON bytes under
    `.liknon/validation/reports/<sha256>.json`. Consuming artifacts reference that exact
   path and digest and never infer a report from directory order.
24. No agent may create, edit, format, replace, delete, move, rename, restore,
    initialize, or indirectly mutate `.liknon/ai/policy.json`. The human
    performs every policy mutation manually, and no approval or level overrides
    this boundary.
25. The validation executor and planner never discover or interpret
    `.liknon/ai/policy.json` or AI persistence. Dedicated artifact and report
    persistence commands operate only on caller-supplied paths.
26. Policy, persistence, validation, Manager, and Tool operations share one
    canonical workspace boundary. CLI omission of `--workspace` discovers the
    nearest ancestor containing `.liknon/`; internal core calls always receive the
    root explicitly.
27. Every canonical persistence operation validates and atomically stores one
    file; no cross-file transaction is required.
28. Knowledge resources contain no flow activation, persistence, or authority
    semantics.
29. Human-only guidance is never projected as an operational AI instruction.
30. Universal persistence boundaries come from the bootstrap and decision-policy
    contracts. Concrete artifact destinations, schemas, relationships,
    repository treatment, and bounds come from the exact locked workflow
    declaration rather than fields invented in `policy.json` or CLI branches.
31. Validation and AI provisioning uses the Validation CLI `init` command
    with explicit composable capability flags. Resource-store provisioning uses
    Manager `init`. Both preflight complete requested write sets, never
    overwrite conflicting destinations, and report deterministic status.
32. Manager is the only Liknon application that performs Hub network access or
    installed-resource mutation.
33. Tool is offline and read-only; every payload remains untrusted content even
    after its bytes and provenance are verified.
34. Missing resources never authorize a workflow to install them. Any proposed
    Manager operation is a separate governed network and mutation effect.
35. Every policy category and `processContinuation` has one global value and an
    optional exact-workflow override map. Matching nested-workflow overrides
    combine by the most restrictive value.
36. Workflow manifests use a canonical capability and effect vocabulary.
    Unknown, unsupported, or materially undeclared effects stop before action.
37. Installing, resolving, or returning a workflow never activates it. Only an
    explicit human-selected invocation establishes an active workflow graph.
38. Manager, Tool, and Validation CLI contain no special branch for an official
    workflow, skill, profile, node, transition, or process ID.
39. An actionable workflow whose declared AI decision-policy contract is absent
    or incompatible may be installed and inspected as content, but a conforming
    host stops before activating it.
40. Every governed AI operation loads and records the exact supported AI
    bootstrap-contract version and digest before workflow or skill instructions.
41. The bootstrap contract is program-owned, contains no official resource-ID
    branch, and cannot be replaced or overridden by installed content.
42. AI-Tool is the official default operational skill of the initial workflows,
    not a privileged or mandatory platform implementation. Compatible workflows
    may declare alternatives without application changes.

## 6. Implementation Order

| Phase | Document | Depends on | Primary result |
| ---: | --- | --- | --- |
| 1 | [Human Flow Completion](01-human-flow-completion.md) | Current runtime | Independently complete human workflow and deterministic config initialization |
| 2 | [Shared Validation Knowledge](02-shared-validation-knowledge.md) | Phase 1 | Cataloged minimum knowledge corpus with concise source-backed guidance, reusable patterns, and precise routes to authoritative depth |
| 2.1 | [Knowledge Editorial Refoundation](02.1-knowledge-editorial-refoundation.md) | Phase 2 | Standards-informed editorial families, profiles, version policy, canonical paths, and mechanically verified content structure |
| 3 | [Official Action Workflow And AI Decision Policy](03-ai-tool-flow.md) | Phases 1, 2, and 2.1; A01.4; B02.5; B03.4 | Generic human-owned policy foundation, official action workflow, and reusable AI-Tool operations without analytical continuity |
| 4 | [Validation-Engineering Workflow Persistence And Safety](04-ai-engineering-persistence-and-safety-foundation.md) | Phases 1-3 | Generic workflow artifact envelope, process trust boundaries, official validation-engineering graph, and independently versioned profiles |
| 5 | [Official Validation-Engineering Domain And Coverage Setup](05-ai-engineering-domain-and-coverage-setup.md) | Phase 4 | Automatically persisted domain and sensorium nodes of the official workflow |
| 6 | [Official Validation-Engineering Triage And Drift Loop](06-ai-engineering-triage-and-drift-loop.md) | Phase 5 | Safe daily execution, remediation, and recalibration through the official workflow |
| 7 | [End-To-End Verification And Release](07-end-to-end-verification-and-release.md) | Phases 1-6 | Proven isolation, composition, four-family agent evaluation, and distributable artifacts |

Complete phases in order. After Phase 2.1, complete A01, B02, and B03 before
Phase 3 so later AI operations retrieve installed resources through Tool rather
than embedding knowledge or instructions in Validation CLI. A01 establishes
the product boundary, B02 establishes the resource contract and Manager, and
B03 completes offline Tool interoperability. Official resources may then evolve
through Hub publication without rebuilding any CLI.

A01 also establishes the shared AI bootstrap identity, compatibility types, and
canonical entry source before B02 and B03 consume them. B02 resolves declared
compatibility, and B03 exposes the exact built-in entry contract. Phase 3 then
implements the complete decision-policy lifecycle and the first official
workflow against those already established generic contracts.

[AI Work Contexts](ai-work-contexts.md) is cross-cutting human-only operator
guidance for the official validation-engineering workflow rather than an implementation phase. Its
public guide is completed with the system documentation and does not alter the
dependency order above.

[AI Decision Policy](ai-decision-policy.md) is a cross-cutting normative
contract implemented in Phase 3. It defines shared AI authority, global and
workflow-specific settings, continuation,
the immutable human-only policy-mutation boundary, action
classification, all category-level meanings, restrictive combination, and the
generic persistence decisions and absolute boundaries every compatible AI
operation obeys. Workflow-specific artifact layouts remain resource contracts.

[AI Bootstrap And Conformance Contract](ai-bootstrap-and-conformance-contract.md)
is the cross-cutting root contract implemented across A01 and B03. It defines
how an external host enters the governed system, while policy, workflows,
skills, profiles, tools, and knowledge retain their narrower contracts.

[Agent Evaluation Program](agent-evaluation-program.md) is the cross-cutting
evaluation contract executed in Phase 7. It first measures CLI-only discovery,
then the incremental value of public documentation, then the incremental value
of the official action workflow and AI-Tool skill, and finally the complete
validation-engineering workflow so failures can be attributed to the correct
evaluation layer.

### Replacement Semantics

The current code and maintained contracts remain authoritative until their
owning implementation phase changes them. Within this plan, the unified target
model and tracks A01 through B03 define the intended replacement. Earlier base
documents may therefore describe the current pre-separation implementation;
that temporary difference is an implementation input, not a requirement to
preserve both models.

A phase is incomplete until it updates or removes every maintained document,
schema, test, fixture, and resource assertion superseded by its target contract.
No compatibility layer, fallback path, duplicated policy model, or stale
official-workflow assumption survives merely because it appeared in the
pre-implementation state.

### Architecture And Distribution Tracks

Repository separation and distribution are planned separately from the
numbered validation-system phases:

| Track | Document | Relationship | Primary result |
| --- | --- | --- | --- |
| A01 | [Product And Repository Separation](A01-product-and-repository-separation/README.md) | Runs after Phase 2.1 and before Phase 3 or publication implementation | Virtual workspace, terminal-independent contracts, three independent CLIs, and modular resource sources |
| B01 | [Liknon Validation CLI Distribution](B01-cli-distribution/README.md) | Requires A01.4; does not imply Phase 7 completion | `liknon-cli` package with `liknon` and `likx` through GitHub source, crates.io source, and Cloudflare native artifacts |
| B02 | [Liknon Resource Hub And Manager](B02-resource-hub-and-manager/README.md) | Requires A01.4 and approved resource sources | Authenticated Hub plus `liknon-manager` and `likm` |
| B03 | [Liknon Tool CLI](B03-tool-cli/README.md) | Requires A01.4 and B02.0; final interoperability requires B02.5 | Offline `liknon-tool` and `likt` |
| B04 | [Liknon Programmatic API And Language Bindings](B04-programmatic-api-and-language-bindings/README.md) | Future API track; requires A01 and approved B04.0 | Justified SDKs and in-process APIs without copied CLIs or resource payloads |

B01, Manager, and Tool have independent releases. Hub resources release
without rebuilding any CLI. B04 remains a future API track until a real
programmatic contract and consumer are approved. Defining a track does not
itself change configuration, report, policy, or AI-flow contracts.

## 7. Planned Documentation, Knowledge, And AI Resource Layout

CLI references and maintainer contracts remain repository documentation.
Knowledge and AI instructions have separate canonical owners:

```text
contracts/
└── ai/
    └── ENTRYPOINT.md                  # Program-owned governed-AI root contract
docs/validation/
├── README.md
├── authoring/                       # Maintainer-only editorial contracts
│   ├── README.md
│   ├── editorial-standard.md
│   ├── editorial-bases.md
│   ├── editorial-profiles.json
│   └── profiles/
├── reference/                       # CLI, config, report, schema, and AI policy contracts
├── knowledge/                       # Canonical authored knowledge resources
└── flows/
    ├── human/                       # Human Flow contract and operation
    ├── action/                      # Official bounded action workflow
    └── validation-engineering/      # Official process graph, persistence, and trust
resources/
├── skills/
│   └── official/
│       └── liknon-ai-tool/                    # Bounded operations
├── workflows/
│   └── official/
│       ├── liknon-action/                     # Routes bounded work to AI-Tool
│       └── liknon-validation-engineering/     # Declares the process graph
├── agent-profiles/
│   └── official/                              # Independently reusable personas
├── tools/
│   └── official/                              # Declarative application capabilities
├── prompts/
│   └── official/
├── guides/
│   └── official/
│       └── liknon-ai-policy-boundary/         # Generated immutable boundary
├── templates/
│   └── official/
└── collections/
    └── official/
```

Every policy-compatible AI route loads the shared policy-boundary guide. The
official workflows route to AI-Tool operations when they need policy guidance,
inspection, configuration, execution, triage, or audit. They use canonical
public CLI references directly only when no declared skill covers the required
capability, and they do not duplicate skill instructions or agent profiles.
Manager obtains explicit selections; workflows discover and retrieve only
installed resources through Tool. Human-only resources are never routed as
agent instructions.

## 8. Planned Consumer Layout

The completed system supports this consumer-owned layout:

```text
.liknon/
├── validation/
│   ├── config.json                   # Sole executable validation policy
│   └── reports/
│       └── <sha256>.json             # Exact persisted CLI report bytes
├── resources/
│   ├── manifest.json                 # Requested resource identities and constraints
│   ├── lock.json                     # Exact versions, origins, dependencies, and hashes
│   ├── knowledge/<namespace>/<name>/<version>/
│   ├── skills/<namespace>/<name>/<version>/
│   ├── workflows/<namespace>/<name>/<version>/
│   ├── agent-profiles/<namespace>/<name>/<version>/
│   ├── tools/<namespace>/<name>/<version>/
│   ├── prompts/<namespace>/<name>/<version>/
│   ├── guides/<namespace>/<name>/<version>/
│   ├── templates/<namespace>/<name>/<version>/
│   └── collections/<namespace>/<name>/<version>/
└── ai/
    ├── policy.json                   # Human-owned global and per-workflow authority
    └── persistence/
        └── <namespace>/<workflow-name>/
            └── <artifact-kind>/
                └── <uuidv7>.json     # Immutable workflow-declared artifact
```

The precise artifact and report-storage contracts are established in Phase 4.
Policy is not required by direct human operation but is required by governed
workflows except for read-only policy guidance. Reports and persistence are not
required by the Human Flow, the official action workflow, or standalone skill
use. Reports, incidents, and drift artifacts are ignored by the consumer
repository in the initial official workflow. Its domain and sensorium artifacts
use `consumer-selected` repository treatment. Other workflows declare their own
artifact kinds and one supported treatment for each kind; neither name nor
treatment is inferred by a CLI.

Configuration, report, runtime, and generic artifact-envelope schemas belong to
core or Validation CLI. Workflow-specific artifact payload schemas and storage
declarations belong to their locked workflow resources.
Resource manifests, locks, and Hub contracts belong to B02 and the shared
workspace contract. Tool owns only local consumption semantics. Consumer
workspaces store configuration, policy, reports, explicitly selected resources,
locks, and workflow artifacts, not manually copied private schemas.

## 9. Completion Definition

The unified plan is complete only when:

- a human can configure, inspect, execute, and diagnose validation without AI;
- an external agent can discover representative CLI workflows from only the
  executable and its in-band output;
- an external agent can perform representative direct CLI tasks from public
  documentation alone without receiving action-workflow, AI-Tool, or
  validation-engineering instructions;
- an agent can complete the official action workflow through policy-governed
  AI-Tool operations without creating policy or analytical persistence;
- a human can direct an external AI to the canonical bootstrap entry point, and
  every governed trial records its exact supported version and digest before
  loading a workflow or standalone skill;
- the official validation-engineering workflow composes skills, profiles,
  tools, and knowledge through explicit process nodes instead of redefining
  them;
- an external workflow with unrelated node IDs, graph shape, profiles, and
  transitions operates through the same Hub, Manager, Tool, and policy
  contracts without application changes;
- an actionable workflow with an absent or incompatible policy-contract version
  can be installed and inspected but cannot be activated by a conforming host;
- canonical shared knowledge supports humans and compatible AI workflows without
  separately authored rewrites;
- human-only operator guidance is structurally separate and never routed as an
  AI instruction;
- AI artifacts are versioned, linked, non-executable, and resumable through
  explicit paths and digests;
- every completed process result is a new immutable UUIDv7 document;
- every artifact relationship is explicit and verified without directory
  scanning or newest-file selection;
- a sensorium is operationally usable only with the exact domain and config
  digests it records;
- the AI decision policy defines every continuation mode and every numeric
  category level through a global value plus optional exact-workflow overrides,
  without process-specific reinterpretation;
- the shared `init` command provisions config and human-owned AI policy only
  through explicit capability flags and never overwrites conflicting resources;
- `policy validate` is read-only, missing or invalid policy blocks every AI
  operation except advisory policy guidance, and the human directly performs
  every policy initialization and change;
- no compatible AI workflow can create, edit, format, replace, delete, move, restore,
  initialize, or indirectly mutate policy under any approval or configured
  level;
- invoked persisting workflow nodes save their declared results without a separate approval
  prompt, except that protected content remains subject to
  `sensitiveDataPersistence`;
- canonical reports are persisted only after
  `validationReportPersistence`, `sensitiveDataPersistence` when applicable,
  and all destination-related categories permit the exact write;
- humans can use separate setup and operational conversations or one unified
  conversation without changing system behavior;
- no schema, artifact, skill or workflow reference, or runtime operation depends
  on a chat identifier or previous chat history;
- every AI-triggered run still passes through the normal configuration and
  report contracts;
- authorization boundaries are exercised by tests and realistic skill and
  workflow trials;
- CLI-only discovery, documentation-only direct CLI use, the official action
  workflow, and the validation-engineering workflow are evaluated independently across the declared model matrix
  with reproducible scenarios and observable evidence;
- all outcome classes, mismatched-artifact cases, and drift cases have
  deterministic fixtures;
- the CLI install target and native artifacts contain only runtime, core
  schemas, CLI assets, examples, and documentation declared by B01, while the
  GitHub source channel still exposes the complete monorepo;
- Manager can obtain selected knowledge and operational resources without
  executing or activating them;
- Tool consumes exact installed resources offline and embeds no corpus;
- Tool exposes the program-owned AI bootstrap contract without treating it as
  an installed or replaceable resource;
- Tool returns workflow graphs and profile dependencies without executing or
  interpreting them, and no CLI contains an official resource-ID branch;
- Validation CLI, Manager, and Tool remain separately installable and the site
  offers each one independently;
- programmatic language packages, when B04 is implemented, expose justified
  typed APIs rather than resource copies, launchers, or copied CLI executables.
