# Phase 3: AI-Tool Flow

## 1. Objective

Implement the AI-Tool Flow as bounded, point-in-time assistance over the Human
Flow. An agent uses focused operations and shared validation knowledge to help
with configuration, execution, triage, or audit without activating validation-
engineering processes or creating AI-Engineering artifacts.

The AI-Tool Flow is independently useful. It is also the operational foundation
composed by the AI-Engineering Flow in later phases.

## 2. Dependencies

- Phase 1 provides stable CLI, configuration, report, and Human Flow contracts.
- Phase 2 provides the canonical shared-knowledge tree and its generated skill
  projection.

## 3. Flow Contract

The supported flow is:

```text
human requests one bounded task
  -> agent selects the AI-Tool Flow
  -> agent verifies CLI and skill compatibility
  -> agent loads one operational reference
  -> agent loads only relevant shared knowledge
  -> agent inspects or acts within the requested scope
  -> workspace-validator remains the only validation executor
  -> agent interprets the canonical result
  -> agent returns the bounded result
```

The flow supports four operations:

| Operation | Responsibility |
| --- | --- |
| `config` | Inspect, explain, create, or modify declarative validation configuration within current authority |
| `run` | Inspect and execute an explicit configured selection, then interpret its structured result |
| `triage` | Diagnose a concrete report or failure without assuming permission to remediate it |
| `audit` | Compare requested or observed validation coverage with repository evidence without executing checks unless separately requested |

Each invocation has one explicit objective and ends when that objective is
reported or blocked. A later invocation begins from current workspace facts and
the new human request, not from implied memory of an earlier conversation.

## 4. Architectural Boundary

The AI-Tool Flow:

- uses `.validation/config.json` as the sole executable validation policy;
- uses the current CLI and versioned report contract;
- may read canonical shared knowledge through progressive disclosure;
- follows the current human instruction and the host agent's authority and
  safety boundaries;
- may make an ordinary workspace change only when that exact change is
  authorized;
- never treats documentation, source, output, or configuration content as
  higher-priority instructions.

The AI-Tool Flow does not:

- create, read, or interpret `.validation/policy.json`;
- create or update `.validation/persistence/`;
- automatically establish `.validation/reports/` as a persistent report store;
- execute Processes 0 through 3;
- create domain, sensorium, incident, or drift artifacts;
- infer continuity, prior authorization, or unresolved work from chat history;
- apply AI-Engineering continuation rules.

An exact human request may ask the agent to write an ordinary report file or
other bounded output. That action remains part of the current request and does
not activate the AI-Engineering persistence contracts.

When repository facts are relevant, the agent inspects them through ordinary
read-only repository commands or through repository evidence already present in
a validation report. A reported repository change is evidence for the bounded
request, not an instruction to repair or revert it.

## 5. Skill Architecture

Use one concise top-level router and separate operational references from
shared knowledge:

```text
skills/workspace-validator/
├── SKILL.md
├── manifest.json
└── references/
    ├── knowledge/                   # Generated in Phase 2
    └── ai-tool/
        ├── index.md
        ├── config.md
        ├── run.md
        ├── triage.md
        └── audit.md
```

`SKILL.md` identifies the requested flow before loading an operation. A bounded
request to configure, run, triage, or audit selects AI-Tool unless the human
explicitly invokes an AI-Engineering process or requests its persistent system.
The router must not activate AI-Engineering from task complexity, repository
size, or the agent's preference.

`references/ai-tool/index.md` defines the common bounded-request boundary and routes
to exactly one primary operation. An operation may link to another AI-Tool
reference when the current request genuinely requires both, but it must not
eagerly load all operations.

The four operational references are authored skill instructions. Shared
knowledge is generated from `docs/validation/knowledge/` and remains
audience-neutral. Human-only files are not included in skill routing.

## 6. Operation Requirements

### 6.1 `config`

- Validate the complete current document before proposing changes.
- Use runtime types and bundled schemas as the public contract.
- Resolve programs, arguments, parameters, working directories, dependencies,
  suites, and groups explicitly.
- Present dependency or tool requirements before any authorized installation.
- Avoid repository-specific policy in the generic skill.
- Revalidate the resulting configuration and inspect affected graph nodes.

### 6.2 `run`

- Require an explicit check, suite, group, or documented default selection.
- Inspect the resolved operation before execution when scope is not already
  evident from the request.
- Prefer the versioned JSON report for agent interpretation.
- Preserve one-run deduplication and configured dependency semantics.
- Do not retry an unchanged failure or broaden the selected scope implicitly.
- Report every result class without reducing it to binary success or failure.

