# Plan B01: Liknon CLI Distribution

## Status And Relationship To The Validation System

This directory defines the distribution track for the Liknon command-line
application after [Plan A01](../A01-product-and-repository-separation/README.md)
separates it from the core and authored ecosystem content. It is independent
from the numbered validation-system phases: it does not change runtime
behavior, configuration or report contracts, and it does not imply that
validation-system Phase 7 is complete.

The track may run for any commit otherwise approved for release. External
account and credential work is staged by the [accounts and permissions
runbook](accounts-and-permissions.md) immediately before the phase that needs
it.

Ecosystem content packages are planned in
[Plan B02](../B02-ecosystem-packs/README.md). Future in-process APIs and language
bindings are planned in
[Plan B03](../B03-programmatic-api-and-language-bindings/README.md). B01 defines
or publishes neither product.

## Objective

Distribute one canonical Liknon CLI through two complementary channels:

| Channel | Distributed form | User experience | Rust required |
| --- | --- | --- | --- |
| GitHub source | Signed tagged source for the complete canonical monorepo | Clone, select a version, and compile or install only `apps/liknon-cli` | Yes |
| Cloudflare Direct | Versioned native archive through a project-controlled domain | Run an authenticated installer or download and verify an archive manually | No |

GitHub is the source channel. Cloudflare is the binary channel. GitHub Releases
may lock release identity and carry metadata, but they do not host Liknon binary
assets.

## Product Boundary

B01 distributes the executable named `liknon`. It includes:

- one version and provenance contract for source and native artifacts;
- one native build per supported Rust target;
- platform-specific sealing and verification where applicable;
- private artifact transport between GitHub Actions jobs;
- immutable archives, signed metadata, checksums, and versioned installers;
- user-local installation without elevated privileges;
- public source-build and direct-binary verification;
- publication recovery and installation documentation.

The GitHub checkout contains all tracked monorepo sources, including authored
packs. B01 installs only the CLI member. Its executable and native archives
contain the generic validation engine, CLI presentation, core schemas, and
generic pack support approved by A01; they do not embed authored knowledge,
skills, recipes, or ecosystem package payloads.

B01 does not include:

- a public Rust library crate;
- a JavaScript, TypeScript, Ruby, Python, or .NET package;
- a language binding, SDK, native extension, or FFI contract;
- a package-manager launcher for the CLI;
- a shared, Rust, JavaScript/TypeScript, or other ecosystem pack;
- binary assets attached to GitHub Releases;
- a second implementation of validation behavior;
- compatibility aliases for an unpublished distribution model;
- changes to CLI, configuration, report, pack, or exit-code contracts.

## Installation Contract

### GitHub Source

Source users clone the canonical repository, select an exact signed release
tag, verify that tag, and install `apps/liknon-cli` with the locked dependency
graph and supported Rust toolchain. Documentation distinguishes an initial
installation from an update of an existing Cargo-installed executable and
never treats the default branch as a versioned release.

### Cloudflare Direct

Binary users either:

- authenticate and run the versioned POSIX or PowerShell installer; or
- download one exact platform archive and verify its signed release metadata
  and hashes before extracting it.

The default destination is user-owned and available on `PATH`. Installation
does not require `sudo`, mutate an operating-system trust store, weaken a host
security policy, or invoke Cargo.

The project website may expose the same immutable Cloudflare objects. It is a
presentation surface, not a third artifact source.

## Architectural Invariants

1. The root virtual workspace `Cargo.toml` is the sole authored CLI
   release-version source; the CLI package inherits it.
2. The initial release grammar is canonical `MAJOR.MINOR.PATCH`, without
   prerelease or build metadata.
3. A signed annotated tag `v<VERSION>` points exactly to the released commit.
4. A metadata-only immutable GitHub Release locks that source identity and
   contains no uploaded Liknon binaries.
5. CI builds the executable once for each supported Rust target. Later jobs may
   copy verified bytes but may not rebuild or modify them.
6. Platform sealing occurs after all binary transformations and before final
   hashing. Windows initially uses the approved self-signed Authenticode mode;
   macOS initially uses ad hoc signing and is explicitly not notarized.
7. The direct archive contains the exact final native bytes recorded by the
   internal build manifest.
8. Linux GNU and MUSL artifacts remain distinct for each architecture. A
   Liknon-owned automatic selector chooses GNU when both implementations are
   detected, selects the sole detected implementation, and fails before
   download when neither can be identified. An explicit supported selection
   takes precedence.
9. Versioned Cloudflare keys are immutable. Mutable channel pointers are
   separate objects and never define version identity.
10. Installers authenticate the exact public manifest before trusting any of
    its fields, then verify both archive and extracted-binary hashes.
11. Unsupported platforms fail explicitly and never trigger source compilation
    or an alternative binary download.
12. Build and verification jobs have no publication credentials. Signing and
    Cloudflare publication use separate protected identity boundaries.
13. Generated binaries, manifests, installers, and archives remain ignored and
    are never committed.
14. A partial publication resumes from exact retained artifacts. If their
    identity cannot be proven, a new patch version is required.
15. CLI build and archive inventories reject authored pack content and
    repository-only plans, fixtures, evaluations, and editorial tools. This
    does not claim those tracked sources are absent from a full GitHub clone.

## Proposed Repository Layout

```text
distribution/
└── cli/
    └── direct/
        ├── install.sh
        ├── install.ps1
        └── public-key.txt

.github/workflows/
└── release.yml

target/distribution/               # generated and ignored
├── native/
├── direct/
│   ├── archives/
│   ├── installers/
│   ├── manifest.json
│   ├── manifest.sig
│   └── SHA256SUMS
└── build-manifest.json
```

