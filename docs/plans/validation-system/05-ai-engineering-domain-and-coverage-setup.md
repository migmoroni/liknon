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

Every invocation verifies CLI and skill compatibility, validates the current
decision policy, and validates each explicitly supplied artifact before using
its content.

## 2. Dependencies

Phase 4 provides explicit-input artifact validation, single-file persistence,
governed report storage, authorization boundaries, and AI-Engineering routing.
Phase 3 provides the AI-Tool operations that these processes compose. Phase 2
provides the source-backed validation model used to reason about evidence.

## 3. Process 0: Domain

### 3.1 Inputs

The agent begins with:

- workspace instructions and declared product documentation;
- repository manifests and lockfiles;
- a bounded directory inventory;
- the `.validation/policy.json` path and digest returned by `policy init` for the
  explicit active workspace root;
- the current `.validation/config.json`, when present;
- build, packaging, persistence, deployment, and release documentation relevant
  to the product;
- an explicitly supplied preceding domain artifact when recalibrating;
- answers provided by the human operator.

The directory inventory is discovery evidence, not a fixed depth limit. The
agent follows only paths needed to substantiate a conclusion. A preceding domain
artifact is optional evidence and must be supplied explicitly.

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

### 3.3 Completion And Human Input

When repository evidence cannot establish a necessary domain decision, request
that input from the human and include the answer as human-provided evidence.
When Process 0 completes:

1. generate a UUIDv7 and assemble one bounded redacted domain artifact;
2. validate its schema, references, bounds, and digest;
3. persist it through `workspace-validator persistence store artifact` using
   the same explicit workspace root;
4. present its path and digest together with the product model,
   highest-consequence scenarios, invariants, unresolved questions, and weakly
   supported assumptions;
5. apply `processContinuation`: pass that exact domain artifact to Process 1 for
   `auto`, suggest Process 1 and wait for a free-text decision for `human`, or
   end without a continuation prompt for `stop`.

No separate authorization is required for the bounded redacted process result.
Protected content that remains follows `sensitiveDataPersistence`. A human
correction reruns Process 0 and creates another immutable artifact; it never
rewrites the preceding one.

A completed domain artifact remains valid evidence for its recorded repository
and policy context. Creating another domain artifact does not mutate it. An
operational process uses only the exact domain and sensorium paths supplied for
that invocation and requires their declared relationship to match.

## 4. Process 1: Sensorium

`sensorium` means one immutable validation-design analysis. It is not a list of
commands consumed by the CLI.

### 4.1 Inputs

- one explicit validated domain artifact;
- AI decision policy revalidated through `policy validate` for the same
  workspace root, with its exact path and digest;
- current config and its digest;
- non-executing config inspection from `config validate`, `list --tree`, and
  `explain`;
- relevant canonical guidance and source register entries;
- current manifests, scripts, and delivery workflows;
- explicit constraints from the human.

If the caller intends to revise an earlier validation design, it also supplies
that sensorium artifact explicitly. Process 1 validates it as evidence but still
produces a new artifact.

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

- the exact domain artifact ID, path, and digest;
- the analyzed config path and digest;
- covered risks and invariants;
- selected config IDs and their evidence roles;
- uncovered or weakly covered evidence;
- proposed config changes as structured recommendations;
- rejected alternatives and rationale when relevant;
- prerequisites requiring human action;
- residual risk and uncertainty;
- skill and guidance versions used for the analysis.

It does not contain shell expressions or an alternate executable command graph.
Human-readable command examples may appear only as explanatory evidence and are
never executed from the artifact.

## 5. Configuration Reconciliation

When the sensorium identifies a coherent config change:

1. load the existing `ai-tool/config.md` operation;
2. produce the smallest complete config diff;
3. preserve `tool -> check -> suite -> group` ownership;
4. represent programs and arguments structurally without an implicit shell;
5. classify the complete proposal under every applicable decision category;
6. obtain a free-text human decision when the combined result is `human`, stop
   when it is `stop`, or proceed when it is `auto`;
