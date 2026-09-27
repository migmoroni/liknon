# Phase 7: End-To-End Verification And Release

## 1. Objective

Prove that Human Flow, AI-Tool Flow, and AI-Engineering Flow are independently
usable, compose only through declared contracts, and ship as coherent
versioned artifacts. Complete documentation, realistic forward trials, package
inspection, security review, and release evidence.

## 2. Dependency

Phases 1 through 6 are implemented. No acceptance test in this phase substitutes
for an incomplete contract from an earlier phase.

## 3. End-To-End Scenario Matrix

### 3.1 Human Flow Scenarios

- create and validate a configuration from documentation;
- inspect a nested group without executing it;
- initialize an integrity baseline, verify a clean workspace, inspect a changed
  workspace, and explicitly accept a new baseline;
- run a check, suite, explicit group, and default group;
- understand pass, fail, blocked, skipped, invalid, internal, and interrupted
  outcomes;
- diagnose and rerun a narrow scope manually;
- operate successfully with no skill, AI decision policy, persistence
  directory, or report store.

### 3.2 AI-Tool Flow Scenarios

- explain a report using only `ai-tool/triage.md`;
- audit coverage without running checks;
- make an authorized config edit without activating AI-Engineering state;
- reject an unauthorized installation or source change;
- recognize stale skill and incompatible CLI versions;
- complete every operation without creating `policy.json`, persistence,
  process artifacts, or an automatic report store;
- run the integrity preflight first without writing AI observations, repairing
  files, or accepting a changed baseline;
- begin a later request without inheriting hidden state or authority.

### 3.3 AI-Engineering Setup

- create and persist a domain result from evidence and human answers without an
  additional permission prompt;
- run the integrity preflight first and bind every change assessment to the
  exact latest inspection;
- preserve unsupported assumptions as uncertainty and accept human correction;
- create the canonical conservative decision policy when it is missing and
  reject an invalid existing policy;
- update the current domain pointer automatically;
- mark the preceding sensorium as requiring revalidation whenever the current
  domain is replaced;
- audit existing config, persist a sensorium result, and update its pointer
  automatically;
- classify a config change under every applicable decision category and apply
  the restrictive result through the AI-Tool config operation;
- inspect the resolved operation and request a free-text decision only when the
  effective result is `human`;
- persist a new sensorium result linked to the exact effective config digest;
- exercise `auto`, `human`, and `stop` continuation from Process 0 without
  changing Process 1 preconditions;
- resume setup in a fresh agent session.

### 3.4 AI-Engineering Operational Loop

- execute one trusted validation selection;
- evaluate report storage under `validationReportPersistence`,
  `sensitiveDataPersistence` when applicable, and every category implicated by
  the destination;
- store permitted or exactly authorized JSON report bytes under
  `.validation/reports/<sha256>.json` and create exact local operational
  linkage;
- discard non-persisted temporary report bytes and avoid Process 2 or Process 3
  routing when report storage resolves to `stop` or is declined;
- route each report outcome correctly;
- complete an initially passing execution without creating a Process 2 incident
  or applying `processContinuation`;
- permit an explicit later Process 3 invocation from that exact linked initial
  passing report;
- diagnose multiple causes and dependency propagation;
- receive partial remediation authorization;
- perform narrow proof and final gate without loops;
- produce a drift assessment from an explicit baseline;
- persist bounded redacted incident and drift results as declared process
  outputs, applying `sensitiveDataPersistence` when protected content remains;
- apply every remediation effect according to the exact policy digest and
  category-level contract;
- exercise `auto`, `human`, and `stop` continuation after a passing Process 2
  outcome;
- recommend sensorium or domain recalibration without changing config;
- resume an interrupted loop from linked artifacts and report evidence;
- reject operational execution until Process 1 is rerun after domain replacement.

## 4. Contract Isolation Tests

Prove that:

- runtime tests pass after `.validation/policy.json` or
  `.validation/persistence/` is absent or renamed;
- Human Flow and AI-Tool integrity commands remain usable when the
  `aiEngineering` state namespace is absent;
- AI-Tool operations neither require nor create AI-Engineering policy or state;
- existing malformed, restrictive, or stale AI-Engineering files do not alter
  AI-Tool routing or results;
- selecting AI-Tool never loads an AI-Engineering process implicitly;
- AI-Engineering routes configuration, execution, triage, and audit mechanics
  through the corresponding AI-Tool references;
