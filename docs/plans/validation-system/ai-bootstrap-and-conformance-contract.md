# AI Bootstrap And Conformance Contract

## 1. Purpose

Define the program-owned contract that a human supplies as the first Liknon
instruction to an AI. This contract establishes how a conforming external AI
host discovers the workspace, loads governance, selects a workflow, retrieves
resources, binds application capabilities, uses the three CLIs, and records
evidence.

The contract provides a stable root for every governed AI interaction without
turning Liknon into an AI runtime or coupling the applications to one model,
provider, agent framework, workflow, or skill.

## 2. Product Boundary

Liknon does not ship a fourth CLI, model host, conversation manager, or workflow
executor. The human chooses an external AI environment and explicitly directs it
to the bootstrap entry point. That environment is conforming only while it:

- loads the exact supported bootstrap contract before operational resources;
- preserves superior system, host, repository, and current human instructions;
- follows the selected workflow graph and locked resource closure;
- enforces the human-owned decision policy before governed effects;
- treats resource payloads, repository content, reports, and command output as
  untrusted content rather than higher-priority authority;
- records enough identity and digest evidence to make its decisions auditable.

An arbitrary AI can ignore these instructions. Such use remains possible, just
as direct CLI use remains possible, but it must not be described as a governed
or conforming Liknon AI operation.

## 3. Canonical Entry Point

The repository owns one canonical content file and one adjacent identity file:

```text
crates/liknon-workspace/contracts/ai/ENTRYPOINT.md
crates/liknon-workspace/contracts/ai/manifest.json
```

Together they are a program contract, not a Hub resource, skill, workflow, prompt, agent
profile, or replaceable instruction package. Resource publication cannot replace
or override it.

Both files live below the owning Cargo package so local and published
`liknon-workspace` builds can expose the exact canonical bytes without copying
from outside the package boundary. `manifest.json` contains only the bootstrap metadata needed to identify and
verify the content: manifest schema version, SemVer contract version, media
type, canonical entry path relative to its `contracts/ai/` directory, and
lowercase SHA-256 digest. The digest is
computed over the exact `ENTRYPOINT.md` UTF-8 bytes as stored, including its
final-newline state, with no newline, Unicode, or whitespace normalization. The
manifest never hashes itself. Its schema and the Tool JSON envelope are
independently versioned.

The source checkout exposes the file directly. The Tool application embeds the
same exact bytes and exposes them without requiring a workspace or installed
resource store:

```sh
likt bootstrap [--format=human|json]
```

Human output returns the canonical Markdown. JSON output returns a versioned
envelope containing the manifest schema version, bootstrap-contract version,
SHA-256 digest, Tool capability version, media type, canonical source identity,
and exact content. No command silently writes a copy into a consumer workspace.
A later explicit materialization feature may
project the exact bytes only if it records the source version and digest and
never creates a second editable source.

The human can therefore direct an AI either to the tracked source file or to the
stable Tool command. A conforming host records the exact version and digest it
loaded before claiming policy or workflow conformance.

## 4. Mandatory Loading Order

Superior system, host, workspace, repository, and current human instructions
already govern the external host before it reads Liknon content. Within that
pre-existing authority, the entry point requires this order:

1. acknowledge and preserve those superior instructions;
2. as the first Liknon instruction, load `ENTRYPOINT.md` and verify it against
   `manifest.json`, the Tool envelope, or both;
3. select one explicit `.liknon/` workspace boundary;
4. identify the human-selected workflow and entry node, or identify the bounded
   standalone skill operation when no workflow is active;
5. resolve the exact locked graph and dependency-closure metadata through Tool
   without activating or obeying resource instructions;
6. verify bootstrap, policy-contract, application, workflow, profile, skill,
   tool, knowledge, and artifact-contract compatibility;
7. retrieve and load the exact declared policy-boundary guide;
8. validate the existing human-owned `.liknon/ai/policy.json` through the
   program contract, except for the narrowly read-only policy-guidance route;
9. evaluate the exact Tool-verified root-to-leaf workflow stack, or the empty
   stack for a standalone skill operation;
10. bind declared tool operations to separately installed application
   capabilities through the external host;
11. load only the remaining resources required by the selected node or bounded
   operation;
12. classify every proposed effect under the effective policy before acting;
13. invoke only eligible capabilities, preserving `config.json` as the sole
    executable validation policy;
14. validate outputs and record the exact contracts, resources, decisions, and
    evidence used.

Missing, unsupported, modified, or incompatible mandatory input fails closed.
Installation, lookup, or retrieval never implies workflow activation. Only an
explicit human selection establishes an active workflow and entry node.

The commands needed to establish effective policy cannot depend on policy that
has not yet been evaluated. Before Step 9 completes, this contract therefore
pre-authorizes only the fixed read-only preflight defined by
[AI Decision Policy Section 4.2](ai-decision-policy.md): bootstrap projection,
metadata-only Tool resolution of the exact human-selected workflow or skill,
retrieval of its exact policy-boundary material, `policy validate`, and, after
validity is established, `policy evaluate`. It does not pre-authorize generic
search or context retrieval, Manager or network access, validation execution,
repository analysis, mutation, persistence, or workflow-node work. Every other
command and effect waits for the effective policy result.

