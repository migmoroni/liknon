# Phase 6: AI-Engineering Triage And Drift Loop

## 1. Objective

Implement the daily AI-Engineering loop over the current domain, sensorium, and
configuration. The loop composes the AI-Tool operations to execute the normal
validator, diagnose structured outcomes, perform only authorized remediation,
verify the smallest affected scope, and evaluate whether recent change
invalidates the current validation design.

## 2. Dependency

Phase 5 has produced:

- one current validated domain artifact;
- one current validated sensorium artifact whose lifecycle state is `current`;
- one exact config digest;
- one valid shared integrity state and an AI assessment linked to its latest
  inspection when that inspection reports changes;
- a trusted config that passes complete configuration validation.

## 3. Execution Process

Before a run, the agent:

1. runs `workspace-validator integrity check`;
2. when the inspection reports changes, inspects `integrity diff`, records a
   bounded observation linked to that exact inspection, and marks every
   affected analytical context without repairing or accepting changed files;
3. validates `.validation/policy.json` and records its exact digest;
4. checks skill and CLI compatibility;
5. validates current persistence references and digests;
6. rejects operational execution when the sensorium is `missing` or
   `revalidation_required`, then confirms that a `current` sensorium references
   the exact current domain and config digests;
7. confirms the requested group, suite, or check through inspection;
8. records repository state without cleaning or restoring it;
9. evaluates execution under every applicable decision category, accounting
   for exact authority in the initiating request;
10. delegates one explicit configured selection to `ai-tool/run.md` with JSON
   output when the effective result permits it;
11. waits for completion and captures the exact JSON bytes in bounded
    non-persistent storage;
12. validates the report contract and hashes the exact bytes;
13. evaluates `validationReportPersistence`,
    `sensitiveDataPersistence` when applicable, and every category implicated
    by the destination;
14. atomically stores eligible or exactly authorized bytes at
    `.validation/reports/<sha256>.json`;
15. creates or atomically updates `operational-state.json` with the operation
    ID, current setup references, decision-policy digest, and exact report path
    and digest;
16. parses that stored report for outcome routing.

When report persistence resolves to `human`, the process waits before Step 14.
When it resolves to `stop` or the human declines storage, the process securely
discards temporary bytes, records only a bounded redacted persistence decision,
and ends without a canonical report link or outcome routing.

The agent never expands a group by manually executing its member checks and
never runs commands from a persistence artifact.

## 4. Outcome Routing

Route outcomes by the report contract and stable CLI exit semantics:

| Outcome | Route |
| --- | --- |
| Pass | Persist the permitted passing result, mark it as the final passing report, and complete the execution without applying `processContinuation` |
| One or more failed checks or repository gate | Create an incident diagnosis |
| Blocked or skipped without failure | Diagnose prerequisite or dependency state; do not assume source failure |
| Invalid configuration or CLI use | Return to the `ai-tool/config.md` operation |
| Internal execution or reporting failure | Report a validator defect boundary; do not patch consumer source |
| Interrupted | Preserve available evidence and stop until the human resumes |

Do not reduce all nonzero exits to one remediation path.

For an initial pass, atomically mark the current report as
`finalPassingReport` in `operational-state.json` and complete the execution.
This is not a Process 2 result and never applies `processContinuation`. Process
3 may be invoked explicitly later from that exact linked passing report. During
remediation, every newly permitted validation report replaces the current
report reference; only a passing final verification may populate
`finalPassingReport`. A non-pass result clears any final-passing reference for
that operation.

## 5. Process 2: Incident

### 5.1 Evidence

Use `ai-tool/triage.md` for report interpretation and the stored JSON report as
canonical evidence. Record its exact path and
digest and extract only the diagnostics needed for analysis. Preserve:

- selection and config digest;
- tool preflight outcomes;
- check IDs, contexts, argument vectors, and working directories;
- statuses, exit codes, timeout and truncation markers;
- concise redacted stdout and stderr evidence;
- dependency propagation;
- repository-integrity findings;
- current domain and sensorium IDs.

