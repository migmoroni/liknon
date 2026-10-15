# Phase 2: Shared Validation Knowledge

## 1. Objective

Establish a concise, curated, source-backed validation knowledge foundation
that helps humans and agents answer four questions:

1. what can fail in the current context;
2. what evidence would meaningfully address that failure;
3. what the selected evidence can and cannot establish;
4. where to continue learning when deeper implementation detail is required.

The foundation provides orientation and decision support. It is not an offline
encyclopedia, exhaustive tool catalog, replacement for upstream documentation,
or autonomous source of project policy. Local guidance contains enough context
to identify applicability, evidence, limitations, and a safe next step. Precise
links route the reader to authoritative depth.

Canonical Markdown remains readable by humans and agents. Small structured
catalogs make it discoverable, traceable, and mechanically verifiable without
duplicating prose.

This phase owns knowledge meaning, source quality, editorial boundaries,
initial content, and routing behavior. It does not own binary embedding,
package-manager publication, pack activation, CLI distribution, AI authority,
or workspace mutation. Repository and product separation belongs to
[A01](A01-product-and-repository-separation/README.md); ecosystem delivery
belongs to [B02](B02-ecosystem-packs/README.md).

## 2. Position And Status

Phase 1 provides stable CLI and configuration behavior. Phase 2 establishes the
accepted initial corpus and deterministic content checks. Phase 2.1 applies the
full editorial profiles. A01 then assigns the accepted resources to shared,
Rust, and JavaScript/TypeScript packs before Phase 3 adds further AI operations.

Accepted content work:

- [x] Five foundation guides and three reusable pattern guides exist.
- [x] Rust and JavaScript/TypeScript language, technology, and tool guidance
      exists as explicitly scoped initial slices.
- [x] Rust CLI and JavaScript/TypeScript package recipes provide complete,
      bounded, non-publishing examples.
- [x] Catalog, source register, generated source index, routing fixtures,
      forward-trial scenarios, and review rubric exist.
- [x] Stable IDs, relationships, internal paths, citations, source locations,
      category indexes, recipe configuration, and deterministic content are
      mechanically checked.
- [x] The corpus is usable without network access while clearly identifying
      when deeper external detail is unavailable.

Packaging, pack manifests, final source paths, and CLI retrieval are A01 and
B02 acceptance criteria, not retroactive Phase 2 implementation tasks.

## 3. Knowledge Ownership Model

After A01, authored knowledge has one owner under `packs/`:

```text
packs/
├── shared/
│   └── knowledge/
│       ├── README.md
│       ├── catalog.json
│       ├── sources.json
│       ├── SOURCES.md
│       ├── explanations/
│       │   ├── foundations/
│       │   ├── patterns/
│       │   └── concerns/
│       ├── how-to/recipes/
│       └── reference/standards/
├── rust/
│   └── knowledge/
│       ├── catalog.json
│       ├── sources.json
│       ├── explanations/
│       │   ├── languages/
│       │   └── technologies/
│       ├── how-to/recipes/
│       └── reference/tools/
└── javascript-typescript/
    └── knowledge/
        ├── catalog.json
        ├── sources.json
        ├── explanations/
        │   ├── languages/
        │   └── technologies/
        ├── how-to/recipes/
        └── reference/tools/
```

This is an ownership map, not a requirement to create empty directories.
Directories exist only when they contain approved resources.

Shared foundations, patterns, general concerns, and standards remain usable by
workspaces in any language. Rust and JavaScript/TypeScript are initial positive
slices because maintained consumers exercise them. A stack-specific guide,
tool, example, or recipe never enters the shared pack merely to make the shared
catalog appear broader.

Each document has one canonical authored path and one resource identity. It may
relate to documents in other packs by stable pack and resource IDs. Do not copy
guidance because it applies to multiple facets or because several ecosystem
packages need the shared baseline.

Knowledge is audience-neutral. It contains no process activation, persistence
behavior, decision authority, or conversational control. Operational skills
and human-only guidance are separate resource kinds even when the shared pack
distributes them alongside knowledge.

## 4. Catalog Contract

Each pack maintains a machine-verifiable catalog for its substantive knowledge
documents. Pack composition produces a deterministic aggregate catalog without
rewriting the authored catalogs.

