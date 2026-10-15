# Plan A01: Liknon Product And Repository Separation

## Status And Purpose

This plan is the architectural prerequisite for every publication track. It
turns the current single-package repository into one workspace with explicit
ownership for the validation engine, two independent command-line
applications, the complete validation-knowledge corpus, ecosystem AI
resources, and repository-only material.

A01 changes repository structure and product composition without creating a
second validation implementation. Configuration, planning, execution, report,
process-control, and exit-status semantics continue to have one owner.

## Product Model

Liknon has four current product surfaces:

1. **Liknon Core** owns deterministic validation behavior and structured
   contracts. It has no terminal, CLI, package-manager, knowledge, or agent
   responsibility.
2. **Liknon CLI** adapts the core to command-line arguments, streams, signals,
   progress, terminal themes, human reports, schemas, and workspace bootstrap.
   Its canonical command is `liknon` and its explicit alias is `liknon-cli`.
3. **Liknon Knowledge** is an independent read-only application containing the
   complete validation-knowledge corpus. Its canonical command is
   `liknon-knowledge` and its compact alias is `liknon-k`.
4. **Liknon Ecosystem Packages** distribute skills, prompts, agent guidance,
   and explicit integration resources for Rust, JavaScript/TypeScript, Ruby,
   Python, and .NET. Every registry package is named `liknon` in its own
   ecosystem. These packages contain neither CLI nor knowledge corpus.

Future in-process APIs extend the ecosystem packages only after their public
contracts are approved by
[Plan B04](../B04-programmatic-api-and-language-bindings/README.md).

## Ownership Boundaries

| Concern | Canonical owner |
| --- | --- |
| Config, plan, execution, process, repository, DAG, and structured report semantics | `crates/liknon-core` |
| Argument parsing, help, themes, progress, signals, human rendering, streams, exit codes, schemas, and generic `.validation/` bootstrap | `apps/liknon-cli` |
| Knowledge catalog, stable document lookup, and read-only knowledge rendering | `apps/liknon-knowledge` |
| Authored validation knowledge and its editorial metadata | `docs/validation/knowledge` |
| Shared and ecosystem-specific skills, prompts, agent guidance, and integration resources | `ai-resources` |
| Registry adapters and explicit resource APIs | `packages/rust`, `packages/node`, `packages/ruby`, `packages/python`, and `packages/dotnet` |
| Plans, evaluations, fixtures, editorial checks, and maintainer procedures | Repository only |

The location of a file follows its owner. The validation CLI does not embed
knowledge or agent resources merely because another Liknon product can use it.

## Target Repository Shape

```text
liknon/
├── Cargo.toml
├── Cargo.lock
├── apps/
│   ├── liknon-cli/
│   └── liknon-knowledge/
├── crates/
│   └── liknon-core/
├── docs/
│   └── validation/
│       └── knowledge/
├── ai-resources/
│   ├── shared/
│   ├── rust/
│   ├── javascript-typescript/
│   ├── ruby/
│   ├── python/
│   └── dotnet/
├── packages/
│   ├── rust/
│   ├── node/
│   ├── ruby/
│   ├── python/
│   └── dotnet/
├── distribution/
│   └── cli/
│       ├── validation/
│       └── knowledge/
├── evals/
├── fixtures/
└── tools/
```

Only directories backed by an approved current contract are created. B03 may
stage package adapters after A01, and B04 may add binding crates or native
bridges only after its API contract is approved.

## Architectural Invariants

1. Validation behavior has one implementation in `liknon-core`.
2. `liknon-core` does not depend on Clap, terminal styling, global process
   exit, authored knowledge, or agent resources.
3. `liknon-cli` depends inward on the core and owns no knowledge command.
4. `liknon-knowledge` is an independent read-only application. It neither
   validates workspaces nor depends on `liknon-cli`.
