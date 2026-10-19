# B02 Hub And Manager Accounts, Permissions, And Authority

## Status

`Provisional`. Blocked by B02.0 and completed just in time for each delivery
phase.

[Back to the Resource Hub and Manager plan](README.md)

## Purpose

Stage only the external ownership, signing, storage, and publication access
needed by the B02 phase currently being implemented.

## Authority Boundaries

The human release owner controls:

- the `liknon-manager` package and every supporting crates.io package selected
  by the approved A01 ledger;
- Manager release tags and approval;
- the Hub domain, object storage, namespace policy, and trust roots;
- official resource publication and trust-class governance;
- recovery identities and incident response.

GitHub Actions may build, verify, and publish reviewed immutable candidates.
Build and test jobs receive no publication credential.

## Required Identities

| Purpose | Isolated authority |
| --- | --- |
| Manager and required supporting-crate publication | `publish-crates-manager` |
| Manager GitHub metadata release | `publish-github-manager` |
| Manager native artifacts | `publish-cloudflare-manager` |
| Hub catalog and payload publication | `publish-liknon-hub` |
| Hub metadata signing | `sign-liknon-hub` |
| Manager native signing | Platform-specific Manager signing environments |
| Human approval | Credential-free protected environments |

Hub publication cannot publish Manager binaries. Manager publication cannot
alter Hub trust metadata or resource namespaces.

## Just-In-Time Checkpoints

### Contract Work

Before B02.0, confirm names and responsible owners. No upload credential is
needed.

### Hub Prototype

Before B02.1, create a non-production catalog and object namespace with a
separate test trust root. Do not place production signing keys in prototype
jobs.

### Manager Candidate

Before B02.5 candidate builds, confirm the `liknon-manager` and
supporting-package identities selected by A01, native target matrix, protected
environments, and artifact retention.

### Publication

Activate narrow production credentials only after credential-free candidate
verification and human approval. Prefer workload identity or trusted
publishing where supported. Bootstrap tokens are temporary and revoked after
use.

## Secret Rules

- Credentials reside only in their protected publication environments.
- Trust-root private keys have offline recovery and documented rotation.
- Resource publisher credentials cannot change namespace ownership policy.
- No secret enters a resource, lock, manifest, binary archive, log, cache key,
  or retained public evidence.
- Community publishers never receive Liknon project publication credentials.

## Exit Criterion

Every B02 publication effect has one accountable owner, one narrow credential
boundary, one recovery path, and no credential exposure to build or resource
content.
