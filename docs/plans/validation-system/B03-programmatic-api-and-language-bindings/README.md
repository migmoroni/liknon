# Plan B03: Liknon Programmatic API And Language Bindings

## Status

This is a future design track, not an implementation-ready commitment. It
records the boundary for real in-process validation APIs after
[A01](../A01-product-and-repository-separation/README.md) separates the core,
CLI, and generic pack infrastructure.

Do not configure binding publication, reserve implementation technology, or
derive APIs directly from CLI commands until Phase B03.0 is approved.

## Relationship To The CLI And Packs

[B01](../B01-cli-distribution/README.md) distributes the command-line
application. [B02](../B02-ecosystem-packs/README.md) distributes authored
knowledge, skills, recipes, and minimal resource locators. B03 exposes selected
validation behavior as typed in-process APIs.

```text
liknon repository
├── crates/liknon-core               # One validation implementation
├── crates/liknon-pack               # Generic pack contract
├── apps/liknon-cli                  # Executable `liknon`; B01
├── packs and ecosystem locators     # Content products; B02
└── language adapters/bindings       # In-process validation APIs; B03
```

The products share internal behavior but have different public contracts:

- CLI callers use arguments, streams, terminal presentation, and exit codes;
- B02 callers locate immutable content packs and explicitly supply them to the
  CLI or another approved consumer;
- B03 callers use typed inputs, return values, errors, cancellation, events,
  and explicit side-effect boundaries.

No B03 package invokes the CLI as a hidden subprocess. A B03 package may extend
an existing B02 ecosystem package only after Phase B03.0 defines one coherent
name, version, dependency, and compatibility contract. Otherwise it uses a
separate package identity. Registry name reuse is never assumed.

## Objective

Expose useful Liknon validation capabilities as in-process APIs for Rust,
JavaScript or TypeScript, Ruby, Python, and .NET, then publish each approved
implementation through the package registry native to that language.

| Ecosystem | Public form | Consumption model |
| --- | --- | --- |
| Rust | Source crate from crates.io | Cargo dependency and Rust API |
| Node.js | npm package | JavaScript/TypeScript imports |
| Ruby | RubyGem | `require` and Ruby objects |
| Python | PyPI project | Python imports |
| .NET | NuGet package | `PackageReference` and managed API |

These packages install no global `liknon` command. Native payloads, when
needed, are libraries or extension modules for the approved API, not copies of
the CLI executable.

## Candidate Capabilities

Phase B03.0 evaluates and narrows candidate use cases such as:

- parse and validate configuration from explicit data or an explicit path;
- inspect groups, suites, checks, tools, and resolved plans;
- execute a selected plan with explicit cancellation and event handling;
- receive typed reports and diagnostics;
- generate core-owned schemas;
- validate pack manifests or consume caller-supplied composed pack catalogs;
- validate or materialize explicitly supplied initialization candidates without
  terminal coupling.

This is discovery input, not a promise that every CLI command becomes a library
method. Help, ANSI rendering, human progress, process exit, implicit current
directory, package-manager discovery, and argument parsing remain outside the
programmatic contract.

## Architectural Boundaries

1. `liknon-core` remains the sole validation implementation.
2. The internal core API, public Rust API, any stable ABI, and language wrappers
   are separately reviewed compatibility surfaces.
3. FFI technology is selected only after prototypes compare safety,
   maintenance, packaging, cancellation, and error fidelity.
4. Every call receives an explicit workspace boundary and explicit
   capabilities for filesystem or process effects.
5. Read-only APIs cannot mutate a workspace or execute configured programs.
6. Wrappers expose idiomatic types while preserving exact domain meaning.
7. No binding package contains a global CLI launcher or command shim.
8. Native library artifacts are built and verified independently from B01 CLI
   binaries and B02 content packages.
9. Package and API versions are intentional contracts, not accidental copies
   of the CLI or pack version.
10. A registry package is published only when it has a representative
    in-process API and conformance tests in that host language.

## Decisions Required Before Implementation

- exact public use cases and deliberately excluded concerns;
- internal crate visibility and public Rust package boundary;
- sync, async, event, and cancellation models;
- typed error taxonomy and panic or exception boundaries;
- filesystem, subprocess, environment, callback, and threading contracts;
- API and ABI versioning;
- relationship among CLI, pack, core, ABI, and wrapper versions;
- memory ownership and serialization;
- per-language bridge technology and supported runtime versions;
- native target matrix and package strategy;
- whether a B03 API extends or remains separate from each B02 package;
- registry names, owners, trusted publishing, and provenance;
- conformance fixtures, documentation, support, and security policy.

## Proposed Repository Shape

Names remain provisional until B03.0 approves them.

```text
crates/
├── liknon-core/
├── liknon-pack/
├── liknon-api/                      # Only if distinct from the core boundary
└── liknon-ffi/                      # Only if approved bindings need it

bindings/
├── node/
├── ruby/
├── python/
└── dotnet/
```

If a binding extends a B02 package, its adapter may live under the corresponding
`packages/` directory instead. Do not create both locations for the same public
surface.

## Implementation Order

| Phase | Document | Depends on | Primary result |
| ---: | --- | --- | --- |
| B03.0 | [Programmatic API Contract](00-programmatic-api-contract.md) | A01.4 and stable relevant runtime behavior | Approved API, side effects, errors, compatibility, and binding boundary |
| B03.1 | [Rust Crate](01-rust-crate.md) | B03.0 | Importable source crate with tested Rust API |
| B03.2 | [Node.js Bindings](02-node-bindings.md) | B03.0 and an approved native boundary when required | Importable JavaScript/TypeScript validation API |
| B03.3 | [Ruby Bindings](03-ruby-bindings.md) | B03.0 and an approved native boundary when required | Importable Ruby validation API |
| B03.4 | [Python Bindings](04-python-bindings.md) | B03.0 and an approved native boundary when required | Importable Python validation API |
| B03.5 | [.NET Bindings](05-dotnet-bindings.md) | B03.0 and an approved native boundary when required | Importable managed validation API |
| B03.6 | [Cross-Language Verification And Publication](06-cross-language-verification-and-publication.md) | Approved binding phases | Conformance evidence and isolated registry publication |

Binding phases need not launch simultaneously. Each enters B03.6 only after its
own design, packaging, and representative API behavior are complete.

## Global Acceptance Criteria

- [ ] One core owns all validation semantics.
- [ ] Public APIs are designed independently from CLI syntax and presentation.
- [ ] No language package installs, downloads, or shells out to the CLI.
- [ ] Every binding uses explicit workspace and side-effect boundaries.
- [ ] Errors, cancellation, events, and reports preserve documented semantics.
- [ ] B02 resource location and B03 validation APIs have an explicit package
      composition decision for every ecosystem.
- [ ] Native packages contain only target-appropriate library artifacts.
- [ ] Shared conformance fixtures produce equivalent domain outcomes.
- [ ] Registry publication uses isolated credentials and verified provenance.
- [ ] B01 CLI and B02 packs remain independently installable and releasable.

## Resulting System

Liknon has one shared validation engine, one CLI application, independently
evolving ecosystem guidance packs, and a family of deliberately designed
in-process APIs. Registries distribute real ecosystem capabilities rather than
generic copies of the CLI.
