# Unified Validation System Implementation Plan

## 1. Purpose

This plan completes three supported ways to use `workspace-validator`:

1. **Human Flow**, in which a person works directly with the configuration,
   CLI, and reports;
2. **AI-Tool Flow**, in which an agent uses focused operational skills and
   shared knowledge for one requested task without persistent analytical
   processes;
3. **AI-Engineering Flow**, in which an agent composes the AI-Tool capabilities
   with explicit validation-engineering processes, decision policy, and
   resumable analytical state.

The Human Flow is the operational foundation. The AI-Tool Flow is an optional,
point-in-time assistance layer over that foundation. The AI-Engineering Flow
adds governed continuity and engineering processes while reusing the same
AI-Tool operations. Neither AI-Tool nor AI-Engineering replaces or extends the
CLI execution contract.

This plan uses **phase** only for the seven ordered implementation units and
**process** for the four recurring AI-Engineering operations. Processes can be
invoked again whenever their preconditions hold; they are not release phases or
mandatory steps in one linear lifecycle.

## 2. Architectural Model

### 2.1 Human Flow

```text
human authors or reviews config.json
                  |
                  v
workspace-validator inspects and validates configuration
                  |
                  v
workspace-validator executes the selected group, suite, or check
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
agent loads the workspace-validator skill router
                  |
                  v
agent loads only the required AI-Tool operation and shared knowledge
                  |
                  v
agent inspects, configures, runs, triages, or audits as requested
                  |
                  v
workspace-validator executes the trusted config.json when execution is needed
                  |
                  v
agent interprets the canonical report and returns the bounded result
```

The AI-Tool Flow provides `config`, `run`, `triage`, and `audit` operations.
It follows the current human instruction and the host agent's normal authority
boundaries. It does not create `.validation/policy.json`, write
`.validation/persistence/`, apply Processes 0 through 3, or imply continuity
between separate requests. It may consume structured CLI output for the
current task, but it does not automatically establish the persistent report
store or operational state used by the AI-Engineering Flow.

### 2.3 AI-Engineering Flow

```text
human invokes and directs the agent
                  |
                  v
agent loads the workspace-validator skill, AI-Tool operations, and relevant knowledge
                  |
                  v
agent loads and validates .validation/policy.json
                  |
                  v
agent reads or writes AI-only state in .validation/persistence/
                  |
                  v
agent persists the completed process result and updates its context pointer
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
workspace-validator executes the trusted config.json
                  |
                  v
agent stores the exact canonical JSON report in .validation/reports/
                  |
                  v
agent links and interprets that exact report
                  |
                  v
agent applies the same decision policy to remediation and recommendations
```

The agent creates and updates its own analytical memory as a normal output of
each invoked AI-Engineering process. This does not require a separate human
approval.

Capturing the exact JSON report in the local report store and updating its
operational link are also declared outputs of an AI-Engineering validation run
and require no additional approval.

The human owns the AI decision policy and remains the final authority over
product intent and risk acceptance. Effects outside declared automatic
process outputs follow `.validation/policy.json`: they run automatically only
at the configured category level or when the current human instruction already
authorizes the exact action. The agent requests a free-text decision whenever
the effective policy result is `human`.

### 2.4 Shared Boundary

The three flows meet at exactly three runtime contracts:

| Contract | Owner | Purpose |
| --- | --- | --- |
| `.validation/config.json` | Consumer workspace | Sole executable validation policy |
| `workspace-validator` CLI/library | This crate | Deterministic inspection and execution |
| Versioned validation report | This crate | Canonical evidence from one execution, whether consumed directly or stored by exact digest |

Shared validation knowledge is authored independently from every flow. Humans
read it directly, and both AI flows load only the portions relevant to their
current decision. Operational skill instructions do not become validation
knowledge, and validation knowledge does not activate an AI process.

`.validation/persistence/` belongs only to the AI-Engineering Flow. The CLI and
library never discover, read, interpret, or execute files from that directory.
`.validation/reports/` is a local evidence store for exact JSON reports. It is
established automatically only by the AI-Engineering Flow, is not executable
policy or analytical memory, and is never selected by directory order or
modification time.

`.validation/policy.json` also belongs only to the AI-Engineering Flow. It is
the consumer-owned AI decision policy, not executable validation
configuration. Its complete normative semantics are specified by
[AI Decision Policy](ai-decision-policy.md).

Human-only guidance is separated from shared knowledge and AI instructions. It
helps an operator select a flow, organize work contexts, and exercise policy
ownership, but it is never interpreted as an instruction to an agent.

