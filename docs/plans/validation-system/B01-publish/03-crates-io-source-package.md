# Phase B01.3: Assemble The crates.io Source Package

[Back to the distribution plan](README.md)

## Purpose

Produce the exact Cargo source-package candidate for crates.io and prove that it
contains everything required to compile, install, and run Liknon independently
from the repository checkout.

This is a source-distribution phase. It does not consume or embed the native
artifacts from Phase B01.2, and it does not publish credentials or upload a
package.

## Dependencies

- [Phase B01.1](01-version-and-provenance.md) is complete.
- The crate name, ownership, minimum supported Rust version, license, repository,
  documentation, and release identity from Phase B01.0 are confirmed.
- The exact signed source commit and its lockfile pass the normal project gates.

## Cargo Package Contract

Cargo is the only producer of the `.crate` archive. Release tooling must not
handcraft, modify, or re-compress the archive after `cargo package` creates it.
The candidate is produced from the exact clean release commit without
`--allow-dirty` and uses the version read from Cargo metadata.

The package must contain every file needed by compilation and runtime behavior,
including:

- Rust source and `build.rs`;
- the lockfile behavior required by locked installation of this binary crate;
- schemas and embedded validation knowledge;
- the agent skill distributed with Liknon;
- README, license, security, configuration-design, and changelog material
  referenced by package metadata or required by the distributed CLI;
- branding assets embedded by the help system;
- examples and package-time verification assets intentionally declared in the
  Cargo include contract.

The package must not contain repository metadata, credentials, local
configuration, generated native binaries, `target/` contents, release staging,
publication evidence, or unrelated development files. It contains no prebuilt
Liknon executable: crates.io users compile the source locally.

Every dependency needed by a published build must be resolvable under the
crates.io publication contract. Unpublished path-only dependencies, hidden
workspace state, user-specific Cargo configuration, and undeclared repository
files are prohibited.

## Assembly And Inspection

Run the package flow with the release Cargo toolchain and a clean source tree:

```sh
cargo package --list --locked
cargo package --locked
cargo publish --dry-run --locked
```

Retain the generated `target/package/liknon-<VERSION>.crate` as the inspected
candidate. Record its package identity, version, source commit, byte size,
SHA-256, Cargo and Rust versions, and sorted logical file inventory in release
evidence.

Extract the archive into a fresh temporary directory and inspect the extracted
package rather than trusting the repository tree. Validate normalized Cargo
metadata, the lockfile, file types, paths, permissions, required assets, and the
absence of prohibited content. Archive entries must not escape the package root
or rely on unsafe links.

Because `cargo publish` owns final packaging, publication later runs from the
same clean signed commit with the same locked Cargo toolchain. Repeated local
packaging and the dry run must produce the same logical package inventory. Raw
archive equality is required when the Cargo version used by the release
guarantees stable bytes; otherwise the retained archive hash and canonical
logical inventory remain distinct evidence and crates.io readback is judged by
the destination-equivalence contract.

## Package-Only Build And Installation

Using the extracted package as the only project source, and isolated
`CARGO_HOME`, target, and installation directories:

1. build and test with the release Rust toolchain;
2. build and install with the declared minimum supported Rust version;
3. generate crate documentation without repository-only inputs;
4. run `liknon --version`, `liknon --help`, `liknon knowledge catalog`, and a
   representative `liknon validate` fixture through the installed executable;
5. uninstall Liknon from the isolated Cargo root and prove that its executable
   no longer resolves.

The package-only build must not read the original checkout, inherit a project
`.cargo` directory, invoke a network host other than the configured Cargo
registry for dependencies, or discover assets through paths outside the
extracted package.

## Implementation Tasks

1. Add a credential-free Cargo source-package job pinned to the release
   toolchain.
2. Generate and retain the `.crate` candidate with Cargo from a clean release
   checkout.
3. Implement required-file and prohibited-file inspection against the extracted
   archive.
4. Record package identity, source identity, toolchain, hash, size, and logical
   inventory as release evidence.
5. Build, document, install, exercise, and uninstall from the extracted package
   under isolated homes and target directories.
6. Repeat the package-only build on the minimum supported Rust version and on
   every operating system advertised for the crates.io source channel.
7. Prove that package assembly and verification leave tracked files unchanged.

## Verification

- `cargo package --list --locked`, `cargo package --locked`, and
  `cargo publish --dry-run --locked` succeed without credentials or a dirty-tree
  override.
- Package name and version exactly match Cargo metadata and the signed release
  tag.
- The extracted package contains every compile-time and runtime asset and no
  prohibited release or repository content.
- A clean package-only build succeeds without access to omitted workspace files
  or user-specific Cargo configuration.
- The minimum supported Rust version accepts the package and produces a working
  CLI.
- Installed CLI behavior and exit codes match the checked-out source build for
  the representative fixture.
- Cargo uninstall removes the isolated command completely.
- Repeated packaging preserves the canonical logical inventory, and any claimed
  raw-byte reproducibility is demonstrated rather than assumed.
- Assembly, inspection, installation, and uninstallation leave the repository
  clean.

## Exit Criterion

One inspected, retained Cargo source-package candidate is attributable to the
signed release source, passes the crates.io dry run, builds and installs from
its extracted contents on every advertised source-build platform and supported
Rust baseline, contains all required assets and no prohibited material, and is
ready for the joined pre-publication verification without registry credentials.
