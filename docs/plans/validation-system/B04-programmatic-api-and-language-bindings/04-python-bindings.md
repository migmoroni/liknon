# Phase B04.4: Publish Python Bindings

[Back to the programmatic API and bindings plan](README.md)

## Purpose

Publish the PyPI `liknon` project only when the approved idiomatic
programmatic Python module and a representative consumer are ready.

## Design Gate

Compare native extension and stable-ABI options against:

- supported Python versions and implementations;
- wheel compatibility and platform policy;
- typing and documentation quality;
- GIL, threading, async, cancellation, and callback behavior;
- exception mapping and native-resource ownership;
- source-distribution policy and reproducible native builds.

Wheels may contain target-specific native extension or library artifacts. They
do not install the Liknon CLI command and do not use a Python launcher to invoke
it.

## Verification

- `import liknon` works in every declared environment.
- Type checking and representative APIs pass their supported contracts.
- Shared fixtures produce equivalent domain outcomes.
- Exceptions, cancellation, interpreter shutdown, and native cleanup are safe.
- Wheel inspection and isolated installation prove correct platform selection.
- The project contains no CLI executable or authored Hub resource payload.

## Exit Criterion

Python applications import Liknon and use its supported functionality in
process through an idiomatic, typed API.
