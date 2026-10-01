# Phase 2: Shared Validation Knowledge

## 1. Objective

Establish one concise, curated, source-backed validation knowledge foundation
that helps humans and agents answer four questions:

1. what can fail in the current context;
2. what evidence would meaningfully address that failure;
3. what the selected evidence can and cannot establish;
4. where to continue learning when deeper implementation detail is required.

The foundation provides orientation and decision support. It is not an offline
encyclopedia, an exhaustive tool catalog, a replacement for upstream
documentation, or an autonomous source of project policy. Local guidance
contains enough context to identify applicability, evidence, limitations, and a
safe next step. Precise links point to authoritative material for depth.

Canonical Markdown remains the readable knowledge content shared by humans and
agents. Small structured JSON catalogs make that content discoverable,
traceable, and mechanically verifiable without duplicating its prose. The
distributed skill receives an exact generated projection of the canonical
knowledge assets that currently exist.

An external agent may read the canonical public documentation directly without
installing or loading the skill. The generated projection improves routed,
progressive AI-Tool and AI-Engineering consumption but is not the only valid
access path.

The knowledge base grows only from demonstrated validation needs. This phase
establishes its editorial and retrieval contracts, source policy, initial
foundations, reusable patterns, and a minimum useful corpus. Later phases add a
topic before relying on that topic in a flow, fixture, or recommendation.

This phase supplies reasoning and reference material. It does not select AI
authority, activate a flow, execute validation, or claim comprehensive coverage.

## 2. Dependency

Phase 1 is complete. Guidance describes stable CLI and configuration behavior
rather than compensating for unfinished runtime contracts.

## 3. Canonical Layout

Create the canonical knowledge tree:

```text
docs/validation/
├── README.md
└── knowledge/
    ├── README.md
    ├── catalog.json
    ├── sources.json
    ├── SOURCES.md                  # Generated human-readable source index
    ├── foundations/
    │   ├── README.md
    │   ├── source-policy.md
    │   ├── validation-model.md
    │   ├── evidence-quality.md
    │   ├── test-types.md
    │   └── validation-strategy.md
    ├── patterns/
    │   ├── README.md
    │   ├── evidence-chain.md
    │   ├── isolation-and-determinism.md
    │   └── oracles-and-failure-paths.md
    ├── languages/
    │   ├── README.md
    │   └── <language>.md
    ├── frameworks/
    │   ├── README.md
    │   └── <framework>.md
    ├── technologies/
    │   ├── README.md
    │   └── <technology>.md
    ├── tools/
    │   ├── README.md
    │   └── <tool>.md
    ├── concerns/
    │   ├── README.md
    │   └── <concern>.md
    ├── standards/
    │   ├── README.md
    │   └── <standard>.md
    └── recipes/
        ├── README.md
        └── <scenario>.md
```

The directories are editorial facets, not a rigid ontology. A document has one
canonical path and may relate to documents in several other facets through the
catalog. Do not duplicate guidance merely because it applies to more than one
language, framework, technology, or concern.

The root index distinguishes shared knowledge from CLI reference material and
the three product flows without duplicating their contracts. The knowledge
index explains how to move from concepts to patterns, stack guidance, tools,
standards, and concrete recipes. Foundation documents provide the reasoning
model. Patterns capture reusable validation approaches independently from one
specific tool. Recipes compose the smallest relevant set of these materials for
one real scenario.

Everything below `knowledge/` is audience-neutral. It contains no process
activation, persistence behavior, decision authority, flow-specific control,
or human-only conversational guidance. Human Flow documentation, AI-Tool
instructions, AI-Engineering process contracts, and human-only operator guides
remain in their dedicated locations.

## 4. Knowledge Catalog Contract

Create `knowledge/catalog.json` as the machine-verifiable inventory and routing
map for every substantive knowledge document. Validate it through the crate-owned
`schemas/knowledge-catalog.schema.json`.

Each document entry records:

- stable document ID;
- document kind: `foundation`, `pattern`, `language`, `framework`,
  `technology`, `tool`, `concern`, `standard`, or `recipe`;
