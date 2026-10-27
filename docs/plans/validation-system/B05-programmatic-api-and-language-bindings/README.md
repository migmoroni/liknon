# Plan B05: Programmatic APIs And Language Bindings

## Status

`Future`.

The [validation-system index](../README.md) owns cross-track order, milestone
gates, and contract ownership. This plan owns only future programmatic APIs and
language bindings.

This is a future design track, not an implementation-ready commitment. It may
start only after [A01](../A01-product-and-repository-separation/README.md) has
established stable core and workspace boundaries and a concrete programmatic
integration use case exists in at least one language ecosystem.

Do not publish placeholder packages, reserve public package identities through
empty releases, choose bridge technology, or derive an API mechanically from
CLI commands before Phase B05.0 is approved.

## Dependencies

B05 depends on stable A01 boundaries and at least one demonstrated in-process
consumer. Each language phase additionally depends on an approved B05.0 API,
runtime support matrix, a bridge strategy when native capabilities require one,
and the
[B05 registry-readiness runbook](accounts-and-permissions.md).

## Relationship To Current Products

- [B01](../B01-validation-cli-distribution/README.md) distributes the
  `liknon-validator` / `likv` validation CLI.
- [B02](../B02-resource-hub-and-manager/README.md) defines the resource Hub and
  the mutating, network-capable `liknon-manager` / `likm` application.
- [B03](../B03-tool-cli/README.md) distributes the offline, read-only
  `liknon-tool` / `likt` application.
- [B04](../B04-unified-launcher-and-cli-suite/README.md) owns package
  `liknon-cli`, launcher commands `liknon` / `likx`, and complete suites.
- B05 may later publish importable libraries named `liknon` in selected
  language registries.

The three capability applications, launcher, and future libraries are separate
products. A B05 library does not install, embed, impersonate, or use a CLI as
hidden transport for programmatic behavior. An explicitly requested
integration API may inspect a configured CLI route and produce project
configuration that invokes it, as defined by the
[Git-hook integration contract](git-hook-integration.md). Package `liknon`
owns no command named `liknon`; that terminal identity belongs exclusively to
the B04 launcher. A library also does not bundle knowledge, skills, workflows,
agent profiles, tool descriptors, prompts, guides, templates, or collections.

A01 may identify and later distribution tracks may publish supporting Cargo
dependencies so crates.io application packages can build without local paths.
That technical availability does not make any supporting crate the supported
B05 Rust API. B05 begins only when the deliberate `liknon` library contract and
a real in-process consumer are approved.

```text
liknon repository
├── crates/<validation-core>        # Name selected by A01 when justified
├── crates/<workspace-contracts>    # Name selected by A01 when justified
├── apps/liknon-cli                 # Launcher and aggregate package; B04
├── apps/liknon-validator            # Validation CLI; B01
├── apps/liknon-manager             # Hub and resource management CLI; B02
├── apps/liknon-tool                # Local resource-consumption CLI; B03
└── language API adapters           # Added only for approved B05 consumers
```

## Objective

Expose deliberately selected Liknon capabilities and workspace-integration
APIs for ecosystems that have a demonstrated consumer:

| Ecosystem | Reserved public package identity | Consumption model |
| --- | --- | --- |
| Rust | crates.io `liknon` | Cargo dependency and Rust API |
| Node.js | npm `liknon` | JavaScript/TypeScript imports |
| Ruby | RubyGems `liknon` | `require` and Ruby objects |
| Python | PyPI `liknon` | Python imports |
| .NET | NuGet `liknon` | `PackageReference` and managed API |

An approved Git-hook integration may be the first narrow, independently useful
API of a language package. That does not authorize an empty package, alternate
CLI installer, or hidden CLI transport: the package must ship the complete
planning and explicit-application behavior with a representative consumer.

The package identity `liknon` is reserved by this plan in every activated
ecosystem, but no package exists merely because the name is approved. Each
package is created only when its API, implementation, support matrix,
documentation, and representative consumer are ready together. No empty
release is published to reserve a registry name.

## Candidate Capabilities

Phase B05.0 evaluates and narrows capabilities such as:

- parse and validate configuration from explicit data or paths;
- inspect groups, suites, checks, tools, and resolved plans;
- execute a selected validation plan with explicit effects and cancellation;
- receive typed reports, events, and diagnostics;
- generate core-owned schemas;
- discover and inspect an explicit `.liknon/` workspace through shared
  contracts;
- read and verify installed resource metadata without acquiring or mutating
  resources, including generic workflow graphs, agent-profile references, tool
  descriptors, and policy-compatibility metadata when an approved consumer
  requires them.
- plan, preview, apply, verify, and remove an explicitly requested integration
  between the Validation CLI and an existing Git-hook provider without
  replacing that provider or mutating the workspace during package
  installation.

This list is discovery input. Help rendering, ANSI presentation, process exit,
implicit current-directory behavior, package-manager discovery, Hub access,
resource installation, and CLI argument parsing are not programmatic APIs.

## Architectural Boundaries

1. The A01-selected validation owner remains the sole validation
   implementation.
2. The A01-selected workspace owner remains responsible for the shared
   `.liknon/` discovery and storage contracts it actually implements.
3. Internal Rust APIs, public Rust APIs, stable ABI surfaces, and host-language
   wrappers are reviewed separately.
