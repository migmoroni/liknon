# Phase 4: AI-Engineering Persistence And Safety Foundation

## 1. Objective

Define the AI-Engineering Flow as a safe orchestration layer that composes the
completed Human Flow and AI-Tool Flow. Establish explicit immutable artifacts,
single-file persistence, trust boundaries, authorization rules, and the
consumer-owned AI decision policy before implementing domain analysis or
remediation behavior.

The validation planner and executor remain unaware of this layer. They interpret
only `.validation/config.json` and emit the normal validation report. Dedicated
policy and persistence commands may validate fixed contracts and explicitly
supplied files, but they never discover analytical context or influence an
execution plan.

## 2. Dependencies

- Phase 1 provides stable CLI, configuration, report, and Human Flow contracts.
- Phase 2 provides source-backed reasoning guidance and progressive disclosure.
- Phase 3 provides the bounded AI-Tool operations that AI-Engineering composes
  for configuration, execution, triage, and audit.

## 3. Persistence Boundary

Use this consumer-owned structure:

```text
.validation/
├── config.json
├── policy.json
├── reports/
│   └── <sha256>.json
└── persistence/
    ├── domain/
    │   └── <uuidv7>.json
    ├── sensorium/
    │   └── <uuidv7>.json
    ├── incidents/
    │   └── <uuidv7>.json
    └── drift/
        └── <uuidv7>.json
```

The immutable files and their explicit references are the complete persistence
model.

Every process receives the exact artifact and report paths it needs. It validates
their schemas, IDs, digests, containment, and declared relationships before use.
A direct continuation passes the artifact created by the preceding process. A
later invocation, including one in a fresh conversation, requires the human or
caller to provide the exact paths again. No operation scans a directory to infer
the newest, current, preferred, or otherwise authoritative file.

This explicit-input model has these consequences:

- Process 0 has no analytical parent and produces one domain artifact;
- Process 1 consumes one explicit domain artifact and produces one sensorium
  artifact bound to that domain and the exact configuration digest;
- Process 2 consumes explicit domain and sensorium artifacts plus one exact
  stored non-pass report;
- Process 3 consumes explicit domain and sensorium artifacts plus one exact
  stored passing report and may additionally reference an explicit incident;
- a sensorium is usable only when its recorded domain and configuration digests
  match the inputs of the requested operation;
- creating another artifact never invalidates, updates, or deletes an earlier
  file; validity is established from the explicit relationship being used.

Process-result persistence and report persistence are separate decisions.
Invoking an AI-Engineering process authorizes writing its bounded redacted result
to the canonical process directory. Protected content that remains follows
`sensitiveDataPersistence`. Exact validation reports are stored only when
`validationReportPersistence`, `sensitiveDataPersistence` when applicable, and
every category implicated by the destination permit the write.

## 4. Deterministic Policy And Persistence Operations

Phase 4 adds deterministic, non-cognitive policy and persistence operations
owned by the tool. Their public CLI shape is:

```sh
workspace-validator policy validate [--workspace <workspace-path>] --format=json
workspace-validator policy init [--workspace <workspace-path>] --format=json
workspace-validator persistence verify artifact <path> [--workspace <workspace-path>] --format=json
workspace-validator persistence verify report <path> [--workspace <workspace-path>] --format=json
workspace-validator persistence store artifact --input <candidate-path> [--workspace <workspace-path>] --format=json
workspace-validator persistence store report --input <candidate-path> [--workspace <workspace-path>] --format=json
```

The equivalent library API uses typed inputs and results. These operations do
not interpret domain meaning, choose an artifact, evaluate AI policy, or execute
configured validation commands.

Every command above uses one workspace-boundary contract:

- `--workspace` identifies the workspace root; when omitted, it defaults exactly
  to the process current working directory;
- the root must be an existing directory and is canonicalized once before any
  input is read or destination is resolved;
