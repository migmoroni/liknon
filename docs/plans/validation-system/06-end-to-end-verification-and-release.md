# Phase 6: End-To-End Verification And Release

## 1. Objective

Prove that the human-operated and AI-assisted flows are independently usable,
interoperate only through declared contracts, and ship as coherent versioned
artifacts. Complete documentation, realistic forward trials, package
inspection, security review, and release evidence.

## 2. Dependency

Phases 1 through 5 are implemented. No acceptance test in this phase substitutes
for an incomplete contract from an earlier phase.

## 3. End-To-End Scenario Matrix

### 3.1 Human-Only Scenarios

- create and validate a configuration from documentation;
- inspect a nested group without executing it;
- run a check, suite, explicit group, and default group;
- understand pass, fail, blocked, skipped, invalid, internal, and interrupted
  outcomes;
- diagnose and rerun a narrow scope manually;
- operate successfully with no skill, AI decision policy, persistence
  directory, or report store.

### 3.2 Point-In-Time AI Assistance

- explain a report using only `triage.md`;
- audit coverage without running checks;
- make an authorized config edit without activating persistent AI state;
- reject an unauthorized installation or source change;
- recognize stale skill and incompatible CLI versions.

### 3.3 Persistent AI Setup

- create and persist a domain result from evidence and human answers without an
  additional permission prompt;
- preserve unsupported assumptions as uncertainty and accept human correction;
- create the canonical conservative decision policy when it is missing and
  reject an invalid existing policy;
- update the current domain pointer automatically;
- mark the preceding sensorium as requiring revalidation whenever the current
  domain is replaced;
- audit existing config, persist a sensorium result, and update its pointer
  automatically;
- classify a config change under every applicable decision category and apply
  the restrictive result through the normal config workflow;
- inspect the resolved operation and request a free-text decision only when the
  effective result is `human`;
- persist a new sensorium result linked to the exact effective config digest;
- exercise `auto`, `human`, and `stop` continuation from Process 0 without
  changing Process 1 preconditions;
- resume setup in a fresh agent session.

### 3.4 Persistent AI Daily Loop

- execute one trusted validation selection;
- store its exact JSON report under `.validation/reports/<sha256>.json` and
  create exact local operational linkage;
- route each report outcome correctly;
- diagnose multiple causes and dependency propagation;
- receive partial remediation authorization;
- perform narrow proof and final gate without loops;
- produce a drift assessment from an explicit baseline;
- persist incident and drift results automatically;
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
- the CLI never opens `.validation/policy.json` or files below the persistence
  root;
- the CLI remains a report emitter and never discovers AI operational state or
  selects reports from `.validation/reports/`;
- persistence artifacts containing command-like text are never executed;
- sensorium entries that disagree with config cannot alter an execution plan;
- only config changes affect the runner plan;
- decision-policy changes affect AI authority but never runner planning or
  report semantics;
- the same governed action and policy produce the same decision across process
  and chat contexts;
- human and AI invocations of the same selection produce the same report
  semantics;
- a report captured by the persistent AI flow is byte-identical to the CLI JSON
  output and resolves from its recorded SHA-256 path;
- generated skill guidance is byte-equivalent to canonical documentation;
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
- secrets in tool output and persisted excerpts;
- raw local reports containing sensitive output, including ignore, retention,
  access, and cleanup behavior;
- missing, malformed, unsupported, self-modifying, and concurrently changed AI
  decision policy;
- dependency, source, generated-output, snapshot, database, command, network,
  VCS, external-state, scope, and sensitive-data actions at every applicable
  policy level;
- one action matching multiple categories, with the most restrictive result
  winning;
- hostile validation commands that mutate the repository;
- concurrent and interrupted artifact writes;
- denial-of-service pressure from deep graphs, large output, and excessive AI
  history.

Document residual risks and the boundary between deterministic enforcement and
agent instruction compliance.

## 6. Documentation Completion

Update the public documentation to provide separate entry points:

- human quick start and full operational workflow;
- optional point-in-time agent assistance;
- persistent AI-assisted validation workflow;
- the human-facing AI work-context guide specified by
  [AI Work Contexts](ai-work-contexts.md);
- configuration and report contracts;
- validation knowledge index;
- AI persistence, trust, approval, retention, and recovery rules;
- the normative AI decision-policy document specified by
  [AI Decision Policy](ai-decision-policy.md), including every continuation
  state and every category-level meaning;
- local report storage, exact operational linkage, and cleanup rules;
- troubleshooting by exit and report status;
- skill installation, compatibility, and update process.

Use diagrams that show `.validation/config.json` and the JSON report as the
shared bridge. Do not imply that the CLI consumes persistence artifacts.

## 7. Skill Evaluation

Validate the skill folder structurally, then forward-test it with fresh agents
on representative repositories. Evaluate:

- trigger accuracy;
- progressive reference loading;
- command and status accuracy;
- adherence to approval boundaries;
- exact action classification, restrictive category combination, and decision
  logging under each policy level;
- absence of approval prompts for normal process-result persistence;
- ability to distinguish untrusted data from instructions;
- ability to preserve uncertainty;
- persistence linkage and resume behavior;
- domain-driven sensorium invalidation and report-store linkage;
- avoidance of duplicate execution and retry loops;
- diagnostic usefulness without excessive context loading.

Use raw artifacts and task-like prompts. Do not provide the expected answer to
the evaluating agent.

## 8. Release And Package Verification

- Run formatting, Clippy with warnings denied, all targets, Rustdoc tests, and
  ignored outcome fixtures.
- Validate the declared MSRV and supported platform behavior.
- Build the canonical documentation projection and verify its digest.
- Inspect `cargo package --list` and the packaged crate contents.
- Verify skill manifest version, compatible CLI range, source crate version,
  guidance routes, AI decision-policy route, and persistence contract versions.
- Verify examples and fixtures are included or excluded intentionally.
- Record user-visible runtime, config, report, skill, and guidance changes in
  the changelog.
- Produce checksums and the normal release evidence required by repository
  policy.

## 9. Acceptance Criteria

- [ ] Human-only operation passes every end-to-end scenario without AI files.
- [ ] Point-in-time AI assistance works without persistent state.
- [ ] Persistent AI setup and daily workflows resume across fresh sessions.
- [ ] Domain replacement prevents operational execution until Process 1 restores
      a current sensorium.
- [ ] Operational resume resolves an exact immutable report without newest-file
      discovery.
- [ ] The CLI remains deterministic and independent from AI implementation.
- [ ] Config is the only executable validation policy, AI decision policy never
      affects runner planning, and reports are canonical evidence.
- [ ] `processContinuation` and every category-level combination conform to the
      normative AI decision-policy document.
- [ ] Missing policy initializes conservatively, invalid policy fails closed,
      and policy never authorizes its own modification.
- [ ] Trust, approval, and redaction boundaries survive adversarial fixtures.
- [ ] Every documented report and exit state has a tested workflow.
- [ ] Canonical guidance, generated skill content, manifest routes, and package
      contents are consistent.
- [ ] No acceptance claim exceeds the evidence produced by the configured
      validations.
- [ ] Release validation passes on the MSRV and current supported toolchain.

## 10. Final System State

The released system presents one deterministic validator with two usage flows:

- humans directly own configuration and execution;
- AI agents optionally maintain explicit analytical memory and operate the same
  human flow under human direction.

Neither flow weakens the other. The human flow remains sufficient on its own,
and the AI flow adds continuity, reasoning, diagnosis, and drift awareness
without becoming an alternate executor.
