# Phase A01.2: Separate Knowledge And AI Resources

[Back to the product and repository separation plan](README.md)

## Purpose

Separate the complete read-only validation-knowledge corpus from skills,
prompts, agent guidance, and ecosystem integration resources. Give each
resource exactly one authored owner and one distribution contract.

## Knowledge Corpus

`docs/validation/knowledge` remains the canonical authored corpus. It includes
shared concepts and all supported language, tool, concern, pattern, reference,
and recipe categories. The taxonomy supports progressive discovery without
splitting the corpus into separately installed units.

The corpus contract preserves:

- stable document IDs;
- one deterministic catalog;
- exact canonical Markdown bytes;
- source provenance and editorial readiness;
- local link and fragment integrity;
- offline read-only retrieval.

`liknon-knowledge` embeds or packages this complete approved corpus from one
source. `liknon-cli` never reads it at build time or runtime.

## AI Resource Sources

Create canonical resources only where current or approved content exists:

- `ai-resources/shared` for the router, common safety boundaries, common
  prompts, and cross-ecosystem agent guidance;
- `ai-resources/rust`;
- `ai-resources/javascript-typescript`;
- `ai-resources/ruby`;
- `ai-resources/python`;
- `ai-resources/dotnet`.

These resources teach an agent how to work with Liknon and its ecosystem
context. They may route the agent to `liknon-knowledge` for validation
knowledge, but they do not copy that corpus.

## Ecosystem Assembly Contract

- Every registry package assembles the exact shared resources plus only its
  ecosystem-specific additions.
- Shared resource IDs and bytes remain identical across packages.
- Package adapters expose their own resources through explicit APIs or
  caller-directed materialization.
- Installation alone executes nothing and changes no workspace or global agent
  configuration.
- Packages contain neither `liknon` nor `liknon-knowledge` binaries.
- Packages contain no hidden downloader, mutable remote payload, or knowledge
  corpus.

## Verification

- Every current knowledge, skill, prompt, and agent instruction has exactly
  one authored owner or a recorded repository-only owner.
- Knowledge editorial checks run without the validation CLI.
- The complete catalog resolves every stable document ID exactly once.
- The knowledge application can be built without `liknon-core`.
- The validation application can be built without knowledge or AI resources.
- Each ecosystem assembly contains the exact shared resources and no content
  belonging exclusively to another ecosystem.
- No resource path or command depends on implicit package discovery or
  workspace activation metadata.

## Exit Criterion

The complete knowledge corpus and ecosystem AI resources have separate
canonical sources, neither CLI owns the other's content, and no implicit
package-discovery or activation state is required.
