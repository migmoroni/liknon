# Phase B03.1: Establish Shared AI Resources

[Back to the ecosystem AI packages plan](README.md)

## Purpose

Create one canonical shared resource set that every ecosystem package can
assemble unchanged.

## Content Boundary

Include only cross-ecosystem agent resources:

- the skill router and compatibility check;
- AI-Tool `policy`, `config`, `run`, `triage`, and `audit` operations;
- AI-Engineering process routing and immutable safety rules;
- policy ownership and prompt-injection boundaries;
- common prompts, output contracts, and routing metadata;
- instructions for choosing the validation or knowledge application.

Do not include the knowledge corpus, product reference copied from canonical
documentation, or an ecosystem-specific command merely because several current
stacks use it.

## Assembly Contract

- Output is deterministic from tracked shared sources.
- Every byte has one stable resource ID and hash.
- Packages include exact output without rewriting instructions.
- The router loads only the operation needed for the current request.
- Version metadata is stored once in the manifest, not repeated in every
  `SKILL.md`.
- Draft operations are marked honestly and cannot claim complete capability.

## Verification

- Independent package assemblers produce the same shared digest.
- Shared instructions contain no host-language-only assumption.
- Every CLI command in a skill is verified against current canonical help.
- Validation and knowledge routing use separate executables.
- Policy mutation remains prohibited in every operation.

## Exit Criterion

One compact, versioned shared resource set is safe to assemble unchanged into
all ecosystem packages.
