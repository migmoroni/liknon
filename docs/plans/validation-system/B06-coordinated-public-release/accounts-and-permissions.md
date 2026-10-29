# B06 Accounts And Permissions

## Purpose

This is the consolidated human readiness checklist for the coordinated first
public release. It records ownership and completion state, never secret values.
The detailed product plans remain authoritative for exact environment names,
permissions, and recovery procedures.

[Back to Plan B06](README.md)

## Readiness Ledger

| Area | Required readiness | Owning plan | State |
| --- | --- | --- | --- |
| Canonical GitHub repository | Protected default branch, Actions enabled, signed-tag policy, immutable-release settings, environment reviewers, artifact retention | B04-B05 | Pending |
| Supporting Rust packages | `liknon-cli-ui`, `liknon-resource-contracts`, and any other exact publishable dependency required by `liknon-cli` | A02/B02/B04 | Pending |
| CLI-suite crates.io package | `liknon-cli` ownership, trusted publisher or scoped bootstrap token, and exact supporting-crate publication order | B04 | Pending |
| CLI-suite native storage | R2 storage, download domain, public-read policy, cache behavior, bucket locks, and suite-only write authority | B04 | Pending |
| CLI-suite metadata signing | Offline or protected private key, published public key, rotation, revocation, and recovery owner | B04 | Pending |
| Hub publication | Catalog authority, metadata-signing key, immutable payload storage, namespace ownership, and recovery owner | B02 | Pending |
| Website | Authority to publish exact installation and trust documentation after public readback | E01 | Pending |
| Platform signing | Approved signing identity and disclosed trust mode for every selected Windows or macOS suite target | B04 | Pending or excluded |
| B05 registries | Namespace, trusted publisher or scoped token, recovery owner, and public-readback account for each activated ecosystem | B05 | Pending or excluded |
| Incident response | Contact, stop authority, partial-release owner, credential-revocation procedure, and status channel | B06 | Pending |

## Secret Placement Rules

- Store publication credentials only in the protected environment that owns
  the corresponding destination.
- Do not store credentials in this ledger, repository files, build artifacts,
  candidate manifests, logs, or local fixtures.
- Do not expose publication credentials to pull-request, build, package,
  rehearsal, or public-verification jobs.
- Use short-lived federation or trusted publishing when available. If the first
  publication requires a bootstrap token, scope it narrowly, create it just in
  time, and revoke it during B06.4.
- Keep Hub signing, release-manifest signing, registry publication, Cloudflare
  publication, website publication, and platform signing as distinct
  authorities.

## Human Checkpoints

1. Approve the frozen release-set ledger.
2. Confirm every selected destination and public trust root.
3. Approve the rehearsal report.
4. Authorize the first irreversible publication.
5. Review any partial-release decision before resume or abandonment.
6. Approve stable-pointer promotion.
7. Confirm bootstrap-secret revocation and final evidence retention.

## Completion

This checklist is complete only when every selected row is `Ready`, every
unused row is explicitly `Excluded`, and no required authority is represented
only by an undocumented credential on one maintainer's machine.
