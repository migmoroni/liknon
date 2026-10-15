# Phase B01.1: Centralize Version And Provenance

[Back to the CLI distribution plan](README.md)

## Purpose

Derive every CLI release identity from structured root-workspace metadata and
create deterministic internal and public evidence linking one source commit to
each native artifact.

## Dependencies

- [Phase B01.0](00-public-contract.md) is complete.
- External readiness checkpoint [AP-1](accounts-and-permissions.md#ap-1) is
  complete.

## Version Authority

The release workflow obtains the version through `cargo metadata` and rejects
the release unless:

- the selected package is exactly the CLI package approved in B01.0;
- the version matches canonical `MAJOR.MINOR.PATCH`;
- the workflow commit is exactly tagged `v<VERSION>`;
- the annotated tag signature is valid under the approved identity;
- source and lockfile pass release gates;
- no GitHub release identity or Cloudflare versioned key contains conflicting
  content for that version.

Release automation does not parse `Cargo.toml` with regular expressions, carry
a second handwritten version, or normalize versions per destination.

The selected package is `apps/liknon-cli`; its package name and binary target
are validated separately, and its version must be inherited from the root
workspace release authority.

## Internal Build Manifest

After native builds, CI creates `build-manifest.json`:

```json
{
  "schemaVersion": 1,
  "version": "<cargo-version>",
  "tag": "v<cargo-version>",
  "commit": "<full-source-commit>",
  "releaseToolchain": "<exact-rustc-version>",
  "artifacts": [
    {
      "target": "x86_64-unknown-linux-gnu",
      "file": "liknon",
      "size": 0,
      "sha256": "<sha256>",
      "platformAuthenticity": {
        "mode": "not-applicable",
        "method": "none"
      },
      "consumer": "direct:x86_64-unknown-linux-gnu"
    }
  ]
}
```

The manifest is private workflow provenance, not a runtime API. Every assembly
and verification job validates it before touching a binary. GNU and MUSL have
separate records. Windows records its expected certificate fingerprint,
signature digest, and verified RFC 3161 timestamp evidence. macOS records strict
ad hoc verification and `notarized: false`. Linux records no native signer.

## Public Direct Manifest

Direct assembly derives a narrower public `manifest.json`. It records:

- schema version, Liknon version, tag, and source commit;
- immutable archive path, target, size, format, and SHA-256;
- extracted executable name, size, and SHA-256;
- accurate platform-authenticity mode and public verification evidence;
- installer path, size, hash, detached-signature path, algorithm, and key ID;
- `SHA256SUMS` path, size, and hash.

`manifest.sig` signs the exact final manifest bytes. Installer metadata is
included before signing so users can authenticate scripts before execution.
The public manifest contains no workflow internals or credentials.

## Implementation Tasks

1. Select the CLI package and version using structured Cargo metadata.
2. Remove authored release versions from workflow and staging paths.
3. Enforce the approved version, tag, commit, and signature contract before
   artifact assembly.
4. Define structurally validated schemas for internal and public manifests.
5. Generate the internal manifest from normalized native-job evidence.
6. Generate the public manifest deterministically from verified direct
   archives, checksum data, and installers.
7. Reject duplicate targets, unsafe paths, unknown consumers, incomplete
   signing evidence, size mismatches, and hash mismatches.
8. Preserve stable serialization required by detached signatures.

## Verification

- Structured Cargo metadata selection handles valid and malformed fixtures.
- Invalid version grammar, tag, commit, or signature fails before publication.
- Schema-version mismatches fail closed.
- A target can feed only its exact direct archive.
- Cross-architecture or cross-libc assignments fail.
- Repeated public-manifest generation from identical inputs is byte-identical.
- Signature verification succeeds for exact bytes and fails after mutation.
- Windows and macOS records cannot overstate their declared trust modes.
- Linux records cannot invent a platform-native signer.
- Generated paths contain no independently authored release version.

## Exit Criterion

One workspace-derived identity and deterministic manifest tooling connect the
signed source revision to every direct-distribution artifact without introducing
a second version or provenance source.
