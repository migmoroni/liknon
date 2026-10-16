# Phase B04.3: Publish Ruby Bindings

[Back to the programmatic API and bindings plan](README.md)

## Purpose

Publish the `liknon` gem only when the approved idiomatic Ruby programmatic
contract and a representative consumer are ready.

## Design Gate

Prototype and select a native integration approach only after evaluating:

- supported Ruby implementations and versions;
- native extension or stable-library loading model;
- garbage collection and native-resource ownership;
- exception, callback, cancellation, and threading behavior;
- platform gem resolution and source-build policy;
- type-signature and documentation strategy.

Platform variants may contain the target-appropriate native library or
extension. The gem exposes no CLI executable and does not shell out to `liknon`.

## Verification

- `require` works in each supported Ruby environment.
- Representative APIs pass shared conformance fixtures.
- Ruby exceptions preserve the approved error taxonomy.
- Native resources survive garbage collection, interruption, and callback
  failure safely.
- Platform resolution retrieves only compatible native artifacts.
- The gem contains no CLI executable or authored Hub resource payload.

## Exit Criterion

Ruby applications consume Liknon as an in-process library through normal gem
dependencies, with no global command or hidden CLI transport.
