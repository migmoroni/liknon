# Phase 3: Official Action Workflow And AI Decision Policy

## 1. Objective

Implement the official action workflow as bounded, point-in-time assistance
over direct human operation. The workflow selects focused AI-Tool skill
operations and shared validation knowledge to help with AI decision policy,
validation configuration, execution, triage, or audit without activating the
validation-engineering workflow or creating analytical artifacts.

This phase also establishes the consumer-owned AI decision policy used by every
AI operation. The policy uses global values and optional exact-workflow
overrides to limit every compatible workflow. It never affects direct human use
of the CLI or the validation execution graph.

The program-owned AI bootstrap contract is already established by A01 and
exposed by B03. This phase does not create a host or copy that contract into a
resource. It makes the official action workflow and AI-Tool declare and obey the
same compatible bootstrap and policy contracts used by any third-party
workflow.

The action workflow is independently useful. Its AI-Tool skill is also an
ordinary dependency reusable by the validation-engineering workflow and other
compatible workflows.

AI-Tool is a model-neutral operational skill for agents. A human may select the
action workflow through a natural task prompt, an external compatible workflow
may reference one of its operations, and validation engineering consumes the
same operation contracts. Both resources remain optional: humans and external
agents may use the public CLI and documentation directly without claiming their
behavior.

AI-Tool is the maintained official default for teaching agents how to apply the
Liknon CLIs, but it is not mandatory platform code. A compatible workflow may
select another operational skill or derivative under the same bootstrap,
policy, capability, effect, and resource rules.

## 2. Dependencies

- Phase 1 provides stable CLI, initialization, configuration, report, and Human
  Flow contracts.
- Phase 2 provides the canonical knowledge model and initial corpus.
- Phase 2.1 provides the standards-informed editorial architecture, canonical
  family paths, stable profile contracts, and version-neutral corpus consumed
  through that interface.
- A01 separates authored modular resources from the three applications.
- B02 provides explicit resource acquisition and organization through
  `liknon-manager` / `likm`.
- B03 provides offline, read-only access to installed resources through
  `liknon-tool` / `likt`.
- A01 and B03 provide the program-owned AI bootstrap contract, its compatibility
  types, and exact `likt bootstrap` projection.
- The root plan's M3 local interoperability gate is complete with B02.4 Manager,
  B03.2 Tool, and neutral resources. Public B02.5/B03.3-B03.4 distribution is
  independent and is not a prerequisite for this phase.

## 3. Workflow Contract

For `config`, `run`, `triage`, and `audit`, the supported flow is:

```text
human requests one bounded task
  -> human directs the agent to the canonical Liknon AI bootstrap entry point
  -> agent records the supported bootstrap version and digest
  -> agent selects the official action workflow
  -> agent uses likt to resolve the exact graph and dependency-closure metadata
  -> workflow selects one declared AI-Tool operation
  -> agent verifies workflow, application, and resource compatibility
  -> agent loads the declared immutable policy-boundary guide
  -> agent validates policy and evaluates the exact active workflow stack
  -> missing or invalid policy routes to policy guidance and stops the task
  -> agent uses likt to load one operational reference and relevant knowledge
  -> agent classifies every proposed effect under the valid policy
  -> agent inspects or acts within the resulting authority
  -> liknon remains the only validation executor
  -> agent interprets the canonical result
  -> agent returns the bounded result
```

The `policy` operation has a deliberately narrower bootstrap flow:

```text
human asks for policy guidance
  -> agent loads and verifies the Liknon AI bootstrap contract
  -> agent loads the immutable policy boundary
  -> agent may run read-only policy validation
  -> agent explains the contract and current diagnostics
  -> agent recommends category values with consequences and rationale
  -> human creates or edits policy.json manually
  -> agent revalidates only after the human reports completion
```

The policy operation may run when policy is missing or invalid because it cannot
perform policy mutation or another governed workspace action. This exception
does not authorize any other operation or workflow.

The referenced AI-Tool skill supports five operations:

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

