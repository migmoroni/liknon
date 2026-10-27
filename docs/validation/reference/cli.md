# CLI Reference

Inspection commands parse and semantically validate the complete configuration
but do not start configured programs:

```text
liknon-validator config validate [--config <path>]
liknon-validator list [--tree] [--config <path>]
liknon-validator explain group <group-id> [--config <path>]
liknon-validator explain suite <suite-id> [--config <path>]
liknon-validator explain check <check-id> [--config <path>]
liknon-validator schema config
liknon-validator schema report

Execution commands run configured programs after full validation and tool
preflight:

```text
liknon-validator validate [group-or-suite] [--config <path>] [--format=human|json]
liknon-validator check <check-id> [--config <path>] [--format=human|json]
```

Provisioning is separate from both categories:

```text
liknon-validator init --config <candidate-path> [--workspace <path>] [--format=human|json]
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
