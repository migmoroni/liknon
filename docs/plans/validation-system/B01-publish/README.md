# Plan B01: Multi-Channel CLI Distribution

## Status And Relationship To The Validation System

This directory defines the distribution track for the Liknon CLI. It is
independent from the numbered validation-system phases: it does not change
runtime behavior, configuration or report contracts, and it does not imply
that validation-system Phase 7 is complete.

The track may be executed for any commit otherwise approved for release. Its
purpose is to make one Rust implementation installable through the source,
package, and direct-download channels preferred by different users without
creating parallel implementations of Liknon.

## Objective

Publish the same Liknon CLI through seven channels:

| Channel | Distributed form | Installation work | Rust required on the user machine |
| --- | --- | --- | --- |
| GitHub | Tagged source repository | Clone, select a revision, and compile locally | Yes |
| crates.io | Cargo source package | Cargo downloads and compiles locally | Yes |
| npm | Root package plus platform package containing a native binary | Registry install only | No |
| RubyGems | Platform gem containing a native binary and a Ruby launcher | Registry install only | No |
| PyPI | Platform wheel containing the native binary as an installed script | Registry install only | No |
| NuGet | .NET Tool pointer package plus one RID package per supported platform | Registry install only | No |
| Cloudflare Direct | Versioned native archive served from R2 through a custom domain | Direct download or project installer | No |

GitHub and crates.io are source channels. npm, RubyGems, PyPI, and NuGet are
self-contained binary package channels. Cloudflare Direct is the independent
binary-download channel for users who do not want to clone source or install
through a language package manager. Public GitHub Release binaries are
deliberately outside this design.

## Scope

This track includes:

- one deterministic version contract shared by every channel artifact;
- one native build per supported Rust target;
- native platform sealing under an explicitly disclosed trust mode before final
  hashing where required by the public platform contract;
- release authenticity through signed metadata and hashes independently from
  operating-system trust in a native signer;
- private transport of built binaries between GitHub Actions jobs;
- immutable direct archives and user-local installers served through a
  Cloudflare R2 custom domain;
- self-contained npm, RubyGems, PyPI, and NuGet packages using those same native
  bytes;
- assembly, inspection, smoke tests, publication, recovery, and public
  installation verification;
- installation documentation for every supported channel.

## Global CLI Installation Contract

Binary package channels primarily install `liknon` as a globally invocable
user command rather than a project dependency. Global means installation into
an isolated, user-owned package-manager home or executable prefix available on
`PATH`; it never requires `sudo` or mutation of an operating-system-managed
runtime.

The supported installation frontends are:

- npm and pnpm global installation for the npm packages;
- `gem install` for global installation in the active user-owned Ruby
  environment, with Bundler additionally tested for platform resolution and
  execution compatibility;
- the pip user installation scheme, pipx, and `uv tool` for globally invocable
  Python-channel commands;
- `dotnet tool install --global` with .NET SDK 10 or later for NuGet.

Tests give each frontend a clean temporary home or prefix so channels cannot
pass by finding a Liknon command installed by another frontend. Project-local,
ephemeral, and package-runner forms may remain ecosystem capabilities, but
they are not the primary installation contract of this track.

Each primary frontend must also replace an older package through its supported
global update or upgrade operation and remove its command completely through
its uninstall operation. Local two-version fixtures prove those lifecycle
semantics before the first publication; public previous-to-current upgrade
tests begin when a second public version exists.

## Non-Goals

This track does not introduce:

- a TypeScript, JavaScript, Ruby, Python, or C# implementation of Liknon;
- a JavaScript, Ruby, Python, or .NET library API, native extension API, or FFI
  surface;
- binary assets attached to GitHub Releases;
- downloads from another host during npm, gem, wheel, or .NET Tool
  installation;
- `postinstall`, lifecycle compilation, Python build-time compilation, or a
  Cargo fallback in npm, gems, wheels, or the .NET Tool;
- a generic Ruby source gem or Python source distribution that compiles
  Liknon;
- Maturin, PyO3, CFFI, or another Rust-to-Python binding layer;
- a NuGet `PackageReference`, MSBuild props or targets, or `dotnet add package`
  integration; NuGet is a CLI installation channel in this track;
- a Cloudflare dependency for crates.io, npm, RubyGems, PyPI, or NuGet
  installations;
- an official GitHub Action or integrations for other CI providers in this
  initial track;