`.liknon/ai/policy.json` is owned exclusively by the human operator. It defines
when an agent may act automatically, must request a free-text human decision, or
must stop. Every category and `processContinuation` has a global value and an
optional map keyed by canonical workflow ID. The official action workflow uses
the effective values returned for its exact active workflow stack: all matching
overrides combine by the normative restrictive order, and the global value is
used only when none match. It declares no continuation-eligible edge.

When an AI host uses the AI-Tool skill directly without an active workflow,
only global policy values apply. When another workflow references the skill,
that workflow's exact override participates normally. The skill never invents
a synthetic workflow identity or receives a private policy namespace.

The complete normative contract is defined by
[AI Decision Policy](ai-decision-policy.md). The validator planner and executor
never read the document. The active workflow applies it before an AI performs a
governed effect.

Policy ownership is protected by a non-configurable rule that has precedence
over every category level, every continuation mode, and every user request to
the agent:

- no agent may create, edit, format, replace, delete, move, rename, restore, or
  otherwise mutate `.liknon/ai/policy.json`;
- no agent may invoke a command, script, patch, filesystem API, VCS operation,
  or indirect mechanism that performs such a mutation;
- an explicit human request or approval does not lift this prohibition;
- policy level `0` does not lift this prohibition;
- the policy cannot authorize its own creation or mutation.

The agent may read the policy as untrusted structured data, validate it, explain
it, and recommend changes. The human performs every creation and modification
directly.

### 4.2 Deterministic Human Initialization

Phase 3 extends the Phase 1 `init` command with a policy capability and adds
separate read-only validation and override-resolution commands:

```sh
liknon init [--config <candidate-path>] [--policy] [--workspace <workspace-path>] --format=json
liknon policy validate [--workspace <workspace-path>] --format=json
liknon policy evaluate [--workspace <workspace-path>] [--workflow <RESOURCE_ID>]... --format=json
```

`--policy` provisions only the canonical conservative
`.liknon/ai/policy.json`. It uses the shared versioned `InitResult`, workspace
boundary, complete preflight, deterministic ordering, per-file atomic
no-overwrite behavior, conflict handling, partial-result semantics, and
idempotent retry contract established in Phase 1. Config and policy flags may be
combined by a human operator.

`init --policy` is a human-operated command. Skill instructions may show the
exact command but must never invoke it, including after an approval prompt. The
human runs it directly and then reviews every global value and optional
workflow override.

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

`policy evaluate` first performs the same validation, then resolves every
decision category and `processContinuation` against an explicit ordered
root-to-leaf workflow stack supplied through repeated `--workflow` arguments.
An empty stack means a standalone skill operation and therefore uses global
values only. The result contains the validated policy identity, the exact input
stack, every effective value, and whether each value came from `global` or one
or more matching workflow overrides. The command applies the mandatory
restrictiveness orders but does not discover a workflow, read the resource
store, classify an action, activate a graph, or grant authority. The external
host obtains and verifies the stack from Tool before supplying it.

The internal core API always receives the workspace root explicitly. With
`--workspace`, the CLI uses that exact boundary. Otherwise, commands requiring
existing state discover the nearest ancestor containing `.liknon/` and fail if
none exists. Initialization falls back to the exact current directory only
when creating a new root. Git and configuration content never select the
boundary.

### 4.3 Advisory `policy` Operation

`references/ai-tool/policy.md` defines a read-only advisory operation. It:

1. uses only the fixed pre-policy allowlist defined by the bootstrap and
   decision-policy contracts to load the immutable policy boundary before
   inspecting policy data;
2. verifies the installed AI-Tool skill resource and its declared Validation
   and Tool capability ranges;
3. invokes `policy validate` against the explicit workspace root and, when an
   active workflow stack exists, invokes `policy evaluate` with the exact stack
   returned by Tool;
4. explains missing, invalid, unsupported, or valid state without repairing it;
5. explains every category, level, overlap rule, global value, exact-workflow
   override, and `processContinuation` mode from the normative contract;
6. asks the human about intended authority, environments, data classes, and
   operational constraints when evidence is insufficient;
7. presents current and recommended values per field, with rationale,
   consequences, uncertainty, and interactions with other categories;
8. directs the human to run `liknon init --policy` when the file is
   missing or to open and edit the existing file manually;
