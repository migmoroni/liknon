# Plan B02: Liknon Resource Hub And Manager

## Status And Relationship To Other Tracks

`Decision gate` at B02.0. The product role and distribution-channel categories
are approved. Command grammar, public resource schemas, Hub behavior, trust,
and recovery are not approved until B02.0 exits. B02.1-B02.5 are
`Provisional`.

The [validation-system index](../README.md) owns cross-track order, milestone
gates, and contract ownership. This plan owns only the Hub, resource protocol,
and Manager track.

This plan defines the public resource protocol, the online Liknon Resource Hub,
and the independent Manager application created by
[Plan A01](../A01-product-and-repository-separation/README.md).

[B01](../B01-validation-cli-distribution/README.md) distributes validation execution.
[B03](../B03-tool-cli/README.md) distributes offline resource consumption.
[B04](../B04-unified-launcher-and-cli-suite/README.md) owns unified routing and
complete-suite composition. [B05](../B05-programmatic-api-and-language-bindings/README.md)
may later expose justified programmatic APIs. None of those tracks replaces
Manager as the canonical networked resource-state mutator.

## Dependencies

B02 depends on A01 product boundaries and on C01 generic resource, workflow,
capability, effect, and policy-compatibility requirements needed by B02.0. Its
distribution phases additionally depend on the
[B02 external-readiness runbook](accounts-and-permissions.md). C02 and C03 are
consumers, not prerequisites for B02.0.

## Objective

Provide one trustworthy path for discovering, selecting, obtaining, locking,
verifying, updating, removing, and explicitly materializing Liknon resources.

The initial singular resource-kind IDs are `knowledge`, `skill`, `workflow`,
`agent-profile`, `tool`, `prompt`, `guide`, `template`, and
`collection`. B02.0 owns their exact pluralized physical-directory mapping.
Users select only what they need.
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

After B04 exists, `liknon manager <args>` is the unified form of
`liknon-manager <args>`. Manager neither depends on nor invokes that launcher;
its direct commands remain complete when the launcher is absent.

The Hub owns public metadata and immutable payload distribution. It is not a
CLI and does not mutate consumer workspaces.

## Exclusions

B02 does not execute validation, interpret or activate installed instructions,
host AI models, conduct workflows, answer knowledge queries, or define Tool's
read-only command surface. It does not publish Validation CLI or Tool artifacts.

## Candidate Command Families

B02.0 must approve the final grammar. The following operations are design
inputs to evaluate, merge, rename, split, or reject rather than an approved
command list:

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

No downstream phase may implement this table before B02.0 records the selected
grammar, outputs, effects, and confirmation points.

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
.liknon/resources/<kind-directory>/<namespace>/<name>/<version>/
```

The resource manifest records the singular kind ID; B02.0 maps it to exactly one
directory name. The lock, rather than the filesystem spelling alone, is
authoritative. A namespace such as `official` identifies a publisher boundary
and grants no special execution or policy authority.

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

The website offers Manager separately. B04 may consume its verified candidate
in a complete native suite, without transferring Manager publication or
introducing runtime dependencies among capability applications.

Resource versions and Hub publication proceed independently from Manager
releases. Publishing or updating a resource never requires rebuilding Manager.

## Deliverables And Implementation Order

| Phase | Document | Depends on | Primary result |
| ---: | --- | --- | --- |
| B02.0 | [Resource And Hub Contract](00-resource-and-hub-contract.md) | A01.4 and the C01 generic governance baseline | Approved identities, schemas, trust, compatibility, and command grammar |
| B02.1 | [Hub Catalog And Publication Pipeline](01-hub-catalog-and-publication.md) | B02.0 | Authenticated catalog, immutable payloads, namespaces, and publication workflow proven against a local/test origin |
| B02.2 | [Manager Workspace And Commands](02-manager-workspace-and-commands.md) | B02.0 | Local state, discovery, inspection, and stable human/JSON interfaces |
| B02.3 | [Resolution, Locking, And Installation](03-resolution-locking-and-installation.md) | B02.1-B02.2 | Deterministic dependency resolution and transactional resource lifecycle |
| B02.4 | [Security, Trust, And Recovery](04-security-trust-and-recovery.md) | B02.3 | Fail-closed verification, rollback, and adversarial coverage |
| B02.5 | [Distribution And End-To-End Verification](05-distribution-and-end-to-end-verification.md) | B02.4 | Published Manager channels and verified Hub-to-workspace behavior |

B02.0-B02.4 establish local implementation readiness. Together with B03.2 they
provide the neutral interoperability evidence required before C02 or C03
official resources are implemented. B02.5 is a separate public-distribution
milestone and is not a prerequisite for authoring those resources.

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
- [ ] Project-controlled conformance workflows and external workflows use the
      same graph, dependency, capability, effect, and agent-profile reference
      contracts; later official workflows require no application branch.
- [ ] A workflow with absent or incompatible decision-policy compatibility may
      be installed and inspected but is never reported as activatable.
- [ ] A workflow with absent or incompatible AI bootstrap compatibility follows
      the same installable-but-not-activatable behavior.
- [ ] No catalog entry or installed resource can replace or shadow the
      program-owned AI bootstrap contract.
- [ ] Neutral operational-skill fixtures and compatible alternatives resolve
      through the same dependency and trust contracts used later by AI-Tool.
- [ ] Manager contains no branch for an official resource ID, process name, or
      workflow shape.
- [ ] Tool can consume Manager-produced state without invoking Manager.
- [ ] Resource publication does not rebuild any CLI.
- [ ] The crates.io package resolves only published supporting crates.
- [ ] The site offers Manager separately; any complete suite is owned by B04
      and consumes this plan's verified candidate unchanged.
- [ ] Generated state and credentials enter no published resource or retained
      public evidence.

## Resulting System

Liknon has one open, versioned resource ecosystem and one canonical client for
changing local resource state. Users can install only the knowledge and
operational assistance they need while retaining exact provenance,
reproducibility, and control.