- a Worker or dynamic download service when static R2 objects and cache rules
  satisfy the direct-distribution contract;
- compatibility aliases for pre-release package or executable names;
- changes to Liknon's CLI, configuration schema, report schema, or exit codes.

## Architectural Invariants

1. `Cargo.toml` is the only authored version source.
2. The initial cross-registry version grammar is canonical SemVer core
   `MAJOR.MINOR.PATCH`, with no leading zeroes, prerelease suffix, or build
   metadata. Extending this grammar requires a reviewed mapping contract before
   a release uses it.
3. The release tag, direct metadata, `liknon --version`, Cargo package, npm
   packages, platform gems, platform wheels, and .NET Tool pointer and RID
   packages use that exact version text.
4. Every release tag is signed and locked by a metadata-only immutable GitHub
   Release. GitHub Release binary assets remain prohibited.
5. CI builds a native binary once for each Rust target. Distribution jobs may
   copy verified bytes but may not rebuild Liknon.
6. Native platform sealing occurs once after build and before final hashing.
   The initial Windows artifact uses a persistent project-controlled self-signed
   Authenticode identity plus an RFC 3161 timestamp using SHA-256. The initial
   macOS artifacts use ad hoc code signatures and are not notarized. These modes
   provide platform-local integrity evidence but do not claim public trust from
   the operating system. The resulting bytes are the canonical native artifacts
   reused by every binary channel.
7. Cloudflare Direct, npm, RubyGems, PyPI, and NuGet contain the same verified
   final native binary bytes for a given target.
8. Generated binaries, release-specific metadata and installers, staged package
   trees, and distribution archives are never committed.
9. npm and Ruby launchers, plus a .NET launcher when the Phase B01.0 prototype
   proves one necessary, contain transport logic only and preserve the CLI
   process contract. PyPI installs the native executable directly and adds no
   Python launcher or importable API.
10. npm, RubyGems, PyPI, and NuGet perform no secondary download and remain
    operational independently from Cloudflare.
11. Direct installers authenticate the exact public manifest before trusting
    its fields, resolve its signing-key identity through independently delivered
    public trust material, then verify archive and extracted-binary SHA-256
    values before installation. The trust contract defines key activation,
    rotation, retirement, and emergency revocation. Installers require no
    elevated privileges by default.
12. Unsupported platforms fail explicitly and never fall back to compilation or
    silently select another binary.
13. Versioned Cloudflare object keys are never overwritten. Mutable channel
    pointers are separate objects and are not version identity.
14. A published version is immutable. A partial release resumes from the exact
    retained local artifacts only after destination-specific equivalence is
    proven, or it is replaced by a new patch version.
15. Human publication approval is a credential-free gate. Detached release
    signing, Windows native signing, and each publication destination use
    separate protected environments or equivalently isolated identity
    boundaries, so no job can obtain another boundary's credential. macOS ad
    hoc signing carries no identity.

## Proposed Repository Layout

```text
distribution/
├── direct/
│   ├── install.sh
│   ├── install.ps1
│   └── public-key.txt
├── npm/
│   ├── root/
│   │   ├── package.json
│   │   └── bin/
│   │       └── liknon.js
│   └── platforms/
│       ├── linux-x64/package.json
│       ├── linux-arm64/package.json
│       ├── darwin-x64/package.json
│       ├── darwin-arm64/package.json
│       └── win32-x64/package.json
├── rubygems/
│   ├── liknon.gemspec
│   ├── exe/
│   │   └── liknon
│   └── lib/
│       └── liknon/
│           └── runner.rb
├── pypi/
│   └── metadata/                  # version-neutral wheel metadata templates
└── nuget/
    ├── Liknon.Tool.csproj
    └── launcher/
        └── Program.cs              # only when the prototype requires it

.github/workflows/
└── release.yml

target/distribution/                 # generated and ignored
├── native/
├── direct/
├── npm/
├── rubygems/
├── pypi/
├── nuget/
│   ├── pointer/
│   └── rids/
├── packages/
└── build-manifest.json
```

Committed package manifests and metadata are version-neutral development
templates. Distribution staging writes complete publishable manifests and
package trees under `target/distribution/` without mutating authored metadata
in the working tree.

## Initial Platform Contract

