# Phase 2: Shared Validation Knowledge

## 1. Objective

Establish one curated, source-backed validation knowledge foundation that helps
humans and agents decide what evidence a project needs, why it matters, and
where to continue learning. The same authored Markdown serves both audiences;
the distributed skill receives an exact generated projection of the content
that currently exists.

The knowledge base is intentionally incremental. Its documents provide concise,
directly useful orientation and link to authoritative primary documentation for
depth. They do not reproduce complete standards, upstream manuals, or an offline
encyclopedia. This phase establishes the structure, editorial contract, source
policy, and initial foundations; later work adds topic guides only when a real
validation need justifies them.

This phase supplies reasoning and reference material. It does not move policy
selection into the runtime or claim comprehensive coverage of validation.

## 2. Dependency

Phase 1 is complete. Guidance describes stable CLI and configuration behavior
rather than compensating for unfinished runtime contracts.

## 3. Canonical Layout

Create the canonical authored knowledge tree:

```text
docs/validation/
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
    └── recipes/
        ├── README.md
        └── <scenario>.md
```

The root index distinguishes shared knowledge from reference material and the
three flows without duplicating their contracts. The knowledge index explains
how to move from validation concepts to a project strategy. Foundation
documents provide the reasoning model. Language, framework, technology, tool,
and concern documents remain semantically separate. Recipes compose them for
concrete project shapes.

Phase 2 requires the root indexes, `SOURCES.md`, the four foundation documents,
and one index for each expandable category. Topic and recipe documents are added
only when their content supports an actual use case. Do not create empty topic
files, speculative catalogs, or placeholder pages merely to populate the tree.
The absence of a topic means that the knowledge base does not yet provide local
guidance for it; it never authorizes an agent to invent that guidance.

Everything below `knowledge/` is audience-neutral. It contains no process
activation, persistence behavior, decision authority, flow-specific control,
or human-only conversational guidance. Human Flow documentation, AI-Tool
instructions, AI-Engineering process contracts, and human-only operator guides
are authored in their dedicated locations in later phases.

## 4. Source And Traceability Contract

Create `knowledge/SOURCES.md` as a versioned source register. Each source
records:

- stable source ID;
- authority and exact title;
- edition, release, or revision;
- primary URI;
- supported scope;
- normative, framework, official-documentation, or local-synthesis status;
- last review date.

The register is shared evidence infrastructure for the knowledge base and for
source-backed flow contracts. Registering a source used by AI-Engineering does
not place that flow's behavioral rules inside `knowledge/`.

Every substantive local recommendation links to an applicable source ID and,
when available, a section, requirement, control, or success criterion. Each
topic guide also links directly to the best primary documentation for further
study. Local interpretation is marked as synthesis and is not presented as
quoted or normative source material.

The initial register contains only sources cited by the initial foundation.
Later phases and topic additions register their sources before relying on them.
Do not pre-populate entries for subjects that have no authored guidance. Prefer
official specifications, standards bodies, maintainers' documentation, and
primary research over secondary summaries. A source entry is a navigational and
traceability aid, not a requirement to summarize the complete source locally.

Draft publications remain labeled as drafts and do not silently replace final
editions. Restricted standards are cited and summarized rather than copied.

## 5. Foundation Model

### 5.1 Validation Model

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

For each dimension, explain what question it helps answer, name its principal
limitations, and point to the most relevant primary sources. Do not attempt to
catalog every technique or tool. No single tool or universal checklist
establishes complete correctness.

### 5.2 Test Types

Establish practical distinctions among the core test types by summarizing their
purpose, boundary, ownership, relative cost, limitations, and common overlaps.
Keep each explanation short and link to authoritative material for detailed
methodology. Where a terminology difference materially changes a validation
decision, identify the competing definitions rather than selecting one silently.

### 5.3 Validation Strategy

Teach this decision process:

1. identify the product, change, or operation;
2. identify affected users, assets, data, platforms, and obligations;
3. assess relevant likelihood and impact;
4. identify applicable quality characteristics and domain standards;
5. select required evidence dimensions and test types;
6. select tools after the evidence is understood;
7. decide where and when each validation runs;
8. record gaps, uncertainty, and accepted residual risk.

Do not invent a universal project-wide assurance ladder. Preserve official
levels such as WCAG conformance, ASVS verification, SLSA tracks, SAMM maturity,
and OSPS maturity within their native version and scope.

