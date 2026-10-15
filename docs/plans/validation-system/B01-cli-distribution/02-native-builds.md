# Phase B01.2: Build Native CLI Artifacts Once

[Back to the CLI distribution plan](README.md)

## Purpose

Produce exactly one runnable `liknon` executable for each supported Rust target
and make those final bytes the sole inputs to direct distribution.

## Dependencies

- [Phase B01.1](01-version-and-provenance.md) is complete.
- The [initial platform contract](README.md#initial-platform-contract) is
  approved.
- External readiness checkpoint [AP-2](accounts-and-permissions.md#ap-2) is
  complete.

## Build Contract

- Each Rust target has one unprivileged build job.
- Every job builds the `apps/liknon-cli` package and selects only its `liknon`
  binary target.
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
- `liknon --version`, representative help, and validation execution succeed.
- Binary and archive inventories contain no authored pack payload.

## Implementation Tasks

1. Add one release build job for each target in the platform contract.
2. Install only approved target and toolchain prerequisites.
3. Build once with the committed lockfile.
4. Apply approved deterministic post-processing.
5. Apply isolated Windows signing and macOS ad hoc signing.
6. Run native smoke and signature checks against final bytes.
7. Record target, filename, size, toolchain, commit, SHA-256, authenticity mode,
   and mode-specific evidence in the build manifest.
8. Upload one private workflow artifact per target with executable metadata
   preserved and bounded retention.

## Verification

- A later build invocation is mechanically prohibited or detected.
- Downloaded workflow artifacts match the internal manifest.
- Every advertised target has native execution evidence.
- GNU and MUSL records cannot cross-feed.
- Windows and macOS reject unsigned, incorrectly signed, or mutated artifacts.
- Windows verification pins the expected fingerprint without importing trust.
- macOS verification neither disables Gatekeeper nor clears quarantine.
- Pre-signature hashes are rejected as non-final evidence.

## Exit Criterion

CI retains exactly one runnable, final, accurately described executable and
hash for each supported target. Every later B01 phase consumes those exact
bytes without recompilation or mutation.