Each document entry records:

- stable resource ID and owning pack ID;
- document kind: `foundation`, `pattern`, `language`, `framework`,
  `technology`, `tool`, `concern`, `standard`, or `recipe`;
- canonical relative path inside the pack;
- concise summary and questions the document helps answer;
- applicability tags and relevant evidence dimensions;
- related resource identities;
- source IDs used by the document;
- editorial status and last review date.

Resource IDs use stable dot-delimited namespaces such as
`pattern.evidence-chain` and `tool.rust.clippy`. Evidence dimensions and
applicability tags come from declared compact vocabularies. Status is `draft`
or `reviewed`.

The catalog contains routing metadata, not a second copy of guidance. Paths
resolve inside the real owning pack, IDs are unique in that pack, and every
substantive guide appears exactly once. Category indexes and generated source
indexes are navigation assets, not duplicate topic guides.

Applicability metadata identifies possible relevance; it never establishes
that a document automatically applies to a workspace. The reader still
confirms versions, repository evidence, product consequences, and human
constraints.

## 5. Source And Traceability Contract

Each pack maintains the source records used by its guidance. Shared source
records live in the shared pack; ecosystem-only records live with that
ecosystem. A composed catalog may deduplicate an identical source identity and
digest, but packages do not fork source records.

Each source records:

- stable source ID;
- authority and exact title;
- edition, release, revision, tag, commit, or documented living-page snapshot;
- primary URI;
- supported scope and source kind;
- publication status, including draft status where applicable;
- last review date;
- named locations supporting precise local claims.

Each named location records the exact chapter, section, requirement, control,
success criterion, or documentation path, plus a deep URI where available and
the local question it supports.

Substantive recommendations cite the narrowest applicable location with a
machine-verifiable source label. If no stable deep URI exists, retain the exact
textual locator. Mark local synthesis as synthesis; do not present it as a
quotation or external normative rule.

Prefer official specifications, standards bodies, maintainers' documentation,
and primary research. Restricted standards are cited and summarized rather
than copied. Draft publications remain labeled. Time-sensitive tool guidance
identifies an appropriate stable release line or reviewed source revision
without pinning incidental local patch versions as universal requirements.

Links provide direction, not implicit network authority. Local guidance remains
useful offline and reports unavailable depth instead of inventing it.

## 6. Common Editorial Contract

Every substantive guide:

- begins with the question or decision it supports;
- states applicability and non-applicability;
- connects consequences and invariants to required evidence before naming a
  tool;
- distinguishes sourced fact, synthesis, example, and inference;
- describes what the evidence can and cannot establish;
- gives a safe, bounded next step;
- links to precise authoritative depth;
- avoids universal project policy, fashionable-tool prescriptions, and
  unsupported completeness claims;
- remains concise enough for progressive human and agent loading.

The detailed family-specific profiles, version language, Markdown structure,
and review rules belong to
[Phase 2.1](02.1-knowledge-editorial-refoundation.md).

## 7. Minimum Useful Corpus

### Foundations

The initial shared pack covers:

- a validation model linking failure, consequence, invariant, evidence,
  oracle, and residual uncertainty;
- evidence quality, including relevance, fidelity, independence,
  reproducibility, freshness, and limits;
- distinctions among static analysis, unit, integration, contract, system,
  end-to-end, acceptance, property, fuzz, mutation, performance, security,
  accessibility, compatibility, and release verification;
- validation strategy driven by risk and context rather than fixed arbitrary
  levels;
- source policy for claims and external depth.

### Reusable Patterns

The initial shared pack includes:

- evidence chains from consequence to evidence and residual uncertainty;
- isolation and determinism;
- oracles and failure paths.

### Ecosystem Slices

The Rust pack includes the current Rust, Cargo, rustfmt, Clippy, and Rust CLI
release material. The JavaScript/TypeScript pack includes the current
JavaScript, TypeScript, Node.js, pnpm, compiler, test-tool, and package-gate
material.

Recipes compose existing foundations, patterns, language guidance, and tools
for one bounded scenario. They are examples, not policy. Their complete config
examples satisfy the actual configuration schema and semantic validator without
executing declared programs.

The corpus grows only when a demonstrated consumer, flow, fixture, or decision
needs a missing topic. Missing coverage is explicit; empty placeholders do not
simulate completeness.

