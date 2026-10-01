# Vitest

## Purpose And Scope

Vitest contributes unit, component, and selected integration observations for
JavaScript and TypeScript under configured pools and environments.

## Version Lines And Sources

This guide covers the maintained Vitest 4 major line represented by the
registered living documentation snapshot. Confirm the project-local runner with
`pnpm exec vitest --version`; review materially different major lines
separately.

## Command Model

Use a one-shot run rather than watch mode for a bounded gate:

```text
pnpm exec vitest run
```

The `run` command performs one non-watch execution.
[Source: vitest.docs-4#run-mode](https://vitest.dev/guide/cli.html#vitest-run)

## Inputs Outputs And Exit Status

Inputs include discovery patterns, configuration, environment, pools, source,
fixtures, mocks, timers, snapshots, and child dependencies. Output reports
discovered files, tests, failures, and duration; failing tests return nonzero.
Per-test timeouts are configurable and bound only the documented test phase.
[Source: vitest.docs-4#test-timeout](https://vitest.dev/config/testtimeout.html)

## Mutation Cost And External Services

Configuration can write snapshots, coverage, caches, and generated output or
start browser and service dependencies. Tests can mutate arbitrary local or
remote state. Record worker cost, network, credentials, and cleanup.

## Evidence Produced

A pass supports selected behavioral claims for discovered tests, configured
environment, fixtures, and oracles under that run.

## Blind Spots

Passing Node-based tests does not establish browser behavior, type correctness,
packaging, every supported Node.js line, or undiscovered inputs. Mocks and fake
timers can hide real integration behavior.

## Related Knowledge

Use [JavaScript Validation](../../explanations/languages/javascript.md),
[TypeScript Validation](../../explanations/languages/typescript.md),
[Node.js Runtime Evidence](../../explanations/technologies/nodejs.md), and the
[JavaScript/TypeScript Package Gate](../../how-to/recipes/javascript-typescript-package-gate.md).
