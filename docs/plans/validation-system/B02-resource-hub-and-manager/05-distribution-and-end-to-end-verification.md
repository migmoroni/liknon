# Phase B02.5: Distribute Manager And Verify Hub-To-Workspace Behavior

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
rules follow the established B01 security model while retaining separate
version, tag, package, artifact, and credential identities.

The crates.io candidate includes no unpublished path dependency. Publish and
read back any exact supporting-crate version in its dependency closure before
publishing `liknon-manager`; reuse an already published byte-equivalent version
rather than attempting to replace it.

The website offers Manager separately and may offer an installer selection or
complete convenience bundle. Selecting the bundle does not make another CLI a
Manager dependency.

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
    namespace-based privilege. When Phase 3 resources already exist, repeat the
    same case with the official action workflow without changing Manager code.

Repeat representative cases through both command names and all published
Manager channels.

## Hub Publication Verification

- Publish immutable official-resource candidates independently from Manager.
- Read every metadata and payload object back through the production origin.
- Authenticate catalog snapshots and object references.
- Verify cache headers, rollback protection, yanking, revocation, and stable
  catalog-pointer behavior.
- Confirm a resource-only release changes no Manager or Tool artifact.

## Evidence

Retain machine-readable evidence for source identity, Manager version, command
name, target, Hub snapshot, resource IDs and versions, lock digest, payload
hashes, mutation result, recovery result, and public readback.

Evidence contains no credential, authorization header, private cache path, or
sensitive workspace content.

## Publication And Recovery

Build and verification jobs have no publication credentials. Manager
publication, Hub metadata signing, Hub object publication, crates.io, GitHub,
and native-object publication use isolated identities and protected approval
boundaries.

A partial publication resumes only from exact retained candidates. Immutable
objects are never overwritten to repair a release.

## Exit Criterion

Users can install Manager through every declared channel and reproducibly obtain
selected, authenticated resources from the Hub without executing resource
content or depending on Validation CLI or Tool.