5. `liknon knowledge` does not exist. Knowledge access uses
   `liknon-knowledge` or `liknon-k` directly.
6. The complete knowledge corpus is distributed as one product. Internal
   language, tool, concern, pattern, and recipe taxonomy remains available for
   precise discovery without creating separately installed units.
7. Registry packages named `liknon` contain AI-facing resources and explicit
   integration APIs, not either CLI and not the knowledge corpus.
8. Shared AI resources have one authored source. Ecosystem packages may
   assemble those exact resources with their ecosystem additions without
   forking the shared source.
9. Package installation performs no hidden download, command execution,
   workspace mutation, or agent-directory mutation.
10. Any workspace materialization is explicit, preflighted, path-safe, and
    transactional.
11. The validation and knowledge applications have independent package,
    binary, release, and compatibility identities.
12. Repository-only plans, fixtures, evaluations, and maintainer tools enter
    no application or ecosystem package unless a public contract names them.

## Replacement Boundary

The product has not published a compatibility contract for its present layout.
A01 therefore updates every current consumer and removes the replaced
single-package layout, `liknon knowledge` command, duplicate authored
resources, and obsolete package include rules. It does not retain aliases or
fallback readers for those unpublished designs.

The command aliases explicitly approved above are current product identities,
not compatibility aliases.

## Implementation Order

| Phase | Document | Depends on | Primary result |
| ---: | --- | --- | --- |
| A01.0 | [Product Boundary And Inventory](00-product-boundary-and-inventory.md) | Current repository | Approved ownership map and replacement ledger |
| A01.1 | [Cargo Workspace And Core Extraction](01-cargo-workspace-and-core-extraction.md) | A01.0 | Virtual workspace, terminal-free core, and two application packages |
| A01.2 | [Knowledge And AI Resource Separation](02-knowledge-and-ai-resource-separation.md) | A01.1 | One complete knowledge corpus plus separate canonical AI resources |
| A01.3 | [Application Composition And Workspace Resources](03-application-composition-and-workspace-resources.md) | A01.2 | Independent validation and knowledge CLIs with explicit workspace integration |
| A01.4 | [Conformance And Documentation](04-conformance-and-documentation.md) | A01.3 | Proven boundaries, equivalent validation behavior, and current documentation |

Do not begin B01, B02, or B03 implementation before A01.4. B04 may perform
design work against the approved core boundary but may not publish an API
before A01.4.

## Global Acceptance Criteria

- [ ] The root manifest is a virtual Cargo workspace with one committed
      lockfile.
- [ ] `cargo install --path apps/liknon-cli --locked` installs `liknon` and
      `liknon-cli` from one application implementation.
- [ ] `cargo install --path apps/liknon-knowledge --locked` installs
      `liknon-knowledge` and `liknon-k` from one application implementation.
- [ ] Core tests construct no CLI arguments and render no terminal output.
- [ ] The validation CLI contains no knowledge corpus or AI resource bundle.
- [ ] The knowledge CLI contains the complete canonical corpus and no
      validation execution engine.
- [ ] No `liknon knowledge` route, implicit package discovery, or workspace
      activation state remains.
- [ ] Rust, JavaScript/TypeScript, Ruby, Python, and .NET package inventories
      contain only their declared AI resources and adapters.
- [ ] Package installation mutates no consumer workspace.
- [ ] Existing config and report fixtures produce equivalent structured
      outcomes through the extracted core and validation CLI.
- [ ] Knowledge catalog and show operations return exact canonical documents
      by stable ID.
- [ ] All public documentation uses the new workspace paths, product names,
      commands, and publication-track numbers.

## Resulting System

The repository can evolve validation execution, validation knowledge, agent
assistance, and future language APIs at the cadence appropriate to each
concern. B01 distributes the validation CLI, B02 distributes the independent
knowledge CLI, B03 publishes non-CLI AI packages, and B04 can later add real
in-process APIs to those ecosystem package identities.
