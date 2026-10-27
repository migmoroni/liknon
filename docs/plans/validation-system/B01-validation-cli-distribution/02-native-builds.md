# Phase B01.2: Build Native CLI Artifacts Once

## Status

`Provisional`. Blocked by B01.1.

[Back to the CLI distribution plan](README.md)

## Purpose

Implement the reusable credential-free native build lane, then produce one
final runnable Validation payload for each supported Rust target and expose it
under `liknon-validate` and `likv` in direct distribution.

## Dependencies

- [Phase B01.1](01-version-and-provenance.md) is complete.
- The [initial platform contract](README.md#initial-platform-contract) is
  approved.
- External readiness checkpoint [AP-2](accounts-and-permissions.md#ap-2) is
  complete.

## Build Contract

- Each Rust target has one unprivileged build job.
- The reusable job accepts only a validated product descriptor and an approved
  target row; it owns no product behavior or publication credential.
- The Validation caller builds the `apps/liknon-validate` package and selects
  only its `liknon-validate` binary target as the canonical native payload.
- The release Rust toolchain and locked dependency graph are recorded exactly.
- All deterministic post-processing occurs before platform signing.
- Windows signing runs in an isolated job with only its self-signed
  Authenticode identity and applies an RFC 3161 SHA-256 timestamp.
- macOS applies an ad hoc signature after every other transformation and does
  not notarize the artifact.
- Final hashes are computed only after all transformations and signatures.
- Timestamped or otherwise non-reproducible final bytes are retained exactly;
  they are never regenerated for an assembly or publication retry.
- Downstream jobs may copy but cannot rebuild or modify the executable.
- Build jobs have no GitHub publication, Cloudflare, or detached-signing
  credential.

## Platform Evidence

Before a target can feed a public archive, CI proves:

- GNU builds run on the declared minimum and current glibc environments;
- MUSL builds satisfy the approved linking contract and run on the supported
  Alpine/MUSL baseline;
- x64 and arm64 GNU/MUSL artifacts remain distinct;
- arm64 runs natively or through the approved trustworthy equivalent;
- macOS builds honor their deployment baseline, pass strict ad hoc signature
  and mutation checks, and claim neither notarization nor publisher identity;
- Windows has no undeclared runtime dependency and passes signature,
  fingerprint, timestamp, and mutation verification while retaining the
  expected absence of public certificate-chain trust;
- executable permission metadata survives private artifact transport;
- both command names report the same version and produce equivalent help and
  validation behavior;
- binary and archive inventories contain no knowledge or AI-resource payload.

## Implementation Tasks

1. Add one reusable release build job parameterized by a validated product
   descriptor and one matrix invocation per target in the platform contract.
2. Install only approved target and toolchain prerequisites.
3. Build the canonical native payload once with the committed lockfile.
4. Apply approved deterministic post-processing.
5. Apply isolated Windows signing and macOS ad hoc signing.
6. Run native smoke and signature checks against final bytes and both command
   names.
7. Record target, filename, size, toolchain, commit, SHA-256, authenticity mode,
   and mode-specific evidence in the build manifest.
8. Upload one private workflow artifact per target with executable metadata
   preserved and bounded retention.
9. Add descriptor fixtures proving product, package, binary, alias, target, and
   output namespaces cannot be crossed.

## Verification

- A later build invocation is mechanically prohibited or detected.
- Downloaded workflow artifacts match the internal manifest.
- Every advertised target has native execution evidence.
- GNU and MUSL records cannot cross-feed.
- Windows and macOS reject unsigned, incorrectly signed, or mutated artifacts.
- Windows verification pins the expected fingerprint without importing trust.
- macOS verification neither disables Gatekeeper nor clears quarantine.
- Pre-signature hashes are rejected as non-final evidence.
- Common build logic contains no Validation smoke behavior; the Validation
  profile supplies and owns its semantic smoke checks.

## Exit Criterion

CI retains exactly one runnable, final, accurately described Validation payload
and hash for each supported target. Every later B01 phase exposes those exact
bytes under both names without recompilation or mutation, and later product
plans can invoke the same build lane with independent descriptors.
