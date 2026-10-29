# Plan B01: Liknon Validation CLI Distribution

## Status And Relationship To The Validation System

The repository-side release implementation is in place. Product acceptance
still requires the human-owned AP-0 through AP-6 setup, an exact signed release
tag, and a release-ready candidate. The first protected hosted release and
public-channel verification are coordinated by
[B06](../B06-coordinated-public-release/README.md) after B02-B05 provide the
other selected release inputs. A01 has consolidated the validation engine,
schemas, CLI adaptation, and binary targets into the single
`liknon-validator` package.

The [validation-system index](../README.md) owns cross-track order, milestone
gates, and contract ownership. This plan owns only Validation CLI distribution.
The [shared CLI distribution foundation](../cli-distribution-foundation.md)
owns the cross-product release mechanisms. B01 is their first production
implementation and Validation is the first product profile.

This directory defines distribution for the validation command-line
application after [A01](../A01-product-and-repository-separation/README.md)
separates it from resource management and local resource tooling. It does not
change validation behavior, configuration, report, or exit contracts.

[B02](../B02-resource-hub-and-manager/README.md) defines the resource Hub and
`liknon-manager`. [B03](../B03-tool-cli/README.md) distributes the independent,
offline `liknon-tool`.
[B04](../B04-unified-launcher-and-cli-suite/README.md) owns the aggregate
launcher and complete-suite distribution. Future in-process APIs belong to
[B05](../B05-programmatic-api-and-language-bindings/README.md).
[B06](../B06-coordinated-public-release/README.md) freezes and activates the
first public multi-product release without taking ownership of this product's
workflow or credentials.

## Dependencies

B01 depends on the approved A01 ledger, a stable Validation CLI package
boundary, release-ready source, and the destination accounts and permissions
listed in the [B01 external-readiness runbook](accounts-and-permissions.md). It
also conforms to the shared CLI distribution foundation. Before producing its
final crates.io candidate, B01 consumes the completed
[A02 shared CLI experience foundation](../A02-shared-cli-experience-foundation/README.md)
and the exact accepted `liknon-cli-ui` source candidate. Prepublication uses a
controlled local-registry rehearsal; B06 publishes and reads back that
supporting version before the dependent Validation package. B01 does not depend
on C02, C03, or D01.

## Objective

Implement the reusable release foundation with Validation as its first real
profile, then distribute one canonical validation application through three
complementary channels:

| Channel | Distributed form | User experience | Rust required |
| --- | --- | --- | --- |
| GitHub source | Signed tagged source for the canonical monorepo | Clone, select a version, and install `apps/liknon-validator` | Yes |
| crates.io | Source application package `liknon-validator` | `cargo install liknon-validator --locked` | Yes |
| Cloudflare Direct | Versioned native archive through a project-controlled domain | Run an authenticated installer or verify and extract an archive | No |

GitHub is the complete source channel, crates.io is the application source
package channel, and Cloudflare is the native binary channel. GitHub Releases
lock source identity and carry metadata but no Liknon binary assets.

## Product Boundary

Every channel installs two command names backed by one implementation:

- `liknon-validator`, the explicit long command;
- `likv`, the compact command.

B01 includes:

- product-neutral release schemas, scripts, installer logic, reusable
  credential-free workflows, and conformance fixtures required by the shared
  distribution foundation;
- one explicit Validation product descriptor and thin release orchestrator;
- one version and provenance contract for source and native artifacts;
- one inspected, self-contained `liknon-validator` source package and its locked
  registry dependency closure;
- one final native executable payload per supported target;
- two equivalent installed command names;
- platform-specific sealing and verification where applicable;
- private artifact transport between GitHub Actions jobs;
- immutable archives, signed metadata, checksums, and installers;
- user-local installation without elevated privileges;
- public verification, publication recovery, and documentation.

The application contains the generic validation engine, Validation-specific CLI
presentation, core schemas, and generic `.liknon/validation/` initialization.
It consumes product-neutral terminal presentation from `liknon-cli-ui` without
transferring Validation behavior to that crate. It does not contain:

- `liknon-manager`, `liknon-tool`, the resource Hub, or installed resources;
- knowledge, skills, workflows, agent profiles, tool descriptors, prompts,
  guides, templates, or collections;
- npm, RubyGems, PyPI, or NuGet packages;
- a programmatic validation binding or SDK;
- binary assets attached to GitHub Releases;
- a second validation implementation;
- a knowledge command namespace or content-activation behavior.

## Exclusions

B01 does not build or publish Manager, Tool, the launcher, complete suites, Hub
resources, official workflows, programmatic bindings, or integrated
agent-evaluation results. It also does not coordinate the complete first public
release. Shared release machinery contains no behavior or publication
authority for those products. Their profiles and evidence remain with B02-B05,
B06, and D01.

## Installation Contract

### GitHub Source

Clone the canonical repository, select and verify an exact signed
`liknon-validator-v<VERSION>` tag, and install `apps/liknon-validator` with the committed
lockfile and supported Rust toolchain.

### crates.io

Install exact published source for package `liknon-validator` with locked
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

1. The `apps/liknon-validator` Cargo package version is the sole authored
   validation-CLI version source.
