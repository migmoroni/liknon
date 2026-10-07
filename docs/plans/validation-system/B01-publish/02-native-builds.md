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
- Windows and macOS signing identities and verification procedures from Phase
  B01.0 are available through protected signing environments.

## Build Contract

- Each Rust target has one unprivileged build job.
- The release Rust toolchain is pinned and its exact resolved version is
  recorded.
- Stripping occurs only where deterministic and appropriate for the target and
  precedes platform signing.
- Dedicated signing jobs Authenticode-sign Windows binaries, Developer ID-sign
  macOS binaries, and submit the approved macOS notarization envelope. They
  receive only the target-specific signing identity and no registry credential.
- Final native bytes are hashed only after every transformation, signature, and
  approved notarization step. Nothing subsequently modifies them.
- Timestamped or otherwise non-reproducible signing output is retained exactly
  and never regenerated to retry assembly or publication.
- Distribution assembly may copy those bytes but may not invoke a Liknon build.
- Build jobs possess no destination credentials.
- Artifact names encode target identity without becoming a second version
  source.

## Platform Evidence

Before a target may feed a public package, CI must prove:

- Linux musl binaries are static and run on glibc-based Linux and Alpine;
- each Linux binary suits every declared GNU/musl RubyGems and PyPI platform
  variant;
- every native binary runs through the approved direct or managed .NET Tool
  entry point on its declared RID;
- the arm64 Linux build runs on native arm64 or a trustworthy equivalent;
- macOS builds honor and test the declared deployment baseline;
- macOS final artifacts pass Developer ID signature and notarization checks;
- the Windows MSVC binary has no undeclared runtime dependency;
- Windows final artifacts pass Authenticode verification;
- executable permission metadata survives artifact transport;
- the binary reports the Cargo-derived version;
- representative `--help` and validation execution work on the target.

## Implementation Tasks

1. Add the target build matrix to the release workflow.
2. Install only approved target and toolchain prerequisites in each job.
3. Build the locked release binary once per target.
4. Apply approved deterministic post-processing before signing.
5. Pass Windows and macOS candidates through isolated target-specific signing
   and notarization jobs.
6. Run native smoke tests and platform-signature verification against the final
   bytes before upload.
7. Compute SHA-256 and record target, filename, size, toolchain, source commit,
   and platform-signing evidence in the build manifest.
8. Upload one private workflow artifact per target with executable metadata
   preserved.
9. Set a bounded retention window sufficient for release recovery.

## Verification

- A second build invocation in distribution jobs is mechanically prohibited or
  detected.
- Downloaded workflow artifacts match the build manifest.
- Native smoke tests run on every supported target.
- Linux compatibility runs in both glibc and Alpine environments.
- macOS and Windows tests use their native runner families.
- macOS and Windows jobs reject unsigned, incorrectly signed, mutated, or
  unnotarized artifacts as applicable.
- Hashes recorded before platform signing are rejected as non-final evidence.
- The release matrix fails if any target cannot provide complete evidence.

## Exit Criterion

CI has exactly one runnable, final signed binary where required and one hashed
binary per supported target. Every later phase consumes those exact artifacts
without recompiling or modifying Liknon.
