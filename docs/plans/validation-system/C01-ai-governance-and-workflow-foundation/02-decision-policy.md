# Phase C01.2: Define The AI Decision Policy

## Status

`Provisional`. Policy semantics are defined here; deterministic operation
ownership and public command forms are blocked by C01.0 and the relevant B
track decision gates.

[Back to Plan C01](README.md)

## 1. Status And Purpose

This document specifies the consumer-facing AI decision policy that every
Liknon AI workflow or bounded AI operation declaring compatibility must obey.
The official action and validation-engineering workflows are initial consumers,
not privileged policy subjects. This is a cross-cutting contract, not a
workflow definition or a validator execution contract.

For governed use, an external AI host loads the program-owned
[AI Bootstrap And Conformance Contract](01-bootstrap-and-conformance.md)
before this policy or any installed resource. This document specializes
decision authority; it is not an alternate AI entry point and does not select a
workflow, skill, agent profile, or model.

The policy contract requires one canonical authored public document. A
candidate location, subject to the A01 ownership ledger, is:

```text
docs/validation/reference/ai-decision-policy.md
```

A01 and C01 select the final authored location before implementation. A
policy-boundary guide is generated deterministically from that source and
treats it as normative; B02.0 and the owning official-resource plan select the
guide's final resource identity and physical path. Compatible skills and
workflows declare that guide as an ordinary exact resource dependency and load
it before reading policy-controlled workspace data. They load the full
contract before classifying or performing a governed effect.

The document must define every category and every level explicitly. An agent
must not infer, rename, merge, or locally reinterpret their meanings. Every
actionable workflow executed as part of the governed Liknon system declares a
compatible policy-contract version; an incompatible or undeclared version may
be inspected as content but stops before activation.

### 1.1 Design Foundations

The public document records the following design basis and labels the resulting
liknon contract as a local synthesis:

- NIST SP 800-162 Attribute Based Access Control for evaluating actor, action,
  resource, and environment attributes;
- Parasuraman, Sheridan, and Wickens, *A Model for Types and Levels of Human
  Interaction with Automation*, for assigning different automation levels to
  different functions rather than one global autonomy level;
- the Policy Decision Point and Policy Enforcement Point separation documented
  by Open Policy Agent, including auditable decision records;
- OASIS XACML restrictive rule combination, adapted here as
  `stop > human > auto`;
- NIST AI RMF human-AI oversight responsibilities;
- OWASP AI Agent Security guidance for risk-aware action classification,
  explicit previews, reversibility, human-in-the-loop controls, and audit
  trails.

The canonical shared source register at `docs/validation/knowledge/sources.json`
contains exact editions, stable source IDs, primary URIs, precise locations,
scope, and review dates. `docs/validation/knowledge/SOURCES.md` is its generated
human-readable projection. The five numeric levels and category tables are this
project's normative synthesis; the document must not misrepresent them as a
standard published verbatim by any one source.

## 2. Separate Policies

The consumer workspace has two policies with different owners and effects:

| File | Responsibility | Consumer |
| --- | --- | --- |
| `.liknon/validation/config.json` | Defines the programs, arguments, composition, and repository-mutation detection used for validation | Validation CLI and the validation owner selected by A01 |
| `.liknon/ai/policy.json` | Defines global and workflow-specific rules for when a compatible AI agent acts automatically, requests a free-text human decision, or stops | Compatible AI operations and workflows |

`policy.json` never contributes commands, checks, suites, groups, arguments, or
validation outcomes. The validation executor and planner do not discover, read,
or interpret it. Every compatible workflow validates its schema and digest before applying
policy decisions. `config.json` remains the sole executable validation policy.
When a governed AI operation proposes a Manager operation, the proposal is
governed by this policy like any other command, network, or workspace effect.
Manager does not interpret `policy.json` itself, and permission to read through
Tool never implies permission to acquire or mutate resources through Manager.

## 3. Canonical Policy Shape

The initial contract is:

```json
{
  "schemaVersion": 1,
  "processContinuation": {
    "global": "human",
    "workflows": {}
  },
  "decisionLevels": {
    "workspaceMutation": { "global": 4, "workflows": {} },
    "scopeExpansion": { "global": 4, "workflows": {} },
    "dependencyManagement": { "global": 4, "workflows": {} },
    "commandExecution": { "global": 4, "workflows": {} },
    "networkAccess": { "global": 4, "workflows": {} },
    "externalStateMutation": { "global": 4, "workflows": {} },
    "versionControlMutation": { "global": 4, "workflows": {} },
    "dataMutation": { "global": 4, "workflows": {} },
    "sensitiveDataAccess": { "global": 4, "workflows": {} },
    "validationReportPersistence": { "global": 4, "workflows": {} },
    "sensitiveDataPersistence": { "global": 4, "workflows": {} }
  }
}
```