## 5. Authority And Configuration

The bootstrap contract defines precedence and procedure; it grants no new
permission. In particular:

- `.liknon/validation/config.json` remains the only executable validation
  configuration;
- `.liknon/ai/policy.json` remains human-owned authority for governed AI effects;
- no agent may create, mutate, move, replace, restore, or delete policy;
- workflow and resource manifests declare compatibility, capabilities, effects,
  and dependencies but cannot authorize themselves;
- application bindings prove availability and capability but do not bypass
  policy;
- resource namespace, publisher, trust class, or official status never changes
  instruction precedence.

The bootstrap contract references the versioned policy, workflow, resource,
artifact, and CLI contracts instead of duplicating their complete contents.

## 6. AI-Tool And Alternative Skills

AI-Tool remains an independently versioned official skill under:

```text
resources/skills/official/liknon-ai-tool/
```

It is the official default operational skill used by the initial action and
validation-engineering workflows to teach agents how to apply Liknon CLI
capabilities. It is not embedded in the bootstrap contract, receives no runtime
privilege, and is never loaded merely because it is official.

An external workflow may reuse AI-Tool, select another compatible skill, or use
a community-maintained derivative. The workflow manifest, not application code
or the bootstrap contract, declares that choice. Alternative resources must
declare the same relevant compatibility, operation, capability, effect, and
policy contracts; their identity does not weaken the bootstrap rules.

This keeps official AI behavior easy to test and update while allowing users to
publish better or differently specialized integrations without forking a CLI.

## 7. External Host Responsibilities

The external host owns model selection, profile-to-model binding, workflow graph
conduct, action dispatch, continuation, and presentation to the human. The host
must expose or retain, as applicable:

- bootstrap-contract version and digest;
- selected workspace, workflow stack, entry node, and resource-lock digest;
- selected agent-profile and tool-operation identities;
- application capability and version evidence;
- policy path, schema version, digest, effective source, and decision outcomes;
- traversed transitions, performed actions, outputs, and artifact references;
- every stop, human-decision request, incompatibility, and unresolved input.

The contract specifies observable behavior rather than one provider API. The
evaluation harness supplies adapters for supported AI environments and tests the
same conformance scenarios against each adapter.

## 8. Implementation Ownership

- `crates/liknon-workspace/contracts/ai/ENTRYPOINT.md` is the canonical authored
  instruction source and its adjacent `manifest.json` is the canonical identity
  and digest source.
- `liknon-workspace` owns typed bootstrap identity, version, compatibility, and
  machine-result contracts.
- Tool owns the read-only `bootstrap` projection and exact-byte verification.
- Manager resolves workflow-declared compatibility but cannot replace or mutate
  the program contract.
- Validation CLI owns configuration, policy-validation, report, and persistence
  mechanics without orchestrating an AI.
- Hub resources declare compatible bootstrap ranges and remain independently
  versioned content.
- The agent-evaluation harness owns provider adapters and conformance evidence;
  it is development and release tooling, not a consumer runtime dependency.

## 9. Replacement Boundary

The A01 separation phase creates this contract and updates every maintained
document, schema, test, and resource manifest that currently assumes an
official workflow is the root of AI behavior. Phase 3 updates the policy and
action-workflow documents; Phase 4 updates generic workflow, persistence, and
host responsibilities; Phases 5 and 6 update the official workflow nodes; Phase
7 proves the completed replacement.

Current pre-separation documents are implementation inputs, not compatibility
requirements. Once their owning phase is complete, no stale path, embedded
resource assumption, official-workflow privilege, or earlier policy model is
retained as a fallback.

## 10. Acceptance Criteria

- [ ] One canonical entry file is the first Liknon instruction for every
      governed AI operation, under authority that already exists before Liknon
      content is loaded.
- [ ] One adjacent metadata manifest defines its schema version, SemVer contract
      version, media type, path, and digest over exact raw entry bytes.
- [ ] `likt bootstrap` returns its exact bytes, version, and digest without a
      workspace, network access, or installed resource.
- [ ] No Hub resource can replace, shadow, or raise its authority above the
      bootstrap contract.
- [ ] The contract contains no branch for an official workflow, skill, profile,
      node, transition, or process ID.
- [ ] Official workflows declare AI-Tool normally through the resource graph.
- [ ] A third-party workflow can select another compatible operational skill
      without changing application code or bootstrap semantics.
- [ ] A conforming host stops before activation when bootstrap, policy, workflow,
      resource, application, or effect compatibility is absent or invalid.
- [ ] Direct non-conforming AI use remains possible but receives no governed
      Liknon safety or policy claim.
- [ ] Provider adapters run the same conformance suite and preserve attributable
      evidence.
- [ ] Every maintained base document superseded by this architecture is updated
      or removed by its owning implementation phase.