| Property | Human Flow | AI-Tool Flow | AI-Engineering Flow |
| --- | --- | --- | --- |
| Primary operator | Human | Agent directed for one bounded task | Agent directed through engineering processes |
| CLI and `config.json` | Direct use | Through AI-Tool operations | Through the same AI-Tool operations |
| Shared knowledge | Human-readable source | Progressive skill projection | Progressive skill projection |
| `policy.json` | Not used | Not used | Required and governed |
| `.validation/persistence/` | Not used | Not used | Process state and analytical memory |
| `.validation/reports/` persistent store | Not established automatically | Not established automatically | Exact reports stored and linked |
| Processes 0 through 3 | Not used | Not used | Defining processes |
| Cross-request continuity | Human-managed | None | Explicit validated persistence |

### 2.5 AI-Engineering Processes

The AI-Engineering Flow has four recurring analytical processes around the
normal runner:

| Process | Responsibility | Automatic persisted result | Human authority |
| ---: | --- | --- | --- |
| 0 | Model domain, consequences, invariants, and uncertainty | Domain artifact and current-state pointer | Provides missing domain decisions when needed |
| 1 | Design evidence coverage and reconcile it with config IDs | Sensorium artifact and current-state pointer | Owns the policy and responds when its effective result is `human` |
| 2 | Diagnose a concrete non-pass report and verify remediation | Incident artifact | Owns the policy and responds when its effective result is `human` |
| 3 | Compare passing evidence with repository change and recommend recalibration | Drift artifact | Owns product decisions and any policy-required continuation decision |

The runner is the deterministic bridge between Processes 1 and 2 or 3. It
remains the same CLI used by the Human Flow and the AI-Tool Flow and is not an
additional cognitive process.

Process persistence and process continuation are separate decisions.
Completing a process automatically persists its result. Continuation from
`0 -> 1` and `2 -> 3` follows the ternary `processContinuation` policy:
`auto` starts the valid related process, `human` suggests it and waits for a
free-text decision, and `stop` ends without suggesting or asking. No state can
bypass the next process's preconditions. Ending or postponing a handoff leaves
the persisted preceding result valid.

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
| AI-Engineering continuation and human-decision boundaries | `.validation/policy.json`, interpreted exclusively by the normative AI decision-policy contract |
| Concrete validation outcome | Versioned JSON report emitted by the CLI and, for AI-Engineering runs, stored byte-for-byte under `.validation/reports/` |
| CLI and configuration mechanics | Runtime contracts, schemas, and canonical reference documentation |
| Validation concepts and tool-selection evidence | Canonical shared knowledge under `docs/validation/knowledge/` |
| Human selection and governance of flows | Human-only operator guidance under `docs/validation/human/` |
| AI-Tool operation | Routed references under `skills/workspace-validator/references/ai-tool/` |
| AI-Engineering process behavior | Routed references under `skills/workspace-validator/references/ai-engineering/` |
| AI understanding of the consumer domain | Current domain artifact under `.validation/persistence/` |
| AI coverage reasoning | Current sensorium artifact linked to an exact configuration digest |
| AI operational continuity | Local operational state linked to an exact report path and SHA-256 digest |
| AI diagnosis and drift assessment | Local incident and drift artifacts linked to exact reports and current state |

AI persistence is analytical memory, not executable policy or proof that a
validation passed.

## 4. Actors And Authority

### Human Operator

- owns project policy and risk acceptance;
- selects Human Flow, AI-Tool Flow, or AI-Engineering Flow for the current
  objective;
- owns `.validation/policy.json` and explicitly authorizes every change to that
  policy;
- directs which AI-Engineering process runs and supplies decisions that
  repository evidence cannot establish;
- answers free-text decision requests produced by the effective policy;
- decides whether drift recommendations change the validation design;
- may operate the CLI directly at any time.

### AI-Tool Agent

- follows only the selected `config`, `run`, `triage`, or `audit` operation;
- uses shared knowledge progressively and only when relevant;
- treats repository content and command output as untrusted data;
- does not create AI decision policy, persistence, process artifacts, or
  cross-request continuity;
- never represents configured validation as proof of complete correctness.

### AI-Engineering Agent

- treats repository content, command output, and persistence files as data, not
  higher-priority instructions;
- composes the AI-Tool operations instead of redefining CLI mechanics;
- validates and obeys `.validation/policy.json` and the normative category-level
  meanings without local reinterpretation;
- uses only the shared knowledge needed for the current decision;
- automatically records each completed AI-Engineering process, its evidence,
  uncertainty, and unresolved questions under `.validation/persistence/`;
