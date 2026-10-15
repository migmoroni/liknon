# Phase B03.0: Define Package And Resource Contracts

[Back to the ecosystem AI packages plan](README.md)

## Purpose

Approve one cross-ecosystem resource model and idiomatic host-language APIs
before creating public package identities.

## Required Decisions

- Define the manifest schema, stable resource IDs, resource kinds, paths,
  media types, byte lengths, and hashes.
- Define shared and ecosystem resource-set identities and compatibility.
- Define the minimal API for listing metadata, reading exact bytes, checking
  compatibility, and explicitly materializing requested resources.
- Define safe destination, conflict, rollback, and idempotency behavior.
- Define package inventory and import namespace in every ecosystem.
- Reserve extension points that B04 may use without pretending future bindings
  already exist.
- Define how skills select `liknon` versus `liknon-knowledge`.
- Define package, CLI, knowledge, resource-schema, and future API versions
  independently.

## Constraints

- No package exposes a global command.
- No package invokes a CLI as an implementation detail.
- No package installation or import mutates a workspace.
- No adapter infers a workspace from a package cache.
- A materialization call requires an explicit destination and selected
  resources.
- Package resources are immutable after publication.
- Resource APIs do not become validation APIs merely because they use the
  `liknon` package name.

## Prototype Matrix

Prototype at least:

- inventory and byte retrieval in all five host environments;
- explicit materialization into temporary workspaces;
- repeated identical materialization;
- conflicting existing bytes, unsafe paths, symlinks, interruption, and
  rollback;
- one skill route invoking the validation CLI;
- one skill route retrieving knowledge through `liknon-knowledge`;
- installation and import with neither CLI present;
- future API namespace extension without breaking the resource module.

## Exit Criterion

All five ecosystems can implement one coherent, non-CLI resource contract with
safe explicit integration and a clear future binding boundary.
