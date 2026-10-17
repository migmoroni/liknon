# Plan A01: Liknon Product And Repository Separation

## Status And Purpose

This plan is the architectural prerequisite for every implementation and
publication track. It turns the current single-package repository into one
workspace with explicit ownership for deterministic validation, resource
acquisition, local resource consumption, authored resources, and
repository-only material.

Liknon has three independent command-line applications over shared,
terminal-independent contracts. The applications interoperate through
versioned files under one consumer-owned `.liknon/` root; none requires
another executable at runtime.

## Product Model

Liknon has the following current product surfaces:

1. **Liknon Core** owns deterministic validation behavior and structured
   validation contracts. It has no terminal, network, Hub, knowledge, or agent
   responsibility.
2. **Liknon Validation CLI** adapts the core to command-line arguments,
   streams, signals, progress, terminal themes, reports, schemas, and explicit
   validation bootstrap. Its commands are `liknon` and `likx`.
3. **Liknon Manager** obtains and organizes versioned resources from the
   Liknon Hub or an explicitly supplied local source. Its commands are
   `liknon-manager` and `likm`.
4. **Liknon Tool** discovers and returns installed resources for humans and
   agents without network access or resource mutation. Its commands are
   `liknon-tool` and `likt`.
5. **Liknon Resource Hub** publishes metadata and immutable payloads for
   knowledge, skills, workflows, agent profiles, tool descriptors, prompts,
   guides, templates, and collections. It is a distribution service, not a
   fourth CLI.

The applications also share one program-owned
[AI bootstrap and conformance contract](../ai-bootstrap-and-conformance-contract.md).
It is the canonical first instruction for governed external AI use, not another
application or Hub resource. Tool exposes its exact bytes through a read-only
command; no Liknon application hosts a model or conducts a workflow.

Future language SDKs and in-process validation APIs are governed by
[Plan B04](../B04-programmatic-api-and-language-bindings/README.md). They are
not required to complete the three applications.

`liknon-core` and `liknon-workspace` are supporting Rust crates. When a CLI is
published on crates.io, any supporting crate in its dependency graph is also
published there in topological order so the application package has no
unpublished path dependency. These crates expose implementation contracts
needed by the applications; they are not the supported cross-language SDK or
the future public `liknon` API governed by B04.

## Ownership Boundaries

| Concern | Canonical owner |
| --- | --- |
| Config, DAG, planning, execution, repository evidence, and structured validation reports | `crates/liknon-core` |
| Shared `.liknon/` root discovery, resource identities, manifests, locks, provenance, and compatibility contracts | `crates/liknon-workspace` |
| AI bootstrap source, identity, version, compatibility, and machine-result contract | `contracts/ai/ENTRYPOINT.md` and `crates/liknon-workspace` |
| Validation arguments, help, themes, progress, signals, human rendering, schemas, policy validation, and explicit config/policy initialization | `apps/liknon-cli` |
| Hub access, dependency resolution, downloads, verification, installation, update, removal, and explicit materialization | `apps/liknon-manager` |
| Offline catalog, search, lookup, resolution, context assembly, and structured resource output | `apps/liknon-tool` |
| Authored validation knowledge and editorial metadata | `docs/validation/knowledge` |
| Authored official skills, workflows, profiles, tool descriptors, prompts, guides, templates, and resource metadata | Type-first paths under `resources/<kind>/official/` and their declared canonical source paths |
| Plans, evaluations, fixtures, editorial checks, and maintainer procedures | Repository only |

One authored source may be assembled into immutable Hub artifacts, but the
artifact is never a second editable source.

## Target Repository Shape

```text
liknon/
├── Cargo.toml
├── Cargo.lock
├── contracts/
│   └── ai/
│       └── ENTRYPOINT.md
├── apps/
│   ├── liknon-cli/
│   ├── liknon-manager/
│   └── liknon-tool/
├── crates/
│   ├── liknon-core/
│   └── liknon-workspace/
├── docs/
│   └── validation/
│       └── knowledge/
├── resources/
│   ├── skills/
│   │   └── official/
│   ├── workflows/
│   │   └── official/
│   ├── agent-profiles/
│   │   └── official/
│   ├── tools/
│   │   └── official/
│   ├── prompts/
│   │   └── official/
│   ├── guides/
│   │   └── official/
│   ├── templates/
│   │   └── official/
│   └── collections/
│       └── official/
├── distribution/
│   └── cli/
│       ├── validation/
│       ├── manager/
│       └── tool/
├── evals/
├── fixtures/
└── tools/
```

Only directories backed by an approved current contract are created. Future
language packages belong to B04 and do not become resource distribution
channels.

## Consumer Workspace Contract

```text
.liknon/
├── validation/
│   ├── config.json
│   └── reports/
├── resources/
│   ├── manifest.json
│   ├── lock.json
│   ├── knowledge/
│   ├── skills/
│   ├── workflows/
│   ├── agent-profiles/
│   ├── tools/
│   ├── prompts/
│   ├── guides/
│   ├── templates/
│   └── collections/
└── ai/
    ├── policy.json
    └── persistence/
```

All applications use the nearest ancestor containing `.liknon/` as the
workspace boundary unless an explicit workspace path is supplied. An explicit
path selects that workspace boundary directly. Each command creates only the
subtree explicitly requested by its own contract.

Commands requiring existing state fail when discovery finds no root.
Initialization may create `.liknon/` at an explicit workspace or, when no root
exists, at the exact process current directory. No application infers a root
from Git or language tooling.

Generated caches, partial downloads, and global Hub indexes use operating
system cache and data directories rather than the consumer workspace.