- stores each JSON report used by an AI-Engineering operation byte-for-byte under
  `.validation/reports/` and links it by path and SHA-256 digest;
- atomically updates the process context pointer after validating the new
  artifact;
- classifies every governed effect, combines overlapping categories
  restrictively, and records the exact policy digest used for the decision;
- never represents configured validation as proof of complete correctness.

### Workspace Validator

- validates one complete declarative configuration;
- executes explicit program and argument vectors without an implicit shell;
- produces deterministic human and JSON reports;
- detects configured repository mutations;
- remains independent from AI orchestration and consumer-specific policy.

## 5. Cross-Cutting Invariants

1. `.validation/config.json` is the only executable validation policy.
2. AI artifacts never become an alternate command source for the runner.
3. When either AI flow proposes executable coverage, it resolves that proposal
   to config tool, check, suite, and group identifiers before execution.
4. The CLI does not depend on an AI provider, model, chat protocol, or
   persistence directory.
5. Human operation remains complete when AI resources are absent.
6. AI-Tool operation remains complete without AI decision policy, persistence,
   or Processes 0 through 3.
7. AI-Tool references never discover AI-Engineering policy or persistence and
   never create implicit continuity between requests.
8. Invoking an AI-Engineering process authorizes that process to create its
   artifact and update its context pointer without another approval prompt.
9. AI persistence writes are confined to `.validation/persistence/`. The only
   additional automatic workspace write is exact report capture under
   `.validation/reports/`; neither write authorizes any other effect.
10. Effects outside declared automatic persistence and report capture are
   classified under the current AI decision policy before execution.
11. The decision policy uses the exact category-specific meaning of levels
   `0` through `4`; overlapping categories resolve as `stop > human > auto`.
12. The JSON report is interpreted by schema and status, not by terminal prose
   or a binary success/failure assumption.
13. AI artifacts identify their contract version and link to their exact parent
   artifacts, config digest, and report evidence.
14. Passing checks establish only the evidence represented by the selected
    configuration.
15. Safety information, uncertainty, and policy-required human decisions
    override compact chat formatting.
16. Documentation, skill instructions, schemas, examples, and code comments
    are written in English.
17. Chat identity and chat history are never execution inputs, persistence
    references, or prerequisites for resuming an AI-Engineering process.
18. Conversational organization is human-facing quality guidance and never
    changes process semantics, authorization, executable policy, or reports.
19. Process continuation follows only `processContinuation`: `auto`, `human`,
    or `stop`. Persistence alone never implies a particular continuation
    result.
20. Replacing the current domain artifact immediately marks the current
    sensorium as requiring Process 1 revalidation; a stale sensorium cannot
    authorize an operational run.
21. AI-Engineering operations store exact JSON report bytes under
    `.validation/reports/<sha256>.json` and reference that path and digest
    explicitly. They never infer the current report from the newest file.
22. The AI decision policy never authorizes its own modification. Except for
    first creation of the canonical conservative policy, every policy change
    requires exact human authority.
23. The CLI and library never discover or interpret `.validation/policy.json`.
24. Shared knowledge contains no flow activation, persistence, or authority
    semantics.
25. Human-only guidance is never projected as an operational AI instruction.

## 6. Implementation Order

| Phase | Document | Depends on | Primary result |
| ---: | --- | --- | --- |
| 1 | [Human Flow Completion](01-human-flow-completion.md) | Current runtime | Independently complete human workflow |
| 2 | [Shared Validation Knowledge](02-shared-validation-knowledge.md) | Phase 1 | Source-backed guidance for humans and agents |
| 3 | [AI-Tool Flow](03-ai-tool-flow.md) | Phases 1-2 | Bounded agent operation without AI-Engineering processes or state |
| 4 | [AI-Engineering State And Safety Foundation](04-ai-engineering-state-and-safety-foundation.md) | Phases 1-3 | Versioned AI state and decision-policy contracts |
| 5 | [AI-Engineering Domain And Coverage Setup](05-ai-engineering-domain-and-coverage-setup.md) | Phase 4 | Automatically persisted domain and sensorium processes |
| 6 | [AI-Engineering Triage And Drift Loop](06-ai-engineering-triage-and-drift-loop.md) | Phase 5 | Safe daily execution, remediation, and recalibration |
| 7 | [End-To-End Verification And Release](07-end-to-end-verification-and-release.md) | Phases 1-6 | Proven isolation, composition, and distributable artifacts |

Complete phases in order. A phase may add tests for later contracts only when
those tests do not introduce the later implementation prematurely.