The incident artifact and `operational-state.json` reference the same stored
report. Derived excerpts may be redacted and bounded, but the report file is
never rewritten after hashing.

### 5.2 Diagnosis

An incident may contain more than one independent primary cause. For each
cause, record:

- category: configuration, environment, validation, timeout, interruption,
  dependency propagation, repository integrity, or validator internal;
- affected checks and evidence excerpts;
- one or more hypotheses with confidence and disconfirming evidence;
- whether it appears related to the requested implementation;
- the smallest useful counter-check or suite;
- every applicable AI decision-policy category and its effective result.

Do not force a singular root cause and do not discard a diagnostic merely
because another failure occurred earlier.

### 5.3 Remediation

Before changing source, config, snapshots, generated data, databases,
dependencies, or tools:

1. explain the supported diagnosis;
2. present the smallest proposed intervention;
3. classify the action by scope, trust origin, environment, reversibility,
   sensitivity, privilege, blast radius, and every applicable decision
   category;
4. combine results as `stop > human > auto`;
5. stop for `stop`, obtain a free-text decision for `human`, or proceed for
   `auto`, accounting for exact authority already supplied by the initiating
   request;
6. apply only the exact eligible or human-authorized scope;
7. classify and authorize the verification command independently when its
   effects were not covered by the preceding decision;
8. run the smallest affected check or suite;
9. compare its failure fingerprint with the original report;
10. stop if the unchanged failure repeats without relevant new evidence;
11. run one final requested or policy-required group after the narrow proof.

The process writes its incident artifact automatically as its analytical
result, including the diagnosis, policy decision status, and any automatic or
human-authorized actions. Protected content that remains follows
`sensitiveDataPersistence`. It does not claim that a proposed patch succeeded
before the verification report exists.

### 5.4 Process 2 Handoff

Process 2 may apply `processContinuation` toward Process 3 only after the
current operational cycle has a canonical passing final verification after
eligible or human-authorized remediation. A run that passes initially does not
create an artificial Process 2 incident and does not apply this continuation
policy.

After persisting the completed Process 2 result, its exact report linkage, and
the current operation update, the agent:

1. summarizes the passing evidence and remaining uncertainty;
2. validates every Process 3 precondition;
3. starts Process 3 for `auto`, suggests it and waits for a free-text decision
   for `human`, or ends without suggesting it for `stop`.

Declining, postponing, not answering, or resolving continuation to `stop` ends
the current process without invalidating Process 2. Process 3 can be invoked
later from any work context using the persisted operational state and exact
linked passing report. A failed, blocked, skipped, invalid, internal-error, or
interrupted outcome never triggers continuation.

## 6. Process 3: Drift

Drift assessment composes the non-executing analysis principles from
`ai-tool/audit.md` and runs only after a passing report. A pass is necessary
evidence for this process, not proof that the workspace is ready
for release or commit.

### 6.1 Baseline

Record an explicit baseline containing:

- current domain and sensorium IDs and digests;
- current config digest;
- passing report path, digest, schema version, and selection;
- repository HEAD revision when available;
- staged, unstaged, untracked, renamed, and deleted paths;
- relevant manifest, lockfile, schema, build, and delivery changes;
- the comparison base used for committed changes.

Do not rely only on `git diff --name-only`, and do not infer the current
incident by selecting the newest file in a directory.

Process 3 resolves the report from `operational-state.json`, verifies its digest
and schema before use, and stops if the link is absent, stale, or inconsistent.
It also requires a current integrity inspection and a linked AI assessment for
any reported changes before using the operational baseline.

### 6.2 Drift Signals

Evaluate whether changes introduce:

- a new language, framework, technology, file class, or generated artifact;
- a new build, package, platform, or delivery path;
- a changed dependency or toolchain boundary;
- a changed schema, persistence model, protocol, concurrency model, or network
  boundary;
- a new user, data, security, accessibility, or operational consequence;
- a mismatch between config IDs and the current sensorium evidence matrix;
- a changed product purpose or invariant.

