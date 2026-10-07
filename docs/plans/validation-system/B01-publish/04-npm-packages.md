# Phase B01.4: Assemble npm Packages

[Back to the distribution plan](README.md)

## Purpose

Package Liknon for npm as one root launcher and exact-version platform packages
that embed the already-built native binaries.

## Dependencies

- [Phase B01.2](02-native-builds.md) is complete.
- The npm name, scope, Node, npm, and pnpm baselines, and owners from Phase
  B01.0 are confirmed.
- Every native input matches `build-manifest.json`.

## Root Package

The root package is named `liknon` and exposes the `liknon` executable through
its `bin` entry. Its `optionalDependencies` list every supported platform
package at the exact release version, preventing a root release from selecting
native bytes from another version.

The root package has no install scripts. Installation obtains all required
content from npm itself and never contacts Cloudflare, GitHub, or another binary
host. Both npm and pnpm must install it globally into an isolated user-owned
prefix and select the same platform package.

## Platform Packages

Each desired `@liknon/<platform>` package contains only:

- complete package metadata;
- notices required by the license;
- one native binary under a stable internal path.

Its manifest declares the applicable `os`, `cpu`, and, where necessary, `libc`
restrictions from the [platform matrix](README.md#initial-platform-contract).

Committed manifests are safe templates with a development placeholder and
`"private": true`. Staging writes complete manifests under
`target/distribution/npm/`, inserts the Cargo-derived version, and removes
`private` without modifying the templates.

## Node Launcher Contract

The launcher must:

- map supported `process.platform` and `process.arch` pairs explicitly;
- distinguish Linux libc only where package selection requires it;
- resolve the exact platform package through Node's package resolver;
- execute its binary directly without a shell;
- preserve arguments, current directory, environment, standard streams, exit
  status, and signal behavior;
- work when installation paths contain spaces;
- distinguish unsupported platforms from omitted optional dependencies;
- never download, compile, or search for an unrelated system binary.

The supported mapping is a small explicit table covered by tests rather than an
unchecked interpolation of package identifiers.

The minimum supported Node version selected in Phase B01.0 is encoded in
`engines.node`. The approved npm and pnpm versions are exercised independently
in CI.

## Implementation Tasks

1. Add non-publishable root and platform templates.
2. Implement the explicit platform resolver and transparent launcher.
3. Stage package manifests with the Cargo version and exact optional
   dependencies.
4. Copy only build-manifest-verified native bytes into platform packages.
5. Create all tarballs with `npm pack`.
6. Inspect package names, versions, platform metadata, contents, permissions,
   scripts, and dependency versions.

## Verification

- Publish the staged tarballs to an isolated local registry and install the root
  package globally with npm in a clean user-owned prefix without Cargo.
- Repeat the global installation with pnpm in a separate clean prefix.
- Run `liknon --version`, `liknon --help`, and a representative
  `liknon validate` fixture after each installation.
- Confirm each frontend installs the root package and only the compatible
  platform payload, even when its lock or metadata records other optional
  variants.
- Compare the extracted native binary with `build-manifest.json`.
- Test arguments, current directory, environment, standard streams, success,
  nonzero status, and signals.
- Test paths containing spaces.
- Test unsupported platforms and an omitted optional platform dependency.
- Prove there are no lifecycle downloads, build scripts, or Cloudflare
  requests.
- Prove staging leaves tracked files unchanged.

## Exit Criterion

Local npm tarballs install globally through npm and pnpm and run with the exact
release binary without Rust, Cargo, lifecycle compilation, or an external
binary host.
