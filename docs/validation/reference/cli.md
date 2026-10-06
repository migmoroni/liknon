# CLI Reference

Inspection commands parse and semantically validate the complete configuration
but do not start configured programs:

```text
liknon config validate [--config <path>]
liknon list [--tree] [--config <path>]
liknon explain group <group-id> [--config <path>]
liknon explain suite <suite-id> [--config <path>]
liknon explain check <check-id> [--config <path>]
liknon schema config
liknon schema report
```

Embedded knowledge commands are read-only and operate before configuration
discovery. They work outside an initialized workspace, do not inspect or create
`.validation/`, and do not execute configured programs:

```text
liknon knowledge catalog [--format=human|json]
liknon knowledge show <document-id>
```

The JSON catalog is the complete embedded canonical `catalog.json`.
`knowledge show` accepts only a stable catalog document ID and writes the exact
canonical Markdown bytes. Malformed or unknown selectors return exit code `3`;
embedded-asset or output failures return exit code `4`.

JSON consumers must inspect each entry's `status`. Draft entries guarantee
`id`, `kind`, `path`, and `status` and may omit the remaining routing metadata;
reviewed entries provide every metadata field required by the catalog schema.

Execution commands run configured programs after full validation and tool
preflight:

```text
liknon validate [group-or-suite] [--config <path>] [--format=human|json]
liknon check <check-id> [--config <path>] [--format=human|json]
```

Provisioning is separate from both categories:

```text
liknon init --config <candidate-path> [--workspace <path>] [--format=human|json]
```

Human CLI output is plain by default. The global `--color[=<palette>]` and
`--presentation=<mode>` options style requested help independently through
semantic color and layout. The root help documents both renderer controls;
focused help indicates them concisely only for `validate` and `check`, where
they also style human execution. Every focused command help documents its own
`-h, --help`, while the consolidated root reference lists that option once.
An explicit palette overrides `NO_COLOR`, while low-vision presentation may use
ANSI typographic emphasis without adding hue. Visual options are rejected
during other command executions and with JSON validation reports; JSON
execution output contains exactly one complete report on standard output.

Standard-output delivery is part of command completion. If inspection output, a
schema, an initialization result, or a final validation report cannot be written
and flushed, the command returns exit code `4`; diagnostic output remains
best-effort and cannot turn that result into a panic exit. Invalid inspection
selections, such as an unknown `explain` target, remain configuration failures
with exit code `3`.
