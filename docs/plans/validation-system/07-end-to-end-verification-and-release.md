# Phase 7: End-To-End Verification And Release

## 1. Objective

Prove that Human Flow, AI-Tool Flow, and AI-Engineering Flow are independently
usable, compose only through declared contracts, and ship as coherent versioned
artifacts. Complete documentation, realistic forward trials, package
inspection, security review, and release evidence.

## 2. Dependency

Phases 1 through 6 are implemented. Acceptance tests in this phase verify their
composition; they do not replace an incomplete earlier contract.

## 3. End-To-End Scenario Matrix

### 3.1 Human Flow

- author a complete configuration candidate, install it through
  `init --config`, and validate the canonical configuration from documentation;
- inspect a nested group without executing it;
- run a check, suite, explicit group, and default group;
- understand pass, fail, blocked, skipped, invalid, internal, and interrupted
  outcomes;
- inspect repository mutations reported for a concrete validation run;
- diagnose and rerun a narrow scope manually;
- operate successfully with only the CLI and `.validation/config.json`.

### 3.2 AI-Tool Flow

- explain a missing policy and direct the human to run `init --policy` without
  invoking that command;
- explain every current policy category, recommend manual values, and revalidate
  after the human reports editing the file;
- refuse to create, edit, format, replace, delete, move, restore, initialize, or
  indirectly mutate policy, including after explicit authorization and under
  level `0`;
- explain a report using only `ai-tool/triage.md`;
- audit coverage without running checks;
- make an authorized config edit within the bounded request;
- reject an unauthorized installation or source change;
- recognize stale skill and incompatible CLI versions;
- inspect repository mutation evidence from a concrete report;
- complete every operation without mutating AI decision policy or creating
  process artifacts;
- begin a later request without inherited authority or analytical context.

### 3.3 AI-Engineering Setup

- prove that a missing policy blocks process work and directs the human to
  invoke `init --policy` personally and review every setting;
- validate the policy created or corrected manually by the human without
  modifying it;
- prove that AI-Engineering delegates policy explanation to `AI-Tool/policy` and
  cannot mutate policy through any process path;
- create and store one immutable domain artifact from evidence and human
  answers;
- verify that artifact by exact path and digest;
- exercise `auto`, `human`, and `stop` continuation from Process 0;
- pass the exact domain artifact directly to Process 1;
- invoke Process 1 later by supplying that same artifact explicitly;
- audit existing config and create one immutable sensorium artifact bound to
  the exact domain and config digests;
- classify a proposed config change under every applicable decision category;
- request a free-text decision only when the restrictive outcome is `human`;
- preserve unsupported assumptions as uncertainty;
- reject mismatched domain, sensorium, and config relationships;
- create another immutable artifact for a correction without replacing an
  earlier result.

### 3.4 AI-Engineering Operational Loop

- execute one trusted validation selection through the ordinary runner;
- evaluate exact report storage under `validationReportPersistence`,
  `sensitiveDataPersistence` when applicable, and every category implicated by
  the destination;
- store permitted report bytes under `.validation/reports/<sha256>.json`;
- discard temporary report bytes when storage resolves to `stop` or is
  declined;
- route every report outcome correctly;
- finish an initially passing execution without creating an incident or
  applying `processContinuation`;
- invoke Process 3 later from that exact passing report when requested;
- diagnose multiple causes and dependency propagation in Process 2;
- receive partial remediation authorization;
- perform one narrow proof and one final gate without an unchanged retry loop;
- create immutable incident and drift artifacts with exact parent references;
- exercise `auto`, `human`, and `stop` continuation after a passing Process 2
  result;
- compare explicit repository revisions and bounded change evidence in Process
  3;
- recommend domain or sensorium recalibration without changing setup artifacts;
- resume from exact caller-supplied artifact and report paths in a fresh
  conversation.

## 4. Contract Isolation Tests

Prove that:

- Human Flow works when `.validation/policy.json` and
  `.validation/persistence/` are absent;
- read-only AI-Tool policy guidance works when policy is absent or invalid, while
  every other AI operation stops before workspace work;
- AI-Tool operations require and obey valid policy without creating or mutating
  it and never create AI-Engineering artifacts;
- selecting AI-Tool never loads an AI-Engineering process implicitly;
- AI-Engineering routes configuration, execution, triage, and audit mechanics
  through the corresponding AI-Tool references;
- the validation planner and executor never discover or interpret
  `.validation/policy.json` or process artifacts;
