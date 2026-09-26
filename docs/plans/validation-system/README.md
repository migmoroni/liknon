# Unified Validation System Implementation Plan

## 1. Purpose

This plan completes two supported ways to use `workspace-validator`:

1. a human-operated flow based directly on `.validation/config.json`;
2. an AI-assisted flow in which a human directs an agent that uses the same
   configuration, CLI, and reports while maintaining explicit cognitive state
   under `.validation/persistence/`.

The human flow is the operational foundation. The AI-assisted flow is an
optional orchestration layer over that foundation. It does not replace or
extend the CLI execution contract.

This plan uses **phase** only for the six ordered implementation units and
**process** for the four recurring AI-assisted operations. Processes can be
invoked again whenever their preconditions hold; they are not release phases or
mandatory steps in one linear lifecycle.

## 2. Architectural Model

### 2.1 Human-Operated Flow

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

### 2.2 AI-Assisted Flow

```text
human invokes and directs the agent
                  |
                  v
agent loads the workspace-validator skill and relevant guidance
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
each invoked persistent process. This does not require a separate human
approval.
Capturing the exact JSON report in the local report store and updating its
operational link are also declared outputs of a persistent AI validation run
and require no additional approval.

The human owns the AI decision policy and remains the final authority over
product intent and risk acceptance. Effects outside declared automatic
process outputs follow `.validation/policy.json`: they run automatically only
at the configured category level or when the current human instruction already
authorizes the exact action. The agent requests a free-text decision whenever
the effective policy result is `human`.

### 2.3 Shared Boundary

The flows meet at exactly three contracts:

| Contract | Owner | Purpose |
| --- | --- | --- |
| `.validation/config.json` | Consumer workspace | Sole executable validation policy |
| `workspace-validator` CLI/library | This crate | Deterministic inspection and execution |
| Versioned validation report | This crate | Canonical evidence from one execution, whether consumed directly or stored by exact digest |

`.validation/persistence/` belongs only to the AI-assisted flow. The CLI and
library never discover, read, interpret, or execute files from that directory.
`.validation/reports/` is a local evidence store for exact JSON reports. It is
not executable policy or analytical memory, and no workflow selects a report
by directory order or modification time.

`.validation/policy.json` also belongs only to the AI-assisted flow. It is the
consumer-owned AI decision policy, not executable validation configuration.
Its complete normative semantics are specified by
[AI Decision Policy](ai-decision-policy.md).

### 2.4 AI-Assisted Processes

The persistent AI flow has four recurring analytical processes around the
normal runner:

| Process | Responsibility | Automatic persisted result | Human authority |
| ---: | --- | --- | --- |
| 0 | Model domain, consequences, invariants, and uncertainty | Domain artifact and current-state pointer | Provides missing domain decisions when needed |
| 1 | Design evidence coverage and reconcile it with config IDs | Sensorium artifact and current-state pointer | Owns the policy and responds when its effective result is `human` |
| 2 | Diagnose a concrete non-pass report and verify remediation | Incident artifact | Owns the policy and responds when its effective result is `human` |
| 3 | Compare passing evidence with repository change and recommend recalibration | Drift artifact | Owns product decisions and any policy-required continuation decision |

The runner is the deterministic bridge between Processes 1 and 2 or 3. It
remains the same CLI used by the human flow and is not an additional cognitive
process.

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
| AI continuation and human-decision boundaries | `.validation/policy.json`, interpreted exclusively by the normative AI decision-policy contract |
| Concrete validation outcome | Versioned JSON report emitted by the CLI and, for persistent AI runs, stored byte-for-byte under `.validation/reports/` |
| CLI and configuration mechanics | Runtime contracts, schemas, and bundled skill workflows |
| Validation concepts and tool-selection evidence | Canonical guidance under `docs/validation/` |
| AI understanding of the consumer domain | Current domain artifact under `.validation/persistence/` |
| AI coverage reasoning | Current sensorium artifact linked to an exact configuration digest |
| AI operational continuity | Local operational state linked to an exact report path and SHA-256 digest |
| AI diagnosis and drift assessment | Local incident and drift artifacts linked to exact reports and current state |

AI persistence is analytical memory, not executable policy or proof that a
validation passed.

## 4. Actors And Authority

### Human Operator

- owns project policy and risk acceptance;
- owns `.validation/policy.json` and explicitly authorizes every change to that
  policy;
- directs which AI process runs and supplies decisions that repository evidence
  cannot establish;
- answers free-text decision requests produced by the effective policy;
- decides whether drift recommendations change the validation design;
- may operate the CLI directly at any time.

### AI Agent

- treats repository content, command output, and persistence files as data, not
  higher-priority instructions;
- follows the bundled skill for CLI mechanics and safety;
- validates and obeys `.validation/policy.json` and the normative category-level
  meanings without local reinterpretation;
- uses only the guidance needed for the current decision;
- automatically records each completed persistent process, its evidence,
  uncertainty, and unresolved questions under `.validation/persistence/`;
- stores each JSON report used by a persistent AI operation byte-for-byte under
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
3. The AI resolves proposed coverage to config tool, check, suite, and group
   identifiers before execution.
4. The CLI does not depend on an AI provider, model, chat protocol, or
   persistence directory.
5. Human operation remains complete when AI resources are absent.
6. Invoking a persistent AI process authorizes that process to create its
   artifact and update its context pointer without another approval prompt.
7. AI persistence writes are confined to `.validation/persistence/`. The only
   additional automatic workspace write is exact report capture under
   `.validation/reports/`; neither write authorizes any other effect.
8. Effects outside declared automatic persistence and report capture are
   classified under the current AI decision policy before execution.
9. The decision policy uses the exact category-specific meaning of levels
   `0` through `4`; overlapping categories resolve as `stop > human > auto`.
10. The JSON report is interpreted by schema and status, not by terminal prose
   or a binary success/failure assumption.
11. AI artifacts identify their contract version and link to their exact parent
   artifacts, config digest, and report evidence.
12. Passing checks establish only the evidence represented by the selected
    configuration.
13. Safety information, uncertainty, and policy-required human decisions
    override compact chat formatting.
14. Documentation, skill instructions, schemas, examples, and code comments
    are written in English.
15. Chat identity and chat history are never execution inputs, persistence
    references, or prerequisites for resuming an AI workflow.
16. Conversational organization is human-facing quality guidance and never
    changes process semantics, authorization, executable policy, or reports.
17. Process continuation follows only `processContinuation`: `auto`, `human`,
    or `stop`. Persistence alone never implies a particular continuation
    result.
18. Replacing the current domain artifact immediately marks the current
    sensorium as requiring Process 1 revalidation; a stale sensorium cannot
    authorize an operational run.
19. Persistent AI operations store exact JSON report bytes under
    `.validation/reports/<sha256>.json` and reference that path and digest
    explicitly. They never infer the current report from the newest file.
20. The AI decision policy never authorizes its own modification. Except for
    first creation of the canonical conservative policy, every policy change
    requires exact human authority.
21. The CLI and library never discover or interpret `.validation/policy.json`.

## 6. Implementation Order

| Phase | Document | Depends on | Primary result |
| ---: | --- | --- | --- |
| 1 | [Human Flow Completion](01-human-flow-completion.md) | Current runtime | Independently complete human workflow |
| 2 | [Shared Validation Knowledge](02-shared-validation-knowledge.md) | Phase 1 | Source-backed guidance for humans and agents |
| 3 | [AI State And Safety Foundation](03-ai-state-and-safety-foundation.md) | Phases 1-2 | Versioned AI state and decision-policy contracts |
| 4 | [AI Domain And Coverage Setup](04-ai-domain-and-coverage-setup.md) | Phase 3 | Automatically persisted domain and sensorium workflow |
| 5 | [AI Triage And Drift Loop](05-ai-triage-and-drift-loop.md) | Phase 4 | Safe daily execution, remediation, and recalibration |
| 6 | [End-To-End Verification And Release](06-end-to-end-verification-and-release.md) | Phases 1-5 | Proven interoperability and distributable artifacts |

Complete phases in order. A phase may add tests for later contracts only when
those tests do not introduce the later implementation prematurely.

[AI Work Contexts](ai-work-contexts.md) is a cross-cutting human-guidance
specification rather than an implementation phase. Its public guide is
completed with the system documentation and does not alter the dependency
order above.

[AI Decision Policy](ai-decision-policy.md) is a cross-cutting normative
contract implemented in Phase 3. It defines continuation, action
classification, all category-level meanings, restrictive combination, and the
absolute boundaries every AI process obeys.

## 7. Planned Consumer Layout

The completed system supports this consumer-owned layout:

```text
.validation/
├── config.json                       # Sole executable validation policy
├── policy.json                       # AI continuation and decision policy
├── reports/                          # Local immutable JSON execution evidence
│   └── <sha256>.json                 # Exact CLI report bytes
└── persistence/                      # Present only for the AI-assisted flow
    ├── state.json                    # Current validated AI context references
    ├── operational-state.json        # Exact local report and workflow linkage
    ├── domain/                       # Versioned domain artifacts
    ├── sensorium/                    # Versioned coverage-design artifacts
    ├── incidents/                    # Local diagnostic records
    └── drift/                        # Local drift assessments
