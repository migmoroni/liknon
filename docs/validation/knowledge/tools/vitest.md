# Vitest

## Identity And Reviewed Scope

- Executable/package: `vitest` from the `vitest` package, invoked locally.
- Reviewed behavior: Vitest 4.1.10 documentation snapshot.
- Evidence class: JavaScript/TypeScript unit, component, and selected
  integration behavior under configured pools and environments.

## Useful Command Structure

```text
pnpm exec vitest run
```

`vitest run` performs a single non-watch execution suitable for a bounded gate.
[Source: vitest.docs-4#run-mode](https://vitest.dev/guide/cli.html#vitest-run)

## Output, Exit, Cost, And Mutation

The command reports discovered files, tests, failures, and duration and exits
nonzero on a failing run. Configuration can enable coverage, snapshots,
parallel workers, browser/service dependencies, or generated caches. Per-test
timeouts are configurable and bound only individual test execution as defined
by the runner.
[Source: vitest.docs-4#test-timeout](https://vitest.dev/config/testtimeout.html)

## Blind Spots And Next Step

Evidence is limited by discovery, environment, mocks, timers, concurrency,
fixtures, and assertions. Passing Node-based tests does not establish browser
behavior, type correctness, packaging, or every supported Node version. Inspect
configuration, choose explicit environments, and combine with compiler and
build evidence only where the claim requires it.