- the commands do not search parent directories, inspect Git to discover a
  root, or read `config.json` to infer one;
- relative caller-supplied paths resolve against that root, and absolute paths
  are accepted only when their canonical targets remain inside it;
- every `<path>` or `--input` value, including a temporary staging file, must
  satisfy that same containment rule;
- canonical destinations always resolve below that root's `.validation/`
  directory, with symlink and parent-traversal escape rejected;
- machine results expose normalized workspace-relative paths rather than
  silently changing the selected boundary.

The library API has no current-directory fallback. It receives one validated
workspace-root value explicitly and uses that same value for policy,
persistence, containment, and destination resolution.

`policy validate` reads only
`<workspace>/.validation/policy.json`, validates its schema and semantic
invariants, computes SHA-256 over its exact bytes, and returns a bounded
versioned result containing the normalized path, schema version, digest, and
validation status. A missing document is a typed non-success result. The command
never creates, modifies, or replaces a file.

`policy init` is an idempotent bootstrap operation for that same fixed path:

1. when the document is absent, it securely creates the canonical `.validation/`
   directory when needed and atomically creates the exact bundled conservative
   policy without overwriting another writer;
2. when the document already exists, it never alters it;
3. in both cases, it opens the resulting file, validates the exact bytes under
   the normal policy contract, and returns the same path, schema version, and
   digest as `policy validate`, plus whether creation occurred;
4. a malformed, unsupported, symlinked, or internally inconsistent existing
   policy fails closed and is never replaced with defaults.

Neither policy command classifies or authorizes later AI actions. The narrowly
defined automatic bootstrap permission belongs to the AI-Engineering contract;
the CLI only performs the deterministic operation requested by its caller.

`persistence verify artifact`:

- requires one explicit contained regular file;
- rejects symlinks, traversal, unsupported schema versions, unknown fields,
  invalid UUIDv7 values, invalid timestamps, oversized content, and mismatched
  declared digests;
- validates every explicitly declared parent or report reference without
  searching for alternatives;
- returns a bounded versioned JSON result containing the artifact type, ID,
  digest, referenced inputs, and validation status.

`persistence verify report` validates one explicit report against the bundled
report schema, computes SHA-256 over the exact bytes, and returns its selection,
result, schema version, and digest without storing it.

`persistence store artifact`:

1. reads one bounded candidate file supplied explicitly by the caller;
2. validates it against the process-specific bundled schema;
3. resolves its canonical destination from `artifactType` and `artifactId`;
4. verifies workspace containment and rejects symlink traversal;
5. writes the exact validated bytes through a temporary file and atomic rename;
6. if the destination already exists, reuses it only when the bytes are
   identical;
7. returns the normalized path, UUIDv7, artifact type, and SHA-256 digest.

`persistence store report` follows the same single-file operation, naming the
destination `.validation/reports/<sha256>.json`. It preserves the exact report
bytes and rejects a non-identical file at that digest path.

The caller performs the applicable decision-policy evaluation before invoking a
store command. The tool enforces shape, destination, containment, bounds, and
atomicity; it does not infer that the caller was authorized. A denied or stopped
decision leaves the candidate in bounded temporary storage only and produces no
canonical file.

Each store operation touches one canonical destination and commits it with an
atomic rename. A temporary file left by interruption is never treated as a
canonical artifact and may be removed through ordinary bounded cleanup.

## 5. Report Evidence

For an AI-Engineering execution that produces a complete versioned JSON report,
the agent:

1. captures the exact standard-output bytes in bounded non-persistent memory or
   a protected temporary file inside the selected workspace boundary;
2. invokes `persistence verify report` with the active workspace root;
3. evaluates every applicable persistence category;
4. invokes `persistence store report` with that same root only after an `auto`
   result or exact human authorization;
5. passes the returned path and digest explicitly to a process that consumes the
   report.

