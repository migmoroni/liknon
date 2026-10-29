# B02 Hub Accounts, Permissions, And Manager Handoff

## Status

`Provisional`. Hub authorities are created just in time. Manager requires no
standalone publication authority.

[Back to the Resource Hub and Manager plan](README.md)

## Purpose

Stage only the external ownership, signing, storage, and publication access
needed for the B02 Hub. Record Manager's credential-free handoff requirements
for B04.

## Authority Boundaries

The human release owner controls:

- Hub domains and object storage;
- namespace and publisher governance;
- resource metadata and payload publication;
- Hub trust roots, signing, revocation, and recovery;
- protected approval environments.

B04 owns crates.io, native storage, signing, and recovery authority for the
complete CLI suite. B02 does not create Manager-specific registry ownership,
release tags, R2 prefixes, stable pointers, or platform-signing environments.

## Required Hub Identities

| Purpose | Isolated authority |
| --- | --- |
| Hub catalog and payload publication | `publish-liknon-hub` |
| Hub metadata signing | `sign-liknon-hub` |
| Resource namespace administration | Human-controlled Hub governance |
| Hub recovery and revocation | Offline recovery owner |
| Human approval | Credential-free protected environment |

Hub authority cannot publish or replace CLI binaries. CLI-suite authority
cannot alter Hub trust metadata or namespace ownership.

## Manager Handoff

B02 supplies B04 only non-secret evidence:

- source closure and supporting-crate requirements;
- behavior, security, and target-test reports;
- Hub protocol compatibility;
- semantic smoke commands;
- expected executable hash binding fields.

No Manager credential is needed to create or verify this handoff.

## Just-In-Time Checkpoints

1. Before Hub prototypes, create a non-production namespace and test trust root.
2. Before B02.5, confirm production domain, storage, cache, trust, revocation,
   retention, and recovery owners.
3. Before B06, activate narrow Hub credentials only after candidate verification
   and human approval.
4. After first publication, revoke bootstrap tokens or reduce them to the
   approved recurring scope.

## Secret Rules

- Credentials exist only in protected publication environments.
- Build, test, Manager, Tool, Validator, and site-preview jobs receive no Hub
  publication secret.
- Trust-root private keys have offline recovery and documented rotation.
- No secret enters a resource, lock, manifest, binary, archive, log, cache key,
  or retained public evidence.
- Community publishers never receive Liknon project credentials.

## Exit Criterion

Every Hub publication effect has one accountable owner, one narrow credential
boundary, one recovery path, and no coupling to CLI-suite publication.
