# Cargo

## Purpose And Scope

Cargo resolves Rust packages and coordinates compilation, tests, documentation,
and package assembly. This reference covers only behavior needed to select and
interpret validation evidence.

## Version Lines And Sources

Use the Cargo line shipped with each maintained Rust toolchain and test the
workspace's declared MSRV line separately when it is a promise. The Cargo Book
and command reference are living documentation. Confirm active versions with
`cargo --version` and `rustc --version`; exact reviewed snapshots are
provenance, not the reader scope.

## Command Model

Representative non-publishing forms are shown below. Adapt workspace, target,
feature, and offline selection to the declared claim.

```text
cargo check --workspace --all-targets --locked
cargo test --workspace --all-targets --locked
cargo test --doc --locked
cargo doc --workspace --no-deps --locked
cargo package --allow-dirty --list
cargo package --allow-dirty --locked --offline
```

`cargo check` performs checking without final code generation.
[Source: rust.cargo#cargo-check](https://doc.rust-lang.org/cargo/commands/cargo-check.html)
Cargo package listing and verification observe the assembled source package.
[Source: rust.cargo#cargo-package](https://doc.rust-lang.org/cargo/commands/cargo-package.html)

## Inputs Outputs And Exit Status

Inputs include manifests, lockfile, selected packages, targets, features,
toolchain, registry/cache state, build scripts, and environment. Successful
commands return zero; diagnostics and nonzero status identify compilation,
test, documentation, or packaging failure. Target selection differs among
commands, so record it explicitly.

## Mutation Cost And External Services

Cargo writes `target`, may populate registries and Git caches, and executes
build scripts and tests. Dependency resolution may use the network unless
offline inputs are available. Package verification builds extracted source.
Publishing and credentialed registry mutation are outside validation.

## Evidence Produced

Cargo can show that selected source checks, builds, tests, docs, or packages
succeed under recorded inputs. `--locked` adds evidence that resolution did not
update `Cargo.lock`. Package inspection adds evidence about shipped contents.

## Blind Spots

Success does not establish semantic correctness, unselected features or
targets, MSRV behavior on a newer compiler, installability on every consumer,
dependency safety, or binary reproducibility. Cached inputs and build scripts
can hide environmental effects.

## Related Knowledge

Use [Rust Validation](../../explanations/languages/rust.md), [Clippy](clippy.md),
[rustfmt](rustfmt.md), and the [Rust CLI Release Gate](../../how-to/recipes/rust-cli-release-gate.md).