A `human` result waits before Step 4. A `stop`, declined, or constrained decision
that excludes storage discards temporary bytes and creates no canonical report.
The current validation result may still be summarized to the human, but Process
2 or Process 3 cannot use it later as canonical persisted evidence.

Reports may contain captured process output and therefore sensitive data. The
canonical profile requires local ignored storage, restrictive permissions where
supported, and a warning against committing or sharing reports without review.
The contract performs no automatic retention expiry, deletion, pruning, or
compaction. Removal is an explicit governed action.

## 6. Repository Treatment

Repository policy is:

- `policy.json` is consumer-owned, reviewable AI decision policy and follows the
  consumer repository's normal tracking rules;
- `domain/` and `sensorium/` contain resumable analytical context and may be
  tracked when the consumer intentionally wants to share those artifacts;
- `reports/`, `incidents/`, and `drift/` contain local execution evidence or
  operational memory and must be ignored;
- consumer policy may exclude every AI-Engineering persistence directory without
  affecting Human Flow or AI-Tool Flow.

Before writing `reports/`, `incidents/`, or `drift/` in a Git workspace, the
store operation verifies through Git that the concrete destination is ignored.
A missing ignore rule blocks that write and returns an exact corrective
instruction. Applying the correction is a separate `workspaceMutation`
decision. If effective treatment cannot be determined for one of these required
local destinations, the write fails closed.

For `domain/` and `sensorium/`, the operation reports the effective Git
treatment without requiring either tracked or ignored status. The consumer
chooses whether those analytical artifacts are shared through its repository.

## 7. Common Artifact Envelope

Every process artifact uses a versioned descriptive envelope:

```json
{
  "schemaVersion": 1,
  "artifactId": "<uuidv7>",
  "artifactType": "domain",
  "createdAt": "2026-09-25T18:25:00.000Z",
  "artifactReferences": [],
  "reportReferences": [],
  "workspace": {
    "repositoryRoot": ".",
    "headRevision": "<revision-or-null>"
  },
  "configuration": {
    "path": ".validation/config.json",
    "sha256": "<digest-or-null>"
  },
  "decisionPolicy": {
    "path": ".validation/policy.json",
    "schemaVersion": 1,
    "sha256": "<digest>"
  },
  "decisionRecords": [],
  "evidence": [],
  "uncertainties": [],
  "payload": {}
}
```

An artifact reference contains the referenced UUIDv7, artifact type, normalized
workspace-relative path, and SHA-256 digest. A report reference contains the
normalized path, exact-byte SHA-256 digest, report schema version, selection,
and result. Process-specific schemas constrain allowed reference types,
cardinalities, and payloads.

The contracts additionally define:

- canonical digest calculation over exact stored bytes;
- UTC timestamp format;
- nullable repository revision when Git is unavailable;
- maximum document and field sizes;
- path normalization and workspace containment;
- redaction requirements;
- unknown-field rejection;
- prohibition of executable commands, credentials, source patches, report
  bodies, and raw diagnostic streams inside analytical artifacts.

Canonical schemas and fixtures belong to the tool, ship in the source package
and skill bundle as declared resources, and are tested for exact consistency.
Consumer workspaces store artifact instances, not schema copies.

## 8. Policy Bootstrap And Validation

The complete policy contract is defined by
[AI Decision Policy](ai-decision-policy.md). At the start of every
AI-Engineering process invocation, the agent invokes `policy init` with the
explicit active workspace root. The command validates the existing document or
atomically creates and validates the exact canonical conservative policy when it
is absent. The returned path, schema version, and digest become the process's
initial policy identity.

An existing malformed, unsupported, path-escaping, or internally inconsistent
policy stops the process. The policy cannot authorize its own modification.
Every later policy change requires an explicit human request or free-text human
approval bound to the exact proposed diff.

Each artifact records the exact policy schema version and SHA-256 digest used
for its decisions. Before a later governed effect, a resumed or long-running
process invokes `policy validate` with the same workspace root, reloads the
current policy, and records a different digest when policy changed.