2. The initial version grammar is canonical `MAJOR.MINOR.PATCH`.
3. Signed tag `liknon-validator-v<VERSION>` points exactly to the released commit.
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
16. The crates.io candidate is built and installed from its extracted package
    with registry dependencies and no unpublished workspace package.
17. Common release primitives contain no Validation behavior, hard-coded
    Validation identity, or Validation publication credential.
18. Validation-specific package names, commands, tags, tests, namespaces,
    approvals, and recovery owners enter through its reviewed product profile.
19. B02-B04 may consume common primitives but cannot consume B01 publication
    authority or silently reuse a Validation candidate.
20. Once another profile adopts a common primitive, changes to that primitive
    run conformance for every adopted profile.

## Proposed Repository Layout

```text
distribution/cli/
├── common/
│   ├── schemas/
│   ├── scripts/
│   └── installers/
└── validation/
    ├── product.json
    ├── direct/
    │   ├── README.md
    │   └── public-key.pem              # Provisioned at AP-3
    └── smoke/
        ├── verify.sh
        └── verify.ps1

.github/workflows/
├── _cli-native-candidate.yml
├── _cli-direct-assembly.yml
├── _cli-public-verification.yml
└── release-validation.yml

target/distribution/validation/      # Generated and ignored
├── crates/
├── native/
├── direct/
└── evidence/
```

## Initial Platform Contract

| Rust target | Installed commands | Direct archive | Initial platform authenticity |
| --- | --- | --- | --- |
| `x86_64-unknown-linux-gnu` | `liknon-validator`, `likv` | `.tar.gz` | Signed release metadata and hashes |
| `x86_64-unknown-linux-musl` | `liknon-validator`, `likv` | `.tar.gz` | Signed release metadata and hashes |
| `aarch64-unknown-linux-gnu` | `liknon-validator`, `likv` | `.tar.gz` | Signed release metadata and hashes |
| `aarch64-unknown-linux-musl` | `liknon-validator`, `likv` | `.tar.gz` | Signed release metadata and hashes |
| `x86_64-apple-darwin` | `liknon-validator`, `likv` | `.tar.gz` | Ad hoc code signature; not notarized |
| `aarch64-apple-darwin` | `liknon-validator`, `likv` | `.tar.gz` | Ad hoc code signature; not notarized |
| `x86_64-pc-windows-msvc` | `liknon-validator.exe`, `likv.exe` | `.zip` | Project-controlled self-signed Authenticode with RFC 3161 timestamp |

Every row is a distinct artifact. A target becomes public only after native
execution, archive, installer, alias-equivalence, and public-readback checks.

## Deliverables And Implementation Order

| Phase | Document | Depends on | Primary result |
| ---: | --- | --- | --- |
| B01.0 | [Public Contract](00-public-contract.md) | A01.5 and release-ready source | Approved shared/product split plus Validation source, native, platform, trust, and ownership contracts |
| B01.1 | [Version And Provenance](01-version-and-provenance.md) | B01.0 | Product-neutral descriptor/manifests and package-derived Validation identity |
| B01.2 | [Native Builds](02-native-builds.md) | B01.1 | Reusable credential-free build lane and one tested Validation payload per target |
| B01.3 | [Cloudflare Direct](03-cloudflare-direct.md) | B01.2 | Reusable archive/installer assembly and verified Validation direct candidates |
| B01.4 | [End-To-End Verification](04-end-to-end-verification.md) | B01.3 | Shared conformance harness plus Validation source, crates, and native evidence |
| B01.5 | [Publication And Recovery](05-publication-and-recovery.md) | B01.4 | Isolated, resumable product publication workflow ready for B06 invocation |
| B01.6 | [Public Verification And Documentation](06-public-verification-and-documentation.md) | B01.5 | Public-readback, stable-pointer, and documentation procedure handed to B06 |

B01.5 and B01.6 remain the product-owned publication and verification
contracts. For the first public release, B06 invokes them with the exact B01
candidate recorded in its frozen release-set ledger. Later Validation releases
may use the same contracts independently.

## Global Acceptance Criteria

- [ ] All channels use the exact same version and commit.
- [ ] The signed tag and metadata-only GitHub Release contain no native binary
      assets.
- [ ] GitHub and crates.io source installations pass in isolated environments.
- [ ] The crates.io candidate has no unpublished workspace package or
      repository-local path dependency.
- [ ] The exact `liknon-cli-ui` dependency passes local-registry rehearsal, and
      B06 publishes and reads it back before publishing the Validation package.
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
- [ ] Shared release primitives accept a validated product descriptor and
      contain no Validation-specific behavior or publication authority.
- [ ] The Validation orchestrator is thin and all Validation-specific values
      are explicit in its profile, tests, or protected environments.
- [ ] Shared-foundation fixture tests prove another product identity cannot
      read, overwrite, or publish Validation state.

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
After B06 activates its first release, users may compile it from a verified
clone, install its source package from crates.io, or obtain one authenticated
native target from Cloudflare without receiving the independent Manager, Tool,
Hub, or modular resources.

The repository also gains one product-neutral release foundation. Later plans
instantiate it under independent product profiles rather than copying B01 or
coupling their releases to Validation.
