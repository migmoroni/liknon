# rustfmt

## Purpose And Scope

rustfmt supplies mechanical formatting evidence for selected Rust source. Check
mode compares formatting without intentionally rewriting files.

## Version Lines And Sources

rustfmt follows the installed Rust toolchain. Run the version line required by
the workspace and record `rustfmt --version` when byte stability matters. The
living rustfmt documentation is provenance; one reviewed patch does not define
guide applicability.

## Command Model

Use the following check form for a non-mutating gate:

```text
cargo fmt --all -- --check
```

rustfmt documents check mode for CI use.
[Source: rust.rustfmt#check-mode](https://github.com/rust-lang/rustfmt#checking-style-on-a-ci-server)

## Inputs Outputs And Exit Status

Inputs include selected Rust files, edition, toolchain, and rustfmt
configuration. A mismatch or parse failure returns nonzero and may print diffs.

## Mutation Cost And External Services

Check mode reads source and uses no external service or network. Omitting
`--check` rewrites source and requires separate authorization.

## Evidence Produced

A pass supports the claim that selected source already matches that rustfmt
version and configuration.

## Blind Spots

Formatting says nothing about compilation, behavior, API quality, or complete
style policy. Parser failure is not a style finding. Toolchain changes can
change formatted bytes.

## Related Knowledge

Use [Rust Validation](../../explanations/languages/rust.md) for other layers and
[Cargo](cargo.md) for workspace selection.
