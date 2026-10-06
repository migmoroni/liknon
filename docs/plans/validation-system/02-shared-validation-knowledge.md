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
traceable, and mechanically verifiable without duplicating its prose.
`docs/validation/knowledge/` is the only authored and tracked knowledge tree.
The distributed skill remains the agent-facing operational interface, but it
contains no copy of shared knowledge. No generated source tree, release mirror,
shared-knowledge skill subtree, or consumer template duplicates these files in
the repository.

The build incorporates the canonical knowledge assets into the CLI without
creating tracked generated files. A human or agent working in the source
repository may read the canonical files directly. A consumer using an installed
binary accesses the same versioned content through `liknon
knowledge`. The command provides progressive disclosure without requiring a
checkout, network access, workspace configuration, or a copied knowledge tree.

The knowledge base grows only from demonstrated validation needs. This phase
establishes its editorial and retrieval contracts, source policy, initial
foundations, reusable patterns, and a minimum useful corpus. Later phases add a
topic before relying on that topic in a flow, fixture, or recommendation.

This phase supplies reasoning and reference material. It does not select AI
authority, activate a flow, execute validation, or claim comprehensive coverage.

The CLI and its knowledge model are ecosystem-agnostic. Rust and
JavaScript/TypeScript are the initial positive implementation slices because
they are the stacks currently exercised by maintained consumer workspaces.
Their guides, tools, examples, and recipes remain explicitly scoped. Shared
foundations, patterns, and general concerns must remain usable by workspaces in
other languages without being routed through Rust or JavaScript-specific
material.

## 2. Implementation Status And Dependency

This document is a continuation plan over an implemented and verified baseline.
The status markers are normative:

- `[x]` records accepted behavior that already exists. It is not an
  implementation task and must not be rebuilt or replaced.
- `[ ]` records correction or completion work that remains in this phase.

Preserve accepted content contracts and regression coverage. A completed area
may be touched only when a remaining item requires a narrowly scoped correction.
Do not add compatibility layers, deprecated aliases, migrations, parallel
knowledge trees, or a second knowledge source.

### 2.1 Accepted Baseline

- [x] `docs/validation/knowledge/` is the single canonical shared-knowledge
      tree.
- [x] `catalog.json` provides compact routing metadata for every substantive
      guide.
- [x] `sources.json` is the canonical source register and deterministically
      generates `SOURCES.md`.
- [x] Stable IDs, catalog relationships, internal paths, citation identifiers
      and registered URIs, category indexes, and the deterministic canonical
      tree digest are checked.
- [x] The five foundation guides and three reusable pattern guides exist.
- [x] Rust and JavaScript/TypeScript language, technology, and tool guidance
      exist as explicitly scoped topic slices.
- [x] The Rust CLI and JavaScript/TypeScript package recipes contain complete,
      bounded, non-publishing configuration examples.
- [x] Routing fixtures, forward-trial prompts, deterministic assertions, and a
      review rubric are versioned for later cross-model execution.
- [x] The catalog, source register, routing fixtures, forward-trial registry,
      rubric, and recipe configurations receive structural and semantic
      validation.
- [x] Formatting, Clippy with warnings denied, regular tests, MSRV `1.87.0`,
      canonical knowledge verification, and current source-package construction
      pass.

### 2.2 Accepted CLI And Packaging Implementation

1. [x] Remove the tracked
       `skills/liknon/references/knowledge/` subtree and every
       contract that projects shared knowledge into the skill. Preserve
       `SKILL.md`, `manifest.json`, and the current `run`, `triage`, `config`,
       and `audit` operational references.
2. [x] Incorporate the canonical `docs/validation/knowledge/` tree into the
       compiled binary through deterministic build output under `OUT_DIR`, with
       no generated file committed to the repository.
3. [x] Implement the read-only `liknon knowledge` CLI namespace
       for catalog discovery and document retrieval by stable document ID.
4. [x] Refactor knowledge release tooling, Cargo package contents, tests, and
       public documentation around the canonical tree and embedded CLI assets,
       removing projection generation and equality checks.
5. [x] Update routing fixtures, forward-trial context, this plan set's
       architecture overview, and downstream phase references so the skill and
       direct agents retrieve shared knowledge through the CLI rather than a
       copied skill subtree.
