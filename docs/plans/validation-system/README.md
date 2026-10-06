# Unified Validation System Implementation Plan

## 1. Purpose

This plan completes three supported ways to use `liknon`:

1. **Human Flow**, in which a person works directly with the configuration,
   CLI, and reports;
2. **AI-Tool Flow**, in which an agent uses focused operational skills and
   shared knowledge under a human-owned AI decision policy for one requested
   task without persistent analytical processes;
3. **AI-Engineering Flow**, in which an agent composes the AI-Tool capabilities
   with explicit validation-engineering processes, decision policy, and
   immutable analytical artifacts.

The Human Flow is the operational foundation. The AI-Tool Flow is an optional,
policy-governed point-in-time assistance layer over that foundation. The
AI-Engineering Flow adds governed continuity and engineering processes while
reusing the same AI-Tool operations and decision policy. Neither AI-Tool nor
AI-Engineering replaces or extends the CLI execution contract.

The CLI, schemas, and public documentation remain model-neutral interfaces.
Humans and external agents may consume them directly without loading the
distributed skill. Direct agent use is not a fourth governed flow and does not
claim the policy, routing, persistence, or safety guarantees of AI-Tool or
AI-Engineering. AI-Tool provides an optional operational interface for a human
working through an agent, another compatible agent system, or AI-Engineering.

Within this plan, an **AI operation** means an operation inside AI-Tool or an
AI-Engineering process unless a section explicitly discusses a direct external
agent. The human-only ownership of `.validation/policy.json` remains a public
boundary for every caller, while category-based agent enforcement is a contract
of the two bundled AI flows.

This plan uses **phase** only for the eight ordered implementation units and
**process** for the four recurring AI-Engineering operations. Processes can be
invoked again whenever their preconditions hold; they are not release phases or
mandatory steps in one linear lifecycle.

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

A human can complete this flow without an AI agent, an installed skill,
`.validation/policy.json`, or any file below `.validation/persistence/`.

### 2.2 AI-Tool Flow

```text
human requests one bounded validation task
                  |
                  v
agent loads the liknon skill router
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
agent retrieves selected shared knowledge through the CLI
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

The AI-Tool Flow provides `policy`, `config`, `run`, `triage`, and `audit`
operations. It validates and obeys `.validation/policy.json` before every
operation except the narrowly read-only policy-guidance path used when the
document is missing or invalid. It never creates or mutates policy, writes
`.validation/persistence/`, applies Processes 0 through 3, or implies continuity
between separate requests. It may consume structured CLI output for the current
task, but it does not automatically establish the persistent report store or
local continuity used by the AI-Engineering Flow.

AI-Tool is a reusable agent-facing interface rather than an exclusive gateway
to the executable. A human may invoke it through an ordinary task prompt, and
another agent or orchestrated flow may consume the same operation contracts.
Public CLI documentation remains sufficient for a direct caller that does not
load AI-Tool.

### 2.3 AI-Engineering Flow

```text
human invokes and directs the agent
                  |
                  v
agent loads the liknon skill and immutable policy boundary
                  |
                  v
agent verifies CLI and skill compatibility
                  |
                  v
agent validates the existing human-owned .validation/policy.json through the CLI
                  |
                  +-- missing or invalid --> read-only policy guidance; stop;
                  |                         human acts, reviews, and invokes again
                  |
                  v valid
agent loads required AI-Tool operations
                  |
                  v
agent retrieves only relevant shared knowledge through the CLI
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

AI-Engineering uses the corresponding AI-Tool operation whenever `policy`,
`config`, `run`, `triage`, or `audit` covers the required work. When a required
CLI capability has no AI-Tool operation, AI-Engineering may use that capability
directly from canonical public reference documentation under the same policy,
trust, workspace, and evidence boundaries. This exception must not duplicate,
bypass, or locally reinterpret an operation that AI-Tool already defines.

The agent creates one immutable analytical artifact as the normal output of
each invoked AI-Engineering process. This does not require a separate human
approval.

Capturing the exact JSON report occurs only when
`validationReportPersistence`, `sensitiveDataPersistence` when applicable, and
every category implicated by the destination resolve to `auto` or receive exact
human authority. Consuming processes receive its exact path and digest
explicitly. A report status never grants storage authority.

