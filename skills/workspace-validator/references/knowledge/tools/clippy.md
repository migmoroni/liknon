# Clippy

## Identity And Reviewed Scope

- Executable/package: `cargo clippy`, implemented by the `clippy-driver`
  component in the Rust toolchain.
- Reviewed range: Rust/Clippy 1.87 and later; use the component paired with the
  selected toolchain.
- Evidence class: compiler and Clippy lint findings for selected crates,
  targets, features, and lint levels.

## Useful Command Structure

```text
cargo clippy --workspace --all-targets --locked -- -D warnings
```

Clippy groups lints and permits levels such as allow, warn, deny, and forbid;
the selected level determines whether findings fail the command.
[Source: rust.clippy#lint-levels](https://doc.rust-lang.org/clippy/usage.html#lint-configuration)

## Output, Exit, Cost, And Mutation

Diagnostics identify lint, location, explanation, and often a suggestion.
Denied findings or compilation failure produce nonzero status. Clippy compiles
selected targets, may run build scripts, and writes build artifacts. Suggested
fixes are not evidence until independently reviewed and validated.

## Blind Spots And Overlap

Clippy observes only enabled/modelled lint classes and compiled cfg/feature
paths. A clean run does not establish behavioral correctness, memory safety of
all unsafe code, portability, accessibility, or supply-chain integrity. It
overlaps compiler checks but adds distinct lint analyses.

## Next Step

Confirm target/feature coverage and repository lint configuration. Treat new
lint noise after toolchain upgrades as a review decision, not permission to
silence findings automatically.