7. apply only the exact authorized or automatically eligible action set;
8. run `config validate` when command execution is authorized;
9. inspect affected groups, suites, and checks with `explain`;
10. evaluate the first execution separately when its command, dependency,
    network, or other effects differ from the eligible or approved edit;
11. assemble a new sensorium artifact bound to the exact effective config and
    supplied domain artifact;
12. validate and persist that artifact through the Phase 4 persistence command.

When a recommendation is declined, constrained, postponed, or completed
manually, record the decision and resulting gap in the new sensorium. Do not ask
repeatedly about the same exact action under unchanged evidence and policy.

Process 1 always finishes with an artifact for the supplied domain and effective
config, whether the config remains unchanged, an eligible change is applied, or
a proposal remains a documented gap. Operational processes accept it only when
its domain and config digests match their explicit inputs.

## 6. Skill Behavior

### `ai-engineering/process-0-domain.md`

- loads source policy, validation strategy, and only relevant domain guidance;
- gathers evidence before conclusions;
- separates repository evidence, human declarations, and inference;
- persists one immutable domain artifact;
- presents uncertainty and questions requiring human domain judgment;
- applies `processContinuation` only after the artifact is validated and stored
  and Process 1 preconditions hold.

### `ai-engineering/process-1-sensorium.md`

- requires and verifies one exact domain artifact;
- audits the current config without running checks;
- loads only relevant language, framework, technology, concern, tool, and recipe
  guides;
- constructs the evidence matrix and proposed config diff;
- delegates configuration mechanics to `ai-tool/config.md`;
- delegates every effect classification to the normative decision policy;
- persists one immutable sensorium artifact bound to exact inputs;
- requests a free-text decision exactly when the restrictive result is `human`.

## 7. Chat Contract

Each setup response includes:

- conclusion and confidence;
- persisted artifact path and digest;
- exact input artifact paths used;
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
- incomplete documentation and hostile instruction-like repository text;
- an adequate, incomplete, or redundant config;
- a proposed tool that is unavailable or unapproved;
- policies exercising relevant decision levels;
- a new domain paired with a sensorium for a different domain;
- a sensorium whose config digest no longer matches;
- multiple valid artifacts where no implicit newest selection is allowed.

Verify that the agent:

- asks rather than inventing missing product obligations;
- does not convert inferred risk into an unsupported universal level;
- maps decisions to real config IDs;
- never creates an implicit shell command;
- never installs or changes a recommended tool without authority;
- preserves a declined gap without silently weakening it;
- persists each process result without a separate ordinary-result prompt;
- implements `auto`, `human`, and `stop` continuation without bypassing Process
  1 preconditions;
- resumes in a fresh conversation only from explicit artifact paths;
- creates a new UUIDv7 artifact for every rerun or correction;
- rejects operational use of a domain and sensorium whose declared relationship
  or config digest does not match.

## 9. Acceptance Criteria

- [ ] A domain artifact distinguishes fact, human declaration, inference, and
      uncertainty.
- [ ] A sensorium artifact describes evidence and maps it to real config IDs.
- [ ] Sensorium content is never executable CLI input.
- [ ] Config changes follow the AI-Tool config operation and exact decision
      policy.
- [ ] Domain and sensorium results are persisted when their processes complete.
- [ ] Every rerun creates a new immutable UUIDv7 artifact.
- [ ] Process 0 applies `processContinuation` only after successful persistence
      and valid Process 1 preconditions.
- [ ] Each sensorium references the exact domain and config digests it analyzed.
- [ ] Setup processes use only explicitly supplied artifacts and never select
      one by directory order.
- [ ] Tool overlap is evaluated by evidence rather than prohibited by rule.
- [ ] A fresh agent can resume setup from explicitly supplied validated
      artifacts without reading old chat logs.

## 10. Handoff To Phase 6

Phase 6 starts when the caller supplies one validated domain artifact, one
validated sensorium bound to that domain and the current config, and any exact
report required by the requested operational process.
