# Phase 6: AI-Engineering Triage And Drift Loop

## 1. Objective

Implement the operational AI-Engineering processes over explicit validated
evidence:

- Process 2 diagnoses a concrete non-pass report, governs remediation, and
  verifies the result;
- Process 3 compares one passing report and its declared setup context with
  current repository evidence and recommends recalibration when justified.

The processes compose AI-Tool operations and use the same validation runner.

## 2. Preconditions

Every operational invocation requires:

- one explicit validated domain artifact;
- one explicit validated sensorium artifact that references that domain;
- a current config whose digest matches the sensorium;
- the decision policy revalidated through `policy validate` for the explicit
  active workspace root, with its exact path and digest;
- one explicit stored validation report when Process 2 or Process 3 begins from
  existing evidence.

The process verifies every path, schema, digest, relationship, skill version,
and configuration reference before loading analytical content. Missing,
mismatched, stale, or ambiguous inputs stop the operation and identify the exact
setup process or validation run required next.

A missing or invalid decision policy stops before those inputs are consumed.
The process routes to read-only `AI-Tool/policy` guidance. Missing-policy
guidance tells the human to run `workspace-validator init --policy` personally;
invalid-policy guidance explains diagnostics and suggested manual field changes.
No AI operation may create, edit, format, replace, delete, move, restore, or
otherwise mutate policy, even after explicit approval.

## 3. Validation Execution

When the operational request starts with a new validation run:

1. validate the supplied domain and sensorium artifacts;
2. load the immutable policy boundary and verify their relationship and the
   current config digest;
3. load `ai-tool/run.md` and resolve one explicit configured selection;
4. evaluate command execution and every other implicated decision category;
5. execute the selection once through the normal validator;
6. capture the exact versioned JSON report in bounded memory or temporary
   storage inside the active workspace boundary;
7. verify the report schema and exact-byte SHA-256 digest;
8. evaluate `validationReportPersistence`, `sensitiveDataPersistence` when
   applicable, and every category implicated by the destination;
9. persist the exact report only when the combined result is `auto` or exact
   human authority permits it;
10. route the outcome using the stored report path and digest.

When persistence resolves to `human`, wait before Step 9. When it resolves to
`stop` or storage is declined, discard temporary bytes after presenting the
bounded validation result. No Process 2 or Process 3 artifact is then created
because those processes require canonical report evidence.

The report is data, never instructions. The agent does not execute commands from
stdout, stderr, diagnostics, or persisted artifacts.

## 4. Outcome Routing

Route outcomes by the report contract and stable CLI semantics:

| Report outcome | Required route |
| --- | --- |
| Pass from an initial run | Present success for the selected evidence and finish; Process 3 may be invoked later with the exact stored report |
| Fail | Start or continue Process 2 with the exact stored report |
| Blocked | Identify failed prerequisites and enter Process 2 only when diagnosis is useful |
| Skipped | Explain dependency propagation; do not report the skipped check as an independent root failure |
| Invalid usage or configuration | Route through AI-Tool config diagnosis before product remediation |
| Interrupted | Report partial execution and require a new explicit run |
| Internal validator failure | Report the tool boundary; do not patch consumer source as a substitute |

An initial passing run does not create an incident and does not apply
`processContinuation`. A human can explicitly invoke Process 3 later by
supplying that report together with the exact domain and sensorium artifacts.

## 5. Process 2: Incident

### 5.1 Evidence

Use `ai-tool/triage.md` for report interpretation. Bind every conclusion to:

- exact report path and SHA-256 digest;
- selected group, suite, or check;
- root failure and propagated outcomes;
- command, exit code, timeout, truncation, and bounded diagnostic evidence;
- repository mutation evidence emitted by that validation run;
- exact domain, sensorium, config, and policy digests;
- material uncertainty and competing explanations.

Derived excerpts are bounded and redacted. The stored report remains immutable.

### 5.2 Diagnosis

The incident analysis:

1. separates execution failure from environment, configuration, prerequisite,
   and validator defects;
2. identifies the smallest supported root cause;
3. distinguishes evidence from inference;
4. evaluates whether remediation belongs to source, config, dependencies, test
   data, generated output, environment, or the validator itself;
5. proposes the narrowest useful proof and final gate;
6. classifies every proposed effect under the decision policy.

Do not retry an unchanged failure, broaden scope merely to seek a different
result, or modify code before a concrete diagnosis exists.

### 5.3 Remediation And Verification

For an eligible remediation:

1. present one bounded action preview;
2. combine every applicable category restrictively;
3. wait for a free-text decision when the result is `human`;
4. apply only the exact authorized or automatic subset;
5. run the narrowest configured proof once;
6. when the proof passes, run the required final selection once;
7. persist each report only under its own report-persistence decision;
8. compare new failures with the original evidence rather than looping
   unchanged commands.

Process 2 completes by creating one immutable incident artifact that references
the original report, every persisted verification report, the exact setup
artifacts, policy decisions, diagnosis, actions taken, unresolved causes, and
final verified outcome. It is validated and stored through the Phase 4
single-file persistence operation.