The conservative document generated by the approved human-operated policy
initialization operation contains no workflow IDs. A human may later add an
exact stable ID to any `workflows` map, for example:

```json
{
  "global": "human",
  "workflows": {
    "workflow:official/liknon-validation-engineering": "auto"
  }
}
```

This fragment illustrates one field only; it is not a complete policy and is
never emitted as a built-in workflow exception.

The shared workspace contract owns and distributes the JSON Schema. Consumer
workspaces store the policy document, not a schema copy. Every listed category
is required. Missing categories, unknown fields, and unsupported values are
rejected rather than resolved through an implicit fallback.

Each workflow key is a canonical stable resource ID containing kind, namespace,
and name but no version. The resource lock fixes the exact version used by one
execution. A syntactically valid ID need not already be installed, which lets a
human prepare policy before acquisition; an unmatched ID is inert and should be
reported as unresolved guidance rather than silently matched by title or path.

Only the human may add, remove, or change an override. A workflow manifest may
declare effects and policy compatibility, but it cannot supply an effective
policy value, request its own exception, or lower a global setting.

### 3.1 Generic Persistence Contract

`policy.json` selects decision levels; it does not configure storage paths,
retention, transport, or workflow-specific artifact kinds. The program-owned AI
bootstrap contract defines universal persistence boundaries. Each exact locked
workflow manifest owns its artifact declarations and may narrow, but never
weaken, those boundaries.

Every persistable workflow artifact declaration includes:

- one stable declaration ID and artifact kind;
- one exact payload schema and bounded common-envelope version;
- one contained storage template rooted below
  `.liknon/ai/persistence/<namespace>/<workflow-name>/`;
- producing node and action IDs plus allowed parent and report relationships;
- sensitivity, minimization, redaction, byte, and reference bounds;
- one repository treatment: `local-ignored` or `consumer-selected`;
- the fixed initial lifecycle of immutable creation and explicit governed
  removal, with no automatic expiry, compaction, pruning, or replacement.

Unknown treatments, paths outside the workflow's canonical storage root,
undeclared executable content, lifecycle commands, and attempts to relax the
bootstrap or policy contracts fail closed. A workflow cannot use its manifest to
authorize persistence; the host still evaluates every applicable policy
category for the concrete artifact and destination.

Exact validation reports are a program contract rather than a workflow artifact
declaration. They retain their exact bytes under
`.liknon/validation/reports/<sha256>.json` and always use `local-ignored`
treatment.

Concrete artifact names and destinations belong to each workflow declaration.
C03 defines those details for the official validation-engineering workflow.
An unrelated compatible workflow declares its own namespaced artifact kinds and
receives the same validation without changing a CLI.

The canonical local private store consists of reports and every declaration
using `local-ignored`. Exporting or sharing content elsewhere is a separate
proposed effect and must evaluate every category implicated by that exact
boundary, including network or external-state categories where applicable; it
does not create another implicit persistence destination.

Artifacts contain explicit paths, digests, bounded decisions, and analytical
results; they do not duplicate report bodies, raw diagnostic streams,
credentials, tokens, private keys, or source patches. Derived artifacts minimize
and redact protected content whenever it is not required. Files use restrictive
permissions where the platform supports them, paths remain within the workspace
without symlink escape, and credentials are never persisted in analytical
artifacts.

The initial generic lifecycle performs no automatic retention expiry, deletion,
history compaction, or pruning. Content remains until an explicit governed
removal. Removal of an artifact or report supplied to an active workflow node is
invalid until that node ends. These lifecycle facts are fixed operational inputs
to the category tables below; they are not inferred from a numeric level.

## 4. Human Ownership, Validation, And Change

### 4.1 Non-Configurable Mutation Boundary

`.liknon/ai/policy.json` is created and changed only by the human operator. No
agent in any compatible flow may create, edit, format, replace, delete, move, rename,
restore, or otherwise mutate it. The prohibition includes direct file APIs,
patches, shell or structured commands, scripts, VCS operations, generated
candidate files intended for replacement, and indirect delegation to another
tool or agent.

This rule is outside and above `policy.json`. It cannot be weakened by level
`0`, `processContinuation`, a category outcome, an explicit approval, or a user
request asking the agent to perform the mutation. The agent may recommend
specific values and explain an exact manual procedure, but the human enters and
applies every effective value.

The approved policy initialization operation is human-operated. It creates the
missing canonical conservative document through the deterministic no-overwrite
initialization contract. When the file exists, it validates and reuses it
without modification. An agent may display the approved invocation after its
owning B track defines it, but must never invoke it or ask for permission to
invoke it.

