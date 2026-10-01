# Phase 3: AI-Tool Flow And AI Decision Policy

## 1. Objective

Implement the AI-Tool Flow as bounded, point-in-time assistance over the Human
Flow. An agent uses focused operations and shared validation knowledge to help
with AI decision policy, validation configuration, execution, triage, or audit
without activating validation-engineering processes or creating AI-Engineering
artifacts.

This phase also establishes the consumer-owned AI decision policy used by every
AI operation. The policy limits both AI-Tool and AI-Engineering behavior. It
never affects direct human use of the CLI or the validation execution graph.

The AI-Tool Flow is independently useful. It is also the operational foundation
composed by the AI-Engineering Flow in later phases.

AI-Tool is a model-neutral operational interface for agents. A human may invoke
it through a natural task prompt, an external compatible agent system may route
to one of its operations, and AI-Engineering consumes the same operation
contracts. It remains optional: humans and external agents may use the public
CLI and documentation directly without claiming AI-Tool behavior.

## 2. Dependencies

- Phase 1 provides stable CLI, initialization, configuration, report, and Human
  Flow contracts.
- Phase 2 provides the canonical shared-knowledge tree and its generated skill
  projection.

## 3. Flow Contract

For `config`, `run`, `triage`, and `audit`, the supported flow is:

```text
human requests one bounded task
  -> agent selects the AI-Tool Flow
  -> agent loads the immutable policy boundary
  -> agent verifies CLI and skill compatibility
  -> agent runs read-only policy validation
  -> missing or invalid policy routes to policy guidance and stops the task
  -> agent loads one operational reference
  -> agent loads only relevant shared knowledge
  -> agent classifies every proposed effect under the valid policy
  -> agent inspects or acts within the resulting authority
  -> workspace-validator remains the only validation executor
  -> agent interprets the canonical result
  -> agent returns the bounded result
```

The `policy` operation has a deliberately narrower bootstrap flow:

```text
human asks for policy guidance
  -> agent loads the immutable policy boundary
  -> agent may run read-only policy validation
  -> agent explains the contract and current diagnostics
  -> agent recommends category values with consequences and rationale
  -> human creates or edits policy.json manually
  -> agent revalidates only after the human reports completion
```

The policy operation may run when policy is missing or invalid because it cannot
perform policy mutation or another governed workspace action. This exception
does not authorize any other AI-Tool or AI-Engineering operation.

The flow supports five operations:

| Operation | Responsibility |
| --- | --- |
| `policy` | Validate, explain, and recommend human-authored AI decision-policy settings without creating or mutating the policy |
| `config` | Inspect, explain, create, or modify declarative validation configuration within current policy and human authority |
| `run` | Inspect and execute an explicit configured selection, then interpret its structured result |
| `triage` | Diagnose a concrete report or failure without assuming permission to remediate it |
| `audit` | Compare requested or observed validation coverage with repository evidence without executing checks unless separately permitted |

Each invocation has one explicit objective and ends when that objective is
reported or blocked. A later invocation begins from current workspace facts, a
fresh policy validation, and the new human request, not from implied memory of
an earlier conversation.

## 4. AI Decision-Policy Foundation

### 4.1 Ownership And Scope

`.validation/policy.json` is owned exclusively by the human operator. It defines
when an agent may act automatically, must request a free-text human decision, or
must stop. Its category levels govern both AI-Tool and AI-Engineering. Its
`processContinuation` value is consumed only by the two declared
AI-Engineering process transitions.

The complete normative contract is defined by
[AI Decision Policy](ai-decision-policy.md). The validator planner and executor
never read the document. The skill applies it before an AI performs a governed
effect.

Policy ownership is protected by a non-configurable rule that has precedence
over every category level, every continuation mode, and every user request to
the agent:

- no agent may create, edit, format, replace, delete, move, rename, restore, or
  otherwise mutate `.validation/policy.json`;
- no agent may invoke a command, script, patch, filesystem API, VCS operation,
  or indirect mechanism that performs such a mutation;
