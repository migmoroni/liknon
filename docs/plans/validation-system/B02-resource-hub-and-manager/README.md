# Plan B02: Liknon Resource Hub And Manager

## Status And Relationship To Other Tracks

This plan defines the public resource protocol, the online Liknon Resource Hub,
and the independent Manager application created by
[Plan A01](../A01-product-and-repository-separation/README.md).

[B01](../B01-cli-distribution/README.md) distributes validation execution.
[B03](../B03-tool-cli/README.md) distributes offline resource consumption.
[B04](../B04-programmatic-api-and-language-bindings/README.md) may later expose
justified programmatic APIs. None of those tracks replaces Manager as the
canonical networked resource-state mutator.

## Objective

Provide one trustworthy path for discovering, selecting, obtaining, locking,
verifying, updating, removing, and explicitly materializing Liknon resources.

The initial resource kinds are knowledge, skills, workflows, agent profiles,
tool descriptors, prompts, guides, templates, and collections. Users select only
what they need.
A complete knowledge collection may be offered, but it is optional and follows
the same resolver and lock contract as every smaller selection.

## Product Boundary

The Cargo application package is `liknon-manager`. It installs two equivalent
commands:

- `liknon-manager`, the explicit command;
- `likm`, the compact command.

Manager owns network access to the Hub and mutation below
`.liknon/resources/`. It does not validate workspaces, interpret or execute
resource instructions, answer knowledge questions, or invoke another Liknon
executable.

The Hub owns public metadata and immutable payload distribution. It is not a
CLI and does not mutate consumer workspaces.

## Command Families

The final command grammar is approved in B02.0. It must cover these semantic
operations:

| Operation | Responsibility |
| --- | --- |
| `init` | Create only the requested resource manifest and lock foundation |
| `search` | Query Hub metadata without installing content |
| `info` | Show one available or installed resource's administrative metadata |
| `install` | Resolve and transactionally install explicit resource specs |
| `update` | Re-resolve only the requested scope under declared constraints |
| `remove` | Remove requested resources when dependency and materialization rules permit |
| `list` | Report desired, locked, installed, and drifted resource state |
| `verify` | Authenticate metadata and verify installed payload bytes |
| `materialize` | Explicitly project selected files into a caller-approved integration destination |

Administrative `list` and `info` describe state, versions, origin,
integrity, and updates. B03 Tool commands consume resource content and do not
duplicate these mutation-oriented semantics.

## Workspace Contract

```text
.liknon/resources/
├── manifest.json
├── lock.json
├── knowledge/
├── skills/
├── workflows/
├── agent-profiles/
├── tools/
├── prompts/
├── guides/
├── templates/
└── collections/
```

`manifest.json` is human-readable desired state. `lock.json` is the exact
resolution: identities, versions, origins, compatibility, dependency edges,
payload paths, lengths, and digests.

Installed payloads are immutable for one locked identity. Manager replaces
state only through a complete preflight and atomic commit. Cache entries and
partial downloads remain outside the workspace.

Payload paths are type-first and namespace-aware:

```text
.liknon/resources/<kind>/<namespace>/<name>/<version>/
```

The lock, rather than the filesystem spelling alone, is authoritative. A
namespace such as `official` identifies a publisher boundary and grants no
special execution or policy authority.

## Hub And Trust Model

Every catalog entry declares namespace, resource kind, version, authorship,
license, source, compatibility, dependencies, payload metadata, capabilities,
and review state.

The initial trust classes are:

- `official`: published by the Liknon project;
- `verified`: third-party identity and review evidence satisfy the documented
  Hub process;
- `community`: listed without Liknon endorsement;
- `local`: supplied outside the public Hub by an explicit caller.

Catalog presence never means instruction authority. Manager exposes trust and
provenance, verifies exact bytes, and never executes payload content.

## Distribution Contract

Manager has an independent version and release train. It is available through:

- exact tagged GitHub source;
- the `liknon-manager` crates.io source package;
- authenticated target-native archives through the project download domain.

The crates.io release publishes and verifies any exact supporting-crate version
required by Manager before publishing the application package. Installing
`liknon-manager` never depends on an unpublished workspace path.

The website offers Manager separately and may include it in a convenience
bundle with Validation CLI and Tool. The bundle does not introduce runtime
dependencies among them.

Resource versions and Hub publication proceed independently from Manager
releases. Publishing or updating a resource never requires rebuilding Manager.

## Implementation Order

| Phase | Document | Depends on | Primary result |
| ---: | --- | --- | --- |
| B02.0 | [Resource And Hub Contract](00-resource-and-hub-contract.md) | A01.4 | Approved identities, schemas, trust, compatibility, and command grammar |
| B02.1 | [Hub Catalog And Publication](01-hub-catalog-and-publication.md) | B02.0 | Authenticated catalog, immutable payloads, namespaces, and publication workflow |
| B02.2 | [Manager Workspace And Commands](02-manager-workspace-and-commands.md) | B02.0 | Local state, discovery, inspection, and stable human/JSON interfaces |
| B02.3 | [Resolution, Locking, And Installation](03-resolution-locking-and-installation.md) | B02.1-B02.2 | Deterministic dependency resolution and transactional resource lifecycle |
| B02.4 | [Security, Trust, And Recovery](04-security-trust-and-recovery.md) | B02.3 | Fail-closed verification, rollback, and adversarial coverage |
| B02.5 | [Distribution And End-To-End Verification](05-distribution-and-end-to-end-verification.md) | B02.4 | Published Manager channels and verified Hub-to-workspace behavior |

## Global Acceptance Criteria

- [ ] `liknon-manager` and `likm` are equivalent entry points.
- [ ] Manager is the only Liknon CLI that contacts the Hub or mutates installed
      resource state.
- [ ] Knowledge can be installed individually, by focused collection, or by an
      optional complete collection.
- [ ] One manifest and lock deterministically reproduce the selected resource
      graph.
- [ ] Every installed byte is covered by authenticated metadata and an exact
      digest.
- [ ] Installation, update, removal, and materialization are preflighted,
      path-safe, transactional, and recoverable.
- [ ] Installing a resource never executes it or activates it implicitly.
- [ ] Official and external workflows use the same graph, dependency,
      capability, effect, and agent-profile reference contracts.
- [ ] A workflow with absent or incompatible decision-policy compatibility may
      be installed and inspected but is never reported as activatable.
- [ ] A workflow with absent or incompatible AI bootstrap compatibility follows
      the same installable-but-not-activatable behavior.
- [ ] No catalog entry or installed resource can replace or shadow the
      program-owned AI bootstrap contract.
- [ ] Official AI-Tool and compatible alternative operational skills resolve
      through the same dependency and trust contracts.
- [ ] Manager contains no branch for an official resource ID, process name, or
      workflow shape.
- [ ] Tool can consume Manager-produced state without invoking Manager.
- [ ] Resource publication does not rebuild any CLI.
- [ ] The crates.io package resolves only published supporting crates.
- [ ] The site offers each CLI separately and treats a complete bundle only as
      a convenience.
- [ ] Generated state and credentials enter no published resource or retained
      public evidence.

## Resulting System

Liknon has one open, versioned resource ecosystem and one canonical client for
changing local resource state. Users can install only the knowledge and
operational assistance they need while retaining exact provenance,
reproducibility, and control.