### 4.2 Preflight For Compatible AI Workflows

Policy cannot authorize the commands required to locate and evaluate itself.
The bootstrap contract therefore defines one fixed, non-configurable
pre-policy allowlist. It permits only:

1. reading the package-owned bootstrap source directly or invoking the approved
   read-only bootstrap projection;
2. invoking metadata-only Tool lookup and resolution for the exact workflow or
   standalone skill explicitly selected by the human, solely to obtain its
   locked root-to-leaf identity, dependency closure, compatibility metadata,
   and declared policy-boundary references;
3. retrieving only the exact locked policy-boundary guide and advisory policy
   operation declared by that verified closure;
4. invoking the approved read-only policy-validation operation for the explicit
   active workspace root; and
5. only after validation succeeds, invoking the approved read-only
   policy-evaluation operation with that exact ordered root-to-leaf stack, or
   with an empty workflow stack for a human-selected standalone skill.

These operations are pre-authorized by the immutable bootstrap and policy
contracts, not by a numeric policy level. They are exempt only from the
`commandExecution` decision needed to establish the effective policy. Every
path, workspace, identity, version, digest, output bound, and no-mutation
condition still applies. The allowlist permits no Hub or other network access,
Manager invocation, generic resource search, undeclared payload retrieval,
validation execution, repository analysis, workspace mutation, persistence,
or workflow-node work.

A valid policy-validation result fixes the exact path, schema version, and
digest used for subsequent decisions. Policy evaluation returns every effective
category and continuation value and its global or matching-override source.
Validation never invents workflow context, and evaluation never discovers or
activates a workflow.

The official advisory policy operation may use the same allowlist while the
document is missing or invalid. Other resources may provide an equivalent
read-only operation only when they declare and satisfy the same contract. In
that state, the operation is limited to validation, explanation, and
recommendations.

When `policy.json` is absent, every other AI operation stops before repository
analysis, execution, mutation, or artifact creation. The agent routes to policy
guidance and directs the human to invoke the approved policy-initialization
operation personally, review `processContinuation` and every category, and then
report completion. The agent does not execute initialization even when the
human offers approval.

An existing malformed, unsupported, path-escaping, or internally inconsistent
policy also stops every other AI operation. Policy guidance explains the
diagnostics and may recommend field values, but only the human edits the file.
Defaults are never applied over an existing document.

After the human reports a manual creation or change, the requested operation
runs policy validation and evaluation again before continuing. No
persistent reviewed marker is added. A policy change invalidates every cached
authorization decision. It does not by itself invalidate domain or sensorium
evidence because it does not change validation coverage.

Persisted workflow artifacts record the exact policy path, schema version,
SHA-256 digest, active workflow identities, and effective global or override
source used for their decisions. Bounded non-persistent operations report those
facts without creating analytical persistence merely to record them.

## 5. Process Continuation

`processContinuation` is a three-mode policy applied only to transitions that
the active workflow declares continuation-eligible. It is not part of the
numeric decision scale:

| Value | Required behavior |
| --- | --- |
| `auto` | After the source node completes successfully, persists every declared result, and satisfies every target-node precondition, traverse the declared continuation edge without another question. |
| `human` | Present the completed result, suggest the declared eligible transition, and wait for a free-text human decision. |
| `stop` | Present the completed result and end the current workflow sequence without suggesting or asking to traverse the edge. |

The platform defines no process numbers, mandatory phases, or fixed edges. A
failed, blocked, interrupted, stale, or incomplete node never continues
automatically. `auto` does not bypass artifact, report, digest, configuration,
policy, or target-node preconditions. `stop` does not prevent the human from
invoking another valid entry point explicitly later.

The policy uses three distinct concepts:

- **continuation mode** is the effective global or workflow-specific
  `processContinuation` value for one declared continuation edge;
- **decision level** is the configured integer from `0` through `4` for one
  decision category;
- **decision outcome** is the computed `auto`, `human`, or `stop` result for one
  concrete governed effect.

Continuation mode is not derived from decision levels, and decision outcomes do
not create additional process-continuation relationships.

### 5.1 Workflow Override Resolution

The deterministic policy-evaluation operation resolves each category and
`processContinuation` as follows:

1. accept the exact ordered active workflow stack supplied by the caller after
   the host obtains it from the locked resource graph;
2. collect exact-ID overrides for those workflows;
3. when no override matches, use `global`;
4. when one override matches, use it instead of `global`;
5. when composed parent and child workflows both match, use the most restrictive
   matching value: numeric `4 > 3 > 2 > 1 > 0`, or continuation
   `stop > human > auto`.

A bounded compatible skill operation invoked without an active workflow has an
empty workflow stack and therefore uses `global`. It cannot manufacture a
workflow identity to obtain an override.

