# Phase B01.11: Verify Public Installation And Documentation

[Back to the distribution plan](README.md)

## Purpose

Install the published release through every advertised channel, update the
stable direct pointer only after all public paths pass, and document exactly the
support that has been proven.

## Dependencies

- [Phase B01.10](10-publication-and-recovery.md) completed publication or reached
  a fully recovered consistent state.
- Every destination exposes the intended immutable version.
- Publication evidence and the public direct manifest are available.

## Public Installation Verification

Run in clean target environments:

- verify the metadata-only immutable GitHub Release, signed tag, and exact
  source commit, then clone that tag and install it with locked Cargo;
- install the exact version from crates.io on every advertised source-build
  platform with the release and minimum supported Rust toolchains;
- install the npm root package globally through npm and pnpm on every supported
  platform using separate clean user prefixes;
- install the matching gem globally through RubyGems and verify the same
  platform resolution separately through Bundler;
- install the matching wheel as a globally invocable user command through the
  pip user scheme, pipx, and `uv tool` on every supported wheel platform;
- install the exact NuGet pointer package globally with .NET SDK 10 on every
  supported platform and confirm only the matching RID package is retrieved;
- download and verify a direct archive manually;
- authenticate and run the versioned POSIX installer on supported Unix targets;
- authenticate and run the versioned PowerShell installer on Windows.

Every installation runs `liknon --version`, `liknon --help`, and a representative
`liknon validate` fixture. Reported version and native hash must agree with the
release evidence.

Windows verification begins on a clean host that does not trust the project's
self-signed certificate. macOS verification begins without disabling Gatekeeper,
removing quarantine metadata, or installing a local trust anchor. Tests record
the operating system's actual response, verify the signed release manifest and
native hash independently, and confirm that installation changes no host trust
or security setting.

If macOS blocks the verified standalone CLI, test Apple's user-mediated
`Open Anyway` path first. If that path does not apply and direct quarantine
removal is required, exercise the documented fallback only after manifest and
hash verification, against the exact Liknon binary, in a clean disposable test
environment. Confirm that it removes only `com.apple.quarantine`, preserves all
other extended attributes, uses no recursive option, and is never performed by
an installer or package-manager launcher.

For the first public release, uninstall every package-manager installation and
prove that its command shim or script no longer resolves. Starting with the
second public release, begin from the immediately preceding supported version,
use each documented package-manager update command to reach the candidate, run
the same identity and behavior checks, and then uninstall it. These public tests
complement rather than replace the two-version local lifecycle fixtures from
Phase B01.9.

## Download-Domain Verification

- Production uses the approved custom domain rather than `r2.dev`.
- Versioned objects return the intended content type, disposition, length,
  cache headers, and immutable semantics.
- `manifest.json`, `manifest.sig`, and `SHA256SUMS` agree after edge delivery.
- POSIX and PowerShell installer signatures verify through the independent trust
  bootstrap before either script is executed.
- Current independently obtained trust material accepts the release signing key,
  rejects unknown or revoked keys, and retains the intended historical behavior
  for uncompromised retired keys.
- Downloaded archive bytes match the public manifest.
- Negative responses under the immutable release prefix are not cached.
- The bucket lock protects the released prefix.
- Monitoring and budget alerts are enabled for storage and request operations.

## Stable Channel Update

Only after all intended public channels pass:

1. Generate `channels/stable.json` from the verified version and manifest URL.
2. Publish it with the reviewed short cache lifetime.
3. Purge or wait out only the mutable pointer cache as required by policy.
4. Read the pointer through the custom domain and follow it to the immutable
   manifest.
5. Repeat the direct installation smoke test through that resolved version.

Automated CI documentation continues to recommend an explicit version rather
than the mutable stable pointer.

## Documentation Deliverables

Update canonical documentation to explain:

- which channels compile locally and require Rust;
- which channels install verified native binaries without Rust;
- exact supported OS, architecture, libc, Node, npm, pnpm, Ruby, RubyGems,
  Bundler, Python, pip, pipx, uv, .NET SDK 10+, and Rust ranges;