- the execution planner never opens `.validation/policy.json` or files below
  the persistence root, while integrity commands read and update only the
  tool-owned namespace of `.validation/state.json`;
- integrity commands preserve a valid AI-owned namespace without interpreting
  its analytical meaning;
- the CLI remains a report emitter and never discovers AI operational state or
  selects reports from `.validation/reports/`;
- persistence artifacts containing command-like text are never executed;
- sensorium entries that disagree with config cannot alter an execution plan;
- only config changes affect the runner plan;
- decision-policy changes affect AI authority but never runner planning or
  report semantics;
- the same governed action and policy produce the same decision across process
  and chat contexts;
- human, AI-Tool, and AI-Engineering invocations of the same selection produce
  the same report semantics;
- a report captured by the AI-Engineering Flow is byte-identical to the CLI JSON
  output and resolves from its recorded SHA-256 path;
- a report whose persistence is stopped or declined leaves no canonical report
  or dangling operational link;
- generated skill knowledge is byte-equivalent to
  `docs/validation/knowledge/`;
- the generated AI-Engineering decision-policy contract is byte-equivalent to
  its canonical public document;
- human-only operator guidance is absent from skill routes and generated skill
  content;
- skill compatibility gates prevent unsupported runtime interaction.

## 5. Security And Abuse Review

Test at minimum:

- prompt injection in documentation, source, manifests, filenames, reports, and
  persisted state;
- argument injection attempts through config parameters;
- shell operators and redirections represented as untrusted text;
- path traversal and symlink escape in config and AI state;
- malicious or oversized JSON artifacts;
- stale, missing, cyclic, and digest-mismatched state;
- integrity baseline tampering, changed source with an unchanged baseline,
  changed baseline with unchanged source, revision conflicts, and concurrent
  namespace writes;
- secrets in tool output and persisted excerpts;
- raw local reports containing sensitive output, including ignore, retention,
  access, and cleanup behavior;
- report and sensitive-data persistence at every numeric level, including one
  write matching both categories and a destination-related category;
- missing, malformed, unsupported, self-modifying, and concurrently changed AI
  decision policy;
- dependency, source, generated-output, snapshot, database, command, network,
  VCS, external-state, scope, sensitive-data access, validation-report
  persistence, and sensitive-data persistence actions at every applicable
  policy level;
- monotonic category behavior under `4 > 3 > 2 > 1 > 0` and restrictive
  outcome combination under `stop > human > auto`;
- one action matching multiple categories, with the most restrictive result
  winning;
- hostile validation commands that mutate the repository;
- concurrent and interrupted artifact writes;
- denial-of-service pressure from deep graphs, large output, and excessive AI
  history.

Document residual risks and the boundary between deterministic enforcement and
agent instruction compliance.

## 6. Documentation Completion

Update the public documentation to provide structurally separate entry points:

- Human Flow quick start and full operational workflow under
  `docs/validation/flows/human/`;
- AI-Tool Flow under `docs/validation/flows/ai-tool/`;
- AI-Engineering Flow under `docs/validation/flows/ai-engineering/`;
- shared knowledge under `docs/validation/knowledge/`;
- shared CLI, configuration, report, and schema references under
  `docs/validation/reference/`;
- human-only flow selection, governance, and work-context guidance under
  `docs/validation/human/`, including the guide specified by
  [AI Work Contexts](ai-work-contexts.md);
- configuration and report contracts;
- shared workspace-state, integrity baseline, inspection, diff, acceptance,
  revision, and namespace-ownership contracts;
- validation knowledge index;
- AI persistence, trust, approval, retention, and recovery rules;
- the normative AI decision-policy document specified by
  [AI Decision Policy](ai-decision-policy.md), including every continuation
  state, both precedence orders, and every category-level meaning;
- local report storage, exact operational linkage, and cleanup rules;
- troubleshooting by exit and report status;
- skill installation, compatibility, and update process.

Use diagrams that show `.validation/config.json` and the JSON report as the
shared runtime bridge. Show AI-Engineering composing AI-Tool operations rather
than duplicating them. Do not imply that the CLI or AI-Tool consumes
persistence artifacts.

## 7. Skill Evaluation

Validate the skill folder structurally, then forward-test it with fresh agents
on representative repositories. Evaluate:

