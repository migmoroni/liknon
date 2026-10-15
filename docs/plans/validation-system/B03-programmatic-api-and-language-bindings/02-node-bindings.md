# Phase B03.2: Publish Node.js Bindings

[Back to the programmatic API and bindings plan](README.md)

## Purpose

Expose the approved Liknon programmatic contract as an idiomatic importable
JavaScript/TypeScript package.

## Design Gate

Before implementation, compare the feasible native bridge options against:

- Node runtime and module-format support;
- async execution, cancellation, callbacks, and thread safety;
- TypeScript declaration quality;
- native artifact selection and install behavior;
- error and report fidelity;
- cross-platform build and maintenance cost.

The package may use target-specific auxiliary packages when required by the
chosen native technology. Those packages carry native library or extension
artifacts, never the Liknon CLI executable, and expose no global `bin` entry.

## Verification

- CommonJS and/or ESM behavior matches the approved support contract.
- TypeScript consumers receive exact public types.
- Representative read-only and execution APIs pass shared conformance fixtures.
- Installation selects only the matching native artifact and performs no hidden
  CLI invocation.
- Exceptions, cancellation, streams, callbacks, and process cleanup behave
  safely.

## Exit Criterion

Node.js applications import Liknon through the normal package dependency model
and receive idiomatic in-process behavior conforming to B03.0.
