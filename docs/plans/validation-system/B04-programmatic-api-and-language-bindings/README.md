# Plan B04: Liknon Programmatic API And Language Bindings

## Status

This is a future design track, not an implementation-ready commitment. It
records the boundary for real in-process validation APIs after
[A01](../A01-product-and-repository-separation/README.md) extracts the core and
[B03](../B03-ecosystem-ai-packages/README.md) establishes the `liknon` package
identity in each ecosystem.

Do not configure binding publication, select bridge technology, or derive APIs
directly from CLI commands until Phase B04.0 is approved.

## Relationship To Current Products

- [B01](../B01-cli-distribution/README.md) distributes `liknon-cli` with command
  names `liknon` and `liknon-cli`.
- [B02](../B02-knowledge-cli-and-corpus/README.md) distributes
  `liknon-knowledge` with command names `liknon-knowledge` and `liknon-k`.
- B03 publishes non-CLI ecosystem packages named `liknon` containing AI
  resources and explicit resource APIs.
- B04 extends those same ecosystem package identities with selected typed
  validation APIs.

```text
liknon repository
├── crates/liknon-core               # One validation implementation
├── apps/liknon-cli                  # Validation application; B01
├── apps/liknon-knowledge            # Knowledge application; B02
├── ai-resources + packages          # Ecosystem assistance; B03
└── API/FFI adapters in packages     # In-process validation APIs; B04
```

CLI callers use arguments, streams, presentation, and exit codes. Knowledge
callers use read-only catalog and document commands. B03 callers use resource
inventory and explicit materialization. B04 callers use typed inputs, return
values, errors, cancellation, and events.

No B04 package invokes either CLI as a hidden subprocess. B04 extends the
existing `liknon` package in each registry; it does not create a parallel
package name or replace the B03 resource contract.

## Objective

Expose useful Liknon validation capabilities in process for Rust,
JavaScript/TypeScript, Ruby, Python, and .NET.

| Ecosystem | Existing package extended | Consumption model |
| --- | --- | --- |
| Rust | crates.io `liknon` | Cargo dependency and Rust API |
| Node.js | npm `liknon` | JavaScript/TypeScript imports |
| Ruby | RubyGems `liknon` | `require` and Ruby objects |
| Python | PyPI `liknon` | Python imports |
| .NET | NuGet `liknon` | `PackageReference` and managed API |

Native payloads, when required, are libraries or extension modules for the
approved API, never copies of either CLI executable.

## Candidate Capabilities

Phase B04.0 evaluates and narrows use cases such as:

- parse and validate configuration from explicit data or path;
- inspect groups, suites, checks, tools, and resolved plans;
- execute a selected plan with explicit cancellation and events;
- receive typed reports and diagnostics;
- generate core-owned schemas;
- validate or materialize explicitly supplied initialization candidates.

This is discovery input, not a promise that every CLI command becomes a method.
Knowledge retrieval remains owned by `liknon-knowledge` unless B04 separately
approves a typed knowledge API. Help, ANSI rendering, human progress, process
exit, implicit current directory, package-manager discovery, and argument
parsing remain outside the programmatic contract.

## Architectural Boundaries

1. `liknon-core` remains the sole validation implementation.
2. Internal core, public Rust API, stable ABI, and wrappers are separately
   reviewed compatibility surfaces.
3. Bridge technology follows prototypes for safety, packaging, cancellation,
   maintenance, and error fidelity.
4. Every call receives an explicit workspace and explicit side-effect
   capabilities.
5. Read-only APIs cannot mutate a workspace or execute configured programs.
6. Wrappers preserve domain meaning through idiomatic host-language types.
7. No package installs, downloads, or shells out to either CLI.
8. Native libraries are built independently from B01 and B02 executables.
9. B03 resource APIs remain usable when B04 APIs are absent or disabled.
10. One registry package version identifies the combined public resource and
    programmatic surface for that ecosystem.
11. A binding is published only with representative in-process behavior and
    host-language conformance tests.

## Decisions Required Before Implementation

- exact public use cases and deliberate exclusions;
- stable internal and public Rust crate boundaries;
- sync, async, event, and cancellation models;
- typed errors and panic or exception boundaries;
- filesystem, subprocess, environment, callback, and threading contracts;
- API and ABI versioning;
- relationship among core, ecosystem package, resource, and wrapper versions;
- memory ownership and serialization;
- bridge technology and supported runtime versions;
- native target matrix and package strategy;
- compatibility with the existing B03 public namespace;
- conformance fixtures, documentation, support, security, and publication.

## Proposed Repository Shape

Names remain provisional until B04.0 approves them.

```text
crates/
├── liknon-core/
├── liknon-api/                      # Only if distinct from core
└── liknon-ffi/                      # Only when approved bindings require it

packages/
├── rust/                            # Extends crates.io liknon
├── node/                            # Extends npm liknon
├── ruby/                            # Extends RubyGems liknon
├── python/                          # Extends PyPI liknon
└── dotnet/                          # Extends NuGet liknon
```

Do not create a second adapter location or public package for the same
ecosystem surface.

## Implementation Order

| Phase | Document | Depends on | Primary result |
| ---: | --- | --- | --- |
| B04.0 | [Programmatic API Contract](00-programmatic-api-contract.md) | A01.4, B03 contract, and stable runtime behavior | Approved API, effects, errors, compatibility, and binding boundary |
| B04.1 | [Rust Crate](01-rust-crate.md) | B04.0 | Existing Rust `liknon` package extended with a tested API |
| B04.2 | [Node.js Bindings](02-node-bindings.md) | B04.0 and approved native boundary when required | Existing npm `liknon` package extended with a JS/TS API |
| B04.3 | [Ruby Bindings](03-ruby-bindings.md) | B04.0 and approved native boundary when required | Existing gem `liknon` extended with a Ruby API |
| B04.4 | [Python Bindings](04-python-bindings.md) | B04.0 and approved native boundary when required | Existing PyPI `liknon` project extended with a Python API |
| B04.5 | [.NET Bindings](05-dotnet-bindings.md) | B04.0 and approved native boundary when required | Existing NuGet `liknon` package extended with a managed API |
| B04.6 | [Cross-Language Verification And Publication](06-cross-language-verification-and-publication.md) | Approved binding phases | Conformance evidence and isolated registry publication |

Binding phases need not launch simultaneously.

## Global Acceptance Criteria

- [ ] One core owns all validation semantics.
- [ ] Public APIs are designed independently from CLI syntax and presentation.
- [ ] Every ecosystem extends its existing B03 `liknon` package.
- [ ] B03 resource behavior remains compatible and independently usable.
- [ ] No package installs, downloads, or shells out to a CLI.
- [ ] Every binding uses explicit workspace and side-effect boundaries.
- [ ] Errors, cancellation, events, and reports preserve documented semantics.
- [ ] Native packages contain only target-appropriate library artifacts.
- [ ] Shared conformance fixtures produce equivalent domain outcomes.
- [ ] Registry publication uses isolated credentials and verified provenance.

## Resulting System

Liknon has one validation engine, two independent CLIs, ecosystem-native AI
assistance packages, and deliberately designed in-process APIs added to those
same package identities. Registries distribute coherent ecosystem libraries
rather than generic copies of terminal applications.
