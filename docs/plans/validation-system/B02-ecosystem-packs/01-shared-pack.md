# Phase B02.1: Establish The Shared Pack

[Back to the ecosystem packs plan](README.md)

## Purpose

Produce one canonical shared baseline that can be assembled unchanged into
multiple ecosystem packages and deduplicated when those packages meet.

## Content Boundary

Include only guidance and operations that remain valid independent of a
programming language or package manager, such as:

- evidence quality, validation strategy, test types, and failure-path design;
- cross-stack concerns, standards, source policy, and repository integrity;
- common AI-Tool and AI-Engineering operation contracts;
- human flow-selection and policy-ownership guidance;
- shared catalog, source register, and routing metadata.

Do not place a document in the shared pack merely because two current stacks
use it. A language, runtime, framework, or tool-specific document belongs to an
ecosystem pack.

## Assembly Contract

- Build output is deterministic from tracked shared sources.
- The manifest declares every byte and stable resource ID.
- The same source commit produces byte-identical pack output for every package
  assembler.
- Packages include the built pack without rewriting paths, metadata, or text.
- Editorial drafts follow their declared readiness semantics without blocking
  structurally valid in-progress content.

## Verification

- Standalone manifest, hash, path, catalog, link, and source checks pass.
- Shared content contains no Rust-only or JavaScript/TypeScript-only routing.
- Two independent assembly paths produce the same digest.
- The pack can be inspected without the CLI and queried through the CLI when
  explicitly supplied.

## Exit Criterion

One sealed shared pack is independently valid, stack-neutral, deterministic,
and ready to be included unchanged by each initial ecosystem package.