| Rust build target | Initial platform authenticity | Native filename | Direct archive suffix | npm package | RubyGems platform package(s) | PyPI wheel platform tag(s) | NuGet RID package(s) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `x86_64-unknown-linux-musl` | Not applicable; signed release metadata and hashes | `liknon` | `x86_64-unknown-linux-musl.tar.gz` | `@liknon/linux-x64` | `liknon` for `x86_64-linux-gnu` and `x86_64-linux-musl` | Approved x86_64 manylinux and/or musllinux tags | `liknon.linux-x64` and/or `liknon.linux-musl-x64` after independent proof |
| `aarch64-unknown-linux-musl` | Not applicable; signed release metadata and hashes | `liknon` | `aarch64-unknown-linux-musl.tar.gz` | `@liknon/linux-arm64` | `liknon` for `aarch64-linux-gnu` and `aarch64-linux-musl` | Approved aarch64 manylinux and/or musllinux tags | `liknon.linux-arm64` and/or `liknon.linux-musl-arm64` after independent proof |
| `x86_64-apple-darwin` | Ad hoc code signature; not notarized | `liknon` | `x86_64-apple-darwin.tar.gz` | `@liknon/darwin-x64` | `liknon` for `x86_64-darwin` | `macosx_<baseline>_x86_64` | `liknon.osx-x64` |
| `aarch64-apple-darwin` | Ad hoc code signature; not notarized | `liknon` | `aarch64-apple-darwin.tar.gz` | `@liknon/darwin-arm64` | `liknon` for `arm64-darwin` | `macosx_<baseline>_arm64` | `liknon.osx-arm64` |
| `x86_64-pc-windows-msvc` | Self-signed Authenticode with RFC 3161 timestamp | `liknon.exe` | `x86_64-pc-windows-msvc.zip` | `@liknon/win32-x64` | `liknon` for `x64-mingw-ucrt` | `win_amd64` | `liknon.win-x64` |

The package names are desired names until ownership and availability are
confirmed. A target enters the public matrix only after its native and packaged
forms pass the verification defined in Phase B01.8. Exact Linux wheel tags,
Linux NuGet RIDs, and the macOS deployment baseline are decisions of Phase
B01.0, not values inferred during packaging.

Platform support and public native-signer trust are separate claims. Windows
and macOS enter the initial matrix only after their declared self-signed and ad
hoc modes pass native execution and integrity tests. Neither mode is described
as a verified publisher, a trusted certificate chain, or notarized software.
The signed release manifest and its independently delivered trust root remain
the cross-platform authenticity authority.

## Implementation Order

| Phase | Document | Depends on | Primary result |
| ---: | --- | --- | --- |
| B01.0 | [Public Contract](00-public-contract.md) | Current release-ready source | Confirmed version grammar, channel identities, runtimes, immutable-tag enforcement, signing model, publishing identities, and platform feasibility |
| B01.1 | [Version And Provenance](01-version-and-provenance.md) | B01.0 | Cargo-derived version authority and deterministic manifest tooling |
| B01.2 | [Native Builds](02-native-builds.md) | B01.1 | One tested, final signed native binary where required per target and the completed build manifest |
| B01.3 | [Cloudflare Direct](03-cloudflare-direct.md) | B01.2 | Verified direct archives, metadata, and installers |
| B01.4 | [npm Packages](04-npm-packages.md) | B01.2 | Self-contained npm platform packages and root launcher |
| B01.5 | [RubyGems Packages](05-rubygems-packages.md) | B01.2 | Self-contained Ruby platform gems and launcher |
| B01.6 | [PyPI Wheels](06-pypi-wheels.md) | B01.2 | Self-contained platform wheels containing the verified native executable |
| B01.7 | [NuGet .NET Tool](07-nuget-dotnet-tool.md) | B01.2 | .NET Tool pointer package plus exact-version RID packages containing verified native assets |
| B01.8 | [End-To-End Verification](08-end-to-end-verification.md) | B01.3-B01.7 | Cross-channel byte identity, installation, update, and uninstall evidence |
| B01.9 | [Publication And Recovery](09-publication-and-recovery.md) | B01.8 | Trusted, isolated, resumable publication with destination-specific readback |
| B01.10 | [Public Verification And Documentation](10-public-verification-and-documentation.md) | B01.9 | Verified public installs, stable pointer, and accurate documentation |

Complete phases in order. B01.3 through B01.7 may be implemented in parallel
only after B01.2 is complete; B01.8 joins all five results. A phase is complete
only when its own exit criterion and applicable tests pass.

