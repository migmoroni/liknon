# Plan: Validation Knowledge Base And Composable Recipes

## 1. Objective

Create a versioned validation knowledge base for humans and coding agents that
explains what to validate, why each validation matters, which tools are suitable
for each language, framework, technology, and quality concern, and how to
represent approved commands in a `workspace-validator` configuration.

Ground substantive guidance in identifiable professional sources. Preserve the
meaning and scope of external standards instead of inventing a universal
validation ladder.

The runtime remains a project-agnostic validation executor. Guidance, tool
selection, and examples live in documentation and skill references rather than
inside execution logic.

## 2. Expected Outcome

The repository provides:

- a human learning path for validation fundamentals;
- a shared taxonomy for checks, tests, validation strategies, and evidence;
- a source-backed method for selecting validation according to context and
  risk;
- focused guides classified by language, framework, technology, tool, and
  cross-cutting concern;
- complete configuration recipes that can be adapted to a workspace;
- agent guidance loaded on demand instead of placing every instruction in the
  main skill file;
- explicit safety boundaries for installation, mutation, network access, and
  external services;
- traceability from recommendations to standards, recognized frameworks, or
  official ecosystem documentation.

## 3. Architectural Boundaries

### 3.1 Runtime Responsibilities

The `workspace-validator` executable continues to:

- load and validate declarative configuration;
- resolve groups, suites, checks, parameters, dependencies, and working
  directories;
- execute approved commands without a shell;
- enforce timeouts and dependency outcomes;
- produce human and JSON reports;
- preserve repository-integrity guarantees.

The runtime does not:

- detect project technologies and choose tools automatically;
- install dependencies or system tools;
- generate policy from repository contents;
- silently modify configuration;
- decide which quality standard a project must adopt.

### 3.2 Documentation Responsibilities

Documentation explains:

- validation concepts and test boundaries;
- how risk influences validation depth;
- validation requirements intrinsic to each language, framework, and
  technology;
- exact capabilities and limitations of each documented tool;
- which claims come from external sources and which are local synthesis;
- command examples and their prerequisites;
- cost, mutation, overlap, and limitations;
- links to primary documentation.

### 3.3 Skill Projection Responsibilities

The generated knowledge projection:

- preserves the canonical authored files byte-for-byte;
- exposes stable routes for progressive disclosure;
- lets AI-Tool and AI-Engineering references load only the knowledge relevant
  to the current decision;
- contains no operational command, flow-selection, persistence, continuation,
  or authority instructions;
- never replaces AI-Tool operation references or AI-Engineering process
  references.

Inspecting a workspace, translating approved commands into configuration,
executing validation, and maintaining engineering state belong to their
respective flow plans rather than to the knowledge base.

## 4. Canonical Content Layout

Use the following structure:

```text
docs/
└── validation/
    ├── README.md
    └── knowledge/
        ├── README.md
        ├── SOURCES.md
        ├── foundations/
        │   ├── source-policy.md
        │   ├── validation-model.md
        │   ├── test-types.md
        │   └── validation-strategy.md
        ├── languages/
        │   ├── rust.md
        │   ├── javascript.md
        │   ├── typescript.md
        │   ├── ruby.md
        │   └── sql.md
        ├── frameworks/
        │   ├── svelte.md
        │   ├── tauri.md
        │   └── rails.md
        ├── technologies/
        │   └── sqlite.md
        ├── tools/
        │   ├── cargo.md
        │   ├── rustfmt.md
        │   ├── clippy.md
        │   ├── cargo-audit.md
        │   ├── cargo-deny.md
        │   ├── pnpm.md
        │   ├── typescript-compiler.md
        │   ├── vite.md
        │   ├── vitest.md
        │   ├── svelte-check.md
        │   ├── tauri-cli.md
        │   ├── bundler.md
        │   ├── rubocop.md
        │   ├── brakeman.md
        │   ├── bundler-audit.md
        │   └── sqlite3.md
        ├── concerns/
        │   ├── structured-data.md
        │   ├── accessibility.md
        │   ├── repository-integrity.md
        │   ├── packaging-release.md
        │   └── security-supply-chain.md
        └── recipes/
            ├── rust-crate.md
            ├── rust-workspace.md
            ├── svelte-application.md
            ├── tauri-svelte-application.md
            ├── rails-application.md
            └── structured-data-pipeline.md

skills/workspace-validator/
├── SKILL.md
├── manifest.json
└── references/
    └── ai-tool/
        ├── index.md
        ├── run.md
        ├── triage.md
        ├── config.md
        └── audit.md

generated skill bundle/
├── SKILL.md
├── manifest.json
└── references/
    ├── ai-tool/
    │   ├── index.md
    │   ├── run.md
    │   ├── triage.md
    │   ├── config.md
    │   └── audit.md
    └── knowledge/               # Generated from docs/validation/knowledge

examples/
└── validation-guides/
    ├── rust-crate/
    ├── rust-workspace/
    ├── svelte-application/
    ├── tauri-svelte-application/
    ├── rails-application/
    └── structured-data-pipeline/
```