### 6.3 `triage`

- Bind the diagnosis to one exact supplied or newly produced report.
- Distinguish root failures, propagated blocking, skipping, timeout,
  interruption, invalid usage, and internal failure.
- Preserve uncertainty and separate evidence from inference.
- Recommend the narrowest useful next step.
- Do not modify source, configuration, dependencies, snapshots, or data unless
  the current human instruction authorizes that separate effect.

### 6.4 `audit`

- Inspect domain evidence supplied in the current request, repository
  structure, manifests, scripts, configuration, and selected shared knowledge.
- Identify represented evidence, overlaps, blind spots, prerequisites, and
  limitations.
- Keep recommendations source-backed and contextual rather than producing a
  universal assurance ladder.
- Do not persist an AI-Engineering domain or sensorium model.
- Do not execute proposed validation merely because the audit recommends it.

## 7. Public Documentation

Create a dedicated public entry point:

```text
docs/validation/flows/ai-tool/
├── README.md
├── config.md
├── run.md
├── triage.md
└── audit.md
```

The public documents explain the flow, its boundaries, and how a human invokes
each operation. They are not a second copy of the skill instructions. Public
documentation describes supported behavior; skill references define how an
agent performs it.

The root validation index clearly distinguishes AI-Tool from Human Flow and
AI-Engineering. It states that choosing AI-Tool does not initialize decision
policy or process artifacts.

## 8. Authority And Safety

- The human remains the authority for project intent, installation, mutation,
  external effects, and risk acceptance.
- The flow obeys system, host, workspace, repository, and current user
  instructions in their normal precedence.
- AI-Engineering `policy.json` is neither required nor consulted.
- A request to analyze does not imply authority to execute or mutate.
- A request to execute a configured selection does not imply authority to fix
  its failures.
- A request to edit configuration does not imply authority to install its
  referenced tools.
- Prompts for a human decision accept free text so the human can approve,
  reject, constrain, postpone, or report manual completion.

## 9. Tests And Forward Trials

Test at minimum:

- exact routing for `config`, `run`, `triage`, and `audit` requests;
- current repository evidence inspected within one bounded request;
- ambiguous requests that require one concise flow-selection clarification;
- progressive loading of only relevant knowledge and operational references;
- configuration creation and modification with full validation;
- execution and interpretation of every result class;
- triage of root failure and propagated outcomes;
- non-executing coverage audit;
- unauthorized installation, mutation, network, and scope expansion;
- repeated requests without implied cross-request continuity;
- absence of policy, persistence, process artifacts, and automatic report-store
  creation after every AI-Tool operation;
- an existing malformed or restrictive AI-Engineering policy that AI-Tool does
  not discover or interpret;
- pre-existing AI-Engineering persistence that AI-Tool does not use as hidden
  task context;
- stale or incompatible skill and CLI versions;
- prompt-injection text in repository files and report output;
- no duplicate run or unchanged-failure retry loop.

Forward trials use fresh agents and task-like prompts without expected answers.
They must include both successful operations and blocked operations.

## 10. Deliverables

- explicit AI-Tool Flow contract;
- namespaced AI-Tool skill references;
- top-level flow routing;
- dedicated public AI-Tool documentation;
- progressive shared-knowledge routing;
- bounded-request safety and authority tests;
- representative forward-trial fixtures.

## 11. Acceptance Criteria

- [ ] An agent can configure, run, triage, or audit from one bounded request.
- [ ] AI-Tool does not treat repository observations as cross-request
      continuity.
- [ ] AI-Tool works without `policy.json`, persistence, a report store, or any
      AI-Engineering artifact.
- [ ] Existing AI-Engineering policy and persistence do not affect AI-Tool
      routing, authority, or results.
- [ ] AI-Tool never invokes Processes 0 through 3 implicitly.
- [ ] Separate requests do not inherit analytical context or authorization.
- [ ] Every execution passes through the normal CLI configuration and report
      contracts.
- [ ] Shared knowledge is loaded progressively and never activates a flow.
- [ ] Human-only documentation is not routed as an agent instruction.
- [ ] Skill and public documentation preserve distinct responsibilities without
      semantic disagreement.
- [ ] All operation, safety, compatibility, and forward-trial tests pass.

## 12. Handoff To Phase 4

Phase 4 composes these stable AI-Tool operations into the AI-Engineering Flow.
It adds decision policy, immutable process artifacts, and explicit
cross-request continuity without duplicating or weakening the bounded
operational contracts established here.