- canonical relative path;
- concise summary;
- questions the document helps answer;
- applicability tags and relevant evidence dimensions;
- related document IDs;
- source IDs used by the document;
- document status and last review date.

Document IDs use stable dot-delimited namespaces such as
`pattern.evidence-chain` and `tool.rust.clippy`. Evidence-dimension IDs and
applicability tags come from compact vocabularies declared by the catalog; do
not create synonymous free-form tags in individual entries. Document status is
either `draft` or `reviewed`.

The catalog contains routing metadata, not a second copy of the guidance. Its
summaries are short enough to inspect before loading a document. Paths resolve
inside `knowledge/`, document IDs are unique, and every substantive Markdown
guide appears exactly once. Category indexes and generated `SOURCES.md` are
declared separately as navigation assets rather than topic guides.

Applicability metadata identifies where guidance may be relevant; it never
claims that a document is automatically applicable to a workspace. The agent
still confirms versions, repository evidence, product consequences, and human
constraints before making a recommendation.

## 5. Source And Traceability Contract

Create `knowledge/sources.json` as the canonical versioned source register and
validate it through the crate-owned `schemas/knowledge-sources.schema.json`.
Generate `knowledge/SOURCES.md` deterministically from that register for human
reading. Do not maintain the JSON and Markdown source indexes independently.

Each source records:

- stable source ID;
- authority and exact title;
- edition, release, or revision;
- primary URI;
- supported scope and source kind;
- publication status, including draft status when applicable;
- last review date;
- one or more named locations when the source supports specific local claims.

Each named location records:

- stable location ID within the source;
- exact chapter, section, requirement, control, success criterion, or
  documentation path;
- a deep URI when the publisher provides one;
- the local question or recommendation the location supports.

Source IDs use stable dot-delimited namespaces such as `rust.clippy`. Location
IDs are source-local kebab-case identifiers such as `lint-levels`.

Substantive recommendations cite the narrowest applicable location with a
machine-verifiable Markdown link label such as:

```text
[Source: rust.clippy#lint-levels](https://doc.rust-lang.org/clippy/...)
```

When a stable deep URI does not exist, the link uses the primary URI and the
source location still records the exact textual locator. Local interpretation
is explicitly marked as synthesis and remains linked to the primary material
that supports it. Synthesis is not presented as a quotation or normative rule.

The initial register contains only sources cited by current guidance. Prefer
official specifications, standards bodies, maintainers' documentation, and
primary research over secondary summaries. Restricted standards are cited and
summarized rather than copied. Draft publications remain labeled as drafts and
do not silently replace final editions.

Links provide direction, not implicit network authority. The local guide must
still contain enough information to explain relevance, evidence, limitations,
and the safe next step when external access is unavailable. If deeper detail is
required and cannot be accessed, the agent reports that limitation instead of
inventing the missing content.

## 6. Common Editorial Contract

Every substantive guide uses stable headings that answer, when applicable:

- what question the guide addresses;
- when it applies and when it does not;
- relevant failure modes or consequences;
- evidence the approach can produce;
- what that evidence cannot establish;
- prerequisites, relative cost, mutation behavior, and external services;
- useful combinations and meaningful overlap with other evidence;
- practical selection guidance;
- precise next steps and source locations.

Documents remain concise. Prefer one directly useful explanation and precise
navigation to authoritative depth over copied manuals, long option catalogs, or
generic prose. Clearly separate sourced fact, local synthesis, example, and
unresolved uncertainty.

Language, framework, technology, and concern guides apply this common contract
to their subject. Standard guides preserve the standard's own version, scope,
levels, and terminology rather than translating them into a universal assurance
scale.

A tool guide additionally records:

- executable and package identity;
- versions or ranges for which commands were reviewed;
- covered defect or evidence class;
- useful command structure rather than an exhaustive command reference;
- output and exit behavior;
- overlap with other tools and known blind spots.

A recipe additionally records:

