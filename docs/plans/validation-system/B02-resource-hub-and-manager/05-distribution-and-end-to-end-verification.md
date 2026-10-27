# Phase B02.5: Distribute Manager And Verify Hub-To-Workspace Behavior

## Status

`Provisional`. Blocked by B02.4, the credential-free shared distribution
foundation implemented and proven through B01.1-B01.4, and explicit human
release approval. It does not wait for B01 Validation publication.

Operation labels in this document are placeholders for the exact grammar
selected by B02.0.

[Back to the Resource Hub and Manager plan](README.md)

## Purpose

Distribute Manager independently and prove the complete path from authenticated
Hub metadata to a verified workspace resource store.

## Distribution Channels

Publish one Manager version through:

- signed tagged GitHub source for `apps/liknon-manager`;
- the `liknon-manager` crates.io source package;
- one authenticated native archive per supported target through the project
  download domain.

Each channel installs `liknon-manager` and `likm` from one implementation.
Native target, signing, provenance, installer, rollback, and public-readback
rules conform to the
[shared CLI distribution foundation](../cli-distribution-foundation.md) while
retaining separate version, tag, package, artifact, and credential identities.

The crates.io candidate includes no unpublished path dependency. Publish and
read back any exact supporting-crate version in its dependency closure before
publishing `liknon-manager`; reuse an already published byte-equivalent version
rather than attempting to replace it.

The website offers Manager separately and may offer an installer selection or
complete convenience bundle. Selecting the bundle does not make another CLI a
Manager dependency.

## Manager Product Profile

The reviewed descriptor supplies at least:

- product ID `manager` and owner B02;
- Cargo package and path `liknon-manager` / `apps/liknon-manager`;
- product orchestrator `.github/workflows/release-manager.yml`;
- canonical payload `liknon-manager` and installed alias `likm`;
- signed-tag prefix `liknon-manager-v`;
- enforceable Manager-only Cloudflare storage boundary, immutable release
  prefix, and stable pointer;
- every adopted target row and runtime baseline;
- Manager-specific source, Hub-client, mutation, recovery, help, and version
  smoke tests;
- Manager-only protected environments and accountable recovery owner.

The descriptor contains no Hub publication credential. Manager release jobs
cannot publish resource catalog or payload objects, and Hub publication jobs
cannot select a Manager candidate.

## Version And Provenance

The exact `liknon-manager` package at `apps/liknon-manager` is Manager's sole
version authority. The release workflow selects it through structured Cargo
metadata and rejects a candidate unless:

- package name, package path, and binary targets match the reviewed Manager
  descriptor;
- the package version has canonical `MAJOR.MINOR.PATCH` form;
- the selected commit is exactly identified by the annotated, signed
  `liknon-manager-v<VERSION>` tag;
- the committed lockfile and inspected source-package closure are the inputs to
  every Manager channel; and
- the release identity does not name different bytes in crates.io, GitHub, or
  Manager's immutable Cloudflare namespace.

Manager carries no handwritten release version and inherits no version, tag,
or commit from Validation, Tool, the Hub resource corpus, or a complete suite.
GitHub source, crates.io source, and native candidates all refer to the same
Manager release identity `(manager, liknon-manager, version, tag, commit)`.

The shared internal manifest records the inspected source-package digest and
supporting-package closure plus the exact final target, commands, size,
SHA-256, toolchain, and platform-authenticity evidence for every native
candidate. The public direct manifest is derived from those retained records
and names only Manager archives, installers, checksums, signatures, and public
readback evidence.

Manager's stable pointer advances only after exact source/native candidates and
public bytes have been verified. Retry, rollback, and recovery reuse the same
retained identity and cannot alter another product's provenance or pointer.

## Native Candidate Contract

For every adopted target, the shared credential-free build lane:

1. builds `apps/liknon-manager` once from the selected commit and lockfile;
2. retains one final canonical payload after required platform sealing;
3. runs target-native version, help, alias, network-boundary, and representative
   Manager semantic checks;
4. records product, package, version, tag, target, toolchain, commit, size,
   final SHA-256, and accurate platform-authenticity evidence;
5. transports the exact retained candidate without publication credentials.

GNU and MUSL candidates remain distinct on Linux x64 and arm64. Windows and
macOS use the common declared trust modes and native verification. A failed or
missing target row blocks that Manager target from publication.

## Direct Assembly And Installation

The common assembler creates Manager-specific archives, signed metadata,
checksums, POSIX and PowerShell installers, and public-readback evidence. It
places objects only below Manager's immutable version prefix and updates only
Manager's stable pointer after public verification.

Installers authenticate metadata before parsing it, select exactly one target,
verify the archive and both installed command names, install transactionally
without elevated privileges, and preserve an existing installation on failure.
Manager installation neither installs resources nor contacts the Hub.

## End-To-End Matrix

From clean environments:

1. discover an official knowledge resource;
2. inspect metadata without downloading its payload;
3. install that resource and one transitive dependency;
4. verify manifest, lock, payloads, provenance, and trust;
5. consume the result with B03 Tool when available;
6. update within an accepted constraint;
7. reject a forged, revoked, incompatible, and malicious resource;
8. remove the selection without deleting unowned bytes;
9. restore exact state from manifest and lock;
10. exercise explicit materialization and drift detection;
11. perform offline verification after the network is removed;
12. install one project-controlled conformance workflow and one external
    workflow with unrelated node IDs, graph shape, profile dependencies, and
    declared effects;
13. prove that neither installation activates a workflow or receives a
    namespace-based privilege. When C02 resources already exist, repeat the
    same case with the official action workflow without changing Manager code.

Repeat representative cases through both command names and all published
Manager channels.

Also run the shared-foundation conformance suite with the Manager descriptor,
including cross-product namespace, candidate, alias, target, stable-pointer,
credential, version, tag, commit, and provenance-substitution rejection.

## Hub Publication Verification

- Publish immutable official-resource candidates independently from Manager.
- Read every metadata and payload object back through the production origin.
- Authenticate catalog snapshots and object references.
- Verify cache headers, rollback protection, yanking, revocation, and stable
  catalog-pointer behavior.
- Confirm a resource-only release changes no Manager or Tool artifact.

## Evidence

Retain machine-readable evidence for source package, signed tag, full commit,
Manager version, source-package digest and closure, command name, target, final
native hash, public-manifest identity, Hub snapshot, resource IDs and versions,
lock digest, payload hashes, mutation result, recovery result, stable-pointer
transition, and public readback.

Evidence contains no credential, authorization header, private cache path, or
sensitive workspace content.

## Publication And Recovery

Build and verification jobs have no publication credentials. Manager
publication, Hub metadata signing, Hub object publication, crates.io, GitHub,
and native-object publication use isolated identities and protected approval
boundaries.

A partial publication resumes only from exact retained candidates. Immutable
objects are never overwritten to repair a release.

The product-specific release orchestrator is thin. Reusable build, assembly,
and verification workflows receive no broad inherited secrets. Isolated
Manager publication jobs receive only their destination-specific authority.

## Exit Criterion

Users can install Manager through every declared channel and reproducibly obtain
selected, authenticated resources from the Hub without executing resource
content or depending on Validation CLI or Tool. Every Manager native target is
the exact retained output of the common build contract, while all publication
identity and authority remain B02-specific.