The evaluation result echoes the exact input stack and policy digest and reports
the source of every effective value. It does not verify graph membership,
classify a proposed action, compute category outcomes for an undisclosed action,
or authorize execution. Those responsibilities remain with the conforming host
under the bootstrap contract.

Absolute prohibitions and superior instructions remain above both global and
workflow-specific values. After resolving one level for every applicable
category, compute each outcome and combine outcomes by
`stop > human > auto`.

## 6. Decision Evaluation Model

Before a governed effect in any compatible AI workflow, the agent constructs a bounded
action preview that
identifies:

- active workflow stack, node, operation, and objective;
- operation and exact target resources;
- matching decision categories;
- whether the action is explicitly in scope;
- trust origin of commands, code, data, and endpoints;
- local, network, remote, or production environment;
- read-only, reversible, destructive, or irreversible effect;
- affected data sensitivity and required privilege;
- expected blast radius and rollback path;
- rationale and material uncertainty.

Workflow manifests use a canonical capability and effect vocabulary. The agent
must also classify effects evident from the concrete proposed action rather than
trusting declarations blindly. An unknown effect, an unsupported capability, or
an observed material effect omitted from the manifest evaluates to `stop` until
the contract is corrected; no workflow-specific override can make it eligible.

This follows an attribute-based model: decisions depend on the actor, action,
resource, and current environment rather than on the action name alone.

An **eligible effect** is relevant to the current objective, permitted by every
superior instruction and repository rule, supported by the available evidence,
within the agent's actual permissions, and valid under all applicable operation
or process preconditions. Level `0` automates only eligible effects; it does not make an
otherwise prohibited or unrelated effect eligible.

Every applicable category evaluates independently to `auto` or `human`. A hard
precondition, superior instruction, unsupported policy, or prohibited action
evaluates to `stop`. Combine overlapping results in this order:

```text
stop > human > auto
```

One human prompt may cover one coherent eligible action set, but it must show
every category that required human authority. Approval is bound to the presented
targets and effects; it does not authorize later scope expansion or any policy
mutation. Prompts accept free text so the human can approve, reject, constrain,
postpone, or report a manually completed action.

A precise initiating request can provide the required human authority for an
eligible exact action and scope it names. It never authorizes an agent to mutate
policy. Invoking a workflow node alone authorizes only that node's declared
persistence outputs; it does not preauthorize config, source, dependency,
command, network, external-state, VCS, data, or sensitive resource effects.

Automatic and human-authorized decisions carry the policy digest, active
workflow IDs and locked versions, matched categories, effective-value source
(`global` or workflow override), action preview, result, and resulting evidence.
Bounded operations report this information without creating persistence;
persisting workflows record it in the applicable immutable artifact. Secrets
and sensitive values are redacted from previews and decision records.

## 7. Numeric Decision Scale

The numeric scale applies independently to each decision category:

```text
4 > 3 > 2 > 1 > 0
```

This is the mandatory restrictiveness order for decision levels: `4` retains
the most granular human control and `0` grants the most automation to eligible
effects. A lower number must never require more human authority than a higher
number for the same category and effect. Different categories still evaluate
independently to decision outcomes, which are combined as
`stop > human > auto`; numeric levels from different categories are not merged
into one synthetic level.

| Level | Name | General orientation |
| ---: | --- | --- |
| `4` | `strict` | Granular human control over nearly every governed effect in the category |
| `3` | `governed` | Human approval for one complete, coherent action set; suitable as a controlled shared-workspace posture |
| `2` | `balanced` | Automatic low-impact work; human approval at material boundaries |
| `1` | `critical_only` | Automatic normal work; human approval only for critical effects in the category |
| `0` | `autonomous` | Automatic execution of every eligible effect in the category |

This table is orientation only. The category-specific definitions below are
normative. An agent must use the applicable category table rather than derive
behavior from the general labels. The names are documentation vocabulary; the
JSON contract stores the integer values only.

### 7.1 `workspaceMutation`

This category covers creating, editing, moving, renaming, or deleting files and
directories in the consumer workspace. Declared process artifacts and exact
report capture use their dedicated persistence contracts. Mutation of
`policy.json` is excluded from every numeric level and follows the absolute
human-only mutation rule in Section 4.

| Level | Required behavior |
| ---: | --- |
| `4` | Obtain a human decision for each exact file or directory target and operation (`create`, `edit`, `move`, or `delete`) before writing it; multiple hunks in one approved edit may remain one decision. |
| `3` | Present and obtain approval for one complete coherent patch set before applying any part of it. |
| `2` | Apply small, reversible edits and creations inside the approved scope automatically; request approval for deletion, movement, broad rewrites, public-contract changes, or generated-output replacement. |
| `1` | Apply reversible in-scope workspace changes automatically; request approval only for destructive, irreversible, or workspace-boundary effects. |
| `0` | Apply every eligible workspace mutation automatically and record the resulting diff. |

