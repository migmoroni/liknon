# Plan B02: Liknon Resource Hub And Manager

> **B04 CLI-distribution supersession:** B02 continues to own the Hub,
> resource protocol, Manager behavior, security, and focused conformance
> evidence. Its standalone `liknon-manager` source package, `likm`
> executable alias, per-capability native archive, tag, and stable pointer are
> not the approved public target. B04 owns Manager's public executable target
> inside `liknon-cli` and the complete suite archive for each target. Hub
> publication remains independently owned by B02.

## Status And Relationship To Other Tracks

The repository-side B02 implementation is in place. B02.0-B02.4 are exercised
against deterministic local Hub and workspace fixtures. Product acceptance
still requires the human-owned Hub setup, focused Manager handoff evidence for
B04, and public Hub readback described by B02.5 and coordinated for the first
release through B06.

The [validation-system index](../README.md) owns cross-track order, milestone
gates, and contract ownership. This plan owns only the Hub, resource protocol,
and Manager track.

This plan defines the public resource protocol, the online Liknon Resource Hub,
and the independent Manager application created by
[Plan A01](../A01-product-and-repository-separation/README.md).

[B01](../B01-validation-cli-distribution/README.md) distributes validation execution.
[B03](../B03-tool-cli/README.md) distributes offline resource consumption.
[B04](../B04-unified-launcher-and-cli-suite/README.md) owns Manager's public
executable target, unified routing, and complete-suite distribution.
[B05](../B05-programmatic-api-and-language-bindings/README.md)
may later expose justified programmatic APIs. None of those tracks replaces
Manager as the canonical networked resource-state mutator.
[E01](../E01-public-site-and-hub-delivery/README.md) owns the human-facing site,
public domain topology, and verified catalog presentation without replacing
the B02-owned Hub origin, protocol, signing, publication, or recovery.

## Dependencies

B02 depends on A01 product boundaries and on C01 generic resource, workflow,
capability, effect, and policy-compatibility requirements needed by B02.0. Its
distribution phases additionally depend on the
[shared CLI distribution foundation](../cli-distribution-foundation.md), its
credential-free implementation and conformance harness established by B01, and
the [B02 external-readiness runbook](accounts-and-permissions.md). Manager's
final terminal presentation and B02.5 source-package closure additionally
depend on the completed
[A02 shared CLI experience foundation](../A02-shared-cli-experience-foundation/README.md).
B02 does not depend on publication of a Validation release. C02 and C03 are
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

Manager's direct command is `liknon-manager`. Its current implementation
package remains a focused source boundary until B04 moves public binary-target
ownership into `liknon-cli`. The target state has no `likm` executable.

Manager owns network access to the Hub and mutation below
`.liknon/resources/`. It does not validate workspaces, interpret or execute
resource instructions, answer knowledge questions, or invoke another Liknon
executable.

After B04 exists, `liknon manager <args>` and `liknon m <args>` route to
`liknon-manager <args>`. Manager neither depends on nor invokes that launcher;
its direct command remains complete when the launcher is absent.

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

Manager contributes focused behavior, security, target, and semantic evidence
to B04. B04 owns its public executable target, suite version, source package,
tag, native archives, stable pointer, credentials, approval, and recovery.

The `apps/liknon-manager` package may remain an internal implementation
boundary, but it owns no public executable target after the B04 cutover.
`liknon-cli` must build Manager from a complete publishable source closure.
The website offers only the complete B04 suite archive selected for the target.

Resource versions and Hub publication proceed independently from Manager
releases. Publishing or updating a resource never requires rebuilding Manager.
The shared CLI release foundation does not publish Hub metadata or payloads;
those remain a separate B02.1 authority and pipeline.

For the first public Liknon release,
[B06](../B06-coordinated-public-release/README.md) records the B04 CLI-suite
candidate and the independently versioned B02 Hub candidate, invokes their
owning authorities in dependency order, and performs coordinated readback.

## Deliverables And Implementation Order