The human exclusively owns and mutates the AI decision policy and remains the
final authority over product intent and risk acceptance. Every governed effect
in either AI flow follows `.validation/policy.json`: it runs automatically only
at the configured category level or when the current human instruction supplies
the exact authority required by a `human` outcome. No authority, approval,
category level, or continuation mode permits an agent to create, alter, move,
restore, or delete the policy itself.

Both AI flows require an existing valid policy, except that read-only
`AI-Tool/policy` guidance remains available to explain how the human can create
or correct it. When policy is absent, the agent stops the requested operation and
directs the human to run `liknon init --policy` personally, review
every field, and then invoke or resume the operation.

### 2.4 Shared Boundary

The system uses four distinct contracts:

| Contract | Owner | Purpose |
| --- | --- | --- |
| `.validation/config.json` | Consumer workspace | Sole executable validation policy |
| `.validation/policy.json` | Human operator in the consumer workspace | Governs AI-Tool and AI-Engineering authority without affecting validator planning |
| `liknon` CLI/library | This crate | Deterministic inspection and execution |
| Versioned validation report | This crate | Canonical evidence from one execution, whether consumed directly or stored by exact digest |

Shared validation knowledge is authored independently from every flow. Humans
read it directly, and both AI flows load only the portions relevant to their
current decision. Operational skill instructions do not become validation
knowledge, and validation knowledge does not activate an AI process.

`.validation/persistence/` belongs only to the AI-Engineering Flow. The normal
validation planner and executor never discover, read, interpret, or execute
files from that directory. Dedicated persistence commands validate or store
only an exact path supplied by the caller. `.validation/reports/` is a local
evidence store for exact JSON reports. A consuming process receives the exact
report path from its caller or the preceding direct handoff.

AI-Engineering continuity is explicit: each process receives exact artifact and
report paths, validates their digests and relationships, and records those
references inside its own immutable result.

`.validation/policy.json` is the human-owned decision policy obeyed by both AI
flows. It is not executable validation configuration. Its complete normative
semantics and immutable human-only mutation boundary are specified by
[AI Decision Policy](ai-decision-policy.md).

Policy and AI-Engineering persistence commands operate inside one selected
workspace boundary. The CLI accepts `--workspace` and defaults its omission to
the exact current working directory, without parent, Git, or configuration-based
root discovery. The equivalent library APIs always require the workspace root
explicitly.

Human-only guidance is separated from shared knowledge and AI instructions. It
helps an operator select a flow, organize work contexts, and exercise policy
ownership, but it is never interpreted as an instruction to an agent.

| Property | Human Flow | AI-Tool Flow | AI-Engineering Flow |
| --- | --- | --- | --- |
| Primary operator | Human | Agent directed for one bounded task | Agent directed through engineering processes |
| CLI and `config.json` | Direct use | Through AI-Tool operations | Through AI-Tool when covered; direct documented CLI use only for uncovered capabilities |
| Shared knowledge | Canonical source or installed CLI | Progressive CLI retrieval by stable document ID | Progressive CLI retrieval by stable document ID |
| `policy.json` | Not used by direct CLI operation | Required; read-only guidance may operate when missing or invalid | Required and governed |
| `.validation/persistence/` | Not used | Not used | Immutable process artifacts |
| Persisted analytical context | Not used | Not used | Explicit immutable artifacts supplied by exact path and digest |
| `.validation/reports/` persistent store | Not established automatically | Not established automatically | Exact reports stored and linked only after the applicable persistence decision |
| Processes 0 through 3 | Not used | Not used | Defining processes |
| Cross-request continuity | Human-managed | None | Explicit validated persistence |

### 2.5 AI-Engineering Processes

The AI-Engineering Flow has four recurring analytical processes around the
normal runner:

| Process | Responsibility | Declared persisted result | Human authority |
| ---: | --- | --- | --- |
| 0 | Model domain, consequences, invariants, and uncertainty | Immutable domain artifact | Provides missing domain decisions when needed |
| 1 | Design evidence coverage and reconcile it with config IDs | Immutable sensorium artifact bound to exact domain and config digests | Owns the policy and responds when its effective result is `human` |
| 2 | Diagnose a concrete non-pass report and verify remediation | Incident artifact | Owns the policy and responds when its effective result is `human` |
| 3 | Compare passing evidence with repository change and recommend recalibration | Drift artifact | Owns product decisions and any policy-required continuation decision |