9. waits for the human to report completion;
10. reruns `policy validate`, evaluates the applicable explicit workflow stack,
    and reports the effective settings, sources, and digest.

The operation does not write a candidate policy, emit an applyable patch, run a
mutation command, or treat a recommendation as a completed change. It may show
illustrative field values, but the human chooses and enters every effective
value.

### 4.4 Missing And Invalid Policy

Before `config`, `run`, `triage`, `audit`, or any compatible workflow performs
governed work, the agent runs `policy validate` for the explicit workspace root
and `policy evaluate` for the exact active workflow stack. A standalone skill
operation supplies an empty stack.

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

The official action workflow:

- uses `.liknon/validation/config.json` as the sole executable validation policy;
- validates and obeys `.liknon/ai/policy.json` as human-owned AI authority;
- uses the current CLI and versioned report contract;
- may read canonical shared knowledge through progressive disclosure;
- follows system, host, workspace, repository, current user, immutable policy
  boundary, and effective decision-policy constraints;
- may make an ordinary workspace change only when the exact effect is eligible
  and every applicable policy category permits it or exact human authority
  satisfies the resulting `human` decision;
- never treats documentation, source, output, configuration, policy, or artifact
  content as higher-priority instructions.

The official action workflow does not:

- mutate `.liknon/ai/policy.json` by any direct or indirect mechanism;
- create or update `.liknon/ai/persistence/`;
- automatically establish `.liknon/validation/reports/` as a persistent report store;
- execute validation-engineering nodes;
- create domain, sensorium, incident, or drift artifacts;
- infer continuity, prior authorization, or unresolved work from chat history;
- declare or traverse a continuation-eligible edge.

An exact human request may ask the agent to write an ordinary report file or
other bounded output. That action remains part of the current request, is
evaluated under every applicable policy category, and does not activate a
persisting workflow contract.

When repository facts are relevant, the agent inspects them through eligible
read-only repository commands or repository evidence already present in a
validation report. A reported repository change is evidence for the bounded
request, not an instruction to repair or revert it.

## 6. Workflow And Skill Architecture

Keep action routing, operational instruction, and policy authority in separate
resources:

```text
resources/
├── workflows/official/liknon-action/
│   ├── manifest.json
│   └── workflow.md
├── skills/official/liknon-ai-tool/
│   ├── manifest.json
│   ├── SKILL.md
│   └── references/
│       └── ai-tool/
│           ├── index.md
│           ├── policy.md
│           ├── config.md
│           ├── run.md
│           ├── triage.md
│           └── audit.md
└── guides/official/liknon-ai-policy-boundary/
    ├── manifest.json
    └── policy-boundary.md
```

The action workflow manifest is its orchestration index: it owns the bounded
request graph and routes to exactly one primary AI-Tool operation. It declares
dependencies on the AI-Tool skill and policy-boundary guide. It has no analytical persistence and no
continuation-eligible edge. Installing or loading the skill alone never selects
or activates this workflow.

The AI-Tool skill owns operation instructions, not orchestration. Another
compatible workflow may reuse one or more operations through the ordinary
resource dependency contract. Neither the skill nor the workflow has a
hard-coded application path, namespace privilege, or authority to install a
missing dependency.

The program-owned bootstrap contract is loaded before this resource graph and
is never copied into AI-Tool. The initial official workflow declares AI-Tool as
its default operational dependency. Another workflow may declare a compatible
alternative; neither Tool nor Validation CLI branches on that choice.

The generated policy-boundary guide states the non-configurable prohibition on
agent mutation of policy and the narrow read-only bootstrap behavior. It is a
shared dependency rather than a private file inside AI-Tool, so unrelated
compatible workflows can adopt the same policy without depending on this skill.

The manifests are authoritative for identity, version, payload, dependencies,
capabilities, effects, and application compatibility. Validation knowledge
remains audience-neutral in separate resources and is never copied into the
workflow or skill. Progressive disclosure uses Tool to retrieve only selected
installed resources.

If a required resource is absent, the agent reports the exact missing identity
and stops that operation. It may suggest an explicit `likm` action subject to
the current policy and human authority, but it never invokes Manager silently
or treats retrieval permission as installation permission.