- the distinct Linux GNU and MUSL artifacts for x64 and arm64, their
  ecosystem-native package names, and the guarantee that installation retrieves
  only the matching OS, architecture, and libc payload;
- that `gnu` denotes the glibc-linked target, GNU and MUSL may legitimately
  coexist, and automatic Liknon-owned selection prefers GNU when both are
  detected;
- the direct POSIX installer's `--libc=gnu|musl` override, sole-detected and
  dual-detected behavior, unknown-libc error, compatibility checks, and guarantee
  that it requests exactly one archive without silent fallback;
- that distribution names, container names, and Docker availability do not
  determine libc selection;
- that RubyGems, PyPI, and NuGet use their ecosystem-native platform resolution
  without a conflicting second selector, while npm and pnpm use package `libc`
  metadata;
- signed Git tag selection and immutable GitHub Release verification for source
  builds;
- crates.io, npm, RubyGems, PyPI, and NuGet installation, upgrade, and uninstall
  commands;
- version-pinned direct downloads and POSIX/PowerShell installers;
- independent trust-root acquisition, installer authentication, signed-manifest
  verification, key rotation and emergency revocation, and manual archive
  SHA-256 verification;
- the distinction between platform support, release authenticity, and public
  native-signer trust;
- Windows self-signed Authenticode fingerprint and RFC 3161 timestamp
  verification, the expected absence of verified-publisher and SmartScreen
  reputation claims, and the later CA-trusted mode only when it actually exists;
- macOS ad hoc signature verification, explicit non-notarized status, expected
  Gatekeeper implications, and Developer ID plus notarization only when that
  trust mode actually exists;
- safe manual verification of signed release metadata and hashes before a user
  makes any operating-system-mediated decision;
- Apple's user-mediated `Open Anyway` procedure as the preferred macOS
  exception, when applicable;
- a clearly warned, exact-path `xattr -d com.apple.quarantine` fallback only
  when target-native tests prove it necessary for the standalone CLI, and only
  after authenticity and integrity verification;
- an explicit prohibition on automatic quarantine removal, `xattr -c`,
  `xattr -cr`, recursive attribute removal, or instructions that clear unrelated
  extended attributes;
- custom-domain, immutable-path, and stable-pointer semantics;
- that npm, RubyGems, PyPI, and NuGet perform no secondary binary download;
- that PyPI provides platform wheels only and no importable Python API;
- that NuGet provides a pointer package plus RID-specific .NET Tool packages,
  uses a transport-only launcher only when required by the SDK contract, and
  provides no `PackageReference` or MSBuild integration;
- that public package-manager instructions install a globally invocable user
  command without requiring elevated privileges;
- that Cloudflare availability does not affect npm, gem, wheel, or .NET Tool
  installation;
- how to verify `liknon --version`;
- unsupported-platform and missing-package errors;
- security contact and provenance expectations for package incidents.

Update README, installation reference, security guidance, and changelog. Keep
each fact in one canonical location and link to it rather than duplicating
wording that can drift.

## Implementation Tasks

1. Add post-publication jobs for every supported channel and platform.
2. Add clean uninstall checks for the first release and previous-to-current
   public upgrade checks for subsequent releases.
3. Verify production domain headers, bytes, metadata, signatures, and bucket
   protections.
4. Update and verify the stable pointer only after all channel tests pass.
5. Update all canonical installation and support documentation.
6. Search for stale channel counts, unsupported platforms, old package names,
   obsolete installation commands, or claims that self-signed or ad hoc
   artifacts have public operating-system trust.
7. Validate any macOS manual-exception instructions against the released bytes
   and prove that they appear only after cryptographic verification.
8. Preserve public evidence for the release without retaining credentials.

## Exit Criterion

Public instructions reproduce every tested installation, update, and uninstall
path, the stable pointer resolves only to a fully verified release, and no
documentation promises an unverified channel, runtime, platform, publisher
identity, certificate-chain trust, or notarization state.
