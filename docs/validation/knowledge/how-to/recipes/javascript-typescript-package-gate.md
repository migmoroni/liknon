# JavaScript/TypeScript Package Gate

## Goal

Create a non-publishing validation gate for one pnpm-managed Node.js package
that authors TypeScript, runs Vitest, and produces a reviewed build artifact.

## Applicability And Preconditions

This worked recipe covers a pnpm-managed Node.js package that authors
TypeScript, runs Vitest, and exposes a reviewed `build` script. It assumes
`package.json`, `pnpm-lock.yaml`, `tsconfig.json`, the package scripts, and the
supported Node.js major are maintained together. It is a bounded example, not
global policy. Install the declared Node.js and pnpm major lines and populate
the pnpm store before using offline mode.

## Risks And Invariants

- Dependency installation must agree with the committed lockfile and use only
  already available package data during this gate.
- The intended TypeScript project must type-check without generated output.
- Behavioral tests must execute once outside watch mode.
- The maintained build script must produce its declared artifact.
- Git-visible state before and after the gate must expose unintended tracked or
  untracked mutations.

This is a local composition of the [Evidence Chain](../../explanations/patterns/evidence-chain.md),
[Isolation And Determinism](../../explanations/patterns/isolation-and-determinism.md),
[JavaScript](../../explanations/languages/javascript.md),
[TypeScript](../../explanations/languages/typescript.md), and
[Node.js](../../explanations/technologies/nodejs.md). Frozen lockfile mode prevents pnpm
from silently updating a lockfile that disagrees with the manifests.
[Source: pnpm.cli-11#frozen-lockfile](https://pnpm.io/11.x/cli/install#--frozen-lockfile)

## Inputs And Workspace Assumptions

Run in the package root with the declared Node.js and pnpm versions already
installed. The dependency check can recreate or modify `node_modules` and run
reviewed lifecycle scripts; it is not a read-only operation even in offline
mode. Type checking then avoids emission, Vitest runs once, and the explicit
build script produces the deliverable. `pnpm run` executes repository-owned
script content, so inspect that content before authorizing the gate.
[Source: pnpm.cli-11#run-scripts](https://pnpm.io/11.x/cli/run)

The example assumes the configuration file lives in `.validation/config.json`,
so `workspaceRoot: ".."` resolves to the package root. Replace the declared
major ranges, config path, script name, timeouts, and working directory when the
consumer's maintained workspace differs.

## Procedure

1. Review `package.json`, lifecycle scripts, `pnpm-lock.yaml`, `tsconfig.json`,
   Vitest configuration, and the build output contract.
2. Confirm the active Node.js and pnpm major lines and authorize dependency and
   lifecycle mutations.
3. Save the adapted configuration below and run `workspace-validator config
   validate`.
4. Inspect the resolved suite, then run `workspace-validator validate
   package-gate --format=json`.
5. Interpret each failed dependency, type, test, build, or repository result
   separately; do not retry toward a pass.

## Complete Configuration Example

```json
{
  "schemaVersion": 6,
  "workspaceRoot": "..",
  "defaultGroup": "package-gate",
  "outputLimitBytes": 1048576,
  "repository": {
    "provider": "git",
    "toolId": "git",
    "detectMutations": true
  },
  "tools": [
    {
      "id": "node",
      "program": "node",
      "requiresTools": [],
      "versionArgs": ["--version"],
      "versionParser": "firstSemver",
      "versionRequirement": ">=22, <23"
    },
    {
      "id": "pnpm",
      "program": "pnpm",
      "requiresTools": ["node"],
      "versionArgs": ["--version"],
      "versionParser": "firstSemver",
      "versionRequirement": ">=11, <12"
    },
    {
      "id": "git",
      "program": "git",
      "requiresTools": [],
      "versionArgs": ["--version"],
      "versionParser": "firstSemver",
      "versionRequirement": ">=2, <3"
    }
  ],
  "checks": [
    {
      "id": "dependencies.locked",
      "label": "Locked dependencies",
      "description": "Requires manifests and lockfile to agree without network resolution.",
      "toolId": "pnpm",
      "args": ["install", "--frozen-lockfile", "--offline"],
      "requiresTools": [],
      "timeoutSeconds": 900
    },
    {
      "id": "typescript.check",
      "label": "TypeScript project",
      "description": "Checks the intended TypeScript project without emitting output.",
      "toolId": "pnpm",
      "args": ["exec", "tsc", "--noEmit", "--project", "tsconfig.json"],
      "requiresTools": [],
      "timeoutSeconds": 600
    },
    {
      "id": "vitest.run",
      "label": "Vitest",
      "description": "Runs the package tests once outside watch mode.",
      "toolId": "pnpm",
      "args": ["exec", "vitest", "run"],
      "requiresTools": [],
      "timeoutSeconds": 900
    },
    {
      "id": "package.build",
      "label": "Package build",
      "description": "Runs the maintained package build script.",
      "toolId": "pnpm",
      "args": ["run", "build"],
      "requiresTools": [],
      "timeoutSeconds": 900
    }
  ],
  "suites": [
    {
      "id": "typescript.package",
      "label": "JavaScript/TypeScript package",
      "description": "Produces dependency, type, test, build, and repository evidence.",
      "workingDirectory": ".",
      "checks": [
        { "checkId": "dependencies.locked", "dependsOn": [] },
        { "checkId": "typescript.check", "dependsOn": ["dependencies.locked"] },
        { "checkId": "vitest.run", "dependsOn": ["typescript.check"] },
        { "checkId": "package.build", "dependsOn": ["vitest.run"] }
      ]
    }
  ],
  "groups": [
    {
      "id": "package-gate",
      "label": "Package gate",
      "description": "Runs the bounded JavaScript/TypeScript package example.",
      "members": [{ "kind": "suite", "id": "typescript.package" }]
    }
  ]
}
```

## Expected Evidence

A successful run reports manifest/lockfile agreement using available offline
inputs, a non-emitting check of the selected TypeScript project, one bounded
Vitest execution, a successful reviewed build script, and no new Git-visible
mutation beyond the run's initial state.

## Failure Interpretation

An installation failure may indicate lockfile drift or an incomplete offline
store. A compiler failure concerns the selected project graph. A Vitest failure
concerns its discovery, environment, fixture, or oracle. A build failure concerns
the repository-owned script. Repository-integrity failure means the gate changed
Git-visible state and requires inspection before acceptance.

## Limitations And Residual Risk

The gate does not establish browser or other Node.js versions, dependency
vulnerability or license status, package publication, installability from a
registry archive, performance, accessibility, or bit-for-bit reproducible
bundles. Offline installation depends on a populated local store and lifecycle
scripts can mutate state that Git does not observe. Vitest coverage remains
bounded by discovery, environment, fixtures, and oracles.
[Source: vitest.docs-4#run-mode](https://vitest.dev/guide/cli.html#vitest-run)

Add a platform matrix, archive inspection, browser execution, or supply-chain
evidence only when product promises or observed failures require it. Publishing
and accepting generated changes are explicitly rejected as implicit validation
steps.