- persistence commands operate only on caller-supplied files and never choose
  an artifact or report;
- policy and persistence commands share one canonical workspace boundary, do
  not discover another root, and expose the same containment behavior through
  CLI and library APIs;
- every `init` capability uses that boundary, validates all requested resources
  before writing, and reports created, reused, conflicted, or partially
  committed resources exactly;
- artifact content containing command-like text is never executed;
- only `.validation/config.json` affects runner planning;
- decision-policy changes affect AI authority but never runner planning or
  report semantics;
- the same governed action, inputs, and policy produce the same decision across
  process and conversation contexts;
- human, AI-Tool, and AI-Engineering invocations of the same configured
  selection produce the same report semantics;
- a stored report is byte-identical to the CLI JSON output and resolves from
  its SHA-256 path;
- declined report storage leaves no canonical report or dependent process
  artifact;
- generated skill knowledge is byte-equivalent to
  `docs/validation/knowledge/`;
- the generated shared AI decision-policy contract is byte-equivalent to
  its canonical public document;
- human-only operator guidance is absent from skill routes and generated skill
  content;
- skill compatibility gates prevent unsupported runtime interaction.

## 5. Security And Abuse Review

Test at minimum:

- prompt injection in documentation, source, manifests, filenames, reports,
  and process artifacts;
- argument injection attempts through config parameters;
- shell operators and redirections represented as untrusted text;
- path traversal and symlink escape in config, report, and artifact paths;
- explicit, omitted, nonexistent, non-directory, nested, and symlinked workspace
  roots, including proof that omission means the exact current directory rather
  than upward discovery;
- malicious, oversized, unsupported, and digest-mismatched JSON artifacts;
- mismatched domain, sensorium, config, report, and incident relationships;
- attempts to select an artifact by directory order or filename recency;
- secrets in tool output and persisted excerpts;
- reports containing sensitive output, including ignore treatment, access
  protection, explicit removal, and absence of automatic pruning;
- report and sensitive-data persistence at every numeric level, including one
  write matching both categories and a destination-related category;
- missing policy before AI work, direct human initialization, malformed,
  unsupported, and concurrently changed AI decision policy;
- adversarial attempts to make either AI flow create, edit, format, replace,
  delete, move, restore, initialize, or indirectly mutate policy, including
  explicit user authorization and policy level `0`;
- missing, malformed, oversized, symlinked, incompatible, and stale supplied
  skill manifests;
- dependency, source, generated-output, snapshot, database, command, network,
  VCS, external-state, scope, sensitive-data access, validation-report
  persistence, and sensitive-data persistence actions at applicable levels;
- monotonic category behavior under `4 > 3 > 2 > 1 > 0` and restrictive outcome
  combination under `stop > human > auto`;
- one action matching multiple categories, with the most restrictive outcome
  winning;
- hostile validation commands that mutate the repository;
- concurrent and interrupted single-file writes;
- denial-of-service pressure from deep validation graphs, large output, and
  excessive artifact content.

Document residual risks and the boundary between deterministic enforcement and
agent instruction compliance.

## 6. Documentation Completion

Update public documentation with separate entry points for:

- Human Flow under `docs/validation/flows/human/`;
- AI-Tool Flow under `docs/validation/flows/ai-tool/`;
- AI-Engineering Flow under `docs/validation/flows/ai-engineering/`;
- shared knowledge under `docs/validation/knowledge/`;
- CLI, initialization, configuration, report, persistence-command, and schema
  references under `docs/validation/reference/`;
- human-only flow selection, governance, and work-context guidance under
  `docs/validation/human/`;
- troubleshooting by exit and report status;
- skill installation, compatibility, and update procedures.

Document the exact process-artifact envelopes, canonical destinations,
single-file atomic storage, explicit parent references, report-byte storage,
trust boundaries, continuation semantics, and decision-policy categories.

Use diagrams that show `.validation/config.json` and the JSON report as the
shared runtime bridge. Show AI-Engineering composing AI-Tool operations rather
than duplicating them. Show direct and fresh-conversation artifact handoff
without implying that the validation planner or executor consumes process
artifacts.

## 7. Skill Evaluation

Validate the skill folder structurally, then forward-test it with fresh agents
on representative repositories. Evaluate:

- trigger accuracy;
- correct selection between AI-Tool and AI-Engineering;
- mandatory loading of the immutable policy boundary for both AI flows;
- correct policy preflight, read-only guidance, and refusal of every policy
  mutation path;
