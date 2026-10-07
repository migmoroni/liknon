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

## Build Contract

- Each Rust target has one unprivileged build job.
- The release Rust toolchain is pinned and its exact resolved version is
  recorded.
- Stripping occurs only where deterministic and appropriate for the target.
- Final native bytes are hashed after every transformation.
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
- the Windows MSVC binary has no undeclared runtime dependency;
- executable permission metadata survives artifact transport;
- the binary reports the Cargo-derived version;
- representative `--help` and validation execution work on the target.

## Implementation Tasks

1. Add the target build matrix to the release workflow.
2. Install only approved target and toolchain prerequisites in each job.
3. Build the locked release binary once per target.
4. Apply any approved deterministic post-processing.
5. Run native smoke tests before upload.
6. Compute SHA-256 and record target, filename, size, toolchain, and source
   commit in the build manifest.
7. Upload one private workflow artifact per target with executable metadata
   preserved.
8. Set a bounded retention window sufficient for release recovery.

## Verification

- A second build invocation in distribution jobs is mechanically prohibited or
  detected.
- Downloaded workflow artifacts match the build manifest.
- Native smoke tests run on every supported target.
- Linux compatibility runs in both glibc and Alpine environments.
- macOS and Windows tests use their native runner families.
- The release matrix fails if any target cannot provide complete evidence.

## Exit Criterion

CI has exactly one runnable, hashed binary per supported target, and every later
phase can consume those artifacts without recompiling Liknon.
