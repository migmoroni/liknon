# Plan A01: Liknon Product And Repository Separation

## Status And Purpose

The [validation-system index](../README.md) owns cross-track order, milestone
gates, and contract ownership. This plan owns only the A01 structural boundary.

This plan is the structural prerequisite for later implementation and
publication tracks. It turns the current single-package repository into one
workspace with explicit ownership for deterministic validation, future resource
acquisition and consumption applications, authored resource sources, and
repository-only material. It does not implement the Resource Hub, complete
Manager behavior, complete Tool behavior, AI decision-policy commands,
workflow-persistence commands, or an official AI workflow.

The approved target architecture has three independent command-line
applications over shared, terminal-independent contracts. The applications
interoperate through versioned files under one consumer-owned `.liknon/` root;
none requires another executable at runtime.

## Product Model

Liknon has the following approved target product surfaces:

1. **Liknon Core** owns deterministic validation behavior and structured
   validation contracts. It has no terminal, network, Hub, knowledge, or agent
   responsibility.
2. **Liknon Validation CLI** adapts the core to command-line arguments,
   streams, signals, progress, terminal themes, reports, schema projection, and
   explicit validation initialization. Its commands are `liknon` and `likx`.
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

The target applications share one program-owned
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
| Config, canonical config/report schemas, DAG, planning, execution, repository evidence, and structured validation reports | `crates/liknon-core` |
| Shared `.liknon/` root discovery, path containment, canonical locations, and AI-bootstrap primitives | `crates/liknon-workspace` |
| AI bootstrap source, identity, version, digest scope, and compatibility primitives | `crates/liknon-workspace/contracts/ai/ENTRYPOINT.md`, its adjacent `manifest.json`, and `crates/liknon-workspace` |
| Validation arguments, help, themes, progress, signals, human rendering, schema projection, and explicit config initialization | `apps/liknon-cli` |
| Future AI-policy and generic workflow-persistence command adapters | `apps/liknon-cli`, implemented only by Phases 3 and 4 |
| Hub access, dependency resolution, downloads, verification, installation, update, removal, and explicit materialization | `apps/liknon-manager` |
| Offline catalog, search, lookup, resolution, context assembly, and structured resource output | `apps/liknon-tool` |
| Authored validation knowledge and editorial metadata | `docs/validation/knowledge` |
| Authored official skills, workflows, profiles, tool descriptors, prompts, guides, templates, and resource metadata | Type-first paths under `resources/<kind-directory>/official/` and their declared canonical source paths |
| Knowledge editorial schemas, evaluation schemas, plans, evaluations, fixtures, editorial checks, and maintainer procedures | Repository only |

One authored source may be assembled into immutable Hub artifacts, but the
artifact is never a second editable source.

## A01 Exit Repository Shape

```text
liknon/
├── Cargo.toml
├── Cargo.lock
├── apps/
│   ├── liknon-cli/
│   │   └── assets/branding/liknon-ascii.txt
│   ├── liknon-manager/
│   └── liknon-tool/
├── crates/
│   ├── liknon-core/
│   │   └── schemas/
│   │       ├── config.schema.json
│   │       └── report.schema.json
│   └── liknon-workspace/
│       └── contracts/ai/
│           ├── ENTRYPOINT.md
│           └── manifest.json
├── docs/
│   └── validation/
│       ├── authoring/schemas/
│       └── knowledge/
├── assets/
│   └── branding/liknon.svg
├── evals/
│   └── schemas/
├── fixtures/
└── tools/
    └── knowledge/                 # publish = false maintainer package
```

This is the required A01 exit, not the final repository after all tracks.
`resources/` begins only when an owning later phase authors or relocates a real
resource. `distribution/` begins in its owning distribution track. Empty
future directories and placeholder official resources are not created. Future
language packages belong to B04 and do not become resource distribution
channels.

## A01 Execution Contract

A01 is executed in order and every numbered unit ends in a buildable, testable
repository. A broken intermediate workspace is not an accepted handoff between
units.

1. A01.0 records the complete replacement ledger before any move.
2. A01.1 performs one atomic package cutover: the root becomes virtual only in
   the same change that creates all workspace members, relocates validation
   code and package-owned assets, removes runtime knowledge embedding, and
   preserves editorial checks as a repository-only package.