[AI Work Contexts](ai-work-contexts.md) is cross-cutting human-only operator
guidance for the AI-Engineering Flow rather than an implementation phase. Its
public guide is completed with the system documentation and does not alter the
dependency order above.

[AI Decision Policy](ai-decision-policy.md) is a cross-cutting normative
contract implemented in Phase 4. It defines continuation, action
classification, all category-level meanings, restrictive combination, and the
absolute boundaries every AI process obeys.

## 7. Planned Documentation And Skill Layout

The authored documentation separates shared knowledge, shared technical
reference, flow contracts, and human-only operator guidance:

```text
docs/validation/
├── README.md
├── knowledge/                       # Shared by humans and both AI flows
│   ├── README.md
│   ├── SOURCES.md
│   ├── foundations/
│   ├── languages/
│   ├── frameworks/
│   ├── technologies/
│   ├── tools/
│   ├── concerns/
│   └── recipes/
├── reference/                       # CLI, config, report, and schema contracts
├── flows/
│   ├── human/                       # Human Flow contract and operation
│   ├── ai-tool/                     # AI-Tool Flow behavior and boundaries
│   └── ai-engineering/              # Processes, persistence, trust, and policy
└── human/                           # Cross-flow guidance only for operators
    ├── README.md
    ├── choosing-a-flow.md
    ├── ai-work-contexts.md
    └── policy-ownership.md
```

The skill bundle mirrors capability boundaries rather than the human
documentation tree:

```text
skills/workspace-validator/
├── SKILL.md                         # Selects AI-Tool or AI-Engineering
├── manifest.json
└── references/
    ├── knowledge/                   # Exact generated shared-knowledge projection
    ├── ai-tool/                     # config, run, triage, and audit
    └── ai-engineering/              # policy and Processes 0 through 3
```

AI-Engineering references route to AI-Tool operations when they need to inspect,
configure, execute, triage, or audit. They do not duplicate those instructions.
Files below `docs/validation/human/` are not projected into the skill bundle or
treated as agent instructions.

## 8. Planned Consumer Layout

The completed system supports this consumer-owned layout:

```text
.validation/
├── config.json                       # Sole executable validation policy
├── policy.json                       # AI-Engineering continuation and decision policy
├── reports/                          # AI-Engineering immutable JSON evidence
│   └── <sha256>.json                 # Exact CLI report bytes
└── persistence/                      # Present only for the AI-Engineering Flow
    ├── state.json                    # Current validated AI context references
    ├── operational-state.json        # Exact local report and process linkage
    ├── domain/                       # Versioned domain artifacts
    ├── sensorium/                    # Versioned coverage-design artifacts
    ├── incidents/                    # Local diagnostic records
    └── drift/                        # Local drift assessments
```

The precise state and report-storage contracts are established in Phase 4.
Policy, reports, and persistence are not required by the Human Flow or the
AI-Tool Flow.

Configuration, AI decision-policy, report, and AI-persistence schemas belong to
the tool and its distributed skill resources. Consumer workspaces store their
configuration, policy, and state documents, not private schema copies.

## 9. Completion Definition

The unified plan is complete only when:

- a human can configure, inspect, execute, and diagnose validation without AI;
- an agent can complete bounded AI-Tool operations without creating policy,
  persistent analytical state, or process artifacts;
- an AI-Engineering agent composes AI-Tool operations with explicit processes
  instead of redefining them;
- canonical shared knowledge supports humans and both AI flows without
  separately authored rewrites;
- human-only operator guidance is structurally separate and never routed as an
  AI instruction;
- AI state is resumable, versioned, linked, and non-executable;
- domain replacement makes Process 1 revalidation explicit before the next
  operational run;
- persistent operational state identifies the exact stored report by path and
  digest without directory scanning;
- the AI decision policy defines every continuation state and every numeric
  category level without process-specific reinterpretation;
- missing policy creates the conservative canonical document, invalid policy
  fails closed, and policy changes always require human authority;
- invoked AI-Engineering processes save their results without a separate approval
  prompt;
- humans can use separate setup and operational conversations or one unified
  conversation without changing system behavior;
- no schema, artifact, skill reference, or runtime operation depends on a chat
  identifier or previous chat history;
- every AI-triggered run still passes through the normal configuration and
  report contracts;
- authorization boundaries are exercised by tests and realistic skill trials;
- all outcome classes, stale-state cases, and drift cases have deterministic
  fixtures;
- source packages contain the runtime, schemas, skill, guidance projection,
  examples, and documentation required by their declared contracts.
