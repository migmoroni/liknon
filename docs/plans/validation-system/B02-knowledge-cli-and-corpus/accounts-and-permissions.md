# B02 External Accounts, Permissions, And Publication Authority

[Back to the knowledge CLI and corpus plan](README.md)

## Purpose

Stage only the account, permission, signing, and credential work required by
the next B02 phase. This is a just-in-time runbook.

## Canonical Identities

| Purpose | Canonical identity |
| --- | --- |
| Repository | `migmoroni/liknon` |
| crates.io package | `liknon-knowledge` |
| Source tag | `liknon-knowledge-v<VERSION>` |
| Human approval | `knowledge-release-approval` |
| crates.io publication | `publish-crates-knowledge` |
| Detached release signing | `sign-knowledge-release` |
| Windows native signing | `sign-knowledge-windows` |
| GitHub Release publication | `publish-github-knowledge` |
| Cloudflare prefix publication | `publish-cloudflare-knowledge` |

## Credential Rules

- Prefer crates.io trusted publishing. Use a narrow temporary bootstrap token
  only if the first publication requires it, then revoke it.
- Scope Cloudflare credentials to the knowledge release prefix or the narrowest
  available bucket boundary.
- Keep detached signing, Windows signing, crates.io, GitHub, and Cloudflare
  identities separate.
- Build and verification jobs receive no publication credential.
- Store no secret in the repository, package, archive, log, cache key, or
  retained evidence.
- The human release owner controls account recovery, signed tags, and final
  approval.

## Checkpoints

### AP-0: Identity

- [ ] Confirm the crates.io name and owner.
- [ ] Confirm repository, Cloudflare domain, and accountable owner.
- [ ] Enable strong multi-factor authentication and offline recovery.

### AP-1: Candidate Infrastructure

- [ ] Configure read-only workflow defaults and protected release environments.
- [ ] Configure the independent signed-tag and immutable-release rules.
- [ ] Confirm native runners and signing modes.
- [ ] Confirm private artifact retention through verification and recovery.

### AP-2: Just-In-Time Publication

- [ ] Configure crates.io trusted publishing or a temporary bootstrap token.
- [ ] Create the narrow Cloudflare write credential.
- [ ] Activate only the signing identities needed by the approved target set.
- [ ] Review exact candidate hashes before human approval.

### AP-3: Closure

- [ ] Verify crates.io and production-domain readback.
- [ ] Revoke any bootstrap token.
- [ ] Confirm no secret entered public or retained evidence.
- [ ] Rotate persistent credentials on their documented schedule.

## Official Starting Points

- [Cargo publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [crates.io trusted publishing](https://crates.io/docs/trusted-publishing)
- [GitHub deployment environments](https://docs.github.com/en/actions/concepts/workflows-and-actions/deployment-environments)
- [Cloudflare R2 API tokens](https://developers.cloudflare.com/r2/api/tokens/)
