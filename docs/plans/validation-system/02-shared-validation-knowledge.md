# Phase 2: Shared Validation Knowledge

## 1. Objective

Create one source-backed validation knowledge base that helps humans and agents
decide what evidence a project needs, why it matters, and which tools can
produce it. The same authored Markdown serves both audiences; the distributed
skill receives an exact generated projection for self-contained use.

This phase supplies reasoning and reference material. It does not move policy
selection into the runtime.

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
    ├── concerns/
    └── recipes/
```

The root index distinguishes shared knowledge from reference material and the
three flows without duplicating their contracts. The knowledge index explains
how to move from validation concepts to a project strategy. Foundation
documents provide the reasoning model. Language, framework, technology, tool,
and concern documents remain semantically separate. Recipes compose them for
concrete project shapes.

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

Every substantive recommendation links to an applicable source ID and, when
available, a section, requirement, control, or success criterion. Local
interpretation is marked as synthesis and is not presented as quoted or
normative source material.

The initial register covers:

- ISO/IEC 25010:2023 for product quality characteristics;
- ISTQB CTFL 4.0.1 and explicitly versioned applicable parts of the
  ISO/IEC/IEEE 29119 family for testing vocabulary and organization;
- final NIST SP 800-218, SSDF 1.1, for secure development practices;
- OWASP ASVS 5.0.0 for application-security verification;
- OWASP SAMM 2.0 for software-assurance maturity;
- WCAG 2.2 for web accessibility conformance;
- SLSA 1.2 for source and build supply-chain guarantees;
- a pinned OpenSSF OSPS Baseline release;
- NIST SP 800-162 for the attribute-based model referenced by the
  shared AI decision policy;
- NIST AI RMF for human-AI oversight responsibilities;
- OASIS XACML 3.0 for restrictive policy-combination semantics;
- Parasuraman, Sheridan, and Wickens' published types-and-levels-of-automation
  model;
- official Open Policy Agent documentation for decision-point,
  enforcement-point, and decision-log separation;
- OWASP AI Agent Security guidance for action classification,
  human-in-the-loop controls, reversibility, and audit;
- official documentation for each documented language, framework, technology,
  and tool.

Draft publications remain labeled as drafts and do not silently replace final
editions. Restricted standards are cited and summarized rather than copied.

## 5. Foundation Model

### 5.1 Validation Model

Describe independent evidence dimensions, including:

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

No single tool or universal checklist establishes complete correctness.

### 5.2 Test Types

For each test type, define purpose, boundary, ownership, cost, limitations, and
common overlaps. Where communities use one term differently, document the
competing definitions rather than selecting one silently.

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

Each tool guide states:

- purpose and covered defect class;
- executable and package identity;
- scope and relative cost;
- mutation behavior;
- prerequisites and external services;
- useful command structure and flags;
- output and exit behavior;
- limitations and evidence overlap;
- official sources.

Recipes provide complete, schema-valid examples for Rust crates, Rust
workspaces, Svelte applications, Tauri applications with Svelte frontends,
Rails applications, and structured-data pipelines. A recipe records its
scenario, assumptions, risks, selected evidence, source basis, execution
contexts, and known gaps. It is an example, not a mandatory project policy.

## 7. Skill Projection

Keep `skills/workspace-validator/SKILL.md` as a small router. Extend the skill
manifest with:

- maintenance metadata pointing to `docs/validation/knowledge/` in the source
  tree;
- one active runtime root below `references/knowledge/`;
- explicit catalog routes for the index, source register, foundations,
  categories, and recipes.

This phase adds only the shared-knowledge branch. Phase 3 implements the shared
AI policy boundary and explicit AI-Tool routing, and Phase 4 adds AI-Engineering
routing. The knowledge branch must not infer or select either flow.

A deterministic repository release task copies the entire canonical tree to
the distributed skill while preserving paths and bytes. The generated copy is
never edited independently.

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
- exact language, framework, technology, tool, concern, and recipe guides for
  the workspace under analysis.

AI-Tool and AI-Engineering references remain distinct from shared knowledge.
Loading knowledge never selects a flow, starts a process, writes an artifact,
or grants authority. Later phases may route to the generated knowledge
projection without modifying it.

## 9. Tests

- Verify canonical and generated trees have identical paths and bytes.
- Verify the generated tree digest is deterministic across supported hosts.
- Validate every manifest route and internal documentation link.
- Validate every recipe without executing its checks.
- Verify every substantive recommendation has traceability or an explicit
  local-synthesis marker.
- Verify source versions and review dates are present.
- Inspect source package contents and the assembled skill bundle.

## 10. Acceptance Criteria

- [ ] A human can learn the validation model without reading a JSON schema.
- [ ] An agent can load only the guidance required for one decision.
- [ ] `docs/validation/knowledge/` is the sole authored shared-knowledge source.
- [ ] The distributed knowledge is an exact generated projection.
- [ ] Recommendations begin with risk and required evidence, not a fashionable
      tool.
- [ ] External standards retain their native versions, levels, and scopes.
- [ ] Recipes are valid examples and never implicit global policy.
- [ ] The runtime remains independent from guidance and tool selection.
- [ ] Shared knowledge contains no Human Flow, AI-Tool, AI-Engineering, or
      human-only operator instructions.

## 11. Handoff To Phase 3

Phase 3 uses this knowledge base in the policy-governed AI-Tool Flow while
keeping each operational skill bounded to one request. It does not turn
educational material into executable policy or AI-Engineering context.