- an explicit human request or approval does not lift this prohibition;
- policy level `0` does not lift this prohibition;
- the policy cannot authorize its own creation or mutation.

The agent may read the policy as untrusted structured data, validate it, explain
it, and recommend changes. The human performs every creation and modification
directly.

### 4.2 Deterministic Human Initialization

Phase 3 extends the Phase 1 `init` command with a policy capability and adds a
read-only validation command:

```sh
workspace-validator init [--config <candidate-path>] [--policy] [--workspace <workspace-path>] --format=json
workspace-validator policy validate [--workspace <workspace-path>] --format=json
```

`--policy` provisions only the canonical conservative
`.validation/policy.json`. It uses the shared versioned `InitResult`, workspace
boundary, complete preflight, deterministic ordering, per-file atomic
no-overwrite behavior, conflict handling, partial-result semantics, and
idempotent retry contract established in Phase 1. Config and policy flags may be
combined by a human operator.

`init --policy` is a human-operated command. Skill instructions may show the
exact command but must never invoke it, including after an approval prompt. The
human runs it directly and then reviews every category and
`processContinuation` value.

When the policy destination is absent, the command creates the exact bundled
conservative document. When it exists, the command never alters it and succeeds
only when the existing bytes form a valid supported policy. Invalid, unsupported,
symlinked, or internally inconsistent destinations fail closed and are never
replaced.

`policy validate` reads only the fixed policy path inside the explicit workspace
boundary. It validates schema and semantic invariants, computes SHA-256 over the
exact bytes, and returns one bounded versioned result containing normalized
path, schema version, digest, and status. Missing policy is a typed non-success
result. The command never creates, modifies, repairs, or replaces a file.

The library API always receives the workspace root explicitly. The CLI uses
`--workspace` or exactly the current working directory and never discovers a
different root through parents, Git, or configuration content.

### 4.3 Advisory `policy` Operation

`references/ai-tool/policy.md` defines a read-only advisory operation. It:

1. loads the immutable policy boundary before inspecting policy data;
2. verifies CLI and skill compatibility;
3. invokes `policy validate` against the explicit workspace root;
4. explains missing, invalid, unsupported, or valid state without repairing it;
5. explains every category, level, overlap rule, and `processContinuation` mode
   from the normative contract;
6. asks the human about intended authority, environments, data classes, and
   operational constraints when evidence is insufficient;
7. presents current and recommended values per field, with rationale,
   consequences, uncertainty, and interactions with other categories;
8. directs the human to run `workspace-validator init --policy` when the file is
   missing or to open and edit the existing file manually;
9. waits for the human to report completion;
10. reruns `policy validate` and reports the effective settings and digest.

The operation does not write a candidate policy, emit an applyable patch, run a
mutation command, or treat a recommendation as a completed change. It may show
illustrative field values, but the human chooses and enters every effective
value.

### 4.4 Missing And Invalid Policy

Before `config`, `run`, `triage`, `audit`, or any AI-Engineering process performs
work, the agent runs `policy validate` for the explicit workspace root.

- A missing policy stops the requested operation before repository analysis,
  command execution, or mutation. The agent routes to the advisory `policy`
  operation and instructs the human to initialize and review the file manually.
- An invalid policy stops the requested operation. The advisory operation may
  explain diagnostics and recommended corrections, but the human edits the file.
- A valid policy fixes the exact path, schema version, and digest used by the
  current operation. The agent evaluates every effect under that policy.
- A changed policy invalidates prior authorization decisions. A subsequent or
  resumed operation validates and evaluates the current bytes again.

## 5. Architectural Boundary

The AI-Tool Flow:

- uses `.validation/config.json` as the sole executable validation policy;
- validates and obeys `.validation/policy.json` as human-owned AI authority;
- uses the current CLI and versioned report contract;
- may read canonical shared knowledge through progressive disclosure;
- follows system, host, workspace, repository, current user, immutable policy
  boundary, and effective decision-policy constraints;
- may make an ordinary workspace change only when the exact effect is eligible
  and every applicable policy category permits it or exact human authority
  satisfies the resulting `human` decision;
