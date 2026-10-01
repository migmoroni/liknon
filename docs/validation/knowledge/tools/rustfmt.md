# rustfmt

## Identity And Reviewed Scope

- Executable/package: `rustfmt`, commonly invoked through `cargo fmt`.
- Reviewed range: rustfmt paired with Rust 1.87 and later for stable formatting
  options used by the repository.
- Evidence class: Rust source formatting conformance.

## Useful Command Structure

```text
cargo fmt --all -- --check
```

The rustfmt maintainers document `cargo fmt --all -- --check` for checking all
packages without writing formatted output.
[Source: rust.rustfmt#check-mode](https://github.com/rust-lang/rustfmt#checking-style-on-a-ci-server)

## Output, Exit, Cost, And Mutation

Check mode exits nonzero when files differ from rustfmt output and does not
intentionally rewrite them. Omitting `--check` mutates source. Output may include
diffs. Formatting depends on the selected toolchain, edition, and supported
configuration.

## Blind Spots And Overlap

Formatting says nothing about compilation, behavior, diagnostics, or API
quality. Parser failure may block formatting and should not be interpreted as a
style finding. Toolchain changes may alter formatting, so pin or record the
reviewed version when byte stability matters.

## Next Step

Run check mode before lint/test evidence and keep any formatting mutation a
separately authorized action.
