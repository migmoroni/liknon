# Phase B05.4: Publish Python Bindings

## Status

`Future`. Blocked by an approved B05.0 Python scope.

[Back to the programmatic API and bindings plan](README.md)

## Purpose

Publish the PyPI `liknon` project only when the approved idiomatic
programmatic Python module and a representative consumer are ready.

## Design Gate

When the selected API requires native core behavior, compare native extension
and stable-ABI options against:

- supported Python versions and implementations;
- wheel compatibility and platform policy;
- typing and documentation quality;
- GIL, threading, async, cancellation, and callback behavior;
- exception mapping and native-resource ownership;
- source-distribution policy and reproducible native builds.

When no selected capability requires native code, do not add an extension
merely for symmetry. Wheels may contain target-specific native extension or
library artifacts. They do not install the Liknon CLI command and do not use a
Python launcher to invoke it.

If B05.0 selects Git-hook integration, expose idiomatic typed plan, diff,
apply, verification, and diagnostic objects over the shared
[integration contract](git-hook-integration.md). Installing or importing the
project must not mutate hook configuration, and generated hooks invoke the
selected native Validation command directly rather than start Python as a
forwarder.

## Verification

- `import liknon` works in every declared environment.
- Type checking and representative APIs pass their supported contracts.
- Shared fixtures produce equivalent domain outcomes.
- Exceptions, cancellation, interpreter shutdown, and native cleanup are safe.
- Wheel inspection and isolated installation prove correct platform selection.
- The project contains no CLI executable or authored Hub resource payload.
- Approved hook adapters pass shared preservation, idempotency, explicit-apply,
  and absent-CLI fixtures.

## Exit Criterion

Python applications import Liknon and use its supported functionality in
process through an idiomatic, typed API.
