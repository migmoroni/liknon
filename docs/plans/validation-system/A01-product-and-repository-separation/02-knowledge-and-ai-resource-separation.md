# Phase A01.2: Separate Authored Knowledge And AI Material

## Status

`Provisional`. Blocked by A01.0 and the green A01.1 workspace.

[Back to Plan A01](README.md)

## Purpose

Remove authored knowledge and AI-support content from application runtime and
package ownership while preserving their canonical source, editorial checks,
history, and handoff to later resource plans.

## Current Inputs

Classify and preserve the actual current surfaces recorded by A01.0:

- `docs/validation/knowledge/`;
- `docs/validation/authoring/`;
- knowledge-related schemas;
- `tools/knowledge/`;
- `build.rs` knowledge generation;
- runtime knowledge modules and CLI routes;
- `skills/liknon/` and its tests;
- knowledge fixtures and evaluations;
- package include rules and documentation that distribute or describe them.

## Required Separation

1. Keep one editable source for every authored document.
2. Keep editorial and release checks as repository-maintenance behavior, not a
   user-facing application dependency.
3. Remove generated corpus embedding and knowledge command behavior from the
   Validation CLI after their source and tests have a recorded destination.
4. Remove knowledge and AI-resource payloads from application package
   inventories.
5. Preserve the useful source of the current skill as a C02 handoff; do not
   silently rename it into the future official skill or delete it before the
   handoff records what is retained and what is superseded.
6. Mark any preserved skill whose commands no longer exist as non-active design
   input rather than distributable current guidance.
7. Keep evaluation-only contracts separate from public resource contracts.
8. Do not create final Hub manifests, locks, trust classes, resource schemas,
   or immutable payloads before B02.0.
9. Do not create empty `resources/` directories or placeholder official
   resources before C02/C03.

## Canonical Source Decision

The A01.0 ledger must name the canonical source for:

- authored knowledge;
- editorial schemas and profiles;
- evaluation fixtures and rubrics;
- repository-only maintenance code;
- the current skill material awaiting C02;
- public explanatory documentation.

Generated catalogs or eventual Hub artifacts are outputs from those sources,
never parallel editable copies.

## Resource-Agnostic Handoff

Record future resource candidates by responsibility only:

- knowledge documents;
- skills;
- workflows;
- agent profiles;
- tool descriptors;
- prompts;
- guides;
- templates;
- collections.

The exact identity, directory mapping, metadata, dependency, trust, and artifact
contracts remain B02.0/C01 decisions. The current repository content does not
prove that every candidate type already exists.

## Verification

- Validation CLI and supporting runtime package inventories contain no authored
  knowledge corpus or AI skill payload.
- Every current authored source has one editable owner.
- Editorial checks run without importing a CLI package or relying on generated
  embedded bytes.
- Minimal and complete knowledge selections can be assembled by repository
  tests without defining the final Hub format.
- No assembly executes content or mutates a consumer workspace.
- Current skill source has an explicit C02 handoff record.
- Repository searches find no maintained claim that knowledge remains a
  Validation CLI command.
- Formatting, tests, doctests, editorial checks, and package inventories pass.

## Exit Criterion

Applications no longer own authored knowledge or AI resources. Canonical
sources and maintenance evidence remain intact, and B02/C01/C02/C03 can define
their contracts without moving source ownership again.
