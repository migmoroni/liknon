# Phase 4: AI-Engineering State And Safety Foundation

## 1. Objective

Define the AI-Engineering Flow as a safe, resumable orchestration layer that
composes the completed Human Flow and AI-Tool Flow. Establish persistence
contracts, trust boundaries, declared process-result recording, authorization
boundaries for external effects, the consumer-owned AI decision policy,
artifact linkage, and skill routing before implementing domain analysis or
remediation behavior.

The validation execution planner remains unaware of this layer. The runtime
integrity subsystem reads and updates only the tool-owned namespace of the
shared workspace state established in Phase 1; it never interprets AI
observations, artifacts, policy, or process semantics.

## 2. Dependency

- Phase 1 provides the stable executable configuration and report contracts.
- Phase 2 provides source-backed reasoning guidance and progressive disclosure.
- Phase 3 provides the bounded AI-Tool operations that AI-Engineering composes
  for configuration, execution, triage, and audit.

## 3. State And Evidence Boundary

Use this consumer-owned structure:

```text
.validation/
├── config.json
├── policy.json
├── state.json
├── reports/
│   └── <sha256>.json
└── persistence/
    ├── operational-state.json
    ├── domain/
    │   └── <artifact-id>.json
    ├── sensorium/
    │   └── <artifact-id>.json
    ├── incidents/
    │   └── <artifact-id>.json
    └── drift/
        └── <artifact-id>.json
```

`state.json` is the shared workspace-state contract. Its tool-owned `integrity`
namespace contains the accepted hash baseline and latest deterministic
inspection. Its AI-owned `aiEngineering` namespace contains current validated
setup references, sensorium lifecycle state, and bounded observations about
integrity changes. `operational-state.json` contains only local continuity for
the current Process 2 and Process 3 cycle, including exact report, artifact,
and AI decision-policy references.

An AI-Engineering process creates its ordinary bounded, redacted artifact and
updates the AI-owned state namespace as a declared process result. If content
that remains in a process artifact or state observation is sensitive, the write
also requires the decision produced by `sensitiveDataPersistence`.

Incident, drift, and report records are selected by explicit linkage, never by
scanning for the lexically newest filename.

For every AI-Engineering execution that produces a complete versioned JSON
report, the agent:

1. captures the exact report bytes emitted on standard output in bounded
   non-persistent memory or a protected temporary file;
2. validates the document against the bundled report contract;
3. calculates SHA-256 over those exact bytes;
4. verifies that the report root and temporary path remain inside the workspace
   without symlink escape;
5. classifies report persistence under `validationReportPersistence` and, when
   the report contains protected values, `sensitiveDataPersistence`, together
   with every category required by the destination;
6. writes the document atomically to
   `.validation/reports/<sha256>.json`;
7. verifies an existing file with that name is byte-identical before reusing
   it;
8. records the normalized path and digest in `operational-state.json` and in
   every incident or drift artifact that consumes the report.

Steps 6 through 8 occur only after an `auto` outcome or an exact free-text human
authorization. A `human` outcome waits without persisting the bytes. A `stop`,
declined, or constrained decision that excludes storage securely discards any
temporary bytes, creates no canonical report file or report link, and records
only a bounded redacted decision result. Without an exact stored report,
Processes 2 and 3 cannot use that execution as canonical evidence.

The report store preserves permitted raw execution evidence byte-for-byte. AI
artifacts contain only the bounded, redacted excerpts needed for reasoning.
The CLI remains unaware of report and AI-persistence directories: it emits the
report, while the AI-Engineering process performs governed storage and linkage
after capture.

Repository policy is:

- `policy.json` is consumer-owned, reviewable AI decision policy and follows
  the consumer repository's normal tracking rules;
- `state.json` is shared deterministic and analytical state, not executable
  policy; repository treatment follows the documented workspace-state policy;
- `domain/` and `sensorium/` are resumable AI context and may be tracked when
  the consumer wants that context shared across sessions;
