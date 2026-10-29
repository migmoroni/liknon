# Liknon CLI

`liknon-cli` installs the complete Liknon command-line suite:

```text
liknon
liknon-validator
liknon-manager
liknon-tool
```

The three capability executables remain directly invocable. `liknon` is a
capability-free launcher that routes an explicit namespace to the sibling
executable from the same installation:

```sh
liknon validator validate
liknon manager list
liknon tool catalog
```

The compact namespace forms are `liknon v`, `liknon m`, and `liknon t`. They
are parser aliases, not additional executables.

Install the complete suite from crates.io after its first public release:

```sh
cargo install liknon-cli --locked
```

From a source checkout:

```sh
cargo install --path apps/liknon-cli --locked --force
```

All four executables belong to one suite version. The launcher resolves only
siblings beside itself, verifies their reported versions, forwards remaining
arguments and process context, and returns the selected capability's status.
