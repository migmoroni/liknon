# Phase B02.0: Define The Resource And Hub Contract

[Back to the Resource Hub and Manager plan](README.md)

## Purpose

Approve the public resource identity, metadata, compatibility, trust, workspace,
and command contracts before implementing network or mutation behavior.

## Resource Identity

Define one stable resource specification that distinguishes namespace, name,
kind, semantic resource version, and an optional exact version or accepted
constraint.

The identity is independent from a URL, local path, registry implementation,
CLI version, or display title. Case, normalization, reserved namespaces, and
collision handling are explicit and tested.

## Resource Manifest

The versioned resource manifest defines:

- identity, title, summary, authorship, license, and source;
- trust evidence and publication status;
- minimum compatible resource-schema, Manager, Tool, and Validation CLI
  capabilities when applicable;
- required and optional resource dependencies;
- declared capabilities and executable-content indicators;
- every payload path, media type, length, and SHA-256 digest;
- collection membership without copied payloads.

Unknown required fields, unsupported schema versions, cycles, conflicting
constraints, unsafe paths, and undeclared payloads fail closed.

## Consumer State

Define versioned schemas for:

- `.liknon/resources/manifest.json`, expressing requested resources and
  accepted constraints;
- `.liknon/resources/lock.json`, expressing one exact complete resolution;
- installed payload layout and integrity state;
- explicit materialization records when files are projected elsewhere.

Manifest ordering and formatting are human-maintainable. Lock serialization is
deterministic. Neither file contains credentials.

## Hub Contract

Approve:

- catalog index and query response schemas;
- immutable resource-version metadata and payload locations;
- signed catalog and release metadata;
- namespace ownership and transfer rules;
- trust-class assignment and revocation;
- publication, deprecation, yanking, and security-advisory behavior;
- pagination, caching, availability, and explicit offline failure semantics.

A yanked or revoked resource remains identifiable in existing locks but is not
silently substituted. Security state is visible during search, update, and
verify.

## Command Contract

Specify the complete grammar, output schemas, exit statuses, confirmation
points, and non-interactive behavior for `init`, `search`, `info`,
`install`, `update`, `remove`, `list`, `verify`, and
`materialize`.

Every mutating command supports a complete dry-run or plan result before commit.
Human output may be themed; machine output contains no ANSI and exposes exact
typed outcomes.

## Prototype Matrix

Before approval, prototype:

- one knowledge item;
- one multi-file skill;
- one collection;
- one transitive dependency graph;
- one incompatible resource;
- one yanked and one revoked version;
- one local source;
- one namespace collision;
- one malicious path and symlink payload;
- one interrupted transaction and recovery.

## Exit Criterion

Resource identity, schemas, trust, Hub behavior, command grammar, and workspace
state are precise enough that Hub, Manager, and Tool can implement the same
contract without inventing local interpretations.
