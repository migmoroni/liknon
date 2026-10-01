# Clippy

## Purpose And Scope

Clippy contributes compiler-integrated lint evidence for selected Rust packages,
targets, features, and lint levels. It does not replace compilation or behavior
tests.

## Version Lines And Sources

Clippy follows the installed Rust toolchain rather than an independent public
major line. Validate with each maintained toolchain line whose diagnostics are
material, including the MSRV when required. Confirm it with
`cargo clippy --version`. The living Clippy usage guide is the source baseline.

## Command Model

The following representative form checks all selected targets and denies
warnings; adapt feature and package selection.

```text
cargo clippy --workspace --all-targets --locked -- -D warnings
```

Clippy defines allow, warn, deny, and forbid levels and supports project lint
configuration. [Source: rust.clippy#lint-levels](https://doc.rust-lang.org/clippy/usage.html#lint-configuration)

## Inputs Outputs And Exit Status

Inputs include Rust source, Cargo selection, compiler/Clippy version, enabled
lints, configuration, dependencies, build scripts, and environment. Diagnostics
name lint findings; denied findings and compilation failures return nonzero.

## Mutation Cost And External Services

Clippy writes build artifacts and may run build scripts. Dependency access can
use caches or the network according to Cargo flags. It should not rewrite
source, but configured build logic can mutate other state.

## Evidence Produced

A passing run supports the claim that selected code produced no findings at the
configured levels under that toolchain and target selection.

## Blind Spots

Lint coverage is rule- and version-bounded. Allowed lints, unselected features,
runtime behavior, public contracts, unsafe invariants, and external systems
remain outside. `-D warnings` can make a toolchain update a gate change.

## Related Knowledge

Use [Cargo](cargo.md) for selection semantics, [Rust Validation](../../explanations/languages/rust.md)
for evidence layers, and behavior tests for claims lints cannot observe.
