# Phase A01.0: Approve Product Boundaries And Inventory

[Back to the product and repository separation plan](README.md)

## Purpose

Classify every current source, schema, document, test, fixture, tool, and asset
before moving files. The result is a complete replacement ledger with one
owner and one intended distribution surface per item.

## Required Decisions

- Approve the dependency direction among `liknon-core`, `liknon-cli`, and
  `liknon-knowledge`.
- Classify each schema as a core contract, validation-CLI contract,
  knowledge-CLI contract, AI-resource contract, or repository-only contract.
- Classify every current knowledge file under the one complete corpus.
- Split skills, prompts, agent guidance, and integration resources into shared
  and Rust, JavaScript/TypeScript, Ruby, Python, or .NET ownership without
  duplicating authored sources.
- Identify which initialization resources are generic validation-CLI bootstrap
  resources and which belong to ecosystem packages.
- Separate structured report construction from human rendering and live
  progress.
- Separate public runtime examples from maintainer fixtures and evaluations.
- Record every path whose command, installation path, package ownership, or
  release identity changes.

## Replacement Ledger

For every current path, record:

- current responsibility;
- target owner and target path;
- runtime and publication consumers;
- contract and tests that protect it;
- whether it moves, splits, or is removed;
- whether moving it changes public behavior or repository structure only.

No path may be owned simultaneously by a CLI and an ecosystem package.
Resources assembled into several packages retain one authored owner.

## Verification

- Every file included by the current root package appears in the ledger.
- Every `src` module and public re-export has one target or is removed.
- Every `include_bytes!`, `include_str!`, `OUT_DIR`, and build-script input has
  an approved owner.
- Every current test has a destination aligned with the behavior it verifies.
- Every current `liknon knowledge` route and embedded-knowledge consumer is
  recorded for removal or replacement.
- No proposed target directory exists solely as a placeholder.

## Exit Criterion

The reviewed ledger covers the entire repository and leaves no unresolved
ownership, dependency direction, schema location, resource source, command
identity, or distribution question needed by A01.1 and A01.2.
