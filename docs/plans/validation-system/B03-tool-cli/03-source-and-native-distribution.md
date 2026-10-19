# Phase B03.3: Build Source And Native Tool Distributions

## Status

`Provisional`. Blocked by B03.2.

Any operation labels used by distribution fixtures remain placeholders for the
exact grammar selected by B03.0.

[Back to the Tool CLI plan](README.md)

## Purpose

Produce independently installable Tool candidates without embedding the
resource corpus or creating a runtime dependency on another Liknon product. The
single program-owned AI bootstrap contract is an explicit Tool input, not part
of the resource corpus.

## GitHub Source Candidate

An exact signed Tool tag identifies one repository commit. Installation targets
`apps/liknon-tool` with the committed lockfile and supported Rust toolchain.
The default branch is not a reproducible release identity.

## crates.io Candidate

Package `liknon-tool` builds from its accepted source contents without
unpublished path dependencies or repository-only files. It installs
`liknon-tool` and `likt` from one implementation.

Package and inspect its complete supporting-crate closure. Publish and read
back an exact supporting version before the application when it is not already
available byte-for-byte in the registry.

Package inspection proves absence of:

- authored knowledge, skills, workflows, prompts, profiles, tool descriptors, guides, templates,
  and collections;
- validation execution logic;
- Hub clients and network dependencies;
- Manager mutation logic;
- plans, evaluations, and maintainer fixtures.

The inspected source-package closure includes the exact canonical C01 contract
through the supporting package chosen by A01. Tool consumes that contract
through one owner and does not duplicate it in its own inventory. No package
in the closure contains another authored AI resource payload.

## Native Candidates

Build one final payload per approved target. Native archives, provenance,
signing, installers, aliases, rollback, and public-readback follow B01's
platform security model with Tool-specific names, versions, tags, paths, and
credentials.

The website serves the same immutable download objects and may present Tool as
an individual selection or part of a convenience bundle.

## Version And Compatibility

Tool version is independent from Manager, Validation CLI, and resource versions.
Tool declares the resource schema versions and capabilities it supports.
Compatibility is tested and reported rather than inferred from equal version
numbers.

Tool also declares the exact AI bootstrap-contract version and digest it embeds.
Changing that contract requires a compatible Tool release; changing a Hub
resource does not.

## Verification

- GitHub and crates.io source candidates install in isolated environments.
- Both command names report the same version and behavior.
- Every native target runs catalog and retrieval fixtures offline.
- Every source and native target returns the same bootstrap contract bytes and
  digest without a workspace.
- Artifact inventories contain only declared Tool runtime files.
- A resource update changes no Tool candidate.
- Installing Tool mutates no consumer workspace.

## Exit Criterion

Source and native channels contain one focused, offline Tool application with
two equivalent commands and no bundled resource payload.
