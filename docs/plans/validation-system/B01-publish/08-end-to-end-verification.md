# Phase B01.8: Verify Distribution End To End

[Back to the distribution plan](README.md)

## Purpose

Prove, before publication, that every source package, native archive, npm
tarball, platform gem, platform wheel, and NuGet pointer/RID package carries
the intended version and native bytes and behaves like the same Liknon CLI.

## Dependencies

- [Cloudflare Direct assembly](03-cloudflare-direct.md) is complete.
- [npm package assembly](04-npm-packages.md) is complete.
- [RubyGems package assembly](05-rubygems-packages.md) is complete.
- [PyPI wheel assembly](06-pypi-wheels.md) is complete.
- [NuGet .NET Tool assembly](07-nuget-dotnet-tool.md) is complete.
- All final candidate artifacts and manifests are retained in private workflow
  storage.

## Verification Layers

### Project Quality Gates

- `cargo fmt --all -- --check`;
- `cargo clippy --all-targets --locked -- -D warnings`;
- `cargo test --all-targets --locked`;
- `cargo test --doc --locked`;
- the declared minimum Rust version where applicable;
- `cargo package --locked`;
- `cargo publish --dry-run --locked`;
- .NET Tool packaging tests under SDK 10, plus formatting, warnings-as-errors
  build, and tests for the managed launcher when one is present.

The Cargo package inspection confirms inclusion of every runtime source,
schema, knowledge document, skill, license, and branding asset required by the
crate.

### Identity And Byte Equality

- Cargo metadata, release tag, native CLI, public direct metadata, npm packages,
  gems, wheels, and every .NET Tool pointer/RID package report the same version.
- The shared version is canonical `MAJOR.MINOR.PATCH`; no destination-specific
  normalization or translated version is accepted.
- Every direct archive, npm tarball, gem, wheel, and pointer/RID `.nupkg` is
  extracted.
- Each extracted native binary SHA-256 equals the corresponding entry in
  `build-manifest.json`.
- Direct archive SHA-256 and size equal the public manifest.
- `manifest.json` and `SHA256SUMS` are deterministic projections of the same
  build evidence.
- `manifest.sig` verifies against the independent trust root and fails after
  mutation.
- Trust-lifecycle fixtures accept active and uncompromised retired keys in their
  defined scope and reject unknown or revoked signing keys.
- Windows binaries retain the approved self-signed Authenticode signature,
  expected certificate fingerprint, and verified RFC 3161 SHA-256 timestamp
  after every package layer is extracted.
- macOS binaries retain their ad hoc signatures and explicit non-notarized
  status after every package layer is extracted.
- The public manifest reports the same platform-authenticity mode as the build
  manifest and never promotes self-signed or ad hoc evidence to public trust in
  a native signer.

### Native And Direct Installation

- Run native `liknon --version`, `liknon --help`, and a representative
  `liknon validate` fixture on every target.
- Inspect the contents and permissions of every direct archive.
- Test manual installation using the documented verification procedure.
- Test POSIX and PowerShell installation from a local HTTP origin without
  Cargo, Node, Ruby, Python, or .NET.
- Verify each installer signature through the independent bootstrap before
  execution, then prove the installer verifies `manifest.sig` before parsing
  manifest fields.
- Reject a modified archive, mismatched manifest, interrupted download,
  malformed channel pointer, and unsupported target.
- Confirm that a failed installation does not replace an existing binary.
- Confirm default installation requires no elevated privileges.
- Begin Windows tests without trusting the self-signed certificate and macOS
  tests without weakening Gatekeeper or quarantine policy. Confirm that no
  installer changes those settings.
- Verify signed release metadata and native hashes independently from the
  platform signature before evaluating any expected operating-system warning.

### Registry Binary Installation

- Run `npm pack` for the root and every platform package.
- Install the local npm package set globally through npm and pnpm in separate
  clean user-owned prefixes without Cargo.
- Run `gem build --strict` for every platform gem.
- Install local gems globally in a clean `GEM_HOME` and resolve them separately
  through a Bundler fixture without Cargo.