6. [x] Prove that knowledge commands work from an empty directory without
       `.validation/`, do not create workspace files, and return content matching
       the canonical source bytes.
7. [x] Rerun formatting, Clippy, regular tests, doctests, MSRV, source-package
       construction, canonical knowledge verification, and the existing
       package matrix.

### 2.3 Remaining Correction Work

1. [x] Add automated installed-package coverage to the package CI job. Build
       and install the source package, run `knowledge catalog --format=json`
       and `knowledge show foundation.validation-model` from an empty temporary
       directory, and compare both outputs byte-for-byte with the corresponding
       canonical files inside the packaged source. The check must exercise the
       installed binary and must not resolve knowledge through repository-local
       runtime paths.
2. [x] Correct the confirmed source-revision mismatch. Replace the unbounded
       `pnpm 11.x` review claim with the exact release, tag, or commit actually
       reviewed. Update affected locations, adjacent guide wording, and
       generated `SOURCES.md` together without changing unrelated guidance.
3. [x] Rerun the focused knowledge release check, installed-package smoke test,
       complete Rust verification matrix, MSRV verification, package listing,
       dependency audit, and repository-integrity check after the two
       corrections above.

### 2.4 Dependency

Phase 1 is complete. Guidance describes stable CLI and configuration behavior
rather than compensating for unfinished runtime contracts.

## 3. Canonical Layout

Preserve this canonical knowledge tree and extend only the categories required
by the remaining corpus:

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
or human-only conversational guidance. Public Human Flow, AI-Tool,
AI-Engineering, and human-only operator documents remain in their dedicated
locations below `docs/validation/`. Operational agent instructions remain in
the distributed skill. The repository does not maintain a parallel shared-
knowledge hierarchy below `skills/`.

## 4. Knowledge Catalog Contract

Maintain `knowledge/catalog.json` as the machine-verifiable inventory and
routing map for every substantive knowledge document. Validate it through the
crate-owned `schemas/knowledge-catalog.schema.json`.

Schema validation means compiling the declared Draft 2020-12 schema with a
maintained validator compatible with the crate MSRV and validating the raw JSON
instance against it before deserializing the typed model. A hand-written subset
of schema constraints is not an equivalent implementation. Keep semantic checks
for cross-document relationships, filesystem state, citation resolution, and
other rules that JSON Schema cannot express cleanly. Enable the standard format
assertions used by these schemas, including `date`, rather than treating those
keywords as annotations only.

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

Maintain `knowledge/sources.json` as the canonical versioned source register
and validate it through the crate-owned
`schemas/knowledge-sources.schema.json`.
Generate `knowledge/SOURCES.md` deterministically from that register for human
reading. Do not maintain the JSON and Markdown source indexes independently.

Compile and execute this schema through the same Draft 2020-12 validation path
used by the catalog. Diagnostics identify the instance path and relevant schema
location without replacing semantic source and citation checks.

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

Audit each citation as an editorial claim, not merely as a matching identifier
and URI. The registered location must support the adjacent statement at the
declared scope. In particular, replace the current Rust test-organization
location used for assertions, expected panic, and `Result`-returning tests with
an exact supporting location, or remove that Rust-specific example from the
general oracle pattern. Living documentation records the precise reviewed
revision or review snapshot available from its publisher; an unbounded version
range is not an exact revision.

Complete the remaining source correction without weakening this contract:

- `pnpm.cli-11` must identify the exact pnpm release, documentation tag, or
  source commit whose behavior was reviewed. `pnpm 11.x` is not an acceptable
  revision because it denotes multiple releases;
- when the authoritative documentation uses a living public page for a precise
  location, retain that deep link for continued study but pair it with an exact
  reviewed release or immutable source snapshot in the source record;
- regenerate `SOURCES.md` from the corrected register and update a local guide
  only when its declared reviewed scope or citation text no longer matches the
  exact source revision.

Do not broaden this correction into a rewrite of the initial knowledge corpus.
The task is to make existing source identity and review claims reproducible.

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

Apply this scope boundary throughout the tree:

- foundations and patterns are technology-neutral;
- a general concern begins with ecosystem-independent risks, evidence, and
  limitations;