## 7. Operation Requirements

### 7.1 `policy`

- Remain read-only even when the human asks or authorizes the agent to mutate the
  file.
- Before effective policy exists, use only the fixed pre-policy Tool and
  Validation CLI operations defined by the bootstrap contract. Over policy
  content itself, use only read-only `policy validate` and `policy evaluate`.
- Explain the complete current policy and every recommendation without deciding
  human risk acceptance.
- Present recommendations per category rather than silently constructing a
  replacement document.
- Require the human to initialize or edit the file manually.
- Revalidate only after the human reports that the manual action is complete.
- Never activate the validation-engineering workflow or create persistence as a
  consequence of policy guidance.

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
  `liknon init --config <candidate-path>` only when every applicable
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
- Do not persist a validation-engineering domain or sensorium model.
- Do not execute proposed validation merely because the audit recommends it.

## 8. Public Documentation

Create dedicated public entry points:

```text
docs/validation/
├── reference/
│   └── ai-decision-policy.md
└── flows/
    └── action/
        ├── README.md
        ├── policy.md
        ├── config.md
        ├── run.md
        ├── triage.md
        └── audit.md
```

The public documents explain the workflow, its boundaries, and how a human invokes
each operation. They are not a second copy of the skill instructions. Public
documentation describes supported behavior; skill references define how an
agent performs it.

The normative policy reference defines the exact shape, category levels,
continuation modes, restrictive combination, generic persistence-decision
rules, fixed report-storage facts, and absolute policy-mutation prohibition.
Workflow-specific artifact declarations remain in their locked workflow
resources. The shared policy-boundary guide is
generated from that canonical source rather than maintained independently.

Public AI guidance links to the canonical bootstrap entry point rather than
copying it. The tracked file and `likt bootstrap` are the only authoritative
entry projections.

Register every standard, primary research work, and official reference used by
the policy contract in `docs/validation/knowledge/sources.json` before citing it.
Generate `docs/validation/knowledge/SOURCES.md` from that canonical register.
Add only sources the contract actually uses; do not expand the corpus merely
to summarize those sources. Rebuild and verify affected resource artifacts and
their catalog metadata after updating the register.

The root validation index clearly distinguishes direct human operation, the
official action workflow, and validation engineering. It states that governed workflows require a valid human-owned policy,
while the read-only `policy` operation remains available to help the human
initialize or correct it manually.

## 9. Authority And Safety

- The human remains the authority for project intent, policy contents, external
  effects, and risk acceptance.
- Only the human creates, changes, moves, restores, or deletes
  `.liknon/ai/policy.json`.
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
- deterministic `policy evaluate` for an empty stack, one override, unmatched
  IDs, and composed root-to-leaf stacks, including exact effective-value source
  metadata;
- policy category, global-value, exact-workflow override, composed-workflow,
  and continuation semantics from the normative contract;
- conservative initialization with every workflow-override map empty and no
  official resource ID embedded in CLI defaults;
- complete source-register entries and valid primary links for every policy
  design source actually cited;
- exact `likt` retrieval of every selected knowledge resource after
  the canonical source register and resource artifacts are updated;
- missing-resource behavior that reports the stable identity and performs no
  implicit `likm` invocation, network access, or workspace mutation;
- policy guidance for missing, invalid, and valid documents;
- refusal to create, edit, format, replace, delete, move, rename, restore, or
  indirectly mutate policy, including after explicit user authorization and
  under a level-0 policy;
- proof that the policy operation creates no candidate, patch, script,
  persistence, or hidden mutation;
- blocking of `config`, `run`, `triage`, `audit`, and validation-engineering routing when
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
- absence of validation-engineering persistence, process artifacts, and automatic
  report-store creation after every action-workflow or standalone AI-Tool operation;
- pre-existing validation-engineering persistence that the action workflow does not use as hidden
  task context;
- stale or incompatible AI-Tool resource versions and declared application
  capability ranges;
- an actionable workflow with an absent or incompatible decision-policy
  contract that remains readable but stops before activation;