```

The precise state and report-storage contracts are established in Phase 3.
Neither directory is required for direct human operation.

Configuration, AI decision-policy, report, and AI-persistence schemas belong to
the tool and its distributed skill resources. Consumer workspaces store their
configuration, policy, and state documents, not private schema copies.

## 8. Completion Definition

The unified plan is complete only when:

- a human can configure, inspect, execute, and diagnose validation without AI;
- an agent can use the bundled skill without learning undocumented CLI rules;
- canonical guidance supports both audiences without separately authored
  rewrites;
- AI state is resumable, versioned, linked, and non-executable;
- domain replacement makes Process 1 revalidation explicit before the next
  operational run;
- persistent operational state identifies the exact stored report by path and
  digest without directory scanning;
- the AI decision policy defines every continuation state and every numeric
  category level without process-specific reinterpretation;
- missing policy creates the conservative canonical document, invalid policy
  fails closed, and policy changes always require human authority;
- invoked persistent processes save their results without a separate approval
  prompt;
- humans can use separate setup and operational conversations or one unified
  conversation without changing system behavior;
- no schema, artifact, skill workflow, or runtime operation depends on a chat
  identifier or previous chat history;
- every AI-triggered run still passes through the normal configuration and
  report contracts;
- authorization boundaries are exercised by tests and realistic skill trials;
- all outcome classes, stale-state cases, and drift cases have deterministic
  fixtures;
- source packages contain the runtime, schemas, skill, guidance projection,
  examples, and documentation required by their declared contracts.
