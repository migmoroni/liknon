# pnpm

## Purpose And Scope

pnpm contributes dependency-graph, lockfile, package-script, and project-local
executable evidence for JavaScript and TypeScript workspaces. It does not prove
runtime behavior or dependency safety by itself.

## Version Lines And Sources

- pnpm 11: maintained TypeScript CLI line, reviewed against an immutable 11.x
  release and its versioned/living command documentation.
  [Source: pnpm.cli-11#release-line](https://github.com/pnpm/pnpm/releases/tag/v11.28.2)
- pnpm 12: maintained Rust CLI line, reviewed against the immutable 12.8.1
  release family. [Source: pnpm.cli-12#release-line](https://github.com/pnpm/pnpm/releases/tag/v12.8.1)

Both lines support the validation roles below, but implementation and some edge
behavior differ. Confirm the workspace's active line with `pnpm --version` and
consult that line's authoritative docs before using version-sensitive flags.

## Command Model

These representative forms separate lockfile, offline, script, and executable
intent; adapt package filtering and workspace scope.

```text
pnpm install --frozen-lockfile --offline
pnpm run <reviewed-script> -- <adapted-arguments>
pnpm exec <project-local-program> <adapted-arguments>
```

Frozen mode rejects an out-of-date lockfile rather than updating it.
[Source: pnpm.cli-11#frozen-lockfile](https://pnpm.io/11.x/cli/install#--frozen-lockfile)

## Inputs Outputs And Exit Status

Inputs include manifests, workspace configuration, lockfile, store state,
selected projects, scripts, environment, Node.js, and pnpm major line. A
nonzero status reports resolution, installation, script, or child-program
failure. Script and executable output belongs to repository-owned code, not
pnpm's evidence alone.

## Mutation Cost And External Services

Install can create or replace `node_modules`, update metadata, populate the
store, and execute lifecycle scripts. Without offline mode it can contact
registries. `run` and `exec` execute local code and may mutate arbitrary local
or external state. Publishing and credentialed registry operations are outside
validation.

## Evidence Produced

Frozen installation supports manifest/lockfile agreement under the selected
line and available package inputs. Offline mode supports absence of network
resolution for that run. Script and exec commands establish only their selected
child program's result.

## Blind Spots

A frozen lockfile does not prove package integrity, vulnerability absence,
license policy, lifecycle safety, or support on another platform. Offline
success depends on a populated store. Major lines can differ in implementation
and edge behavior even when command intent is shared.

## Related Knowledge

Use [Node.js Runtime Evidence](../../explanations/technologies/nodejs.md), the
[TypeScript Compiler](typescript-compiler.md), [Vitest](vitest.md), and the
[JavaScript/TypeScript Package Gate](../../how-to/recipes/javascript-typescript-package-gate.md).
