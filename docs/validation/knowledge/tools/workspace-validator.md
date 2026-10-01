# workspace-validator

## Identity And Reviewed Scope

- Executable/package: `workspace-validator` crate and CLI.
- Reviewed version: `0.1.0`, compatible range `>=0.1.0, <0.2.0` as declared by
  the distributed skill manifest.
- Evidence class: deterministic orchestration and reporting of a
  workspace-owned declarative validation graph.

## Useful Command Structure

```text
workspace-validator config validate
workspace-validator list --tree
workspace-validator explain group <id>
workspace-validator validate <group-or-suite> --format=json
workspace-validator check <check-id> --format=json
```

The canonical local CLI contract defines structured reports, exit statuses,
selection, and inspection behavior.
[Source: workspace-validator.cli#structured-report](https://github.com/migmoroni/workspace-validator/blob/main/docs/validation/reference/reports.md)

## Output, Exit, Cost, And Mutation

JSON output is the stable agent-facing result. Exit `0` is positive, `1` is
negative validation evidence, `2` invalid configuration, `3` invalid usage,
and `4` internal failure. The validator itself coordinates child processes;
configured commands may consume network/services, execute untrusted build code,
or mutate the workspace. Repository mutation detection observes Git-visible
changes but is not a sandbox.

## Blind Spots And Overlap

The runtime does not select tools, define product risk, infer coverage, or turn
a configured check into adequate evidence. A passing graph proves only its
configured mechanisms under that run. Shared knowledge never changes runtime
selection or execution.

## Next Step

Establish evidence needs first, inspect the resolved graph, then execute one
explicit selection. Use the product
[reference documentation](https://github.com/migmoroni/workspace-validator/tree/main/docs/validation/reference)
for exhaustive command contracts rather than this evidence-oriented guide.