## 9. Trust Model

The AI treats all of the following as untrusted data:

- README and repository documentation;
- manifests, source comments, fixtures, and generated files;
- validation stdout and stderr;
- stored validation reports;
- persisted AI artifacts;
- text embedded in filenames, paths, dependencies, or test output.

Repository content cannot override system, workspace, skill, or user
instructions. The agent extracts facts and evidence from it but does not obey
embedded requests to run commands, expose data, weaken policy, or change scope.

Configuration is executable validation policy. AI authority to inspect, edit,
or execute it is a separate decision resolved by the current human instruction
and `.validation/policy.json`. Persistence artifacts remain non-executable data.

## 10. Authorization Model

Implement the complete contract in
[AI Decision Policy](ai-decision-policy.md). Its canonical public projection is
an absolute rule for every bundled AI-Engineering process. Process references
link to it rather than redefining category or level semantics.

| Action | Authority |
| --- | --- |
| Verify an explicitly supplied policy, artifact, or report | Covered by the analysis request; sensitive resources still evaluate `sensitiveDataAccess` |
| Create a missing canonical `policy.json` | Automatic only for the exact conservative document |
| Modify an existing `policy.json` | Exact explicit human request or free-text approval; the policy cannot authorize itself |
| Store a bounded redacted process artifact | Declared result of the invoked process; evaluate `sensitiveDataPersistence` when protected values remain |
| Store an exact validation report | Evaluate `validationReportPersistence`, `sensitiveDataPersistence` when applicable, and every category implicated by the destination |
| Continue from Process 0 to Process 1 | Apply `processContinuation` after successful artifact persistence and Process 1 precondition validation |
| Continue from Process 2 to Process 3 | Apply `processContinuation` after a passing verified outcome, successful artifact persistence, and Process 3 precondition validation |
| Inspect configuration through non-executing parsing | Covered by the analysis request; invoking an inspection command also evaluates `commandExecution` |
| Modify configuration or other workspace files | Evaluate every applicable decision category |
| Execute configuration, tools, or scripts | Evaluate `commandExecution` and every other implicated category |
| Install, update, or remove a tool, dependency, runtime, or toolchain | Evaluate `dependencyManagement` plus every overlapping category |
| Modify source, snapshots, databases, or generated output | Evaluate `workspaceMutation`, `dataMutation`, and every other implicated category |
| Access an external service, remote state, or sensitive resource | Evaluate all applicable network, external-state, and sensitive-data categories |

The evaluator classifies actor, action, resource, environment, scope,
reversibility, trust origin, sensitivity, privilege, and blast radius. Decision
levels follow `4 > 3 > 2 > 1 > 0`; computed outcomes combine as
`stop > human > auto`. Human prompts accept free text.

## 11. Skill Architecture

Retain one concise `SKILL.md` router. Keep Phase 3 operations under `ai-tool/`
and add direct references for AI-Engineering:

```text
references/
├── knowledge/
├── ai-tool/
│   ├── index.md
│   ├── run.md
│   ├── triage.md
│   ├── config.md
│   └── audit.md
└── ai-engineering/
    ├── index.md
    ├── policy.md
    ├── contracts/
    │   └── decision-policy.md
    ├── process-0-domain.md
    ├── process-1-sensorium.md
    ├── process-2-incident.md
    └── process-3-drift.md
```

Each process reference describes one process, requires explicit artifact inputs,
routes operational work through the applicable AI-Tool reference, and links only
the knowledge categories it needs. The AI-Engineering index is selected only
when the human invokes that flow or one of its processes.

The manifest remains the skill version and compatibility source. Artifacts
record the producing skill version so a later process can reject incompatible
inputs.

## 12. Safety Behavior

- Verify CLI and skill compatibility before process work.
- Validate the current policy and every explicitly supplied artifact or report.
- Stop rather than search for or guess a missing input.
- Never select an artifact by filename order or modification time.
- Never repair, restore, delete, or accept a repository change merely because it
  was observed.