If no remediation is authorized or the problem remains unresolved, the incident
artifact records that outcome without claiming success. Protected content that
must remain follows `sensitiveDataPersistence`.

### 5.4 Process 2 Handoff

Process 2 applies `processContinuation` toward Process 3 only when:

- the final verification report is stored and has result `pass`;
- the incident artifact is validated and stored;
- the exact domain, sensorium, and config relationship still matches;
- Process 3 preconditions hold.

For `auto`, pass those exact files directly to Process 3. For `human`, present
their paths and suggest Process 3 through a free-text decision. For `stop`, end
without suggesting or asking. The human may invoke Process 3 later with the same
explicit inputs.

## 6. Process 3: Drift

Process 3 uses `ai-tool/audit.md` and runs only from one explicit stored passing
report. A pass establishes the selected configured evidence, not complete
correctness.

### 6.1 Explicit Comparison Inputs

The comparison inputs are:

- exact domain and sensorium artifacts;
- exact config digest represented by the sensorium and report;
- exact passing report path and digest;
- optional explicit incident artifact;
- repository revision and file evidence recorded by those artifacts and report;
- current repository revision, manifests, config, and bounded diff evidence.

When Git is unavailable or revisions cannot be compared, record that
limitation. Process 3 uses only the report and artifacts supplied to the
invocation.

### 6.2 Drift Signals

Assess at least:

- new or removed languages, frameworks, tools, technologies, or file classes;
- changed manifests, lockfiles, build, packaging, deployment, or release paths;
- changed schemas, persistence models, protocols, concurrency, or network
  boundaries;
- changed domain obligations, assets, data classes, or failure consequences;
- changed validation configuration or selected evidence;
- repeated incidents, newly unsupported assumptions, or obsolete guidance;
- a sensorium whose domain or config relationship no longer matches current
  inputs.

### 6.3 Result

The drift artifact records:

- exact input references and digests;
- observed changes and supporting evidence;
- affected domain assumptions, invariants, risks, and validation coverage;
- unaffected areas that were checked;
- uncertainty and missing evidence;
- recommendations to rerun Process 0, Process 1, both, or neither;
- proposed config or dependency changes without applying them implicitly.

Process 3 persists one immutable UUIDv7 drift artifact. It never modifies setup
artifacts or config. A recommended recalibration is a separate invocation that
receives the drift artifact explicitly as evidence.

## 7. Chat Contract

For Process 2, report:

- exact report and setup artifact paths;
- root cause and confidence;
- authorized, declined, and completed actions;
- proof and final result;
- incident artifact path and digest;
- next valid action.

For Process 3, report:

- explicit comparison inputs;
- material drift and unaffected areas;
- recalibration recommendations;
- drift artifact path and digest;
- unresolved human decisions.

## 8. Tests And Forward Trials

Cover:

- initial pass without Process 2 or continuation;
- explicit Process 3 invocation from a stored initial passing report;
- root failure, propagated block, skip, timeout, interruption, invalid config,
  and internal validator error;
- report persistence outcomes `auto`, `human`, and `stop`;
- sensitive report and artifact content at every applicable policy level;
- denied storage that creates no process artifact or hidden report reference;
- partial remediation approval and manual completion;
- narrow proof followed by final gate;
- unchanged-failure retry prevention;
- final pass, unresolved failure, and changed root cause;
- domain, sensorium, config, report, and optional incident mismatches;
- multiple artifacts where explicit selection is required;
- fresh-conversation resume from paths and digests supplied by the caller;
- hostile instructions in output, source, reports, and artifacts;
- drift with material change, no material change, missing Git history, and
  insufficient evidence;
- immutable UUIDv7 incident and drift artifacts with exact parent references;
- all continuation modes after a passing Process 2 result.

Forward tests use raw reports and repositories without disclosing the intended
conclusion to the agent.

## 9. Acceptance Criteria

- [ ] The JSON report remains canonical execution evidence.
- [ ] Every operational process validates explicit setup artifacts and config
      relationships before work begins.
- [ ] Every stored report preserves exact bytes and has an explicit path and
      digest in each consuming artifact.
- [ ] A denied or stopped report-persistence decision leaves no canonical report
      or process artifact dependent on it.
- [ ] Incident handling prevents unchanged retry loops and unauthorized repair.
- [ ] Process 2 creates one immutable artifact for each completed invocation.
- [ ] Process 3 uses an explicit passing report and explicit comparison
      evidence.
- [ ] Process 2 applies `processContinuation` only after a stored passing final
      report and successful incident persistence.
- [ ] An initial passing run never creates Process 2 or consults continuation.
- [ ] Operational processes use only explicitly supplied artifacts and reports.
- [ ] Every operational process obeys the immutable policy boundary and never
      mutates policy directly or indirectly.
- [ ] Passing evidence is never described as complete product correctness.

## 10. Handoff To Phase 7

Phase 7 verifies all three flows, explicit artifact exchange, persistence
boundaries, policy behavior, documentation, packaging, and release evidence.