- Inspect every wheel and verify all `RECORD` entries.
- Install local wheels through the pip user scheme, pipx, and `uv tool` in
  separate clean user homes without Cargo.
- Inspect the final NuGet pointer and every RID package, including their
  `DotnetTool` identities, exact relationships, selected entry point, native
  payload, and absence of MSBuild project assets.
- Install the local pointer package globally with .NET SDK 10 on each supported
  host and prove that only its matching RID package is retrieved.
- Confirm no Python sdist is produced or accepted as a fallback.
- Confirm npm, gem, wheel, and .NET Tool installation performs no secondary
  download, lifecycle build, Rust source compilation, or Cloudflare request.
- Test an omitted npm optional platform dependency and unsupported package
  platforms, plus an unsupported .NET Tool host.

### Package-Manager Lifecycle

For npm, pnpm, RubyGems, the pip user scheme, pipx, `uv tool`, and .NET Tool,
exercise two non-publishable fixture versions assembled through the same
package, launcher, platform-selection, and command-shim paths as release
artifacts:

- install the first fixture version globally into a clean isolated home;
- update or upgrade to the second version using the frontend's supported global
  command;
- prove that the invoked command resolves only the second version and payload;
- uninstall through the same frontend;
- prove that no active command shim, script, or conflicting platform payload
  remains.

Lifecycle fixtures validate package-manager replacement behavior only. They do
not satisfy release identity or native-byte checks; the actual release candidate
is separately installed and compared with `build-manifest.json` in the same
matrix.

### Launcher Transparency

For native, Node, Ruby, wheel-installed, and direct or managed .NET Tool entry
points, exercise:

- arguments containing spaces and non-ASCII data where supported;
- installation and execution paths containing spaces;
- current-directory and environment propagation;
- inherited stdin, stdout, and stderr;
- successful completion;
- nonzero exit status;
- signal propagation supported by the operating system.

### Platform Coverage

- Linux musl binaries run on both glibc-based Linux and Alpine.
- Linux arm64 runs on native arm64 or the approved equivalent.
- macOS binaries run against the declared deployment baseline.
- Windows executes without an undeclared runtime dependency.
- Windows native verification proves the expected self-signed fingerprint,
  timestamp, and mutation detection while recording the absence of public chain
  trust as expected state.
- macOS native verification proves strict ad hoc signature validation, mutation
  detection, lack of notarization, and actual execution on clean runners.
- Unix executable permissions survive every archive and package layer.

### Repository Hygiene

- Assembly modifies no tracked template or source file.
- Generated binaries, metadata, installers, package trees, and archives remain
  ignored.
- No assembly or test job rebuilds Liknon after the target build job.

## Global Installation Forms Under Test

Use real, version-pinned installation forms in isolated user-owned homes or
prefixes:

```sh
npm install --global liknon@<VERSION>
pnpm add --global liknon@<VERSION>
gem install liknon --version <VERSION>
python -m pip install --user "liknon==<VERSION>"
pipx install "liknon==<VERSION>"
uv tool install "liknon==<VERSION>"
dotnet tool install --global liknon --version <VERSION>
```

Each global installation runs `liknon validate` directly. The Bundler fixture
also runs `bundle exec liknon validate` to prove platform resolution, but
Bundler is not presented as the global installation path.

Do not use `liknon .` as an installation probe because `.` is interpreted as a
validation target identifier.

## Implementation Tasks

1. Add a release-candidate verification workflow or reusable jobs with no
   publication credentials.
2. Download every private native and staged package artifact.
3. Verify the build and public manifests before invoking any launcher.
4. Execute the complete matrix above in clean target environments.
5. Execute two-version update and uninstall fixtures independently for every
   primary global package-manager frontend.
6. Emit concise machine-readable evidence identifying artifact, target, test,
   declared platform-authenticity mode, public-trust status, and result.
7. Fail the joined phase if any channel lacks evidence for any advertised
   platform.

## Exit Criterion

Every distribution artifact is independently installable, contains the exact
native binary intended for its platform, preserves the Liknon process contract,
and passes the complete pre-publication matrix without destination credentials.
Every primary package-manager frontend also proves update and complete removal
without stale command resolution.