3. A01.2 completes authored-source and neutral-fixture separation only after
   the new workspace is green. It does not manufacture official resources.
4. A01.3 atomically replaces `.validation/` with `.liknon/validation/` in code,
   fixtures, tests, examples, and maintained documentation. No reader or test
   may support both roots.
5. A01.4 proves the final boundaries and updates public documentation to claim
   only behavior implemented by A01.

The initial dependency graph is deliberately small:

```text
liknon-cli --------> liknon-core
       |-----------> liknon-workspace
liknon-manager ----> liknon-workspace
liknon-tool -------> liknon-workspace
knowledge-maintenance -> liknon-core (only where config validation is needed)
```

`liknon-core` and `liknon-workspace` do not depend on each other or on an
application. Applications never depend on one another. The repository-only
knowledge maintainer is not a runtime dependency of any application.

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

This is the shared target namespace, not an A01 creation checklist. A01 creates
no empty resource, policy, report, or persistence directory merely to match the
tree; each owning later command creates only paths explicitly requested by its
implemented contract.

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
3. `liknon-workspace` contains bootstrap, workspace-boundary, and path-safe
   primitives. Resource IDs, manifests, locks, provenance, workflow, policy,
   and artifact contracts enter only in their later owning phases. It contains no
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
11. B03 must make Tool work with any valid local resource store, regardless of
    how it was populated; A01 only establishes the package and ownership
    boundary needed for that implementation.
12. Knowledge is a selectable resource kind. No application embeds or requires
    the complete corpus.
13. Resource updates do not require rebuilding any CLI.
14. Downloaded instructions are data. Installation never executes scripts or
    activates a skill implicitly.
15. Once B02 implements them, `manifest.json` records requested resources and
   `lock.json` records exact resolved versions, origins, compatibility, and
   hashes. A01 defines only their future location, not their schema.
16. Repository-only plans, fixtures, evaluations, and maintainer tools enter no
    application or resource artifact unless a public contract names them.
17. Publishable manifests are structured so their distribution tracks can later
    declare exact registry versions and prove registry-only resolution. A01
    performs no registry publication and does not require public readback.
18. Resource identity is `kind`, `namespace`, `name`, and version. Source
    origin, trust class, review state, and signatures are independent metadata;
    the `official` namespace grants no runtime privilege.
19. Workflows declare orchestration graphs, stable actions, and references to
    skills, agent profiles, tool descriptors, knowledge, prompts, and other
    workflows through ordinary resource dependencies. No application recognizes
    a workflow by a built-in ID.
20. Agent profiles describe roles and behavioral constraints. They do not own
    transitions, policy semantics, executable commands, or application routing.
21. `crates/liknon-workspace/contracts/ai/ENTRYPOINT.md` is a program contract,
    not an installable Hub resource, and every governed AI operation loads its
    exact supported version and digest before resource instructions.
22. AI-Tool is an ordinary official skill selected by workflow manifests. It is
    the default of the initial official workflows but receives no built-in
    application privilege and may be replaced by a compatible alternative.

## Replacement Boundary

The product has no published compatibility contract for the current layout.
A01 updates every current consumer and removes replaced command names, paths,
embedded-corpus behavior, package-distributed resource assumptions, and
obsolete include rules. No aliases, fallback readers, or duplicate roots are
retained.

The six command names approved above are the sole target identities A01
installs. The Manager and Tool packages are non-published structural shells in
A01: they expose identity, version, and help only, and make no claim to the
feature grammars owned by B02.0 and B03.0.

## Implementation Order

