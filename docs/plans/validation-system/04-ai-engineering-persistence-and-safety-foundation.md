# Phase 4: Validation-Engineering Workflow Persistence And Safety Foundation

## 1. Objective

Define the official validation-engineering workflow as a safe orchestration
resource that composes the completed Human Flow with the official action
workflow and its `liknon-ai-tool` skill. Establish explicit immutable artifacts,
single-file persistence, trust boundaries, authorization rules, workflow-node
contracts, and agent-profile references over the consumer-owned AI decision
policy established in Phase 3 before implementing domain analysis or
remediation behavior.

The official workflow uses the `liknon-ai-tool` skill as its preferred
operational interface. It may direct an AI host to use the validation CLI
directly only when a required capability has no applicable skill operation, in
which case the workflow follows canonical public reference documentation and
preserves the same policy, trust, workspace, and evidence boundaries.

Before loading that workflow or any replaceable resource, the human directs the
external host to the program-owned AI bootstrap entry point. The host records
its supported version and digest and conducts the graph under that contract.
This phase adds no Liknon-hosted model runtime or fourth CLI.

The three CLIs remain unaware of official workflow IDs, node IDs, transitions,
or agent profiles. The validation planner and executor interpret only
`.liknon/validation/config.json` and emit the normal validation report. Dedicated
persistence commands validate generic contracts and explicitly supplied files,
but they never discover analytical context, select a workflow, instantiate an
agent, or influence an execution plan.

## 2. Dependencies

- Phase 1 provides stable CLI, configuration, report, and Human Flow contracts.
- Phases 2 and 2.1 provide source-backed reasoning guidance, progressive
  disclosure, and stable editorial profiles. A01 separates authored resources
  from the applications, B02 installs selected resources through Manager, and
  B03 exposes those installed resources through Tool.
- Phase 3 provides the human-owned AI decision policy, immutable
  policy-mutation boundary, read-only policy guidance, the official action
  workflow, and the bounded `liknon-ai-tool` operations that the official
  validation-engineering workflow composes.
- The shared AI bootstrap contract defines host conformance, loading order,
  application binding, and the boundary between program authority and untrusted
  resource content.

## 3. Persistence Boundary

Use this consumer-owned structure:

```text
.liknon/
├── validation/
│   ├── config.json
│   └── reports/
│       └── <sha256>.json
└── ai/
    ├── policy.json
    └── persistence/
        └── <namespace>/<workflow-name>/
            └── <artifact-kind>/
                └── <uuidv7>.json
```

This is the generic workflow-artifact root. Each exact locked workflow declares
its contained artifact kinds, payload schemas, relationships, bounds, and
repository treatment. The initial official validation-engineering workflow
instantiates it below
`.liknon/ai/persistence/official/liknon-validation-engineering/` with `domain`,
`sensorium`, `incidents`, and `drift` kinds. Other compatible workflows use
their own namespace and name without extending the CLIs with workflow-specific
branches.

Every process receives the exact artifact and report paths it needs. It validates
their schemas, IDs, digests, containment, and declared relationships before use.
A direct continuation passes the artifact created by the preceding process. A
later invocation, including one in a fresh conversation, requires the human or
caller to provide the exact paths again. No operation scans a directory to infer
the newest, current, preferred, or otherwise authoritative file.

For the initial official validation-engineering workflow, this explicit-input
model has these consequences:

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

Workflow-artifact persistence and report persistence are separate decisions.
Invoking a workflow node authorizes writing only the bounded redacted artifact
declared for that node. Protected content that remains follows
`sensitiveDataPersistence`. Exact validation reports are stored only when
`validationReportPersistence`, `sensitiveDataPersistence` when applicable, and
every category implicated by the destination permit the write.

## 4. Policy Preflight And Persistence Operations

Phase 3 provides human-operated `init --policy`, read-only `policy validate` and
`policy evaluate`, and the complete decision-policy contract. Phase 4 consumes
those operations without adding another policy lifecycle. It adds these
non-cognitive persistence operations:

