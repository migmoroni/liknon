# Cross-Cutting Contract: AI Decision Policy

## 1. Status And Purpose

This document specifies the consumer-facing AI decision policy that Phase 3
implements and every persistent AI process obeys. It is a cross-cutting
contract, not an implementation phase or a validator execution contract.

The canonical public document is authored at:

```text
docs/validation/ai-decision-policy.md
```

The distributed skill receives the exact generated projection of that document
and treats it as normative. `SKILL.md` and every persistent AI-process
reference must require the agent to load this contract before classifying or
performing an effect governed by `.validation/policy.json`.

The document must define every category and every level explicitly. An agent
must not infer, rename, merge, or locally reinterpret their meanings.

### 1.1 Design Foundations

The public document records the following design basis and labels the resulting
workspace-validator contract as a local synthesis:

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

The source register contains exact editions, stable source IDs, primary URIs,
scope, and review dates. The five numeric levels and category tables are this
project's normative synthesis; the document must not misrepresent them as a
standard published verbatim by any one source.

## 2. Separate Policies

The consumer workspace has two policies with different owners and effects:

| File | Responsibility | Consumer |
| --- | --- | --- |
| `.validation/config.json` | Defines the programs, arguments, composition, and repository-integrity behavior used for validation | `workspace-validator` CLI and library |
| `.validation/policy.json` | Defines when an AI agent acts automatically, requests a free-text human decision, or ends a related process | Persistent AI-process skill workflows only |

`policy.json` never contributes commands, checks, suites, groups, arguments, or
validation outcomes. The CLI and library do not discover, read, or interpret
it. `config.json` remains the sole executable validation policy.

## 3. Canonical Policy Shape

The initial contract is:

```json
{
  "schemaVersion": 1,
  "processContinuation": "human",
  "decisionLevels": {
    "workspaceMutation": 4,
    "scopeExpansion": 4,
    "dependencyManagement": 4,
    "commandExecution": 4,
    "networkAccess": 4,
    "externalStateMutation": 4,
    "versionControlMutation": 4,
    "dataMutation": 4,
    "sensitiveDataAccess": 4
  }
}
```

The tool owns and distributes the JSON Schema. Consumer workspaces store the
policy document, not a schema copy. Every listed category is required. Missing
categories, unknown fields, and unsupported values are rejected rather than
resolved through an implicit fallback.

## 4. Missing, Invalid, And Changed Policy

On the first AI-assisted process invocation, when `policy.json` does not exist,
the agent atomically creates the canonical document above. This narrowly
defined bootstrap write requires no separate approval because it establishes
the most human-controlled decision levels and `human` continuation.

An existing malformed, unsupported, path-escaping, or internally inconsistent
policy stops the AI process before any governed effect. The agent reports the
problem and does not silently replace the document or apply defaults.

The policy cannot authorize its own modification. Any change to
`.validation/policy.json`, including lowering or raising a level, requires an
explicit human request or a free-text human approval bound to the exact proposed
diff. The initial safe creation is the only automatic policy mutation.

Each persistent process records the exact policy path, schema version, and
SHA-256 digest used for its decisions. A policy change does not invalidate
domain or sensorium evidence, because it does not change validation coverage.
It does invalidate any cached authorization decision. A resumed process reloads
and re-evaluates the current policy before its next governed effect.

## 5. Process Continuation

`processContinuation` is a three-state policy, not part of the numeric decision
scale:

| Value | Required behavior |
| --- | --- |
| `auto` | After the preceding process completes successfully, persists its result, and satisfies every precondition of the next process, start `0 -> 1` or `2 -> 3` without another question. |
| `human` | Present the completed result, suggest the valid next process, and wait for a free-text human decision. |
| `stop` | Present the completed result and end the workflow without suggesting or asking to start the related next process. |

The policy applies only to the defined `0 -> 1` and `2 -> 3` relationships. A
failed, blocked, interrupted, stale, or incomplete process never continues
automatically. `auto` does not bypass sensorium lifecycle, report, digest,
configuration, or process-specific preconditions. `stop` does not prevent the
human from invoking the next process explicitly later.

## 6. Decision Evaluation Model

Before a governed effect, the agent constructs a bounded action preview that
identifies:

- current process and objective;
- operation and exact target resources;
- matching decision categories;
- whether the action is explicitly in scope;
- trust origin of commands, code, data, and endpoints;
- local, network, remote, or production environment;
- read-only, reversible, destructive, or irreversible effect;
- affected data sensitivity and required privilege;
- expected blast radius and rollback path;
- rationale and material uncertainty.

This follows an attribute-based model: decisions depend on the actor, action,
resource, and current environment rather than on the action name alone.

An **eligible effect** is relevant to the current objective, permitted by every
superior instruction and repository rule, supported by the available evidence,
within the agent's actual permissions, and valid under all process
preconditions. Level `0` automates only eligible effects; it does not make an
otherwise prohibited or unrelated effect eligible.

Every applicable category evaluates independently to `auto` or `human`. A hard
precondition, superior instruction, unsupported policy, or prohibited action
evaluates to `stop`. Combine overlapping results in this order:

```text
stop > human > auto
```

One human prompt may cover one coherent action set, but it must show every
category that required human authority. Approval is bound to the presented
targets and effects; it does not authorize later scope expansion. Prompts accept
free text so the human can approve, reject, constrain, postpone, or report a
manually completed action.

