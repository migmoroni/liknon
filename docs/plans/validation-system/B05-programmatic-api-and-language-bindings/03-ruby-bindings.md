# Phase B05.3: Publish Ruby Bindings

## Status

`Future`. Blocked by an approved B05.0 Ruby scope.

[Back to the programmatic API and bindings plan](README.md)

## Purpose

Publish the `liknon` gem only when the approved idiomatic Ruby programmatic
contract and a representative consumer are ready.

## Design Gate

When the selected API requires native core behavior, prototype and select an
integration approach only after evaluating:

- supported Ruby implementations and versions;
- native extension or stable-library loading model;
- garbage collection and native-resource ownership;
- exception, callback, cancellation, and threading behavior;
- platform gem resolution and source-build policy;
- type-signature and documentation strategy.

When no selected capability requires native code, do not add an extension
merely for symmetry. Platform variants may contain the target-appropriate
native library or extension. The gem exposes no CLI executable and does not use
`liknon` as hidden subprocess transport.

If B05.0 selects Git-hook integration, expose idiomatic Ruby plan, diff, apply,
verification, and diagnostic objects over the shared
[integration contract](git-hook-integration.md). Installing or requiring the
gem must not mutate hook configuration, and generated hooks invoke the selected
native Validation command directly rather than start Ruby as a forwarder.

## Verification

- `require` works in each supported Ruby environment.
- Representative APIs pass shared conformance fixtures.
- Ruby exceptions preserve the approved error taxonomy.
- Native resources survive garbage collection, interruption, and callback
  failure safely.
- Platform resolution retrieves only compatible native artifacts.
- The gem contains no CLI executable or authored Hub resource payload.
- Approved hook adapters pass shared preservation, idempotency, explicit-apply,
  and absent-CLI fixtures.

## Exit Criterion

Ruby applications consume Liknon as an in-process library through normal gem
dependencies, with no global command or hidden CLI transport.
