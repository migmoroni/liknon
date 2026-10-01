# workspace-validator

## Purpose And Scope

`workspace-validator` executes a workspace-owned declarative graph without a
shell and reports typed validation and repository-integrity evidence. It does
not select tools, infer coverage, or grant authority to configured commands.

## Version Lines And Sources

The embedded guide travels with the installed CLI's crate version. Confirm the
active version with `workspace-validator --version` and use documentation from
that release line. The registered immutable source snapshot supports the
current 0.1 report contract without narrowing this guide to one source commit.
[Source: workspace-validator.cli#structured-report](https://github.com/migmoroni/workspace-validator/blob/7bb373f9e57af82f2b5d857b933063f0b6c72d11/docs/validation/reference/reports.md)

## Command Model

The following representative forms inspect and execute explicit selections:

```text
workspace-validator config validate
workspace-validator list --tree
workspace-validator explain group <group-id>
workspace-validator validate <group-or-suite-id> --format=json
workspace-validator check <check-id> --format=json
workspace-validator knowledge show <document-id>
```

Replace bracketed placeholders with IDs returned by inspection commands.

## Inputs Outputs And Exit Status

Inputs include configuration, selected graph IDs, tool preflight, child
processes, timeouts, output bounds, cancellation, and Git observations. JSON is
the stable agent-facing result. Exit `0` is positive evidence, `1` negative
validation, `2` invalid configuration, `3` invalid usage, and `4` internal
failure.

## Mutation Cost And External Services

Read-only inspection and knowledge retrieval do not run configured tools.
Validation executes workspace-owned programs, which can consume network and
services, run untrusted build code, require privileges, or mutate state.
Repository-integrity reporting is observation, not sandboxing.

## Evidence Produced

The report establishes which configured prerequisites and checks ran, their
bounded outputs and statuses, and detected Git-visible mutations for that
selection and execution context.

## Blind Spots

A passing graph proves only its configured mechanisms. It does not establish
that the graph covers product risk, that ignored or external state is unchanged,
or that commands were safe. Shared knowledge never becomes execution policy.

## Related Knowledge

Use [Validation Strategy](../../explanations/foundations/validation-strategy.md)
before configuration and the [Rust CLI Release Gate](../../how-to/recipes/rust-cli-release-gate.md)
only when its public assumptions match the consumer workspace.
