# Phase A01.2: Separate Authored Sources From Resource Artifacts

[Back to the product and repository separation plan](README.md)

## Purpose

Complete the separation of canonical knowledge and maintainer-owned editorial
material from application source, then prepare neutral deterministic assembly
inputs while preserving one editable owner for every file. A01.1 has already
removed runtime embedding, the knowledge command, and the superseded skill that
depended on it. B02.0 later codifies the publishable resource manifest, kind
mapping, and Hub contracts.

## Reserved Resource Kinds

The target architecture reserves the following kinds for B02.0 to codify in a
public contract:

- `knowledge`;
- `skill`;
- `workflow`;
- `agent-profile`;
- `tool`;
- `prompt`;
- `guide`;
- `template`;
- `collection`.

`knowledge` is the intended singular kind name. Collections are intended to
reference resources rather than duplicate payloads. These terms organize
authored sources and neutral fixtures in A01; they are not yet a public schema,
wire contract, directory mapping, or accepted Manager/Tool input.

## Knowledge Sources

`docs/validation/knowledge` remains the canonical authored knowledge source.
Its editorial taxonomy, stable document identities, provenance, source links,
readiness state, and local-link checks remain intact.

Publication transforms approved documents into immutable resource payloads
with stable IDs and versions. Documents can be installed individually or
through collections. A complete collection may exist, but no CLI embeds or
requires it.

Move catalog, source, and editorial-profile schemas from the former root
`schemas/` grab bag to `docs/validation/authoring/schemas/`. Move routing,
forward-trial, and rubric schemas to `evals/schemas/`. Update canonical
metadata, fixtures, the maintainer package, evaluation harnesses, and
documentation to reference their one owning location. These schemas remain
repository-only until B02.0 deliberately adopts any part of them into a public
resource contract.

## Other AI Resource Sources

Any existing skill, workflow, agent profile, tool descriptor, prompt, guide,
template, collection, or source metadata that survives the A01.0 ledger moves
to one canonical type-first source under a recorded path. At the current
baseline, the only skill tree is the superseded `skills/liknon` integration;
A01.1 removes it because its contract depends on the deleted runtime knowledge
command. It is not renamed or promoted into the future official AI-Tool.

Kind IDs and their exact `resources/<kind-directory>/<namespace>/` mapping are
codified in B02.0. `official` is reserved for content that actually exists and
has completed its owning phase; it does not substitute for source origin,
review, signature, or trust metadata. A01.2 therefore creates no empty
`resources/` directories.

`crates/liknon-workspace/contracts/ai/ENTRYPOINT.md` and its adjacent
`manifest.json` are deliberately outside this future resource tree. They form
the package-contained program root contract loaded before any replaceable
resource and cannot be published, shadowed, or selected as a Hub payload.
Future actionable resource manifests declare a compatible bootstrap contract
range instead.

The target governed AI assistance uses independently selectable official
resources, but A01 does not author these later products:

- `resources/skills/official/liknon-ai-tool/` owns the bounded operational
  instructions used by action-oriented assistance;
- `resources/workflows/official/liknon-action/` owns the action workflow and
  routes its declared nodes to the AI-Tool skill;
- `resources/workflows/official/liknon-validation-engineering/` owns the
  validation-engineering graph and its process instructions;
- `resources/agent-profiles/official/` owns the independently versioned roles
  selected by validation-engineering workflow nodes;
- `resources/tools/official/` owns descriptors that bind stable tool operations
  and capabilities to separately installed applications without embedding
  executables or credentials.

Phase 3 authors the final official action workflow, AI-Tool resource, and policy
boundary guide. Phase 4 authors the validation-engineering root workflow and
profiles. Phases 5 and 6 complete its node behavior. A01 must not create those
later resources, mark drafts publishable, or use their absence as a failed gate.

AI-Tool is the official default operational skill selected by the initial
official workflows. That status supplies provenance and a maintained reference
implementation, not implicit activation or runtime privilege. External
workflows may select a compatible alternative or derivative through the same
dependency contract.

No source contains another resource's instructions. Workflows compose skills,
agent profiles, tools, knowledge, prompts, and optional subworkflows through the
ordinary dependency contract. Installing any dependency by itself never
activates or implies execution of a workflow.

### Generic Workflow Requirements For B02.0

A future workflow manifest is the generic orchestration index. B02.0 turns the
following requirements into a public schema. It declares stable node,
action, and transition IDs, entry and terminal nodes, preconditions,
continuation-eligible edges, referenced
resources, action inputs and outputs, potential capabilities and effects, and
produced artifact types. The platform
does not prescribe process names, numbering, graph shape, or required personas.

An agent-profile resource describes a role, posture, scope, and behavioral
constraints. A workflow may select a different profile for every node. The
profile neither owns the graph nor grants itself authority. The consuming AI
host binds a selected profile to an available model; Manager and Tool do not
instantiate an agent.

A tool resource is a declarative descriptor for an application capability or
operation. It does not install the application, contain credentials, execute an
action, or grant command authority. A workflow action references tool and skill
operations by stable identity; the AI host evaluates policy and performs the
eligible action through the separately installed application.

The B02.0 public contract must require every publishable resource to declare:

- stable namespace, name, kind, and version;
- title, summary, authorship, license, and source origin;
- compatibility requirements;
- compatible AI bootstrap and decision-policy contract ranges for actionable
  workflows and governed operational skills;
- dependencies and optional relationships;
- payload media types, lengths, and SHA-256 digests;
- declared capabilities and executable-content indicators;
- for workflows, nodes, actions, transitions, entry points, continuation
  eligibility, referenced profile/skill/tool IDs, and declared artifact
  outputs;
- trust and review metadata assigned by the Hub publication process.

Resource metadata never grants an instruction authority over a human, agent,
policy, or host.

## Assembly Boundary

- Neutral fixture artifacts are assembled deterministically from canonical
  sources to prove the source boundary without defining the final Hub format.
- Their fixture-only inventory and digest representation is explicitly marked
  non-public and may be replaced by the B02.0 contract without migration or
  compatibility support.
- An artifact contains only declared files.
- Publication does not modify authored sources.
- Resource updates do not rebuild a CLI.
- B02 later makes Manager install exact immutable artifacts selected by its
  resolver; B03 later makes Tool return installed bytes and metadata without
  reinterpretation.
- No language registry package becomes a second resource distribution channel.
- No resource package contains or overrides the program-owned AI bootstrap
  entry point.

## Verification

- Every authored resource has exactly one source owner.
- Every neutral assembly fixture maps back to source commit, fixture identity,
  and exact files.
- Knowledge editorial checks run independently from all CLIs.
- Stable source identities have no duplicates within the repository.
- Selection of one resource does not install an unrelated language or tool
  collection.
- Neutral workflow fixtures include unrelated graph shapes, nonnumeric nodes,
  and replaceable operational-skill references for B02.0/B03.0 design, without
  representing either fixture as an official product workflow.
- A complete knowledge source selection and a minimal selection assemble from
  the same canonical corpus without embedding it in a CLI.
- No assembly executes resource content or mutates a consumer workspace.
- Every editorial check runs through `liknon-knowledge-maintenance`; no check
  imports an application package or depends on an embedded generated catalog.
- Package inventories for all three CLIs and both support crates contain no
  knowledge document, editorial schema, neutral resource fixture, or stale
  skill source.

## Exit Criterion

Knowledge and maintainer material have one modular authored source,
deterministic neutral assembly evidence, and no embedded-corpus,
ecosystem-package, stale-skill, or premature official-resource ownership.
B02.0 can now approve public artifacts and schemas without moving source
ownership again.
