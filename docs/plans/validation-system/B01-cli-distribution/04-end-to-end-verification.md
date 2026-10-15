# Phase B01.4: Verify CLI Distribution End To End

[Back to the CLI distribution plan](README.md)

## Purpose

Prove before publication that clone-based source installation and every direct
native path carry the intended version, source identity, and executable
behavior.

## Dependencies

- [Phase B01.3](03-cloudflare-direct.md) is complete.
- Final candidate source identity, native artifacts, archives, installers, and
  manifests are retained in private workflow storage.
- External readiness checkpoint [AP-4](accounts-and-permissions.md#ap-4) is
  complete.

## Verification Layers

### Project Gates

- `cargo fmt --all -- --check`;
- `cargo clippy --all-targets --locked -- -D warnings`;
- `cargo test --all-targets --locked`;
- `cargo test --doc --locked`;
- the declared minimum Rust version where applicable.

### GitHub Source Installation

- Clone into an isolated environment with no access to the maintainer checkout.
- Fetch and verify the exact signed release tag.
- Build and install `apps/liknon-cli` with the committed lockfile and supported
  Rust baseline.
- Run `liknon --version`, `liknon --help`, schema output, and a representative
  validation fixture without activating any ecosystem pack.
- Update an older source-installed fixture through the documented procedure.
- Uninstall it and prove the isolated Cargo root no longer resolves `liknon`.

### Identity And Byte Equality

- Tag, commit, Cargo metadata, native CLI, build manifest, public manifest,
  archives, checksums, and installers agree on one version and source identity.
- Every extracted executable hash equals its exact build-manifest record.
- Cross-architecture and cross-libc mappings fail even if a binary happens to
  execute on the test host.
- Signed metadata and checksum projections are deterministic.
- Platform-authenticity claims remain accurate after archive extraction.

### Direct Installation

- Inspect archive contents, permissions, and absence of undeclared files.
- Exercise manual installation and authenticated POSIX/PowerShell installers
  against a local HTTP origin.
- Run representative success, failure, argument, current-directory,
  environment, stdin/stdout/stderr, and exit-code cases.
- Verify update and uninstall behavior using two internal fixture versions.
- Reject modified archives, forged metadata, interrupted downloads, malformed
  pointers, unsupported hosts, and unavailable verification tooling.
- Prove a failed update preserves the previous executable.

### Platform Matrix

- Test GNU and MUSL baselines independently on x64 and arm64.
- Exercise automatic and explicit Linux libc selection and verify request count.
- Execute macOS and Windows candidates on native runner families.
- Verify Windows self-signed signature evidence without adding trust.
- Verify macOS ad hoc signature evidence without weakening host policy.
- Preserve Unix executable modes through every transport layer.

### Repository Hygiene

- Generated outputs remain ignored.
- Assembly and verification modify no tracked source or template.
- No downstream job rebuilds or mutates Liknon.
- No test can read publication or signing credentials.
- CLI packages and archives contain no authored knowledge, skill, recipe, or
  ecosystem pack.

## Evidence Contract

Emit concise machine-readable evidence for every case containing:

- source commit and version;
- target and artifact hash;
- installation form;
- test identity and result;
- declared platform-authenticity and public-trust status;
- for Linux selection, detected implementations, decision source, selected
  target, and archive request count.

Evidence contains no credential or secret path.

## Implementation Tasks

1. Add credential-free release-candidate verification jobs.
2. Download and authenticate every retained private candidate.
3. Run source installation in an isolated clone.
4. Run direct archive and installer tests across the platform matrix.
5. Exercise update, failure preservation, and uninstall fixtures.
6. Emit and retain the reviewed evidence set.
7. Fail the joined phase when any advertised target lacks complete evidence.

## Exit Criterion

The source and direct-binary channels independently produce the same observable
Liknon CLI contract. Every supported target is authenticated, installable,
updatable, executable, and removable without publication credentials.
