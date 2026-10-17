# Phase A01.2: Separate Authored Sources From Resource Artifacts

[Back to the product and repository separation plan](README.md)

## Purpose

Turn knowledge and AI-facing material into independently selectable,
versioned resources while preserving one canonical authored source for every
file.

## Resource Kinds

The initial resource contract supports:

- `knowledge`;
- `skill`;
- `workflow`;
- `agent-profile`;
- `tool`;
- `prompt`;
- `guide`;
- `template`;
- `collection`.

`knowledge` is the canonical singular kind name. Collections reference
resources; they do not duplicate payloads.

## Knowledge Sources

`docs/validation/knowledge` remains the canonical authored knowledge source.
Its editorial taxonomy, stable document identities, provenance, source links,
readiness state, and local-link checks remain intact.

Publication transforms approved documents into immutable resource payloads
with stable IDs and versions. Documents can be installed individually or
through collections. A complete collection may exist, but no CLI embeds or
requires it.

## Other Official Resources

Official skills, workflows, agent profiles, tool descriptors, prompts, guides, templates, and
their publication metadata have one canonical type-first source under
`resources/<kind>/official/` or an explicitly recorded existing source path.
`official` is the Liknon-controlled namespace for these authored resources; it
does not substitute for source-origin, review, signature, or trust metadata.

`contracts/ai/ENTRYPOINT.md` is deliberately outside this tree. It is the
program-owned root contract loaded before any replaceable resource and cannot be
published, shadowed, or selected as a Hub payload. Resource manifests declare a
compatible bootstrap-contract range instead.

The initial governed AI assistance uses independently selectable official
resources:

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

AI-Tool is the official default operational skill selected by the initial
official workflows. That status supplies provenance and a maintained reference
implementation, not implicit activation or runtime privilege. External
workflows may select a compatible alternative or derivative through the same
dependency contract.

No source contains another resource's instructions. Workflows compose skills,
agent profiles, tools, knowledge, prompts, and optional subworkflows through the
ordinary dependency contract. Installing any dependency by itself never
activates or implies execution of a workflow.

### Generic Workflow Contract

A workflow manifest is the generic orchestration index. It declares stable node,
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

Every publishable resource declares:

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

## Assembly Contract

- Hub artifacts are assembled deterministically from canonical sources.
- An artifact contains only declared files.
- Publication does not modify authored sources.
- Resource updates do not rebuild a CLI.
- Manager installs exact immutable artifacts selected by the resolver.
- Tool returns installed bytes and metadata without reinterpretation.
- No language registry package becomes a second resource distribution channel.
- No resource package contains or overrides the program-owned AI bootstrap
  entry point.

## Verification

- Every authored resource has exactly one source owner.
- Every artifact maps back to source commit, resource version, and exact files.
- Knowledge editorial checks run independently from all CLIs.
- The catalog resolves every stable resource ID exactly once per version.
- Selection of one resource does not install an unrelated language or tool
  collection.
- Official and non-official workflows resolve through the same manifest,
  dependency, compatibility, and policy-declaration contracts.
- A workflow selecting a non-official AI-Tool alternative validates without an
  application branch or bootstrap-contract change.
- A workflow with nonnumeric node IDs and a graph unlike either official
  workflow validates and resolves without application changes.
- A complete knowledge collection and a minimal selection resolve through the
  same contract.
- No assembly executes resource content or mutates a consumer workspace.

## Exit Criterion

Knowledge and AI-facing material are modular resource sources with deterministic
artifacts, explicit metadata, and no embedded-corpus or ecosystem-package
ownership.
