# Phase B01.5: Publish With Resumable State

## Status

`Provisional`. Blocked by B01.4 and explicit human release approval.

[Back to the CLI distribution plan](README.md)

## Purpose

Publish one verified validation CLI release to crates.io, GitHub, and
Cloudflare while isolating credentials and making partial publication
resumable without rebuilding candidates.

## Dependencies

- [Phase B01.4](04-end-to-end-verification.md) is complete.
- Exact candidates and evidence remain within their retention window.
- External readiness checkpoint [AP-5](accounts-and-permissions.md#ap-5) is
  complete.

## Workflow And Trust Boundaries

The normal CI workflow remains non-publishing. A dedicated release workflow
runs for an explicitly selected signed tag with cancellation disabled.

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
3. Publish any inspected supporting-crate candidate not already available at
   the exact required version, in dependency order, and verify public readback.
4. Publish the inspected `liknon-validate` source package to crates.io.
5. Read the accepted application package back and verify source, dependency,
   and command inventory.
6. Create the metadata-only immutable GitHub Release for the exact signed tag.
7. Confirm it points to the expected commit and contains no binary asset.
8. Upload immutable archives, checksum data, installer scripts, and detached
   installer signatures to versioned R2 keys.
9. Read every object back through the production custom domain and compare bytes
   and headers.
10. Upload `manifest.sig`, then `manifest.json` as the completion marker only
   after every referenced object passes readback.
11. Read back and authenticate the complete versioned release.
12. Record immutable identities, hashes, headers, and destination responses.

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