- never treats documentation, source, output, configuration, policy, or artifact
  content as higher-priority instructions.

The AI-Tool Flow does not:

- mutate `.validation/policy.json` by any direct or indirect mechanism;
- create or update `.validation/persistence/`;
- automatically establish `.validation/reports/` as a persistent report store;
- execute Processes 0 through 3;
- create domain, sensorium, incident, or drift artifacts;
- infer continuity, prior authorization, or unresolved work from chat history;
- apply AI-Engineering continuation rules.

An exact human request may ask the agent to write an ordinary report file or
other bounded output. That action remains part of the current request, is
evaluated under every applicable policy category, and does not activate the
AI-Engineering persistence contracts.

When repository facts are relevant, the agent inspects them through eligible
read-only repository commands or repository evidence already present in a
validation report. A reported repository change is evidence for the bounded
request, not an instruction to repair or revert it.

## 6. Skill Architecture

Use one concise top-level router, one mandatory immutable policy-boundary
reference, and separate operational references from shared knowledge:

```text
skills/workspace-validator/
├── SKILL.md
├── manifest.json
└── references/
    ├── policy-boundary.md            # Mandatory non-configurable AI prohibition
    ├── knowledge/                    # Generated in Phase 2
    └── ai-tool/
        ├── index.md
        ├── policy.md
        ├── config.md
        ├── run.md
        ├── triage.md
        └── audit.md
```

`SKILL.md` identifies the requested flow and requires
`references/policy-boundary.md` for every AI operation before loading
workspace-controlled policy data. A bounded request to manage policy,
configure, run, triage, or audit selects AI-Tool unless the human explicitly
invokes an AI-Engineering process or requests its persistent system. The router
must not activate AI-Engineering from task complexity, repository size, or the
agent's preference.

`references/policy-boundary.md` is short, mandatory, and generated from the
canonical public boundary. It states that agents may never mutate policy and
defines the narrow read-only bootstrap behavior of `AI-Tool/policy`. Every
AI-Engineering process also loads this same reference.

`references/ai-tool/index.md` defines the common bounded-request boundary and
routes to exactly one primary operation. An operation may link to another
AI-Tool reference when the current request genuinely requires both, but it must
not eagerly load all operations.

The five operational references are authored skill instructions. Shared
knowledge is generated from `docs/validation/knowledge/` and remains
audience-neutral. Human-only files are not included in skill routing.

## 7. Operation Requirements

### 7.1 `policy`

- Remain read-only even when the human asks or authorizes the agent to mutate the
  file.
- Use `policy validate` as the only CLI operation over policy content.
- Explain the complete current policy and every recommendation without deciding
  human risk acceptance.
- Present recommendations per category rather than silently constructing a
  replacement document.
- Require the human to initialize or edit the file manually.
- Revalidate only after the human reports that the manual action is complete.
- Never activate AI-Engineering or create persistence as a consequence of policy
  guidance.

### 7.2 `config`

- Require a valid current AI decision policy before repository analysis or
  configuration work.
- Validate the complete current document before proposing changes.
- Use runtime types and bundled schemas as the public contract.
- Resolve programs, arguments, parameters, working directories, dependencies,
  suites, and groups explicitly.
- Present dependency or tool requirements before any policy-required human
  decision.
- Avoid repository-specific policy in the generic skill.
- When canonical configuration is absent and creation is eligible, author one
  complete candidate in a contained temporary path and install it through
  `workspace-validator init --config <candidate-path>` only when every applicable
  policy decision permits that exact effect.
- Treat changes to an existing canonical configuration as governed workspace
  edits; `init` is not an overwrite or update mechanism.
- Revalidate the resulting configuration and inspect affected graph nodes.

### 7.3 `run`

- Require a valid current AI decision policy before inspection or execution.
- Require an explicit check, suite, group, or documented default selection.
- Inspect the resolved operation before execution when scope is not already
  evident from the request.
