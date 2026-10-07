# Phase B01.9: Publish With Resumable State

[Back to the distribution plan](README.md)

## Purpose

Publish one fully verified release to R2, npm, RubyGems, PyPI, NuGet, and
crates.io while isolating credentials and making a partial multi-destination
release safely resumable without rebuilding or replacing bytes.

## Dependencies

- [Phase B01.8](08-end-to-end-verification.md) is complete.
- Every final candidate archive and both manifests are retained exactly.
- Destination identities and credentials approved in Phase B01.0 are available
  through repository or environment secret controls.

## Workflow And Trust Boundaries

The existing continuous-integration workflow remains non-publishing. A
dedicated release workflow runs for an explicitly selected release tag with
`cancel-in-progress: false`.

The workflow separates these trust zones:

1. **Preflight:** validate tag, version, source commit, destination state, custom
   domain readiness, and normal project gates.
2. **Native build:** build and test one binary per target without destination
   credentials.
3. **Build-manifest assembly:** record target, filename, toolchain, commit, size,
   and SHA-256.
4. **Distribution assembly:** create direct archives, installers, Cargo package,
   npm tarballs, gems, wheels, and the .NET Tool pointer/RID packages without
   rebuilding Liknon.
5. **Distribution verification:** prove every embedded native binary matches the
   build manifest.
6. **Destination publication:** give each isolated job only its own credential
   or trusted-publishing identity.
7. **Post-publication handoff:** expose destination evidence to Phase B01.10
   without exposing publication credentials.

Third-party actions are pinned to full commit SHAs. Internal GitHub Actions
artifacts are private workflow transport, not a public download channel. Their
retention is the shortest reviewed window that still permits deterministic
recovery.

## Credential Boundaries

- Build, assembly, and verification jobs have no publish credential.
- The Cloudflare job receives one bucket-scoped credential and cannot publish
  to a registry.
- crates.io, npm, RubyGems, PyPI, and NuGet jobs each receive only their own
  identity.
- A destination job cannot modify another destination.
- Secrets never enter manifests, archives, logs, cache keys, or committed
  configuration.

## Publication Sequence

1. Preflight every final artifact and every destination before the first write.
2. Retain the exact build manifest, public metadata, direct archives, npm
   tarballs, gems, wheels, .NET Tool pointer/RID packages, and Cargo package.
3. Upload immutable direct archives, checksums, signature, and installers.
4. Read each direct object back through the custom domain and verify bytes and
   headers.
5. Upload the versioned direct `manifest.json` only after all referenced objects
   pass verification.
6. Publish npm platform packages before the npm root package.
7. Publish Ruby platform gems through the isolated RubyGems job.
8. Publish platform wheels through the isolated PyPI job.
9. Publish every .NET Tool RID package through the isolated NuGet job and read
   each accepted package identity back.
10. Publish the top-level .NET Tool pointer package only after every referenced
    RID package is available at the exact version.
11. Publish the Cargo source package through the isolated crates.io job.
12. Record the destination response, immutable identity, and checksum for every
   successful operation.

`channels/stable.json` is not updated here. Phase B01.10 updates it only after
all intended public installation paths pass.

## Partial Publication And Recovery

R2 and five registries cannot commit atomically. Recovery follows these rules:

1. Never rebuild or restage an artifact merely to retry publication.
2. Before skipping an existing object or package, compare destination content
   and identity with the retained artifact.
3. Resume only missing operations whose prerequisites remain hash-identical.
4. Publish package dependency leaves before packages that reference them.
5. Treat the NuGet pointer package as that registry's completion marker; never
   publish it while a referenced RID package is absent or mismatched.
6. Treat the direct `manifest.json` as the completion marker for its versioned
   R2 prefix.
7. Never overwrite an immutable R2 release key.
8. Never replace package bytes already accepted by a registry.
9. If exact recovery is impossible, stop, diagnose, and prepare a new patch
   version.

There is no rollback fiction. Unpublishing packages or deleting released R2
objects is not the normal recovery mechanism.

## Implementation Tasks

1. Add the dedicated release workflow and concurrency policy.
2. Implement tag/version/destination preflight before credentials are exposed.
3. Add isolated Cloudflare, npm, RubyGems, PyPI, NuGet, and crates.io
   publication jobs.
4. Implement upload/read-back verification and direct-manifest-last ordering.
5. Implement exact-artifact existence checks for retries.
6. Record publication evidence without credentials or mutable aliases.
7. Exercise failure injection after each destination boundary to prove resumable
   behavior.

## Verification

- A mismatched tag, existing conflicting version, missing artifact, or failed
  signature blocks all writes.
- Build and assembly jobs cannot access publication identities.
- Every destination job is unable to authenticate to the other destinations.
- An interrupted workflow resumes from the exact retained artifacts.
- A destination mismatch blocks retry and requires a new version.
- The direct manifest never references an absent or unverified object.
- `stable.json` remains unchanged throughout this phase.

## Exit Criterion

Each destination exposes a mutually consistent immutable version, or any
partial release remains safely resumable from the exact retained artifacts with
no rebuild, overwrite, or cross-destination credential exposure.