## 6. Tool Guides And Recipes

Define this concise template for each tool guide that is actually added:

- purpose and covered defect class;
- executable and package identity;
- scope and relative cost;
- mutation behavior;
- prerequisites and external services;
- useful command structure and flags;
- output and exit behavior;
- limitations and evidence overlap;
- official sources.

Tool guides explain enough to decide whether and how to evaluate a tool. They do
not mirror complete command references, configuration manuals, or upstream
tutorials. Exhaustive options remain in the linked official documentation.

Define a recipe template that records scenario, assumptions, risks, selected
evidence, source basis, execution contexts, and known gaps. An authored recipe
contains a complete schema-valid example for its declared scenario, but Phase 2
does not require a recipe for every supported language or stack. Recipes are
added as concrete reusable scenarios emerge and never become mandatory project
policy.

## 7. Skill Projection

Keep `skills/workspace-validator/SKILL.md` as a small router. Extend the skill
manifest with:

- maintenance metadata pointing to `docs/validation/knowledge/` in the source
  tree;
- one active runtime root below `references/knowledge/`;
- explicit catalog routes for the index, source register, foundations, category
  indexes, and only the topic or recipe documents that currently exist.

This phase adds only the shared-knowledge branch. Phase 3 implements the shared
AI policy boundary and explicit AI-Tool routing, and Phase 4 adds AI-Engineering
routing. The knowledge branch must not infer or select either flow.

A deterministic repository release task copies the current canonical tree to
the distributed skill while preserving paths and bytes. Adding a topic later
extends the same projection without requiring another knowledge architecture.
The generated copy is never edited independently.

Tests reject:

- missing, extra, stale, or modified generated files;
- unsafe or traversing manifest paths;
- routes outside the active knowledge root;
- package contents that omit routed knowledge;
- a version declared independently from the skill manifest.

## 8. Progressive Disclosure

The skill loads only the references relevant to the current decision:

- the validation model for evidence questions;
- test types for boundary questions;
- validation strategy for context and risk selection;
- source policy for authority questions;
- exact available language, framework, technology, tool, concern, and recipe
  guides relevant to the workspace under analysis.

When no local topic guide exists, the skill reports that absence and uses the
foundation plus registered primary sources. It does not fabricate a missing
guide or load unrelated material as a substitute.

AI-Tool and AI-Engineering references remain distinct from shared knowledge.
Loading knowledge never selects a flow, starts a process, writes an artifact,
or grants authority. Later phases may route to the generated knowledge
projection without modifying it.

## 9. Tests

- Verify canonical and generated trees have identical paths and bytes.
- Verify the generated tree digest is deterministic across supported hosts.
- Validate every manifest route and internal documentation link.
- Validate every authored recipe without executing its checks.
- Verify every substantive recommendation has traceability or an explicit
  local-synthesis marker.
- Verify every registered source has its required version and review metadata.
- Verify category indexes route only to documents that currently exist.
- Verify the initial tree contains no empty topic or recipe placeholders.
- Inspect source package contents and the assembled skill bundle.

## 10. Acceptance Criteria

- [ ] A human can learn the validation model without reading a JSON schema.
- [ ] An agent can load only the guidance required for one decision.
- [ ] Initial documents provide concise decision support and primary-source
      paths without attempting to reproduce upstream documentation.
- [ ] `docs/validation/knowledge/` is the sole authored shared-knowledge source.
- [ ] The distributed knowledge is an exact generated projection.
- [ ] New topic guides and recipes can be added independently without changing
      the knowledge architecture or runtime contracts.
- [ ] Missing topic coverage is explicit and never replaced by invented advice
      or empty placeholders.
- [ ] Recommendations begin with risk and required evidence, not a fashionable
      tool.
- [ ] External standards retain their native versions, levels, and scopes.
- [ ] Recipes are valid examples and never implicit global policy.
- [ ] The runtime remains independent from guidance and tool selection.
- [ ] Shared knowledge contains no Human Flow, AI-Tool, AI-Engineering, or
      human-only operator instructions.

## 11. Handoff To Phase 3

Phase 3 uses the available knowledge foundation in the policy-governed AI-Tool
Flow while keeping each operational skill bounded to one request. It registers
the source entries required by its own normative policy contract, but it does not
turn educational material into executable policy or AI-Engineering context.