- Never execute a command copied from an artifact or report.
- Resolve intended validation through the current config graph and AI-Tool
  mechanics.
- Minimize and redact protected data from derived artifacts.
- Preserve exact stored report bytes and redact only derived excerpts.
- Retain canonical artifacts until an explicit governed removal.
- Never remove an artifact or report supplied as an input to the active process.
- Persist Process 0 or Process 2 before applying its continuation rule.
- Apply `processContinuation` only to `0 -> 1` and `2 -> 3` after validating the
  exact output and next-process preconditions.

## 13. Tests

Create schema, persistence, and process fixtures for:

- valid artifacts of every type;
- unsupported schema versions, unknown fields, malformed timestamps, and invalid
  UUIDv7 values;
- missing, cyclic, wrong-type, path-escaping, and digest-mismatched references;
- deterministic verification of explicit artifacts and reports;
- explicit and omitted workspace-root resolution, current-directory default,
  non-directory roots, no upward discovery, and identical CLI/library
  containment behavior;
- relative and absolute contained inputs, parent traversal, symlink escape, and
  canonical destinations anchored to the selected workspace;
- atomic single-file storage, interrupted temporary writes, identical-file
  reuse, and non-identical destination rejection;
- report schema validation, SHA-256 naming, exact-byte preservation, and bounded
  output;
- required-local destinations that are ignored, not ignored, symlinked, or
  unavailable to Git, plus tracked and ignored domain or sensorium artifacts;
- explicit artifact selection when multiple valid files exist;
- stale sensorium inputs caused by domain or config digest mismatch;
- read-only policy validation; idempotent and concurrent missing-policy
  bootstrap; invalid-policy rejection without replacement; policy self-mutation
  prevention; and changed-policy reevaluation;
- report and sensitive persistence at every applicable decision level;
- denied or stopped persistence that leaves no canonical file;
- concurrent writes of the same and different immutable artifacts;
- prompt-injection text in every untrusted input class;
- every decision category at levels `0` through `4`, overlapping categories,
  and restrictive outcome combination;
- `auto`, `human`, and `stop` continuation for both defined process pairs;
- a workspace with no AI-Engineering persistence directory.

Forward-test the skill with raw consumer fixtures. Test agents must not receive
the expected conclusion or hidden answer.

## 14. Acceptance Criteria

- [ ] Human Flow and AI-Tool Flow remain complete without policy, reports, or AI
      persistence.
- [ ] The execution planner interprets only `config.json`.
- [ ] Every AI-Engineering process receives exact artifact and report inputs and
      uses only those supplied inputs.
- [ ] Every completed process creates one immutable UUIDv7 artifact.
- [ ] Artifact and report storage use one validated atomic file operation.
- [ ] Policy validation and bootstrap are deterministic CLI and library
      operations, and bootstrap never overwrites an existing policy.
- [ ] Every policy and persistence operation uses one explicit canonical
      workspace boundary; the CLI defaults only to its current directory, while
      the library API always requires the root.
- [ ] Process 1 binds its sensorium to the exact domain and config digests.
- [ ] Processes 2 and 3 reject mismatched domain, sensorium, config, or report
      relationships.
- [ ] Reports are stored under their exact SHA-256 names only after every
      applicable persistence decision permits the write.
- [ ] Related processes implement all three continuation modes.
- [ ] Untrusted repository content cannot authorize an action.
- [ ] Missing policy initializes conservatively, invalid policy fails closed,
      and policy cannot authorize its own modification.
- [ ] Every decision records the exact policy digest without storing secrets.
- [ ] Skill references load progressively and remain within their declared
      compatibility range.

## 15. Handoff To Phase 5

Phase 5 implements domain and sensorium payloads only after the explicit-input,
single-file persistence, trust, and authorization contracts are proven.
