# Phase B01.3: Assemble Cloudflare Direct Distribution

[Back to the distribution plan](README.md)

## Purpose

Create immutable native archives, public release metadata, and user-local
installers for users who want a ready Liknon binary without Cargo, Node, Ruby,
Python, .NET, or a repository clone.

This phase assembles and tests direct distribution locally. Phase B01.9 performs
the first public upload.

## Dependencies

- [Phase B01.2](02-native-builds.md) is complete.
- The custom domain, R2 bucket, signing model, and platform matrix approved in
  Phase B01.0 are available.
- Every native input matches `build-manifest.json`.

## Public Object Layout

The desired production layout is:

```text
https://<download-domain>/
├── releases/
│   └── v<VERSION>/
│       ├── manifest.json
│       ├── manifest.sig
│       ├── SHA256SUMS
│       ├── install.sh
│       ├── install.sh.sig
│       ├── install.ps1
│       ├── install.ps1.sig
│       ├── liknon-v<VERSION>-x86_64-unknown-linux-musl.tar.gz
│       ├── liknon-v<VERSION>-aarch64-unknown-linux-musl.tar.gz
│       ├── liknon-v<VERSION>-x86_64-apple-darwin.tar.gz
│       ├── liknon-v<VERSION>-aarch64-apple-darwin.tar.gz
│       └── liknon-v<VERSION>-x86_64-pc-windows-msvc.zip
└── channels/
    └── stable.json
```

`<download-domain>` is replaced only with the domain approved in Phase B01.0.
The `r2.dev` development endpoint is not a production channel.

## Archive Contract

- Each archive contains the exact native bytes identified by the build
  manifest, plus required license and notice files.
- Archive names derive from Cargo version and target identity rather than a
  handwritten version.
- Unix archives preserve executable permissions.
- Windows uses ZIP and the exact `liknon.exe` filename.
- Archive SHA-256, size, format, and extracted-binary SHA-256 appear in the
  public manifest.
- `SHA256SUMS` lists archives and installer scripts, not the manifest or its
  signature; the signed manifest records the checksum document's own hash.
- Repeated assembly from identical inputs is deterministic where the selected
  archive format and toolchain support it; unavoidable metadata is normalized.

## Object And Cache Contract

- Every object under `releases/v<VERSION>/` is immutable.
- A release is never repaired by overwriting an object. A conflict blocks that
  version and requires diagnosis or a new patch version.
- Versioned archives and metadata use long-lived public cache headers with
  `immutable` semantics.
- `channels/stable.json` is the only planned mutable release pointer. It has a
  short cache lifetime and is not version identity.
- Explicit cache rules cover archives, JSON, checksums, signatures, and scripts
  because custom domains do not cache every file type by default.
- Negative responses under immutable release paths are not cached, preventing
  a guessed pre-publication URL from leaving a stale cached `404`.
- A bucket lock protects the immutable release prefix. Optional staging objects
  use a separate unlocked, non-public prefix.
- Publication uploads and verifies every referenced object before uploading
  `manifest.json`; the manifest is the completion marker.

Normal publication does not depend on invalidating immutable objects. Cache
purges are limited to mutable channel metadata or incident response.

## Installer Contract

The versioned POSIX and PowerShell installers must:

- obtain the exact `manifest.json` and `manifest.sig` for the selected version;
- verify the detached signature over the exact manifest bytes against the
  Phase B01.0 trust root before parsing or trusting any manifest field;
- detect only supported operating-system and architecture combinations;
- select the exact archive declared by the public manifest;
- download through HTTPS into a temporary directory;
- verify the archive SHA-256 before extraction;
- verify the extracted native binary against the manifest;
- install to a user-writable location by default;
- accept an explicit installation directory;
- replace the destination only after all verification succeeds;
- clean temporary files on success, failure, or interruption;
- distinguish unsupported platforms, missing signature or hash verification
  tools, invalid signatures, malformed metadata, hash failures, and network
  failures;
- never execute Cargo, npm, RubyGems, Python package tools, or a command
  obtained from release metadata.

Primary documentation downloads the installer and its detached signature as
files, verifies the installer against the independently obtained trust root, and
only then executes it. Installer signatures cannot be bootstrapped solely from
the same R2 origin. A `curl | sh` form may be secondary and must state that it
forgoes installer authentication before execution. Manual archive download and
verification remain fully supported.

Automated consumers pin a concrete version rather than defaulting to `latest`
or `stable`.

## Cloudflare Operational Contract

- Use R2 Standard storage unless measured behavior justifies another class.
- Connect the approved custom domain for production caching, TLS, and traffic
  controls.
- Keep `r2.dev` disabled as a public release endpoint.
- Publish with a bucket-scoped credential available only to the Cloudflare job.
- Use an official CLI or an S3-compatible client at an exact version.
- Do not depend on an unpinned third-party upload action.
- Configure public reads through the custom domain rather than object ACL flags.
- Monitor storage and operations and configure a budget alert. Free allowances
  and zero egress pricing are advantages, not correctness assumptions.
- Keep this channel operationally independent from npm, RubyGems, PyPI, and
  NuGet.

## Implementation Tasks

1. Add version-neutral POSIX and PowerShell installer templates that pin the
   approved manifest-verification identity.
2. Assemble one target-specific archive from each manifest-verified binary.
3. Generate release-specific installers without creating a second version
   source, then sign each installer for pre-execution verification.
4. Generate `SHA256SUMS` and the public `manifest.json`, including installer and
   checksum-document hashes, deterministically.
5. Sign the exact final manifest bytes as `manifest.sig`.
6. Define reviewed cache rules, content metadata, bucket lock, and stable-channel
   shape as reproducible infrastructure configuration.
7. Keep all staged outputs under `target/distribution/direct/`.

## Verification

- Archive and extracted-binary hashes match the build manifest.
- `manifest.sig` validates with the independent trust root and fails after
  mutation.
- Installer signatures validate through the independently documented bootstrap
  and fail after script mutation.
- The signed manifest authenticates `SHA256SUMS`; a modified checksum document
  is rejected before it is used for manual verification.
- POSIX and PowerShell installers work against a local HTTP origin without
  Rust, Node, Ruby, Python, or .NET.
- Installers reject a forged manifest before parsing it, including when a
  matching forged archive and forged hash values are supplied beside it.
- Manual installation follows the documented hash-verification path.
- Installation succeeds in a path containing spaces and without elevated
  privileges.
- Tampered archives, mismatched manifests, malformed pointers, interrupted
  downloads, and unsupported targets fail without replacing an existing binary.
- Staging leaves tracked files unchanged.

## Exit Criterion

Direct archives and authenticated installers work without a language toolchain,
elevated privileges, or an external package manager. Installers authenticate
metadata before use, and every staged byte is covered by versioned, signed
metadata derived from the native build manifest.