## 8. Retrieval And Progressive Disclosure

A01 preserves the read-only CLI namespace while changing its source from
compiled assets to explicitly active packs:

```text
liknon knowledge catalog [--format=human|json]
liknon knowledge show <resource-id>
```

The composed catalog identifies pack ownership. `show` accepts only a stable
resource selector and returns exact canonical UTF-8 Markdown bytes. It does not
accept arbitrary paths, URLs, or undocumented aliases.

Knowledge operations:

- execute before configuration loading;
- do not inspect or create `.validation/` unless an explicit pack activation
  operation was separately requested;
- do not execute checks, tools, scripts, or repository providers;
- do not read AI policy or persistence;
- do not access the network;
- emit deterministic machine output without ANSI;
- use no embedded-content or repository-relative fallback.

Progressive consumption follows this sequence:

1. inspect the composed catalog;
2. match the current question and confirmed workspace evidence to the smallest
   applicable resource set;
3. load only those exact resources;
4. load foundations or patterns only when the decision needs them;
5. follow a source location only when deeper detail is required and access is
   available and authorized;
6. report missing local coverage or inaccessible depth explicitly.

Reading knowledge never selects a flow, starts an AI-Engineering process,
writes an artifact, grants authority, or authorizes network access.

## 9. Verification And Forward Trials

Verify mechanically:

- every catalog and source register satisfies its actual compiled schema before
  typed and semantic validation;
- stable IDs are unique in their owning pack and all relationships resolve;
- every substantive Markdown guide has exactly one catalog entry;
- catalog paths, internal links, and source-location citations resolve within
  their declared boundaries;
- generated `SOURCES.md` files are deterministic;
- pack composition preserves unique identities and deterministic routing;
- exact duplicate shared packs deduplicate and conflicting bytes fail;
- recipe JSON satisfies config schema and normal semantic validation in a
  deterministic temporary workspace without executing commands;
- no empty topic or recipe placeholders enter a pack;
- no knowledge content activates flows or becomes executable policy.

Version small model-neutral routing fixtures for exact selection, valid
multi-resource composition, absent coverage, inaccessible sources, overlapping
guidance, and rejection of unrelated resources. Positive coverage includes
both Rust and JavaScript/TypeScript without cross-loading unrelated stack
guidance.

Forward trials require an agent to:

- select only relevant resources through the catalog;
- distinguish sourced fact, synthesis, example, and inference;
- move from consequence or invariant to evidence before naming tools;
- state what selected evidence cannot prove;
- use precise source locations for requested depth;
- remain useful offline and report unavailable depth;
- decline to invent guidance for a missing topic.

Phase 7 executes cross-model trials under the Agent Evaluation Program. Phase 2
keeps prompts, deterministic assertions, and rubrics provider-neutral.

## 10. Acceptance Criteria

- [x] Humans can understand the validation reasoning model without reading a
      schema.
- [x] Agents can use compact routing metadata to load only relevant guidance.
- [x] Shared guidance remains useful across ecosystems.
- [x] Rust and JavaScript/TypeScript material is explicitly scoped.
- [x] Foundations, patterns, concerns, languages, technologies, tools,
      standards, and recipes retain distinct responsibilities.
- [x] Source locations support adjacent claims and direct deeper study.
- [x] Recommendations begin with consequence, invariant, and evidence rather
      than a preferred tool.
- [x] Recipes are valid bounded examples, never implicit global policy.
- [x] Missing coverage is explicit and not replaced with invented advice.
- [x] Routing fixtures and forward trials cover both initial ecosystems.
- [x] Knowledge remains independent from validation planning and execution.
- [x] Knowledge contains no flow activation, persistence, authority, or
      human-only conversational instruction.
- [ ] A01 assigns every accepted resource to exactly one authored pack owner
      and removes embedded CLI content.
- [ ] B02 proves independent ecosystem packaging and polyglot composition.

## 11. Handoff

Phase 2.1 refounds the accepted corpus under standards-informed editorial
profiles while preserving stable resource identities and source traceability.
A01 then separates the repository and pack sources. Phase 3 consumes selected
knowledge from active packs and never copies it into an operational skill or
`.validation/`.