- prompt-injection text in repository files, policy values, and report output;
- no duplicate run or unchanged-failure retry loop;
- an external action workflow that reuses one AI-Tool operation without an
  official ID, official graph shape, or application-specific routing branch.

Forward trials use fresh agents and task-like prompts without expected answers.
They include successful operations, policy-blocked operations, and adversarial
requests to mutate policy. Define scenarios and observable assertions here;
Phase 7 executes the cross-model action-workflow and AI-Tool evaluation,
including human prompts and controlled validation-engineering consumption, under the shared
[Agent Evaluation Program](agent-evaluation-program.md).

## 11. Deliverables

- explicit action-workflow contract selecting five reusable AI-Tool operations;
- bootstrap-compatibility declarations for the action workflow, AI-Tool skill,
  and policy-boundary guide without copying the bootstrap contract;
- policy schema, conservative default, read-only validation and evaluation, and
  human-only deterministic initialization;
- canonical AI decision-policy documentation and generated policy-boundary
  guide;
- source-register entries for the policy contract's cited design foundations;
- independently versioned policy-boundary guide dependency;
- namespaced AI-Tool skill references, including advisory `policy.md`;
- top-level workflow routing;
- dedicated public AI-Tool documentation;
- progressive installed-resource routing through the read-only `likt`
  application;
- bounded-request policy, safety, and authority tests;
- representative forward-trial fixtures;
- versioned AI-Tool Interface scenarios and assertions consumable by the final
  agent-evaluation matrix.

## 12. Acceptance Criteria

- [ ] An agent can guide policy, configure, run, triage, or audit from one
      bounded request.
- [ ] Every governed action-workflow and standalone AI-Tool trial begins from
      the exact supported bootstrap contract and records its version and digest.
- [ ] `policy.json` is owned and mutated only by the human operator.
- [ ] No AI operation can create, alter, replace, remove, move, restore, or
      indirectly mutate policy, regardless of human approval or configured
      level.
- [ ] The advisory policy operation works when policy is missing or invalid and
      performs no governed mutation.
- [ ] Every other governed operation or workflow requires a valid policy
      before work begins.
- [ ] Every actionable workflow declares a compatible AI decision-policy
      contract before a conforming host activates it.
- [ ] Every actionable workflow and governed operational skill declares a
      compatible AI bootstrap contract before a conforming host uses it.
- [ ] The action workflow validates and obeys the same global, override, and
      category meanings used by every compatible workflow.
- [ ] `policy validate` reports document validity and identity without inventing
      workflow context; `policy evaluate` resolves only the exact ordered stack
      supplied by the caller and never discovers or activates a workflow.
- [ ] `init --policy` emits only global defaults and empty override maps; a
      human adds every workflow-specific rule manually.
- [ ] The action workflow has no continuation-eligible transition; AI-Tool may
      explain `processContinuation` without consuming it.
- [ ] AI-Tool does not treat repository observations as cross-request
      continuity.
- [ ] The action workflow never invokes validation-engineering nodes implicitly.
- [ ] Separate requests do not inherit analytical context or authorization.
- [ ] Every execution passes through the normal CLI configuration and report
      contracts.
- [ ] The validation planner and executor never read or interpret policy.
- [ ] Shared knowledge is loaded progressively and never activates a flow.
- [ ] The skill contains no copied shared-knowledge subtree and retrieves
      selected knowledge only through `likt`.
- [ ] A fixture workflow can select a compatible non-official AI-Tool
      alternative without changing application or bootstrap code.
- [ ] A missing resource never causes implicit Manager use, network access, or
      installation.
- [ ] Human-only documentation is not routed as an agent instruction.
- [ ] Skill and public documentation preserve distinct responsibilities without
      semantic disagreement.
- [ ] Human-prompt and downstream-agent scenarios expose enough observable
      evidence to evaluate AI-Tool independently from validation engineering.
- [ ] All operation, policy, safety, compatibility, and forward-trial tests pass.

## 13. Handoff To Phase 4

Phase 4 composes these stable policy-governed AI-Tool operations and separate
agent profiles into the official validation-engineering workflow. It adds
immutable process artifacts and explicit
cross-request continuity without duplicating policy guidance or weakening the
human-only policy-mutation boundary.