`docs/validation/README.md` distinguishes shared knowledge from the three flow
entry points. `docs/validation/knowledge/README.md` is the shared learning path.
The detailed guides under `docs/validation/knowledge/` are the only authored
source for the validation knowledge base. Their Markdown serves humans and
coding agents.

Sibling trees for `reference/`, `flows/`, and human-only operator guidance are
defined by the unified validation-system plan. They are intentionally omitted
from this knowledge-specific layout and never become part of the generated
knowledge projection.

Keep AI-Tool operations below `references/ai-tool/`. Place the educational
knowledge base only in `docs/validation/knowledge/` while authoring. The
distributed skill receives a byte-equivalent generated copy below
`references/knowledge/` so it remains self-contained outside the source
repository. AI-Engineering process references remain outside this plan and
consume both namespaces without changing either one.

The guidance taxonomy is semantic rather than ecosystem-shaped:

- languages describe properties and risks of source languages;
- frameworks describe framework-specific behavior;
- technologies describe infrastructure and data engines;
- tools document concrete executables and command semantics;
- concerns describe validation dimensions that cross technical boundaries;
- recipes compose the other categories for a concrete project shape.

Do not merge different categories into a single guide. In particular, keep
Rust separate from Cargo, TypeScript separate from its compiler, Svelte separate
from `svelte-check`, Ruby separate from Rails, and SQL separate from SQLite.

## 5. Canonical Source And Skill Bundle

Do not maintain separate human and agent rewrites. Human readability, semantic
headings, compact sections, explicit terminology, and local source citations
make the canonical Markdown suitable for both audiences.

The skill bundle generation process must:

1. copy the complete `docs/validation/knowledge/` tree into
   `references/knowledge/`;
2. preserve relative paths and file contents exactly;
3. reject missing, extra, stale, or modified generated files;
4. produce a deterministic tree digest for release verification;
5. leave operational references authored inside the skill unchanged;
6. avoid a second independently editable knowledge source.

The generated bundle is the distributable artifact. Whether generated files are
staged for a source release or assembled in a release directory, tests must
prove that they are exact projections of the canonical documentation.

### 5.1 Knowledge Manifest

Extend `skills/workspace-validator/manifest.json` with a discoverable catalog.
Paths in `entries` are relative knowledge paths. The release task reads each
path below `canonicalSource` and writes the same path below `root`:

```json
{
  "knowledge": {
    "canonicalSource": "../../docs/validation/knowledge",
    "root": "references/knowledge",
    "entries": {
      "index": "README.md",
      "sources": "SOURCES.md",
      "foundations": {
        "sourcePolicy": "foundations/source-policy.md",
        "validationModel": "foundations/validation-model.md",
        "testTypes": "foundations/test-types.md",
        "validationStrategy": "foundations/validation-strategy.md"
      },
      "languages": {
        "rust": "languages/rust.md",
        "javascript": "languages/javascript.md",
        "typescript": "languages/typescript.md",
        "ruby": "languages/ruby.md",
        "sql": "languages/sql.md"
      },
      "frameworks": {
        "svelte": "frameworks/svelte.md",
        "tauri": "frameworks/tauri.md",
        "rails": "frameworks/rails.md"
      },
      "technologies": {
        "sqlite": "technologies/sqlite.md"
      },
      "tools": {
        "cargo": "tools/cargo.md",
        "rustfmt": "tools/rustfmt.md",
        "clippy": "tools/clippy.md",
        "cargoAudit": "tools/cargo-audit.md",
        "cargoDeny": "tools/cargo-deny.md",
        "pnpm": "tools/pnpm.md",
        "typescriptCompiler": "tools/typescript-compiler.md",
        "vite": "tools/vite.md",
        "vitest": "tools/vitest.md",
        "svelteCheck": "tools/svelte-check.md",
        "tauriCli": "tools/tauri-cli.md",
        "bundler": "tools/bundler.md",
        "rubocop": "tools/rubocop.md",
        "brakeman": "tools/brakeman.md",
        "bundlerAudit": "tools/bundler-audit.md",
        "sqlite3": "tools/sqlite3.md"
      },
      "concerns": {
        "structuredData": "concerns/structured-data.md",
        "accessibility": "concerns/accessibility.md",
        "repositoryIntegrity": "concerns/repository-integrity.md",
        "packagingRelease": "concerns/packaging-release.md",
        "securitySupplyChain": "concerns/security-supply-chain.md"
      },
      "recipes": {
        "rustCrate": "recipes/rust-crate.md",
        "rustWorkspace": "recipes/rust-workspace.md",
        "svelteApplication": "recipes/svelte-application.md",
        "tauriSvelteApplication": "recipes/tauri-svelte-application.md",
        "railsApplication": "recipes/rails-application.md",
        "structuredDataPipeline": "recipes/structured-data-pipeline.md"
      }
    }
  }
}
```

