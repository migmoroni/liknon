# Human Flow

The Human Flow covers authoring, inspection, execution, and correction without
an AI component. Configuration is trusted executable policy and must be
reviewed like code before any execution command.

## Author And Initialize

Start from a complete candidate. The validator does not invent project tools,
checks, suites, or groups. Representative candidates are available for a
[single crate](../../examples/single-crate.json), a
[heterogeneous workspace](../../examples/heterogeneous-workspace.json), and
[shared suites](../../examples/shared-suites.json).

```sh
liknon init --config validation.config.json
liknon config validate
```

`init` only installs validated bytes. For an existing canonical configuration,
start at `config validate`; `init` is not an edit or replacement command.

## Inspect Before Running

```sh
liknon list --tree
liknon explain group <group-id>
liknon explain suite <suite-id>
liknon explain check <check-id>
```

These commands start no configured process. Confirm ownership, reuse,
directories, parameters, dependencies, program names, and argument vectors.

## Execute And Interpret

```sh
liknon validate <group-or-suite>
liknon check <check-id>
liknon validate <group-or-suite> --format=json
```

Use the smallest intended selection. Read failed results first, then blocked,
skipped, and passing results. Check exit codes, timeouts, truncation, diagnostics,
and repository mutations. A pass covers only counted evidence in the selection.

Correct environment, configuration, or source under human control. Do not
expect the validator to install tools or repair the workspace. Rerun the
smallest affected check or suite, then run the final gate required by the
workspace or user. Do not rerun unchanged failures or duplicate a group by
manually invoking each member.