## Global Acceptance Criteria

- [ ] `Cargo.toml` is the sole authored release version.
- [ ] The version matches canonical `MAJOR.MINOR.PATCH` and requires no
      registry-specific normalization.
- [ ] The release tag, direct metadata, native CLI, and every package report the
      same version.
- [ ] The signed release tag is locked by a metadata-only immutable GitHub
      Release containing no uploaded binary assets.
- [ ] GitHub and crates.io are tested source-build channels.
- [ ] GitHub Releases are not used as a native-binary installation source.
- [ ] CI builds exactly one binary for each declared Rust target.
- [ ] Cloudflare Direct, npm, RubyGems, PyPI, and NuGet consume the same
      hash-verified binary per target.
- [ ] Windows and macOS native artifacts satisfy the platform-signing contract
      before final hashes are recorded and reused by every binary channel.
- [ ] Windows is described and verified as self-signed rather than publicly
      trusted; macOS is described and verified as ad hoc signed and not
      notarized.
- [ ] Every Windows Authenticode signature carries a verified RFC 3161 timestamp
      using SHA-256 before the final binary hash is recorded.
- [ ] No installer, launcher, or package imports a self-signed certificate into
      a user trust store or automatically disables, bypasses, or weakens
      SmartScreen, Gatekeeper, quarantine, or another operating-system security
      policy.
- [ ] macOS documentation prefers Apple's user-mediated `Open Anyway` path and
      permits a path-specific `xattr -d com.apple.quarantine` fallback only after
      signed-manifest and native-hash verification. It never recommends
      `xattr -c`, `xattr -cr`, broad recursion, or automatic quarantine removal.
- [ ] Distribution assembly jobs cannot rebuild Liknon.
- [ ] npm, gem, wheel, and .NET Tool installations require no Rust toolchain or
      secondary download.
- [ ] PyPI publishes platform wheels only, with no source distribution,
      Python launcher, or importable API.
- [ ] Direct installation requires no Rust, Node, Ruby, Python, .NET, or
      elevated privileges by default.
- [ ] Production downloads use a project-controlled custom domain rather than
      `r2.dev`.
- [ ] Versioned direct objects are immutable, cacheable, and protected from
      accidental overwrite or deletion.
- [ ] The public manifest is published only after all referenced objects are
      uploaded and read back successfully.
- [ ] Direct installers verify archive and extracted-binary hashes before
      replacing the destination.
- [ ] Direct installers verify the detached signature over the exact manifest
      bytes before parsing or trusting any manifest field.
- [ ] Public direct metadata has a documented detached signature and an
      independently available trust root.
- [ ] Direct-distribution trust material identifies keys stably and has tested
      activation, rotation, retirement, and emergency-revocation behavior.
- [ ] NuGet uses the .NET SDK 10 RID-specific tool model: platform packages are
      published before the pointer package and installation retrieves only the
      compatible RID package.
- [ ] NuGet is a .NET Tool rather than a project dependency or MSBuild
      integration, and any required managed launcher contains no validation
      logic.
- [ ] npm, pnpm, gem, Bundler, pip, pipx, uv, and global .NET Tool paths have
      isolated installation and execution evidence appropriate to their role.
- [ ] npm, pnpm, gem, pip, pipx, uv, and .NET Tool have isolated update and
      uninstall evidence without stale command shims.
- [ ] npm, RubyGems, PyPI, and NuGet do not depend on Cloudflare at installation
      or runtime.
- [ ] Launchers contain no validation logic and preserve CLI process behavior.
- [ ] Unsupported platforms fail explicitly without compilation fallback.
- [ ] Every final distribution artifact is inspected and tested before
      publication.
- [ ] Partial publication resumes from exact retained local artifacts only after
      destination-specific equivalence is proven.
- [ ] Human publication approval exposes no credential; detached release
      signing, Windows native signing, and each destination use isolated
      identity boundaries, macOS ad hoc signing carries no identity, and build
      jobs remain unprivileged.
- [ ] Registry publication uses OIDC or trusted publishing wherever supported;
      any unavoidable first-publication credential is narrowly scoped, audited,
      and revoked after bootstrap.
- [ ] Generated binaries, release metadata, installers, and archives remain
      untracked.
- [ ] Public documentation describes only verified channels and platforms.

## Normative References