### 7.2 `scopeExpansion`

This category covers work that was not included in the current explicit
objective, domain, or approved action set. It never permits changing the user's
objective into an unrelated one.

| Level | Required behavior |
| ---: | --- |
| `4` | Obtain a separate human decision for every newly discovered item before adding it to scope. |
| `3` | Present one coherent expansion set with rationale and obtain approval before acting on it. |
| `2` | Include necessary adjacent work in the same domain automatically; request approval before crossing a module, product behavior, data, security, deployment, or release boundary. |
| `1` | Include necessary work anywhere in the repository automatically; request approval only when it changes the objective, external contract, security posture, protected data behavior, or release outcome. |
| `0` | Expand scope automatically only as needed to achieve the current objective, and record every expansion and rationale. |

### 7.3 `dependencyManagement`

This category covers project dependencies, host-installed tools, plugins,
features, versions, lockfiles, runtimes, and toolchains.

| Level | Required behavior |
| ---: | --- |
| `4` | Obtain a separate human decision for every package, tool, version, feature, install, update, or removal. |
| `3` | Present the complete dependency plan and obtain one approval before resolution, download, installation, or file mutation. |
| `2` | Inspect and audit existing dependency state automatically; request approval for every manifest, feature, version, lockfile, installed-tool, runtime, or toolchain mutation. |
| `1` | Resolve and inspect existing declarations and refresh a lockfile consistent with unchanged manifests automatically; request approval for additions, removals, upgrades, downgrades, host installations, runtime changes, or toolchain changes. |
| `0` | Perform every eligible dependency and toolchain operation automatically and record exact versions and changed files. |

### 7.4 `commandExecution`

This category covers starting local programs and scripts outside the internal
non-executing reasoning of the agent. Explicit validator selections remain
commands and are classified here unless the current human request already
authorizes that exact run. Selecting a compatible workflow preauthorizes only
the fixed pre-policy allowlist in Section 4.2: bootstrap projection, bounded
metadata-only Tool resolution, exact policy-boundary retrieval,
policy validation, and, after a valid result, policy evaluation. Invoking a
workflow node additionally preauthorizes only the
verification commands required for explicitly supplied inputs and the
deterministic store command for that node's declared artifact once every
applicable persistence condition is satisfied. No workflow preauthorizes
the human-operated policy-initialization operation. A decision that
permits exact report persistence likewise authorizes the deterministic report
store command. These narrow exceptions never extend to initialization,
validation execution, repair, installation, or another command.

| Level | Required behavior |
| ---: | --- |
| `4` | Obtain a human decision before every command invocation, with program, arguments, directory, and expected effects. |
| `3` | Present one ordered command set and obtain approval before executing any command in it. |
| `2` | Run declared read-only inspection and configured validation commands automatically; request approval for undeclared, mutating, networked, privileged, or externally sourced commands. |
| `1` | Run trusted local tools and workspace-owned scripts automatically; request approval only for downloaded or untrusted scripts, elevated execution, destructive commands, or execution outside the workspace boundary. |
| `0` | Run every eligible command automatically and record the exact program, arguments, directory, status, and bounded output evidence. |

### 7.5 `networkAccess`

This category covers outbound or inbound communication, remote reads,
downloads, uploads, and authenticated service access. A remote write also
matches `externalStateMutation`.

| Level | Required behavior |
| ---: | --- |
| `4` | Obtain a human decision for every request or connection, identifying endpoint, purpose, data direction, and authentication. |
| `3` | Obtain one approval for a bounded session with one service, purpose, endpoint set, and data class. |
| `2` | Perform read-only requests to already declared trusted endpoints automatically; request approval for new endpoints, downloads, uploads, authentication, or writes. |
| `1` | Perform reads and ordinary downloads from declared trusted sources automatically; request approval for uploads, authenticated writes, executable retrieval, or access to an undeclared service. |
| `0` | Perform every eligible network operation automatically and record endpoint, purpose, transferred data class, and outcome without persisting credentials. |

### 7.6 `externalStateMutation`

This category covers persistent effects outside the workspace, including API
writes, issue changes, messages, cloud resources, publication, release, and
deployment.

| Level | Required behavior |
| ---: | --- |
| `4` | Obtain a human decision for every individual external mutation and exact target. |
| `3` | Present one bounded mutation set for one service and obtain approval before applying it. |
| `2` | Mutate explicitly disposable sandbox or test state automatically; request approval for every persistent external write. |
| `1` | Apply reversible non-production external changes automatically; request approval for publication, deployment, production, communication to third parties, destructive changes, or irreversible effects. |
| `0` | Apply every eligible external mutation automatically and record service, target, parameters, outcome, and rollback evidence when available. |

