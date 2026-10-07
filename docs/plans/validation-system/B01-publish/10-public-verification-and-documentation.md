# Phase B01.10: Verify Public Installation And Documentation

[Back to the distribution plan](README.md)

## Purpose

Install the published release through every advertised channel, update the
stable direct pointer only after all public paths pass, and document exactly the
support that has been proven.

## Dependencies

- [Phase B01.9](09-publication-and-recovery.md) completed publication or reached
  a fully recovered consistent state.
- Every destination exposes the intended immutable version.
- Publication evidence and the public direct manifest are available.

## Public Installation Verification

Run in clean target environments:

- clone the tagged GitHub source and install it with locked Cargo;
- install the exact version from crates.io on the supported Rust baseline;
- install the npm root package globally through npm and pnpm on every supported
  platform using separate clean user prefixes;
- install the matching gem globally through RubyGems and verify the same
  platform resolution separately through Bundler;
- install the matching wheel as a globally invocable user command through the
  pip user scheme, pipx, and `uv tool` on every supported wheel platform;
- install the exact NuGet pointer package globally with .NET SDK 10 on every
  supported platform and confirm only the matching RID package is retrieved;
- download and verify a direct archive manually;
- run the versioned POSIX installer on supported Unix targets;
- run the versioned PowerShell installer on Windows.

Every installation runs `liknon --version`, `liknon --help`, and a representative
`liknon validate` fixture. Reported version and native hash must agree with the
release evidence.

## Download-Domain Verification

- Production uses the approved custom domain rather than `r2.dev`.
- Versioned objects return the intended content type, disposition, length,
  cache headers, and immutable semantics.
- `manifest.json`, `manifest.sig`, and `SHA256SUMS` agree after edge delivery.
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
- Git tag selection for source builds;
- crates.io, npm, RubyGems, PyPI, and NuGet installation and upgrade commands;
- version-pinned direct downloads and POSIX/PowerShell installers;
- manual archive, SHA-256, and signature verification;
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
2. Verify production domain headers, bytes, metadata, and bucket protections.
3. Update and verify the stable pointer only after all channel tests pass.
4. Update all canonical installation and support documentation.
5. Search for stale channel counts, unsupported platforms, old package names,
   and obsolete installation commands.
6. Preserve public evidence for the release without retaining credentials.

## Exit Criterion

Public instructions reproduce every tested installation path, the stable pointer
resolves only to a fully verified release, and no documentation promises an
unverified channel, runtime, or platform.