- language, framework, technology, and tool documents contain ecosystem-specific
  behavior;
- recipes compose explicitly declared stacks and never redefine a shared
  concept;
- a stack-specific example inside a general document is labeled as an example,
  appears in a bounded subsection, and does not determine the document title,
  question, applicability, or general recommendation;
- when guidance only applies to one ecosystem, give it an explicitly scoped
  title, catalog applicability, canonical path, and related topic documents
  instead of presenting it as shared guidance.

A document carrying `general` applicability must remain useful without loading
any language, framework, package manager, compiler, or test-runner guide.
Foundations and patterns must carry `general` and must not depend on a
stack-specific applicability tag. A general concern may relate to scoped
examples, but its own applicability and evidence chain remain independent from
them.

Language, framework, technology, and concern guides apply the common editorial
contract to their subject. Standard guides preserve the standard's own version,
scope, levels, and terminology rather than translating them into a universal
assurance scale.

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

Refine the implemented patterns as follows:

- preserve the evidence-chain pattern as shared guidance;
- keep dependency locking in isolation and determinism as an
  ecosystem-independent requirement, with Cargo or pnpm only as labeled
  examples of mechanisms that may contribute bounded evidence;
- ground oracle and failure-path guidance in an applicable general testing
  source; move Rust-only outcome mechanisms to the Rust guide or a labeled Rust
  example with an exact citation;
- ensure each pattern can be selected for an unrelated ecosystem without first
  loading Rust or JavaScript/TypeScript material.

### 7.6 Minimum Corpus Boundary

Phase 2 requires:

- the root and category indexes;
- valid catalog and source registries plus generated `SOURCES.md`;
- the five foundation documents;
- the three initial reusable pattern documents;
- one complete Rust worked recipe drawn from this maintained repository;
- one complete JavaScript/TypeScript worked recipe drawn from a maintained
  consumer shape or deterministic evaluation fixture;
- only the topic guides needed to support that recipe, the current repository,
  and Phase 3 acceptance scenarios.

The JavaScript/TypeScript slice is intentionally narrow. Add only:

- the applicability vocabulary entries `javascript`, `typescript`, `nodejs`,
  and `pnpm`, used only by documents whose scope actually requires them;
- concise `language.javascript` and `language.typescript` guidance with
  distinct boundaries;
- `technology.nodejs` for the runtime contract;
- `tool.pnpm` for package management and script execution;
- `tool.typescript-compiler` and `tool.vitest` for the static/type and test
  evidence used by the worked recipe;
- one complete recipe covering immutable dependency expectations, static/type
  evidence, tests, build or package evidence when applicable, repository
  mutation observation, and explicit limitations;
- at least one positive routing fixture and one forward-trial scenario that
  select this slice without loading Rust guidance.

Use Vitest because it is already exercised by the maintained
JavaScript/TypeScript consumer scenario. Do not add unrelated frameworks,
linters, package managers, or deployment systems merely to make the catalog
appear broad. Svelte and other framework-specific guidance remain absent until
a demonstrated scenario needs framework evidence that the language, runtime,
tools, and shared patterns do not provide.

Refactor `concerns/reproducible-release-evidence.md` as a genuinely general
release concern. Its question, catalog applicability, risks, evidence, limits,
and next steps cover source-to-artifact release claims independently from Cargo,
pnpm, crates, or npm packages. It may link to the Rust and
JavaScript/TypeScript recipes as bounded ecosystem applications. Cargo packaging
behavior remains in `tools/cargo.md`, the Rust guide, and the Rust recipe rather
than defining the shared concern. Its catalog applicability includes `general`,
`generated-assets`, and `release`, and no longer uses `cargo` or another
stack-specific tag.

Do not populate every category, language, framework, technology, standard, or
tool. Do not create empty topic files, speculative catalogs, or placeholder
pages. The absence of a topic is explicit and never authorizes an agent to
invent local guidance.

When a later phase introduces a fixture or recommendation that depends on a new
topic, that phase first adds the smallest source-backed guide needed for the
scenario and rebuilds the embedded knowledge assets. Knowledge growth follows
demonstrated use, not anticipation.

## 8. Embedded Knowledge And CLI Retrieval