### 7.7 `versionControlMutation`

This category covers VCS index, branches, commits, tags, remotes, and history.
Read-only inspection does not match this category.

| Level | Required behavior |
| ---: | --- |
| `4` | Obtain a human decision before every mutating VCS command. |
| `3` | Present one complete index, branch, commit, tag, or remote-operation set and obtain approval before mutation. |
| `2` | Stage or unstage explicitly scoped paths and create disposable local branches automatically; request approval for commits, tags, branch deletion, remote writes, or history rewriting. |
| `1` | Perform reversible local index, branch, and commit operations automatically; request approval for remote writes, tags, history rewriting, destructive cleanup, or force operations. |
| `0` | Perform every eligible VCS mutation automatically and record the before and after references. |

### 7.8 `dataMutation`

This category covers database contents and schemas, structured datasets,
snapshots, generated data, and other persistent non-source state. A file change
may also match `workspaceMutation`.

| Level | Required behavior |
| ---: | --- |
| `4` | Obtain a human decision for every target and mutation operation. |
| `3` | Present one complete transaction or data-change set and obtain approval before applying it. |
| `2` | Mutate disposable test, fixture, or temporary data automatically; request approval for persistent, schema, bulk, generated-output, destructive, remote, or production data changes. |
| `1` | Apply reversible local persistent-data changes automatically; request approval for destructive, irreversible, remote, production, schema, or migration effects. |
| `0` | Perform every eligible data mutation automatically and record target, transaction boundary, verification, and rollback evidence when available. |

### 7.9 `sensitiveDataAccess`

This category covers reading or transmitting confidential source material,
personal data, production data, secrets, credentials, tokens, private keys,
and similarly protected resources. Ordinary repository reads do not match this
category unless the resource is classified as sensitive.

| Level | Required behavior |
| ---: | --- |
| `4` | Obtain a human decision for every sensitive resource access and exact purpose. |
| `3` | Obtain one approval for a bounded session covering one data class, source, purpose, and recipient set. |
| `2` | Access internal non-secret material needed by the approved scope automatically; request approval for personal, confidential, production, credential, secret, or private-key access. |
| `1` | Access explicitly scoped confidential and personal data automatically; request approval for credentials, tokens, private keys, unredacted production data, or transfer to another trust boundary. |
| `0` | Access every explicitly eligible sensitive resource automatically while preserving minimization, redaction, and non-disclosure requirements; any persistent write is evaluated separately under `sensitiveDataPersistence`. |

### 7.10 `validationReportPersistence`

This category covers creating, retaining, replacing, exporting, or removing a
canonical validation report. It applies regardless of report outcome or data
sensitivity. Persistence outside the canonical local report store also matches
every applicable workspace, network, external-state, and data-mutation
category.

| Level | Required behavior |
| ---: | --- |
| `4` | Obtain a human decision before each report persistence or removal action, identifying the selection, exact destination, known data classes, and canonical lifecycle. |
| `3` | Present one bounded report-persistence action set for a validation session, including selections, destination, lifecycle effects, and cleanup if requested, and obtain approval before applying any part of it. |
| `2` | Persist reports automatically in the canonical local private report store; request approval for removal, another destination, export, or sharing. |
| `1` | Persist reports automatically in the canonical local private report store and perform eligible cleanup of unreferenced reports when that cleanup is explicitly in scope; request approval for a new trust boundary, public exposure, or removal that has broader lifecycle effects. |
| `0` | Perform every eligible report-persistence action within the canonical local report contract automatically and record the exact path, digest, and lifecycle outcome. |

This category does not inspect report meaning to decide whether a `Pass`,
`Fail`, `Blocked`, or other status deserves persistence. Status never grants
authority. When report content is sensitive, `sensitiveDataPersistence` also
applies.

### 7.11 `sensitiveDataPersistence`

This category covers writing confidential source material, personal data,
production data, secrets, credentials, tokens, private keys, or similarly
protected values to any persistent medium. It applies to reports, AI artifacts,
logs, snapshots, local files, databases, and external
stores. Reading the same material is evaluated separately under
`sensitiveDataAccess`.

