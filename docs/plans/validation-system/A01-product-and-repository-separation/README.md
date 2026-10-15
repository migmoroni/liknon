# Plan A01: Liknon Product And Repository Separation

## Status And Purpose

This plan is the architectural prerequisite for every publication track. It
turns the current single-package repository into a workspace with explicit
ownership boundaries for the validation engine, command-line application,
pack infrastructure, authored ecosystem content, and repository-only assets.

A01 changes repository structure and product composition without creating a
second validation implementation. Observable configuration, planning,
execution, report, process-control, and exit-status semantics remain owned by
one core.

## Product Model

Liknon has three distinct product layers:

1. **Liknon Core** owns deterministic validation behavior and structured
   contracts. It has no terminal, command-line, package-manager, or authored
   knowledge responsibility.
2. **Liknon CLI** adapts the core to arguments, streams, signals, progress,
   terminal themes, human reports, schemas, and workspace bootstrap. It is the
   only product that installs the global `liknon` executable.
3. **Liknon Packs** contain versioned shared or ecosystem-specific knowledge,
   skills, recipes, templates, and routing metadata. Packs are data products;
   they neither implement validation nor install another CLI.

Future in-process language APIs are a fourth distribution concern built from
the same core, but their public contracts belong to
[Plan B03](../B03-programmatic-api-and-language-bindings/README.md), not A01.

## Ownership Boundaries

| Concern | Canonical owner |
| --- | --- |
| Config, plan, execution, process, repository, DAG, and structured report semantics | `crates/liknon-core` |
| Generic pack manifest, path safety, hashes, compatibility, catalog composition, and explicit materialization | `crates/liknon-pack` |
| Argument parsing, help, terminal themes, progress, signals, human rendering, stream routing, and exit codes | `apps/liknon-cli` |
| Shared validation guidance and agent operations | `packs/shared` |
| Rust-specific guidance, recipes, tools, and skill extensions | `packs/rust` |
| JavaScript/TypeScript-specific guidance, recipes, tools, and skill extensions | `packs/javascript-typescript` |
| Plans, evaluations, fixtures, editorial checks, and maintainer procedures | Repository only |

The location of a file follows its owner. A resource is not copied into the CLI
merely because the CLI can display or materialize it.

## Target Repository Shape

```text
liknon/
├── Cargo.toml                       # Virtual workspace and shared metadata
├── Cargo.lock                       # Committed CLI dependency lock
├── apps/
│   └── liknon-cli/
│       ├── Cargo.toml               # Package name may differ; binary is `liknon`
│       ├── src/
│       └── assets/                  # CLI-only runtime assets
├── crates/
│   ├── liknon-core/
│   └── liknon-pack/
├── packs/
│   ├── shared/
│   ├── rust/
│   └── javascript-typescript/
├── distribution/
│   └── cli/                         # Authored B01 distribution resources
├── docs/
├── evals/
├── fixtures/
└── tools/
```

`packages/` is added by B02 only for ecosystem packages that have approved
content and a tested consumption model. Empty language placeholders are not
created. A future FFI or binding crate is added only by B03 after its API
contract is approved.

## Architectural Invariants

1. Validation behavior has one implementation in `liknon-core`.
2. `liknon-core` does not depend on Clap, terminal styling, progress rendering,
   global process exit, current-directory discovery, or authored pack content.
3. `liknon-pack` is format- and filesystem-oriented. It contains no Rust,
   JavaScript, TypeScript, Ruby, Python, or .NET validation opinions.
4. The CLI depends inward on core and pack infrastructure; neither depends on
   the CLI.
5. The CLI binary embeds no validation knowledge or operational skill bundle.
6. `liknon knowledge` and any future skill/resource operation read only packs
   explicitly active for the selected workspace or explicitly supplied by the
   caller.
7. The CLI performs no implicit package-manager discovery, network download,
   dependency installation, or post-install workspace mutation.