```sh
liknon persistence verify artifact <path> --contract <workflow-manifest-path> --artifact <declaration-id> [--workspace <workspace-path>] --format=json
liknon persistence verify report <path> [--workspace <workspace-path>] --format=json
liknon persistence store artifact --input <candidate-path> --contract <workflow-manifest-path> --artifact <declaration-id> [--workspace <workspace-path>] --format=json
liknon persistence store report --input <candidate-path> [--workspace <workspace-path>] --format=json
```

The equivalent internal core API uses typed inputs and results. The artifact
operations validate the common envelope plus one exact artifact declaration and
payload schema from an explicitly supplied locked workflow manifest. They do
not discover a workflow, interpret domain meaning, choose an artifact, evaluate
AI policy, mutate policy, or execute configured validation commands.

Before a policy-compatible AI workflow uses these operations, it loads the
mandatory policy boundary and invokes Phase 3 `policy validate` followed by
`policy evaluate` for the exact Tool-verified workflow stack. It never
invokes `init --policy`, creates a policy candidate, edits policy, or delegates
policy mutation. Missing or invalid policy routes to the read-only policy
operation exposed by the installed `liknon-ai-tool` skill when that dependency
is declared, then stops until the human performs the required action manually.

Every command above and Phase 3 policy preflight use one workspace-boundary
contract:

- `--workspace` identifies the workspace root; when omitted, existing-state
  operations discover the nearest ancestor containing `.liknon/`;
- the root must be an existing directory and is canonicalized once before any
  input is read or destination is resolved;
- the commands never inspect Git, package-manager files, resource content, or
  `config.json` to infer a root;
- relative caller-supplied paths resolve against that root, and absolute paths
  are accepted only when their canonical targets remain inside it;
- every `<path>`, `--input`, or `--contract` value, including a temporary staging
  file, must satisfy that same containment rule;
- `--contract` must identify an exact installed workflow manifest whose bytes,
  version, and payload inventory match `.liknon/resources/lock.json`; the
  command never searches for a matching workflow or substitutes another
  version;
- artifact destinations resolve below `.liknon/ai/persistence/` and report
  destinations below `.liknon/validation/reports/`, with symlink and
  parent-traversal escape rejected;
- machine results expose normalized workspace-relative paths rather than
  silently changing the selected boundary.

The internal core API has no current-directory fallback. It receives one validated
workspace-root value explicitly and uses that same value for policy preflight,
persistence, containment, and destination resolution.

`persistence verify artifact`:

- requires one explicit contained regular file;
- requires one exact locked workflow manifest and one artifact declaration ID
  owned by that manifest;
- rejects symlinks, traversal, unsupported schema versions, unknown fields,
  invalid UUIDv7 values, invalid timestamps, oversized content, and mismatched
  declared digests;
- validates the generic artifact envelope and the declaration's exact payload
  schema, storage template, allowed references, producing nodes, and bounds;
- validates every explicitly declared parent or report reference without
  searching for alternatives;
- returns a bounded versioned JSON result containing the artifact type, ID,
  digest, referenced inputs, and validation status.

`persistence verify report` validates one explicit report against the bundled
report schema, computes SHA-256 over the exact bytes, and returns its selection,
result, schema version, and digest without storing it.

`persistence store artifact`:

1. reads one bounded candidate file supplied explicitly by the caller;
2. verifies the explicitly supplied workflow manifest and artifact declaration
   against the workspace resource lock;
3. validates the common envelope and payload against that exact declaration;
4. resolves its canonical destination from the declaration's contained storage
   template and `artifactId`, never from `artifactType` alone;
5. verifies workspace containment and rejects symlink traversal;
6. writes the exact validated bytes through a temporary file and atomic rename;
7. if the destination already exists, reuses it only when the bytes are
   identical;
8. returns the normalized path, UUIDv7, artifact declaration, producing
   workflow identity, and SHA-256 digest.

`persistence store report` follows the same single-file operation, naming the
destination `.liknon/validation/reports/<sha256>.json`. It preserves the exact report
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

