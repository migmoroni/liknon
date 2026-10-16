# Plan B04: Programmatic APIs And Language Bindings

## Status

This is a future design track, not an implementation-ready commitment. It may
start only after [A01](../A01-product-and-repository-separation/README.md) has
established stable core and workspace boundaries and a concrete in-process use
case exists in at least one language ecosystem.

Do not publish placeholder packages, reserve public package identities through
empty releases, choose bridge technology, or derive an API mechanically from
CLI commands before Phase B04.0 is approved.

## Relationship To Current Products

- [B01](../B01-cli-distribution/README.md) distributes the validation CLI,
  `liknon` and `likx`.
- [B02](../B02-resource-hub-and-manager/README.md) defines the resource Hub and
  the mutating, network-capable `liknon-manager` / `likm` application.
- [B03](../B03-tool-cli/README.md) distributes the offline, read-only
  `liknon-tool` / `likt` application.
- B04 may later publish importable libraries named `liknon` in selected
  language registries.

The three applications and future libraries are separate products. A B04
library does not install, embed, invoke, or impersonate any CLI. It also does
not bundle knowledge, skills, workflows, prompts, guides, templates,
collections, or agent profiles.

A01 may publish `liknon-core` and `liknon-workspace` as supporting Cargo
dependencies so crates.io application packages can build without local paths.
That technical availability does not make either crate the supported B04 Rust
API. B04 begins only when the deliberate `liknon` library contract and a real
in-process consumer are approved.

```text
liknon repository
├── crates/liknon-core              # Validation semantics
├── crates/liknon-workspace         # Shared .liknon contracts and discovery
├── apps/liknon-cli                 # Validation CLI; B01
├── apps/liknon-manager             # Hub and resource management CLI; B02
├── apps/liknon-tool                # Local resource-consumption CLI; B03
└── language API adapters           # Added only for approved B04 consumers
```

## Objective

Expose deliberately selected Liknon capabilities in process for ecosystems
that have a demonstrated consumer:

| Ecosystem | Candidate public package | Consumption model |
| --- | --- | --- |
| Rust | crates.io `liknon` | Cargo dependency and Rust API |
| Node.js | npm `liknon` | JavaScript/TypeScript imports |
| Ruby | RubyGems `liknon` | `require` and Ruby objects |
| Python | PyPI `liknon` | Python imports |
| .NET | NuGet `liknon` | `PackageReference` and managed API |

These identities are candidates, not pre-existing contracts. Each package is
created only when its API, implementation, support matrix, documentation, and
representative consumer are ready together.

## Candidate Capabilities

Phase B04.0 evaluates and narrows capabilities such as:

- parse and validate configuration from explicit data or paths;
- inspect groups, suites, checks, tools, and resolved plans;
- execute a selected validation plan with explicit effects and cancellation;
- receive typed reports, events, and diagnostics;
- generate core-owned schemas;
- discover and inspect an explicit `.liknon/` workspace through shared
  contracts;
- read and verify installed resource metadata without acquiring or mutating
  resources.

This list is discovery input. Help rendering, ANSI presentation, process exit,
implicit current-directory behavior, package-manager discovery, Hub access,
resource installation, and CLI argument parsing are not programmatic APIs.

## Architectural Boundaries

1. `liknon-core` remains the sole validation implementation.
2. `liknon-workspace` remains the shared owner of `.liknon/` discovery and
   storage contracts.
3. Internal Rust APIs, public Rust APIs, stable ABI surfaces, and host-language
   wrappers are reviewed separately.
4. Every operation receives an explicit workspace and explicit capabilities
   for filesystem access, process execution, environment access, and network
   access.
5. Read-only APIs cannot mutate a workspace or execute configured programs.
6. Hub acquisition and canonical resource mutation remain Manager
   responsibilities; bindings cannot silently reproduce them.
7. Wrappers preserve domain meaning through idiomatic host-language types.
8. No package downloads, installs, or shells out to a Liknon CLI.
9. Native payloads, when required, are libraries or extension modules, never
   CLI executable copies.
10. No authored resource payload is duplicated into a language package.
11. A binding is published only with representative in-process behavior and
   host-language conformance tests.

## Decisions Required Before Implementation

- concrete consumers and use cases for each proposed language;
- deliberate exclusions and side-effect boundaries;
- stable internal and public Rust crate boundaries;
- sync, async, event, callback, and cancellation models;
- typed errors and panic or exception boundaries;
- API and ABI versioning;
- memory ownership, threading, serialization, and runtime loading;
- bridge technology and supported runtime versions;
- native target matrix and per-registry packaging strategy;
- relationship among core, wrapper, native library, and package versions;
- conformance fixtures, documentation, support, security, and publication.

## Proposed Repository Shape

Names remain provisional until B04.0 approves them.

```text
crates/
├── liknon-core/
├── liknon-workspace/
├── liknon-api/                     # Only if a public Rust layer is justified
└── liknon-ffi/                     # Only if approved bindings require it

packages/
├── rust/                           # Only when distinct packaging is needed
├── node/
├── ruby/
├── python/
└── dotnet/
```

Do not create an adapter directory, native bridge, or public package before its
consumer and contract are approved.

## Implementation Order

| Phase | Document | Depends on | Primary result |
| ---: | --- | --- | --- |
| B04.0 | [Programmatic API Contract](00-programmatic-api-contract.md) | A01.4 and demonstrated consumers | Approved API, effects, errors, compatibility, and binding boundary |
| B04.1 | [Rust Crate](01-rust-crate.md) | B04.0 Rust scope | Tested public Rust API when justified |
| B04.2 | [Node.js Bindings](02-node-bindings.md) | B04.0 Node scope and approved native boundary when required | Importable JS/TS API |
| B04.3 | [Ruby Bindings](03-ruby-bindings.md) | B04.0 Ruby scope and approved native boundary when required | Importable Ruby API |
| B04.4 | [Python Bindings](04-python-bindings.md) | B04.0 Python scope and approved native boundary when required | Importable Python API |
| B04.5 | [.NET Bindings](05-dotnet-bindings.md) | B04.0 .NET scope and approved native boundary when required | Importable managed API |
| B04.6 | [Cross-Language Verification And Publication](06-cross-language-verification-and-publication.md) | Implemented binding phases | Conformance evidence and isolated registry publication |

Binding phases are optional and need not launch simultaneously.

## Global Acceptance Criteria

- [ ] Every published binding has a documented, representative consumer.
- [ ] One core owns all validation semantics.
- [ ] Public APIs are designed independently from CLI syntax and presentation.
- [ ] No package embeds or invokes a Liknon CLI.
- [ ] No package embeds Hub content or authored resources.
- [ ] Every binding uses explicit workspace and side-effect boundaries.
- [ ] Errors, cancellation, events, and reports preserve documented semantics.
- [ ] Native packages contain only target-appropriate library artifacts.
- [ ] Shared conformance fixtures produce equivalent domain outcomes.
- [ ] Each registry identity is created only when its package is releasable.
- [ ] Registry publication uses isolated credentials and verified provenance.

## Resulting System

Liknon retains three focused CLIs and may gain ecosystem-native libraries only
where applications need in-process integration. Registries distribute real
language APIs, not generic content bundles or alternate CLI installers.
