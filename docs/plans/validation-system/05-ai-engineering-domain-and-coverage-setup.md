# Phase 5: AI-Engineering Domain And Coverage Setup

## 1. Objective

Implement the two setup processes of the AI-Engineering Flow:

1. domain understanding, which records what the workspace is and which failures
   matter;
2. validation design, which maps required evidence to the executable
   configuration used by all three flows.

These processes run at onboarding, after a domain-relevant change, when the
human directs the agent to reassess them, or after a drift assessment recommends
recalibration. They do not run before every normal validation.

## 2. Dependency

Phase 4 provides safe automatic persistence, authorization boundaries, and
AI-Engineering routing contracts. Phase 3 provides the AI-Tool operations that
these processes compose. Phase 2 provides the source-backed validation model
used to reason about evidence.

## 3. Process 0: Domain

### 3.1 Inputs

The agent begins with:

- workspace instructions and declared product documentation;
- repository manifests and lockfiles;
- a bounded directory inventory;
- the validated `.validation/policy.json` and its digest;
- the current `.validation/config.json`, when present;
- build, packaging, persistence, deployment, and release documentation relevant
  to the product;
- current domain state, when recalibrating;
- answers provided by the human operator.

The initial directory inventory is discovery evidence, not a fixed depth limit.
The agent follows only paths needed to substantiate a conclusion.

### 3.2 Domain Payload

The domain artifact records:

- product purpose and deployment model;
- supported platforms and execution environments;
- important user and operator groups;
- data classes and external systems;
- assets that require protection;
- stated legal, contractual, accessibility, or domain obligations;
- business, safety, privacy, security, integrity, and availability consequences;
- invariants with repository or human evidence;
- uncertainty, missing evidence, and questions requiring human judgment;
- applicable source-backed validation concerns from Phase 2.

Do not reduce the workspace to a universal project-level risk label. Risk
reasoning records the concrete consequence, likelihood basis, affected asset,
and uncertainty. Domain-specific external levels retain their native standard,
version, and scope.

### 3.3 Process Completion And Human Input

When repository evidence cannot establish a necessary domain decision, request
that input from the human and include the answer as human-provided evidence.
When the process completes:

1. write the immutable domain artifact;
2. validate its shape, references, bounds, and digest;
3. atomically update `state.json` so the new domain becomes current and the
   sensorium state becomes `missing` when no sensorium exists, or
   `revalidation_required` when a current sensorium is bound to the preceding
   domain;
4. present the product and deployment model, highest-consequence failure
   scenarios, invariants, unresolved questions, weakly supported assumptions,
   artifact path, and digest;
5. apply `processContinuation`: start Process 1 for `auto`, suggest it and wait
   for a free-text decision for `human`, or end without a continuation prompt
   for `stop`.

No separate authorization is required to write the artifact or update the
pointer. If the human corrects, constrains, or supplements the result, rerun the
process with that input, write a new artifact, and update the pointer to the new
validated result. Do not begin Process 1 merely because Process 0 completed or
its state update succeeded: continuation must resolve to `auto` or receive an
accepting human response, and every Process 1 precondition must hold. A
declined, postponed, or `stop` handoff leaves Process 0 complete and allows
Process 1 to be invoked later from any work context.

Every replacement of the current domain reference invalidates the current
sensorium, even when the preceding sensorium file remains available for audit.
The transition and domain pointer update are one atomic state operation, so no
observable state can pair a new domain with a sensorium validated for the
preceding domain.

Any `operational-state.json` linked to the preceding domain or sensorium becomes
stale by reference mismatch. It remains local audit context and cannot be
resumed. After Process 1, a later operational process starts with a new
operation and report linkage.

## 4. Process 1: Sensorium

`sensorium` means the AI's current validation-design memory. It is not a list
of commands consumed by the CLI.

### 4.1 Inputs

- current domain artifact;
- validated AI decision policy and exact digest;
- current sensorium lifecycle state and any revalidation marker;
- current config and its digest;
- non-executing config inspection from `config validate`, `list --tree`, and
  `explain`;
- relevant canonical guidance and source register entries;
- current manifests, scripts, and delivery workflows;
- explicit constraints from the human.

### 4.2 Evidence Matrix

For every relevant risk or invariant, record:

- required evidence;
- applicable validation dimension or test type;
- existing config group, suite, and check IDs that produce that evidence;
- known limitations and residual gaps;
- expected cost and suitable execution context;
- prerequisites and mutation behavior;
- source IDs supporting the recommendation;
- justified overlap with other evidence.

Model evidence dimensions independently. Partial overlap is valid when two
checks establish complementary evidence or defend against distinct failure
modes. Unnecessary duplication is reported as a cost concern.

### 4.3 Sensorium Payload

The sensorium artifact records:

- current domain artifact ID and digest;
- analyzed config digest;
- covered risks and invariants;
- selected config IDs and their evidence roles;
- uncovered or weakly covered evidence;
- proposed config changes as structured recommendations;
- rejected alternatives and rationale when relevant;
- prerequisites requiring human action;
- residual risk and uncertainty;
- skill and guidance versions used for the analysis.

It does not contain shell expressions or an alternate executable command
graph. Human-readable command examples may appear only as explanatory evidence
and are never executed from the artifact.

