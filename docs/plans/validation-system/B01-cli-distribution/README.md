# Plan B01: Liknon Validation CLI Distribution

## Status And Relationship To The Validation System

This directory defines distribution for the validation command-line
application after [A01](../A01-product-and-repository-separation/README.md)
separates it from the core, knowledge application, and AI resources. It does
not change validation behavior, configuration, report, or exit contracts.

[B02](../B02-knowledge-cli-and-corpus/README.md) distributes the independent
knowledge application. [B03](../B03-ecosystem-ai-packages/README.md) publishes
non-CLI ecosystem packages. [B04](../B04-programmatic-api-and-language-bindings/README.md)
governs future in-process APIs.

## Objective

Distribute one canonical validation application through three complementary
channels:

| Channel | Distributed form | User experience | Rust required |
| --- | --- | --- | --- |
| GitHub source | Signed tagged source for the canonical monorepo | Clone, select a version, and install `apps/liknon-cli` | Yes |
| crates.io | Source application package `liknon-cli` | `cargo install liknon-cli --locked` | Yes |
| Cloudflare Direct | Versioned native archive through a project-controlled domain | Run an authenticated installer or verify and extract an archive | No |

GitHub is the complete source channel, crates.io is the application source
package channel, and Cloudflare is the native binary channel. GitHub Releases
lock source identity and carry metadata but no Liknon binary assets.

## Product Boundary

Every channel installs two command names backed by one implementation:

- `liknon`, the canonical user command;
- `liknon-cli`, the explicit product command.

B01 includes:

- one version and provenance contract for source and native artifacts;
- one inspected crates.io source package;
- one final native executable payload per supported target;
- two equivalent installed command names;
- platform-specific sealing and verification where applicable;
- private artifact transport between GitHub Actions jobs;
- immutable archives, signed metadata, checksums, and installers;
- user-local installation without elevated privileges;
- public verification, publication recovery, and documentation.

The application contains the generic validation engine, CLI presentation, core
schemas, and generic `.validation/` bootstrap. It does not contain:

- the `liknon-knowledge` application or knowledge corpus;
- skills, prompts, AI-Tool or AI-Engineering resources;
- npm, RubyGems, PyPI, or NuGet packages;
- a programmatic validation binding or SDK;
- binary assets attached to GitHub Releases;
- a second validation implementation;
- a knowledge command namespace or content-activation behavior.

## Installation Contract

### GitHub Source

Clone the canonical repository, select and verify an exact signed
`liknon-cli-v<VERSION>` tag, and install `apps/liknon-cli` with the committed
lockfile and supported Rust toolchain.

### crates.io

Install exact published source for package `liknon-cli` with locked
dependencies. The package contains both binary targets and builds from its
published contents without repository-only files or unpublished path
dependencies.

### Cloudflare Direct

Authenticate and run the versioned POSIX or PowerShell installer, or download
one exact target archive and verify its signed metadata and hashes. The
installer places both command names in one user-owned destination on `PATH`.
It does not require `sudo`, Cargo, trust-store mutation, or weakened host
security.

The website may expose the same immutable Cloudflare objects. It is a
presentation surface, not another artifact source.

## Architectural Invariants

1. The `apps/liknon-cli` Cargo package version is the sole authored
   validation-CLI version source.
2. The initial version grammar is canonical `MAJOR.MINOR.PATCH`.
3. Signed tag `liknon-cli-v<VERSION>` points exactly to the released commit.
4. A metadata-only immutable GitHub Release locks that source identity.
5. Both command names execute one implementation and report one version.
6. CI builds the native payload once per target. Later jobs may copy verified
   bytes but do not rebuild or modify them.
7. crates.io packages source; Cloudflare packages final target-native bytes.
8. Windows initially uses the approved self-signed Authenticode mode; macOS
   uses ad hoc signing and is explicitly not notarized.
9. Linux GNU and MUSL artifacts remain distinct for each architecture. GNU is
   selected when both implementations are detected.
10. Installers authenticate the public manifest before trusting any field and
    verify archive and extracted command hashes.
11. Versioned Cloudflare keys and accepted crates.io versions are immutable.
12. Build and verification jobs have no publication credentials.
13. Generated artifacts remain ignored.
14. Partial publication resumes only from exact retained candidates.
15. Application artifacts exclude knowledge, AI resources, plans, fixtures,
    evaluations, and editorial tooling.

## Proposed Repository Layout

```text
distribution/cli/
└── validation/
    ├── crates/
    └── direct/
        ├── install.sh
        ├── install.ps1
        └── public-key.txt

.github/workflows/
└── release-cli.yml

target/distribution/validation/      # Generated and ignored
├── crates/
├── native/
├── direct/
└── evidence/
```

## Initial Platform Contract