The manifest remains the sole source for the skill bundle version and supported
crate compatibility. Individual guides do not repeat those version values.

Automated tests verify that every catalog entry:

- is a normalized relative path without traversal;
- exists below the canonical source;
- exists with identical bytes below the active knowledge root;
- is included in the Cargo package and source release;
- can be discovered without scanning every Markdown file.

## 6. Documentation Contracts

Each category has its own contract. A shared template must not erase the
semantic difference between a language and an executable.

### 6.1 Foundation Guides

Define vocabulary, decision criteria, evidence types, and risk models without
prescribing a project-specific command set.

### 6.2 Language Guides

Describe language semantics that affect validation, common defect classes,
compiler or type-system evidence, test boundaries, and links to relevant tool
guides. They do not duplicate command manuals.

### 6.3 Framework Guides

Describe framework lifecycle, compilation, rendering, runtime, integration, and
packaging risks. They reference their underlying languages and concrete tools
instead of absorbing those subjects.

### 6.4 Technology Guides

Describe guarantees and failure modes of a data engine or infrastructure
technology. For SQLite, this includes integrity, foreign keys, transactions,
concurrency, backup, and representative query behavior.

### 6.5 Tool Guides

Document one concrete executable or tightly coupled tool, including supported
validation capability, command structure, relevant flags, prerequisites,
outputs, exit behavior, mutations, cost, limitations, and official sources.

Each tool guide includes:

| Field | Meaning |
| --- | --- |
| Purpose | Risk or defect class covered by the validator |
| Identity | Executable, package, and official project name |
| Maturity | Core, common, specialized, or experimental |
| Scope | Files, packages, targets, or runtime covered |
| Cost | Expected relative execution time and resource use |
| Mutation | Whether it may alter files, caches, databases, or external state |
| Prerequisites | Runtime, dependency, service, or credential requirements |
| Placement | Suggested suite or group role |
| Limitations | Important gaps and overlap with other checks |

### 6.6 Concern Guides

Describe cross-cutting quality dimensions independently of any language or
framework. They identify the evidence required and link to tools capable of
producing it.

### 6.7 Recipes

Compose foundation, language, framework, technology, concern, and tool guides
for a concrete project shape. A recipe contains configuration examples but does
not become a second source for tool behavior.

### 6.8 Source And Traceability Contract

`docs/validation/knowledge/SOURCES.md` is the source register. Each entry records:

| Field | Meaning |
| --- | --- |
| Source ID | Stable identifier used by the guides |
| Authority | Standards body, project, or official maintainer |
| Title | Exact publication or documentation title |
| Version | Referenced edition, release, or revision |
| URI | Primary source location |
| Scope | Decisions the source can legitimately support |
| Status | Normative standard, framework, official documentation, or local synthesis |
| Reviewed | Date on which applicability and availability were checked |

Every substantive guide contains a `Sources and traceability` section. It maps
claims or recommendations to a source ID and, when available, a section,
requirement, control, or success-criterion identifier. It also explains the
local interpretation without presenting that interpretation as source text.

Use this authority order:

1. published standards and approved specifications;
2. recognized professional assurance frameworks;
3. official language, framework, technology, and tool documentation;
4. reputable technical literature when the preceding sources do not cover the
   subject;
5. explicit local synthesis.

Normative words such as `MUST`, `SHOULD`, and conformance claims are preserved
only when the cited source defines them and the source applies to the stated
scope. Otherwise, use explanatory language and label the conclusion as local
synthesis. Do not reproduce restricted standards or extensive copyrighted
material; cite and summarize them.

The initial source register includes:

- ISO/IEC 25010:2023 for the software product quality model;
- ISTQB CTFL 4.0.1 and the ISO/IEC/IEEE 29119 family for testing vocabulary,
  test organization, and risk-based testing;
- the final NIST SP 800-218 (SSDF 1.1) for secure software development
  practices; draft revisions remain identified as drafts and do not silently
  replace the final publication;
- OWASP ASVS 5.0.0 for application security verification requirements;
- OWASP SAMM 2.0 for software-assurance process maturity;
- WCAG 2.2 for web accessibility conformance;
- SLSA 1.2 for source and build supply-chain guarantees;
- a pinned OpenSSF OSPS Baseline release for open-source project security
  controls;
- official documentation for every language, framework, technology, and tool.

## 7. Validation Model

`validation-model.md` defines validation as independent evidence dimensions and
relates them to recognized software-quality characteristics rather than treating
the dimensions as an invented universal standard:

- formatting;
- linting and static analysis;
- type checking;
- unit tests;
- component tests;
- integration tests;
- contract tests;
- end-to-end tests;
- smoke tests;
- build and package verification;
- installability and artifact inspection;
- schema, persistence, and generated-output validation;
- dependency, license, and supply-chain security;
- supported platform and runtime-version verification;
- repository integrity and mutation detection;
- performance and load testing;
- property-based, fuzz, and mutation testing.

The guide explains that no single tool proves overall correctness. It also
distinguishes source quality, behavioral confidence, packaging confidence,
security evidence, and operational confidence. Its quality vocabulary is
traceable to the source register, with local extensions identified explicitly.

## 8. Test-Type Guide

`test-types.md` defines the purpose, boundary, cost, and expected ownership of:

- unit tests;
- component tests;
- integration tests;
- contract tests;
- end-to-end tests;
- smoke tests;
- regression tests;
- property-based tests;
- fuzz tests;
- mutation tests;
- performance and load tests.

The guide includes sourced terminology, examples of overlapping tests, and an
explanation of how to avoid paying twice for equivalent evidence while retaining
complementary coverage. Where communities use the same term differently, show
the competing definitions instead of silently choosing one.

## 9. Validation Strategy And Selection

`validation-strategy.md` teaches a decision process. It does not define a
universal project-wide assurance scale.

The process guides a person or agent to:

1. identify the product, change, or operation being evaluated;
2. identify affected assets, users, data, platforms, and external obligations;
3. assess relevant product and project risks using likelihood and impact;
4. identify applicable quality characteristics and domain standards;
5. select the evidence dimensions and test types that address those risks;
6. select tools only after the required evidence is understood;
7. decide where and when each validation runs;
8. document limitations, accepted residual risk, and unresolved uncertainty.

Execution context remains independent from assurance or risk. The guide may
discuss local feedback, change review, main-branch integration, release,
scheduled audit, and incident investigation as execution contexts, but it does
not turn them into a universal progression.

Preserve official domain levels exactly and always include their source version
and scope. WCAG conformance, ASVS verification, SLSA tracks, OpenSSF project
maturity, and OWASP SAMM maturity are distinct models. Do not flatten or map
them into a shared numeric or named scale.

Recipes may demonstrate one possible selection for a stated scenario. They must
list their assumptions and remain examples rather than default policy. When
context is incomplete, the skill asks for the missing decision inputs or
presents supported alternatives instead of silently choosing a strategy.

## 10. Initial Guidance Catalog

### 10.1 Languages

- Rust: ownership and type evidence, unsafe boundaries, feature combinations,
  targets, MSRV, doctests, and package-facing APIs.
- JavaScript: runtime behavior, module boundaries, linting, tests, builds, and
  package execution.
- TypeScript: compiler configuration, type evidence, declaration output, and
  the boundary between type checking and runtime behavior.
- Ruby: dynamic-language risks, loading, tests, linting, dependencies, and
  packaging.
- SQL: DDL, constraints, representative queries, transactions, and dialect
  awareness.

### 10.2 Frameworks

- Svelte: compilation, component behavior, browser behavior, accessibility,
  SSR or prerendering, and production builds.
- Tauri: Rust/frontend integration, command contracts, permissions,
  capabilities, platform targets, bundling, and application smoke tests.
- Rails: application boot, routes, models, requests, schema consistency,
  assets, jobs, security, and production-mode behavior.

### 10.3 Technologies

- SQLite: canonical DDL, foreign-key enforcement, integrity checks,
  transactions, rollback, concurrency assumptions, and backup or restore
  verification. Migration validation applies only when the consumer explicitly
  maintains migrations.

