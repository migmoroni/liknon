# Phase A01.0: Approve Product Boundaries And Inventory

[Back to the product and repository separation plan](README.md)

## Purpose

Classify every current source, schema, document, test, fixture, tool, and asset
before moving files. The result is a complete migration ledger with one owner
and one intended distribution surface per item.

## Required Decisions

- Approve the `liknon-core`, `liknon-pack`, and `liknon-cli` dependency
  direction.
- Classify each schema as a core contract, pack contract, content-authoring
  contract, or repository-only evaluation contract.
- Split current knowledge and skills into shared, Rust, and
  JavaScript/TypeScript ownership without duplicating authored files.
- Identify which initialization resources are generic CLI bootstrap resources
  and which are supplied by packs.
- Separate structured report construction from human rendering and live
  progress.
- Separate public runtime examples from maintainer fixtures and evaluations.
- Record every documentation path whose command, installation path, or content
  ownership changes.

## Migration Ledger

For every current path, record:

- current path and responsibility;
- target owner and target path;
- runtime or publication consumers;
- contract and tests that protect it;
- whether it moves, splits, or is removed;
- whether moving it changes a public behavior or only repository structure.

No path may be classified as both CLI-owned and pack-owned. Shared content can
be assembled into multiple ecosystem packages, but it has one authored owner.

## Verification

- Every file included by the current root package appears in the ledger.
- Every `src` module and public re-export has one target crate or is explicitly
  removed.
- Every `include_bytes!`, `include_str!`, `OUT_DIR`, and build-script input has
  an approved owner.
- Every current test has a destination aligned with the behavior it verifies.
- No proposed target directory exists solely as a placeholder for future work.

## Exit Criterion

The reviewed ledger covers the entire current package and leaves no unresolved
ownership, dependency direction, schema location, content source, or
distribution question needed by A01.1 and A01.2.