| Phase | Document | Depends on | Primary result |
| ---: | --- | --- | --- |
| B02.0 | [Resource And Hub Contract](00-resource-and-hub-contract.md) | A01.4 and the C01 generic governance baseline | Approved identities, schemas, trust, compatibility, and command grammar |
| B02.1 | [Hub Catalog And Publication Pipeline](01-hub-catalog-and-publication.md) | B02.0 | Authenticated catalog, immutable payloads, namespaces, and publication workflow proven against a local/test origin |
| B02.2 | [Manager Workspace And Commands](02-manager-workspace-and-commands.md) | B02.0 | Local state, discovery, inspection, and stable human/JSON interfaces |
| B02.3 | [Resolution, Locking, And Installation](03-resolution-locking-and-installation.md) | B02.1-B02.2 | Deterministic dependency resolution and transactional resource lifecycle |
| B02.4 | [Security, Trust, And Recovery](04-security-trust-and-recovery.md) | B02.3 | Fail-closed verification, rollback, and adversarial coverage |
| B02.5 | [Distribution And End-To-End Verification](05-distribution-and-end-to-end-verification.md) | B02.4, A02, and the credential-free shared distribution foundation | Manager handoff evidence for B04, an independent Hub candidate, and verified Hub-to-workspace behavior |

B02.0-B02.4 establish local implementation readiness. Together with B03.2 they
provide the neutral interoperability evidence required before C02 or C03
official resources are implemented. B02.5 is a separate release-readiness
milestone and is not a prerequisite for authoring those resources. Its first
public activation occurs through B06.

## Global Acceptance Criteria

Checked items below are satisfied by the B02.0-B02.4 repository implementation
and deterministic local Hub and workspace fixtures. Unchecked items still
require B03, E01, B02.5, publication, or public B06 readback.

- [ ] B04 exposes `liknon-manager` directly and through `manager|m` without a
      physical `likm` executable.
- [x] Manager is the only Liknon CLI that contacts the Hub or mutates installed
      resource state.
- [x] Knowledge can be installed individually, by focused collection, or by an
      optional complete collection.
- [x] One manifest and lock deterministically reproduce the selected resource
      graph.
- [x] Every installed byte is covered by authenticated metadata and an exact
      digest.
- [x] Installation, update, removal, and materialization are preflighted,
      path-safe, transactional, and recoverable.
- [x] Installing a resource never executes it or activates it implicitly.
- [x] Project-controlled conformance workflows and external workflows use the
      same graph, dependency, capability, effect, and agent-profile reference
      contracts; later official workflows require no application branch.
- [x] A workflow with absent or incompatible decision-policy compatibility may
      be installed and inspected but is never reported as activatable.
- [x] A workflow with absent or incompatible AI bootstrap compatibility follows
      the same installable-but-not-activatable behavior.
- [x] No catalog entry or installed resource can replace or shadow the
      program-owned AI bootstrap contract.
- [x] Neutral operational-skill fixtures and compatible alternatives resolve
      through the same dependency and trust contracts used later by AI-Tool.
- [x] Manager contains no branch for an official resource ID, process name, or
      workflow shape.
- [x] Tool can consume Manager-produced state without invoking Manager.
- [x] Resource publication does not rebuild any CLI.
- [ ] The `liknon-cli` source closure resolves only published supporting
      crates.
- [ ] The site offers one complete B04 suite archive per target.
- [ ] Manager evidence is incorporated without giving Manager an independent
      public package, native archive, or stable pointer.
- [x] Manager uses `liknon-cli-ui` for shared terminal behavior while retaining
      its own commands, human content, JSON schema, errors, and exit codes.
- [ ] Every Manager executable resolves to the B04 suite version, exact signed
      tag, full commit, and immutable provenance record.
- [ ] Every supported B04 target has Manager native evidence before suite
      publication.
- [ ] Generated state and credentials enter no published resource or retained
      public evidence.

## Resulting System

Liknon has one open, versioned resource ecosystem and one canonical client for
changing local resource state. Users can install only the knowledge and
operational assistance they need while retaining exact provenance,
reproducibility, and control.
