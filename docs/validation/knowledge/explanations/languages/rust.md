# Rust Validation

## Scope

This guide covers Rust language and compiler-visible failure surfaces for a
maintained crate. Cargo dependency resolution, command behavior, formatting,
and lints remain separate tool concerns.

## Failure Modes

Relevant failures include syntax and type errors, ownership and lifetime
mistakes rejected by compilation, unsafe-code invariants, feature and target
combinations, conditional compilation, public API breakage, panic and error
paths, documentation examples, and behavior that differs at the declared
minimum Rust version.

## Validation Layers

Compile every promised target and feature set that materially changes code.
Use unit and integration tests for selected behavior, doctests for executable
examples, static analysis for modeled defect classes, and package/install
evidence for the deliverable. Exercise error paths and platform behavior that
compilation alone cannot observe.

## Evidence And Limitations

The manifest `rust-version` field communicates and participates in Cargo's
minimum-version behavior, but an active newer compiler does not establish that
minimum toolchain support. [Source: rust.cargo#rust-version](https://doc.rust-lang.org/cargo/reference/rust-version.html)

Compilation cannot prove semantic correctness, tests remain input-bounded, and
one host cannot establish every target. Build scripts and tests may execute
code, write caches or artifacts, use network services, or require native tools.

## Version Scope

Apply the guide to maintained Rust editions and toolchain lines declared by the
crate. Test the declared MSRV directly when it is a promise. Record the active
compiler and Cargo versions; do not make one workstation patch the public scope.

## Related Tools And Recipes

Use [Cargo](../../reference/tools/cargo.md), [Clippy](../../reference/tools/clippy.md),
and [rustfmt](../../reference/tools/rustfmt.md) for command semantics. Adapt the
[Rust CLI Release Gate](../../how-to/recipes/rust-cli-release-gate.md) when its
assumptions match.

## Next Step

Map each declared edition, MSRV, target, feature, API, and package promise to a
bounded observation and state what remains untested.
