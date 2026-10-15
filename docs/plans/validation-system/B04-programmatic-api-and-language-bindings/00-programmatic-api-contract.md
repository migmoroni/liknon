# Phase B04.0: Define The Programmatic API Contract

[Back to the programmatic API and bindings plan](README.md)

## Purpose

Determine which Liknon behavior is reusable in process and define a stable core
contract before exposing any language package or ABI.

## Status

Discovery only. Do not implement bindings or configure registry publication
until this phase has an approved exit record.

## Required Design Work

### Use Cases

- Identify concrete host-application workflows for configuration, inspection,
  planning, execution, reporting, core schemas, and explicit initialization
  resource provisioning.
- Separate reusable use cases from terminal presentation and shell-oriented
  ergonomics.
- Define which operations are read-only, which may execute external programs,
  and which may mutate a workspace.

### API Semantics

- Require an explicit workspace root and explicit inputs.
- Define sync versus async entry points.
- Define cancellation, timeout, progress-event, and output-capture behavior.
- Define stable result and diagnostic types.
- Define deterministic serialization where callers need persisted evidence.
- Define thread-safety, reentrancy, callback, and resource-lifetime rules.

### Failure Boundaries

- Establish a typed error taxonomy independent from CLI exit codes.
- Ensure Rust panics and foreign-language exceptions cannot cross an FFI
  boundary unsafely.
- Define behavior for invalid paths, malformed configuration, unavailable tools,
  interrupted processes, partial workspace mutation, and callback failure.

### Packaging And Compatibility

- Decide crate and module boundaries.
- Decide whether bindings share one C-compatible ABI, use ecosystem-specific
  native extension systems, or combine approaches.
- Version the Rust API, native ABI, serialized contracts, and language wrappers
  explicitly.
- Decide how validation CLI, knowledge CLI, B03 ecosystem package, API, ABI,
  and wrapper versions relate while remaining explicit compatibility surfaces.
- Preserve the existing B03 resource API and namespace when extending each
  package.
- Define supported targets and runtime baselines from measured package needs,
  not by copying B01's executable matrix automatically.

## Prototype Requirements

Prototype at least one representative read-only operation and one operation
that executes a validation plan. Each candidate boundary must prove:

- typed input and output without CLI text parsing;
- explicit workspace ownership;
- progress and cancellation behavior;
- error fidelity;
- safe memory and resource ownership;
- no CLI subprocess invocation;
- feasibility in at least two materially different host runtimes before a
  shared ABI is declared universal.

## Verification

- API examples can run without constructing CLI arguments.
- The same fixture produces equivalent domain results through core and CLI
  adapters.
- Terminal rendering and process exit remain outside the core.
- Workspace mutation cannot occur through a read-only entry point.
- Invalid callbacks, cancellation, and host-language exceptions fail safely.
- The design identifies every public compatibility surface and its owner.

## Exit Criterion

An approved API specification, prototype evidence, crate boundary, compatibility
model, and binding strategy exist. Every language phase can proceed without
inventing its own validation semantics or treating the CLI as its API.
