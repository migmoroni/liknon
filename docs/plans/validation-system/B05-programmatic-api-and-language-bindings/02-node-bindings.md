# Phase B05.2: Publish Node.js Bindings

## Status

`Future`. Blocked by an approved B05.0 Node.js scope.

[Back to the programmatic API and bindings plan](README.md)

## Purpose

Publish npm package `liknon` only when the approved idiomatic
JavaScript/TypeScript programmatic contract and a representative consumer are
ready.

## Design Gate

When the selected API requires native core behavior, compare feasible native
bridge options against:

- Node runtime and module-format support;
- async execution, cancellation, callbacks, and thread safety;
- TypeScript declaration quality;
- native artifact selection and install behavior;
- error and report fidelity;
- cross-platform build and maintenance cost.

If B05.0 selects Git-hook integration, expose idiomatic TypeScript plan, diff,
apply, verification, and diagnostic types over the shared
[integration contract](git-hook-integration.md). Importing or installing the
package must not mutate hook configuration, and generated hooks must invoke the
selected native Validation command directly rather than start Node.js as a
forwarder.

When no selected capability requires native code, do not add a bridge merely
for symmetry. The package may use target-specific auxiliary packages when
required by the chosen native technology. Those packages carry native library
or extension artifacts, never the Liknon CLI executable, and expose no global
`bin` entry.

## Verification

- CommonJS and/or ESM behavior matches the approved support contract.
- TypeScript consumers receive exact public types.
- Representative read-only and execution APIs pass shared conformance fixtures.
- Installation selects only the matching native artifact and performs no hidden
  CLI invocation.
- The package contains no CLI executable or authored Hub resource payload.
- Approved hook adapters pass shared preservation, idempotency, explicit-apply,
  and absent-CLI fixtures.
- Exceptions, cancellation, streams, callbacks, and process cleanup behave
  safely.

## Exit Criterion

Node.js applications import Liknon through the normal package dependency model
and receive idiomatic in-process behavior conforming to B05.0.