### 6.3 Recommendations

Use explicit recommendations rather than universal drift classes:

- no uncovered drift identified from the available evidence;
- review validation coverage and produce a new sensorium proposal;
- review domain assumptions, then produce a new sensorium proposal;
- insufficient evidence; request human clarification.

The process writes the drift artifact automatically. The human decides whether
to invoke domain or sensorium recalibration. A recalibration process persists
its own result automatically, but no process changes executable configuration
without evaluation under the current AI decision policy. Drift recommendations
do not count as either of the two `processContinuation` relationships.

## 7. Chat Contract

For execution and incident handling, report:

- selection and overall result;
- primary causes or blocking conditions;
- affected check IDs and shortest useful evidence;
- artifact path and report linkage;
- policy decision request or next smallest action.

For drift, report:

- baseline and inspected change scope;
- uncovered or uncertain evidence;
- recommendation and its rationale;
- drift artifact path;
- human decision required, when any.

Compact presentation never suppresses failures, uncertainty, secrets warnings,
or approval boundaries.

## 8. Tests And Forward Trials

Exercise the complete loop with fixtures for:

- pass with no identified drift;
- initial pass that completes without applying `processContinuation`;
- explicit Process 3 invocation from an initially passing linked report;
- pass with a new source type;
- pass with manifest and lockfile changes;
- independent failures in two checks;
- one primary failure with skipped dependents;
- missing or incompatible tool;
- timeout and truncated output;
- invalid configuration;
- internal validator failure;
- interruption;
- repository mutation caused by a check;
- stale domain, sensorium, config, and report linkage;
- `missing` and `revalidation_required` sensorium states;
- exact report persistence and operational-state resume after a fresh session;
- `human` and `stop` report-persistence outcomes that create no report link and
  cannot enter Process 2 or Process 3;
- no prior incident;
- hostile instructions in stdout, stderr, source, and persistence;
- a declined patch, partial approval, and human-performed repair;
- `auto`, `human`, and `stop` Process 2 to Process 3 continuation, including
  accepted, declined, constrained, postponed, and unanswered human decisions;
- remediation actions at every applicable category level, including actions
  that match multiple categories;
- a changed policy digest during a resumable incident;
- repeated unchanged failure fingerprint;
- secrets present in captured output;
- integrity changes assessed as context-preserving and context-invalidating,
  without source restoration or implicit baseline acceptance.

Forward tests use raw reports and repositories without disclosing the intended
diagnosis to the test agent.

## 9. Acceptance Criteria

- [ ] Every CLI result class reaches the correct AI-Tool operation or
      AI-Engineering process.
- [ ] The JSON report remains the canonical execution evidence.
- [ ] Every AI-Engineering operational run stores and links exact report bytes
      only when all applicable persistence outcomes permit it.
- [ ] No operational run starts while Process 1 revalidation is required.
- [ ] Incidents support multiple independent primary causes.
- [ ] Every patch, installation, config mutation, command, network effect, and
      other governed action follows the exact effective policy result.
- [ ] Narrow revalidation precedes an applicable final gate.
- [ ] Repeated unchanged failures stop rather than loop.
- [ ] Drift compares against an explicit baseline and complete visible change
      state.
- [ ] A passing report is never described as complete correctness or automatic
      commit readiness.
- [ ] Incident and drift processes persist bounded redacted analytical results
      without a separate approval prompt, while protected content that remains
      follows `sensitiveDataPersistence`.
- [ ] Process 2 applies `processContinuation` only after a passing verified
      outcome and valid Process 3 preconditions.
- [ ] An initial passing run never applies `processContinuation` or creates an
      artificial Process 2.
- [ ] Recalibration never changes executable configuration without an effective
      `auto` result or exact human authority.

## 10. Handoff To Phase 7

Phase 7 validates all three flows as one distributable system, including
scenarios where AI-Tool or AI-Engineering is absent, selected independently,
stale, interrupted, or resumed in a fresh session where applicable.
