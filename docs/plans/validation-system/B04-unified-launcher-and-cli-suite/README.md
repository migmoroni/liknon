# B04 - Unified Launcher And CLI Suite

## Status

`Approved target`. B04 is the normative owner of the public CLI suite,
including source packaging, native archives, installation, launcher routing,
and public release identity.

B01, B02, and B03 remain the owners of Validator, Manager, and Tool behavior.
Their earlier standalone distribution work is retained as implementation
evidence and reusable infrastructure, but B04 supersedes standalone public CLI
packages and standalone native downloads.

## Objective

Ship one compact CLI suite that preserves three independent capability
processes while giving users one installation and one optional launcher:

| Physical executable | Responsibility |
| --- | --- |
| `liknon` | Capability-free launcher and suite entry point |
| `liknon-validator` | Declarative workspace validation |
| `liknon-manager` | Resource acquisition and workspace organization |
| `liknon-tool` | Safe local resource inspection and retrieval |

The capability executables remain directly invocable. The launcher never
absorbs their domain behavior.

## Public Package Contract

The crates.io package is `liknon-cli`. It owns exactly four Cargo binary
targets:

```text
liknon
liknon-validator
liknon-manager
liknon-tool
```

`cargo install liknon-cli --locked` installs all four executables through
Cargo's normal multi-binary package behavior. No post-install hook, secondary
installer, dependency-side binary, or package mutation is required.

`liknon-cli` is a package name, not an additional executable name. The compact
executables `likx`, `likv`, `likm`, and `likt` do not exist in the approved
target. There is no compatibility layer because the product has not been
publicly released.

Supporting implementation crates may remain separate inside the repository,
but after the B04 cutover they own no public executable targets. They must be
publishable dependencies of `liknon-cli`, or their implementation must be
folded into its publishable source closure.

The package name `liknon` remains reserved for the programmatic and
workspace-integration product defined by B05. It does not install this CLI
suite.

## Launcher Grammar

The launcher accepts only an explicit capability namespace:

```text
liknon validator <args...>  -> liknon-validator <args...>
liknon v <args...>          -> liknon-validator <args...>

liknon manager <args...>    -> liknon-manager <args...>
liknon m <args...>          -> liknon-manager <args...>

liknon tool <args...>       -> liknon-tool <args...>
liknon t <args...>          -> liknon-tool <args...>
```

`v`, `m`, and `t` are parser aliases inside `liknon`; they are not files,
symlinks, copied executables, Cargo binary targets, or download artifacts.

The launcher strips only the selected namespace and forwards all remaining
arguments in order. It resolves the sibling executable from the installed
suite, preserves standard streams and signals, and returns the child exit
status. It rejects missing or unknown namespaces and never chooses a default
capability.

## Help And Version

`liknon --help` explains the three namespaces and points users to the focused
help of each capability executable. It does not duplicate their complete
command references.

`liknon --version` reports the suite version and the version of each bundled
capability executable. A mismatch is an integrity failure, not an alternate
supported state.

Each direct executable retains its own focused `--help` and `--version`.

## Source Installation

The source package installs the complete suite:

```sh
cargo install liknon-cli --locked
```

A source checkout uses the same package and committed lockfile. Packaging tests
must prove that the crates.io source closure builds all four targets outside the
repository without unpublished path dependencies.

The package does not install AI resources, Hub content, or a knowledge corpus.
Those remain independently managed data.

## Native Distribution

The site and Cloudflare R2 publish exactly one native suite archive for each
supported target. Every archive contains the same four executable roles:

```text
liknon
liknon-validator
liknon-manager
liknon-tool
```

Windows archives use the corresponding `.exe` filenames. Archive format,
platform authenticity, and runtime baseline remain target-specific.

Examples of distinct target artifacts include:

```text
x86_64-unknown-linux-gnu
x86_64-unknown-linux-musl
x86_64-pc-windows-msvc
aarch64-apple-darwin
```

GNU and MUSL are separate target archives. Platform detection selects exactly
one compatible archive; when both environments appear viable, GNU is the
preferred Linux target. No archive contains binaries for another target.

The website offers one suite download for the selected target. R2 does not
publish separate Validator, Manager, or Tool downloads in the approved target.

Each archive has one authenticated manifest that records:

- suite package, version, tag, and source commit;
- Rust target and runtime baseline;
- all four executable filenames, sizes, and SHA-256 hashes;
- archive hash and size;
- provenance and platform-authenticity references;
- installer compatibility and minimum supported installer version.

Installers authenticate metadata before payload use and replace the four files
transactionally. Interrupted or partial installation must leave either the
previous complete suite or the new complete suite, never a mixed version.

## Release Identity

`apps/liknon-cli/Cargo.toml` is the sole authored version authority for the
public CLI suite. The package, all four executables, source candidate, native
archives, manifests, installers, documentation, and public stable pointer agree
on one version, tag, commit, and source closure.

B01, B02, and B03 do not retain independent public CLI release trains after the
B04 cutover. Their behavior can evolve in separate modules and tests, but the
public executable suite is promoted atomically.

The resource Hub and resource versions remain independent because they are
data products rather than bundled CLI executables.

External authority and least-privilege setup are defined in
[Accounts And Permissions](accounts-and-permissions.md).

## Internal Boundaries

- Validator behavior remains owned by B01 and its implementation modules.
- Manager and Hub behavior remain owned by B02.
- Tool behavior remains owned by B03.
- `liknon-cli-ui` remains the shared terminal-experience crate.
- `liknon-resource-contracts` remains the shared resource contract where
  Manager and Tool need identical semantics.
- `liknon` performs routing only.
- Direct and routed execution must invoke the same installed capability
  executable and produce equivalent behavior.

## Implementation Units

| Unit | Purpose | Exit evidence |
| --- | --- | --- |
| B04.0 | Freeze launcher, package, executable, and suite contracts | [Decision gate](00-launcher-and-suite-contract.md) approved with executable ownership and archive topology fixed |
| B04.1 | Move all public binary targets under `liknon-cli` and implement routing | Exactly four package-owned executables; no compact executable aliases |
| B04.2 | Close and verify the source package | `cargo package` and isolated `cargo install` produce all four commands |
| B04.3 | Build and install one native suite archive per target | Authenticated archive, direct/routed smoke tests, transactional install, and rollback evidence |
| B04.4 | Prepare coordinated publication | One suite candidate, stable pointer, site contract, documentation, and public readback evidence |

## Verification

The B04 verification matrix must prove:

- the package owns exactly four binary targets;
- no compact executable alias is installed or archived;
- direct invocations of all three capability executables work;
- long and short launcher namespaces route to the same executable;
- argument bytes, environment, working directory, standard streams, signals,
  and exit codes are preserved;
- launcher help and version behavior remain capability-free;
- the source package builds outside the repository;
- each native archive contains exactly the four target-compatible executables
  plus declared metadata;
- all four executable hashes match the authenticated manifest;
- installation and update are atomic across the suite;
- archive extraction, replacement, interruption, and rollback reject unsafe or
  incomplete states;
- the site selects one archive for one target and never downloads unrelated
  platform payloads;
- direct and routed execution remain semantically equivalent on every supported
  target.

## Acceptance Criteria

- [ ] `liknon-cli` is the only public source package for the CLI suite.
- [ ] The package installs `liknon`, `liknon-validator`, `liknon-manager`, and
  `liknon-tool`.
- [ ] `likx`, `likv`, `likm`, and `likt` are absent from Cargo metadata,
  archives, installers, and supported-command documentation; tests assert their
  rejection.
- [ ] `validator|v`, `manager|m`, and `tool|t` are launcher-only namespaces.
- [ ] Each capability executable remains independently invocable.
- [ ] The launcher owns no capability behavior.
- [ ] Source packaging has a complete publishable dependency closure.
- [ ] Every supported target has exactly one authenticated native suite archive.
- [ ] Every archive contains exactly the four executable roles for that target.
- [ ] Installation, update, rollback, and uninstall treat the suite as one
  coherent version.
- [ ] The website and R2 expose one suite download per target, not standalone
  capability downloads.
- [ ] B01, B02, and B03 behavior gates remain green after consolidation.
- [ ] B06 can promote one reviewed CLI suite candidate without reconstructing
  or replacing any binary.

## Exit

B04 exits when users can obtain one source package or one target-native archive,
receive all four executable roles, invoke each capability directly or through
`liknon`, and verify that every route belongs to one authenticated suite
release.