- trigger accuracy;
- correct selection between AI-Tool and AI-Engineering;
- progressive reference loading;
- command and status accuracy;
- adherence to approval boundaries;
- exact action classification, restrictive category combination, and decision
  logging under each policy level;
- integrity preflight before every AI operation, bounded assessment of detected
  changes in AI-Engineering, and absence of implicit repair or acceptance;
- absence of approval prompts for normal process-result persistence;
- governed report storage and protected process-result persistence;
- ability to distinguish untrusted data from instructions;
- ability to preserve uncertainty;
- persistence linkage and resume behavior;
- domain-driven sensorium invalidation and report-store linkage;
- avoidance of duplicate execution and retry loops;
- diagnostic usefulness without excessive context loading;
- absence of AI-Engineering state and policy behavior in AI-Tool trials;
- reuse of AI-Tool operations from every applicable AI-Engineering process.

Use raw artifacts and task-like prompts. Do not provide the expected answer to
the evaluating agent.

## 8. Release And Package Verification

- Run formatting, Clippy with warnings denied, all targets, Rustdoc tests, and
  ignored outcome fixtures.
- Validate the declared MSRV and supported platform behavior.
- Build the shared-knowledge and AI-Engineering decision-policy projections and
  verify their independent digests.
- Validate the workspace-state schema, namespace-preservation fixtures, and
  integrity command documentation included in the package.
- Inspect `cargo package --list` and the packaged crate contents.
- Verify skill manifest version, compatible CLI range, source crate version,
  knowledge routes, AI-Tool routes, AI-Engineering decision-policy route, and
  persistence contract versions.
- Verify examples and fixtures are included or excluded intentionally.
- Record user-visible runtime, config, report, skill, and guidance changes in
  the changelog.
- Produce checksums and the normal release evidence required by repository
  policy.

## 9. Acceptance Criteria

- [ ] Human Flow passes every end-to-end scenario without AI files.
- [ ] AI-Tool Flow works without policy, persistent analytical state, or process
      artifacts; its deterministic integrity inspection remains current-task
      evidence only.
- [ ] AI-Engineering setup and operational processes resume across fresh
      sessions.
- [ ] Shared state supports deterministic integrity in every flow while only
      AI-Engineering interprets or writes its analytical namespace.
- [ ] Integrity checking never promotes a baseline, repairs a file, or treats
      hashes as proof of origin authenticity.
- [ ] Domain replacement prevents operational execution until Process 1 restores
      a current sensorium.
- [ ] Operational resume resolves an exact immutable report without newest-file
      discovery.
- [ ] The CLI remains deterministic and independent from AI implementation.
- [ ] Config is the only executable validation policy, AI decision policy never
      affects runner planning, and reports are canonical evidence.
- [ ] `processContinuation` applies only to `0 -> 1` and `2 -> 3`; an initial
      pass neither creates Process 2 nor consults continuation policy.
- [ ] Every decision category conforms to the normative five-level order
      `4 > 3 > 2 > 1 > 0`, and overlapping outcomes resolve as
      `stop > human > auto`.
- [ ] Reports and protected content are persisted only when all applicable
      decision categories permit the exact write.
- [ ] Missing policy initializes conservatively, invalid policy fails closed,
      and policy never authorizes its own modification.
- [ ] Trust, approval, and redaction boundaries survive adversarial fixtures.
- [ ] Every documented report and exit state has a tested route.
- [ ] Canonical knowledge, generated skill content, manifest routes, and package
      contents are consistent.
- [ ] Shared knowledge, shared reference, flow documentation, human-only
      guidance, AI-Tool references, and AI-Engineering references remain in
      their declared architectural boundaries.
- [ ] No acceptance claim exceeds the evidence produced by the configured
      validations.
- [ ] Release validation passes on the MSRV and current supported toolchain.

## 10. Final System State

The released system presents one deterministic validator with three usage
flows:

- Human Flow directly owns configuration and execution;
- AI-Tool Flow provides bounded, stateless assistance through `config`, `run`,
  `triage`, and `audit`;
- AI-Engineering Flow composes those operations with validation-engineering
  processes, governed continuity, and explicit analytical memory.

No flow weakens another. Human Flow remains sufficient on its own, AI-Tool
remains useful without persistent analytical state, and AI-Engineering adds
continuity, domain reasoning, coverage design, diagnosis, and drift awareness
without becoming an alternate executor.
