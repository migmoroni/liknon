# Phase B02.0: Define The Resource And Hub Contract

[Back to the Resource Hub and Manager plan](README.md)

## Purpose

Approve the public resource identity, metadata, compatibility, trust, workspace,
and command contracts before implementing network or mutation behavior.

## Resource Identity

Define one stable resource specification that distinguishes namespace, name,
kind, semantic resource version, and an optional exact version or accepted
constraint.

The identity is independent from a URL, local path, registry implementation,
CLI version, or display title. Case, normalization, reserved namespaces, and
collision handling are explicit and tested.

Resource kind IDs and physical directory names are distinct fixed contract
fields:

| Kind ID | Kind directory |
| --- | --- |
| `knowledge` | `knowledge` |
| `skill` | `skills` |
| `workflow` | `workflows` |
| `agent-profile` | `agent-profiles` |
| `tool` | `tools` |
| `prompt` | `prompts` |
| `guide` | `guides` |
| `template` | `templates` |
| `collection` | `collections` |

Schemas serialize only the singular kind ID. Shared path primitives derive the
corresponding directory; callers and resource payloads cannot supply an
alternative spelling.

Namespace identifies the publisher identity. Source origin, trust class,
review state, signatures, and catalog channel are separate fields. Neither the
`official` namespace nor any trust class changes instruction precedence,
activation, policy, or application behavior.

## Resource Manifest

The versioned resource manifest defines:

- identity, title, summary, authorship, license, and source;
- trust evidence and publication status;
- minimum compatible resource-schema, Manager, Tool, and Validation CLI
  capabilities when applicable;
- compatible program-owned AI bootstrap and decision-policy contract ranges
  when the resource participates in governed AI operation;
- required and optional resource dependencies;
- declared capabilities and executable-content indicators;
- every payload path, media type, length, and SHA-256 digest;
- collection membership without copied payloads.

Workflow manifests additionally act as generic orchestration indexes and define:

- stable node, action, entry-point, terminal-node, transition, and edge IDs;
- transition preconditions and explicit continuation eligibility;
- referenced skill, agent-profile, tool, knowledge, prompt, and optional
  subworkflow resource identities and constraints;
- action inputs, outputs, referenced operations, potential effects, and required
  capabilities per node;
- declared artifact outputs with stable declaration IDs, exact payload schemas,
  producing nodes and actions, allowed relationships, bounded namespaced storage
  templates, sensitivity and size limits, and `local-ignored` or
  `consumer-selected` repository treatment, without embedding executable
  policy.

An actionable workflow declares the supported Liknon AI decision-policy
and AI bootstrap contract versions. A workflow without either compatibility may
still be published, installed, and inspected as content, but it cannot be
represented as a governed Liknon workflow or activated by a conforming AI host.

The bootstrap contract is a program capability, not a resolvable resource. The
Hub cannot publish a replacement for
`crates/liknon-workspace/contracts/ai/ENTRYPOINT.md`, and Manager
cannot satisfy bootstrap compatibility by installing resource content.

Agent-profile manifests define roles, posture, scope, and behavioral
constraints independently from workflow control flow. They do not select
models, own transitions, change policy, or grant execution authority.

Tool-resource manifests describe stable application capabilities and operation
contracts. They do not embed an executable, install a prerequisite, contain a
credential, or authorize command execution. Workflows reference them as typed
dependencies, and the invoking host verifies the corresponding application
capability separately.

Operational skill compatibility is resolved normally. Neutral fixtures prove
that one workflow may select one skill while another selects a compatible
alternative without receiving a different application or policy path. Phase 3
later publishes AI-Tool and the official action workflow through this unchanged
contract.

Unknown required fields, unsupported schema versions, cycles, conflicting
constraints, unsafe paths, and undeclared payloads fail closed.

## Consumer State

Define versioned schemas for:

- `.liknon/resources/manifest.json`, expressing requested resources and
  accepted constraints;
- `.liknon/resources/lock.json`, expressing one exact complete resolution;
- installed payload layout and integrity state;
- explicit materialization records when files are projected elsewhere.

Manifest ordering and formatting are human-maintainable. Lock serialization is
deterministic. Neither file contains credentials.

## Hub Contract

Approve:

- catalog index and query response schemas;
- immutable resource-version metadata and payload locations;
- signed catalog and release metadata;
- namespace ownership and transfer rules;
- trust-class assignment and revocation;
- publication, deprecation, yanking, and security-advisory behavior;
- pagination, caching, availability, and explicit offline failure semantics.

A yanked or revoked resource remains identifiable in existing locks but is not
silently substituted. Security state is visible during search, update, and
verify.

## Command Contract

Specify the complete grammar, output schemas, exit statuses, confirmation
points, and non-interactive behavior for `init`, `search`, `info`,
`install`, `update`, `remove`, `list`, `verify`, and
`materialize`.

Every mutating command supports a complete dry-run or plan result before commit.
Human output may be themed; machine output contains no ANSI and exposes exact
typed outcomes.

## Prototype Matrix

Before approval, prototype:

- one knowledge item;
- one multi-file skill;
- one tool descriptor;
- one collection;
- one transitive dependency graph;
- one neutral workflow with nonnumeric node IDs, multiple profiles, and an
  intentionally irregular graph;
- one actionable workflow with an absent or incompatible policy-contract
  version;
- one composed workflow whose parent and child declare distinct effects;
- one incompatible resource;
- one yanked and one revoked version;
- one local source;
- one namespace collision;
- one malicious path and symlink payload;
- one interrupted transaction and recovery.

## Exit Criterion

Resource identity, schemas, trust, Hub behavior, command grammar, and workspace
state are precise enough that Hub, Manager, and Tool can implement the same
contract without inventing local interpretations.