## Architectural Invariants

1. Validation behavior has one implementation in `liknon-core`.
2. `liknon-core` has no Clap, terminal, Hub, resource, or authored-content
   dependency.
3. `liknon-workspace` contains contracts and path-safe primitives, not
   command parsing, network access, terminal rendering, or validation
   execution.
4. The three applications share contracts but never invoke one another as
   hidden subprocesses.
5. `liknon` and `likx` are equivalent validation entry points.
6. `liknon-manager` and `likm` are equivalent acquisition and organization
   entry points.
7. `liknon-tool` and `likt` are equivalent local-consumption entry points.
8. Manager is the only application that contacts the Hub or mutates installed
   resource state.
9. Tool consumes installed resources offline and never installs, updates,
   removes, or executes them.
10. Validation works without Manager, Tool, or installed resources.
11. Tool works with any valid local resource store, regardless of how it was
    populated.
12. Knowledge is a selectable resource kind. No application embeds or requires
    the complete corpus.
13. Resource updates do not require rebuilding any CLI.
14. Downloaded instructions are data. Installation never executes scripts or
    activates a skill implicitly.
15. `manifest.json` records requested resources; `lock.json` records exact
    resolved versions, origins, compatibility, and hashes.
16. Repository-only plans, fixtures, evaluations, and maintainer tools enter no
    application or resource artifact unless a public contract names them.
17. A crates.io application candidate declares both a local development path
    and an exact compatible registry version for each supporting crate; package
    verification proves that registry resolution needs no repository file.
18. Resource identity is `kind`, `namespace`, `name`, and version. Source
    origin, trust class, review state, and signatures are independent metadata;
    the `official` namespace grants no runtime privilege.
19. Workflows declare orchestration graphs, stable actions, and references to
    skills, agent profiles, tool descriptors, knowledge, prompts, and other
    workflows through ordinary resource dependencies. No application recognizes
    a workflow by a built-in ID.
20. Agent profiles describe roles and behavioral constraints. They do not own
    transitions, policy semantics, executable commands, or application routing.
21. `contracts/ai/ENTRYPOINT.md` is a program contract, not an installable Hub
    resource, and every governed AI operation loads its exact supported version
    and digest before resource instructions.
22. AI-Tool is an ordinary official skill selected by workflow manifests. It is
    the default of the initial official workflows but receives no built-in
    application privilege and may be replaced by a compatible alternative.

## Replacement Boundary

The product has no published compatibility contract for the current layout.
A01 updates every current consumer and removes replaced command names, paths,
embedded-corpus behavior, package-distributed resource assumptions, and
obsolete include rules. No aliases, fallback readers, or duplicate roots are
retained.

The six command names approved above are current product identities.

## Implementation Order

| Phase | Document | Depends on | Primary result |
| ---: | --- | --- | --- |
| A01.0 | [Product Boundary And Inventory](00-product-boundary-and-inventory.md) | Current repository | Complete ownership and replacement ledger |
| A01.1 | [Cargo Workspace And Core Extraction](01-cargo-workspace-and-core-extraction.md) | A01.0 | Virtual workspace, validation core, shared AI bootstrap contract, workspace contracts, and three application packages |
| A01.2 | [Knowledge And Resource Separation](02-knowledge-and-ai-resource-separation.md) | A01.1 | Modular publishable resources with one authored source |
| A01.3 | [Application Composition And Workspace Resources](03-application-composition-and-workspace-resources.md) | A01.2 | Independent validation, Manager, and Tool CLIs over one `.liknon/` contract |
| A01.4 | [Conformance And Documentation](04-conformance-and-documentation.md) | A01.3 | Proven boundaries and reconciled public documentation |

Do not begin B01, B02, or B03 implementation before A01.4. B04 may perform
design work against the approved internal contracts but may not publish an API
before A01.4.

## Global Acceptance Criteria

- [ ] The root manifest is a virtual Cargo workspace with one committed
      lockfile.
- [ ] Installing `apps/liknon-cli` provides `liknon` and `likx`.
- [ ] Installing `apps/liknon-manager` provides `liknon-manager` and
      `likm`.
- [ ] Installing `apps/liknon-tool` provides `liknon-tool` and `likt`.
- [ ] Validation-core tests construct no CLI arguments and render no terminal
      output.
- [ ] Validation artifacts contain no authored knowledge or agent resources.
- [ ] Manager artifacts contain no validation engine or authored resource
      payload beyond required built-in trust metadata.
- [ ] Tool artifacts contain no complete resource corpus and require no
      network access.
- [ ] Tool embeds only the program-owned AI bootstrap contract, returns its
      exact version and digest, and does not classify it as installed content.
- [ ] A selected knowledge resource is installed by Manager and returned
      byte-for-byte by Tool.
- [ ] Validation succeeds when Manager and Tool are absent.
- [ ] Tool succeeds against a valid manually populated resource fixture.
- [ ] Every crates.io application candidate builds from published supporting
      crates and contains no unpublished path dependency.
- [ ] No installation or lookup executes resource-provided commands.
- [ ] All maintained documentation uses the approved paths, command names,
      ownership boundaries, and publication-track numbers.
- [ ] Every maintained pre-separation AI document is updated or removed by its
      owning phase; no earlier official-workflow root or embedded-resource model
      remains as a fallback.

## Resulting System

Liknon evolves validation execution, resource acquisition, resource
consumption, authored content, and future language APIs at independent
cadences. B01 distributes the validation CLI, B02 establishes the Resource Hub
and Manager, B03 distributes the Tool CLI, and B04 may later add justified
programmatic APIs and language bindings.
