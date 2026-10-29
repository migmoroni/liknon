# Plan F01: Rust 1.90 MSRV Modernization

## Status

`Future`. Execute this plan only after the current roadmap has reached a
terminal state and B06 has closed the coordinated first public release. A plan
may count as terminal when it is complete, explicitly deferred, or explicitly
excluded from the approved release set.

This plan does not change the current Rust 1.87 contract. Until F01 is
activated and completed, manifests, release descriptors, documentation, and CI
must continue to declare and test Rust 1.87.

[Back to the validation-system roadmap](../README.md)

## Purpose

Raise the Liknon workspace minimum supported Rust version from 1.87 to 1.90 as
one reviewed compatibility change, then use the newly available language,
standard-library, Cargo, and compiler behavior where it produces a concrete
improvement.

The objective is not to replace old syntax merely because newer syntax exists.
Every modernization must do at least one of the following:

- remove a dependency or custom compatibility layer;
- improve correctness or error classification;
- simplify code without hiding domain behavior;
- strengthen target, package, or release evidence;
- measurably improve build or release operation;
- make a maintained contract more explicit.

Rust 1.90 is the fixed target of this plan. Do not silently replace it with the
then-current stable toolchain while implementing F01. A later MSRV increase is
a separate compatibility decision.

## Activation Gate

Before changing a manifest or workflow, approve one F01 activation record that
contains:

- the exact commit after the current roadmap and B06 closure;
- every workspace package and separately published Rust package in scope;
- the public versions that currently promise Rust 1.87 compatibility;
- the release version or versions that will first require Rust 1.90;
- the supported native and cross-compilation targets at that time;
- all current MSRV declarations, generated examples, test fixtures, release
  descriptors, workflow pins, and user-facing claims;
- the accepted modernization ledger from this plan;
- an explicit decision on Rust 2024 edition adoption;
- an explicit decision on Cargo workspace publication adoption;
- rollback ownership before and after public publication.

The activation inventory must be regenerated from the repository at that time.
The current inventory below is a seed, not a frozen prediction of code that
will exist after B03-B06 and E01.

## Official Basis

Review the complete Rust, Cargo, and Clippy release notes for every version
between the old and new MSRV. The initial source set is:

