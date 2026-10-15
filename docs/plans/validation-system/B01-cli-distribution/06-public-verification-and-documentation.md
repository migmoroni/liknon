# Phase B01.6: Verify Public Installation And Documentation

[Back to the CLI distribution plan](README.md)

## Purpose

Verify the actual public source and binary surfaces, publish the stable pointer
only after success, and make public instructions reproduce tested behavior.

## Dependencies

- [Phase B01.5](05-publication-and-recovery.md) completed publication or reached
  a recorded recoverable state.
- External readiness checkpoint [AP-6](accounts-and-permissions.md#ap-6) is
  complete.

## Public Installation Verification

From clean environments that cannot read private workflow artifacts:

- clone the canonical GitHub repository, fetch and verify the released signed
  tag, build or install with locked dependencies, and run representative CLI
  behavior;
- download and authenticate one manual archive for every supported target;
- download, authenticate, and execute the versioned POSIX and PowerShell
  installers against the production domain;
- verify version, help, schema output, and a representative validation without
  an ecosystem pack;
- exercise update and uninstall behavior where a previous public version
  exists;
- verify unsupported platforms and integrity failures remain explicit.

Public verification must not succeed by finding another `liknon` executable on
`PATH`.

## Download-Domain Verification

- Versioned objects return exact expected bytes and immutable cache headers.
- The custom domain presents valid TLS and the production origin.
- `r2.dev` is not documented or enabled as a production endpoint.
- `manifest.json`, signatures, checksums, installers, and archives have correct
  content types and cache behavior.
- Unknown immutable keys do not create stale negative-cache failures.
- Bucket-lock and overwrite protections remain active.

## Stable Channel Update

After every public check passes:

1. create `channels/stable.json` as a narrow pointer to the exact versioned
   manifest;
2. upload it through the Cloudflare publication boundary;
3. read it back through the custom domain;
4. resolve and authenticate its versioned manifest;
5. record the stable-pointer evidence.

A failed target or installation path leaves the previous stable pointer intact.

## Documentation Deliverables

Document in one canonical installation reference:

- source installation from an exact signed tag;
- the exact `apps/liknon-cli` source-install path;
- source update and uninstall behavior;
- authenticated POSIX and PowerShell installer usage;
- manual archive selection, signature verification, and hash verification;
- supported operating systems, architectures, Linux libc variants, and runtime
  baselines;
- automatic libc behavior and explicit override;
- user-owned default installation directories and `PATH` setup;
- release authenticity versus native publisher trust;
- honest Windows self-signed and macOS ad hoc/non-notarized behavior;
- safe, user-mediated macOS exception guidance where target-native testing
  requires it;
- unsupported-platform, unavailable-verifier, and integrity-error behavior;
- how to verify `liknon --version` and report a release-security incident;
- that GitHub does not host Liknon binary assets;
- that authored knowledge and skills are installed separately through B02
  ecosystem packages and are not contained in the CLI binary.

Update README, installation reference, security guidance, and changelog without
duplicating canonical wording that can drift.

## Implementation Tasks

1. Add post-publication source and binary verification jobs.
2. Verify production domain bytes, headers, signatures, and protections.
3. Update the stable pointer only after complete success.
4. Update canonical installation and support documentation.
5. Search for stale channel names, package-manager CLI instructions, obsolete
   targets, or overstated signing claims.
6. Preserve public evidence without retaining credentials.

## Exit Criterion

Public instructions reproduce every tested source, installer, and manual
download path. The stable pointer resolves only to a completely verified CLI
release, and documentation claims no unverified platform or trust state.
