# Phase B01.0: Confirm The Public Distribution Contract

[Back to the distribution plan](README.md)

## Purpose

Resolve every external identity and compatibility decision required by later
phases before implementation creates package templates, manifests, workflows,
or public promises.

This phase does not publish Liknon. It establishes the reviewed inputs that all
later phases must consume.

## Dependencies

- A Liknon commit otherwise suitable for release work.
- The shared invariants and platform matrix in the [plan
  index](README.md#architectural-invariants).
- Human ownership of the intended registry accounts, Cloudflare account, and
  download domain.

## Channel Contract

### GitHub Source

GitHub provides source from signed annotated release tags. Each release tag is
locked by publishing a metadata-only GitHub Release with immutable releases
enabled; no native binary is uploaded as a GitHub Release asset. A user choosing
this channel builds locally:

```sh
git clone https://github.com/migmoroni/liknon.git
cd liknon
git checkout "v<VERSION>"
cargo install --path . --locked
```

Installing another branch or commit is an explicit source-build choice. Liknon
does not attach native installers to GitHub Releases. This is an architectural
choice rather than an assumption that GitHub currently charges for Release
download bandwidth.

The repository must prove that immutable releases are enabled before the first
release. A tag is not described as immutable merely because it exists. The
release workflow verifies the tag signature, exact commit, and repository
immutability setting before any destination write.

### crates.io Source

crates.io distributes the Cargo source package:

```sh
cargo install "liknon@<VERSION>" --locked
```

This path requires a compatible Rust toolchain and compiles locally. The Cargo
package must contain every runtime source, schema, knowledge document, skill,
license, and branding asset required by the crate.

### Cross-Registry Version Grammar

The initial public contract accepts only canonical SemVer core versions:

```text
MAJOR.MINOR.PATCH
```

Each numeric identifier is `0` or begins with a nonzero digit. Prerelease
identifiers and build metadata are excluded because supported registries use
different version grammars and normalization rules. Every registry receives the
exact text returned by Cargo metadata. Supporting another version shape later
requires a reviewed cross-registry mapping and identity contract; release code
must not invent one during publication.

### npm Binary Packages

The desired root package is `liknon`, supported by exact-version platform
packages under the desired `@liknon` scope. Name and scope availability remain
unconfirmed until checked against the registry and assigned to an accountable
owner. Global installation is verified with both npm and pnpm in isolated
user-owned prefixes.

### RubyGems Binary Packages

The desired gem name is `liknon`, published as platform-specific variants of
the same version. Name ownership and every intended RubyGems platform identifier
must be confirmed before gem templates are treated as public contracts.
`gem install` is the global installation path; Bundler additionally proves that
the same platform variants resolve and execute correctly in a bundle.

### PyPI Binary Wheels

The desired PyPI project name is `liknon`. It publishes platform-specific
wheels of the same version and does not publish a source distribution. Each
wheel installs the verified native executable directly as a script; it does not
expose an importable Python API or invoke a Rust build backend.

Project-name ownership, supported Python installer baseline, exact platform
tags, and native-script behavior on every target must be confirmed before wheel
metadata is treated as a public contract. A single native binary may feed more
than one Linux wheel envelope only when every advertised tag is independently
validated. The pip user scheme, pipx, and `uv tool` are the supported
globally-invocable installation frontends.

### NuGet .NET Tool

The desired top-level NuGet package ID is `liknon`, with package type
`DotnetTool` and tool command name `liknon`. The channel requires .NET SDK 10.0
or later and uses its RID-specific tool packaging model. The top-level package
is a pointer; each exact-version RID package contains only the
manifest-verified Rust binary for that platform plus any transport entry point
proved necessary by the Phase B01.0 prototype.

The .NET CLI infers the host RID during global installation and retrieves only
the matching RID package. Every RID package has the same version as the pointer
package, all RID packages are published first, and no RID-agnostic `any`
fallback is published. Unsupported platforms fail explicitly.

The prototype must first determine whether the .NET SDK 10 tool contract can
expose the prebuilt Rust executable directly. If it cannot, each RID package
adds the smallest possible `net10.0` managed launcher. That launcher only
locates its co-packaged binary and preserves the CLI process contract; it
contains no validation behavior.

This channel is installed with `dotnet tool install`, not
`dotnet add package`. It contributes no `PackageReference`, MSBuild props,
targets, library API, or transitive project dependency. It performs no
secondary download and does not require Rust or Cargo on the user machine.

Top-level and generated RID package-name ownership, exact supported RIDs,
global tool behavior, direct-entry-point feasibility, conditional launcher
shape, package-size limits, and Unix executable-permission handling must be
proven before NuGet metadata is treated as a public contract.

### Cloudflare Direct Binary

Cloudflare Direct uses an R2 bucket connected to a project-controlled custom
domain. The final domain is selected in this phase. `r2.dev` may support local
experiments but is not enabled or documented as a production release endpoint.

Cloudflare Direct is independent from npm, RubyGems, PyPI, and NuGet. Their
packages never retrieve a binary from the download domain.

The detached-signature trust root and expected installer-verification procedure
are published through a channel independent from the R2 release credential.
Versioned installers are themselves authenticated before execution, pin the
approved verification identity, and authenticate the exact manifest bytes
before parsing any release metadata. Fetching an installer and its trust root
solely from the same mutable origin is not an accepted bootstrap.

### Native Platform Signing

Every Windows binary entering a public binary channel is Authenticode signed.
Every macOS binary is Developer ID signed and passes the notarization procedure
approved for its submission and distribution envelopes. Signing and
notarization occur after compilation and other byte transformations but before
final hashes and package assembly. The signed native bytes become the sole
artifact reused by Cloudflare Direct, npm, RubyGems, PyPI, and NuGet.

Signing identities are isolated from build and registry credentials. If the
required identity or target-native verification cannot be established, that
platform does not enter the initial binary matrix; it is not published unsigned
under the same support claim.

### Publication Identity And Bootstrap

Registry publication uses GitHub Actions OIDC or the registry's trusted
publishing mechanism wherever supported. Phase B01.0 records the current
first-publication procedure for each registry, including whether a pending
publisher can create the project or a human bootstrap publication is required.

Any unavoidable bootstrap credential is destination-scoped, exposed only to a
protected release environment, used once, audited, and revoked immediately
after trusted publishing is established. Normal releases do not use long-lived
registry tokens. The protected release environment requires explicit human
approval before any publishing identity becomes available.

## Decisions To Record

Record the following in implementation or canonical release documentation
rather than leaving them implicit in workflow code:

- crate name and publishing owner;
- npm root name, platform scope, and publishing owner;
- RubyGem name and publishing owner;
- PyPI project name and publishing owner;
- NuGet top-level and generated RID package IDs and publishing owner;
- R2 bucket, custom domain, account owner, and budget owner;
- minimum supported Node, npm, and pnpm versions;
- minimum supported Ruby, RubyGems, and Bundler versions;
- minimum supported Python, pip, pipx, and uv versions;
- .NET SDK 10.0 as the minimum NuGet frontend and `net10.0` as the managed
  launcher target when a launcher is required;
- supported Rust baseline inherited from `Cargo.toml`;
- canonical `MAJOR.MINOR.PATCH` release grammar and exact-text checks for every
  registry;
- confirmed target matrix and evidence available for every target;
- macOS deployment baseline;
- exact wheel compatibility tags and their supporting evidence;
- exact NuGet RID package IDs and their supporting compatibility evidence;
- GitHub tag-signing identity and immutable-release repository setting;
- Windows Authenticode identity and verification procedure;
- macOS Developer ID identity, notarization procedure, and verification
  evidence;
- manifest- and installer-signing identity, algorithm, independently delivered
  trust root, and verification method;
- trusted-publishing mechanism, protected environment, first-publication
  bootstrap, and accountable owner for each destination.

No registry or Cloudflare secret is committed. This phase records ownership and
required secret boundaries, not secret values.

## Implementation Tasks

1. Reserve or verify the crate, npm root/scope, RubyGem, PyPI project, and NuGet
   package names.
2. Verify that canonical `MAJOR.MINOR.PATCH` text is accepted without
   normalization by every destination and encode that grammar as a release
   precondition.
3. Enable GitHub immutable releases and establish signed annotated release tags
   plus metadata-only GitHub Releases.
4. Reserve the download domain, create the R2 bucket, and confirm custom-domain
   connectivity without making `r2.dev` a release path.
5. Select and document minimum npm, pnpm, Ruby, RubyGems, Bundler, Python, pip,
   pipx, and uv versions; record the fixed .NET SDK 10.0 minimum.
6. Establish Windows and macOS signing identities and prove target-native
   signature and notarization verification before final hashing.
7. Select detached signatures for the manifest and installers, define the
   independently delivered trust root, and document the bootstrap procedure.
8. Confirm every row in the [initial platform
   contract](README.md#initial-platform-contract) on native or trustworthy
   emulated runners.
9. Configure a protected release environment and record destination ownership,
   OIDC or trusted-publishing setup, and first-publication prerequisites.
10. Mark unavailable targets as unsupported rather than weakening or
   mislabeling the matrix.

## Verification

- Registry queries establish whether each desired name can be owned and
  published as planned.
- Every accepted version round-trips through registry validation without text
  normalization; prerelease, build-metadata, and noncanonical numeric forms are
  rejected before assembly.
- A signed tag is locked by a metadata-only immutable GitHub Release, and the
  repository rejects tag movement after publication.
- DNS and R2 tests establish that the selected custom domain can serve an
  object over TLS without exposing the production bucket through `r2.dev`.
- Runtime baselines are supported by their package metadata formats and CI
  runners.
- Every proposed wheel tag is accepted by PyPI and accurately describes the
  native executable it contains.
- A local .NET SDK 10 prototype creates a pointer package and one package per
  approved RID, installs globally on every supported platform, downloads only
  the matching RID package, invokes the exact native binary, and proves the
  selected Unix executable-permission strategy.
- The prototype records whether a prebuilt Rust executable can be the direct
  RID tool entry point. If not, it proves the minimal `net10.0` launcher and its
  transparent process behavior.
- The NuGet prototype contains no `build/*.props`, `build/*.targets`, project
  dependency integration, Rust compilation, or secondary download.
- Every RID package and the pointer package remain within registry size limits.
- Isolated global-installation tests cover npm, pnpm, gem, the pip user scheme,
  pipx, `uv tool`, and .NET Tool; Bundler independently covers gem platform
  resolution and execution.
- The signing method can sign and verify a fixed test payload using trust
  material stored outside R2.
- A direct installer obtained through the documented bootstrap rejects a forged
  installer signature and a forged manifest before parsing metadata.
- Windows and macOS verification tools accept the final signed test binaries;
  macOS also proves the selected notarization path.
- Each platform has a credible native execution environment for later smoke
  tests.
- Every external account and credential has a named human owner.
- Every registry has a documented trusted-publishing path and an explicit,
  revocable procedure for any unavoidable first-publication bootstrap.

## Exit Criterion

Every advertised channel, package, runtime, and platform has a tested identity,
an accountable owner, and a recorded compatibility decision. Version grammar,
tag immutability, native signing, installer trust bootstrap, and publication
identity are proven. Later phases can consume these decisions without inventing
names, versions, trust roots, wheel tags, or platform promises.