### 8.1 Single-Source Build Contract

Keep every authored knowledge asset below `docs/validation/knowledge/`. Add no
tracked knowledge projection below `skills/`, `src/`, `generated/`, `dist/`, or
another directory. Remove only
`skills/liknon/references/knowledge/` and the package rules, tools,
manifest metadata, or tests that preserve that copied subtree. Do not keep it as
ignored output: the repository build does not create it at all.

Preserve the distributed skill as a separate agent-facing concern. `SKILL.md`,
`manifest.json`, and the current `run`, `triage`, `config`, and `audit`
references contain routing and operational instructions rather than shared
validation knowledge. Factual CLI behavior belongs in the applicable canonical
document below `docs/validation/reference/`. Phase 3 owns the immutable policy
boundary, the public AI-Tool documents, and the corresponding new operational
skill references without copying shared knowledge into either surface.

The build incorporates the canonical tree directly into the executable. Use a
small deterministic build step that:

1. walks `docs/validation/knowledge/` recursively;
2. normalizes and sorts repository-relative paths;
3. rejects non-UTF-8 content, unsafe paths, duplicate normalized paths, symlinks,
   and unsupported file kinds;
4. emits a Rust asset registry only under Cargo's `OUT_DIR`;
5. emits precise `cargo:rerun-if-changed` directives for the canonical root and
   its current files;
6. makes the compiled registry available through a dedicated internal
   `knowledge` module.

The generated registry is build output, not a second source tree. Do not create
a release mirror or require a generated knowledge directory to be present in a
checkout. Keep `docs/validation/**` and the build entrypoint in the Cargo source
package so `cargo package` and installation from that package compile the same
embedded assets. The implementation must not require a network connection or a
workspace `.validation/` directory at build or query time.

Use this ownership boundary:

```text
build.rs                    # Deterministic canonical-tree asset registry
src/knowledge/mod.rs        # Read-only embedded catalog and ID lookup
src/cli/knowledge.rs        # CLI rendering and error translation
docs/validation/knowledge/  # Only authored and tracked knowledge assets
```

The build step can use the Rust standard library to discover and sort files; do
not add a build dependency merely for directory embedding. `src/knowledge/`
owns no validation execution behavior and `src/cli/knowledge.rs` owns no
canonical content.

Refactor the existing release helper so its write mode generates only derived
files that canonically belong inside `docs/validation/knowledge/`, currently
`SOURCES.md`. Remove `PROJECTED_ROOT`, knowledge projection copying, projected
tree comparison, and skill-manifest knowledge metadata. Keep canonical schema,
citation, fixture, recipe, source-index, and tree-digest verification.

### 8.2 CLI Contract

Add this read-only command namespace:

```text
liknon knowledge catalog [--format=human|json]
liknon knowledge show <document-id>
```

`knowledge catalog` reads the embedded `catalog.json`. Human output provides a
compact list of document IDs, kinds, summaries, and applicability. JSON output
returns the complete canonical catalog JSON so an agent can route without
loading every guide. Catalog `path` values remain canonical metadata relative
to `docs/validation/knowledge/`; installed consumers select documents by ID and
must not interpret those values as local workspace paths.

`knowledge show` accepts only a stable catalog document ID and writes that
document's canonical UTF-8 Markdown bytes to standard output. Preserve the
canonical final newline without adding a second one. The catalog already
provides structured metadata, so do not introduce a second per-document JSON
envelope or schema.

Do not accept arbitrary filesystem paths, URLs, or undocumented aliases as
document selectors. An unknown or malformed ID is a usage/selection error with
exit code `3`, a diagnostic on standard error, and no partial document on
standard output. Failure to decode or access an asset compiled into the binary
is an internal error with exit code `4`.

Both commands:

- work independently from configuration discovery and before any call to
  `config::load`;
- do not inspect or create `.validation/`;
- do not replace or shadow embedded documents with workspace files,
  environment variables, remote content, or fallback directories;
- do not read workspace policy or persistence;
- do not execute checks, tools, shell commands, or repository providers;
- do not access the network;
- do not emit progress bars, ANSI styling, or incidental diagnostics on
  standard output;
- produce deterministic output for the same binary and arguments.

