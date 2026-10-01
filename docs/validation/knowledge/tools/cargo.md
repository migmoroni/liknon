# Cargo

## Identity And Reviewed Scope

- Executable/package: `cargo`, distributed with the Rust toolchain.
- Reviewed command family: Cargo 1.87 and later behavior documented for
  `check`, `test`, `doc`, and `package`; confirm the installed version and
  project MSRV before use.
- Evidence class: Rust compilation, tests, documentation build, dependency
  resolution constraints, and package construction/inspection.

## Useful Command Structure

```text
cargo check --workspace --all-targets --locked
cargo test --workspace --all-targets --locked
cargo test --doc --locked
cargo doc --workspace --no-deps --locked
cargo package --locked --list
cargo package --locked
```

Cargo documents `check` as checking a local package and dependencies for errors
without the final code-generation step, so it is faster evidence than a build
but not a deliverable.
[Source: rust.cargo#cargo-check](https://doc.rust-lang.org/cargo/commands/cargo-check.html)
`cargo package --list` lists files that would enter the package, while package
verification builds from the generated archive unless disabled.
[Source: rust.cargo#cargo-package](https://doc.rust-lang.org/cargo/commands/cargo-package.html)

## Output, Exit, Cost, And Mutation

Success exits zero; command or validation failure exits nonzero and diagnostics
normally go to standard error. Cargo may write `target/`, update resolution
without `--locked`, access registries without offline controls, and run build
scripts or test executables. Treat code and build scripts as executable input.

## Blind Spots And Overlap

Cargo orchestrates mechanisms; it does not make their oracles complete. `check`
overlaps compilation performed by test/build but does not execute tests.
Package success does not establish archive policy unless the contents are
inspected. One toolchain/host does not establish the declared platform matrix.

## Next Step

Choose target, feature, network, and lockfile flags from the actual claim. Pair
with [Clippy](clippy.md), [rustfmt](rustfmt.md), and the
[Rust language guide](../languages/rust.md) only where their evidence differs.
