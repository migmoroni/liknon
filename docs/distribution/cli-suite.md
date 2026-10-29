# CLI Suite Distribution

Liknon distributes its command-line product as one coherent suite. Every
supported installation contains exactly these four physical executables at one
version:

```text
liknon
liknon-validator
liknon-manager
liknon-tool
```

`liknon` is a capability-free launcher. It routes `validator|v`, `manager|m`,
or `tool|t` to the corresponding sibling process. These short namespace forms
are not executable files. The three capability executables remain directly
invocable.

## Current Source Installation

Until the first protected public release is activated, install from a trusted
checkout:

```sh
git clone https://github.com/migmoroni/liknon.git
cd liknon
cargo install --path apps/liknon-cli --locked --force
```

Rust 1.87 or newer is required. Reinstall after changing or updating the
checkout because Cargo installs compiled files rather than links.

Verify the complete installation:

```sh
liknon --version
liknon-validator --version
liknon-manager --version
liknon-tool --version
liknon validator --version
liknon manager --version
liknon tool --version
```

## Planned Public Channels

After B06 publication and public readback, users may obtain the same suite from
either source or a target-native archive:

- the crates.io package `liknon-cli` installs all four Cargo binary targets;
- one native archive per supported target contains all four executables;
- the project-controlled download origin serves authenticated installers and
  immutable target archives;
- GitHub Releases contain metadata only, not binary assets.

No capability has an independent public package, archive, installer, stable
pointer, or version train.

## Supported Candidate Targets

The reviewed target matrix currently contains:

| Target | Archive | Platform authenticity |
| --- | --- | --- |
| `x86_64-unknown-linux-gnu` | `.tar.gz` | Signed metadata and exact hashes |
| `x86_64-unknown-linux-musl` | `.tar.gz` | Signed metadata and exact hashes |
| `aarch64-unknown-linux-gnu` | `.tar.gz` | Signed metadata and exact hashes |
| `aarch64-unknown-linux-musl` | `.tar.gz` | Signed metadata and exact hashes |
| `x86_64-apple-darwin` | `.tar.gz` | Ad hoc code signatures, not notarized |
| `aarch64-apple-darwin` | `.tar.gz` | Ad hoc code signatures, not notarized |
| `x86_64-pc-windows-msvc` | `.zip` | Project-controlled Authenticode identity with RFC 3161 timestamp |

GNU and MUSL are separate Linux artifacts. Automatic selection prefers GNU
when both environments appear viable; callers may select MUSL explicitly.
An archive never contains payloads for another target.

## Integrity And Installation

The authenticated manifest records the suite identity, source commit, target,
runtime baseline, archive hash, and the name, size, and SHA-256 digest of every
executable. Installers authenticate metadata and payloads before replacement.

Install, update, rollback, and uninstall treat all four files as one
transaction. A failed or interrupted update preserves the previous complete
suite and cannot leave mixed component versions. The launcher also rejects a
missing or version-mismatched sibling and never falls back to another `PATH`
installation.

macOS candidates are ad hoc signed and not notarized. If Gatekeeper quarantines
an explicitly downloaded archive, inspect its origin and signatures first;
only then remove quarantine from the exact installed Liknon files. Documentation
must not recommend disabling Gatekeeper globally.

The canonical machine contract is
[`../../distribution/cli/suite/product.json`](../../distribution/cli/suite/product.json).
Public channel commands and URLs are documented only after their first verified
publication.