Dispatch the `knowledge` namespace in the same early read-only CLI layer as
`schema`, before the branch that resolves or loads consumer configuration. Do
not thread an optional configuration path through these commands.

### 8.3 Progressive Disclosure

Progressive agent consumption follows this sequence:

1. run `liknon knowledge catalog --format=json`;
2. match the current question and confirmed workspace evidence to the smallest
   applicable document set;
3. run `liknon knowledge show <document-id>` only for those exact
   documents;
4. load foundations or patterns only when the decision requires them;
5. follow an external source location only when deeper detail is required and
   access is available and authorized;
6. report missing local coverage or inaccessible depth explicitly.

The catalog narrows context; it does not make the decision. Reading embedded
knowledge never selects a flow, starts an AI-Engineering process, writes an
artifact, grants authority, or authorizes network access.

### 8.4 Consumer Initialization Boundary

Phase 2 does not materialize shared knowledge in `.validation/`. The knowledge
commands remain available whether or not the current directory belongs to an
initialized workspace.

Future initialization capabilities use explicit resource flags. Each flag
creates only the consumer-owned resource requested by that invocation. They do
not implicitly install every AI flow or copy the complete knowledge tree.
Skill installation remains a separate documented distribution action owned by
the AI-flow phases; it installs the operational bundle without materializing
shared knowledge in `.validation/` or inside the skill. Shared knowledge
continues to be retrieved from the CLI.

### 8.5 Repository Reconciliation

Reconcile every current consumer of the removed knowledge projection. At
minimum:

- retain `/skills/**` in `Cargo.toml` package inclusion and include `/build.rs`;
- remove only `skills/liknon/references/knowledge/` from the
  repository;
- preserve the root README's skill-installation procedure while documenting
  that installed skills retrieve shared knowledge through
  `liknon knowledge`;
- refactor `tools/knowledge/` and `tests/knowledge.rs` around canonical and
  embedded assets;
- update `tests/skills.rs` to verify skill routing, compatibility metadata, the
  absence of a copied knowledge subtree, and use of the CLI retrieval contract;
- update `docs/validation/README.md` and
  `docs/validation/knowledge/README.md` with direct-source and installed-CLI
  access paths;
- reconcile `docs/plans/validation-system/README.md`,
  `01-human-flow-completion.md`, `03-ai-tool-flow.md`, and
  `04-ai-engineering-persistence-and-safety-foundation.md` so no phase restores
  a tracked knowledge projection while the operational skill remains intact.

Use the crate version as the version of the embedded knowledge. The existing
skill manifest continues to version the operational skill and declare CLI
compatibility, but it contains no knowledge root, digest, projection version, or
copied-file routing metadata. Do not create an independent version for the
knowledge retrieval surface.

## 9. Verification And Forward Trials

Verify mechanically:

- the catalog and source registries satisfy their compiled Draft 2020-12
  schemas, including closed objects, enums, patterns, cardinality, and
  `uniqueItems` constraints;
- every stable ID is unique and every relationship resolves;
- every substantive Markdown guide has exactly one catalog entry;
- catalog paths, internal links, and source-location citations resolve;
- generated `SOURCES.md` is deterministic and matches `sources.json`;
- the build asset registry contains every canonical knowledge file exactly once
  and contains no path outside the canonical tree;
- the canonical tree digest and compiled asset digests are deterministic across
  supported hosts;
- every authored recipe contains exactly one complete JSON configuration
  example that satisfies `schemas/config.schema.json` and the same semantic and
  filesystem-aware configuration validation used by the CLI, against a
  deterministic temporary workspace and without executing any declared tool or
  check;
- category indexes list only documents that currently exist;
- no empty topic or recipe placeholders enter the tree;
- the source package contains the canonical knowledge tree, the build logic,
  and the operational skill, contains no
  `skills/liknon/references/knowledge/` subtree, and can build and
  query knowledge from the packaged source.

Extend the existing package workflow with a regression gate over the installed
artifact. After `cargo package --locked` and `cargo install --path
<package-dir>`, the workflow must:

1. change to an empty temporary directory that has no `.validation/` ancestor;
2. run the installed binary's `knowledge catalog --format=json` command and
   compare its standard output byte-for-byte with
   `<package-dir>/docs/validation/knowledge/catalog.json`;