- [Rust 1.88.0](https://blog.rust-lang.org/2025/06/26/Rust-1.88.0/): let
  chains under Rust 2024, boolean `cfg`, Cargo cache collection, collection
  extraction APIs, and slice chunk APIs;
- [Rust 1.89.0](https://blog.rust-lang.org/2025/08/07/Rust-1.89.0/): standard
  file locking, typed lock contention, cross-compiled doctests, the
  `mismatched_lifetime_syntaxes` lint, and `Result::flatten`;
- [Rust 1.90.0](https://blog.rust-lang.org/2025/09/18/Rust-1.90.0/): LLD as
  the default `x86_64-unknown-linux-gnu` linker, native Cargo workspace
  publication, and the `x86_64-apple-darwin` support-tier change.

Do not treat the announcement summaries as an exhaustive implementation
inventory. At activation, inspect their linked detailed release notes and the
effective Cargo and Clippy notes as well.

## Target Contract

After F01 completes:

- every maintained Rust package declares `rust-version = "1.90"`, directly or
  through the workspace package contract;
- every release and MSRV workflow uses the exact `1.90.0` toolchain where an
  exact toolchain is required;
- active documentation and generated examples say Rust 1.90 or `>=1.90`;
- current stable Rust remains a separate CI lane rather than replacing the
  declared MSRV lane;
- package candidates and installation rehearsals are built and tested with
  Rust 1.90;
- no maintained package accidentally retains a Rust 1.87 claim;
- historical plans and retained release evidence keep their historical
  toolchain values;
- every accepted modernization has focused behavioral evidence;
- every rejected candidate records why it did not improve the current code.

An MSRV increase is a consumer compatibility change. It must be reflected in
the applicable package versions, changelogs, release notes, and public
installation documentation before publication.

## Current Impact Seed

The activation inventory must at least revisit these current surfaces:

| Surface | Current Rust 1.87 ownership | Required F01 treatment |
| --- | --- | --- |
| Workspace manifest | `Cargo.toml` workspace `rust-version` | Change once to `1.90` after the baseline gate |
| Lockfile | Dependency closure includes `fs2` | Regenerate only through reviewed dependency changes |
| CI | Stable and exact Rust 1.87 lanes, including Windows cross-checks | Replace the MSRV lane with exact 1.90 while retaining current-stable evidence |
| Release workflows | Product and supporting-crate toolchain pins | Change all active exact pins atomically |
| Product descriptors | `releaseRustToolchain` fields | Change active descriptors to `1.90.0` and revalidate generated contracts |
| Documentation | Root, product, and distribution installation requirements | Update active requirements and generated references |
| Schemas and examples | Tool-version requirements used in fixtures and examples | Update only examples that describe the current Liknon contract |
| Distribution rehearsals | Source-package and native-candidate checks | Execute with 1.90 and preserve isolated-source closure |
| Current-state record | Describes the implemented MSRV | Update only after the code and gates prove the new state |

Use a repository-wide hidden-file search at activation, for example:

```sh
rg --hidden -n \
  '1\.87\.0|>=1\.87|Rust 1\.87|rust-version = "1\.87"' \
  --glob '!.git/**' --glob '!target/**' .
```

Classify every result as active contract, generated artifact, test fixture,
historical evidence, or unrelated prose. Do not mechanically rewrite
historical release records or completed-plan evidence.

## Modernization Ledger

This ledger is intentionally maintainable before activation. New candidates
discovered while executing the current plans may be appended with their source
version, concrete consumer, expected benefit, and required evidence. A release
feature without a concrete Liknon consumer remains rejected or deferred.

| Candidate | Minimum Rust | Initial decision | Required work and evidence |
| --- | ---: | --- | --- |
| Replace `fs2` with `std::fs::File` locking | 1.89 | Accept | Remove the direct dependency; use `File::try_lock` and `File::unlock`; distinguish `TryLockError::WouldBlock` from `TryLockError::Error`; preserve process exclusion and recovery tests on supported platforms |
| `mismatched_lifetime_syntaxes` default lint | 1.89 | Accept | Run all targets with warnings denied and make hidden returned lifetimes explicit where reported; do not suppress the lint workspace-wide |
| Cross-target doctest execution | 1.89 | Accept where a native runner exists | Add native-platform or configured-runner doctest evidence; use target-specific ignores only for genuinely inapplicable examples |
| LLD default on `x86_64-unknown-linux-gnu` | 1.90 | Accept as default, subject to release evidence | Build, install, and smoke-test every selected Linux candidate; record the toolchain/linker policy; add a BFD opt-out only for a demonstrated defect |
| Cargo workspace publication | 1.90 | Evaluate at F01.0 | Compare `cargo publish --workspace --dry-run` with the established independent publication authorities, isolated-source rehearsals, approvals, and readback; adopt only where it preserves product ownership and exact ordering |
| `Result::flatten` | 1.89 | Candidate | Replace manual nested-result flattening only where an actual occurrence becomes clearer and keeps error context |
| `HashMap::extract_if` and `HashSet::extract_if` | 1.88 | Candidate | Adopt only for an existing filter-and-remove operation with simpler ownership and equivalent ordering semantics |
| Slice `as_chunks` and `as_rchunks` APIs | 1.88 | Candidate | Adopt only where fixed-width parsing becomes safer or removes manual bounds arithmetic |
| Boolean `cfg(true)` and `cfg(false)` | 1.88 | Candidate | Use only if maintained manifests or generated configuration still express unconditional predicates indirectly |
| Let chains | 1.88 plus Rust 2024 edition | Deferred to edition decision | Do not use under the current Rust 2021 edition; consider only as part of one explicit edition migration with `cargo fix --edition` and full behavior review |
| Cargo automatic cache collection | 1.88 | Observe, no product change | Ensure CI and release rehearsals never depend on indefinite Cargo-cache retention; do not add configuration without a reproduced problem |
| `x86_64-apple-darwin` Tier 2 status | 1.90 | Support-policy review | Record the effective target matrix and hosted evidence; do not silently remove or promise support because the upstream tier changed |

The ledger must not become a checklist that forces every new API into the
codebase. `Rejected` is a valid final decision when the existing code is
clearer or the benefit is speculative.

## Implementation Units

| Unit | Scope | Exit result |
| --- | --- | --- |
| F01.0 | Freeze the post-roadmap inventory, release boundary, target matrix, edition decision, and accepted modernization ledger | One approved transition contract with no moving toolchain target |
| F01.1 | Establish the Rust 1.87 baseline and test Rust 1.90 without changing public declarations | Comparative evidence identifies compiler, lint, doctest, linker, package, and platform effects |
| F01.2 | Change manifests, workflows, descriptors, fixtures, examples, and active documentation atomically to Rust 1.90 | One internally consistent MSRV contract across source and release automation |
| F01.3 | Apply accepted standard-library and language modernizations, beginning with native file locking | Dependencies and code are simplified without behavior regression |
| F01.4 | Evaluate Cargo workspace publication and validate LLD/native distribution behavior | Release mechanisms use 1.90 deliberately while preserving independent authority and public contracts |
| F01.5 | Run full conformance, package inspection, hosted target jobs, release rehearsals, documentation review, and public readback for the selected release | Published versions and retained evidence prove the Rust 1.90 requirement |

## F01.1 Baseline And Comparative Run

Before editing the MSRV declaration:

1. run the complete repository gate on exact Rust 1.87 and current stable;
2. install Rust 1.90.0 without making it the repository default;
3. run formatting, checks, Clippy with warnings denied, tests, doctests, strict
   Rustdoc, package creation, source-package rehearsals, and selected native
   builds with Rust 1.90;
4. capture every new compiler, Cargo, Clippy, linker, doctest, and platform
   difference in the activation record;
5. separate required compatibility fixes from optional modernization;
6. prove that generated schemas and machine-readable outputs do not change
   unless a separately approved product contract requires it.

This comparative run prevents the declaration change from hiding unrelated
breakage.

## F01.2 Contract Cutover

Apply the MSRV update as one coherent change:

- update workspace and separately owned package manifests;
- update exact toolchain pins in CI and release workflows;
- update active product descriptors and their tests;
- update current examples and machine-readable tool requirements;
- update installation documentation and package metadata;
- regenerate `Cargo.lock` only when dependency changes require it;
- update the current-state assessment after the implementation passes;
- add a changelog entry that describes the consumer requirement and principal
  benefits.

Do not preserve a Rust 1.87 compatibility lane, fallback implementation, or
dual lock backend. The completed state supports one declared minimum: Rust
1.90.

## F01.3 Native File Locking

The Manager currently requires a cross-process exclusive lock before recovery
or workspace mutation. Keep that safety property while removing `fs2`.

The new implementation must:

- open the lock file with the access required by all supported platforms;
- use `std::fs::File::try_lock` for non-blocking exclusive acquisition;
- map `TryLockError::WouldBlock` to the existing blocked-process outcome;
- preserve the underlying I/O error from `TryLockError::Error` with the lock
  path and correct Manager error category;
- release the lock deterministically on the successful path and by file drop on
  every early-return path;
- prove that recovery never begins when another Manager process owns the lock;
- test contention and non-contention I/O failure separately;
- run the behavior on every supported native CI platform;
- remove `fs2` from manifests and verify it is absent from the final dependency
  tree.

Do not replace this with a sentinel-file existence check. Creation and cleanup
of a marker file do not provide equivalent process-crash semantics.

## F01.4 Cargo And Distribution Review

### LLD

Rust 1.90 selects LLD by default for `x86_64-unknown-linux-gnu`. Treat the
resulting candidate bytes as a new release input:

- rebuild archives, checksums, signatures, manifests, and provenance from the
  same retained candidate;
- run installation and semantic smoke tests against the retained bytes;
- compare size and build-time evidence when available, without making a
  performance claim from one run;
- keep the default unless a reproducible linker defect requires a documented
  opt-out.

### Workspace publication

`cargo publish --workspace` can order workspace packages but is not an atomic
registry transaction. Before adopting it:

- run `cargo publish --workspace --dry-run` against the selected package set;
- verify treatment of unpublished supporting crates and independently
  versioned products;
- preserve protected environments, approvals, exact candidate hashes,
  dependency-order readback, and failure recovery;
- retain product-specific publication authority;
- keep the existing explicit process if the workspace command weakens any of
  those contracts.

The existence of a shorter Cargo command is not sufficient reason to replace a
reviewed release protocol.

## Validation Matrix

The exact commands may evolve before activation, but F01 completion requires
equivalent evidence for all packages selected at F01.0:

```sh
cargo +1.90.0 fmt --all -- --check
cargo +1.90.0 check --workspace --all-targets --locked
cargo +1.90.0 clippy --workspace --all-targets --locked -- -D warnings
cargo +1.90.0 test --workspace --all-targets --locked
cargo +1.90.0 test --workspace --doc --locked
RUSTDOCFLAGS='-D warnings -D missing-docs' \
  cargo +1.90.0 doc --workspace --no-deps --locked
```

Also require:

- current-stable formatting, Clippy, tests, doctests, and strict Rustdoc;
- supported native-platform tests and packaging;
- cross-target checks and doctests where an appropriate runner exists;
- ignored outcome fixtures and security/integrity scenarios;
- dependency policy, advisory, license, and package-inventory checks;
- source-package closure from isolated registry-style sources;
- install-and-run rehearsals for every selected CLI package;
- native archive, installer, manifest, checksum, signature, and provenance
  verification;
- clean-client readback after publication;
- a final hidden-file search proving that active Rust 1.87 claims are gone.

## Release And Recovery

Before public activation, consumers must be able to see the new minimum in
package metadata and installation documentation. Do not publish a package whose
manifest still claims Rust 1.87 while its code requires 1.90.

Before publication, rollback means restoring the last reviewed revision and
rerunning its complete gate. After publication, registry artifacts are
immutable: recover through a new correctly versioned release rather than
overwriting bytes, weakening metadata, or introducing a hidden compatibility
backend.

## Acceptance Criteria

- [ ] F01.0 records the exact post-roadmap scope, release versions, targets,
  edition decision, and accepted modernization ledger.
- [ ] The Rust 1.87 baseline and Rust 1.90 comparative run are retained.
- [ ] Every active package, workflow, descriptor, test, example, and current
  document agrees on Rust 1.90.
- [ ] Historical evidence remains historically accurate.
- [ ] `fs2` is absent and Manager lock contention is distinguished from lock
  I/O failure.
- [ ] Every adopted 1.88-1.90 feature has a concrete consumer and focused test.
- [ ] Every rejected candidate has a short rationale.
- [ ] LLD-built Linux candidates pass package, install, smoke, integrity, and
  provenance verification.
- [ ] Cargo workspace publication is either proven compatible with Liknon's
  release authorities or explicitly rejected.
- [ ] Exact Rust 1.90, current stable, package, platform, and hosted release
  gates pass.
- [ ] The selected public release is readable from clean clients and reports
  the Rust 1.90 requirement accurately.

