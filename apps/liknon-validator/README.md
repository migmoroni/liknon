# Liknon Validator

`liknon-validator` is the validation capability of the Liknon CLI suite. It
validates declarative pipelines for heterogeneous workspaces, executes programs
without an implicit shell, and emits accessible human output or a versioned
JSON report.

Human output, help, accessible palettes, `NO_COLOR`, and low-vision behavior
follow the shared
[CLI experience contract](../../docs/cli-experience.md). Validation keeps its
report hierarchy, progress behavior, JSON schema, errors, and exit codes local.

The public `liknon-cli` package installs `liknon-validator` beside the `liknon`
launcher and the other capability executables. Install the complete suite from
the repository checkout with:

```sh
cargo install --path apps/liknon-cli --locked --force
liknon-validator --version
liknon validator --version
```

Configuration, command, report, security, and source-install documentation is
maintained in the canonical repository:
[github.com/migmoroni/liknon](https://github.com/migmoroni/liknon).

The `liknon-validator` Rust package is a publishable implementation dependency
of `liknon-cli`; it owns no public binary target. Its library surface supports
the suite and is not yet a promised programmatic integration API.