- [Cargo `install`](https://doc.rust-lang.org/cargo/commands/cargo-install.html)
- [Cargo manifest version field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field)
- [Cargo package publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Python version specifiers](https://packaging.python.org/en/latest/specifications/version-specifiers/)
- [NuGet package versioning](https://learn.microsoft.com/en-us/nuget/concepts/package-versioning)
- [npm `package.json` reference](https://docs.npmjs.com/files/package.json/)
- [RubyGems platforms](https://guides.rubygems.org/platforms/)
- [RubyGems specification reference](https://guides.rubygems.org/specification-reference/)
- [Wheel binary package format](https://packaging.python.org/en/latest/specifications/binary-distribution-format/)
- [Python platform compatibility tags](https://packaging.python.org/en/latest/specifications/platform-compatibility-tags/)
- [.NET tools overview](https://learn.microsoft.com/en-us/dotnet/core/tools/global-tools)
- [Create a .NET tool](https://learn.microsoft.com/en-us/dotnet/core/tools/global-tools-how-to-create)
- [`dotnet tool install`](https://learn.microsoft.com/en-us/dotnet/core/tools/dotnet-tool-install)
- [RID-specific .NET tools](https://learn.microsoft.com/en-us/dotnet/core/tools/rid-specific-tools)
- [NuGet package publishing](https://learn.microsoft.com/en-us/nuget/nuget-org/publish-a-package)
- [NuGet signed packages](https://learn.microsoft.com/en-us/nuget/reference/signed-packages-reference)
- [GitHub immutable releases](https://docs.github.com/en/code-security/concepts/supply-chain-security/immutable-releases)
- [GitHub deployment environments](https://docs.github.com/en/actions/how-tos/deploy/configure-and-manage-deployments/control-deployments)
- [npm trusted publishing](https://docs.npmjs.com/trusted-publishers/)
- [RubyGems trusted publishing](https://guides.rubygems.org/trusted-publishing/)
- [PyPI trusted publishing](https://docs.pypi.org/trusted-publishers/)
- [crates.io trusted publishing](https://blog.rust-lang.org/2025/07/11/crates-io-development-update-2025-07/)
- [NuGet trusted publishing](https://learn.microsoft.com/en-us/nuget/nuget-org/trusted-publishing)
- [macOS Developer ID distribution](https://developer.apple.com/documentation/xcode/creating-distribution-signed-code-for-the-mac/)
- [macOS notarization](https://developer.apple.com/documentation/security/notarizing-macos-software-before-distribution)
- [macOS code-signing requirements and ad hoc signatures](https://developer.apple.com/library/archive/documentation/Security/Conceptual/CodeSigningGuide/RequirementLang/RequirementLang.html)
- [Apple guidance for safely opening unnotarized software](https://support.apple.com/en-us/102445)
- [Windows SignTool](https://learn.microsoft.com/en-us/windows/win32/seccrypto/signtool)
- [Authenticode time stamping](https://learn.microsoft.com/en-us/windows/win32/seccrypto/time-stamping-authenticode-signatures)
- [Windows code-signing options and self-signed limitations](https://learn.microsoft.com/en-us/windows/apps/package-and-deploy/code-signing-options)
- [Cloudflare R2 pricing](https://developers.cloudflare.com/r2/pricing/)
- [Cloudflare R2 public buckets](https://developers.cloudflare.com/r2/buckets/public-buckets/)
- [Cloudflare cache with R2](https://developers.cloudflare.com/cache/interaction-cloudflare-products/r2/)
- [Cloudflare R2 consistency](https://developers.cloudflare.com/r2/reference/consistency/)
- [Cloudflare R2 bucket locks](https://developers.cloudflare.com/r2/buckets/bucket-locks/)
- [Cloudflare R2 limits](https://developers.cloudflare.com/r2/platform/limits/)
- [GitHub Release storage and bandwidth](https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases)

These sources define package-manager and hosting mechanics. Liknon's stricter
rules, especially one build per target, immutable direct releases, no package
secondary download, exact cross-channel versions, and hash-verified reuse,
remain project policy.

## Resulting System

Liknon remains one Rust CLI with one behavior and one version. GitHub and
crates.io users compile trusted source. npm, RubyGems, PyPI, and NuGet users
receive the already-built binary from their selected registry. Direct users
receive the same verified bytes through the Cloudflare custom domain. CI builds
each Rust target once and publishes delivery bridges rather than parallel
implementations.
