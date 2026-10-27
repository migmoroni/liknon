# Phase B01.0: Confirm The CLI Distribution Contract

## Status

`Decision gate`. Run only after A01 has established the actual
package boundary and release candidate.

[Back to the CLI distribution plan](README.md)

## Purpose

Resolve every public identity, platform, trust, and installation decision that
later B01 phases must implement. It also approves the boundary between reusable
release mechanisms and the Validation product profile. This phase changes no
runtime behavior and publishes nothing.

## Dependencies

- [Plan A01](../A01-product-and-repository-separation/README.md) is complete.
- The [shared CLI distribution foundation](../cli-distribution-foundation.md)
  is the governing cross-product release contract.
- Current validation behavior is preserved by A01. Package name `liknon-validate`
  and command names `liknon-validate` and `likv` are approved target identities and must
  exist before this distribution contract exits.
- The repository is otherwise eligible to enter release preparation.
- External readiness checkpoint [AP-0](accounts-and-permissions.md#ap-0) is
  complete.

## Public Channels

### GitHub Source

The canonical source identity is the approved public repository. A source
release consists of:

- one signed annotated tag `liknon-validate-v<VERSION>`;
- the exact commit selected by that tag;
- the committed `Cargo.lock`;
- a metadata-only immutable GitHub Release that locks the tag and carries no
  Liknon binary assets.

The documented source path clones the repository, checks out and verifies the
exact tag, and installs the `apps/liknon-validate` package with locked dependencies.
The default branch is never presented as a reproducible released version.

### crates.io Source Package

The crates.io channel publishes source package `liknon-validate` with both
approved binary targets. The package builds from its own accepted contents,
the locked registry dependency graph selected by the installer, and no
unpublished workspace or path dependency. It does not require another Liknon
package. Every accepted package version is immutable.

### Cloudflare Direct

The direct channel uses a project-controlled custom domain backed by a private
R2 publication credential. It serves:

- one immutable archive per supported target;
- `manifest.json` and its detached signature;
- `SHA256SUMS` authenticated by the manifest;
- signed versioned POSIX and PowerShell installers;
- one separately mutable stable-channel pointer.

The website may link to these exact objects. It does not duplicate or transform
them.

## Version Contract

- The `apps/liknon-validate` Cargo package version is the sole authored version
  source for this application and is read through structured Cargo metadata.
- The initial grammar is canonical `MAJOR.MINOR.PATCH`.
- The tag is exactly `liknon-validate-v<VERSION>` and points to the workflow commit.
- `liknon-validate --version`, `likv --version`, crates.io metadata, internal
  build evidence, public direct metadata, archive names, and installer
  metadata carry that exact version.
- A previously published version is immutable.

## Platform And Linux Selection Contract

Confirm each row in the [initial platform contract](README.md#initial-platform-contract)
against an actual build and target-native execution path.

For Linux, approve deterministic GNU/glibc and MUSL capability probes. The
installer must:

1. honor an explicit `--libc=gnu|musl` selection;
2. otherwise choose GNU when both implementations are detected;
3. choose the sole detected implementation;
4. fail before download when neither is identifiable;
5. never infer libc from distribution name, container metadata, or a trial
   download.

Record the minimum glibc baseline, the MUSL support baseline, and the execution
method used to prove x64 and arm64 artifacts independently.

## Authenticity And Native Trust Contract

Release authenticity is provided by signed source identity, signed public
metadata, and exact hashes. Native platform signing is additional evidence and
must never be described more strongly than its real trust mode.

- Linux has no invented native signer.
- Windows initially uses one persistent project-controlled self-signed
  Authenticode identity and an RFC 3161 SHA-256 timestamp. Its fingerprint is
  public; no installer imports it into a trust store.
- macOS initially uses ad hoc signing and is not notarized. Documentation
  prefers Apple's user-mediated exception flow when needed and never disables
  Gatekeeper or removes quarantine automatically.
- The detached-manifest trust root is delivered independently from the R2
  origin and has explicit activation, rotation, retirement, and revocation
  semantics.

## Publication Authority

- The human release owner creates the signed tag, reviews evidence, and
  authorizes publication.
- GitHub Actions builds, verifies, and publishes exact retained artifacts.
- Build and pre-publication verification jobs have no destination credential.
- Human approval is a credential-free gate.
- crates.io publication, detached release signing, Windows signing, GitHub
  Release publication, and Cloudflare publication use separate identity
  boundaries.
- macOS ad hoc signing requires no credential.

## Decisions To Record

- structured Validation product-descriptor location and schema;
- exact common/product split for workflows, scripts, schemas, installers,
  fixtures, generated state, and protected environments;
- canonical repository and release-workflow identity;
- exact Cloudflare account, bucket, custom domain, and responsible owner;
- release version grammar and signed-tag identity;
- supported targets and minimum runtime baselines;
- archive formats and executable filenames;
- GitHub-source and crates.io installation and update commands;
- default and explicit direct-installation directories;
- libc probes and diagnostics;
- detached-signature algorithm, trust-root location, and key lifecycle;
- Windows certificate fingerprint and timestamp service;
- macOS deployment baseline and accurately disclosed ad hoc mode;
- artifact retention period required for recovery;
- immutable-path, cache, budget, and monitoring policies;
- exact CLI package and archive inventory proving that no knowledge corpus or
  AI resource enters the executable distribution.
- exact package inventory and registry dependency evidence for the
  self-contained `liknon-validate` candidate.

## Implementation Tasks

1. Record every product-specific decision above in one reviewed Validation
   descriptor and keep product-neutral constraints in shared schemas.
2. Prototype source installation from an exact local signed-tag fixture without
   access to repository-external files.
3. Package and install `liknon-validate` from an isolated crates.io candidate,
   proving that its extracted contents require no unpublished workspace file
   or package.
4. Build and execute each proposed native target through its real runtime path.
5. Prototype Linux selector probes for GNU-only, MUSL-only, dual-libc, explicit,
   and unknown environments.
6. Prototype Windows and macOS signing and mutation verification without
   changing user trust settings.
7. Prototype detached-manifest verification and key lifecycle fixtures.
8. Confirm Cloudflare immutable-key, custom-domain, cache, and bucket-lock
   capabilities against the selected account design.
9. Confirm that every required credential can be isolated as defined by the
   [runbook](accounts-and-permissions.md).
10. Prove with fixture descriptors that common release code rejects an unknown
    product, missing authority, cross-product namespace, unsafe path, and
    product/target mismatch.

## Verification

- Source installation uses only the selected tag and committed files.
- The packaged crate builds in isolation and installs both command names.
- Both command names are behaviorally equivalent.
- Every target executes and reports the expected version.
- Linux selection requests exactly one matching archive in every successful
  case and none in an unknown case.
- A modified manifest, installer, archive, or binary fails verification.
- Windows verification detects mutation and records self-signed status rather
  than public chain trust.
- macOS verification detects mutation and records ad hoc, non-notarized status.
- No installer or test alters trust stores, Gatekeeper, SmartScreen, quarantine,
  or another host-security policy.
- The GitHub Release design cannot upload native binary assets.
- Versioned Cloudflare objects cannot be overwritten by the normal release
  identity.
- An accepted crates.io version cannot be replaced.
- Common release code contains no hard-coded Validation behavior and cannot
  select another product's credential, namespace, candidate, or stable pointer.

## Exit Criterion

The three public channels, command aliases, complete platform matrix, release
identity, signing model, registry and Cloudflare boundaries, and human/CI
authority are explicit and empirically feasible. The reusable foundation and
Validation profile have a reviewed, testable boundary.