| Phase | Document | Depends on | Primary result |
| ---: | --- | --- | --- |
| A01.0 | [Product Boundary And Inventory](00-product-boundary-and-inventory.md) | Current repository | Complete ownership and replacement ledger |
| A01.1 | [Cargo Workspace And Core Extraction](01-cargo-workspace-and-core-extraction.md) | A01.0 | Atomic virtual-workspace cutover, validation core, bootstrap source and metadata, maintainer-tool preservation, and three isolated application packages |
| A01.2 | [Knowledge And Resource Separation](02-knowledge-and-ai-resource-separation.md) | A01.1 | Canonical authored sources and neutral assembly inputs without an embedded corpus, stale skill, or premature public-resource claim |
| A01.3 | [Application Composition And Workspace Boundary](03-application-composition-and-workspace-resources.md) | A01.2 | Complete Validation CLI plus isolated Manager and Tool package boundaries over one `.liknon/` root contract |
| A01.4 | [Conformance And Documentation](04-conformance-and-documentation.md) | A01.3 | Proven boundaries and reconciled public documentation |

B01 distribution work and B02/B03 feature implementation begin only after
A01.4. Their plan documents may inform A01 ownership decisions, but A01 does not
claim their public contracts or behavior complete. B04 may perform design work
against approved internal contracts but may not publish an API before A01.4.

## Per-Unit Handoff Gates

| Unit | Required handoff evidence |
| --- | --- |
| A01.0 | Current baseline gates pass; the replacement ledger covers every package input, source, export, test, schema, asset, fixture, and maintained document without an unresolved owner |
| A01.1 | Workspace metadata matches the approved graph; formatting, Clippy, all-target tests, doctests, alias smoke tests, bootstrap-digest checks, maintainer checks, and local package inventories pass from the virtual workspace |
| A01.2 | Editorial schemas and sources have one owner; maintainer checks and neutral exact-byte assembly fixtures pass; application and support-crate inventories contain no authored corpus or stale skill |
| A01.3 | Validation behavior and outcome fixtures pass exclusively through `.liknon/validation/`; root discovery, explicit-root, symlink, containment, initialization, help, stream, JSON, signal, and exit tests pass; stale-path search is clean |
| A01.4 | Full workspace, MSRV, package-boundary, local-install, local-link, documentation, and repository-search gates pass with no behavior claim assigned to a later phase |

At each handoff, `git diff --check` and the repository's complete formatting,
lint, test, and doctest commands run against the whole workspace. A unit is not
marked complete by passing only the package it changed.

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
- [ ] Manager shell artifacts contain no validation engine, authored resource
      payload, Hub implementation, or premature trust metadata.
- [ ] Tool artifacts contain no complete resource corpus and require no
      network access.
- [ ] The canonical bootstrap Markdown and metadata define one version, media
      type, path, and SHA-256 digest over exact UTF-8 bytes without newline
      normalization; Tool projection remains a B03 responsibility.
- [ ] The root has no package, `build.rs`, runtime knowledge module, knowledge
      command, or package include list left from the former single crate.
- [ ] Knowledge editorial checks run from the repository-only
      `liknon-knowledge-maintenance` package with `publish = false`.
- [ ] Canonical validation schemas are package-contained by `liknon-core`;
      canonical editorial schemas remain maintainer-owned under
      `docs/validation/authoring/schemas/`, and evaluation-only schemas remain
      under `evals/schemas/`.
- [ ] The old `skills/liknon` source and its tests are removed; Phase 3, rather
      than A01, authors the final official AI-Tool resource.
- [ ] The old programmatic `examples/inspect.rs` surface is removed rather than
      preserving an accidental pre-B04 public API.
- [ ] Validation succeeds when Manager and Tool are absent.
- [ ] Manager and Tool package shells build independently without containing
      validation execution or authored payloads.
- [ ] Neutral authored-resource and workspace fixtures have one owner and are
      ready for B02.0 and B03.0 contract work.
- [ ] Package inspection uses only local workspace candidates; registry-only
      resolution and public readback remain distribution-track gates.
- [ ] No installation or lookup executes resource-provided commands.
- [ ] All maintained documentation uses the approved paths, command names,
      ownership boundaries, and publication-track numbers.
- [ ] Every maintained pre-separation AI document is updated or removed by its
      owning phase; no earlier official-workflow root or embedded-resource model
      remains as a fallback.

## Resulting System

Liknon leaves A01 with preserved validation behavior, explicit package and
source ownership, and stable structural boundaries. B02 then establishes the
Resource Hub and Manager behavior, B03 implements Tool behavior, B01 owns
Validation CLI distribution, and B04 may later add justified programmatic APIs
and language bindings.
