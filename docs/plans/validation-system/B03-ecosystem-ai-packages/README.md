# Plan B03: Liknon Ecosystem AI Packages

## Status And Relationship To Other Tracks

This plan publishes the AI-facing resources separated by
[A01](../A01-product-and-repository-separation/README.md) through the native
package registry of each supported ecosystem.

[B01](../B01-cli-distribution/README.md) distributes validation execution.
[B02](../B02-knowledge-cli-and-corpus/README.md) distributes the complete
knowledge application. B03 packages contain neither CLI nor knowledge corpus.
[B04](../B04-programmatic-api-and-language-bindings/README.md) may later extend
these package identities with real in-process APIs.

## Objective

Publish one package named `liknon` in each initial ecosystem:

| Ecosystem | Registry | Initial package role |
| --- | --- | --- |
| Rust | crates.io | Rust API for shared and Rust agent resources |
| JavaScript/TypeScript | npm | Node API and types for shared and JS/TS agent resources |
| Ruby | RubyGems | Ruby API for shared and Ruby agent resources |
| Python | PyPI | Python API for shared and Python agent resources |
| .NET | NuGet | Managed API for shared and .NET agent resources |

The identical name is not a collision because each registry has its own package
namespace. None of these packages installs a terminal command.

## Resource Boundary

The initial packages may contain:

- the versioned Liknon skill router;
- AI-Tool operation instructions;
- AI-Engineering process instructions;
- shared safety and policy boundaries;
- prompts and templates for agent interaction;
- ecosystem-specific directions for selecting tools and invoking installed
  Liknon applications;
- a small native API for inventory, exact resource bytes, and explicit
  caller-selected materialization;
- package metadata and documentation.

They do not contain:

- `liknon`, `liknon-cli`, `liknon-knowledge`, or `liknon-k` executables;
- the validation-knowledge corpus;
- configuration planning, execution, report, or terminal-rendering logic;
- a hidden CLI subprocess adapter;
- a post-install hook that changes a workspace or agent configuration;
- a downloader for another Liknon product;
- empty placeholder bindings.

## Consumption Model

```text
native package manager installs liknon
                  |
                  v
consumer imports the package resource API
                  |
                  v
consumer inspects exact resource IDs and bytes
                  |
                  v
human explicitly selects a workspace destination
                  |
                  v
package preflights and materializes only requested resources
                  |
                  v
agent loads the installed skill and invokes:
  liknon / liknon-cli for validation
  liknon-knowledge / liknon-k for knowledge
```

Package installation and workspace integration are different actions. The
package manager owns dependency resolution and locking. The package API owns
only its immutable resources and explicit materialization.

## Canonical Resource Model

Shared resources are authored once under `ai-resources/shared`. Ecosystem
resources are authored under their corresponding `ai-resources/<ecosystem>`
directory.

Each package assembles:

- one exact shared resource set;
- one ecosystem-specific resource set;
- one deterministic manifest with stable IDs, paths, media types, lengths, and
  SHA-256 digests;
- one minimal adapter in the host language.

The same shared resource version and digest are byte-identical across all five
packages. Ecosystem resources may route to applicable `liknon-knowledge`
document IDs, but do not copy those documents.

## Version And Compatibility Model

Keep these identities explicit:

- package version in each registry;
- AI-resource schema version;
- shared resource-set version;
- ecosystem resource-set version;
- minimum supported `liknon-cli` capability when an operation invokes
  validation;
- minimum supported `liknon-knowledge` capability when an operation retrieves
  knowledge;
- future programmatic API version added by B04.

Package versions may advance independently. Compatibility is declared and
verified; it is never inferred from equal version numbers.

## Security And Mutation Rules

1. Installation executes no workspace-mutating hook.
2. Resources are fully declared and hashed.
3. Resource text is untrusted data until an agent explicitly loads the
   applicable skill operation.
4. Materialization requires an explicit real workspace and destination.
5. Preflight validates containment, symlinks, existing bytes, conflicts, and
   the complete write set before mutation.
6. Writes are atomic and idempotent for identical bytes.
7. No package discovers or installs either CLI.
8. Policy ownership remains human-only; no packaged instruction may authorize
   policy mutation.
9. Package lockfiles and resource manifests identify the exact instructions
   used for an agent-assisted action.

## Proposed Repository Shape

```text
ai-resources/
├── shared/
├── rust/
├── javascript-typescript/
├── ruby/
├── python/
└── dotnet/

packages/
├── rust/
├── node/
├── ruby/
├── python/
└── dotnet/
```

Package directories contain adapters and assembly metadata. They do not become
second authored copies of `ai-resources`.

## Implementation Order

| Phase | Document | Depends on | Primary result |
| ---: | --- | --- | --- |
| B03.0 | [Package And Resource Contract](00-package-and-resource-contract.md) | A01.4 | Approved manifest, API, materialization, version, and package contracts |
| B03.1 | [Shared AI Resources](01-shared-ai-resources.md) | B03.0 | One canonical shared router, safety boundary, and common operation set |
| B03.2 | [Rust Package](02-rust-package.md) | B03.1 | crates.io `liknon` package without binaries |
| B03.3 | [Node Package](03-node-package.md) | B03.1 | npm `liknon` package without `bin` |
| B03.4 | [Ruby Package](04-ruby-package.md) | B03.1 | RubyGems `liknon` package without executables |
| B03.5 | [Python Package](05-python-package.md) | B03.1 | PyPI `liknon` package without console entry points |
| B03.6 | [.NET Package](06-dotnet-package.md) | B03.1 | NuGet `liknon` library package, not a .NET Tool |
| B03.7 | [Polyglot Conformance](07-polyglot-conformance.md) | Implemented package phases | Equivalent shared resources and coherent multi-package workspace use |
| B03.8 | [Publication And Maintenance](08-publication-and-maintenance.md) | B03.7 | Isolated publication and independent resource maintenance |

Package phases may be implemented incrementally, but B03.7 must include every
package selected for the initial public release.

## Global Acceptance Criteria

- [ ] Every registry package is named `liknon` and installs no command.
- [ ] No package contains either CLI or the knowledge corpus.
- [ ] Shared resource bytes are identical across packages.
- [ ] Ecosystem additions contain no unrelated ecosystem guidance.
- [ ] Each host-language API exposes deterministic inventory and exact bytes.
- [ ] Explicit materialization is safe, transactional, and independently
      requested.
- [ ] Installing, importing, or inspecting a package mutates no workspace.
- [ ] Skills route validation to `liknon` and knowledge to
      `liknon-knowledge`.
- [ ] A polyglot workspace can install several packages without identity,
      file, or instruction conflict.
- [ ] Package updates can ship without rebuilding either CLI.
- [ ] B04 can extend the existing package identities without replacing the
      B03 resource contract.

## Resulting System

Every stack receives native package-manager access to relevant AI assistance,
while validation execution and knowledge remain focused applications. A
polyglot workspace combines package resources without duplicating CLIs or
embedding fast-changing guidance inside an executable.
