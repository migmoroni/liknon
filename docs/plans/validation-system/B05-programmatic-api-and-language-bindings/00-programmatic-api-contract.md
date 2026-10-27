# Phase B05.0: Define The Programmatic API Contract

## Status

`Future`. This becomes the decision gate only after a demonstrated in-process
consumer and a stable A01 boundary.

[Back to the programmatic API and bindings plan](README.md)

## Purpose

Determine which Liknon behavior is reusable in process and define a stable core
contract before exposing any language package or ABI.

Discovery only. Do not implement bindings or configure registry publication
until this phase has an approved exit record.

## Required Design Work

### Use Cases

- Identify concrete host-application workflows for configuration, inspection,
  planning, execution, reporting, core schemas, explicit initialization
  resource provisioning, and workspace-tool integration.
- Require at least one representative consumer for every capability selected
  for a public language API.
- Separate reusable use cases from terminal presentation and shell-oriented
  ergonomics.
- Define which operations are read-only, which may execute external programs,
  and which may mutate a workspace.
- If Git-hook integration enters scope, adopt the shared
  [Git-hook integration contract](git-hook-integration.md) and identify one
  representative consumer and provider.

### API Semantics

- Require an explicit workspace root and explicit inputs.
- Define sync versus async entry points.
- Define cancellation, timeout, progress-event, and output-capture behavior.
- Define stable result and diagnostic types.
- Define deterministic serialization where callers need persisted evidence.
- Separate inspection, deterministic change planning, diff presentation,
  explicit application, and verification for configuration integrations.
- If installed-resource inspection enters scope, expose workflow graphs,
  actions, profiles, tool descriptors, artifact declarations, and
  policy-compatibility facts as generic typed data without interpreting or
  activating them.
- Define thread-safety, reentrancy, callback, and resource-lifetime rules.

### Failure Boundaries

- Establish a typed error taxonomy independent from CLI exit codes.
- Ensure Rust panics and foreign-language exceptions cannot cross an FFI
  boundary unsafely.
- Define behavior for invalid paths, malformed configuration, unavailable tools,
  interrupted processes, partial workspace mutation, and callback failure.
- For configuration integrations, define stale-plan detection, ambiguous
  ownership, concurrent edits, atomic-write failure, and rollback evidence.

### Packaging And Compatibility

- Decide crate and module boundaries.
- Decide whether bindings share one C-compatible ABI, use ecosystem-specific
  native extension systems, or combine approaches.
- Version the Rust API, native ABI, serialized contracts, and language wrappers
  explicitly.
- Decide how core, workspace contract, API, ABI, and wrapper versions relate
  while remaining explicit compatibility surfaces.
- Keep Hub access and resource mutation outside the binding boundary. A
  read-only installed-resource API may be approved separately when a real
  consumer requires it.
- Keep CLI installation outside the binding boundary. A selected integration
  capability may return installation guidance, explicitly probe compatibility,
  and write reviewed provider configuration that references a public CLI route.
- Define supported targets and runtime baselines from measured package needs,
  not by copying B01's executable matrix automatically.

## Prototype Requirements

Prototype every capability selected for the first package. If in-process
validation enters scope, prototype at least one representative read-only
operation and one operation that executes a validation plan. If Git-hook
integration enters scope, prototype plan, no-op reapply, explicit apply, and
removal against an existing provider configuration. Each candidate boundary
must prove:

- typed input and output without CLI text parsing;
- explicit workspace ownership;
- progress and cancellation behavior;
- error fidelity;
- safe memory and resource ownership;
- no CLI subprocess transport; an explicit compatibility probe is permitted
  only for an approved integration operation;
- no embedded authored-resource payload or implicit Hub access;
- no official resource ID, process number, graph shape, or profile identity in
  API control flow;
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
- Every proposed package has a named consumer or remains unimplemented.
- Any selected resource-inspection API passes an external workflow fixture with
  non-numeric nodes and independently versioned profiles.
- Any selected Git-hook API passes the shared preservation, idempotency,
  stale-plan, absent-CLI, and direct-native-command fixtures.

## Exit Criterion

An approved API specification, prototype evidence, crate boundary, compatibility
model, and binding strategy exist. Every language phase can proceed without
inventing its own validation semantics or treating the CLI as its API.
