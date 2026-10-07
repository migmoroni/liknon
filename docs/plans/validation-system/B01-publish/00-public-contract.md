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
license, and branding asset required by the crate. Its assembly and package-only
verification are defined by [Phase B01.3](03-crates-io-source-package.md).

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
user-owned prefixes. Linux uses distinct `linux-x64-gnu`, `linux-x64-musl`,
`linux-arm64-gnu`, and `linux-arm64-musl` packages, each containing only its
matching Rust target. Package metadata and the launcher must cause installation
to retrieve only the host-compatible OS, architecture, and libc payload.

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
metadata is treated as a public contract. Linux manylinux wheels contain only
the matching GNU build and musllinux wheels contain only the matching MUSL
build; one native artifact cannot cross that libc boundary. The pip user scheme,
pipx, and `uv tool` are the supported globally-invocable installation frontends.

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

The public trust contract assigns stable identifiers to signing keys and defines
how keys become active, rotate, retire, and are revoked after compromise. The
selected mechanism may use a long-lived root with release-signing keys or an
independently distributed trusted-key set, but it must preserve verification of
releases signed by retired, uncompromised keys and let current trust material
reject revoked keys. The exact mechanism is decided and tested in this phase
rather than improvised by an installer.

### Linux Libc Selection

`gnu` is the public suffix for the glibc-linked Linux target; `musl` identifies
the distinct MUSL target. Both implementations may legitimately be available on
one system. Their coexistence is not an installation error.

Whenever Liknon-owned installation code must choose between those artifacts, it
applies this precedence:

1. An explicit supported `gnu` or `musl` selection wins.
2. If automatic detection finds both GNU/glibc and MUSL, select GNU.
3. If automatic detection finds exactly one, select that implementation.
4. If neither can be identified, fail before retrieving a native archive and
   require an explicit selection.

Selection uses reviewed runtime-capability probes. It does not infer libc from
the Linux distribution name, the presence of a container runtime, or whether the
process happens to run inside a container. The chosen artifact must still meet
its declared compatibility baseline, and a failed compatibility check never
causes silent fallback to the other libc.

Cloudflare Direct exposes this choice through the POSIX installer's
`--libc=gnu|musl` option and otherwise uses the automatic policy. Package-manager
channels instead declare exact platform metadata, compatibility tags, or RIDs.
Their native resolver is authoritative once it selects a package; Ruby, Python,
or .NET launchers do not repeat libc detection or replace that package. npm and
pnpm must use the declared `libc` metadata, while the Node launcher only resolves
the package made available by that contract and performs no secondary download.

### Native Platform Signing

Release authenticity and native platform trust are separate contracts. The
signed public manifest, its independently delivered trust root, and the hashes
it authenticates establish the cross-platform release identity. Native signing
adds platform-local integrity evidence without replacing that release trust
chain.

Every initial Windows binary entering a public binary channel is Authenticode
signed with SHA-256 by one persistent, project-controlled self-signed
code-signing certificate. It also receives an RFC 3161 timestamp using SHA-256 from
the approved timestamp authority. The private key is created once, stored only
in a protected signing environment, and never generated afresh for an
individual release. The public certificate and its SHA-256 fingerprint are
published independently from the binary host. Verification covers the
cryptographic signature, expected fingerprint, timestamp, digest algorithms,
and mutation detection. It does not claim a publicly trusted certificate chain,
a verified publisher recognized by Windows, or SmartScreen reputation.

Every initial macOS binary receives an ad hoc code signature with the `-`
pseudo-identity after all other byte transformations. Verification proves that
`codesign` accepts the signature, records the resulting code-directory identity,
and detects mutation. Ad hoc signing uses no publisher certificate, provides no
publisher identity, and does not satisfy Developer ID or notarization. CI must
therefore prove actual command execution on clean macOS runners in addition to
signature integrity.

Linux has no platform-native signing requirement in the initial matrix. Its
artifacts remain covered by the same signed release metadata, hashes, package
registry evidence, and target-native execution tests as every other platform.
GNU and MUSL are separate native targets for both supported architectures and
must retain separate compatibility evidence through every distribution channel.

Platform sealing occurs after compilation and every other byte transformation
but before final hashes and package assembly. The final native bytes become the
sole artifacts reused by Cloudflare Direct, npm, RubyGems, PyPI, and NuGet. The
Windows private key is isolated from build, detached-release-signing, and
registry credentials; macOS ad hoc signing requires no secret. No installer,
launcher, package, or automated instruction may import the self-signed
certificate into a user trust store, disable or bypass SmartScreen or
Gatekeeper, remove quarantine metadata, or otherwise weaken operating-system
security policy.

Canonical macOS documentation may describe a manual, user-initiated exception
only after the user has verified the signed release manifest and the exact
native hash. It prefers Apple's `Open Anyway` flow when that flow applies. If a
tested standalone-CLI path still requires direct quarantine removal, the only
documented command deletes `com.apple.quarantine` from the exact Liknon binary
path. Documentation explains the lost Gatekeeper signal and never recommends
clearing all extended attributes, recursive removal, `xattr -c`, or
`xattr -cr`.

A platform enters the initial binary matrix only when its declared authenticity
mode, target-native verification, clean installation, and execution tests pass.
The absence of public native-signer trust alone does not exclude Windows or
macOS, but it must remain explicit in manifests and user documentation. A later
move to CA-trusted Authenticode or Developer ID plus notarization is a new
reviewed trust mode, not a silent reinterpretation of the initial evidence.

### Publication Identity And Bootstrap