Authored installer templates and public trust material are version-neutral.
Release-specific files are staged only under `target/distribution/`.

## Initial Platform Contract

| Rust target | Native filename | Direct archive | Initial platform authenticity |
| --- | --- | --- | --- |
| `x86_64-unknown-linux-gnu` | `liknon` | `.tar.gz` | Signed release metadata and hashes |
| `x86_64-unknown-linux-musl` | `liknon` | `.tar.gz` | Signed release metadata and hashes |
| `aarch64-unknown-linux-gnu` | `liknon` | `.tar.gz` | Signed release metadata and hashes |
| `aarch64-unknown-linux-musl` | `liknon` | `.tar.gz` | Signed release metadata and hashes |
| `x86_64-apple-darwin` | `liknon` | `.tar.gz` | Ad hoc code signature; not notarized |
| `aarch64-apple-darwin` | `liknon` | `.tar.gz` | Ad hoc code signature; not notarized |
| `x86_64-pc-windows-msvc` | `liknon.exe` | `.zip` | Project-controlled self-signed Authenticode with RFC 3161 timestamp |

Every row is a distinct artifact. A target becomes public only after its native
execution, archive, installer, and public-readback checks pass. Platform
support and public trust in a native signer remain separate claims.

## Implementation Order

| Phase | Document | Depends on | Primary result |
| ---: | --- | --- | --- |
| B01.0 | [Public Contract](00-public-contract.md) | A01.4 and release-ready source | Approved source, binary, platform, trust, and ownership contracts |
| B01.1 | [Version And Provenance](01-version-and-provenance.md) | B01.0 | Cargo-derived version authority and deterministic manifests |
| B01.2 | [Native Builds](02-native-builds.md) | B01.1 | One tested final native binary per target |
| B01.3 | [Cloudflare Direct](03-cloudflare-direct.md) | B01.2 | Verified archives, metadata, and installers |
| B01.4 | [End-To-End Verification](04-end-to-end-verification.md) | B01.3 | Credential-free source and binary installation evidence |
| B01.5 | [Publication And Recovery](05-publication-and-recovery.md) | B01.4 | Isolated, resumable GitHub and Cloudflare publication |
| B01.6 | [Public Verification And Documentation](06-public-verification-and-documentation.md) | B01.5 | Verified public installs, stable pointer, and accurate documentation |

A phase is complete only when its own exit criterion and corresponding external
checkpoint pass.

## Global Acceptance Criteria

- [ ] Source and binary channels use the exact same version and commit.
- [ ] The signed tag and metadata-only immutable GitHub Release contain no
      uploaded native binary assets.
- [ ] Clone-based installation is verified from an exact signed tag in an
      isolated environment using `apps/liknon-cli`.
- [ ] Exactly one final native artifact is retained per declared target.
- [ ] Linux x64 and arm64 each have separate GNU and MUSL artifacts.
- [ ] Windows and macOS evidence describes their actual initial trust modes
      without claiming public publisher trust or notarization.
- [ ] Every archive and installer is authenticated by the signed public
      manifest and exact hashes.
- [ ] Direct installation succeeds without Rust or elevated privileges.
- [ ] Automatic and explicit Linux libc selection follow the public contract
      and request exactly one archive.
- [ ] Unsupported or unverifiable environments fail before installation.
- [ ] Versioned Cloudflare objects cannot be overwritten.
- [ ] Publication credentials are isolated by job and destination.
- [ ] A failed installation preserves the previously installed executable.
- [ ] Public documentation reproduces the exact tested source, script, and
      manual-download paths.
- [ ] Generated distribution state leaves tracked files unchanged.
- [ ] The installed CLI and native archive contain no authored pack, knowledge,
      skill, recipe, or ecosystem-package payload, while source instructions
      accurately describe the full monorepo checkout.

## Normative References

- [Cargo install](https://doc.rust-lang.org/cargo/commands/cargo-install.html)
- [Git tag signing](https://git-scm.com/book/en/v2/Git-Tools-Signing-Your-Work)
- [GitHub immutable releases](https://docs.github.com/en/code-security/concepts/supply-chain-security/immutable-releases)
- [GitHub artifact attestations](https://docs.github.com/en/actions/security-guides/using-artifact-attestations-to-establish-provenance-for-builds)
- [SLSA provenance](https://slsa.dev/spec/v1.0/provenance)
- [Cloudflare R2 authentication](https://developers.cloudflare.com/r2/api/tokens/)
- [Cloudflare R2 public buckets](https://developers.cloudflare.com/r2/buckets/public-buckets/)
- [Cloudflare R2 consistency](https://developers.cloudflare.com/r2/reference/consistency/)
- [Cloudflare R2 bucket locks](https://developers.cloudflare.com/r2/buckets/bucket-locks/)
- [Microsoft SignTool](https://learn.microsoft.com/en-us/windows/win32/seccrypto/signtool)
- [Apple code-signing requirements](https://developer.apple.com/library/archive/documentation/Security/Conceptual/CodeSigningGuide/RequirementLang/RequirementLang.html)

## Resulting System

Liknon remains one lean command-line application with one executable contract.
GitHub users build `apps/liknon-cli` from a selected source revision; users who
do not want a Rust toolchain obtain one authenticated native artifact from
Cloudflare. Ecosystem packs and language libraries remain separate products
and cannot silently become alternate CLI distribution channels.
