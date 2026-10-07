# Phase B01.2: Build Native Artifacts Once

[Back to the distribution plan](README.md)

## Purpose

Produce exactly one runnable native binary for each supported Rust target and
make those bytes the only native inputs accepted by all later distribution
phases.

## Dependencies

- [Phase B01.1](01-version-and-provenance.md) is complete.
- The [initial platform contract](README.md#initial-platform-contract) is
  approved.
- The Windows self-signed identity, protected-key boundary, macOS ad hoc signing
  procedure, and verification contracts from Phase B01.0 are available.

## Build Contract

- Each Rust target has one unprivileged build job.
- The release Rust toolchain is pinned and its exact resolved version is
  recorded.
- Stripping occurs only where deterministic and appropriate for the target and
  precedes platform signing.
- A dedicated Windows signing job Authenticode-signs each Windows binary with
  the persistent project-controlled self-signed identity and applies an RFC 3161
  SHA-256 timestamp. It receives only that target-specific identity and no
  detached-release-signing or registry credential.
- A macOS job applies an ad hoc signature after all other transformations. It
  uses no signing credential and performs no notarization.
- Final native bytes are hashed only after every transformation and declared
  platform-signing step. Nothing subsequently modifies them.
- Timestamped or otherwise non-reproducible signing output is retained exactly
  and never regenerated to retry assembly or publication.
- Distribution assembly may copy those bytes but may not invoke a Liknon build.
- Build jobs possess no destination credentials.
- Artifact names encode target identity without becoming a second version
  source.

## Platform Evidence

Before a target may feed a public package, CI must prove:

- Linux GNU binaries are built against the approved minimum glibc baseline and
  run on that baseline and a current glibc distribution;
- Linux MUSL binaries satisfy the approved static-linking contract and run on
  the supported Alpine/MUSL baseline;
- x64 and arm64 each produce distinct GNU and MUSL artifacts, and every Linux
  binary feeds only package variants carrying its own architecture and libc;
- every native binary runs through the approved direct or managed .NET Tool
  entry point on its declared RID;
- the arm64 Linux build runs on native arm64 or a trustworthy equivalent;
- macOS builds honor and test the declared deployment baseline;
- macOS final artifacts pass strict ad hoc signature and mutation checks, report
  no publisher identity or notarization, and execute on clean native runners;
- the Windows MSVC binary has no undeclared runtime dependency;
- Windows final artifacts pass Authenticode signature, expected self-signed
  certificate fingerprint, RFC 3161 timestamp, and mutation verification while
  retaining the expected absence of public certificate-chain trust;
- executable permission metadata survives artifact transport;
- the binary reports the Cargo-derived version;
- representative `--help` and validation execution work on the target.

In artifact and package names, GNU means the glibc-linked target. Build jobs do
not perform host-selection logic: they produce and prove all four Linux target
artifacts independently so later selectors and ecosystem resolvers can choose
without rebuilding or crossing libc identities.

## Implementation Tasks

1. Add the target build matrix to the release workflow, including separate
   `x86_64-unknown-linux-gnu`, `x86_64-unknown-linux-musl`,
   `aarch64-unknown-linux-gnu`, and `aarch64-unknown-linux-musl` jobs.
2. Install only approved target and toolchain prerequisites in each job.
3. Build the locked release binary once per target.
4. Apply approved deterministic post-processing before signing.
5. Pass Windows candidates through the isolated self-signed Authenticode and RFC
   3161 timestamping job. Apply macOS ad hoc signatures without a credential or
   notarization step.
6. Run native smoke tests and platform-signature verification against the final
   bytes before upload.
7. Compute SHA-256 and record target, filename, size, toolchain, source commit,
   exact platform-authenticity mode, and mode-specific evidence in the build
   manifest.
8. Upload one private workflow artifact per target with executable metadata
   preserved.
9. Set a bounded retention window sufficient for release recovery.

## Verification

- A second build invocation in distribution jobs is mechanically prohibited or
  detected.
- Downloaded workflow artifacts match the build manifest.
- Native smoke tests run on every supported target.
- Every Linux GNU target runs in its declared minimum and current glibc
  environments; every Linux MUSL target runs in its declared Alpine/MUSL
  environment.
- GNU and MUSL outputs for the same architecture have distinct manifest records
  and no downstream consumer is assigned to both.
- macOS and Windows tests use their native runner families.
- macOS and Windows jobs reject unsigned, incorrectly signed, or mutated
  artifacts. macOS additionally rejects any manifest claim that its initial ad
  hoc artifact is notarized or has a publisher identity.
- Windows jobs reject an absent, invalid, non-RFC-3161, or non-SHA-256 timestamp
  before final hashing.
- Windows verification pins the expected self-signed certificate fingerprint
  and does not require or simulate public chain trust. Clean installation tests
  do not import the certificate into the user's trust stores.
- macOS verification uses the native code-signing tools and actual execution; it
  does not disable Gatekeeper, remove quarantine metadata, or simulate Developer
  ID acceptance.
- Hashes recorded before platform signing are rejected as non-final evidence.
- The release matrix fails if any target cannot provide complete evidence.

## Exit Criterion

CI has exactly one runnable, final platform-sealed binary under the declared
mode and one hash per supported target. Windows is self-signed and timestamped;
macOS is ad hoc signed and explicitly not notarized; Linux relies on signed
release metadata rather than an invented native signer. Every later phase
consumes those exact artifacts without recompiling or modifying Liknon.