3. run the installed binary's `knowledge show foundation.validation-model`
   command and compare its standard output byte-for-byte with
   `<package-dir>/docs/validation/knowledge/explanations/foundations/validation-model.md`;
4. fail on any command error or byte mismatch and confirm that neither command
   creates files in the temporary directory.

This gate complements the in-repository integration tests. It specifically
proves that package inclusion, build-time embedding, installation, and runtime
retrieval remain connected in the artifact distributed to users.

Validate the recipe JSON exactly as authored. Place that unchanged document at
a deterministic temporary configuration path, create only the filesystem shape
needed for its declared `workspaceRoot` and suite directories, and call the
ordinary configuration loader. Do not rewrite paths, insert missing fields,
weaken validation, discover tools, or execute the resulting plan merely to make
an example pass.

Version small routing fixtures that map representative questions and workspace
facts to expected document IDs. Cover exact selection, valid multi-document
composition, absent local coverage, inaccessible external sources, overlapping
guidance, and rejection of unrelated documents.

Update forward-trial contexts and prompts to treat
`liknon knowledge catalog --format=json` and
`liknon knowledge show <document-id>` as the installed-consumer
retrieval surface. They may still read `docs/validation/knowledge/` directly
when the scenario explicitly represents an agent operating in the source
repository. Remove projected knowledge paths, knowledge fields from the skill
manifest, and generated-copy assumptions from fixtures, assertions, and
rubrics. Preserve the skill manifest and operational routes in scenarios that
exercise AI-Tool or AI-Engineering.

Define and enforce complete crate-owned schemas at
`schemas/knowledge-routing.schema.json`,
`schemas/knowledge-forward-trials.schema.json`, and
`schemas/knowledge-rubric.schema.json` for the routing fixture, forward-trial
scenario registry, and evaluation rubric respectively. Their validators reject:

- missing or unknown fields;
- duplicate case, scenario, criterion, assertion, or document IDs;
- empty questions, facts, prompts, behavior names, or assertions;
- overlap between expected and rejected document sets;
- unknown document IDs, behavior IDs, rubric criteria, or unsafe prompt paths;
- a declared prompt or rubric path that does not resolve inside its canonical
  root;
- scenario sets that omit one of the required reasoning behaviors;
- positive routing coverage that exercises only one programming ecosystem.

Keep semantic checks after schema validation for catalog references, expected
and rejected set relationships, required behavior coverage, and filesystem
resolution. Do not infer missing fixture fields or accept malformed assets just
because the current typed models can deserialize a subset.

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

Add negative regression tests that mutate isolated in-memory or temporary copies
of every structured asset. At minimum, prove rejection of an unknown property,
invalid enum, malformed ID, invalid date, duplicate `uniqueItems` value, missing
required property, unsafe path, unresolved relationship, expected/rejected
overlap, stale generated source index, unsafe embedded asset path, and catalog
entries without a corresponding embedded document. These tests must exercise
the same validators used by the release check rather than testing an unrelated
approximation.

The release check compiles each schema once per invocation, validates raw JSON,
then deserializes and applies semantic checks. This includes the catalog,
source register, routing fixture, forward-trial registry, evaluation rubric,
and extracted recipe configuration examples. Recipe examples additionally pass
through the ordinary semantic configuration validator after schema validation.

The JSON Schema dependency is a development/release-tooling concern and must
not enter validation execution or knowledge routing. Select a maintained Draft
2020-12 implementation compatible with MSRV `1.87.0`; adding that dependency
requires the repository's normal human authorization before implementation.

Add CLI integration coverage that proves:

- `knowledge catalog` succeeds in human and JSON formats from an empty temporary
  directory with no `.validation/` ancestor;
- `knowledge show` returns the exact canonical Markdown for every cataloged
  document ID;
- malformed IDs, unknown IDs, path traversal strings, and canonical paths used
  as selectors are rejected without leaking another document;
- broken standard output follows the existing internal-output failure contract;
- repeated queries do not create or mutate files in the current directory;
- validation commands continue to use only `.validation/config.json` and do not
  load knowledge as execution policy;