- `reports/`, `operational-state.json`, `incidents/`, and `drift/` contain local
  execution evidence or operational memory and are ignored;
- consumer policy may exclude all AI-Engineering persistence without affecting
  Human Flow or AI-Tool Flow.

Reports can contain captured process output and therefore potentially sensitive
data. Documentation requires local-only retention by default, restrictive file
handling, bounded retention and cleanup, and a warning against committing or
sharing reports without review. Persistence authority follows
`validationReportPersistence` and `sensitiveDataPersistence`. An
AI-Engineering process never edits or redacts a stored canonical report after
hashing it.

Writing process results inside `.validation/persistence/` is part of the invoked
AI-Engineering process. Ordinary bounded, redacted process results require no
separate approval prompt. Sensitive content that remains necessary in an
artifact or state observation is evaluated under `sensitiveDataPersistence`.
This declared output is strictly confined to AI context files and the
`aiEngineering` namespace of `.validation/state.json`; it does not extend to
`.validation/config.json` or any other workspace path. Report persistence is
governed independently as defined above.

Every AI skill invocation first runs `workspace-validator integrity check`.
When shared state is uninitialized, invalid, or in a revision conflict, the
agent does not infer a baseline: it follows the exact authority required for
`integrity init` or stops with that prerequisite. A changed result may be
inspected through `integrity diff`; the agent never restores files or promotes
the observed hashes through `integrity accept` implicitly.

On the first AI-Engineering process invocation, the agent validates the current
`policy.json` or atomically creates the canonical conservative policy when the
file is absent. It then creates the persistence root and required artifact
subdirectory when authorized by the invoked process. The report store and
`operational-state.json` are created only when governed report persistence
succeeds. References for processes that have not completed remain absent rather
than pointing to placeholders.

Persistence schemas are canonical resources owned by the tool and distributed
with the skill. The consumer stores only its state documents; it does not copy
or independently maintain these schemas.

The AI decision-policy schema is another canonical tool-owned resource bundled
with the skill. Phase 4 implements the schema, skill references, fixtures,
canonical safe creation, action classification, restrictive decision
combination, policy digest tracking, and the public normative document
specified by [AI Decision Policy](ai-decision-policy.md). These resources do
not become part of the CLI execution planner.

## 4. Common Artifact Envelope

Every artifact uses descriptive field names and a versioned envelope:

