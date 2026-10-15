# Plan B02: Liknon Ecosystem Packs

## Status And Relationship To Other Tracks

This plan distributes the authored guidance and agent resources separated by
[A01](../A01-product-and-repository-separation/README.md). It begins only after
A01 proves the generic pack contract and the lean CLI boundary.

[B01](../B01-cli-distribution/README.md) distributes the `liknon` executable.
B02 distributes data and small ecosystem-native integration surfaces. B02
packages do not install a global command, contain a copy of the CLI, or expose
the validation engine as an FFI binding. True in-process APIs belong to
[B03](../B03-programmatic-api-and-language-bindings/README.md).

## Objective

Let each stack in a workspace install its own versioned Liknon package and
contribute relevant knowledge, skills, recipes, templates, and routing metadata
to the same CLI. A Rust and JavaScript/TypeScript workspace can install both
packages; the CLI then composes their packs with one shared baseline.

The initial scope is deliberately limited to content that exists and is used
today:

- one shared pack;
- one Rust ecosystem pack and package;
- one JavaScript/TypeScript ecosystem pack and package;
- one polyglot composition contract.

Ruby, Python, .NET, and other ecosystems follow the same model only after they
have substantive authored content and representative fixtures. B02 creates no
empty registry placeholder.

## Product Boundary

An ecosystem package may provide:

- the exact shared pack version selected for that package;
- one or more ecosystem-specific packs;
- a minimal ecosystem-native API that returns package version, manifests, and
  either immutable pack roots or exact sealed pack bytes with explicit
  caller-directed materialization;
- documentation showing how a human or agent explicitly activates those roots
  through the Liknon CLI;
- package-manager metadata needed to install and locate its own immutable
  resources.

It does not provide:

- the `liknon` executable or any global command;
- a launcher that downloads or invokes a hidden CLI;
- configuration planning or execution logic;
- terminal help, color, presentation, or human report rendering;
- a native validation library or language binding;
- automatic workspace mutation, post-install activation, or implicit network
  access by the CLI.

The resource-locator API is useful on its own and is not represented as the
future Liknon validation API. B03 may extend an ecosystem package only after it
resolves naming, versioning, and compatibility with this content contract.

## Composition Model

```text
ecosystem package manager
          |
          v
installs immutable package resources
          |
          v
human or agent obtains a declared pack root or sealed artifact
          |
          v
explicit Liknon pack activation or one-shot path
          |
          v
liknon-pack validates manifests, hashes, and compatibility
          |
          v
Liknon CLI composes catalogs and materializes only requested resources
```

Package installation and pack activation are separate operations. The package
manager owns dependency resolution and package locking. Liknon owns manifest
validation, active-pack composition, resource selection, and explicit safe
materialization. B02 does not assume that every package manager preserves a
stable runtime filesystem path to package resources.

## Shared Baseline

Shared content is authored once under `packs/shared`. Each ecosystem package
may carry the exact built shared pack so that it is independently useful. In a
polyglot workspace:

- equal shared pack ID, version, and digest are deduplicated;
- a shared pack identity with different bytes is rejected;
- incompatible shared versions require explicit resolution before resources
  are read or written;
- no package edits or forks shared resources during assembly.

This permits independent registry installation without creating multiple
sources of truth.

## Version And Compatibility Model

Keep these identities explicit:

- package version, governed by the ecosystem registry;
- pack version, governed by content changes;
- pack schema version, governed by `liknon-pack`;
- minimum and maximum supported pack-contract versions;
- optional minimum CLI capability required for materialization or retrieval.

Package and CLI versions do not need to advance together. Compatibility is
declared in manifests and verified structurally, never inferred because version
numbers happen to match.

## Security And Mutation Rules

1. Package installation runs no workspace-mutating post-install hook.
2. Packages contain no publication credential, executable download script, or
   remote mutable content reference.
3. Pack payloads are fully declared and hashed.
4. The CLI never recursively discovers package-manager directories.
5. The caller supplies a pack root through the approved explicit integration.
6. Materialization preflights every destination and is transactional.
7. Skills and Markdown are data until explicitly selected; a pack cannot cause
   code execution merely by being present.
8. Package-manager lockfiles and Liknon manifests together identify the exact
   bytes used for a validation-engineering decision.

## Proposed Repository Shape

```text
packs/
├── shared/
├── rust/
└── javascript-typescript/

packages/
├── rust/                            # Created when B02.2 is implemented
└── node/                            # Created when B02.3 is implemented
```

The package directories contain only ecosystem adapters and assembly metadata.
They do not become second authored copies of `packs/`.

## Implementation Order

| Phase | Document | Depends on | Primary result |
| ---: | --- | --- | --- |
| B02.0 | [Package And Activation Contract](00-package-and-activation-contract.md) | A01.4 | Approved package, locator, activation, version, and mutation contracts |
| B02.1 | [Shared Pack](01-shared-pack.md) | B02.0 | One independently validated shared baseline |
| B02.2 | [Rust Ecosystem Package](02-rust-ecosystem-package.md) | B02.1 | Installable Rust package exposing exact shared and Rust pack payloads |
| B02.3 | [JavaScript And TypeScript Ecosystem Package](03-javascript-typescript-ecosystem-package.md) | B02.1 | Installable package exposing shared and JavaScript/TypeScript pack roots |
| B02.4 | [Polyglot Composition](04-polyglot-composition.md) | B02.2 and B02.3 | Deterministic multi-package activation and conflict handling |
| B02.5 | [Publication And Maintenance](05-publication-and-maintenance.md) | B02.4 | Independently releasable, publicly verified ecosystem packages |

## Global Acceptance Criteria

- [ ] The CLI binary remains byte-independent from authored pack content.
- [ ] Installing a package adds no global `liknon` command and mutates no
      consumer workspace.
- [ ] Each package exposes immutable pack roots or exact sealed pack artifacts
      without environment-wide searches or terminal-text parsing.
- [ ] Shared content is authored once and byte-identical wherever assembled.
- [ ] Rust and JavaScript/TypeScript resources remain independently versioned
      and selectable.
- [ ] A polyglot workspace can activate both packages deterministically.
- [ ] Same-ID same-digest shared packs deduplicate; conflicting bytes fail.
- [ ] Pack catalogs and resources are usable by humans, AI-Tool, and
      AI-Engineering without embedding them in the CLI.
- [ ] Registry packages contain only declared content and minimal locator code.
- [ ] Package updates can ship without rebuilding or republishing the CLI.
- [ ] No future ecosystem is published before it has real content, fixtures,
      ownership, and a maintained consumption path.

## Resulting System

Liknon's executable remains stable and stack-agnostic while validation guidance
can evolve where the relevant ecosystem already manages dependencies. A
polyglot workspace composes several content packages around one CLI rather than
requiring one oversized binary or duplicated validation engines.
