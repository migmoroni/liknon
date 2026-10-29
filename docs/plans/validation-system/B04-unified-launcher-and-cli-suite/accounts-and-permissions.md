# B04 CLI Suite Accounts, Permissions, And Publication Authority

## Status

`Future`. Provision each authority only when the corresponding B04 candidate
and B06 release step are ready.

[Back to Plan B04](README.md)

## Purpose

Define the least-privilege external authority required to publish the complete
`liknon-cli` source and native suite without restoring per-capability
publication identities.

## Authority Boundaries

B04 owns publication authority for:

- the `liknon-cli` crates.io package;
- supporting Rust crates required by its publishable source closure;
- the signed CLI-suite source tag;
- one immutable native suite archive per target;
- suite manifests, signatures, provenance, installers, and stable pointer;
- platform-specific authenticity for the complete target archive.

B02 separately owns Hub authority. E01 owns site deployment. B05 owns activated
programmatic registries. B06 coordinates but does not receive a universal
credential.

## Required Identities

| Purpose | Isolated authority |
| --- | --- |
| Supporting Rust packages | Package-specific trusted publishing or narrow tokens |
| `liknon-cli` source package | `publish-crates-cli-suite` |
| Signed source tag and metadata | `publish-github-cli-suite` |
| Immutable native suite objects | `publish-cloudflare-cli-suite` |
| Suite metadata signing | `sign-liknon-cli-suite` |
| Windows/macOS authenticity | Target-specific protected signing environments |
| Stable pointer promotion | `promote-cli-suite-stable` |
| Human approval | Credential-free protected environments |

No authority is scoped to Validator, Manager, or Tool as an independent public
product.

## Separation Rules

- Build and test jobs receive no publication credential.
- Target build jobs cannot write public storage.
- Signing jobs receive only the reviewed target payloads and cannot rebuild.
- Immutable-object publication cannot advance the stable pointer.
- Stable-pointer promotion cannot replace immutable objects.
- Site deployment cannot publish CLI payloads.
- Hub publication cannot publish CLI payloads.
- No single credential can publish crates.io source, R2 objects, and stable
  metadata.

## Just-In-Time Checkpoints

1. Reserve or confirm source-package ownership before B04 source rehearsal.
2. Create non-production storage and test signing roots before installer
   prototypes.
3. Confirm production domains, R2 retention, cache policy, trust roots, and
   platform-authenticity modes before candidate freeze.
4. Enable narrow publication credentials only after B06 freezes and approves
   the exact candidate.
5. Revoke bootstrap tokens or reduce them to recurring least privilege after
   public readback.

## Secret Rules

- Secrets reside only in protected publication environments.
- Private trust material has an offline recovery and rotation procedure.
- No secret enters a source package, executable, archive, manifest, provenance
  statement, installer, log, cache key, or retained public report.
- Fork and pull-request workflows receive no production secret.
- Candidate identifiers and hashes are non-secret and are passed instead of raw
  credentials between workflows.

## Exit Criterion

Every CLI-suite publication effect has one accountable owner, one narrow
credential boundary, one recovery path, and no per-capability publication
authority.