| Rust target | Installed commands | Direct archive | Initial platform authenticity |
| --- | --- | --- | --- |
| `x86_64-unknown-linux-gnu` | `liknon`, `liknon-cli` | `.tar.gz` | Signed release metadata and hashes |
| `x86_64-unknown-linux-musl` | `liknon`, `liknon-cli` | `.tar.gz` | Signed release metadata and hashes |
| `aarch64-unknown-linux-gnu` | `liknon`, `liknon-cli` | `.tar.gz` | Signed release metadata and hashes |
| `aarch64-unknown-linux-musl` | `liknon`, `liknon-cli` | `.tar.gz` | Signed release metadata and hashes |
| `x86_64-apple-darwin` | `liknon`, `liknon-cli` | `.tar.gz` | Ad hoc code signature; not notarized |
| `aarch64-apple-darwin` | `liknon`, `liknon-cli` | `.tar.gz` | Ad hoc code signature; not notarized |
| `x86_64-pc-windows-msvc` | `liknon.exe`, `liknon-cli.exe` | `.zip` | Project-controlled self-signed Authenticode with RFC 3161 timestamp |

Every row is a distinct artifact. A target becomes public only after native
execution, archive, installer, alias-equivalence, and public-readback checks.

## Implementation Order

| Phase | Document | Depends on | Primary result |
| ---: | --- | --- | --- |
| B01.0 | [Public Contract](00-public-contract.md) | A01.4 and release-ready source | Approved source, crates, native, platform, trust, and ownership contracts |
| B01.1 | [Version And Provenance](01-version-and-provenance.md) | B01.0 | Package-derived version authority and deterministic evidence |
| B01.2 | [Native Builds](02-native-builds.md) | B01.1 | One tested final native payload per target |
| B01.3 | [Cloudflare Direct](03-cloudflare-direct.md) | B01.2 | Verified native archives, metadata, and installers |
| B01.4 | [End-To-End Verification](04-end-to-end-verification.md) | B01.3 | Credential-free source, crates, and native installation evidence |
| B01.5 | [Publication And Recovery](05-publication-and-recovery.md) | B01.4 | Isolated, resumable crates.io, GitHub, and Cloudflare publication |
| B01.6 | [Public Verification And Documentation](06-public-verification-and-documentation.md) | B01.5 | Verified public installs, stable pointer, and accurate documentation |

## Global Acceptance Criteria

- [ ] All channels use the exact same version and commit.
- [ ] The signed tag and metadata-only GitHub Release contain no native binary
      assets.
- [ ] GitHub and crates.io source installations pass in isolated environments.
- [ ] Both command names are equivalent through every channel.
- [ ] Exactly one final native payload is retained per target.
- [ ] Linux x64 and arm64 each have separate GNU and MUSL artifacts.
- [ ] Windows and macOS claims describe their actual trust modes.
- [ ] Every archive and installer is authenticated by signed metadata and
      exact hashes.
- [ ] Direct installation succeeds without Rust or elevated privileges.
- [ ] Unsupported or unverifiable environments fail before installation.
- [ ] Publication credentials are isolated by destination.
- [ ] Failed installation preserves the previous commands.
- [ ] Public documentation reproduces all tested installation paths.
- [ ] Generated distribution state leaves tracked files unchanged.
- [ ] No application artifact contains knowledge or AI resources.

## Normative References

- [Cargo install](https://doc.rust-lang.org/cargo/commands/cargo-install.html)
- [Cargo publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [crates.io trusted publishing](https://crates.io/docs/trusted-publishing)
- [Git tag signing](https://git-scm.com/book/en/v2/Git-Tools-Signing-Your-Work)
- [GitHub immutable releases](https://docs.github.com/en/code-security/concepts/supply-chain-security/immutable-releases)
- [GitHub artifact attestations](https://docs.github.com/en/actions/security-guides/using-artifact-attestations-to-establish-provenance-for-builds)
- [SLSA provenance](https://slsa.dev/spec/v1.0/provenance)
- [Cloudflare R2 authentication](https://developers.cloudflare.com/r2/api/tokens/)
- [Cloudflare R2 public buckets](https://developers.cloudflare.com/r2/buckets/public-buckets/)
- [Cloudflare R2 bucket locks](https://developers.cloudflare.com/r2/buckets/bucket-locks/)
- [Microsoft SignTool](https://learn.microsoft.com/en-us/windows/win32/seccrypto/signtool)
- [Apple code-signing requirements](https://developer.apple.com/library/archive/documentation/Security/Conceptual/CodeSigningGuide/RequirementLang/RequirementLang.html)

## Resulting System

Liknon validation remains one lean application with two explicit command names.
Users may compile it from a verified clone, install its source package from
crates.io, or obtain one authenticated native target from Cloudflare without
receiving the independent knowledge or ecosystem AI products.
