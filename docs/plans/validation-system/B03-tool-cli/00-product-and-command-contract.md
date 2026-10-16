# Phase B03.0: Define The Tool Product And Command Contract

[Back to the Tool CLI plan](README.md)

## Purpose

Approve Tool identity, local-store inputs, command grammar, output schemas,
failure classes, and safety boundaries before implementation.

## Required Decisions

- Confirm Cargo package `liknon-tool` and commands `liknon-tool` and
  `likt`.
- Define exact grammar for `catalog`, `search`, `show`, `resolve`, and
  `context`.
- Define workspace discovery and explicit-root behavior.
- Define human and versioned JSON output contracts.
- Define stable ordering, text matching, output-size, file-count, and binary
  payload behavior.
- Define compatibility, trust, revocation, and integrity presentation.
- Define behavior for absent, partial, unsupported, and manually populated
  resource stores.
- Define stdin, stdout, stderr, exit-status, color, presentation, and
  `NO_COLOR` behavior.
- Confirm Tool has no network or mutation capability.

## Hard Boundaries

Tool:

- reads only the selected `.liknon/resources/` store;
- verifies locked metadata and payload bytes before returning content;
- treats every payload as untrusted data;
- performs deterministic local operations;
- never invokes Manager, Validation CLI, a shell, an agent, or a remote model;
- never writes resource, policy, validation, persistence, or agent files.

No option may turn a read operation into installation or execution.

## Command Semantics

`catalog` and `search` are discovery operations. `show` retrieves an exact
resource or declared file. `resolve` returns an exact locked closure.
`context` packages an explicit selection with boundaries and metadata; it
does not decide what an agent should believe or execute.

If a resource is absent, Tool reports the stable ID and absence. It may provide
a human-readable suggestion to use Manager, but it does not invoke Manager or
construct authority for an agent to install anything.

## Prototype Matrix

Prototype:

- an empty store;
- one knowledge document;
- one multi-file skill;
- one collection and transitive closure;
- duplicate display titles with distinct IDs;
- modified and missing payloads;
- incompatible and revoked resources;
- text and binary files;
- output-limit truncation;
- human, JSON, colored, plain, standard, and low-vision presentation.

## Exit Criterion

Tool behavior is completely defined as local verified consumption, with no
ambiguity between administrative Manager operations and content-oriented Tool
operations.