### 10.4 Tools

Document exact behavior for the initial Rust, JavaScript/TypeScript, Svelte,
Tauri, Ruby/Rails, and SQLite tools listed in the canonical layout. A tool guide
must not present itself as the validation model for its entire ecosystem.

For example, the Rust language guide explains why feature combinations matter,
while the Cargo guide explains the commands and flags that exercise them. The
Svelte guide explains framework risks, while the `svelte-check` guide explains
the evidence produced by that executable.

### 10.5 Cross-Cutting Concerns

- Structured data and generated artifacts: JSON Schema, Markdown ASTs, links,
  deterministic generation, referential integrity, generated databases, media
  references, and checksums.
- Accessibility: static analysis, component behavior, keyboard interaction,
  semantic markup, and browser evidence.
- Repository integrity: unexpected mutation, generated files, ignored outputs,
  and reproducibility boundaries.
- Packaging and release: package contents, installability, distributable
  artifacts, checksums, signatures, and supported environments.
- Security and supply chain: advisories, licenses, provenance, lockfiles,
  secrets, and the distinction between source and runtime security.

### 10.6 Recipes

Provide composed guidance for Rust crates, Rust workspaces, Svelte applications,
Tauri applications with Svelte frontends, Rails applications, and structured
data pipelines.

## 11. Recommendation Policy

Every guide and skill follows these rules:

1. Begin with the risk being addressed, not with a fashionable tool.
2. Identify the evidence needed before selecting an implementation tool.
3. Prefer published standards, approved specifications, and primary sources.
4. Pin the referenced source version and cite its precise section or control
   when available.
5. Mark local synthesis and project policy explicitly.
6. Preserve the native meaning of domain-specific conformance levels.
7. Prefer official ecosystem capabilities and actively maintained tools.
8. Identify whether a recommendation is first-party or third-party.
9. Keep installation separate from configuration and execution.
10. Never install a dependency automatically.
11. Never recommend opaque remote shell execution.
12. Identify commands that mutate files or require external services.
13. Avoid validators that produce the same evidence without a stated reason.
14. State uncertainty when maintenance status or ecosystem consensus is weak.

Tool recommendations are reviewed periodically because versions, maintenance
status, and ecosystem practices can change independently of the crate.

## 12. Configuration Recipes

Each example directory contains a complete, minimal configuration fragment or
standalone configuration that demonstrates:

- reusable checks;
- suite-owned targets and working directories;
- structured argument parameters;
- explicit timeouts and output limits;
- groups that compose suites without duplicating execution;
- non-mutating validation commands by default.

Recipes conform to the current configuration schema and pass configuration
validation without executing their checks. They use fictional package and path
names when a real repository context is not required.

Every recipe states its scenario, assumptions, relevant risks, selected
evidence, source basis, execution contexts, and known gaps. A recipe illustrates
one defensible composition and never claims to be a default policy for all
projects of the same shape.

Do not place installers, watch commands, development servers, destructive
operations, or interactive commands in recipes.

## 13. Knowledge Routing

Keep `SKILL.md` concise. It selects AI-Tool or AI-Engineering before either flow
loads shared knowledge according to the task:

- read `foundations/validation-model.md` for validation-model questions;
- read `foundations/test-types.md` when choosing test boundaries;
- read `foundations/validation-strategy.md` when selecting validation from risk
  and context;
- consult `foundations/source-policy.md` when evaluating authority or
  traceability;
- classify the request before loading language, framework, technology, tool,
  concern, or recipe references;
- read only the exact categories needed for the current decision;
- read `concerns/security-supply-chain.md` for dependency, provenance, and
  artifact concerns.

The consuming flow explains why a proposed validator applies before introducing
it. The presence of a manifest or lockfile signals relevance, but never grants
permission to install software or alter project policy. Knowledge routing
itself never performs that installation or policy change.

The skill always resolves shared knowledge from `knowledge.root`. The
`knowledge.canonicalSource` field is maintenance metadata for repository
generation and integrity tests; it is never an alternate runtime lookup path.
The skill must not combine divergent copies or continue when bundle integrity
fails.

## 14. Implementation Phases

### Phase 1: Content Contract

- Create the shared knowledge index.
- Create the source policy and pinned source register.
- Define the taxonomy and the contract for every guidance category.
- Define the tool-guide recommendation table.
- Add canonical-source metadata, one active knowledge root, and categorized
  entries to the skill manifest.