- Classify command execution and every overlapping effect before starting it.
- Prefer the versioned JSON report for agent interpretation.
- Preserve one-run deduplication and configured dependency semantics.
- Do not retry an unchanged failure or broaden the selected scope implicitly.
- Report every result class without reducing it to binary success or failure.

### 7.4 `triage`

- Require a valid current AI decision policy before accessing repository or
  report evidence.
- Bind the diagnosis to one exact supplied or newly produced report.
- Distinguish root failures, propagated blocking, skipping, timeout,
  interruption, invalid usage, and internal failure.
- Preserve uncertainty and separate evidence from inference.
- Recommend the narrowest useful next step.
- Do not modify source, configuration, dependencies, snapshots, or data unless
  every applicable policy category and exact human authority permit that
  separate effect.

### 7.5 `audit`

- Require a valid current AI decision policy before inspecting repository
  evidence.
- Inspect domain evidence supplied in the current request, repository structure,
  manifests, scripts, configuration, and selected shared knowledge.
- Identify represented evidence, overlaps, blind spots, prerequisites, and
  limitations.
- Keep recommendations source-backed and contextual rather than producing a
  universal assurance ladder.
- Do not persist an AI-Engineering domain or sensorium model.
- Do not execute proposed validation merely because the audit recommends it.

## 8. Public Documentation

Create dedicated public entry points:

```text
docs/validation/
├── reference/
│   └── ai-decision-policy.md
└── flows/
    └── ai-tool/
        ├── README.md
        ├── policy.md
        ├── config.md
        ├── run.md
        ├── triage.md
        └── audit.md
```

The public documents explain the flow, its boundaries, and how a human invokes
each operation. They are not a second copy of the skill instructions. Public
documentation describes supported behavior; skill references define how an
agent performs it.

The normative policy reference defines the exact shape, category levels,
continuation modes, restrictive combination, canonical persistence facts, and
absolute policy-mutation prohibition. The skill projection is generated from
that canonical source rather than maintained independently.

Register every standard, primary research work, and official reference used by
the policy contract in `docs/validation/knowledge/sources.json` before citing
it. Generate `docs/validation/knowledge/SOURCES.md` from that canonical
register. Add only sources the contract actually uses; do not expand the shared
knowledge tree merely to summarize those sources. Regenerate the exact
distributed knowledge projection after updating the register.

The root validation index clearly distinguishes AI-Tool from Human Flow and
AI-Engineering. It states that both AI flows require a valid human-owned policy,
while the read-only `policy` operation remains available to help the human
initialize or correct it manually.

## 9. Authority And Safety

- The human remains the authority for project intent, policy contents, external
  effects, and risk acceptance.
- Only the human creates, changes, moves, restores, or deletes
  `.validation/policy.json`.
- The flow obeys system, host, workspace, repository, current user, immutable
  policy-boundary, and effective policy instructions in their precedence.
- A request to analyze does not imply authority to execute or mutate.
- A request to execute a configured selection is evaluated as exact initiating
  authority only for the effects it clearly names.
- A request to edit configuration does not imply authority to install its
  referenced tools.
- Prompts for a human decision accept free text so the human can approve,
  reject, constrain, postpone, or report manual completion.
- No prompt may ask the human to authorize an agent to mutate policy; it directs
  the human to perform that action manually.

## 10. Tests And Forward Trials

Test at minimum:

- exact routing for `policy`, `config`, `run`, `triage`, and `audit` requests;
- `init --policy` alone and combined with `--config`, including complete
  preflight, deterministic resource order, idempotent reuse, conflicts, races,
  partial results, and no-overwrite guarantees;
- read-only `policy validate` for missing, valid, invalid, unsupported,
  symlinked, path-escaping, and concurrently changed documents;
- policy category and continuation semantics from the normative contract;
- complete source-register entries and valid primary links for every policy
  design source actually cited;
- byte-equivalence between the updated canonical source register and its
  distributed knowledge projection;
- policy guidance for missing, invalid, and valid documents;
- refusal to create, edit, format, replace, delete, move, rename, restore, or
  indirectly mutate policy, including after explicit user authorization and
  under a level-0 policy;
