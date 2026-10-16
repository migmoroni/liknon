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

Official skills, workflows, agent profiles, prompts, guides, templates, and
their publication metadata have one canonical source under
`resources/official` or an explicitly recorded existing source path.

The initial governed AI flows use two separate official resources:

- `resources/official/skills/liknon-ai-tool/` owns the AI-Tool router,
  immutable policy-boundary projection, and bounded operation instructions;
- `resources/official/workflows/liknon-ai-engineering/` owns the four
  AI-Engineering process instructions and declares a compatible dependency on
  the AI-Tool skill.

Neither source contains the other's instructions. The workflow composes the
skill through the ordinary resource dependency contract, and installing the
skill by itself never activates or implies installation of the workflow.

Every publishable resource declares:

- stable namespace, name, kind, and version;
- title, summary, authorship, license, and source origin;
- compatibility requirements;
- dependencies and optional relationships;
- payload media types, lengths, and SHA-256 digests;
- declared capabilities and executable-content indicators;
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

## Verification

- Every authored resource has exactly one source owner.
- Every artifact maps back to source commit, resource version, and exact files.
- Knowledge editorial checks run independently from all CLIs.
- The catalog resolves every stable resource ID exactly once per version.
- Selection of one resource does not install an unrelated language or tool
  collection.
- A complete knowledge collection and a minimal selection resolve through the
  same contract.
- No assembly executes resource content or mutates a consumer workspace.

## Exit Criterion

Knowledge and AI-facing material are modular resource sources with deterministic
artifacts, explicit metadata, and no embedded-corpus or ecosystem-package
ownership.
