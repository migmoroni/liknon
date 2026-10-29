# Shared CLI Distribution Foundation

## Status

`Approved cross-product contract`. B01 is the first production implementation
and Validation is its first product profile. This does not make B01 the owner
of Manager, Tool, launcher, or complete-suite publication.

This document is normative for B01-B04. Product plans own product behavior,
release readiness, and publication. This contract owns only the release
mechanisms they deliberately share.

Runtime terminal behavior is outside this document. Palettes, presentation,
help styling, `NO_COLOR`, and shared accessibility primitives belong to
[A02 Shared CLI Experience Foundation](A02-shared-cli-experience-foundation/README.md).
Sharing runtime experience does not merge the release identities or
publication authorities defined here.

## Purpose

Provide one maintained release foundation for four native delivery subjects:

| Subject | Standalone owner | Native responsibility |
| --- | --- | --- |
| Validation | B01 | Build and publish `liknon-validator` / `likv` |
| Manager | B02 | Build and publish `liknon-manager` / `likm` |
| Tool | B03 | Build and publish `liknon-tool` / `likt` |
| Launcher and complete suite | B04 | Build `liknon` / `likx` and compose exact B01-B03 candidates |

Sharing the machinery must not merge release trains, credentials, artifacts,
versions, tags, stable pointers, readiness gates, or publication authority.

## Shared Mechanisms

The repository should implement product-neutral primitives for:

- structured product and target descriptors;
- Cargo package, version, binary-target, tag, and commit inspection;
- credential-free native builds and one-final-payload-per-target enforcement;
- platform sealing and verification adapters;
- internal build manifests and public direct-distribution manifests;
- deterministic archive assembly, checksums, and detached signatures;
- POSIX and PowerShell installer generation;
- operating-system, architecture, and Linux libc selection;
- private candidate transport and bounded retention;
- credential-free source, native, installer, and public-readback verification;
- isolated publication, resumable state, and recovery evidence.

The implementation may use reusable GitHub workflows, repository scripts, or
both. Product workflows remain thin, explicit orchestrators. They must not copy
the shared logic into four independently maintained release pipelines.

## Product Descriptor

Every standalone product and the launcher suite provide reviewed structured
inputs at least for:

- stable product ID and owning plan;
- Cargo package name and path;
- canonical executable and installed aliases;
- for an aggregate suite, the ordered component slots and accepted component
  identity/compatibility contract;
- version source and signed-tag prefix;
- supported target rows and runtime baselines;
- archive and Cloudflare storage boundary and object prefixes;
- required license and notice inventory;
- product-specific smoke and semantic tests;
- crates.io, GitHub, signing, Cloudflare, and approval identities;
- stable-pointer identity and recovery owner.

Shared code rejects missing, unknown, conflicting, or unsafe values. It never
infers publication authority from a package name and never receives arbitrary
commands from public metadata.

A standalone product descriptor resolves to one canonical implementation
payload per target plus exactly one compact command alias in the current
foundation contract. Widening that cardinality requires a coordinated schema,
installer, and conformance change. The B04 suite descriptor resolves to one
launcher payload plus authenticated component payload records. Shared schemas
represent both forms explicitly; they do not flatten a suite into one binary
or silently substitute a missing component.

## Version And Provenance Contract

Every release subject has one independently owned release identity:

```text
(product, package, version, tag, commit)
```

The product descriptor identifies the exact Cargo package and path that own the
version plus the signed-tag prefix. Release automation selects the package and
version through structured Cargo metadata. It does not parse manifests with
regular expressions, carry a second handwritten version, inherit another
product's version, or rewrite versions for a destination.

Before creating a source or native candidate, the product orchestrator must
prove that:

- the selected package and binary targets match the reviewed descriptor;
- the version has canonical `MAJOR.MINOR.PATCH` form;
- the workflow commit is exactly the commit named by the product's annotated,
  signed `PREFIX<VERSION>` tag;
- the committed lockfile and accepted source closure are the inputs used for
  packaging and native builds;
- the same release identity does not already name different bytes in a source
  registry, immutable object path, GitHub release identity, or stable-pointer
  history.

The shared internal build-manifest schema records at least:

- schema version, product, package, version, tag, full source commit, and exact
  release toolchain;
- for a source candidate, package inventory and digest plus the exact accepted
  supporting-package closure;
- for each native candidate, target, payload name, installed commands, size,
  final SHA-256, consumer assignment, and accurate platform-authenticity
  evidence;
- references to any detached signature, attestation, or public readback
  evidence required by the product profile.

Every downstream job validates this manifest before reading a candidate. Once
the final hash is recorded, no assembly or publication job may rebuild, resign,
or mutate that payload.

Direct assembly derives a narrower public manifest from exact retained
evidence. It includes the release identity, immutable archive identity, target,
size and hashes, installed commands, platform-authenticity claim, installer and
checksum identities, and detached-signature metadata. The detached manifest
signature covers the exact final bytes. Public metadata contains no credentials
or private workflow paths.

