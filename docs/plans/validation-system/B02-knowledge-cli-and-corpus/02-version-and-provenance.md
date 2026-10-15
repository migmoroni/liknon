# Phase B02.2: Centralize Knowledge Version And Provenance

[Back to the knowledge CLI and corpus plan](README.md)

## Purpose

Link one independent knowledge application version and source commit to every
source package, native binary, catalog, and document payload.

## Version Authority

- The `apps/liknon-knowledge` Cargo package version is the sole authored
  version source for this application and is read through structured Cargo
  metadata.
- The signed tag is `liknon-knowledge-v<VERSION>`.
- Both command aliases report that exact version.
- crates.io, Cloudflare metadata, archive names, and documentation use the same
  version.
- An accepted version is immutable.

## Provenance Records

Generate deterministic internal evidence containing:

- application version and source commit;
- Rust toolchain and locked dependency identity;
- target triple for native candidates;
- executable hashes for both installed names;
- catalog schema version and catalog digest;
- ordered document ID, path, length, and SHA-256 inventory;
- build invocation and reproducibility inputs.

The public manifest projects only the fields needed to authenticate and select
public artifacts. It contains no secret, runner path, or mutable cache path.

## Verification

- Tag, Cargo metadata, binaries, crates package, manifests, and corpus inventory
  agree.
- A source or corpus change without a version change fails.
- Reordering source files does not change deterministic catalog order.
- Rebuilding the same candidate reproduces all deterministic payloads.
- Neither alias can report a different version or corpus.

## Exit Criterion

Every release form is traceable to one versioned source and one exact corpus,
with enough retained evidence to reject rebuilds or mixed candidates.