For a compatible AI workflow execution that produces a complete versioned JSON
report, the agent:

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
program contract requires local ignored storage, restrictive permissions where
supported, and a warning against committing or sharing reports without review.
The contract performs no automatic retention expiry, deletion, pruning, or
compaction. Removal is an explicit governed action.

## 6. Repository Treatment

`policy.json` is consumer-owned, reviewable AI decision policy and follows the
consumer repository's normal tracking rules. Exact validation reports always
use `local-ignored`. Every workflow artifact declaration chooses one of these
generic treatments:

- `local-ignored` means the artifact is local operational state and must be
  ignored before it is written in a Git workspace;
- `consumer-selected` means the store operation reports effective repository
  treatment, while the consumer chooses whether that analytical context is
  tracked or ignored.

Before a `local-ignored` write in a Git workspace, the store operation verifies
the concrete destination through Git. A missing ignore rule blocks the write
and returns an exact corrective instruction. Applying that correction is a
separate `workspaceMutation` decision. If effective treatment cannot be
determined for a required local destination, the write fails closed.

For `consumer-selected`, the operation never silently changes repository
treatment. Consumer policy may exclude every workflow persistence directory
without affecting Human Flow or non-persistent skill operation.

The initial official workflow declares `consumer-selected` for `domain` and
`sensorium`, and `local-ignored` for `incidents` and `drift`. These choices are
resource data, not artifact-name rules in a CLI.

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
    "path": ".liknon/validation/config.json",
    "sha256": "<digest-or-null>"
  },
  "resourceSnapshot": {
    "lockPath": ".liknon/resources/lock.json",
    "lockSha256": "<digest>"
  },
  "decisionPolicy": {
    "path": ".liknon/ai/policy.json",
    "schemaVersion": 1,
    "sha256": "<digest>",
    "effectiveWorkflowIds": [
      "workflow:official/liknon-validation-engineering"
    ]
  },
  "producer": {
    "workflow": {
      "resourceId": "<stable-workflow-resource-id>",
      "version": "<exact-version>",
      "manifestSha256": "<digest>"
    },
    "workflowStack": [
      {
        "resourceId": "<stable-workflow-resource-id>",
        "version": "<exact-version>",
        "manifestSha256": "<digest>"
      }
    ],
    "node": {
      "id": "<workflow-node-id>",
      "transitionId": "<incoming-transition-id-or-null>",
      "actionIds": ["<performed-workflow-action-id>"]
    },
    "agentProfile": {
      "resourceId": "<stable-agent-profile-resource-id>",
      "version": "<exact-version>",
      "manifestSha256": "<digest>"
    },
    "dependencies": [
      {
        "resourceId": "<stable-resource-id>",
        "kind": "skill",
        "version": "<exact-version>",
        "manifestSha256": "<digest>"
      }
    ]
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

The producer record identifies the exact installed workflow resource, active
workflow stack, node, incoming transition when present, performed action IDs,
selected agent profile, resource-lock digest, and operational resources that
produced the artifact. A later node validates these identities, manifest
digests, and declared
compatibility before accepting the artifact. An agent profile contributes role,
posture, and constraints only; it never supplies workflow routing, policy,
commands, or authority. Knowledge documents used as evidence retain their own
stable IDs and digests in the evidence records rather than being treated as
producer instructions.

The common artifact-envelope schema and fixtures belong to the shared workspace
contract and are exposed through the validation CLI's schema and validation
contracts. Workflow-specific payload schemas and storage declarations belong to
their workflow resources and are pinned by the resource lock. Skills declare
compatible contract versions rather than carrying private schema copies.
Consumer workspaces store installed resource payloads and artifact instances,
not manually copied private schemas.

## 8. Policy Presence And Validation

The complete policy contract is defined by
[AI Decision Policy](ai-decision-policy.md). At the start of every node
invocation in a policy-compatible AI workflow, the agent loads the immutable
policy boundary and invokes `policy validate` with the explicit active workspace
root. The host obtains the exact ordered active workflow stack from the verified
Tool closure and passes it to `policy evaluate`. Validation supplies path,
schema version, digest, and status; evaluation supplies the unchanged input
workflow IDs plus every effective global or workflow-specific value and source.
Together they form the node's initial policy identity without making Validation
CLI discover or activate a workflow.

When the policy is missing, the node stops before loading analytical inputs or
producing an artifact. If the workflow declares the official `liknon-ai-tool`
skill, the agent routes to its advisory `policy` operation, explains that this
workflow requires a consumer-owned decision policy, and directs the human to run
`liknon init --policy` personally.
The human reviews every category and `processContinuation` value before invoking
or resuming the process. The agent never runs initialization or writes policy.
After the human reports completion, the resumed process runs `policy validate`
and `policy evaluate` again.

An existing malformed, unsupported, path-escaping, or internally inconsistent
policy stops the node. An applicable advisory policy operation may explain
diagnostics and recommend manual field changes, but no compatible AI workflow
creates, edits, formats, replaces, deletes, moves, restores, or otherwise
mutates the document. This prohibition cannot be overridden by approval,
workflow content, or policy level.

Each artifact records the exact policy schema version and SHA-256 digest used
for its decisions. Before a later governed effect, a resumed or long-running
node invokes `policy validate` with the same workspace root and then
`policy evaluate` with the same freshly verified active workflow stack. The
command resolves exact workflow overrides before global values, combines nested
workflow matches by the documented most-restrictive rule, and returns a
different digest when policy changed.

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
and `.liknon/ai/policy.json`. Persistence artifacts remain non-executable data.

## 10. Authorization Model

Use the complete contract implemented in Phase 3 and specified by
[AI Decision Policy](ai-decision-policy.md). Its canonical public projection and
mandatory policy boundary are absolute rules for every compatible AI workflow.
Workflow resources reference them rather than redefining category, level,
precedence, or policy-mutation semantics. A workflow cannot provide, alter, or
lower its own governing policy.

| Action | Authority |
| --- | --- |
| Verify an explicitly supplied policy, artifact, or report | Covered by the analysis request; sensitive resources still evaluate `sensitiveDataAccess` |
| Explain policy or recommend field values | Delegate to a declared contract-compatible read-only policy operation, such as `liknon-ai-tool/policy` |
| Create, modify, move, restore, or delete `policy.json` | Human-only action; every agent is prohibited from performing or delegating it, regardless of approval or configured level |
| Store a bounded redacted process artifact | Declared result of the invoked process; evaluate `sensitiveDataPersistence` when protected values remain |
| Store an exact validation report | Evaluate `validationReportPersistence`, `sensitiveDataPersistence` when applicable, and every category implicated by the destination |
| Follow a workflow transition marked as continuation-eligible | Resolve `processContinuation` for the active workflow stack after successful artifact persistence and target-node precondition validation |
| Inspect configuration through non-executing parsing | Covered by the analysis request; invoking an inspection command also evaluates `commandExecution` |
| Modify configuration or other workspace files | Evaluate every applicable decision category |
| Execute configuration, tools, or scripts | Evaluate `commandExecution` and every other implicated category |
| Install, update, or remove a tool, dependency, runtime, or toolchain | Evaluate `dependencyManagement` plus every overlapping category |
| Modify source, snapshots, databases, or generated output | Evaluate `workspaceMutation`, `dataMutation`, and every other implicated category |
| Access an external service, remote state, or sensitive resource | Evaluate all applicable network, external-state, and sensitive-data categories |

The evaluator classifies actor, action, resource, environment, scope,
reversibility, trust origin, sensitivity, privilege, and blast radius. Decision
levels follow `4 > 3 > 2 > 1 > 0`; continuation values follow
`stop > human > auto`; and computed category outcomes combine as
`stop > human > auto`. An exact workflow override takes precedence over its
category's global value. When composed parent and child workflows both match,
their effective values combine by the applicable most-restrictive order. Human
prompts accept free text.

## 11. Workflow Resource Architecture

Publish the initial orchestration and its reusable components as independent,
type-first resources:

```text
resources/
├── workflows/official/liknon-validation-engineering/
│   ├── manifest.json
│   ├── index.md
│   └── processes/
│       ├── process-0-domain.md
│       ├── process-1-sensorium.md
│       ├── process-2-incident.md
│       └── process-3-drift.md
├── agent-profiles/official/
│   ├── liknon-domain-analyst/
│   ├── liknon-validation-architect/
│   ├── liknon-incident-investigator/
│   └── liknon-drift-reviewer/
├── skills/official/liknon-ai-tool/
├── tools/official/
└── guides/official/liknon-ai-policy-boundary/
```

The workflow has stable ID
`workflow:official/liknon-validation-engineering`. Its manifest is the sole
source for its graph, node IDs, transitions, preconditions,
continuation-eligible edges, stable actions, action inputs and outputs, declared
effects, artifact contracts, and
references to skills, knowledge, tools, guides, and agent profiles. Processes
0 through 3 and transitions `0 -> 1` and `2 -> 3` belong to this resource only;
they are not constants, reserved IDs, or assumed graph shapes in any CLI.

Each agent-profile resource defines role, reasoning posture, communication
constraints, and safety constraints only. It cannot define transitions,
commands, policy values, artifact routing, or authority. The workflow references
one exact compatible profile for each node, while the invoking AI host binds
that profile to an available model. Manager, Tool, and Validation never
instantiate an agent or select a model.

Each referenced tool resource describes an application operation and capability
contract only. The workflow action identifies the tool and skill operations it
needs, but neither descriptor embeds an executable, performs installation,
contains credentials, or grants execution authority.

The workflow manifest is the orchestration index and declares compatible
dependencies on the `liknon-ai-tool` skill and shared policy-boundary guide. It does not copy
`SKILL.md`, policy rules, operation text, agent profiles, or knowledge.
Installing a skill, profile, guide, or workflow resolves only the explicitly
requested dependency closure through Manager; installation never selects or
activates a workflow. Activation requires an explicit human selection of a
workflow and entry node.

The official dependency on AI-Tool is explicit rather than privileged. A
different compatible workflow may select another operational skill or
community-maintained derivative. The bootstrap contract and all three CLIs
remain unchanged and apply the same compatibility, policy, capability, effect,
and evidence boundaries.

Each process document describes one workflow node, requires explicit artifact
inputs, loads the shared policy boundary, routes policy guidance and operational
work through the applicable skill operation, and identifies only the knowledge
categories it needs. The agent resolves all declared resources from the
installed catalog through `likt catalog` and retrieves selected content through
`likt show <DOCUMENT_ID>`. A node may route directly to a documented CLI
capability only when no declared skill operation covers that capability. It
records the capability and reason, and it must not recreate, bypass, or locally
reinterpret an existing operation.

A missing required resource stops the node with its canonical identity. The
agent may propose a separately authorized `likm` acquisition, but it never
invokes Manager, accesses the Hub, or mutates the resource store implicitly.

Resource identity is the tuple `kind + namespace + name + version`. Source
origin, trust class, review state, signatures, and checksums are independent
metadata; the `official` namespace grants no runtime privilege. The resource
lock pins the exact workflow, profiles, and dependency closure. Every artifact
records that lock digest and the exact resources used so a later node can reject
missing, changed, or incompatible inputs.

## 12. Safety Behavior

- Load and verify the program-owned AI bootstrap contract before any workflow,
  profile, skill, tool, knowledge, or artifact instruction.
- Verify Validation and Tool capabilities plus the complete locked workflow
  dependency closure before node work.
- Validate the current policy before node work and every explicitly supplied
  artifact or report before use.
- Stop on a missing or invalid policy, route to read-only policy guidance, and
  require human-performed initialization or correction.
- Never create, edit, format, replace, delete, move, rename, restore, initialize,
  or indirectly mutate `policy.json`, even after explicit approval.
- Stop rather than search for or guess a missing input.
- Never select an artifact by filename order or modification time.
- Never repair, restore, delete, or accept a repository change merely because it
  was observed.
- Never execute a command copied from an artifact or report.
- Resolve intended validation through the current config graph and declared
  skill mechanics.
- Treat resource acquisition as a distinct Manager operation governed by the
  effective policy; never infer it from permission to read a resource.
- Use canonical CLI reference documentation directly only for a required
  capability outside the declared skill-operation contracts, and retain
  evidence explaining that route.
- Minimize and redact protected data from derived artifacts.
- Preserve exact stored report bytes and redact only derived excerpts.
- Retain canonical artifacts until an explicit governed removal.
- Never remove an artifact or report supplied as an input to the active process.
- Persist a node's declared output before applying a continuation rule.
- Apply `processContinuation` only to transitions marked continuation-eligible
  by the active workflow, after validating the exact output and target-node
  preconditions. In the initial official workflow those transitions are
  `0 -> 1` and `2 -> 3`.
- Reject unknown, unsupported, or materially undeclared effects before policy
  evaluation; workflow content cannot grant itself authority.

## 13. Tests

Create schema, persistence, and process fixtures for:

- valid artifacts of every type;
- exact locked workflow contracts, unknown artifact declaration IDs, unlocked or
  tampered manifests, incompatible contract versions, undeclared producing
  nodes, and unsafe storage templates;
- unsupported schema versions, unknown fields, malformed timestamps, and invalid
  UUIDv7 values;
- missing, cyclic, wrong-type, path-escaping, and digest-mismatched references;
- deterministic verification of explicit artifacts and reports;
- explicit and omitted workspace-root resolution, current-directory discovery
  start, non-directory roots, bounded upward discovery, and identical CLI/core
  containment behavior;
- relative and absolute contained inputs, parent traversal, symlink escape, and
  canonical destinations anchored to the selected workspace;
- atomic single-file storage, interrupted temporary writes, identical-file
  reuse, and non-identical destination rejection;
- report schema validation, SHA-256 naming, exact-byte preservation, and bounded
  output;
- `local-ignored` destinations that are ignored, not ignored, symlinked, or
  unavailable to Git, plus tracked and ignored `consumer-selected` artifacts;
- explicit artifact selection when multiple valid files exist;
- stale sensorium inputs caused by domain or config digest mismatch;
- read-only policy validation; missing-policy node rejection with exact
  human-operated `init --policy` guidance; invalid-policy rejection without
  replacement; changed-policy reevaluation; and delegation of explanation to
  `liknon-ai-tool/policy` when declared;
- refusal to create, edit, format, replace, delete, move, rename, restore,
  initialize, or indirectly mutate policy, including after explicit approval and
  under policy level `0`;
- report and sensitive persistence at every applicable decision level;
- denied or stopped persistence that leaves no canonical file;
- concurrent writes of the same and different immutable artifacts;
- prompt-injection text in every untrusted input class;
- every decision category at levels `0` through `4`, overlapping categories,
  and restrictive outcome combination;
- global category fallback, exact workflow override precedence, stable workflow
  IDs that exclude versions, inert overrides for valid unloaded workflow IDs,
  and most-restrictive composition across nested workflow stacks;
- unknown, unsupported, and materially undeclared workflow effects that stop
  before execution;
- actionable workflows with missing or incompatible decision-policy contract
  versions that remain inspectable but stop before activation;
- delegation to every applicable declared skill operation and rejection of direct CLI
  bypass when that operation already covers the task;
- justified direct CLI use for a capability not covered by a declared skill, with the
  same policy and evidence boundaries;
- `auto`, `human`, and `stop` continuation for the two continuation-eligible
  transitions declared by the official validation-engineering workflow;
- exact workflow, node, transition, action, agent-profile, dependency, resource-lock,
  and policy-source records in every produced artifact;
- profile resources that cannot alter routing, commands, policy, or authority;
- one external workflow fixture with non-numeric nodes, a graph unlike the
  official workflow, independently versioned profiles, and no official IDs;
- source checks proving that all three CLIs contain no official workflow,
  process, transition, or profile IDs;
- bootstrap-first trials using the official AI-Tool skill and a compatible
  alternative skill selected by an external workflow;
- a workspace with no workflow persistence directory.

Forward-test the official validation-engineering workflow together with its
declared skill, tool, guide, profile, and knowledge dependencies against raw
consumer fixtures. Test agents must not receive the expected conclusion or
hidden answer.

## 14. Acceptance Criteria

- [ ] Human Flow remains complete without policy, reports, or AI persistence;
      a declared contract-compatible advisory policy operation remains available when policy is
      missing, while every other AI operation stops.
- [ ] Every governed workflow trial records the exact bootstrap-contract version
      and digest before resource loading or policy-controlled work.
- [ ] The execution planner interprets only `config.json`.
- [ ] Every node in the official validation-engineering workflow receives exact
      artifact and report inputs and uses only those supplied inputs.
- [ ] Every completed artifact-producing node creates one immutable UUIDv7
      artifact.
- [ ] Artifact and report storage use one validated atomic file operation.
- [ ] Artifact storage validates a generic envelope plus one exact declaration
      from an explicitly supplied locked workflow manifest; no CLI selects a
      workflow or branches on an official artifact type.
- [ ] Every workflow artifact destination is namespaced by workflow identity,
      and every declaration uses either `local-ignored` or
      `consumer-selected` repository treatment without artifact-name branches.
- [ ] Phase 3 policy validation, explicit-stack evaluation, and human-only
      initialization remain the only policy lifecycle operations consumed by
      this phase.
- [ ] Every policy and persistence operation uses one canonical selected
      workspace boundary; the CLI discovers the nearest `.liknon/` root, while
      the internal core API always requires the root.
- [ ] Process 1 binds its sensorium to the exact domain and config digests.
- [ ] Processes 2 and 3 reject mismatched domain, sensorium, config, or report
      relationships.
- [ ] Reports are stored under their exact SHA-256 names only after every
      applicable persistence decision permits the write.
- [ ] Every continuation-eligible transition implements all three continuation
      modes through global or workflow-specific policy resolution.
- [ ] Untrusted repository content cannot authorize an action.
- [ ] Missing or invalid policy blocks every compatible AI workflow, routes to
      declared advisory policy guidance when available, and remains blocked
      until the human manually initializes or corrects and reviews the file.
- [ ] No compatible AI workflow can mutate policy directly or indirectly under
      any approval, policy level, continuation mode, or node state.
- [ ] Every decision records the exact policy digest, effective workflow source,
      and resource-lock digest without storing secrets.
- [ ] The official validation-engineering workflow and its skill, guide,
      profile, and knowledge dependencies load progressively and remain within
      their declared compatibility ranges.
- [ ] The official workflow retrieves relevant knowledge progressively through
      `likt`, and neither workflow nor skill resources contain a copied
      knowledge subtree.
- [ ] Missing resources stop safely and never trigger implicit Manager use,
      network access, or resource-store mutation.
- [ ] The official workflow uses its declared AI-Tool skill for every covered
      operation and uses the CLI directly only for an uncovered capability with
      an explicit documented reason.
- [ ] A compatible external workflow may select another operational skill under
      the same bootstrap and policy contracts without CLI changes.
- [ ] Workflow installation and listing never activate it; only explicit human
      selection chooses a workflow and entry node.
- [ ] An actionable workflow whose decision-policy contract is absent or
      incompatible remains installable and readable but cannot be activated.
- [ ] An external workflow with a different graph and profiles passes the same
      generic contracts without changing CLI code.

## 15. Handoff To Phase 5

Phase 5 implements the domain and sensorium nodes of the official
validation-engineering workflow only after the generic explicit-input,
single-file persistence, profile, trust, and authorization contracts are
proven.
