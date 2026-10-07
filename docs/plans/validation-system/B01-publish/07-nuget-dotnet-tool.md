# Phase B01.7: Assemble The NuGet .NET Tool

[Back to the distribution plan](README.md)

## Purpose

Package Liknon with the .NET SDK 10 RID-specific tool model: one top-level
pointer package and one exact-version package for each supported runtime
identifier. Global installation must retrieve only the package compatible with
the user's platform and execute the already-built Rust binary it contains.

This phase does not create a C# implementation of Liknon. A managed launcher is
added only if the Phase B01.0 prototype proves that the SDK cannot expose the
prebuilt Rust executable directly as the RID tool entry point.

## Dependencies

- [Phase B01.2](02-native-builds.md) is complete.
- The top-level package ID, generated RID package IDs, publishing owner, exact
  RID matrix, and Unix permission strategy from Phase B01.0 are confirmed.
- .NET SDK 10.0 is the minimum installation and packaging frontend.
- The Phase B01.0 prototype has decided between a direct native entry point and
  a minimal `net10.0` launcher and has proven the selected form globally on
  every supported platform.
- Every native input matches `build-manifest.json`.

## Distribution Model

NuGet exposes `liknon` as the package users install. It is a `DotnetTool`
pointer package that identifies exact-version RID packages such as:

```text
liknon
├── liknon.win-x64
├── liknon.linux-x64
├── liknon.linux-musl-x64
├── liknon.linux-arm64
├── liknon.linux-musl-arm64
├── liknon.osx-x64
└── liknon.osx-arm64
```

Only RIDs approved in Phase B01.0 are emitted. The illustrative Linux GNU and
musl variants above may contain the same static Rust binary only after each RID
claim has independent execution evidence. No RID-agnostic `any` package or
source-build fallback is published.

All RID packages use the exact pointer-package version. The .NET CLI infers the
host RID, resolves the matching package, and downloads only that platform
payload during installation. Platform selection never occurs on the first
Liknon execution and never requires a secondary host.

The public installation form is global:

```sh
dotnet tool install --global liknon --version <VERSION>
liknon --version
```

Global update and removal use the corresponding `dotnet tool update --global`
and `dotnet tool uninstall --global` commands. Local tool manifests are outside
the supported installation matrix for this track.

This channel does not introduce `dotnet add package`, `PackageReference`,
`build/*.props`, `build/*.targets`, compile assets, or another MSBuild
integration. A future project-build integration, if ever needed, is a separate
product and plan.

## Entrypoint Decision

Phase B01.0 must test both permitted transport shapes against the actual .NET
SDK 10 package contract:

1. **Direct native entry point:** the RID package exposes the exact Rust
   executable as the installed tool command, with no managed launcher.
2. **Managed transport entry point:** when direct exposure is unsupported, the
   RID package contains one minimal `net10.0` launcher beside one Rust binary.

The direct form is selected when it satisfies the official tool contract and
all process tests. Otherwise, the managed launcher must:

- resolve only the single native executable co-packaged for its RID;
- invoke it without a shell or `PATH` lookup;
- preserve arguments exactly, including spaces and non-ASCII data where the
  platform supports them;
- inherit the caller's current directory, environment, stdin, stdout, and
  stderr;
- return the native process exit code and preserve supported interruption and
  signal behavior;
- perform no validation, platform selection, configuration discovery, network
  access, update check, binary download, or Rust compilation.

On Unix, installation and execution must preserve or safely establish
executable permission for only the package-owned Rust binary. Permission repair
may change mode bits but never binary bytes. It must use the strategy proven in
Phase B01.0 and fail explicitly when the installed location cannot be handled
safely.

## Package Contract

- Use the .NET SDK 10 RID-specific tool packaging mechanism rather than custom
  runtime dependency selection.
- The top-level `liknon` package is a pointer and contains no native payload.
- Each `liknon.<rid>` package contains exactly one manifest-verified Rust
  binary, required notices, SDK tool metadata, and only the selected transport
  entry point.
