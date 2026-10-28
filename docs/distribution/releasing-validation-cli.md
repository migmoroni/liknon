# Releasing The Validation CLI

The product orchestrator is
`.github/workflows/release-validation.yml`. It consumes the reviewed descriptor
at `distribution/cli/validation/product.json` and the product-neutral machinery
under `distribution/cli/common`.

The current cross-platform matrices remain active during development. A later,
explicit cost-driven reduction to Linux x86-64 is specified in
[`future-linux-x86-64-build-scope.md`](future-linux-x86-64-build-scope.md); that
document is not an instruction to reduce the current matrices automatically.

## Preconditions

Complete the matching AP-0 through AP-5 checkpoints in
[`accounts-and-permissions.md`](../plans/validation-system/B01-validation-cli-distribution/accounts-and-permissions.md).
In particular, do not create a release tag before protected environments,
public trust roots, Windows signing evidence, immutable R2 policy, and recovery
ownership are reviewed.

The workflow requires:

- exactly one tag trust file in `distribution/cli/validation/direct`:
  `tag-signing-public-key.asc` or `tag-signing-allowed-signers`;
- the production detached-signature trust root as `public-key.pem`;
- environment-scoped credentials and public variables listed in the B01
  external-readiness runbook.

crates.io publication prefers its configured GitHub OIDC trusted publisher and
uses the resulting job-scoped token. Because crates.io requires an initial
accepted package before that publisher can be configured, the first release may
use the narrow `CARGO_REGISTRY_TOKEN` bootstrap secret. Revoke that token after
configuring the workflow/environment identity recorded in the runbook.

## Release Sequence

1. Update only `apps/liknon-validator/Cargo.toml` for the intended application
   version and commit a clean release candidate.
2. Run normal CI and review package boundaries.
3. Create and push the signed annotated tag
   `liknon-validator-v<VERSION>` at that exact commit.
4. Dispatch `Release Validation CLI` with that tag.
5. Review retained source, native, direct, and attestation evidence at the
   credential-free approval gate.
6. Allow the isolated crates.io, GitHub, and Cloudflare preflights to prove
   destination availability, authority, and immutable-object compatibility.
7. Approve publication only when every target, both aliases, and all three
   destination preflights have passed.
8. Confirm crates.io, the metadata-only GitHub Release, and immutable R2
   readback.
9. Allow public verification to complete before the stable pointer advances.

The workflow is intentionally non-cancellable for a given release identity.
Normal CI never receives publication credentials.

## Detached-Signing Key Lifecycle

Each released version selects exactly one detached-signing key ID in
`distribution/cli/validation/product.json` and one matching public key in the
verified source tag. The release workflow proves that the protected private key
derives that exact public key before signing any candidate.

- **Activation:** commit the independently reviewed public key and a previously
  unused key ID before creating the signed release tag. Provision its private
  half only in `sign-cli-release` and offline recovery storage.
- **Rotation:** activate a new key ID and public key for a new release. Never
  reuse an ID for different key bytes or resign an existing version. Historical
  immutable releases retain the trust root selected by their exact source tag.
- **Retirement:** remove the private key from online signing after its final
  release. Retired keys cannot sign a new candidate because the current
  descriptor, public key, and protected private key must agree.
- **Revocation:** immediately stop the affected workflow environment, keep the
  affected version out of the stable pointer, publish a security advisory, and
  release a new patch under a new key ID. Immutable historical bytes are not
  rewritten. A versioned installer embeds its release trust root, so revocation
  guidance must identify affected versions explicitly; it is not represented
  as retroactive cryptographic invalidation.

Candidate verification rejects a different public key or a manifest key ID
that does not exactly match the product descriptor. Production activation,
rotation, retirement, or revocation also requires the corresponding AP-3/AP-5
operator review.

## Recovery

Retry the same workflow and tag only while its private candidates remain within
retention. R2 publication compares existing immutable bytes before skipping
them; a conflicting object fails closed. crates.io readback must equal the
retained `.crate`, and the GitHub Release must contain no assets.

Never rebuild, resign, overwrite, or retag an incomplete version. If exact
retained bytes cannot be recovered, leave that version incomplete, revoke or
retire compromised signing material as appropriate, and prepare a new patch
release. The stable pointer remains on the previous fully verified version.
