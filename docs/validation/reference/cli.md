# CLI Reference

Inspection commands parse and semantically validate the complete configuration
but do not start configured programs:

```text
workspace-validator config validate [--config <path>]
workspace-validator list [--tree] [--config <path>]
workspace-validator explain group <group-id> [--config <path>]
workspace-validator explain suite <suite-id> [--config <path>]
workspace-validator explain check <check-id> [--config <path>]
workspace-validator schema config
workspace-validator schema report
```

Execution commands run configured programs after full validation and tool
preflight:

```text
workspace-validator validate [group-or-suite] [--config <path>] [--format=human|json]
workspace-validator check <check-id> [--config <path>] [--format=human|json]
```

Provisioning is separate from both categories:

```text
workspace-validator init --config <candidate-path> [--workspace <path>] [--format=human|json]
```

Human validation output is plain by default. `--color[=<palette>]` and
`--presentation=<mode>` apply only to execution output and are rejected with
`--format=json`. JSON execution output contains exactly one complete report on
standard output.
Standard-output delivery is part of command completion. If inspection output, a
schema, an initialization result, or a final validation report cannot be written
and flushed, the command returns exit code `4`; diagnostic output remains
best-effort and cannot turn that result into a panic exit. Invalid inspection
selections, such as an unknown `explain` target, remain configuration failures
with exit code `3`.