## 5. Configuration Reconciliation

When the sensorium identifies a coherent config change:

1. load the existing `ai-tool/config.md` operation;
2. produce the smallest complete config diff;
3. preserve `tool -> check -> suite -> group` ownership;
4. represent programs and arguments structurally without an implicit shell;
5. classify the complete proposal under every applicable AI decision-policy
   category;
6. obtain a free-text human decision when the combined result is `human`, stop
   when it is `stop`, or proceed when it is `auto`;
7. apply only the exact authorized or automatically eligible action set;
8. run `config validate` when command execution is authorized by the same
   policy evaluation;
9. inspect affected groups, suites, and checks with `explain`;
10. evaluate the resolved first execution separately when its command,
    dependency, network, or other effects differ from the eligible or approved
    edit;
11. after the effective configuration is validated, write a new sensorium
    artifact containing the exact current domain and config digests, then
    atomically set the sensorium state to `current` and clear any revalidation
    marker.

When a recommendation requires a human decision and is declined, constrained,
postponed, or completed manually, record the decision and resulting gap. Do not
repeatedly ask about the same exact action under the same state and policy
revision.

Process 1 always finishes by writing a sensorium artifact for the exact current
domain and effective config, whether the config remains unchanged, an approved
or automatically eligible change is applied, or a proposed change is declined
and retained as a gap. Only after that artifact validates may the process
atomically set sensorium state to `current` and clear the revalidation marker.

## 6. Skill Behavior

### `ai-engineering/process-0-domain.md`

- loads source policy, validation strategy, and only relevant domain guidance;
- gathers evidence before conclusions;
- separates repository evidence, human declarations, and inference;
- writes the completed domain artifact and updates current state automatically;
- presents uncertainty and questions requiring human domain judgment;
- applies `processContinuation` only after Process 0 is persisted and Process 1
  preconditions are validated.

### `ai-engineering/process-1-sensorium.md`

- verifies the current domain artifact;
- treats `missing` and `revalidation_required` as mandatory Process 1 work
  rather than accepting the preceding sensorium as current;
- audits the current config without running checks;
- loads only relevant language, framework, technology, concern, tool, and
  recipe guides;
- constructs the evidence matrix and proposed config diff;
- delegates configuration mechanics to `ai-tool/config.md`;
- delegates every effect classification to the normative AI decision-policy
  contract;
- writes the sensorium result and updates current state automatically;
- requests a free-text decision exactly when the restrictive policy result is
  `human`.

## 7. Chat Contract

Default responses remain concise but are not constrained to an exact line
count. Each setup response includes:

- conclusion and confidence;
- persisted artifact path;
- unresolved decisions or policy-required free-text request;
- next valid action.

Safety warnings, material uncertainty, and partial approvals are never omitted
to preserve formatting.

## 8. Tests And Forward Trials

Create representative fixtures for:

- a Rust library;
- a heterogeneous Rust and TypeScript workspace;
- a Svelte and Tauri application;
- a Rails application;
- a structured-data pipeline;
- a repository with incomplete documentation;
- a repository whose README contains instruction-like hostile text;
- an existing adequate config;
- an incomplete config;
- a config with redundant checks;
- a proposed tool that is unavailable or unapproved;
- policies exercising every decision level relevant to configuration,
  dependencies, commands, network access, and workspace mutation;
- a replaced domain with an otherwise valid preceding sensorium.

Verify that the agent:

- asks rather than inventing missing product obligations;
- does not convert inferred risk into an unsupported universal level;
- maps decisions to real config IDs;
- never places `&&`, pipes, redirection, or substitutions into an implicit
  command string;
- never installs or changes a recommended tool unless the exact effective
  policy result and current authority permit it;
- preserves a declined gap without silently weakening it;
- saves each process result without a separate persistence prompt;
- implements `auto`, `human`, and `stop` continuation without bypassing Process
  1 preconditions;
- can resume from current state in a fresh session;
- cannot enter the operational flow after a domain replacement until Process 1
  binds a new sensorium to that domain.

## 9. Acceptance Criteria

- [ ] Domain state distinguishes fact, human declaration, inference, and
      uncertainty.
- [ ] Sensorium state describes evidence and maps it to real config IDs.
- [ ] The sensorium is never an executable input to the CLI.
- [ ] Config changes follow the AI-Tool config operation and the exact
      category-level AI decision policy.
- [ ] Domain and sensorium results are persisted automatically when their
      processes complete.
- [ ] Process 0 applies `processContinuation` only after persistence and valid
      Process 1 preconditions.
- [ ] The current sensorium references the exact config digest it analyzed.
- [ ] Replacing the current domain atomically marks any preceding sensorium as
      requiring Process 1 revalidation.
- [ ] Process 1 clears the revalidation marker only after persisting a sensorium
      bound to the exact current domain and config digests.
- [ ] Tool overlap is evaluated by evidence rather than prohibited by rule.
- [ ] A fresh agent can resume setup from state without reading old chat logs.

## 10. Handoff To Phase 6

Phase 6 starts after one current domain artifact, one sensorium whose lifecycle
state is `current`, and the exact linked config have been validated. The daily
loop consumes those references but does not reinterpret setup from scratch.