```json
{
  "schemaVersion": 1,
  "artifactId": "<opaque-collision-resistant-id>",
  "artifactType": "domain",
  "createdAt": "2026-09-25T18:25:00.000Z",
  "parentArtifactIds": [],
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

The process-specific schema constrains `artifactType` and `payload`. The contract
must additionally define:

- collision-resistant artifact ID generation;
- canonical digest calculation;
- UTC timestamp format;
- nullable repository state when Git is unavailable;
- parent type and cardinality rules;
- bounded decision records containing an opaque decision ID, matched
  categories and configured levels, redacted action preview, combined result,
  authority source, policy digest, outcome, and timestamp;
- maximum document and field sizes;
- path normalization and containment;
- redaction requirements;
- unknown-field rejection.

Canonical schemas and schema fixtures live in the tool repository, are included
in the source package and skill bundle as declared resources, and are tested
for exact consistency. Their presence does not authorize the CLI to discover
or read consumer persistence.

Do not optimize canonical artifacts with cryptic keys. Transport or prompt
assembly may create a temporary compact projection without replacing the
auditable source document.

## 5. Shared Workspace State Contract

`.validation/state.json` is one versioned document with explicit ownership
boundaries:

```json
{
  "schemaVersion": 1,
  "stateRevision": "<opaque-revision>",
  "integrity": {
    "status": "clean",
    "baseline": {},
    "lastInspection": {}
  },
  "aiEngineering": {
    "currentDomain": null,
    "sensorium": { "status": "missing" },
    "observations": []
  }
}
```

The exact schema constrains every placeholder above. The tool owns
`schemaVersion`, `stateRevision`, and `integrity`. Integrity commands may update
only those fields and must preserve a valid `aiEngineering` value byte-for-byte
at the semantic JSON-value level. The AI-Engineering Flow owns only
`aiEngineering`; it must preserve the tool-owned fields and update the document
through an atomic revision check. Neither namespace may contain executable
commands, credentials, source patches, or raw diagnostic streams.

The `integrity` namespace contains:

- a baseline ID, creation time, SHA-256 algorithm identifier, resolved scope,
  deterministically ordered path and file-digest entries, and aggregate digest;
- a last-inspection ID, time, baseline reference, observed aggregate digest,
  status, and bounded added, modified, removed, and hash-equivalent rename
  records;
- `uninitialized`, `clean`, `changed`, `invalid`, and `conflict` as distinct
  states;
- no automatic baseline promotion. Only `integrity init` creates the first
  baseline and only `integrity accept` promotes a successfully inspected state.

The `aiEngineering` namespace contains:

- the current validated domain artifact ID and digest;
- a discriminated sensorium state: `missing`, `current`, or
  `revalidation_required`;
- when `current`, the validated sensorium artifact ID and digest, its domain
  artifact ID and digest, and the configuration digest it analyzed;
- when `revalidation_required`, the preceding sensorium reference for audit and
  one or more bounded observation references that explain which validated
  dependency changed;
- bounded observations linked to one exact integrity inspection ID and digest,
  containing changed paths, evidence, assessed context impact, uncertainty,
  required follow-up, timestamp, and decision-policy digest when persistence
  required a governed decision;
- the completion time and producing process version for each current reference.

An AI skill always runs `integrity check` first. When the latest inspection is
`changed`, no AI-Engineering process may rely on existing analytical context
until an observation linked to that exact inspection assesses its impact. The
agent uses `integrity diff`, repository evidence, and referenced digests for the
assessment. It records the observation but never repairs, restores, deletes, or
accepts changed files merely because it detected them.

This is one general invalidation mechanism rather than a growing list of
file-specific rules. A changed validated dependency, including the exact config
digest bound to the current sensorium, marks the affected analytical context as
`revalidation_required`. A change shown not to affect current domain or
sensorium assumptions is recorded as such and does not invalidate them.
Accepting a new hash baseline does not clear an analytical invalidation;
Process 1 is the only process that can return the sensorium to `current`.

The state machine is explicit:

| Event | Integrity and AI-context result |
| --- | --- |
| No baseline exists | `uninitialized`; stop before relying on integrity claims |
| `integrity check` matches the baseline | `clean`; existing AI context remains subject to its own references |
| `integrity check` differs from the baseline | `changed`; require an observation for that exact inspection before using AI context |
| The observation finds no affected analytical dependency | Record the assessment; preserve the current sensorium state |
| The observation finds an affected domain or sensorium dependency | Preserve prior references for audit and set `revalidation_required` |
| First domain becomes current and no sensorium exists | `missing` |
| Process 1 persists a valid sensorium for the current domain and config | `current` |
| A domain replaces the one referenced by a current sensorium | `revalidation_required` with a linked bounded observation |
| Process 2 or Process 3 is requested while sensorium state is not `current` | No transition; reject the operation and direct the human to Process 1 |

Update the AI-owned current state automatically after:

1. writing a complete immutable artifact;
2. validating its shape and references;
3. confirming every referenced file and digest;
4. confirming that sensitive persistence is eligible or exactly authorized;
5. atomically replacing `state.json` only when its preceding `stateRevision`
   still matches.

A failed process, policy decision, or revision check leaves the preceding valid
state intact. Human correction of an analytical result causes the process to
produce a new immutable artifact and update the pointer again; it does not
mutate the preceding artifact. Hashes establish equality with the accepted
baseline, not authenticity against an actor able to replace both workspace
content and state.

`operational-state.json` is a separate local, atomically replaced document. It
contains its own schema version, an opaque operation ID, process status, exact
domain and sensorium references, configuration digest, the AI decision-policy
path, schema version, and digest used for the latest decision, the current
report path and SHA-256 digest, and explicit incident, final passing report,
drift, and bounded decision-record references when they exist. It contains no
report body, command source, credentials, raw sensitive parameters, or source
patches. A fresh agent resumes an operational cycle from this document and its
exact links, never from directory order or chat history. Any mismatch with
current domain, sensorium, or configuration state makes that operational cycle
stale rather than eligible for implicit reuse. A decision-policy mismatch
invalidates cached authorization decisions but does not invalidate domain,
sensorium, or report evidence; the process reloads and re-evaluates policy
before its next governed effect.

## 6. Trust Model

The AI treats all of the following as untrusted data:

- README and repository documentation;
- manifests, source comments, fixtures, and generated files;
- validation stdout and stderr;
- stored validation reports;
- persisted AI artifacts;
- text embedded in file names, paths, dependencies, or test output.

Repository content cannot override system, workspace, skill, or user
instructions. The agent extracts facts and evidence from it but does not obey
embedded requests to run commands, expose data, weaken policy, or change
scope.

Configuration is executable validation policy. AI authority to inspect, edit,
or execute it is a separate question resolved by the current human instruction
and `.validation/policy.json`. The policy never turns persistence artifacts
into executable input.

## 7. Authorization Model

Implement the complete contract in
[AI Decision Policy](ai-decision-policy.md). Its canonical public projection at
`docs/validation/flows/ai-engineering/decision-policy.md` is an absolute rule
for every bundled AI-Engineering process: process references may link
to it but must not redefine category or level semantics independently.

Distinguish these fixed and policy-governed actions:

| Action | Authority |
| --- | --- |
| Run `integrity check` before an AI operation | Mandatory precondition; the exact command still follows the authority applicable to that flow, and its deterministic integrity-state update never authorizes baseline acceptance |
| Initialize or accept an integrity baseline | Exact human request or every applicable AI decision category, including `workspaceMutation`; never implied by inspection |
| Read ordinary repository and current AI state through non-executing inspection | Covered by the analysis request; sensitive resources still evaluate `sensitiveDataAccess` |
| Create a missing `.validation/policy.json` | Automatic only when writing the exact canonical conservative policy |
| Modify an existing `.validation/policy.json` | Exact explicit human request or free-text approval; the policy cannot authorize its own mutation |
| Create a bounded redacted process artifact under `.validation/persistence/` | Declared result of invoking that AI-Engineering process; evaluate `sensitiveDataPersistence` when protected values remain |
| Update the `aiEngineering` namespace of `state.json` | Declared result of the invoked process or integrity assessment; evaluate `sensitiveDataPersistence` when protected values remain and require the preceding state revision |
| Store a captured canonical report under `.validation/reports/` | Evaluate `validationReportPersistence`, `sensitiveDataPersistence` when applicable, and every category implicated by the destination |
| Create or update `operational-state.json` with exact report and artifact links | Declared result after governed report persistence succeeds; evaluate `sensitiveDataPersistence` if protected values remain |
| Continue from Process 0 to Process 1 | Apply `processContinuation` after successful persistence and Process 1 precondition validation |
| Continue from Process 2 to Process 3 | Apply `processContinuation` after a passing verified outcome, successful persistence, and Process 3 precondition validation |
| Inspect configuration through non-executing parsing | Covered by the analysis request; invoking an inspection command also evaluates `commandExecution` |
| Modify configuration or other workspace files | Evaluate every applicable decision category |
| Execute configuration, tools, or scripts | Evaluate `commandExecution` and every other category implicated by the exact action |
| Install, update, or remove a tool, dependency, runtime, or toolchain | Evaluate `dependencyManagement` plus every overlapping category |
| Modify source, snapshots, databases, or generated output | Evaluate `workspaceMutation`, `dataMutation`, and any other applicable category |
| Access an external service, remote state, or sensitive resource | Evaluate all applicable network, external-state, and sensitive-data categories |

The evaluator classifies actor, action, resource, environment, scope,
reversibility, trust origin, sensitivity, privilege, and blast radius. Every
applicable category evaluates independently. Decision levels follow
`4 > 3 > 2 > 1 > 0`; computed outcomes combine as
`stop > human > auto`. Approval prompts accept free text so the human can
authorize only part of a proposal, impose constraints, postpone it, or report
an action performed manually.

## 8. Skill Architecture

Retain one concise `SKILL.md` router. Keep Phase 3 operations under `ai-tool/`
and add direct references for the AI-Engineering processes:

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

Each process reference describes one process, routes operational work through
the applicable AI-Tool reference, and links to the exact knowledge categories
it may need. `ai-engineering/policy.md` is the concise mandatory router to the
exact generated contract at
`ai-engineering/contracts/decision-policy.md`; it does not duplicate the
category tables. Do not make one AI reference load every other process.

A deterministic release task projects only the normative decision-policy
document from `docs/validation/flows/ai-engineering/decision-policy.md` to that
contract path and verifies identical paths, bytes, and digest. Other public
flow documents and every file under `docs/validation/human/` remain outside the
skill projection.

The AI-Engineering index is selected only when the human invokes this flow or
one of its processes. AI-Tool requests never load this directory implicitly.

The manifest remains the skill version and compatibility source. Persistence
artifacts record the producing skill version so an agent can identify stale or
incompatible state before relying on it.

Public AI-Engineering contracts live under:

```text
docs/validation/flows/ai-engineering/
├── README.md
├── decision-policy.md
├── persistence.md
├── trust-model.md
└── processes/
```

Human-only governance guidance lives outside that flow documentation:

```text
docs/validation/human/
├── README.md
├── choosing-a-flow.md
├── ai-work-contexts.md
└── policy-ownership.md
```

`choosing-a-flow.md` explains when a human may select Human Flow, AI-Tool Flow,
or AI-Engineering Flow and affirms that the human may override the suggested
mode. `policy-ownership.md` explains human ownership of `policy.json` without
redefining its normative tables. These human-only files are public operator
guides, are not copied into skill references, and cannot activate agent
behavior.

## 9. State Safety Behavior

- Run the deterministic integrity check before loading AI observations or
  performing an AI operation.
- Require an observation linked to the latest changed inspection before relying
  on existing AI context.
- Never initialize or accept an integrity baseline as an implicit consequence
  of inspection, validation success, or process completion.
- Never repair, restore, delete, or rewrite a changed source file merely because
  the integrity check reported it.
- Reject malformed, unsupported, path-escaping, oversized, or internally
  inconsistent state.
- Stop rather than guessing when a current artifact is missing or has the wrong
  digest.
- Stop before operational execution when sensorium state is `missing` or
  `revalidation_required`; direct the human to Process 1.
- Reconcile state against current repository facts and ask the human only when
  the conflict requires domain judgment that evidence cannot establish.
- Never silently select an older artifact as fallback.
- Never execute a command copied from an artifact.
- Resolve an intended validation through the current config graph and existing
  skill mechanics.
- Minimize and redact protected data from derived artifacts whenever it is not
  required; evaluate `sensitiveDataPersistence` before any protected content
  that remains is written.
- Evaluate `validationReportPersistence` and every overlapping category before
  exact report storage. Once permitted and stored, preserve its bytes exactly
  and redact only derived AI artifact excerpts.
- Define retention and cleanup for ignored operational artifacts.
- Never remove a report that is referenced by current operational state or a
  retained incident or drift artifact.
- Persist a completed Process 0 or Process 2 before applying continuation toward
  its defined related process.
- Apply `processContinuation` only to `0 -> 1` and `2 -> 3`, after successful
  persistence and the target process's preconditions: continue automatically
  for `auto`, request a free-text decision for `human`, and end without a
  continuation prompt for `stop`.
- Never interpret successful persistence alone as a continuation decision.
- Reload the current decision policy before a governed effect when its digest
  differs from the persisted operational decision context.

## 10. Tests

Create schema and process fixtures for:

- valid artifacts of every type;
- unsupported schema versions;
- unknown fields and malformed timestamps;
- missing and cyclic parent references;
- digest mismatch and stale current state;
- integrity initialization, clean and changed inspection, deterministic diff,
  explicit acceptance, and unchanged baseline after inspection;
- bounded AI observations linked to the exact latest integrity inspection;
- integrity changes that do and do not invalidate current analytical context;
- state revision conflicts between integrity commands and AI updates;
- domain replacement that marks an existing sensorium for revalidation;
- rejection of Process 2 and Process 3 while sensorium revalidation is required;
- exact report capture, schema validation, SHA-256 naming, atomic write,
  identical-file reuse, and mismatched-file rejection;
- report persistence at all levels of `validationReportPersistence`, with and
  without sensitive content;
- sensitive persistence at all levels of `sensitiveDataPersistence` across
  reports, process artifacts, and state observations;
- declined, stopped, and constrained persistence that leaves no canonical
  report or dangling report link;
- report-store traversal, symlink escape, interrupted write, and concurrent
  identical capture;
- missing, stale, or digest-mismatched operational report linkage;
- path traversal and symlink escape;
- oversized state and diagnostic fields;
- interrupted writes and current-state rollback;
- concurrent artifact creation;
- prompt-injection text in every untrusted input class;
- unauthorized config, dependency, and source changes;
- missing-policy bootstrap, invalid-policy rejection, policy self-mutation
  prevention, and changed-policy re-evaluation;
- every category at levels `0` through `4`, including overlapping categories
  whose most restrictive result wins;
- monotonic category behavior under `4 > 3 > 2 > 1 > 0`;
- bounded and redacted records for automatic, human, and stopped decisions;
- `auto`, `human`, and `stop` continuation for both related process pairs;
- accepted, declined, constrained, postponed, and unanswered human decisions;
- a workspace with no persistence directory.

Forward-test the skill with raw consumer fixtures. Test agents must not receive
the expected conclusion or hidden answer.

## 11. Acceptance Criteria

- [ ] Human Flow and AI-Tool Flow remain unchanged when AI-Engineering decision
      policy and persistence are absent.
- [ ] The CLI and library interpret only the tool-owned integrity namespace of
      shared state and contain no AI persistence or decision-policy discovery.
- [ ] Every AI operation begins from a current deterministic integrity result,
      and changed state is assessed without implicit repair or baseline
      acceptance.
- [ ] Every AI artifact is versioned, linked, bounded, and non-executable.
- [ ] Every invoked AI-Engineering process writes its bounded redacted result
      and updates current state without requesting separate approval; protected
      content that remains follows `sensitiveDataPersistence`.
- [ ] Current state references only successfully completed, validated artifacts.
- [ ] Replacing the current domain makes any existing current sensorium
      unusable until Process 1 produces a replacement.
- [ ] AI-Engineering runs store exact report bytes under their SHA-256 name only
      after every applicable persistence outcome permits it, and operational
      state then references the exact path and digest.
- [ ] Related processes implement all three continuation states without
      treating successful persistence alone as authority.
- [ ] No process selects state by newest filename.
- [ ] Untrusted repository content cannot authorize an action.
- [ ] Every governed effect follows the exact category-level contract and
      restrictive combination rule.
- [ ] Missing policy creates the canonical conservative policy, invalid policy
      fails closed, and policy cannot authorize its own modification.
- [ ] Every AI-Engineering decision records the exact policy digest without
      storing secrets.
- [ ] Skill references load progressively and remain within the declared
      compatibility range.

## 12. Handoff To Phase 5

Phase 5 implements domain and sensorium payloads only after this phase proves
that their declared persistence lifecycle and trust boundaries are safe.
