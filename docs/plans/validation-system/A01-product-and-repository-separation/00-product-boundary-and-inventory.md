# Phase A01.0: Approve Product Boundaries And Inventory

[Back to the product and repository separation plan](README.md)

## Purpose

Classify every current source, schema, document, test, fixture, tool, resource,
and asset before moving files. The result is a complete replacement ledger with
one owner and one intended distribution surface per item.

## Required Decisions

- Approve dependency direction among `liknon-core`, `liknon-workspace`,
  `liknon-cli`, `liknon-manager`, and `liknon-tool`.
- Record `liknon-core` and `liknon-workspace` as publishable Cargo support
  dependencies without representing them as the future B04 public API.
- Classify each schema as a validation contract, workspace/resource contract,
  application contract, or repository-only contract.
- Classify every knowledge, skill, workflow, agent profile, prompt, guide,
  template, and collection as one authored resource.
- Identify which files are compiled application assets, published Hub
  resources, public documentation, or maintainer-only material.
- Assign every current `.validation/` path to its canonical destination below
  `.liknon/validation/` or `.liknon/ai/`.
- Identify every embedded-corpus input and every resource copy that must be
  replaced by Hub publication and workspace installation.
- Separate structured report construction from human rendering and live
  progress.
- Record every path whose command, installation path, package ownership,
  release identity, or test expectation changes.

## Replacement Ledger

For every current path, record:

- current responsibility;
- target owner and target path;
- runtime, resource, and publication consumers;
- contract and tests that protect it;
- whether it moves, splits, or is removed;
- whether moving it changes public behavior or repository structure only.

No path may have multiple editable owners. An authored file may enter an
immutable resource artifact only through deterministic assembly.

The ledger explicitly covers removal of:

- the `liknon-cli` command alias;
- `liknon-knowledge`, `liknon-k`, and any `liknon knowledge` route;
- embedded complete-corpus behavior;
- ecosystem-package resource distribution;
- the standalone `.validation/` root.

## Verification

- Every file included by the current root package appears in the ledger.
- Every source module and public re-export has one target or is removed.
- Every `include_bytes!`, `include_str!`, `OUT_DIR`, and build-script
  input has an approved owner.
- Every current test has a destination aligned with the behavior it verifies.
- Every command and path replacement is represented in help, docs, fixtures,
  and distribution plans.
- No proposed target directory exists solely as a placeholder.

## Exit Criterion

The reviewed ledger covers the repository and leaves no unresolved ownership,
dependency direction, command identity, workspace path, resource source, or
distribution question needed by A01.1 and A01.2.