| Level | Required behavior |
| ---: | --- |
| `4` | Obtain a human decision for every persistence action, identifying the exact data class, values or bounded fields, canonical destination, protection, purpose, and lifecycle. |
| `3` | Present one bounded persistence plan for one data class, purpose, destination, protection, and lifecycle, and obtain approval before writing any part of it. |
| `2` | Persist internal non-secret sensitive material automatically only in the canonical local private store; request approval for personal, confidential, production, credential, secret, private-key, remote, or cross-boundary persistence. |
| `1` | Persist explicitly scoped confidential, personal, and production data automatically in the canonical local private store; request approval for credentials, tokens, private keys, other authentication secrets, or a new trust boundary. |
| `0` | Persist every explicitly eligible sensitive data class automatically only within a contract-valid declared destination while preserving minimization, access control, non-disclosure, lifecycle rules, and auditable linkage. |

This category governs authority, not storage security. An `auto` outcome never
makes an unsuitable destination safe, bypasses path containment or access
controls, or authorizes disclosure. Exact bytes may be persisted only when all
applicable categories permit the action; otherwise the bytes remain
non-persistent and the operation records only a bounded redacted decision
result.

### 7.12 Required Classification Examples

The normative public document includes at least these overlap examples:

- installing a new validator matches `dependencyManagement`,
  `commandExecution`, `networkAccess`, and any resulting
  `workspaceMutation`;
- listing or reading already installed resources through Tool is read-only
  and offline, but still matches `sensitiveDataAccess` when the selected payload
  is classified as sensitive;
- installing or updating a Hub resource through Manager matches
  `commandExecution`, `networkAccess`, and `workspaceMutation`, plus
  `versionControlMutation` only when the resulting tracked files are committed;
- removing a local resource through Manager matches `commandExecution` and
  `workspaceMutation`; it does not imply network authority;
- changing `config.json` and running its first validation matches
  `workspaceMutation` for the edit and a separately evaluated
  `commandExecution`, plus dependency or network categories when applicable;
- regenerating an approved snapshot matches `commandExecution`,
  `workspaceMutation`, and `dataMutation`;
- committing and pushing a repair matches `commandExecution`,
  `versionControlMutation`, `networkAccess`, and `externalStateMutation`;
- calling an authenticated API with protected data matches `networkAccess`,
  `externalStateMutation`, and `sensitiveDataAccess`;
- storing a non-sensitive canonical validation report in the configured local
  report store matches `validationReportPersistence`;
- storing a canonical report containing protected values matches
  `validationReportPersistence` and `sensitiveDataPersistence`, in addition to
  any categories required by its destination;
- persisting a sensitive AI artifact matches `sensitiveDataPersistence`, while
  reading its protected source also matches `sensitiveDataAccess`;
- fixing an adjacent failure discovered during a workflow incident-analysis
  node additionally matches `scopeExpansion` when it was outside the approved
  incident scope.

Each example demonstrates that one permissive category never cancels a more
restrictive applicable category.

## 8. Absolute Boundaries

The decision policy adjusts human involvement. It does not:

- override system, developer, workspace, repository, legal, or security rules;
- grant filesystem, network, credential, sandbox, or operating-system
  permissions;
- make an action relevant to the current objective merely because its level is
  `0`;
- bypass operation or process preconditions, artifact validation, report
  validation, or the repository mutation checks configured for a validation run;
- permit the agent to expose sensitive values or persist them without the
  applicable decision outcomes, destination protections, and canonical
  lifecycle rules;
- convert uncertainty about a human-owned product decision into automatic
  authority;
- authorize an agent to create, edit, format, replace, delete, move, rename,
  restore, initialize, or otherwise mutate `policy.json` itself.

When required facts are missing, the agent requests clarification. When an
action is prohibited by a superior rule, it stops. These outcomes are not
downgraded by numeric levels.

## 9. Documentation And AI Resource Requirements

The canonical public document must contain:

- the exact policy shape and defaults;
- universal persistence boundaries, the fixed report destination and
  treatment, and the requirements imposed on workflow artifact declarations;
- the distinction between validation and AI decision policy;
- continuation mode, decision level, and decision outcome as separate concepts;
- the three continuation modes;
- all eleven categories and all five levels for each category;
- the level restrictiveness order `4 > 3 > 2 > 1 > 0` and the outcome
  restrictiveness order `stop > human > auto`;
- action classification and restrictive combination rules;
- explicit-request, free-text approval, audit, redaction, human-only policy
  ownership, and manual policy-change behavior;
- examples of one action matching multiple categories;
- examples from both official workflows and one external workflow with a
  different graph and agent profiles;
- invalid-policy and missing-policy behavior;
- the absolute boundaries above.

The official policy-boundary guide is an exact concise projection of the
human-only policy-mutation rule and the read-only policy-guidance exception. Every
official workflow or skill that performs governed work declares that shared
resource, and the invoking agent loads it before work begins. Third-party resources claiming
Liknon policy compatibility follow the same dependency contract. No resource
restates or redefines level semantics independently.

## 10. Verification

Schema, decision-table, and forward-agent fixtures cover:

- missing policy blocking every non-policy AI operation before work, exact
  human-facing initialization and review guidance, and direct human canonical
  level-4 initialization;
- explicit workspace-root selection, nearest-ancestor `.liknon/` discovery for
  existing state, exact-current-directory initialization when no root
  exists, and absence of Git or configuration-based root discovery;
- read-only policy validation, deterministic explicit-stack policy evaluation,
  and idempotent non-overwriting human-operated policy initialization;
- the exact pre-policy allowlist succeeding without a prior
  `commandExecution` result and rejecting generic search, undeclared payload
  retrieval, Manager/network access, validation execution, mutation,
  persistence, and workflow-node work;
- malformed, unsupported, unknown-field, and path-escaping policy;
- all `processContinuation` modes for unrelated continuation-eligible edges;
- global fallback, one exact workflow override, unmatched workflow IDs, and
  most-restrictive resolution across composed parent and child workflows;
- stable workflow identity across version updates and exact locked-version
  recording for each execution;
- prevention of continuation after failed or incomplete prerequisites;
- every level of every category at its boundary examples;
- monotonic category behavior under `4 > 3 > 2 > 1 > 0`;
- actions matching two or more categories, with the most restrictive result
  winning;
- exact initiating authority, partial approval, rejection, postponement, and
  manually completed actions;
- refusal by official and external compatible workflows to create, edit,
  format, replace, delete, move,
  rename, restore, initialize, or indirectly mutate policy, including after an
  explicit user request and under level `0`;
- advisory policy guidance with no candidate file, patch, mutation command,
  process artifact, or hidden persistence;
- policy change during a resumable process and authorization re-evaluation;
- redacted action previews and decision records;
- non-sensitive, sensitive, approved, declined, and stopped canonical-report
  persistence;
- canonical local destinations, attempts to use another destination, explicit
  governed removal, active-process protection, and absence of automatic pruning
  or compaction;
- sensitive persistence in reports, AI artifacts, and external destinations;
- a level-0 policy that still obeys absolute boundaries;
- a level-4 policy that does not request duplicate approval for an exact action
  already authorized by the current human instruction;
- Tool reads, missing-resource failures, and proposed Manager install, update,
  and removal operations with no implicit escalation from Tool to Manager;
- unknown capabilities, unknown effects, and material undeclared effects that
  fail closed before execution.

Forward tests give agents only the policy when present, the current operation or
process inputs, and raw workspace evidence. They do not disclose the expected
classification. Missing-policy trials exercise the advisory policy operation and
attempt to persuade the agent to perform a prohibited mutation.

## 11. Acceptance Criteria

- [ ] `processContinuation` is ternary and never interpreted as a numeric
      decision level.
- [ ] `processContinuation` applies only to continuation-eligible transitions
      declared by the exact active workflow graph.
- [ ] Every decision category and `processContinuation` has one required global
      value and an optional exact-workflow override map.
- [ ] An exact workflow override takes precedence over the global value; nested
      matching overrides combine by the most restrictive value.
- [ ] Validation reports only document status and identity; evaluation accepts
      only a caller-supplied ordered workflow stack and reports every effective
      value and source without workflow discovery or activation.
- [ ] Workflow manifests cannot create, modify, or lower policy values.
- [ ] Decision levels follow `4 > 3 > 2 > 1 > 0` within every category.
- [ ] Every category-level combination has one documented normative meaning.
- [ ] The same action and policy produce the same required decision regardless
      of process or chat context.
- [ ] Overlapping categories use `stop > human > auto`.
- [ ] Missing policy blocks every governed operation except a contract-valid
      read-only policy-guidance operation until the human directly initializes
      and reviews the decision settings.
- [ ] Every governed policy evaluation occurs under a previously loaded and
      recorded compatible AI bootstrap contract; this policy is never treated
      as an alternate entry point.
- [ ] Invalid policy fails closed without silent replacement.
- [ ] No AI operation mutates policy directly or indirectly, even after explicit
      approval or under level `0`.
- [ ] Policy guidance remains read-only and requires the human to apply every
      effective value manually.
- [ ] Every bounded decision reports, and every persisted workflow decision
      records, the exact policy digest, active workflow identity and version,
      effective global or override source, and applicable categories without
      recording secrets.
- [ ] Numeric levels select authority behavior without inventing storage fields
      absent from `policy.json`; universal boundaries come from the bootstrap
      and policy contracts, while concrete artifact facts come from the exact
      locked workflow declaration.
- [ ] The validation executor and planner remain independent from AI decision
      policy; explicit artifact verification never affects execution planning or
      report semantics.
- [ ] The generated `policy-boundary.md` matches the deterministic projection
      of the canonical document and carries no independently authored policy
      semantics.