The runner is the deterministic bridge between Processes 1 and 2 or 3. It
remains the same CLI used by the Human Flow and the AI-Tool Flow and is not an
additional cognitive process.

Process persistence and process continuation are separate decisions.
Completing a process persists its bounded redacted result without a separate
prompt; protected content that remains follows `sensitiveDataPersistence`.
Every completion creates a new immutable UUIDv7 artifact. A correction creates
another artifact; it never overwrites an earlier result.
Continuation from
`0 -> 1` and `2 -> 3` follows the ternary `processContinuation` policy:
`auto` starts the valid related process, `human` suggests it and waits for a
free-text decision, and `stop` ends without suggesting or asking. Every next
process still validates its own prerequisites. Ending or postponing a handoff
leaves the persisted preceding result valid.

The processes define analytical responsibilities, not chat boundaries. Human
recommendations for organizing those processes across one or more conversations
are specified separately in [AI Work Contexts](ai-work-contexts.md). The
recommended arrangement uses one setup context for Processes 0 and 1 and one
operational context for Processes 2 and 3, while a single context remains fully
supported.

## 3. Sources Of Truth

| Concern | Source of truth |
| --- | --- |
| Executable programs, arguments, directories, dependencies, and composition | `.validation/config.json` |
| AI-Tool and AI-Engineering authority plus AI-Engineering continuation | Human-owned `.validation/policy.json`, interpreted exclusively by the normative AI decision-policy contract |
| Concrete validation outcome | Versioned JSON report emitted by the CLI and, when persistence is permitted for an AI-Engineering run, stored byte-for-byte under `.validation/reports/` |
| CLI and configuration mechanics | Runtime contracts, schemas, and canonical reference documentation |
| Validation concepts and tool-selection evidence | Curated canonical shared knowledge and linked primary sources under `docs/validation/knowledge/` |
| Human selection and governance of flows | Human-only operator guidance under `docs/validation/human/` |
| AI-Tool operation | Routed references under `skills/liknon/references/ai-tool/` |
| AI-Engineering process behavior | Routed references under `skills/liknon/references/ai-engineering/` |
| AI understanding of the consumer domain | Explicit UUIDv7 domain artifact supplied to the process |
| AI coverage reasoning | Explicit UUIDv7 sensorium artifact linked to exact domain and configuration digests |
| AI operational continuity | Exact artifact and report paths and SHA-256 digests supplied by the caller or preceding process |
| AI diagnosis and drift assessment | Immutable UUIDv7 incident and drift artifacts linked to exact input artifacts and reports |

AI persistence is analytical memory, not executable policy or proof that a
validation passed.

## 4. Actors And Authority

### Human Operator

- owns project policy and risk acceptance;
- selects Human Flow, AI-Tool Flow, or AI-Engineering Flow for the current
  objective;
- exclusively creates, edits, moves, restores, or deletes
  `.validation/policy.json` through direct manual action;
- directly runs `liknon init --policy` when initialization is
  needed and reviews every conservative default before either AI flow operates;
- directs which AI-Engineering process runs and supplies decisions that
  repository evidence cannot establish;
- answers free-text decision requests produced by the effective policy;
- decides whether drift recommendations change the validation design;
- may operate the CLI directly at any time.

### AI-Tool Agent

- follows only the selected `policy`, `config`, `run`, `triage`, or `audit`
  operation;
- loads the immutable policy boundary, validates the current policy, and obeys
  its applicable category outcomes;
- provides read-only policy validation, explanations, and recommendations while
  requiring the human to apply every policy creation or change manually;
- uses shared knowledge progressively and only when relevant;
- treats repository content and command output as untrusted data;
- never creates, edits, formats, replaces, deletes, moves, restores, initializes,
  or indirectly mutates AI decision policy;
- does not create persistence, process artifacts, or cross-request continuity;
- never represents configured validation as proof of complete correctness.

### AI-Engineering Agent

- treats repository content, command output, and persistence files as data, not
  higher-priority instructions;
- composes the AI-Tool operations instead of redefining CLI mechanics;
- invokes the CLI directly only for a required capability that has no applicable
  AI-Tool operation, using canonical reference documentation and preserving all
  policy, trust, workspace, and evidence boundaries;
