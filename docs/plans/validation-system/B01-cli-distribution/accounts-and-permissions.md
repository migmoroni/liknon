# B01 External Accounts, Permissions, And Publication Authority

[Back to the CLI distribution plan](README.md)

## Purpose

Stage the human-owned account, permission, signing, and credential work needed
for CLI distribution. This is a just-in-time runbook, not an extra phase and not
a checklist to complete all at once.

## Publication Authority

- The human release owner controls crates.io, GitHub, and Cloudflare ownership,
  creates the signed tag, reviews evidence, and authorizes publication.
- GitHub Actions is the normal builder and publisher.
- The maintainer workstation is not a second release build path.
- Human approval occurs in a credential-free protected environment.
- Each signing or publication job receives only its own minimum identity.

## Canonical GitHub Identities

| Purpose | Canonical identity |
| --- | --- |
| Repository | `migmoroni/liknon` |
| crates.io package | `liknon-cli` |
| Release workflow | `.github/workflows/release-cli.yml` |
| Human approval | `cli-release-approval` |
| crates.io publication | `publish-crates-cli` |
| Detached release signing | `sign-cli-release` |
| Windows native signing | `sign-cli-windows` |
| GitHub Release publication | `publish-github-cli` |
| Cloudflare publication | `publish-cloudflare-cli` |

Build, assembly, and verification use `contents: read`. Only the isolated
GitHub publication job receives `contents: write`. Cloudflare uses a separate
bucket-scoped credential and does not require repository write access.

## Credential Placement

| Identity or credential | Location | Lifetime |
| --- | --- | --- |
| crates.io trusted-publishing identity or bootstrap token | `publish-crates-cli` environment | OIDC preferred; bootstrap token revoked after use |
| R2 access key and secret | `publish-cloudflare-cli` environment | Persistent, bucket-scoped, rotated |
| Detached-manifest signing key | `sign-cli-release` environment | Persistent, separately backed up and rotated |
| Windows PFX and password | `sign-cli-windows` environment | Persistent project identity, separately backed up |
| Human Git tag signing key | Maintainer-controlled local or hardware key | Never copied to publication jobs |
| Recovery codes | Offline maintainer storage | Never stored in GitHub or the repository |

Do not use repository-wide secrets when an isolated environment can hold the
credential. No secret may enter workflow artifacts or generated metadata.

## Phase Checkpoint Map

| B01 phase | Required checkpoint |
| --- | --- |
| B01.0 Public Contract | [AP-0](#ap-0) |
| B01.1 Version And Provenance | [AP-1](#ap-1) |
| B01.2 Native Builds | [AP-2](#ap-2) |
| B01.3 Cloudflare Direct | [AP-3](#ap-3) |
| B01.4 End-To-End Verification | [AP-4](#ap-4) |
| B01.5 Publication And Recovery | [AP-5](#ap-5) |
| B01.6 Public Verification | [AP-6](#ap-6) |

### AP-0

**Ownership inventory. Complete before B01.0.**

- [ ] Confirm the canonical GitHub repository owner.
- [ ] Confirm ownership of crates.io package `liknon-cli`.
- [ ] Confirm access to the Cloudflare account, DNS zone, and domain owner.
- [ ] Record accountable and recovery owners.
- [ ] Enable strong multi-factor authentication.
- [ ] Store recovery material outside GitHub and the repository.

No upload or signing credential is required yet.

### AP-1

**GitHub release identity. Complete before B01.1.**

- [ ] Enable Actions and confirm `.github/workflows/release-cli.yml` as the stable
      release workflow identity.
- [ ] Create the named protected environments without credentials.
- [ ] Set the default workflow token posture to read-only.
- [ ] Protect release tags and the workflow through applicable repository
      rules.
- [ ] Enable immutable GitHub Releases.
- [ ] Configure and test the human-owned signed-tag identity.

### AP-2

**Native signing readiness. Complete before B01.2.**

- [ ] Create the persistent project-controlled self-signed Windows identity.
- [ ] Record its public fingerprint and offline recovery copy.
- [ ] Store its PFX and password only in `sign-cli-windows`.
- [ ] Select and test the RFC 3161 timestamp service.
- [ ] Confirm macOS uses ad hoc signing and needs no Apple signing secret.
- [ ] Confirm native Windows and macOS runners are available.

### AP-3

**Cloudflare and detached-signing foundation. Complete before B01.3.**

- [ ] Activate R2 and any required billing profile.
- [ ] Create the production bucket and attach the chosen custom domain.
- [ ] Keep the public `r2.dev` endpoint disabled.
- [ ] Configure cache, TLS, immutable-prefix, bucket-lock, monitoring, and
      budget policies.
- [ ] Select the detached-signature mechanism and create its signing identity.
- [ ] Store private signing material only in `sign-cli-release` and offline
      recovery.
- [ ] Publish the trust root independently from R2.

The Cloudflare upload credential remains absent at this checkpoint.

### AP-4

**Credential-free candidate verification. Complete before B01.4.**

- [ ] Confirm private workflow artifact transport and retention.
- [ ] Prove build, assembly, and verification jobs cannot read signing or
      publication environments.
- [ ] Confirm source and local-origin installation tests need no public write
      permission.
- [ ] Confirm the crates.io candidate can be packaged and installed without a
      publication credential.
- [ ] Review every unresolved publication prerequisite.

### AP-5

**Just-in-time publication activation. Complete as B01.5 begins.**

- [ ] Create one R2 Object Read & Write credential scoped only to the production
      bucket and store it in `publish-cloudflare-cli`.
- [ ] Configure crates.io trusted publishing, or create a narrow temporary
      bootstrap token in `publish-crates-cli`.
- [ ] Verify `sign-cli-release` and `sign-cli-windows` contain only their own identities.
- [ ] Create and push the signed annotated tag only after pre-publication checks
      pass.
- [ ] Review retained candidate hashes and approve `cli-release-approval`.
- [ ] Confirm only `publish-github-cli` has repository write permission.

### AP-6

**Post-publication closure. Complete during B01.6.**

- [ ] Verify public ownership and recovery access remain current.
- [ ] Verify crates.io readback and revoke any bootstrap token.
- [ ] Confirm the R2 credential remains bucket-scoped and schedule rotation.
- [ ] Confirm no secret entered logs or evidence.
- [ ] Publish the stable pointer only after every public path passes.
- [ ] Preserve release evidence and expire private artifacts on schedule.

## Official References

- [GitHub deployment environments](https://docs.github.com/en/actions/concepts/workflows-and-actions/deployment-environments)
- [GitHub immutable releases](https://docs.github.com/en/code-security/concepts/supply-chain-security/immutable-releases)
- [crates.io trusted publishing](https://crates.io/docs/trusted-publishing)
- [Cloudflare R2 API tokens](https://developers.cloudflare.com/r2/api/tokens/)
- [Cloudflare R2 custom domains](https://developers.cloudflare.com/r2/buckets/public-buckets/)
