# Phase B01.9: Publish With Resumable State

[Back to the distribution plan](README.md)

## Purpose

Publish one fully verified release to GitHub, R2, npm, RubyGems, PyPI, NuGet,
and crates.io while isolating credentials and making a partial
multi-destination release safely resumable without rebuilding or replacing
bytes.

## Dependencies

- [Phase B01.8](08-end-to-end-verification.md) is complete.
- Every final candidate archive and both manifests are retained exactly.
- The credential-free approval gate and the isolated signing and destination
  environments or equivalent identity boundaries approved in Phase B01.0 are
  configured.

## Workflow And Trust Boundaries

The existing continuous-integration workflow remains non-publishing. A
dedicated release workflow runs for an explicitly selected release tag with
`cancel-in-progress: false`.

The workflow separates these trust zones:

1. **Preflight:** validate signed tag, canonical version, source commit,
   immutable-release setting, destination state, custom-domain readiness, and
   normal project gates.
2. **Native build and signing:** build one binary per target without destination
   credentials, apply isolated native platform signing where required, and test
   the final bytes.
3. **Build-manifest assembly:** record target, filename, toolchain, commit, size,
   and SHA-256.
4. **Distribution assembly:** create direct archives, installers, Cargo package,
   npm tarballs, gems, wheels, and the .NET Tool pointer/RID packages without
   rebuilding Liknon.
5. **Distribution verification:** prove every embedded native binary matches the
   build manifest.
6. **Destination publication:** after the credential-free approval gate, give
   each isolated job only its own destination environment and OIDC or
   trusted-publishing identity, or its explicitly approved one-time bootstrap
   credential.
7. **Post-publication handoff:** expose destination evidence to Phase B01.10
   without exposing publication credentials.

Third-party actions are pinned to full commit SHAs. Internal GitHub Actions
artifacts are private workflow transport, not a public download channel. Their
retention is the shortest reviewed window that still permits deterministic
recovery.

The protected human-approval gate contains no publication or signing credential.
Passing it authorizes destination jobs to start but does not itself expose an
identity. Native signing and every publication destination use distinct
protected environments or an equivalent isolation mechanism. OIDC policies bind
the exact repository, workflow, destination environment, and supported package
scope wherever the provider permits those claims.

## Credential Boundaries

- Build, assembly, and verification jobs have no publish credential.
- Native signing jobs use signing-only environments and cannot publish to any
  destination.
- The GitHub Release job uses its own destination environment, receives
  repository `contents: write` permission, and has no external destination
  identity.
- The Cloudflare job uses its own destination environment, receives one
  bucket-scoped credential, and cannot publish to a registry.
- crates.io, npm, RubyGems, PyPI, and NuGet jobs each use a distinct destination
  environment and receive only their own identity.
- Registry jobs use OIDC or destination-native trusted publishing wherever the
  registry supports it.
- An unavoidable first-publication credential is destination-scoped, used only
  for the documented bootstrap operation, audited, and revoked before a normal
  release can be considered complete.
- Persistent R2 and native-signing credentials, when required, are isolated from
  one another and from every registry, have accountable owners, and follow their
  recorded rotation procedures.
- A destination job cannot modify another destination.
- Secrets never enter manifests, archives, logs, cache keys, or committed
  configuration.

## Publication Sequence

1. Preflight every final artifact and every destination before the first write.
2. Retain the exact build manifest, public metadata, direct archives, npm
   tarballs, gems, wheels, .NET Tool pointer/RID packages, and Cargo package.
3. Publish the metadata-only immutable GitHub Release for the signed tag, with
   no uploaded binary assets, and verify that the tag is locked to the workflow
   commit.
4. Upload immutable direct archives, checksums, signatures, and installers.
5. Read each direct object back through the custom domain and verify bytes and
   headers.
6. Upload the versioned direct `manifest.json` only after all referenced objects
   pass verification.
7. Publish npm platform packages before the npm root package.
8. Publish Ruby platform gems through the isolated RubyGems job.
9. Publish platform wheels through the isolated PyPI job.
10. Publish every .NET Tool RID package through the isolated NuGet job and read
   each accepted package identity back.
11. Publish the top-level .NET Tool pointer package only after every referenced
    RID package is available at the exact version.
12. Publish the Cargo source package through the isolated crates.io job.
13. Record the destination response, immutable identity, and applicable digest
    or verified logical contents for every successful operation.

`channels/stable.json` is not updated here. Phase B01.10 updates it only after
all intended public installation paths pass.

## Destination Equivalence Contract

The retained local candidate remains byte-identical across retries. Read-back
verification is destination-specific because registries may add signatures or
normalize their outer package envelope:

- R2 objects must be byte-identical to the retained object and match expected
  headers;
- the GitHub Release must retain the exact signed tag and commit and contain no
  uploaded binary assets;
- npm, RubyGems, PyPI, and crates.io must match package identity, exact version,
  registry-reported digest where available, logical package contents, and every
  embedded native hash; raw package equality is additionally required only when
  the registry guarantees preservation of uploaded bytes;
- NuGet must match package identity, exact version, RID and pointer
  relationships, valid repository signature, logical package contents, and the
  embedded native hash. Raw downloaded `.nupkg` equality is not required after
  nuget.org adds its repository signature.

Each destination adapter implements and tests its own equivalence predicate.
Generic file equality must not replace those contracts. A retry skips an
existing destination object only after its predicate proves equivalence with the
retained candidate and build manifest.

## Partial Publication And Recovery

GitHub, R2, and five registries cannot commit atomically. Recovery follows
these rules:

1. Never rebuild or restage an artifact merely to retry publication.
2. Before skipping an existing object or package, apply its destination-specific
   equivalence predicate to the retained artifact and build manifest.
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

1. Add the dedicated release workflow, credential-free approval gate, isolated
   signing and destination environments, and concurrency policy.
2. Implement signed-tag, canonical-version, immutable-release, and destination
   preflight before publishing identities are exposed.
3. Add isolated Cloudflare, npm, RubyGems, PyPI, NuGet, and crates.io
   publication jobs using trusted publishing where supported.
4. Implement upload/read-back verification and direct-manifest-last ordering.
5. Implement and test destination-specific equivalence checks, including NuGet
   repository-signature verification.
6. Implement exact retained-artifact existence checks for retries.
7. Record publication evidence without credentials or mutable aliases.
8. Exercise failure injection after each destination boundary to prove resumable
   behavior.

## Verification

- A mismatched tag, existing conflicting version, missing artifact, or failed
  signature blocks all writes.
- A noncanonical version or unavailable immutable-release protection blocks all
  writes.
- Build and assembly jobs cannot access publication identities.
- The human-approval gate exposes no publication or signing identity.
- Every destination job is unable to authenticate to the other destinations.
- An interrupted workflow resumes from the exact retained artifacts.
- A destination mismatch blocks retry and requires a new version.
- A repository-signed NuGet package passes through logical-content and embedded
  binary verification without an invalid raw-archive equality requirement.
- Normal registry publication exposes no long-lived token; a bootstrap token,
  when unavoidable, is proven revoked before completion.
- The direct manifest never references an absent or unverified object.
- `stable.json` remains unchanged throughout this phase.

## Exit Criterion

Each destination exposes a mutually consistent immutable version, or any
partial release remains safely resumable from the exact retained artifacts
after destination-specific equivalence checks, with no rebuild, overwrite,
long-lived registry publication token, or cross-destination credential
exposure. Persistent R2 and native-signing credentials remain confined to their
separately governed boundaries.
