# Phase A01.2: Define Packs And Separate Authored Content

[Back to the product and repository separation plan](README.md)

## Purpose

Replace CLI-embedded knowledge and skills with a generic, versioned pack
contract and move authored content into shared and ecosystem-specific packs.

## Generic Pack Contract

`crates/liknon-pack` defines a canonical manifest and validates at least:

- pack schema version, stable pack ID, pack version, and producer identity;
- compatible Liknon pack-contract range;
- optional ecosystem identity and declared pack dependencies;
- stable resource ID, kind, relative path, media type, byte length, and SHA-256;
- normalized paths contained by the real pack root;
- unique identities and deterministic catalog order;
- compatibility and conflict behavior when packs compose.

The serialized pack form may be a validated real directory or a deterministic
sealed archive. Both forms represent the same manifest and resource bytes; an
archive is never executable and is materialized only by an explicit caller
operation.

The contract rejects absolute paths, traversal, symlinks, undeclared files in a
sealed payload, digest mismatch, duplicate conflicting IDs, unsupported schema
versions, and dependency cycles. Parsing and validation require no network and
execute no content.

## Authored Pack Sources

Create only packs backed by current content:

- `packs/shared` for validation foundations, cross-stack concerns, common
  standards, shared AI-Tool and AI-Engineering operations, and human guidance;
- `packs/rust` for Rust language, Cargo, rustfmt, Clippy, and Rust-specific
  recipes;
- `packs/javascript-typescript` for JavaScript, TypeScript, Node.js, pnpm, test
  tools, and stack-specific recipes.

Classify Git guidance as shared unless a document is genuinely stack-specific.
Split mixed documents rather than assigning an entire mixed tree to one pack.
Catalog and source metadata are generated or validated per pack from one
authored source.

## Composition Contract

- A shared resource included by multiple ecosystem distributions retains the
  same pack ID, version, resource IDs, and digest.
- Loading the same pack identity and digest more than once is idempotent.
- Loading one identity with different versions or bytes is an explicit
  conflict unless the approved selection contract chooses exactly one before
  use.
- Cross-pack references name pack and resource IDs; they do not depend on
  repository-relative paths.
- Packs cannot shadow core schemas, CLI commands, or another pack's resource
  identity.

## Verification

- Every current knowledge and skill file has exactly one authored pack owner or
  a recorded repository-only owner.
- Pack validation runs without compiling or executing the CLI.
- Tampered bytes, unsafe paths, symlinks, missing resources, duplicate IDs,
  incompatible versions, and cycles fail deterministically.
- Shared plus Rust and shared plus JavaScript/TypeScript each produce valid
  catalogs.
- A combined Rust and JavaScript/TypeScript fixture deduplicates the shared pack
  and retains both ecosystem catalogs.
- Building the CLI no longer reads `docs/validation/knowledge` or
  `skills/liknon` through a build script.

## Exit Criterion

Generic pack infrastructure owns no ecosystem opinion, all current authored
content has one pack or repository owner, and the CLI can be built without any
knowledge or skill payload.