4. Every operation receives an explicit workspace and explicit capabilities
   for filesystem access, process execution, environment access, and network
   access.
5. Read-only APIs cannot mutate a workspace or execute configured programs.
6. Hub acquisition and canonical resource mutation remain Manager
   responsibilities; bindings cannot silently reproduce them.
7. Wrappers preserve domain meaning through idiomatic host-language types.
8. No package downloads or installs a Liknon CLI, and no general programmatic
   API uses one as subprocess transport. An approved integration API may
   explicitly probe a selected CLI version and generate configuration whose
   runtime command invokes that CLI directly.
9. Native payloads, when required, are libraries or extension modules, never
   CLI executable copies.
10. No authored resource payload is duplicated into a language package.
11. A public resource API, when approved, exposes generic contracts and never
    branches on official workflow, node, action, transition, skill, tool, or
    profile IDs.
12. A binding is published only with representative in-process behavior and
   host-language conformance tests.

## Exclusions

B05 does not package CLIs, reserve registries speculatively, expose internal
Rust modules by default, duplicate Hub resources, become a Git-hook manager, or
begin from a desire for symmetry across language ecosystems. Package
installation never edits provider configuration or Git hooks.

## Decisions Required Before Implementation

- concrete consumers and use cases for each proposed language;
- deliberate exclusions and side-effect boundaries;
- stable internal and public Rust crate boundaries;
- sync, async, event, callback, and cancellation models;
- typed errors and panic or exception boundaries;
- API and ABI versioning;
- memory ownership, threading, serialization, and runtime loading;
- bridge technology when selected capabilities require it and supported
  runtime versions;
- native target matrix when native payloads exist and the per-registry
  packaging strategy;
- relationship among core, wrapper, native library, and package versions;
- provider-neutral Git-hook planning, ownership, merge, apply, removal, and CLI
  discovery contracts when that capability is selected;
- conformance fixtures, documentation, support, security, and publication.

## Proposed Repository Shape

Internal crate, adapter, and directory names remain provisional until B05.0
approves them. The public package identity `liknon` is already fixed.

```text
crates/
├── <validation-core>/              # Selected by A01
├── <workspace-contracts>/          # Selected by A01 when justified
├── <public-api>/                   # Only if a public Rust layer is justified
└── <ffi-layer>/                    # Only if approved bindings require it

packages/
├── rust/                           # Only when distinct packaging is needed
├── node/
├── ruby/
├── python/
└── dotnet/
```

Do not create an adapter directory, native bridge, or public package before its
consumer and contract are approved.

## Deliverables And Implementation Order

| Phase | Document | Depends on | Primary result |
| ---: | --- | --- | --- |
| B05.0 | [Programmatic API Contract](00-programmatic-api-contract.md) | A01.4 and demonstrated consumers | Approved API, effects, errors, compatibility, and binding boundary |
| B05.1 | [Rust Crate](01-rust-crate.md) | B05.0 Rust scope | Tested public Rust API when justified |
| B05.2 | [Node.js Bindings](02-node-bindings.md) | B05.0 Node scope and approved native boundary when required | Importable JS/TS API |
| B05.3 | [Ruby Bindings](03-ruby-bindings.md) | B05.0 Ruby scope and approved native boundary when required | Importable Ruby API |
| B05.4 | [Python Bindings](04-python-bindings.md) | B05.0 Python scope and approved native boundary when required | Importable Python API |
| B05.5 | [.NET Bindings](05-dotnet-bindings.md) | B05.0 .NET scope and approved native boundary when required | Importable managed API |
| B05.6 | [Cross-Language Verification And Publication](06-cross-language-verification-and-publication.md) | Implemented binding phases | Conformance evidence and isolated registry publication |

The [Git-hook integration contract](git-hook-integration.md) is a
cross-cutting B05.0 design input rather than a separate publication phase. Each
activated language phase implements it only when the approved package scope
includes that capability.

Binding phases are optional and need not launch simultaneously.

## Global Acceptance Criteria

- [ ] Every published binding has a documented, representative consumer.
- [ ] One core owns all validation semantics.
- [ ] Public APIs are designed independently from CLI syntax and presentation.
- [ ] No package embeds a Liknon CLI or invokes one as hidden programmatic
      transport; only an explicit integration compatibility probe may execute
      the caller-selected route.
- [ ] Any Git-hook integration follows explicit plan/diff/apply semantics,
      preserves unrelated configuration, is idempotent, and never mutates a
      workspace during package installation.
- [ ] Generated hooks invoke an explicitly selected public Validation command
      directly, while CI remains the authoritative validation gate.
- [ ] No package embeds Hub content or authored resources.
- [ ] Any exposed workflow, profile, tool, or policy type passes external
      non-official resource fixtures without application-specific branches.
- [ ] Every binding uses explicit workspace and side-effect boundaries.
- [ ] Errors, cancellation, events, and reports preserve documented semantics.
- [ ] Native packages contain only target-appropriate library artifacts.
- [ ] Shared conformance fixtures produce equivalent domain outcomes.
- [ ] Each registry identity is created only when its package is releasable.
- [ ] Registry publication uses isolated credentials and verified provenance.

## Resulting System

Liknon retains three focused CLIs and may gain ecosystem-native libraries only
where applications need programmatic integration. Registries distribute real
language APIs, including approved configuration integrations, not generic
content bundles or alternate CLI installers.