Versioned objects are immutable and product-scoped. A product's mutable stable
pointer may advance only after source/native verification and public readback of
the exact referenced bytes. A release, rollback, or recovery operation cannot
change another product's pointer or provenance record.

The B04 launcher and complete suite have a B04-owned release identity; they do
not borrow the version or tag of a capability CLI. A direct suite manifest also
records, for every component, its product, package, version, signed tag, source
commit, target, final payload hash, and provenance reference. Component
versions may differ. Suite assembly proves compatibility explicitly and never
infers it from equal version numbers.

The aggregate `liknon-cli` source package is a distinct provenance case: its
source candidate records the accepted package closure from which it compiles
all owned binary targets. A direct native suite instead records the exact
retained B01-B03 native candidates that it composes. Neither form may be
presented as evidence for the other.

## Platform Contract

The B01 target matrix is the initial common target matrix. B02, B03, and B04
must explicitly adopt every row they publish. A product may narrow or extend
the matrix only through its owning plan, with accurate public support claims
and target-native evidence. Silence is not an approved deviation.

Common target mechanics include:

- distinct GNU and MUSL artifacts for Linux x64 and arm64;
- GNU preference only when the installer detects both compatible
  implementations;
- native Windows execution and declared signing verification;
- native macOS execution and an accurate signing/notarization claim;
- one retained final payload and final hash per product and target;
- no rebuild or mutation after a candidate enters downstream assembly.

## Repository Shape

The intended ownership shape is:

```text
distribution/cli/
|-- common/
|   |-- schemas/
|   |-- scripts/
|   `-- installers/
|-- validation/
|-- manager/
|-- tool/
`-- suite/

.github/workflows/
|-- _cli-native-candidate.yml
|-- _cli-direct-assembly.yml
|-- _cli-public-verification.yml
|-- release-validation.yml
|-- release-manager.yml
|-- release-tool.yml
`-- release-suite.yml
```

Names may be refined during B01.0, but the common/product separation and thin
product orchestrators are required. Reusable workflows run with read-only,
credential-free defaults. Product publication jobs bind only their own
protected environments and pass secrets explicitly; broad secret inheritance
is prohibited.

Generated state remains product-scoped below `target/distribution/` and is
ignored. Public Cloudflare objects use independent immutable product prefixes
and independent mutable stable pointers. Publication authority must also be
enforced by storage authorization. If the provider cannot scope a credential
to one product prefix, products use separate buckets or another equally strong
boundary rather than relying only on workflow convention.

## Build And Publication Boundaries

The foundation may build, inspect, assemble, verify, and publish only the
product selected by its caller. It does not create a multi-product release by
default.

- B01, B02, and B03 each build their standalone payload once per target and
  publish it under their own authority.
- B04 builds the launcher payload once per target.
- A B04 direct native suite combines the exact retained launcher payload with
  exact verified B01-B03 payloads; it does not rebuild capability CLIs.
- The `liknon-cli` crates.io source package is a separate source-distribution
  case: it owns its declared binary targets and compiles them from the accepted
  package closure rather than embedding native candidates.

Common implementation does not imply coordinated product versions. A complete
suite records the exact version, commit, target, and hash of every component it
contains.

## Verification Contract

Shared-foundation tests cover generic invariants with fixture descriptors.
Every product plan additionally covers its own commands and semantics. Once a
second product adopts a shared primitive, changes to that primitive must run
the conformance cases for every adopted product profile.

Required common evidence includes:

- descriptor/schema validation and unsafe-input rejection;
- structured package selection, canonical version, exact signed tag, and source
  commit agreement;
- source-package inventory, digest, and supporting-package closure agreement;
- one-build-per-product-target enforcement;
- archive, manifest, installer, alias, and hash consistency;
- deterministic internal and public manifest generation;
- platform and libc selection;
- credential isolation;
- immutable-object conflict rejection;
- interrupted installation and publication recovery;
- public readback of exact retained bytes;
- proof that one product release cannot alter another product storage boundary
  or namespace.

## Acceptance Criteria

- [ ] B01 implements the shared primitives and a Validation product profile.
- [ ] No common primitive contains Validation behavior or hard-coded
      Validation publication authority.
- [ ] B02 and B03 instantiate the same contract with independent profiles and
      releases.
- [ ] B04 builds only its launcher and composes capability candidates without
      rebuilding them.
- [ ] Every product retains explicit target, version, tag, credential,
      namespace, test, approval, and recovery ownership.
- [ ] Every source and native channel is traceable to one product release
      identity without a handwritten or inherited version.
- [ ] Every complete suite records exact component identities and hashes without
      requiring coordinated component versions.
- [ ] Shared changes are tested against every product profile that already
      consumes them.
