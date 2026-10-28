# Security Policy

## Supported Versions

Security fixes are provided for the latest `0.5.x` release. This policy is
updated when another release line becomes supported.

## Reporting A Vulnerability

Report vulnerabilities privately through
[GitHub Security Advisories](https://github.com/migmoroni/liknon/security/advisories/new).
Do not open a public issue before coordinated disclosure.

Include, when available:

- the affected `liknon` version and operating system;
- the configuration and command needed to reproduce the behavior;
- expected and observed results;
- security impact and prerequisites;
- logs or a minimal repository that contains no secrets.

An acknowledgement, impact assessment, and disclosure timeline are coordinated
through the private advisory. Public disclosure occurs after a fix or an
agreed mitigation is available.

## Trust Boundary

`.liknon/validation/config.json` is trusted executable input. It chooses programs and
arguments that run with the caller's operating-system permissions. The absence
of an implicit shell reduces command-interpretation risk but does not make an
untrusted configuration safe.

Repository mutation detection is an integrity signal for Git-visible files. It
is not a sandbox, permission boundary, malware scanner, or complete record of
filesystem and network side effects. Review third-party configuration before
execution and use operating-system isolation when the source is not trusted.

## Release Distribution Trust

Validation CLI releases bind one package-derived version to a signed annotated
Git tag, exact source commit, committed lockfile, source package, and native
artifact hashes. GitHub Releases contain metadata only. Native archives are
served through the project-controlled download domain and authenticated by a
detached RSA/SHA-256 signature over the exact public manifest.

Trust the manifest public key only after obtaining it from an exact verified
source tag. Authenticate installers before execution, then let the installer
authenticate the manifest and archive. Never continue after a signature,
fingerprint, hash, target, or identity mismatch.

Windows initially uses a persistent project-controlled self-signed
Authenticode certificate with an RFC 3161 timestamp; the installer does not add
it to a trust store. macOS binaries are ad hoc signed and not notarized. Linux
claims no native publisher signature. These platform signals supplement, but
do not replace, signed release metadata and exact hashes.

Production signing keys, PFX material, registry tokens, and Cloudflare
credentials are isolated in protected publication environments and never
stored in repository files or workflow artifacts.