8. Shared authored content has one source under `packs/shared`. Ecosystem
   packages may include the same built shared pack, but do not fork its source.
9. Multiple packs compose by stable identity. Equal identities and digests are
   idempotent; conflicting versions or bytes fail before use or mutation.
10. Workspace materialization is explicit, preflighted, path-safe, and
    transactional. Package installation alone never writes `.validation/` or
    agent directories.
11. Root workspace metadata is the sole authored version source for packages
    that intentionally share the CLI release train.
12. Repository-only plans, fixtures, evaluations, and maintainer tools do not
    enter CLI artifacts or ecosystem packages unless a package contract names
    them explicitly.

## Migration Boundary

The repository has not published a compatibility contract for its present
layout. A01 therefore updates every current consumer to the new paths and
removes the replaced root package, embedded-knowledge build pipeline, and
obsolete package include rules. It does not retain aliases, fallback lookup,
duplicate sources, or a second installation path.

The CLI may preserve a command name such as `knowledge`, but its source changes
from compiled-in documents to active packs. Missing packs produce an explicit
empty-state or selection diagnostic; they never trigger an embedded fallback.

## Implementation Order

| Phase | Document | Depends on | Primary result |
| ---: | --- | --- | --- |
| A01.0 | [Product Boundary And Inventory](00-product-boundary-and-inventory.md) | Current repository | Approved ownership map and migration ledger |
| A01.1 | [Cargo Workspace And Core Extraction](01-cargo-workspace-and-core-extraction.md) | A01.0 | Virtual workspace, terminal-free core, and CLI adapter package |
| A01.2 | [Pack Contract And Content Separation](02-pack-contract-and-content-separation.md) | A01.1 | Generic pack crate plus shared, Rust, and JavaScript/TypeScript authored packs |
| A01.3 | [CLI Composition And Workspace Resources](03-cli-composition-and-workspace-resources.md) | A01.2 | Lean CLI consuming explicit packs and provisioning only requested resources |
| A01.4 | [Conformance And Documentation](04-conformance-and-documentation.md) | A01.3 | Proven boundaries, equivalent validation behavior, and current documentation |

Do not begin B01 or B02 implementation before A01.4. B03 may perform design
work against the approved boundaries but may not publish an API before A01.4.

## Global Acceptance Criteria

- [ ] The root manifest is a virtual workspace with one committed lockfile.
- [ ] `cargo install --path apps/liknon-cli --locked` installs the only global
      `liknon` executable.
- [ ] Core tests construct no CLI arguments and render no terminal output.
- [ ] CLI tests exercise argument, stream, presentation, signal, and exit-code
      adaptation independently from domain tests.
- [ ] No compiled CLI asset contains authored knowledge or skill documents.
- [ ] Pack validation rejects unsafe paths, undeclared bytes, digest mismatch,
      incompatible contracts, duplicate conflicting IDs, and symlinks.
- [ ] Shared, Rust, and JavaScript/TypeScript content has one clear authored
      owner and can be validated without compiling the CLI.
- [ ] One polyglot fixture composes shared, Rust, and JavaScript/TypeScript packs
      without duplicated catalog entries or ambiguous resources.
- [ ] CLI operation without packs remains deterministic and explains the
      missing capability without downloading or discovering dependencies.
- [ ] Existing config and report fixtures produce equivalent structured
      outcomes through the extracted core and CLI adapter.
- [ ] CLI package and direct-artifact inventories exclude repository-only and
      authored pack content outside their declared inputs; the full repository
      remains available through the GitHub source channel.
- [ ] All public documentation uses the new workspace paths and product names.

## Resulting System

The repository can evolve its executable, validation engine, and ecosystem
guidance at the cadence appropriate to each concern. B01 can distribute a lean
CLI, B02 can update one ecosystem's pack without rebuilding that CLI, and B03
can design true in-process APIs without extracting them from terminal code.
