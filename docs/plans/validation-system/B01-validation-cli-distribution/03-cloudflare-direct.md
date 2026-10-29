# Phase B01.3: Assemble Cloudflare Direct Distribution

> **Superseded public topology:** standalone Validator objects and pointers are
> not the approved target. B04 publishes one CLI-suite archive and one suite
> metadata chain per target.

## Status

`Provisional`. Blocked by B01.2.

[Back to the CLI distribution plan](README.md)

## Purpose

Implement product-neutral archive, metadata, and installer assembly, then create
immutable Validation archives, signed public release metadata, and user-local
installers for users who want a ready Liknon CLI without Rust or a repository
clone. This phase assembles and tests candidates locally; Phase B01.5 publishes
them.

## Dependencies

- [Phase B01.2](02-native-builds.md) is complete.
- Every native input matches `build-manifest.json`.
- External readiness checkpoint [AP-3](accounts-and-permissions.md#ap-3) is
  complete; the upload credential remains deferred to Phase B01.5.

## Public Object Layout

```text
https://<download-domain>/
├── releases/
│   └── liknon-validator/
│       └── v<VERSION>/
│           ├── manifest.json
│           ├── manifest.sig
│           ├── SHA256SUMS
│           ├── install.sh
│           ├── install.sh.sig
│           ├── install.ps1
│           ├── install.ps1.sig
│           ├── liknon-validator-v<VERSION>-x86_64-unknown-linux-gnu.tar.gz
│           ├── liknon-validator-v<VERSION>-x86_64-unknown-linux-musl.tar.gz
│           ├── liknon-validator-v<VERSION>-aarch64-unknown-linux-gnu.tar.gz
│           ├── liknon-validator-v<VERSION>-aarch64-unknown-linux-musl.tar.gz
│           ├── liknon-validator-v<VERSION>-x86_64-apple-darwin.tar.gz
│           ├── liknon-validator-v<VERSION>-aarch64-apple-darwin.tar.gz
│           └── liknon-validator-v<VERSION>-x86_64-pc-windows-msvc.zip
└── channels/
    └── liknon-validator/
        └── stable.json
```

The `r2.dev` development endpoint is not a production channel.

## Archive And Object Contract

- Shared assembly derives paths and names only from the validated product
  descriptor and manifest. The layout below is the Validation instance, not a
  hard-coded common layout.
- Each archive contains the exact manifest-verified payload under both command
  names plus required license and notice files.
- Names derive from Cargo version and target identity.
- Unix archives preserve executable permissions; Windows uses ZIP.
- Archive and extracted-executable size and SHA-256 appear in the public
  manifest.
- `SHA256SUMS` lists archives and installer scripts; the signed manifest records
  the checksum file's own hash.
- Objects below `releases/liknon-validator/v<VERSION>/` are immutable and use
  reviewed immutable cache semantics.
- `channels/liknon-validator/stable.json` is the only planned mutable pointer and has a short
  cache lifetime.
- Negative responses under immutable paths are not cached.
- Publication uploads and reads back every referenced object before
  `manifest.json`, which is the versioned completion marker.

## Installer Contract

The versioned POSIX and PowerShell installers:

- obtain the exact manifest and detached signature for the selected version;
- authenticate the exact manifest bytes before parsing fields;
- reject unknown, retired-out-of-scope, or revoked signing identities;
- detect only supported operating systems and architectures;
- on Linux, implement the approved automatic and explicit libc selection;
- download exactly one declared archive through HTTPS;
- verify archive and both extracted-command hashes;
- install both names transactionally to a user-owned default or explicit
  destination;
- preserve an existing installation after any failure;
- clean temporary state on success, failure, and interruption;
- never execute metadata-provided commands, invoke Cargo, alter trust stores,
  weaken operating-system security, or silently select another target;
- distinguish unsupported host, missing verifier, bad signature, malformed
  metadata, hash mismatch, network failure, and filesystem failure.

Before download, the POSIX installer emits concise selection diagnostics:
operating system, architecture, detected libc implementations, decision source,
and selected target. Automated consumers pin a version instead of relying on
the stable pointer.

Primary documentation downloads and authenticates an installer before running
it. A pipe-to-shell form may appear only as an explicitly weaker secondary
convenience. Manual archive installation remains fully supported.

For macOS, documentation prefers Apple's user-mediated `Open Anyway` path. An
exact-path quarantine-removal command may appear only when target-native tests
prove it necessary, after cryptographic verification and with a warning. Broad
or recursive attribute clearing is prohibited.

## Cloudflare Operational Contract

- Use one Validation production R2 bucket and project-controlled custom domain.
- Scope the Validation upload authority to that bucket or an equally strong
  storage boundary; do not rely on an object-prefix convention for isolation
  from other product releases.
- Keep the production `r2.dev` endpoint disabled.
- Use a bucket-scoped write credential only in the publication job.
- Pin the official or S3-compatible upload client version.
- Configure TLS, immutable and mutable cache rules, bucket lock, monitoring, and
  a budget alert as reviewed infrastructure.
- Do not depend on an unpinned third-party upload action.

## Implementation Tasks

1. Add product-neutral, version-neutral POSIX and PowerShell installer
   templates driven by authenticated product metadata.
2. Implement deterministic target and Linux libc selection.
3. Assemble one archive with both command names from each manifest-verified
   payload.
4. Generate and sign release-specific installers.
5. Generate `SHA256SUMS` and deterministic public metadata.
6. Sign the exact final manifest bytes.
7. Add signing-key lifecycle fixtures.
8. Define reproducible Cloudflare object, cache, and lock configuration.
9. Stage every generated output under
   `target/distribution/validation/direct/`.
10. Test a second fixture product through assembly and installation without
    granting it Validation paths, names, stable pointers, or credentials.

## Verification

- Archive and both command hashes match internal build evidence.
- Manifest and installer signatures fail after any mutation.
- Key activation, rotation, retirement, and revocation fixtures behave exactly
  as documented.
- POSIX and PowerShell installers work against a local HTTP fixture without
  Rust or elevated privileges.
- Linux selection covers GNU-only, MUSL-only, dual-libc, explicit, unknown, and
  insufficient-baseline cases on x64 and arm64.
- Every successful selection requests one archive; unknown selection requests
  none.
- Forged or malformed metadata is rejected before use.
- Installation works in paths containing spaces.
- Failure never replaces either valid existing command.
- Assembly modifies no tracked file.
- One product descriptor cannot select or overwrite another product's archive,
  manifest, installer, release prefix, or stable pointer.

## Exit Criterion

Every supported target has one authenticated direct archive and both installer
paths operate transactionally without a language toolchain or elevated
privileges. All staged bytes are covered by deterministic, signed, versioned
metadata, and the common assembler remains product-neutral.