- one concrete scenario and its assumptions;
- risks, invariants, and required evidence;
- selected patterns and topic guides;
- tools only after required evidence is established;
- execution contexts and ordering;
- one complete schema-valid example for the declared scenario;
- known gaps, rejected alternatives when material, and residual uncertainty.

Recipes demonstrate composition. They never become mandatory project policy or
universal stack presets.

## 7. Foundation And Minimum Useful Corpus

### 7.1 Validation Model

Provide a concise map of independent evidence dimensions, including:

- formatting and linting;
- type and static analysis;
- unit, component, integration, contract, end-to-end, and smoke tests;
- build, package, installation, and artifact inspection;
- persistence, schema, and generated-output validation;
- accessibility;
- dependency, license, and supply-chain security;
- repository consistency and reproducibility;
- platform and runtime support;
- performance, load, property-based, fuzz, and mutation testing.

For each dimension, explain the question it helps answer, its principal
limitations, and its relationship to adjacent evidence. No single dimension,
tool, or universal checklist establishes complete correctness.

### 7.2 Evidence Quality

Explain the chain from consequence or invariant to claim, failure mode,
observable evidence, validation mechanism, result, limitation, and residual
uncertainty. Cover test oracles, false confidence, false positive and false
negative outcomes, determinism, flakiness, representative fixtures, and the
difference between executing a check and establishing a claim.

### 7.3 Test Types

Establish practical distinctions among core test types by summarizing purpose,
boundary, ownership, relative cost, limitations, and common overlaps. Where a
terminology difference changes a decision, identify the competing definitions
rather than selecting one silently.

### 7.4 Validation Strategy

Teach this decision process:

1. identify the product, change, or operation;
2. identify affected users, assets, data, platforms, and obligations;
3. identify concrete consequences, invariants, and uncertainty;
4. assess relevant likelihood and impact without inventing a universal score;
5. identify applicable quality characteristics and domain standards;
6. select required evidence dimensions, patterns, and test types;
7. select tools after the evidence is understood;
8. decide where and when each validation runs;
9. record gaps, overlap, uncertainty, and accepted residual risk.

Preserve official levels such as WCAG conformance, ASVS verification, SLSA
tracks, SAMM maturity, and OSPS maturity within their native version and scope.

### 7.5 Initial Reusable Patterns

The initial pattern set establishes:

- an evidence chain from risk or invariant to bounded validation evidence;
- isolation, deterministic inputs, reproducible execution, and explicit
  treatment of flaky evidence;
- suitable oracles, negative cases, failure paths, and limits of success-only
  validation.

These patterns remain tool-neutral. Later patterns are added only when a real
scenario cannot be explained adequately by the existing set.

### 7.6 Minimum Corpus Boundary

Phase 2 requires:

- the root and category indexes;
- valid catalog and source registries plus generated `SOURCES.md`;
- the five foundation documents;
- the three initial reusable pattern documents;
- at least one complete worked recipe drawn from a maintained repository or
  evaluation fixture;
- only the topic guides needed to support that recipe, the current repository,
  and Phase 3 acceptance scenarios.

Do not populate every category, language, framework, technology, standard, or
tool. Do not create empty topic files, speculative catalogs, or placeholder
pages. The absence of a topic is explicit and never authorizes an agent to
invent local guidance.

When a later phase introduces a fixture or recommendation that depends on a new
topic, that phase first adds the smallest source-backed guide needed for the
scenario and regenerates the projection. Knowledge growth follows demonstrated
use, not anticipation.

## 8. Skill Projection And Progressive Disclosure

Keep `skills/workspace-validator/SKILL.md` as a small router. Extend the skill
manifest with maintenance metadata pointing to the canonical knowledge tree and
one active knowledge route below `references/knowledge/`. Do not duplicate the
complete document catalog in the skill manifest.

A deterministic repository release task copies the current canonical tree to
the distributed skill while preserving paths and bytes. The generated copy is
never edited independently. Its root index points to the projected
`catalog.json`, which remains the authoritative inventory for progressive
loading inside the knowledge branch.

Progressive disclosure follows this sequence:

1. load the knowledge index and compact catalog;
2. match the current question and confirmed workspace evidence to the smallest
   applicable document set;
