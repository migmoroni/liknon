# Phase B01.1: Centralize Version And Provenance

> **Superseded public topology:** this phase records implemented provenance
> machinery. B04 applies it to one `liknon-cli` suite identity rather than an
> independent Validator release identity.

## Status

`Provisional`. Blocked by B01.0.

[Back to the CLI distribution plan](README.md)

## Purpose

Implement product-neutral descriptor and manifest contracts, then derive the
Validation release identity from structured package metadata. Create
deterministic internal and public evidence linking one source commit to the
crates.io source package and each native artifact.

## Dependencies

- [Phase B01.0](00-public-contract.md) is complete.
- External readiness checkpoint [AP-1](accounts-and-permissions.md#ap-1) is
  complete.

## Version Authority

The release workflow obtains the version through `cargo metadata` and rejects
the release unless:

- the selected package is exactly the CLI package approved in B01.0;
- the version matches canonical `MAJOR.MINOR.PATCH`;
- the workflow commit is exactly tagged `liknon-validator-v<VERSION>`;
- the annotated tag signature is valid under the approved identity;
- source and lockfile pass release gates;
- no crates.io version, GitHub release identity, or Cloudflare versioned key
  contains conflicting content for that version.

Release automation does not parse `Cargo.toml` with regular expressions, carry
a second handwritten version, or normalize versions per destination.

The selected package is `apps/liknon-validator`. Its package name `liknon-validator`,
package version, and binary targets `liknon-validator` and `likv` are validated
separately. Shared workspace metadata may define version policy, but it is not a
second application-version value.

## Internal Build Manifest

After native builds, CI creates `build-manifest.json`:

```json
{
  "schemaVersion": 1,
  "product": "validation",
  "package": "liknon-validator",
  "version": "<cargo-version>",
  "tag": "liknon-validator-v<cargo-version>",
  "commit": "<full-source-commit>",
  "releaseToolchain": "<exact-rustc-version>",
  "artifacts": [
    {
      "target": "x86_64-unknown-linux-gnu",
      "payloadFile": "liknon-validator",
      "installedCommands": ["liknon-validator", "likv"],
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

The schema is shared; the record is product-specific private workflow
provenance, not a runtime API. Every assembly and verification job validates it
before touching a binary. GNU and MUSL have separate records. Windows records
its expected certificate fingerprint, signature digest, and verified RFC 3161
timestamp evidence. macOS records strict ad hoc verification and
`notarized: false`. Linux records no native signer.

## Public Direct Manifest

Direct assembly derives a narrower public `manifest.json`. It records:

- schema version, Liknon version, tag, and source commit;
- immutable archive path, target, size, format, and SHA-256;
- extracted command names, size, payload equality, and SHA-256;
- accurate platform-authenticity mode and public verification evidence;
- installer path, size, hash, detached-signature path, algorithm, and key ID;
- `SHA256SUMS` path, size, and hash.

`manifest.sig` signs the exact final manifest bytes. Installer metadata is
included before signing so users can authenticate scripts before execution.
The public manifest contains no workflow internals or credentials.

## Implementation Tasks

1. Define and validate the product-neutral descriptor and manifest schemas.
2. Select the CLI package and version using structured Cargo metadata and the
   reviewed Validation descriptor.
3. Validate both binary targets and assemble the inspected crates.io candidate.
4. Remove authored release versions from workflow and staging paths.
5. Enforce the approved version, tag, commit, and signature contract before
   artifact assembly.
6. Generate the internal manifest from normalized native-job evidence.
7. Generate the public manifest deterministically from verified direct
   archives, checksum data, and installers.
8. Reject duplicate targets, unsafe paths, unknown products or consumers,
   product/namespace mismatches, incomplete signing evidence, size mismatches,
   and hash mismatches.
9. Preserve stable serialization required by detached signatures.

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
- A fixture for another product passes the shared schema without activating a
  Validation path, tag, test, namespace, or authority.

## Exit Criterion

One package-derived Validation identity and deterministic evidence connect the
signed source revision to the crates.io package and every direct artifact
without introducing another version or provenance source. The same schemas can
represent another CLI without copying logic or inheriting Validation identity.
