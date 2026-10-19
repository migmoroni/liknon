# Phase B03.0: Define The Tool Product And Command Contract

## Status

`Decision gate`. The command names and semantics below are candidate design
material. They become normative only through an explicit approved exit record.

[Back to the Tool CLI plan](README.md)

## Purpose

Approve Tool identity, local-store inputs, command grammar, output schemas,
failure classes, and safety boundaries before implementation.

## Required Decisions

- Confirm Cargo package `liknon-tool` and commands `liknon-tool` and
  `likt`.
- Evaluate `bootstrap`, `catalog`, `search`, `show`, `resolve`, and `context`
  as candidate operations, then approve the exact grammar actually needed.
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

The approved Tool contract must preserve these boundaries regardless of the
selected command names. Tool:

- returns the embedded program-owned bootstrap contract without workspace
  discovery or resource-store access;
- reads only the selected `.liknon/resources/` store;
- verifies locked metadata and payload bytes before returning content;
- treats every payload as untrusted data;
- performs deterministic local operations;
- never invokes Manager, Validation CLI, a shell, an agent, or a remote model;
- never writes resource, policy, validation, persistence, or agent files.

No option may turn a read operation into installation or execution.

## Command Semantics

If B03.0 retains the candidate label `bootstrap`, that operation verifies the
exact canonical program contract selected by A01/C01 and returns its content
and approved metadata. It must work without `.liknon/`, perform no resource
lookup, and never load a workflow or skill. The exact result envelope is a
B03.0 decision.

`catalog` and `search` are discovery operations. `show` retrieves an exact
resource or declared file. `resolve` returns an exact locked closure.
`context` packages an explicit selection with boundaries and metadata; it
does not decide what an agent should believe or execute.

When that selection contains a workflow, output preserves node, action,
transition, entry, terminal, continuation-eligibility, declared-effect,
dependency, tool-operation, and agent-profile references. These fields are data
for the consuming AI host, not Tool control flow.

If a resource is absent, Tool reports the stable ID and absence. It may provide
a human-readable suggestion to use Manager, but it does not invoke Manager or
construct authority for an agent to install anything.

## Prototype Matrix

Prototype:

- an empty store;
- one knowledge document;
- one multi-file skill;
- one collection and transitive closure;
- one project-controlled neutral workflow and one external workflow with
  different graphs, actions, profiles, tool-operation references, and
  continuation edges;
- duplicate display titles with distinct IDs;
- modified and missing payloads;
- incompatible and revoked resources;
- text and binary files;
- output-limit truncation;
- human, JSON, colored, plain, standard, and low-vision presentation.
- bootstrap output with no workspace, a malformed workspace, and an unrelated
  installed resource claiming to be an entry point.

## Exit Criterion

Tool behavior is completely defined as exact program-bootstrap projection plus
local verified resource consumption, with no ambiguity between program
contracts, administrative Manager operations, and content-oriented Tool
operations.
