# Phase B01.5: Publish With Resumable State

> **Superseded public topology:** B04 owns CLI-suite publication and recovery;
> B06 coordinates its first activation. This file remains implementation
> evidence for the reusable mechanism.

## Status

`Provisional`. Blocked by B01.4 and explicit human release approval. The first
public invocation is coordinated by B06 from its frozen release-set ledger;
later Validation releases may invoke this product workflow independently.

[Back to the CLI distribution plan](README.md)

## Purpose

Provide the product-owned workflow that publishes one verified validation CLI
release to crates.io, GitHub, and Cloudflare while isolating credentials and
making partial publication resumable without rebuilding candidates.

## Dependencies

- [Phase B01.4](04-end-to-end-verification.md) is complete.
- Exact candidates and evidence remain within their retention window.
- External readiness checkpoint [AP-5](accounts-and-permissions.md#ap-5) is
  complete.

## Workflow And Trust Boundaries

The normal CI workflow remains non-publishing. A dedicated release workflow
runs for an explicitly selected signed tag with cancellation disabled. It is a
thin Validation orchestrator over shared credential-free mechanisms and
Validation-owned protected publication environments.

The workflow separates:

1. preflight and project gates;
2. native build and platform sealing;
3. internal manifest assembly;
4. direct archive, installer, checksum, and public-manifest assembly;
5. credential-free end-to-end verification;
6. credential-free human approval;
7. isolated crates.io publication;
8. isolated GitHub publication;
9. isolated Cloudflare publication;
10. post-publication evidence handoff.

Third-party actions are pinned to full commit SHAs. Private GitHub Actions
artifacts are workflow transport, not a public download channel.
Reusable workflows receive no broad inherited secret set. A product-specific
caller passes only the credential required by the selected isolated job.

## Credential Boundaries

- Build, assembly, and verification jobs have no write credential.
- The approval environment contains no secret.
- Windows signing has only its code-signing identity.
- Detached release signing has only its manifest-signing identity.
- crates.io publication has only its trusted-publishing identity or temporary
  bootstrap token.
- GitHub publication receives only repository contents permission.
- Cloudflare publication receives only a bucket-scoped credential.
- macOS ad hoc signing uses no identity.
- No credential enters an archive, manifest, log, cache key, or retained public
  evidence.

## Publication Sequence

1. Preflight crates.io, GitHub, and Cloudflare without writing.
2. Reauthenticate all retained candidates and evidence.
3. Publish the inspected `liknon-validator` source package to crates.io.
4. Read the accepted application package back and verify source, dependency,
   and command inventory.
5. Create the metadata-only immutable GitHub Release for the exact signed tag.
6. Confirm it points to the expected commit and contains no binary asset.
7. Upload immutable archives, checksum data, installer scripts, and detached
   installer signatures to versioned R2 keys.
8. Read every object back through the production custom domain and compare bytes
   and headers.
9. Upload `manifest.sig`, then `manifest.json` as the completion marker only
   after every referenced object passes readback.
10. Read back and authenticate the complete versioned release.
11. Record immutable identities, hashes, headers, and destination responses.

The stable pointer is not updated here. Phase B01.6 updates it only after public
installation succeeds.

## Recovery Contract

Publication state records `pending`, `published`, `verified`, or `failed` for
each exact destination object. A retry:

- revalidates tag, commit, version, manifests, and retained hashes;
- never rebuilds, resigns, restages, or edits a candidate;
- skips a destination object only after public readback proves equivalence;
- rejects an occupied immutable key with different content;
- never attempts to replace an accepted crates.io version;
- resumes only while every required retained artifact remains available.

If exact recovery cannot be proven, the version remains incomplete and a new
patch release is prepared. The workflow never overwrites an immutable object to
repair a release.

## Implementation Tasks

1. Add the dedicated release workflow and explicit tag input.
2. Configure per-job permissions, protected environments, and concurrency.
3. Implement preflight for tag, immutable GitHub Release state, R2 object
   conflicts, custom-domain readiness, and retained-artifact identity.
4. Implement isolated crates.io, metadata-only GitHub, and R2 publication.
5. Implement destination-specific readback and resumable state.
6. Retain enough private evidence for Phase B01.6 and bounded recovery.
7. Add fault-injection tests at every publication boundary.
8. Prove the Validation storage authorization cannot write Manager, Tool,
   suite, or unrelated release state and cannot select their protected
   environments.

## Verification

- No publication starts before candidate verification and human approval.
- Build and verification jobs cannot access destination credentials.
- crates.io, GitHub, and Cloudflare publication cannot access one another's
  credentials.
- GitHub contains no uploaded Liknon binary.
- R2 readback is byte-identical to retained candidates.
- Interrupted publication resumes without rebuilding or overwriting.
- A conflicting object, changed tag, expired candidate, or mismatched manifest
  fails closed.
- Logs and retained evidence contain no secret.

## Exit Criterion

crates.io serves the inspected source package, GitHub records the immutable
source release, and Cloudflare serves the native release. Every public object
has verified readback, and recovery uses exact retained state.
