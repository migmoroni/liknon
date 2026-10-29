# Plan B06: Coordinated Public Release

## Status

`Future`. Blocked until A02, B01 through B05, and E01 have completed the
release-readiness work applicable to the supporting package, products, and
public surfaces selected for the first public release.

[Back to the validation-system roadmap](../README.md)

## Purpose

B01 through B05 define, build, test, and retain the candidates owned by each
product. B06 is the final coordination plan that turns those independently
verified candidates into one deliberate first public release of the Liknon
system.

This plan does not redesign products, rebuild accepted candidates, merge
credentials, or transfer publication authority. It freezes the exact release
set, verifies every external authority, rehearses the complete sequence,
coordinates irreversible publication, verifies public readback, and closes the
bootstrap credentials used for the first release.

## Products In Scope

The release set may contain:

| Owner | Product or channel | B06 input |
| --- | --- | --- |
| A02 | `liknon-cli-ui` supporting crate | Verified source-package candidate required by selected CLI packages |
| B01 | `liknon-validator` / `likv` | Verified source and native candidates |
| B02 | Resource Hub and `liknon-manager` / `likm` | Verified Hub objects plus Manager candidates |
| B03 | `liknon-tool` / `likt` | Verified source and native candidates |
| B04 | `liknon-cli`, `liknon` / `likx`, and complete suites | Verified aggregate source and native candidates |
| B05 | Activated programmatic APIs and language packages | Verified candidates only for ecosystems whose activation gate passed |
| E01 | `liknon.org` site and public Hub delivery surface | Verified site candidate, domain topology, trust-root delivery, and exact B02 Hub identity |

An unimplemented B05 ecosystem is explicitly excluded. B06 never publishes a
placeholder package merely to reserve a name or make the release set appear
complete.

## Ownership Boundary

- A02 owns its supporting-crate source release, while B01-B05 own product
  versions, tags, package contents, target matrices, candidates,
  destination-specific workflows, credentials, recovery rules, and recurring
  release trains.
- E01 owns the site candidate, site deployment identity, public-domain
  topology, and verified human presentation of B02 and product-release outputs.
- B06 owns the frozen first-release ledger, dependency order, coordinated human
  approval, cross-product rehearsal, public activation record, integrated
  readback, and closure evidence.
- The shared CLI distribution foundation owns reusable mechanisms, not a
  product release or credential.
- A02 owns the `liknon-cli-ui` package version, crates.io candidate,
  publication authority, and recurring supporting-crate release train; B06
  only coordinates its first publication before dependent application crates.
- D01 may consume the released system for evaluation but does not publish it.

There is no universal publication credential. The B06 coordinator invokes each
product's protected workflow and environment; it cannot publish directly to
every destination.

## Release Principles

1. Every published byte comes from an exact retained candidate already
   accepted by its owning plan.
2. Product versions remain independent. Coordination does not force one shared
   semantic version.
3. One human approval authorizes the frozen release set before the first
   irreversible destination publication. Destination protections still enforce
   their own approvals and least privilege.
4. Publication follows the actual dependency graph, not a hard-coded list that
   ignores package or resource dependencies.
5. Cross-registry publication is not atomic. A partial release is recorded and
   resumed from the same candidates; accepted versions are never replaced.
6. Stable pointers are promoted only after all selected immutable channels pass
   public readback.
7. The aggregate suite pointer is promoted last.

## Phases

| Phase | Document | Primary result |
| ---: | --- | --- |
| B06.0 | [Release Set And Freeze](00-release-set-and-freeze.md) | One approved ledger of exact products, versions, candidates, dependencies, and exclusions |
| B06.1 | [External Authority And Trust](01-external-authority-and-trust.md) | Every required account, environment, key, domain, registry, and authority is ready without shared credentials |
| B06.2 | [Candidate Rehearsal](02-candidate-rehearsal.md) | Complete credential-free rehearsal using the exact retained candidates |
| B06.3 | [Coordinated Publication](03-coordinated-publication.md) | Dependency-ordered public activation with resumable state |
| B06.4 | [Public Verification And Closure](04-public-verification-and-closure.md) | Public readback, stable-pointer promotion, documentation, and bootstrap-secret closure |

The operational prerequisites are consolidated in
[Accounts And Permissions](accounts-and-permissions.md).

## Entry Criteria

- B01-B04 have produced release-ready candidates and destination workflows.
- A02 has produced a release-ready `liknon-cli-ui` source candidate whenever a
  selected application package depends on a version not already public and
  byte-identical.
- E01 has produced a release-ready site candidate and verified public domain,
  Hub, trust-root, and site-delivery inputs selected for the release.
- Every B05 ecosystem included in the release set has passed its own activation
  gate and produced a real programmatic API candidate.
- D01 has completed every integrated scenario designated as blocking for the
  first public release.
- Candidate retention covers the complete rehearsal, publication, readback,
  and recovery window.
- No selected artifact depends on an unpublished path, workspace package,
  mutable branch, or mutable Hub object.
- The human release owner has approved the release-set ledger.

## Completion Criteria

- [ ] The exact release set and explicit exclusions are retained as evidence.
- [ ] All selected products are publicly available from their declared
      channels.
- [ ] Every public object, package, tag, manifest, and archive matches the
      retained candidate and provenance record.
- [ ] Every supported installation and representative integrated workflow
      passes from a clean environment.
- [ ] Partial failures, if any, were resumed without rebuilding or overwriting.
- [ ] Stable pointers were promoted only after immutable public verification.
- [ ] Bootstrap credentials were revoked or reduced to the approved recurring
      release model.
- [ ] Public documentation states exactly which products, channels, platforms,
      and trust modes are available.
- [ ] Each product returns to its independent recurring release train after the
      coordinated first release.

## Result

Liknon has one auditable first public release across all products that are
actually ready. The coordination is repeatable without turning later product
releases into one permanently coupled train.
