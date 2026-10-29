# Phase B03.3: Build Source And Native Tool Distributions

## Status

`Provisional`. Blocked by B03.2 and the credential-free shared distribution
foundation implemented and proven through B01.1-B01.4. It does not wait for a
published Validation release.

Any operation labels used by distribution fixtures remain placeholders for the
exact grammar selected by B03.0.

[Back to the Tool CLI plan](README.md)

## Purpose

Produce independently installable Tool candidates without embedding the
resource corpus or creating a runtime dependency on another Liknon product. The
single program-owned AI bootstrap contract is an explicit Tool input, not part
of the resource corpus.

All candidate mechanics conform to the
[shared CLI distribution foundation](../cli-distribution-foundation.md).

## Tool Product Profile

The reviewed descriptor supplies at least:

- product ID `tool` and owner B03;
- Cargo package and path `liknon-tool` / `apps/liknon-tool`;
- product orchestrator `.github/workflows/release-tool.yml`;
- canonical payload `liknon-tool` and installed alias `likt`;
- signed-tag prefix `liknon-tool-v`;
- enforceable Tool-only Cloudflare storage boundary, immutable release prefix,
  and stable pointer;
- every adopted target row and runtime baseline;
- Tool-specific offline, bootstrap, catalog, retrieval, integrity, help, and
  version smoke tests;
- Tool-only protected environments and accountable recovery owner.

## Version And Provenance

The exact `liknon-tool` package at `apps/liknon-tool` is Tool's sole version
authority. Structured Cargo metadata selects the package and version. Candidate
creation fails unless:

- package name, path, and binary targets match the reviewed Tool descriptor;
- the version has canonical `MAJOR.MINOR.PATCH` form;
- the selected commit is exactly identified by the annotated, signed
  `liknon-tool-v<VERSION>` tag;
- the committed lockfile and inspected source-package closure are used by every
  source and native channel; and
- that release identity does not name different bytes in crates.io, GitHub, or
  Tool's immutable Cloudflare namespace.

Tool carries no handwritten release version and inherits no version, tag, or
commit from Manager, Validation, resources, or a complete suite. GitHub source,
crates.io source, and native candidates all refer to the same Tool release
identity `(tool, liknon-tool, version, tag, commit)`.

The shared internal manifest records the inspected source-package digest and
supporting-package closure plus every final native target, command, size,
SHA-256, toolchain, and platform-authenticity result. Direct assembly derives
Tool's public manifest only from those retained records. Tool's stable pointer
advances only after exact public readback, and recovery cannot mutate another
product's provenance or pointer.

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

That closure includes the exact accepted `liknon-cli-ui` version established by
A02. Package inspection proves Tool consumes the shared implementation rather
than embedding a private palette, presentation profile, generic help renderer,
or terminal-color policy. The supporting crate is a source dependency, not a
second executable or runtime file in Tool's native archive.

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

Build one final payload per approved target through the reusable
credential-free build lane. Native archives, provenance, signing, installers,
aliases, rollback, and public readback follow the shared contract with
Tool-specific names, versions, tags, paths, tests, and credentials.

Each target build records the exact product, package, version, tag, target,
toolchain, commit, size, final SHA-256, and platform-authenticity evidence. GNU
and MUSL remain distinct for Linux x64 and arm64. Windows and macOS use the
common declared trust modes and native verification. Downstream jobs copy exact
retained bytes and never rebuild or mutate Tool.

The website serves the same immutable download objects and may present Tool as
an individual selection or part of a convenience bundle.

## Compatibility

Tool compatibility is independent from Manager, Validation CLI, and resource
versions. Tool declares the resource schema versions and capabilities it
supports. Compatibility is tested and reported rather than inferred from equal
version numbers.

Tool also declares the exact AI bootstrap-contract version and digest it embeds.
Changing that contract requires a compatible Tool release; changing a Hub
resource does not.

## Verification

- GitHub and crates.io source candidates install in isolated environments.
- Both command names report the same version and behavior.
- Both command names satisfy the A02 palette, presentation, help, `NO_COLOR`,
  and ANSI-free machine-output conformance matrix.
- GitHub source, crates.io source, and every native candidate agree on Tool's
  package-derived version, signed tag, full commit, and source closure.
- Every native target runs catalog and retrieval fixtures offline.
- Every source and native target returns the same bootstrap contract bytes and
  digest without a workspace.
- Artifact inventories contain only declared Tool runtime files.
- A resource update changes no Tool candidate.
- Installing Tool mutates no consumer workspace.
- Shared-foundation conformance rejects cross-product candidate, namespace,
  target, alias, stable-pointer, credential, version, tag, commit, and
  provenance substitution.

## Exit Criterion

Source and native channels contain one focused, offline Tool application with
two equivalent commands and no bundled resource payload. Common release
mechanisms remain product-neutral and all Tool publication authority remains
B03-specific.