- automated packaged-source verification installs the packaged binary, runs it
  from an empty temporary directory, and proves that its catalog and selected
  document bytes equal the canonical files contained in that package.

## 10. Acceptance Criteria

- [x] A human can understand the validation reasoning model without reading a
      JSON schema.
- [x] An agent can inspect the compact catalog and load only the guidance needed
      for one decision.
- [x] Initial shared guidance explains relevance, evidence, limitations, and
      safe next steps without depending semantically on one programming
      ecosystem.
- [x] Every citation's precise source location supports the adjacent claim at
      its declared scope and directs deeper study without reproducing upstream
      manuals.
- [x] The minimum corpus supports complete bounded Rust and
      JavaScript/TypeScript validation recipes.
- [x] Foundations, patterns, general concerns, scoped topic guides, tools,
      standards, and recipes retain distinct responsibilities.
- [x] Directory placement remains editorial while catalog metadata provides
      cross-facet discovery.
- [x] `docs/validation/knowledge/` is the only authored and tracked
      shared-knowledge tree in the repository.
- [x] No `skills/liknon/references/knowledge/`, generated knowledge
      mirror, or release projection remains in the source tree or Cargo package.
- [x] The distributed skill remains packaged with its router, manifest, and
      current `run`, `triage`, `config`, and `audit` operational references, and
      retrieves shared knowledge only through the CLI.
- [x] The Cargo build embeds the complete canonical knowledge tree through
      deterministic untracked output under `OUT_DIR`.
- [x] `liknon knowledge catalog` exposes compact human and JSON
      discovery without configuration or workspace initialization.
- [x] `liknon knowledge show <document-id>` exposes the exact
      selected Markdown without accepting filesystem paths.
- [x] Knowledge queries are read-only, offline, deterministic, and create no
      `.validation/` directory or other workspace file.
- [x] Missing topic coverage is explicit and never replaced by invented advice
      or empty placeholders.
- [x] Recommendations begin with consequence, invariant, and required evidence,
      not a fashionable tool.
- [x] External standards retain their native versions, levels, and scopes.
- [x] Recipes are valid examples and never implicit global policy.
- [x] The raw catalog and source register satisfy their actual compiled Draft
      2020-12 schemas before typed and semantic validation.
- [x] Every recipe configuration satisfies the actual configuration schema and
      ordinary semantic validation without executing a declared command.
- [x] Routing fixtures, forward-trial scenarios, and the rubric satisfy their
      complete machine-validated contracts.
- [x] Positive routing and forward-trial coverage proves both Rust and
      JavaScript/TypeScript selection without cross-loading unrelated stack
      guidance.
- [x] Validation planning and execution remain independent from knowledge
      retrieval even though the binary embeds and serves the guidance.
- [x] The source package contains the canonical docs and builds a binary whose
      knowledge commands work without repository-relative runtime files.
- [x] Package CI installs that source package and compares catalog and document
      output from the installed binary byte-for-byte with the canonical files
      contained in the package.
- [x] `pnpm.cli-11` identifies one exact reviewed release, tag, or commit.
- [x] The focused release checks, complete verification matrix, MSRV checks,
      package checks, dependency audit, and repository-integrity gate pass after
      the remaining corrections.
- [x] Future initialization is documented as flag-selective materialization of
      essential consumer resources, never an implicit copy of all knowledge.
- [x] Shared knowledge contains no Human Flow, AI-Tool, AI-Engineering, or
      human-only operator instructions.

## 11. Handoff To Phase 2.1

Complete every Phase 2 acceptance criterion before beginning Phase 2.1. The
knowledge tree, catalog, source register, embedding pipeline, read-only CLI,
routing fixtures, and package verification form the accepted operational
baseline for the editorial refoundation.

Phase 2.1 reorganizes that corpus by reader intent, establishes
standards-informed editorial profiles, removes incidental version coupling,
and refounds the current content under one mechanically verified authoring
contract. It preserves stable document IDs and the read-only
`liknon knowledge` interface.

Phase 3 begins only after Phase 2.1 is complete. It retrieves selected shared
knowledge by stable CLI document ID and does not copy the knowledge or
maintainer-only authoring trees into the skill or `.validation/`.
