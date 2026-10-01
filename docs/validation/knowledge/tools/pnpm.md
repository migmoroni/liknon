# pnpm

## Identity And Reviewed Scope

- Executable/package: `pnpm` command-line package manager.
- Reviewed behavior: pnpm 11.x documentation snapshot; confirm the active
  version and lockfile format for the workspace.
- Evidence class: dependency graph consistency, workspace script invocation,
  and project-local binary selection.

## Useful Command Structure

```text
pnpm install --frozen-lockfile --offline
pnpm exec <project-binary> <arguments...>
pnpm run <script>
```

The frozen-lockfile option fails when the lockfile needs an update, and offline
mode restricts resolution to locally available package data.
[Source: pnpm.cli-11#frozen-lockfile](https://pnpm.io/cli/install#--frozen-lockfile)
`pnpm exec` exposes dependency binaries from the project rather than requiring
a global tool installation.
[Source: pnpm.cli-11#exec](https://pnpm.io/cli/exec)

## Output, Exit, Cost, And Mutation

Nonzero status reports resolution, script, or child-command failure. Install
can write the dependency store links and `node_modules`, run lifecycle scripts,
and consume substantial disk; offline does not make lifecycle code safe.
`pnpm run` executes repository-controlled scripts with process authority.
[Source: pnpm.cli-11#run-scripts](https://pnpm.io/cli/run)

## Blind Spots And Next Step

A frozen lockfile establishes consistency with declared dependency metadata,
not package integrity, vulnerability absence, correct runtime behavior, or
reproducible build bytes. Inspect scripts and authority first, then use the
smallest project-local compiler, test, or build command that supplies the
required evidence.
