# Liknon Validator

`liknon-validator` is the standalone validation CLI from the Liknon suite. It
validates declarative pipelines for heterogeneous workspaces, executes programs
without an implicit shell, and emits accessible human output or a versioned
JSON report.

Human output, help, accessible palettes, `NO_COLOR`, and low-vision behavior
follow the shared
[CLI experience contract](../../docs/cli-experience.md). Validation keeps its
report hierarchy, progress behavior, JSON schema, errors, and exit codes local.

The package installs two equivalent commands:

- `liknon-validator`, the explicit command;
- `likv`, the compact command.

Install an exact release from crates.io with locked dependencies:

```sh
cargo install liknon-validator --version "=<VERSION>" --locked
```

Configuration, command, report, security, and source-install documentation is
maintained in the canonical repository:
[github.com/migmoroni/liknon](https://github.com/migmoroni/liknon).

The Rust library target supports the package's binaries. It is not yet a
promised programmatic integration API.
