# Phase B02.0: Define Package And Activation Contracts

[Back to the ecosystem packs plan](README.md)

## Purpose

Approve how ecosystem packages expose pack resources and how callers activate
them without coupling the CLI to Cargo, npm, or a specific filesystem layout.

## Required Decisions

- Define the minimal resource-access API for each initial ecosystem: stable
  root when the package manager guarantees one, otherwise exact sealed bytes
  and explicit caller-directed materialization.
- Define one CLI input contract for one-shot and persisted activation from a
  validated pack root or sealed pack artifact.
- Decide the consumer-owned activation record and whether it belongs inside
  `.validation/`.
- Define package, pack, schema, and CLI compatibility independently.
- Define deterministic ordering, dependency, duplicate, and conflict behavior.
- Define how activation evidence records exact package and pack identities.
- Define the boundary between read-only catalog access and explicit resource
  materialization.

## Constraints

- The CLI receives resolved paths; it does not invoke a package manager to find
  them.
- Locator APIs perform no download, activation, or workspace mutation.
- Activation accepts only validated real directories or deterministic sealed
  artifacts carrying their declared manifests.
- Persisted paths cannot silently depend on one developer's global package
  cache when another reproducible project-relative representation is required.
- No post-install script edits `.validation/`, agent configuration, shell
  profiles, or global state.
- Package and pack compatibility failures occur before any destination write.
- A Cargo dependency is not assumed to have a durable runtime path merely
  because its source exists in a build cache.

## Prototype Matrix

Prototype at least:

- a Rust consumer obtaining exact Rust package resources without relying on a
  durable Cargo registry-cache path;
- a Node consumer obtaining the JavaScript/TypeScript package's pack roots;
- one-shot CLI use of each root;
- explicit persisted activation in a temporary workspace;
- a polyglot activation with duplicate shared content;
- conflicting shared versions, tampered bytes, missing packages, moved global
  caches, and unsupported pack-schema versions.

## Exit Criterion

The package locator and CLI activation contracts are explicit, reproducible,
package-manager-neutral at the CLI boundary, and proven before a public package
identity is created.
