# Rust Validation

## Question And Applicability

What language-specific evidence is useful for a maintained Rust crate or
workspace? Apply this guide after confirming `Cargo.toml`, the supported Rust
version, targets, feature policy, and deliverables. It does not prescribe
commands for non-Cargo builds or prove unsafe code sound.

## Failure Modes And Evidence

- Compilation and type checking can expose invalid types, names, cfg branches,
  and selected target incompatibilities.
- Unit, integration, and documentation tests observe different Rust/Cargo test
  boundaries. Cargo documents how target selection and doctests are executed.
  [Source: rust.cargo#cargo-test-targets](https://doc.rust-lang.org/cargo/commands/cargo-test.html#target-selection)
- Lints identify compiler- and Clippy-modeled defect/style classes.
- Formatting checks establish conformity to the selected rustfmt toolchain.
- MSRV checks provide direct evidence that selected targets compile with the
  declared minimum toolchain. Cargo's `rust-version` field communicates that
  minimum but does not itself execute the compiler matrix.
  [Source: rust.cargo#rust-version](https://doc.rust-lang.org/cargo/reference/rust-version.html)
- Packaging and archive inspection detect missing, extra, or generated release
  assets that compilation does not observe.

## Limitations And Combinations

One host and toolchain cannot establish every supported target. A clean
`cargo check` does not execute behavior; tests do not establish formatting,
packaging, licensing, supply-chain integrity, or performance. Feature and target
matrices should come from promises and failure consequences, not maximal
combinatorics.

## Selection Guidance

Use locked resolution where reproducibility matters, run lint/test/build
commands against explicit target sets, and inspect the package that will be
distributed. Record whether examples, benches, doctests, optional features,
cross targets, or unsafe code remain outside the selected evidence.

## Next Steps

Confirm exact commands in [Cargo](../tools/cargo.md), [Clippy](../tools/clippy.md),
and [rustfmt](../tools/rustfmt.md), then adapt the
[Rust CLI release recipe](../recipes/rust-cli-release-gate.md) only if its
assumptions match.
