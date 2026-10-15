# Phase B03.7: Prove Polyglot Resource Conformance

[Back to the ecosystem AI packages plan](README.md)

## Purpose

Prove that independently installed ecosystem packages provide coherent AI
resources in one polyglot workspace without duplicate authored sources,
instruction conflicts, or hidden CLI coupling.

## Representative Workspace

Use a workspace containing every ecosystem selected for the initial release,
with native dependency locks. Verify:

- byte-identical shared resource manifests;
- unique ecosystem resource IDs;
- deterministic inventory and routing;
- validation operations invoke `liknon` or `liknon-cli`;
- knowledge operations invoke `liknon-knowledge` or `liknon-k`;
- selected resources materialize to one coherent workspace layout;
- identical resources are idempotent;
- conflicting bytes fail before mutation;
- package order does not change the result.

## Failure Matrix

Exercise incompatible resource schemas, different bytes for one shared
identity, unsafe paths, symlinks, partial writes, missing CLIs, incompatible CLI
capabilities, interrupted materialization, and removal of one package.

## Exit Criterion

All selected packages coexist in one workspace, agree on shared resources,
retain ecosystem-specific scope, and fail closed without relying on hidden
activation state or executables.