- Define a deterministic, cross-platform repository release task that generates
  the self-contained skill bundle without entering validation runtime logic.
- Add tests for path safety, canonical presence, generated equality, packaging,
  and version ownership.
- Include the new documentation and example trees in Cargo package metadata.

### Phase 2: Foundations

- Write the validation model.
- Write the test-type guide.
- Write the validation-strategy guide based on context, likelihood, impact,
  evidence, and residual risk.
- Link the foundation guides from the knowledge index and skill knowledge
  routes.
- Verify that no foundation introduces a universal local assurance scale.

### Phase 3: Languages, Frameworks, And Technologies

- Write the Rust, JavaScript, TypeScript, Ruby, and SQL language guides.
- Write the Svelte, Tauri, and Rails framework guides.
- Write the SQLite technology guide.
- Verify that every guide links across categories rather than duplicating them.

### Phase 4: Tool Catalog

- Write the initial Rust tool guides.
- Write the initial JavaScript, TypeScript, and Svelte tool guides.
- Write the Tauri tool guide.
- Write the initial Ruby and Rails tool guides.
- Write the SQLite CLI guide.

### Phase 5: Concerns And Recipes

- Write the structured-data, accessibility, repository-integrity,
  packaging-release, and security-supply-chain guides.
- Write each recipe defined in the canonical layout.
- Add complete example configurations for every recipe.

### Phase 6: Skill Integration

- Generate `references/knowledge/` solely from the canonical knowledge
  documentation.
- Expose stable knowledge routes that independently authored AI-Tool and
  AI-Engineering references can load progressively.
- Verify that the projection contains no operational flow, persistence,
  continuation, or authority instructions.
- Record and verify the generated tree digest.
- Document that consumer workspaces may add local policy skills alongside the
  generic crate skill.
- Confirm that the distributed skill remains self-contained.

### Phase 7: Verification

- Validate the skill bundle structure.
- Verify byte equality and path equality between canonical knowledge and the
  generated skill projection.
- Validate every example configuration without running its checks.
- Test canonical-source metadata, the single active knowledge root, catalog
  routes, path safety, source version ownership, and package inclusion.
- Check internal documentation links.
- Check source-register links and references to sections or controls.
- Run formatting, Clippy, all Rust tests, and Rustdoc tests.
- Inspect `cargo package --list` and build the packaged crate.

## 15. Maintenance Rules

- Write documentation and examples in English.
- Keep one canonical source for each concept.
- Edit validation knowledge only below `docs/validation/knowledge/`; regenerate the skill
  projection after changes.
- Pin external references by version and record their review date.
- Review mutable ecosystem documentation before changing recommendations.
- Update the skill bundle version when its routing or guidance contract changes.
- Update the compatible crate version when runtime contracts require it.
- Record user-visible guidance changes in the changelog.
- Reassess third-party recommendations for maintenance and security status.
- Remove superseded guidance instead of retaining parallel variants.

## 16. Out Of Scope

- automatic dependency installation;
- automatic modification of consumer configuration;
- runtime technology detection or tool selection;
- automatic selection of validation policy or assurance level;
- a CLI configuration generator;
- MCP integration;
- exhaustive coverage of every language and framework;
- consumer-specific quality policy;
- a separately authored documentation edition for coding agents;
- redistribution of complete restricted standards;
- certification of conformance with external standards;
- compatibility layers for superseded guidance.

## 17. Acceptance Criteria

- [ ] A human can learn the validation model without reading the JSON schema
      first.
- [ ] An agent can load only the references relevant to the current task.
- [ ] `docs/validation/knowledge/` is the only authored validation knowledge
      source.
- [ ] The distributed knowledge tree is an exact generated projection of the
      canonical knowledge documentation.
- [ ] Every substantive recommendation cites an applicable source or identifies
      itself as local synthesis.
- [ ] External conformance and maturity levels retain their native names,
      versions, scopes, and meanings.
- [ ] Every tool recommendation states purpose, cost, mutation behavior,
      prerequisites, placement, limitations, and a primary source.
- [ ] Validation strategy is selected from context and risk without a universal
      project-wide scale.
- [ ] Every recipe passes configuration validation without executing checks.
- [ ] Every recipe states assumptions, source basis, execution contexts, and
      known gaps.
- [ ] The distributed skill is self-contained and uses the manifest as its
      version source.
- [ ] Manifest routes, links, examples, tests, and Cargo package contents remain
      consistent.
- [ ] Optional recommendations never become automatic installation or mandatory
      runtime behavior.