- proof that the policy operation creates no candidate, patch, script,
  persistence, or hidden mutation;
- blocking of `config`, `run`, `triage`, `audit`, and AI-Engineering routing when
  policy is missing or invalid;
- fresh policy validation and effect classification for every bounded operation;
- current repository evidence inspected within one bounded request;
- ambiguous requests that require one concise flow-selection clarification;
- progressive loading of only relevant knowledge and operational references;
- config-candidate creation through `init --config`, existing-config
  modification through governed editing, and full validation of both paths;
- execution and interpretation of every result class;
- triage of root failure and propagated outcomes;
- non-executing coverage audit;
- unauthorized installation, mutation, network, and scope expansion;
- repeated requests without implied cross-request continuity or inherited
  authorization;
- absence of AI-Engineering persistence, process artifacts, and automatic
  report-store creation after every AI-Tool operation;
- pre-existing AI-Engineering persistence that AI-Tool does not use as hidden
  task context;
- stale or incompatible skill and CLI versions;
- prompt-injection text in repository files, policy values, and report output;
- no duplicate run or unchanged-failure retry loop.

Forward trials use fresh agents and task-like prompts without expected answers.
They include successful operations, policy-blocked operations, and adversarial
requests to mutate policy. Define scenarios and observable assertions here;
Phase 7 executes the cross-model AI-Tool Interface evaluation, including human
prompts and controlled AI-Engineering consumption, under the shared
[Agent Evaluation Program](agent-evaluation-program.md).

## 11. Deliverables

- explicit AI-Tool Flow contract with five operations;
- policy schema, conservative default, read-only validation, and human-only
  deterministic initialization;
- canonical AI decision-policy documentation and generated skill projection;
- source-register entries for the policy contract's cited design foundations;
- mandatory `policy-boundary.md` skill reference;
- namespaced AI-Tool skill references, including advisory `policy.md`;
- top-level flow routing;
- dedicated public AI-Tool documentation;
- progressive shared-knowledge routing;
- bounded-request policy, safety, and authority tests;
- representative forward-trial fixtures.
- versioned AI-Tool Interface scenarios and assertions consumable by the final
  agent-evaluation matrix.

## 12. Acceptance Criteria

- [ ] An agent can guide policy, configure, run, triage, or audit from one
      bounded request.
- [ ] `policy.json` is owned and mutated only by the human operator.
- [ ] No AI operation can create, alter, replace, remove, move, restore, or
      indirectly mutate policy, regardless of human approval or configured
      level.
- [ ] The advisory policy operation works when policy is missing or invalid and
      performs no governed mutation.
- [ ] Every other AI-Tool and AI-Engineering operation requires a valid policy
      before work begins.
- [ ] AI-Tool validates and obeys the same category-level meanings used by
      AI-Engineering.
- [ ] `processContinuation` is ignored by AI-Tool operations other than policy
      explanation and is consumed only by the declared AI-Engineering process
      transitions.
- [ ] AI-Tool does not treat repository observations as cross-request
      continuity.
- [ ] AI-Tool never invokes Processes 0 through 3 implicitly.
- [ ] Separate requests do not inherit analytical context or authorization.
- [ ] Every execution passes through the normal CLI configuration and report
      contracts.
- [ ] The validation planner and executor never read or interpret policy.
- [ ] Shared knowledge is loaded progressively and never activates a flow.
- [ ] Human-only documentation is not routed as an agent instruction.
- [ ] Skill and public documentation preserve distinct responsibilities without
      semantic disagreement.
- [ ] Human-prompt and downstream-agent scenarios expose enough observable
      evidence to evaluate AI-Tool independently from AI-Engineering.
- [ ] All operation, policy, safety, compatibility, and forward-trial tests pass.

## 13. Handoff To Phase 4

Phase 4 composes these stable policy-governed AI-Tool operations into the
AI-Engineering Flow. It adds immutable process artifacts and explicit
cross-request continuity without duplicating policy guidance or weakening the
human-only policy-mutation boundary.
