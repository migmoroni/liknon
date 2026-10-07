# Phase B01.1: Centralize Version And Provenance

[Back to the distribution plan](README.md)

## Purpose

Derive every release identity from `Cargo.toml` and establish the internal and
public manifests that prove which source commit and native bytes feed every
distribution channel.

## Dependencies

- [Phase B01.0](00-public-contract.md) is complete.
- Package, domain, platform, runtime, and signing decisions are recorded.

## Version Authority

The release workflow obtains the version with `cargo metadata` and rejects the
release unless:

- the selected package is exactly `liknon`;
- the source commit is tagged `v<version>`;
- the tag points exactly to the workflow commit;
- source and lockfile pass the release gates;
- no registry package or direct-download key contains conflicting bytes for
  that version.

Release scripts do not parse `Cargo.toml` with regular expressions or carry a
second handwritten version. Existing workflow paths containing a literal Cargo
package version are replaced with metadata-derived paths.

## Internal Build Manifest

At release execution, after native builds complete in Phase B01.2, CI creates
`build-manifest.json`:

```json
{
  "schemaVersion": 1,
  "version": "<cargo-version>",
  "tag": "v<cargo-version>",
  "commit": "<full-source-commit>",
  "releaseToolchain": "<exact-rustc-version>",
  "artifacts": [
    {
      "target": "x86_64-unknown-linux-musl",
      "file": "liknon",
      "sha256": "<sha256>",
      "consumers": [
        "direct:x86_64-unknown-linux-musl",
        "npm:linux-x64",
        "gem:x86_64-linux-gnu",
        "gem:x86_64-linux-musl",
        "pypi:linux-x64",
        "nuget:linux-x64",
        "nuget:linux-musl-x64"
      ]
    }
  ]
}
```

This manifest is private workflow provenance rather than a runtime API. Every
assembly and verification job consumes it and validates hashes before touching
a native binary. This phase implements and tests the manifest producer against
fixtures; Phase B01.2 supplies its real native-build records.

## Public Direct-Release Manifest

Direct assembly derives a separate public `manifest.json` containing no
credentials or workflow internals. For each target it records:

- exact Liknon version and source commit;
- immutable archive path, byte size, and SHA-256;
- extracted native filename and SHA-256;
- archive format and target identifier;
- public manifest schema version.

The direct release also contains `SHA256SUMS` and `manifest.sig`, a detached
signature over the exact public manifest bytes. The signing identity, algorithm,
verification method, and independent trust root are the decisions approved in
Phase B01.0.

The public manifest is a versioned direct-distribution contract. It is derived
from, but intentionally narrower than, the internal build manifest.

## Implementation Tasks

1. Add a release helper that selects Liknon and obtains package data through
   `cargo metadata`.
2. Remove literal Cargo package versions from workflow paths.
3. Define typed or structurally validated schemas for both manifests.
4. Implement internal build-manifest generation from normalized native-job
   metadata and test it with fixtures.
5. Implement deterministic public-manifest generation from a build manifest
   and staged direct archives, using fixtures until those artifacts exist.
6. Reject tag, version, commit, target, filename, consumer, size, or hash
   mismatches.
7. Ensure serializers produce stable bytes required by detached signing.

## Verification

- Unit tests cover Cargo metadata selection and malformed metadata.
- Tag/version/commit mismatches fail before any publication work.
- Manifest schema-version mismatches fail closed.
- Duplicate targets, unknown consumers, unsafe paths, and missing hashes are
  rejected.
- Repeated public-manifest generation from the same inputs is byte-identical.
- Signature verification succeeds for the exact manifest and fails after any
  byte changes.
- Workflow package paths contain no authored release version.

## Exit Criterion

Cargo-derived version authority and tested manifest tooling are ready for the
native matrix. Given complete fixture inputs, the tooling produces one
immutable build manifest and a deterministic, signed public projection for
direct downloads.
