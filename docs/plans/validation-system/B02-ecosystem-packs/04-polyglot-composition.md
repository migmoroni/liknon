# Phase B02.4: Prove Polyglot Pack Composition

[Back to the ecosystem packs plan](README.md)

## Purpose

Prove that independently installed ecosystem packages compose around one CLI
without duplicate guidance, ambiguous routing, or package-manager coupling.

## Representative Workspace

Create a fixture containing real Rust and JavaScript/TypeScript project
boundaries, each with its native dependency lock. Activate the pack roots from
both packages through the B02.0 contract.

Verify:

- one shared pack identity after deduplication;
- one Rust and one JavaScript/TypeScript ecosystem identity;
- deterministic merged catalog order;
- globally unique resource routing;
- correct skill and recipe selection for shared and stack-specific requests;
- no change to validation planning unless the consumer explicitly changes
  `.validation/config.json`;
- equivalent behavior regardless of caller activation order.

## Failure Matrix

Exercise incompatible shared versions, same-version different-digest tampering,
missing dependencies, unknown pack schemas, duplicate ecosystem resource IDs,
unsafe paths, moved installations, partial materialization, and removal of one
ecosystem package.

Every failure occurs before content use or destination mutation and identifies
the conflicting package and pack identities without exposing host-specific
cache paths unnecessarily.

## Exit Criterion

One polyglot workspace composes independently installed packages
deterministically, deduplicates exact shared content, and fails closed for every
identity, compatibility, integrity, or path conflict.
