# Plan B06: Coordinated Public Release

## Status

`Future`. Blocked until B04 has produced the complete CLI-suite candidate,
B02 has produced the selected Hub candidate, E01 has produced the site
candidate, D01 has produced the required blocking evidence, and every selected
B05 integration has passed its own activation gate.

[Back to the validation-system roadmap](../README.md)

## Purpose

B06 turns exact, already reviewed candidates into one deliberate first public
release. It does not redesign products, rebuild payloads, split the CLI suite,
merge credentials, or transfer recurring publication authority.

B01, B02, and B03 provide focused behavior and conformance evidence to B04.
B04 is the sole owner of the public CLI-suite candidate consumed here.

## Release Set

The release set may contain:

| Owner | Product or channel | B06 input |
| --- | --- | --- |
| A02 and other supporting-crate owners | Publishable Rust dependencies required by `liknon-cli` | Exact source-package candidates in dependency order |
| B02 | Resource Hub | Immutable catalog, payload, trust, revocation, and stable metadata candidates |
| B04 | Liknon CLI suite | One `liknon-cli` source candidate plus one complete four-executable archive per supported target |
| B05 | Activated programmatic APIs and language packages | Exact candidates only for ecosystems whose activation gate passed |
| E01 | `liknon.org` site and public Hub/download presentation | Exact site candidate, domain topology, trust-root delivery, and links to B02/B04 outputs |

An unimplemented B05 ecosystem is excluded. B06 never publishes a placeholder
package merely to reserve a name.

## CLI Suite Input

The B04 candidate has one release identity and contains:

- source package `liknon-cli`;
- executable `liknon`;
- executable `liknon-validator`;
- executable `liknon-manager`;
- executable `liknon-tool`;
- one authenticated native archive for each supported target;
- one suite stable pointer;
- source, target, archive, executable, provenance, and installer evidence.

B06 does not accept separate Validator, Manager, or Tool packages, native
archives, versions, tags, or stable pointers.

## Ownership Boundary

- B04 owns the CLI-suite version, tag, package contents, target matrix,
  candidates, destination workflow, recovery, and recurring release train.
- B02 owns Hub versions, objects, trust metadata, publication, and recovery.
- B05 owns each activated programmatic ecosystem.
- E01 owns the site candidate, deployment identity, and human presentation.
- Supporting-crate owners retain their package authority.
- B06 owns the frozen first-release ledger, dependency order, coordinated human
  approval, rehearsal, activation record, public readback, and closure evidence.
- D01 evaluates but does not publish.

There is no universal publication credential. The coordinator invokes
destination-specific protected workflows with least-privilege credentials.

## Release Principles

1. Every published byte comes from an exact retained candidate accepted by its
   owning plan.
2. All four CLI executables share the B04 suite version and are promoted
   atomically as one CLI product.
3. Hub resources and activated B05 integrations retain their own versions and
   release identities.
4. One human approval authorizes the frozen release set before the first
   irreversible publication.
5. Publication follows the real dependency graph.
6. Cross-destination publication is resumable, not falsely treated as atomic.
7. Stable pointers move only after immutable public readback.
8. The CLI-suite stable pointer is promoted only when every selected target
   archive and the source package pass verification.
9. No phase rebuilds, renames, or recomposes accepted payloads.

## Phases

| Phase | Document | Primary result |
| ---: | --- | --- |
| B06.0 | [Release Set And Freeze](00-release-set-and-freeze.md) | One approved ledger of exact candidates, targets, dependencies, destinations, and exclusions |
| B06.1 | [External Authority And Trust](01-external-authority-and-trust.md) | Every required account, key, environment, domain, registry, and authority is ready |
| B06.2 | [Candidate Rehearsal](02-candidate-rehearsal.md) | Credential-free rehearsal using the exact retained candidates |
| B06.3 | [Coordinated Publication](03-coordinated-publication.md) | Dependency-ordered public activation with resumable state |
| B06.4 | [Public Verification And Closure](04-public-verification-and-closure.md) | Public readback, stable-pointer promotion, documentation, and bootstrap-secret closure |

Operational prerequisites remain in
[Accounts And Permissions](accounts-and-permissions.md).

## Entry Criteria

- B04 has one release-ready `liknon-cli` source candidate and one complete
  archive per supported target.
- B01-B03 focused behavior and security gates pass for direct and launcher-routed
  execution in that exact B04 candidate.
- Every supporting Rust package required by `liknon-cli` has an exact
  publishable candidate or is already public and byte-identical.
- B02 has a release-ready Hub candidate.
- E01 has a release-ready site candidate and verified domain, Hub, trust-root,
  and suite-download inputs.
- Every included B05 ecosystem has a real programmatic API candidate.
- D01 has completed every integrated scenario designated as blocking.
- Candidate retention covers rehearsal, publication, readback, and recovery.
- No selected artifact depends on an unpublished path, mutable branch, mutable
  Hub object, or reconstructed binary.
- The human release owner has approved the immutable release-set ledger.

## Completion Criteria

- [ ] The exact release set and explicit exclusions are retained.
- [ ] The public CLI source package installs exactly four executables.
- [ ] Every supported target exposes exactly one complete native suite archive.
- [ ] No standalone Validator, Manager, Tool, or compact-alias artifact exists.
- [ ] Every public object, package, tag, manifest, archive, and executable
  matches its retained candidate and provenance.
- [ ] Hub, site, selected B05 integrations, and supporting packages pass public
  readback.
- [ ] Clean installations pass direct and launcher-routed workflows.
- [ ] Partial failures were resumed without rebuilding or overwriting.
- [ ] Stable pointers moved only after immutable verification.
- [ ] Bootstrap credentials were revoked or reduced to the recurring model.
- [ ] Public documentation states the exact products, channels, targets, and
  trust modes available.

## Result

Liknon has one auditable first public CLI-suite release alongside the exact Hub,
site, and activated programmatic products that are genuinely ready. Later
releases retain their owning boundaries without splitting the four-executable
CLI suite into independent public products.