- Every package receives the Cargo-derived version, repository and source
  commit, license, description, and package type through deterministic staging.
- RID package references in the pointer are exact-version relationships.
- The package set contains no `any` fallback, installation hook, secondary
  downloader, project build asset, or Cloudflare dependency.
- Generated package trees and `.nupkg` files remain under
  `target/distribution/nuget/` and are never committed.

The exact internal paths and SDK-generated members are accepted only after
inspection of the Phase B01.0 prototype. The plan does not reproduce a
handwritten approximation of the .NET Tool package layout.

## Assembly Contract

- Do not invoke Cargo or rebuild Liknon during NuGet assembly.
- Copy a native binary into a RID package only after its SHA-256 matches
  `build-manifest.json`.
- Generate the package version, RID list, package IDs, and pointer relationships
  from Cargo metadata and the approved platform matrix.
- Build a managed launcher only when the prototype requires it; launcher build
  output is transport code and remains independent from every Rust target
  build.
- Produce one deterministic candidate pointer package and one deterministic
  candidate package per supported RID.
- Verify every package remains within NuGet limits before release-candidate
  handoff.

## Implementation Tasks

1. Complete the direct-entry-point versus managed-launcher prototype with .NET
   SDK 10 and record the chosen transport shape.
2. Add version-neutral .NET Tool metadata under `distribution/nuget/`, using
   `ToolPackageRuntimeIdentifiers` or the corresponding proven SDK 10 contract.
3. If required, implement the shell-free `net10.0` transport launcher without
   platform-selection or validation logic.
4. Add deterministic staging that consumes only manifest-verified native
   artifacts and creates one tree per approved RID.
5. Pack every RID package and the pointer package without invoking Cargo.
6. Inspect package IDs, exact versions, RID relationships, metadata, contents,
   native hashes, permissions, and absence of project build assets.
7. Prove assembly leaves tracked files unchanged.

## Verification

- Unpack every `.nupkg`; confirm the pointer contains no native binary and each
  RID package contains exactly the intended platform binary.
- Compare every packaged native executable SHA-256 with
  `build-manifest.json`.
- Confirm package IDs, versions, `DotnetTool` identity, command name, RIDs,
  repository metadata, licenses, and pointer relationships.
- Publish the complete candidate set to an isolated local NuGet source, install
  `liknon` globally on each supported host, and prove that only the pointer and
  matching RID package are retrieved.
- Run `liknon --version`, `liknon --help`, and a representative
  `liknon validate` fixture from a clean global tool home.
- Exercise arguments, paths containing spaces, current directory, environment,
  standard streams, successful completion, nonzero status, and supported
  interruption behavior.
- Confirm Linux and macOS execution after real NuGet extraction, including the
  approved executable-permission behavior.
- Confirm an unsupported operating system, architecture, or libc has no
  compatible RID package and fails without selecting another binary or
  compiling source.
- Confirm installation and execution perform no secondary download, Cargo
  invocation, source compilation, Cloudflare request, or project modification.
- Confirm the package set contains no MSBuild props, targets, compile assets,
  library API, or validation logic in a conditional launcher.
- Update between two locally published fixture versions and uninstall through
  global .NET Tool commands without leaving a conflicting command shim.

## Publication Ordering Contract

All RID packages must be published and read back successfully before the
top-level pointer package is published. The pointer is the NuGet completion
marker for that version. A retry may skip a RID package only after confirming
that the registry already contains the exact staged identity; it never rebuilds
or replaces accepted package bytes.

## Exit Criterion

The local NuGet package set installs globally through .NET SDK 10 on every
advertised platform, retrieves only the compatible RID package, invokes the
exact manifest-verified Liknon binary with a transparent process contract,
requires no Rust toolchain or secondary download, and contributes no dependency
or build behavior to .NET projects.