3. load foundations or patterns only when the decision requires them;
4. load exact topic guides and recipes instead of whole categories;
5. follow an external source location only when deeper detail is required and
   access is available and authorized;
6. report missing local coverage or inaccessible depth explicitly.

The catalog narrows context; it does not make the decision. AI-Tool and
AI-Engineering references remain distinct from shared knowledge. Loading
knowledge never selects a flow, starts a process, writes an artifact, grants
authority, or authorizes network access.

Tests reject:

- missing, extra, stale, or modified generated files;
- unsafe or traversing catalog and manifest paths;
- routes outside the active knowledge root;
- package contents that omit routed knowledge;
- a version declared independently from the skill manifest.

## 9. Verification And Forward Trials

Verify mechanically:

- catalog and source registries satisfy their schemas;
- every stable ID is unique and every relationship resolves;
- every substantive Markdown guide has exactly one catalog entry;
- catalog paths, internal links, and source-location citations resolve;
- generated `SOURCES.md` is deterministic and matches `sources.json`;
- canonical and projected trees have identical paths and bytes;
- the projected tree digest is deterministic across supported hosts;
- every authored recipe contains a complete schema-valid example without
  executing its checks;
- category indexes list only documents that currently exist;
- no empty topic or recipe placeholders enter the tree;
- the source package contains the intended canonical and projected assets.

Version small routing fixtures that map representative questions and workspace
facts to expected document IDs. Cover exact selection, valid multi-document
composition, absent local coverage, inaccessible external sources, overlapping
guidance, and rejection of unrelated documents.

Define model-neutral forward-trial scenarios in which an agent must:

- select only relevant guidance through the catalog;
- distinguish sourced fact, synthesis, example, and inference;
- move from consequence or invariant to required evidence before naming tools;
- state what selected evidence cannot prove;
- use precise source locations for requested depth;
- remain useful without network access and report unavailable depth;
- decline to invent guidance for a missing topic.

Phase 2 versions prompts, fixtures, deterministic assertions, and review
rubrics. Phase 7 executes cross-model trials under the shared Agent Evaluation
Program, avoiding an AI-provider dependency in the knowledge implementation.

Normal deterministic tests validate URI structure and registered locators
without depending on external network availability. Link-health checks may run
separately and update review metadata; transient publisher availability does not
make the local package nondeterministic.

## 10. Acceptance Criteria

- [x] A human can understand the validation reasoning model without reading a
      JSON schema.
- [x] An agent can inspect the compact catalog and load only the guidance needed
      for one decision.
- [x] Initial guidance explains relevance, evidence, limitations, and safe next
      steps without external access.
- [x] Precise source locations direct deeper study without reproducing upstream
      manuals.
- [x] The minimum corpus supports at least one complete real validation recipe.
- [x] Foundations, patterns, topic guides, tools, standards, and recipes retain
      distinct responsibilities.
- [x] Directory placement remains editorial while catalog metadata provides
      cross-facet discovery.
- [x] `docs/validation/knowledge/` is the sole canonical shared-knowledge source.
- [x] The distributed knowledge is an exact generated projection.
- [x] Missing topic coverage is explicit and never replaced by invented advice
      or empty placeholders.
- [x] Recommendations begin with consequence, invariant, and required evidence,
      not a fashionable tool.
- [x] External standards retain their native versions, levels, and scopes.
- [x] Recipes are valid examples and never implicit global policy.
- [x] Routing fixtures define the expected minimal document sets for Phase 7
      forward trials without encouraging eager category loading.
- [x] The runtime remains independent from guidance and tool selection.
- [x] Shared knowledge contains no Human Flow, AI-Tool, AI-Engineering, or
      human-only operator instructions.

## 11. Handoff To Phase 3

Phase 3 uses the minimum useful knowledge corpus in the policy-governed AI-Tool
Flow while keeping each operational skill bounded to one request. It registers
the source entries and smallest topic guides required by its own acceptance
scenarios before relying on them. It does not turn educational material into
executable policy or AI-Engineering context.