Registry publication uses GitHub Actions OIDC or the registry's trusted
publishing mechanism wherever supported. Phase B01.0 records the current
first-publication procedure for each registry, including whether a pending
publisher can create the project or a human bootstrap publication is required.

Human approval occurs in a credential-free protected gate. Detached release
signing, Windows native signing, and each publication destination use separate
protected environments or equivalently isolated identity boundaries. macOS ad
hoc signing uses no identity. Each destination's OIDC policy is bound to the
exact repository, workflow, environment, and package scope supported by that
provider. A job cannot obtain another boundary's identity merely because the
release was approved.

Any unavoidable bootstrap credential is destination-scoped, exposed only to its
destination environment after approval, used once, audited, and revoked
immediately after trusted publishing is established. Normal registry releases
do not use long-lived publication tokens. Persistent R2,
detached-release-signing, or Windows native-signing credentials remain
separately scoped, stored, and rotated.

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
- minimum supported glibc baseline and Alpine/MUSL baseline for each Linux
  architecture;
- GNU/MUSL detection probes, the `--libc=gnu|musl` direct-installer override,
  the GNU preference when both implementations are detected, and the diagnostic
  evidence emitted for each selection;
- canonical `MAJOR.MINOR.PATCH` release grammar and exact-text checks for every
  registry;
- confirmed target matrix and evidence available for every target;
- macOS deployment baseline;
- exact wheel compatibility tags and their supporting evidence;
- exact NuGet RID package IDs and their supporting compatibility evidence;
- GitHub tag-signing identity and immutable-release repository setting;
- Windows self-signed Authenticode certificate subject, SHA-256 fingerprint,
  validity and rotation policy, SHA-256 digest policy, RFC 3161 timestamp
  authority, protected-key owner, and signature and timestamp verification
  procedure;
- macOS ad hoc signing command, code-directory evidence, explicit non-notarized
  status, clean-runner execution procedure, verification evidence, and the
  reviewed user-mediated exception procedure when one is actually required;
- manifest- and installer-signing identity, algorithm, independently delivered
  trust root, key identifiers, activation, rotation, retirement, emergency
  revocation, and verification method;
- credential-free human approval gate plus the trusted-publishing mechanism,
  isolated protected environment or equivalent identity boundary,
  first-publication bootstrap, and accountable owner for each destination.

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
   pipx, and uv versions; record the fixed .NET SDK 10.0 minimum and the Linux
   glibc and Alpine/MUSL compatibility baselines. Approve the Linux libc probes,
   explicit override, automatic precedence, and failure behavior without using
   distribution or container names as detection inputs.
6. Establish the persistent Windows self-signed signing identity and protected
   key boundary; prove Authenticode and RFC 3161 SHA-256 timestamp verification.
   Prove macOS ad hoc signing, mutation detection, and clean-runner execution.
   Record both platforms' lack of public native-signer trust before final
   hashing.
7. Select detached signatures for the manifest and installers, define the
   independently delivered trust root, define and test key activation, rotation,
   retirement, and emergency revocation, and document the bootstrap procedure.
8. Confirm every row in the [initial platform
   contract](README.md#initial-platform-contract) on native or trustworthy
   emulated runners.
9. Configure a credential-free protected approval gate plus isolated signing and
   destination environments or equivalent identity boundaries. Record each
   destination's ownership, OIDC or trusted-publishing policy, and
   first-publication prerequisites.
10. Mark unavailable targets as unsupported rather than weakening or
    mislabeling the matrix; do not confuse a tested self-signed or ad hoc mode
    with an absent signature or with public operating-system trust.

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
- Direct-installer fixtures prove explicit GNU and MUSL selection, automatic
  GNU-only and MUSL-only selection, GNU selection when both are detected, and a
  pre-download error when neither is identifiable.
- Linux selection evidence records operating system, architecture, detected
  libc implementations, decision source, selected target, and downloaded
  archive; every successful case retrieves exactly one archive.
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
- The detached release-signing method can sign and verify a fixed test payload
  using trust material stored outside R2.
- A key-lifecycle fixture proves that a newly active key signs a new release,
  an uncompromised retired key still verifies its historical release, and a key
  marked revoked is rejected by current trust material.
- A direct installer obtained through the documented bootstrap rejects a forged
  installer signature and a forged manifest before parsing metadata.
- Windows verification proves the final Authenticode signature, expected
  self-signed certificate fingerprint, RFC 3161 SHA-256 timestamp, and expected lack
  of public chain trust. macOS verification proves the final ad hoc signature,
  mutation detection, explicit absence of notarization, and execution on clean
  target runners.
- Installation tests begin without pre-trusting the Windows certificate or
  weakening macOS Gatekeeper or quarantine policy. No shipped artifact changes
  those settings.
- A macOS documentation test proves that any required manual fallback is shown
  only after manifest-signature and native-hash verification, targets only
  `com.apple.quarantine` on the exact Liknon path, and never clears unrelated
  extended attributes or acts recursively.
- Each platform has a credible native execution environment for later smoke
  tests.
- Every external account and credential has a named human owner.
- Approval-gate jobs have no publication or signing credential, and every
  signing or destination job is unable to obtain another boundary's identity.
- Every registry has a documented trusted-publishing path and an explicit,
  revocable procedure for any unavoidable first-publication bootstrap.

## Exit Criterion

Every advertised channel, package, runtime, and platform has a tested identity,
an accountable owner, a recorded compatibility decision, and an explicit native
authenticity mode that does not overstate operating-system trust. Version
grammar, tag immutability, initial platform sealing, Windows timestamping,
installer trust bootstrap and key lifecycle, and isolated publication identity
are proven. Later phases can consume these decisions without inventing names,
versions, trust roots, wheel tags, or platform promises.