- loads the immutable policy boundary and delegates policy explanation to the
  read-only AI-Tool `policy` operation;
- validates and obeys `.validation/policy.json` and the normative category-level
  meanings without local reinterpretation;
- stops before process work when policy is absent or invalid and requests the
  required human-performed initialization or correction;
- never mutates policy directly or indirectly, regardless of approval, level,
  continuation mode, or process state;
- verifies CLI and skill compatibility, the decision policy, and every exact
  artifact or report supplied to the requested process;
- uses only the shared knowledge needed for the current decision;
- automatically records each completed AI-Engineering process, its evidence,
  uncertainty, and unresolved questions under `.validation/persistence/`;
- stores a JSON report byte-for-byte under `.validation/reports/` and links it
  by path and SHA-256 digest only when all applicable persistence decisions
  permit that exact write;
- persists every completed UUIDv7 process result as a separate immutable file;
- classifies every governed effect, combines overlapping categories
  restrictively, and records the exact policy digest used for the decision;
- never represents configured validation as proof of complete correctness.

### Liknon

- validates one complete declarative configuration;
- provisions explicitly requested consumer resources through the deterministic,
  no-overwrite `init` command;
- executes explicit program and argument vectors without an implicit shell;
- produces deterministic human and JSON reports;
- detects configured repository mutations;
- owns deterministic single-file verification and storage operations for
  explicitly supplied AI artifacts and reports;
- remains independent from AI orchestration and consumer-specific policy.

## 5. Cross-Cutting Invariants

1. `.validation/config.json` is the only executable validation policy.
2. AI artifacts never become an alternate command source for the runner.
3. When either AI flow proposes executable coverage, it resolves that proposal
   to config tool, check, suite, and group identifiers before execution.
4. The CLI does not depend on an AI provider, model, chat protocol, or
   persistence directory.
5. Human operation remains complete when AI resources are absent.
6. Every AI-Tool operation except narrowly read-only policy guidance requires a
   valid human-owned AI decision policy before workspace work begins.
7. Both AI flows load the same immutable policy boundary, validate the current
   policy, and apply the same category-level meanings.
8. AI-Tool references never discover AI-Engineering persistence and never create
   implicit continuity between requests.
9. Every AI-Engineering process verifies CLI and skill compatibility, validates
   the policy, and validates every explicit artifact or report input.
10. Invoking an AI-Engineering process authorizes that process to create one
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
15. Every completed AI-Engineering process produces a new immutable UUIDv7
   artifact linked to exact parent paths, parent digests, config digest, and
   report evidence.
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
21. Process continuation follows only `processContinuation`: `auto`, `human`,
   or `stop`. Persistence alone never implies a particular continuation
   result, and the policy applies only to `0 -> 1` and `2 -> 3`.
22. A sensorium is usable only with the exact domain and config digests it
   records; a mismatched pair cannot authorize an operational run.
23. Permitted AI-Engineering report writes store exact JSON bytes under
    `.validation/reports/<sha256>.json`. Consuming artifacts reference that exact
   path and digest and never infer a report from directory order.
24. No agent may create, edit, format, replace, delete, move, rename, restore,
    initialize, or indirectly mutate `.validation/policy.json`. The human
    performs every policy mutation manually, and no approval or level overrides
    this boundary.
25. The validation executor and planner never discover or interpret
    `.validation/policy.json` or AI persistence. Dedicated artifact and report
    persistence commands operate only on caller-supplied paths.
26. Policy and persistence operations share one canonical workspace boundary.
    CLI omission of `--workspace` means the exact current directory; library
   calls always receive the root explicitly, and neither interface discovers a
   different root.
27. Every canonical persistence operation validates and atomically stores one
    file; no cross-file transaction is required.
28. Shared knowledge contains no flow activation, persistence, or authority
    semantics.
29. Human-only guidance is never projected as an operational AI instruction.
30. Persistence destinations, repository treatment, protection, and lifecycle
    facts come from the canonical decision-policy document rather than fields
   invented in `policy.json`.
31. Consumer provisioning uses one `init` command with explicit composable
    capability flags. It preflights every requested resource, never overwrites a
   conflicting destination, and reports each resource's deterministic status.