A precise initiating request can provide the required human authority for the
exact action and scope it names. Invoking an AI process alone authorizes only
that process's declared persistence outputs; it does not preauthorize config,
source, dependency, command, network, external-state, VCS, data, or sensitive
resource effects.

Automatic and human-authorized decisions are recorded with policy digest,
matched categories, action preview, result, and resulting evidence. Secrets and
sensitive values are redacted from previews and decision records.

## 7. Numeric Decision Scale

The numeric scale applies independently to each decision category:

| Level | Name | General orientation |
| ---: | --- | --- |
| `4` | `strict` | Granular human control over nearly every governed effect in the category |
| `3` | `governed` | Human approval for one complete, coherent action set; suitable as a controlled shared-workspace posture |
| `2` | `balanced` | Automatic low-impact work; human approval at material boundaries |
| `1` | `critical_only` | Automatic normal work; human approval only for critical effects in the category |
| `0` | `autonomous` | Automatic execution of every eligible effect in the category |

This table is orientation only. The category-specific definitions below are
normative. A process must use the applicable category table rather than derive
behavior from the general labels. The names are documentation vocabulary; the
JSON contract stores the integer values only.

### 7.1 `workspaceMutation`

This category covers creating, editing, moving, renaming, or deleting files and
directories in the consumer workspace. Declared process artifacts and exact
report capture use their separate automatic-output contract. Mutation of
`policy.json` follows the fixed rule in Section 4.

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
authorizes that exact run.

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
| `0` | Access every explicitly eligible sensitive resource automatically while preserving minimization, redaction, non-disclosure, and non-persistence requirements. |

### 7.10 Required Classification Examples

The normative public document includes at least these overlap examples:

- installing a new validator matches `dependencyManagement`,
  `commandExecution`, `networkAccess`, and any resulting
  `workspaceMutation`;
- changing `config.json` and running its first validation matches
  `workspaceMutation` for the edit and a separately evaluated
  `commandExecution`, plus dependency or network categories when applicable;
- regenerating an approved snapshot matches `commandExecution`,
  `workspaceMutation`, and `dataMutation`;
- committing and pushing a repair matches `commandExecution`,
  `versionControlMutation`, `networkAccess`, and `externalStateMutation`;
- calling an authenticated API with protected data matches `networkAccess`,
  `externalStateMutation`, and `sensitiveDataAccess`;
- fixing an adjacent failure discovered during Process 2 additionally matches
  `scopeExpansion` when it was outside the approved incident scope.

Each example demonstrates that one permissive category never cancels a more
restrictive applicable category.

## 8. Absolute Boundaries

The decision policy adjusts human involvement. It does not:

- override system, developer, workspace, repository, legal, or security rules;
- grant filesystem, network, credential, sandbox, or operating-system
  permissions;
- make an action relevant to the current objective merely because its level is
  `0`;
- bypass process preconditions, state validation, report validation, or
  repository-integrity checks;
- permit the agent to expose or persist secrets;
- convert uncertainty about a human-owned product decision into automatic
  authority;
- authorize the agent to modify `policy.json` itself.

When required facts are missing, the agent requests clarification. When an
action is prohibited by a superior rule, it stops. These outcomes are not
downgraded by numeric levels.

## 9. Documentation And Skill Requirements

The canonical public document must contain:

- the exact policy shape and defaults;
- the distinction between validation and AI decision policy;
- the three continuation states;
- all nine categories and all five levels for each category;
- action classification and restrictive combination rules;
- explicit-request, free-text approval, audit, redaction, and policy-change
  behavior;
- examples of one action matching multiple categories;
- examples for Processes 0 through 3;
- invalid-policy and missing-policy behavior;
- the absolute boundaries above.

The skill router loads a concise policy summary for every persistent process
and loads the full document whenever an effect must be classified. Process
references link to the canonical projected contract rather than restating
level semantics independently.

## 10. Verification

Schema, decision-table, and forward-agent fixtures cover:

- missing policy and canonical level-4 creation;
- malformed, unsupported, unknown-field, and path-escaping policy;
- all `processContinuation` states for both valid transitions;
- prevention of continuation after failed or incomplete prerequisites;
- every level of every category at its boundary examples;
- actions matching two or more categories, with the most restrictive result
  winning;
- exact initiating authority, partial approval, rejection, postponement, and
  manually completed actions;
- mandatory human authority for policy mutation;
- policy change during a resumable process and authorization re-evaluation;
- redacted action previews and decision records;
- a level-0 policy that still obeys absolute boundaries;
- a level-4 policy that does not request duplicate approval for an exact action
  already authorized by the current human instruction.

Forward tests give agents only the policy, current process inputs, and raw
workspace evidence. They do not disclose the expected classification.

## 11. Acceptance Criteria

- [ ] `processContinuation` is ternary and never interpreted as a numeric
      decision level.
- [ ] Every category-level combination has one documented normative meaning.
- [ ] The same action and policy produce the same required decision regardless
      of process or chat context.
- [ ] Overlapping categories use `stop > human > auto`.
- [ ] Missing policy creates only the canonical conservative policy.
- [ ] Invalid policy fails closed without silent replacement.
- [ ] Policy cannot authorize its own modification.
- [ ] Every persistent decision records the exact policy digest and applicable
      categories without recording secrets.
- [ ] The CLI and library remain independent from AI decision policy.
- [ ] The canonical document and its distributed skill projection are
      byte-identical.