- progressive reference loading;
- command and report-status accuracy;
- adherence to approval boundaries;
- exact action classification, restrictive category combination, and decision
  recording under each policy level;
- absence of approval prompts for ordinary bounded process-result persistence;
- governed report storage and protected process-result persistence;
- validation of every explicit process prerequisite;
- ability to distinguish untrusted data from instructions;
- preservation of uncertainty;
- explicit artifact handoff within one conversation and across fresh
  conversations;
- domain and config relationship enforcement for sensorium artifacts;
- avoidance of duplicate execution and unchanged retry loops;
- diagnostic usefulness without excessive context loading;
- absence of AI-Engineering persistence and artifact behavior in AI-Tool trials;
- reuse of AI-Tool operations from every applicable AI-Engineering process.

Use raw artifacts and task-like prompts. Do not provide the expected answer to
the evaluating agent.

## 8. Release And Package Verification

- Run formatting, Clippy with warnings denied, all targets, Rustdoc tests, and
  ignored outcome fixtures.
- Validate the declared MSRV and supported platform behavior.
- Build shared-knowledge, shared AI decision-policy, and immutable policy-boundary
  projections and
  verify their independent digests.
- Validate every process-artifact schema, explicit-reference fixture,
  report-storage fixture, and persistence-command contract included in the
  package.
- Inspect `cargo package --list` and the packaged crate contents.
- Verify skill manifest version, compatible CLI range, source crate version,
  knowledge routes, policy-boundary route, AI-Tool routes, AI-Engineering routes,
  and persistence contract versions.
- Verify examples and fixtures are included or excluded intentionally.
- Record user-visible runtime, config, report, skill, and guidance changes in
  the changelog.
- Produce checksums and the normal release evidence required by repository
  policy.

## 9. Acceptance Criteria

- [ ] Human Flow passes every end-to-end scenario with only the CLI and config.
- [ ] AI-Tool Flow performs bounded policy-governed assistance without mutating
      policy or creating process artifacts.
- [ ] AI-Engineering processes continue across fresh conversations from exact
      explicitly supplied artifacts.
- [ ] Domain replacement requires a new matching Process 1 result before an
      operational process uses it.
- [ ] Operational invocation resolves exact immutable reports and artifacts
      without directory scanning or recency selection.
- [ ] The CLI remains deterministic and independent from AI implementation.
- [ ] Config is the only executable validation policy, AI decision policy never
      affects runner planning, and reports are canonical execution evidence.
- [ ] `processContinuation` applies only to `0 -> 1` and `2 -> 3`; an initial
      pass neither creates Process 2 nor consults continuation policy.
- [ ] Every decision category conforms to `4 > 3 > 2 > 1 > 0`, and overlapping
      outcomes resolve as `stop > human > auto`.
- [ ] Reports and protected content persist only when all applicable decision
      categories permit the exact write.
- [ ] Every completed AI-Engineering process creates one immutable UUIDv7
      artifact with exact parent and report references.
- [ ] Missing or invalid policy blocks every AI operation except read-only policy
      guidance until direct human initialization or correction and review.
- [ ] Neither AI flow mutates policy directly or indirectly under any approval,
      configured level, continuation mode, or process state.
- [ ] `init`, `policy validate`, and every persistence operation use the declared
      workspace boundary without config, Git, or parent-directory discovery.
- [ ] Trust, approval, redaction, containment, and atomic-write boundaries
      survive adversarial fixtures.
- [ ] Every documented report and exit outcome has a tested route.
- [ ] Canonical knowledge, generated skill content, manifest routes, and package
      contents are consistent.
- [ ] Shared knowledge, shared reference, flow documentation, human-only
      guidance, AI-Tool references, and AI-Engineering references remain in
      their declared architectural boundaries.
- [ ] No acceptance claim exceeds the evidence produced by configured
      validations.
- [ ] Release validation passes on the MSRV and current supported toolchain.

## 10. Final System Shape

The released system presents one deterministic validator with three usage
flows:

- Human Flow directly owns configuration and execution;
- AI-Tool Flow provides bounded assistance through `policy`, `config`, `run`,
  `triage`, and `audit` under the human-owned decision policy;
- AI-Engineering Flow composes those operations with decision policy and
  explicit immutable analytical artifacts.

Human Flow remains sufficient on its own. AI-Tool remains bounded to the
current request. AI-Engineering adds domain reasoning, coverage design,
diagnosis, drift assessment, and cross-conversation continuity through exact
artifact exchange without becoming an alternate executor.