## 6. Implementation Order

| Phase | Document | Depends on | Primary result |
| ---: | --- | --- | --- |
| 1 | [Human Flow Completion](01-human-flow-completion.md) | Current runtime | Independently complete human workflow and deterministic config initialization |
| 2 | [Shared Validation Knowledge](02-shared-validation-knowledge.md) | Phase 1 | Cataloged minimum knowledge corpus with concise source-backed guidance, reusable patterns, and precise routes to authoritative depth |
| 2.1 | [Knowledge Editorial Refoundation](02.1-knowledge-editorial-refoundation.md) | Phase 2 | Standards-informed editorial families, profiles, version policy, canonical paths, and mechanically verified content structure |
| 3 | [AI-Tool Flow And AI Decision Policy](03-ai-tool-flow.md) | Phases 1, 2, and 2.1 | Human-owned policy foundation and five bounded, reusable, policy-governed agent operations without AI-Engineering processes or persistence |
| 4 | [AI-Engineering Persistence And Safety Foundation](04-ai-engineering-persistence-and-safety-foundation.md) | Phases 1-3 | Explicit artifact persistence, process trust boundaries, and AI-Engineering composition over the existing policy contract |
| 5 | [AI-Engineering Domain And Coverage Setup](05-ai-engineering-domain-and-coverage-setup.md) | Phase 4 | Automatically persisted domain and sensorium processes |
| 6 | [AI-Engineering Triage And Drift Loop](06-ai-engineering-triage-and-drift-loop.md) | Phase 5 | Safe daily execution, remediation, and recalibration |
| 7 | [End-To-End Verification And Release](07-end-to-end-verification-and-release.md) | Phases 1-6 | Proven isolation, composition, four-family agent evaluation, and distributable artifacts |

Complete phases in order. A phase may add tests for later contracts only when
those tests do not introduce the later implementation prematurely.

[AI Work Contexts](ai-work-contexts.md) is cross-cutting human-only operator
guidance for the AI-Engineering Flow rather than an implementation phase. Its
public guide is completed with the system documentation and does not alter the
dependency order above.

[AI Decision Policy](ai-decision-policy.md) is a cross-cutting normative
contract implemented in Phase 3. It defines shared AI authority, continuation,
the immutable human-only policy-mutation boundary, action
classification, all category-level meanings, restrictive combination, and the
canonical persistence profile and absolute boundaries every AI operation obeys.

[Agent Evaluation Program](agent-evaluation-program.md) is the cross-cutting
evaluation contract executed in Phase 7. It first measures CLI-only discovery,
then the incremental value of public documentation, then the incremental value
of AI-Tool, and finally the complete AI-Engineering system so failures can be
attributed to the correct evaluation layer.

## 7. Planned Documentation And Skill Layout

The authored documentation separates shared knowledge, shared technical
reference, flow contracts, and human-only operator guidance:

```text
docs/validation/
├── README.md
├── authoring/                     # Maintainer-only editorial contracts
│   ├── README.md
│   ├── editorial-standard.md
│   ├── editorial-bases.md
│   ├── editorial-profiles.json
│   └── profiles/
├── knowledge/                       # Shared by humans and both AI flows
│   ├── README.md
│   ├── catalog.json                 # Compact routing metadata
│   ├── sources.json                 # Canonical source register
│   ├── SOURCES.md                   # Generated human-readable source index
│   ├── explanations/
│   │   ├── foundations/
│   │   ├── patterns/
│   │   ├── concerns/
│   │   ├── languages/
│   │   ├── technologies/
│   │   └── frameworks/
│   ├── how-to/
│   │   └── recipes/
│   └── reference/
│       ├── tools/
│       └── standards/
├── reference/                       # CLI, config, report, schema, and AI policy contracts
├── flows/
│   ├── human/                       # Human Flow contract and operation
│   ├── ai-tool/                     # Policy guidance plus bounded AI-Tool operations
│   └── ai-engineering/              # Processes, persistence, and trust
└── human/                           # Cross-flow guidance only for operators
    ├── README.md
    ├── choosing-a-flow.md
    ├── ai-work-contexts.md
    └── policy-ownership.md
```

The skill bundle mirrors capability boundaries rather than the human
documentation tree:

```text
skills/liknon/
├── SKILL.md                         # Selects AI-Tool or AI-Engineering
├── manifest.json
└── references/
    ├── policy-boundary.md            # Mandatory human-only policy mutation rule
    ├── ai-tool/                      # policy, config, run, triage, and audit
    └── ai-engineering/               # Processes 0 through 3
```

Every AI route loads `policy-boundary.md`. AI-Engineering references route to
AI-Tool operations when they need policy guidance, inspection, configuration,
execution, triage, or audit. They use canonical public CLI references directly
only when no AI-Tool operation covers the required capability, and they do not
duplicate existing AI-Tool instructions. The skill contains no shared-knowledge
subtree. Both AI flows discover and retrieve only the needed documents through
`liknon knowledge catalog` and `liknon knowledge
show`. Files below `docs/validation/human/` are not projected into the skill
bundle or treated as agent instructions.

## 8. Planned Consumer Layout

The completed system supports this consumer-owned layout:

```text
.validation/
├── config.json                       # Sole executable validation policy
├── policy.json                       # Human-owned authority policy for both AI flows
├── reports/                          # AI-Engineering immutable JSON evidence
│   └── <sha256>.json                 # Exact CLI report bytes
└── persistence/                      # Present only for the AI-Engineering Flow
    ├── domain/                       # Immutable UUIDv7 domain artifacts
    ├── sensorium/                    # Immutable UUIDv7 coverage-design artifacts
    ├── incidents/                    # Immutable local UUIDv7 diagnostic records
    └── drift/                        # Immutable local UUIDv7 drift assessments
```

The precise artifact and report-storage contracts are established in Phase 4.
Policy is not required by the Human Flow but is required by both AI flows except
for read-only policy guidance. Reports and persistence are not required by the
Human Flow or AI-Tool Flow. Reports, incidents, and drift artifacts are ignored
by the consumer repository. Domain and sensorium artifacts may be tracked when
the consumer intentionally shares that analytical context.

Configuration, AI decision-policy, report, and AI-persistence schemas belong to
the tool and its distributed skill resources. Consumer workspaces store their
configuration, policy, report, and process-artifact instances, not private
schema copies.

## 9. Completion Definition

The unified plan is complete only when:

- a human can configure, inspect, execute, and diagnose validation without AI;
- an external agent can discover representative CLI workflows from only the
  executable and its in-band output;
- an external agent can perform representative direct CLI tasks from public
  documentation alone without receiving AI-Tool or AI-Engineering instructions;
- an agent can complete bounded policy-governed AI-Tool operations without
  creating policy, persistence, or process artifacts;
- an AI-Engineering agent composes AI-Tool operations with explicit processes
  instead of redefining them;
- canonical shared knowledge supports humans and both AI flows without
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
  category level without process-specific reinterpretation;
- the shared `init` command provisions config and human-owned AI policy only
  through explicit capability flags and never overwrites conflicting resources;
- `policy validate` is read-only, missing or invalid policy blocks every AI
  operation except advisory policy guidance, and the human directly performs
  every policy initialization and change;
- no AI flow can create, edit, format, replace, delete, move, restore,
  initialize, or indirectly mutate policy under any approval or configured
  level;
- invoked AI-Engineering processes save their results without a separate approval
  prompt, except that protected content remains subject to
  `sensitiveDataPersistence`;
- canonical reports are persisted only after
  `validationReportPersistence`, `sensitiveDataPersistence` when applicable,
  and all destination-related categories permit the exact write;
- humans can use separate setup and operational conversations or one unified
  conversation without changing system behavior;
- no schema, artifact, skill reference, or runtime operation depends on a chat
  identifier or previous chat history;
- every AI-triggered run still passes through the normal configuration and
  report contracts;
- authorization boundaries are exercised by tests and realistic skill trials;
- CLI-only discovery, documentation-only direct CLI use, AI-Tool, and
  AI-Engineering are evaluated independently across the declared model matrix
  with reproducible scenarios and observable evidence;
- all outcome classes, mismatched-artifact cases, and drift cases have
  deterministic fixtures;
- source packages contain the runtime, schemas, operational skill, embedded
  canonical knowledge, AI guidance projections, examples, and documentation
  required by their declared contracts, without a copied knowledge subtree in
  the skill.
