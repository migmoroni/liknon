# Phase A02.1: CLI UI Crate And Validator Extraction

## Status

`Planned`. Blocked by the approved A02.0 inventory and behavior baseline.

[Back to Plan A02](README.md)

## Purpose

Create the publishable `liknon-cli-ui` crate and make Validation its first
consumer without changing observable Validation behavior.

## Package Contract

Add `crates/liknon-cli-ui` to the workspace with:

- package name `liknon-cli-ui`;
- the workspace edition, MSRV, authorship, license, and repository;
- an initial version aligned with the repository's current pre-release package
  set at implementation time;
- crates.io publication metadata, README, and complete Rustdoc;
- no binary targets; and
- only focused terminal dependencies demonstrated by the implementation.

The expected direct dependencies are Clap and `console`. Adding another direct
dependency requires a concrete shared requirement and an MSRV/package-closure
review. In particular, the crate must not depend on `indicatif`,
`liknon-resource-contracts`, any application crate, network clients, schema
libraries, or process-control libraries.

Add one exact workspace dependency entry for `liknon-cli-ui`. Application
manifests use that entry rather than repeating version policy.

## Public API Shape

The crate should expose a small set of cohesive concepts:

- `PaletteProfile` and `PresentationProfile`;
- parsed visual-option values and deterministic profile resolution;
- a theme or style resolver based on neutral semantic tokens;
- ANSI-independent style specifications;
- a Clap adapter for help, usage, and parser diagnostics;
- presentation transforms used by compact and complete help;
- generic root-help composition configured by application-provided metadata;
- branding data or a branding hook for the canonical ASCII mark; and
- terminal policy helpers needed to enforce default/plain, explicit color,
  `NO_COLOR`, and machine-output rules.

Public API names should describe terminal semantics rather than the first
application. No public item may include `Validator`, `Manager`, `Tool`,
`Group`, `Suite`, `Check`, `Gate`, or a product command name.

Avoid a universal application runner. Parsing domain commands, dispatching
operations, serializing reports, and choosing exit codes remain application
responsibilities.

## Style Architecture

Refactor the current role-indexed palette so that palettes resolve neutral
semantic tokens. Product adapters then map local roles to those tokens.

The shared layer owns:

- terminal colors and typographic attributes;
- palette-specific accessibility invariants;
- low-vision presentation modifiers;
- generic spacing values used by help and ordinary human sections;
- style conversion for Clap and `console`; and
- deterministic composition with no global mutable theme.

Validation retains local values when they describe validation rather than a
general terminal surface, including progress width, execution gap, report-tree
layout, and any mapping needed for group/suite/check/gate distinctions.

## Help Architecture

Move only product-neutral mechanics from the current Validation help system:

- palette/presentation-aware Clap styles;
- recursive low-vision option layout;
- compact/full help selection support;
- command-index extraction and insertion;
- root command and subcommand separator rendering;
- generic usage annotation;
- help-option deduplication rules; and
- optional canonical branding placement.

The API accepts a built Clap command plus typed rendering configuration. The
caller supplies:

- product/root name and summary;
- command tree and text;
- which commands support visual options during execution;
- product-specific long descriptions; and
- any product-owned examples.

The shared crate never identifies execution-capable commands by hard-coded
string.

Move the ASCII branding source from the Validation package to the shared
crate's packaged assets when all root help consumers use that same canonical
asset. Keep the repository SVG in its existing brand-asset ownership. Cargo
package inspection must prove the ASCII source required by `include_str!` is
present in the published supporting package.

## Validator Migration

Add the shared dependency to `apps/liknon-validator` and replace the local
generic implementation in buildable steps:

1. Consume shared profiles and style specifications while retaining the local
   Validation role enum and renderers.
2. Introduce a local mapping from Validation roles to neutral shared tokens.
3. Move generic Clap and help composition to shared APIs while supplying the
   existing Validation command metadata.
4. Keep Validation-only metrics, execution presentation, progress integration,
   report hierarchy, diagnostic content, and command applicability local.
5. Move or remove the old generic modules only after every current consumer
   compiles against the shared owner.
6. Remove duplicate profile parsing, palette tables, generic help transforms,
   and branding sources after parity tests pass.

Do not leave reexports or compatibility wrappers for the former private module
layout. It was not a public contract and the repository has not released the
product.

## Validation Behavior Preservation

The migration must preserve:

- accepted palette and presentation values;
- visual-option syntax and validation;
- `NO_COLOR` precedence;
- default ANSI-free output;
- root and focused `-h`/`--help` content and hierarchy;
- standard and low-vision spacing;
- Validator logo behavior;
- human execution progress and result rendering;
- JSON bytes and schema;
- command grammar;
- stdout/stderr assignment;
- interruption behavior; and
- exit codes.

Where exact byte preservation is impractical because a generic internal API
changes incidental whitespace, the implementation must first determine whether
that whitespace is part of the documented public help contract. Deliberate
changes require explicit plan approval rather than being hidden in extraction.

## Crate Verification

Add crate-level tests for:

- every palette/presentation combination;
- color-avoidance invariants for each accessibility palette;
- low-vision removal of dim styling;
- ANSI stripping preserving all text;
- default/plain and machine-output policy;
- `NO_COLOR` and explicit override resolution;
- compact and complete help transforms;
- main-command versus nested-subcommand separators;
- product-neutral help configuration with arbitrary command names; and
- absence of process, network, resource, and application dependencies.

Run Validator integration tests through both `liknon-validator` and `likv`
after extraction. Package and inspect both crates independently.

## Source-Package And Version Ownership

`liknon-cli-ui` owns its own supporting-crate package version. Validation owns
its application version. Equal pre-release versions do not imply permanently
coupled release trains.

Before any crates.io Validation candidate is accepted:

- the exact shared crate candidate is packaged and inspected;
- its dependency closure and MSRV are verified;
- a controlled local-registry rehearsal proves the application candidate
  resolves that exact package rather than a workspace path; and
- B06 records and enforces crates.io publication plus public readback of the
  supporting package before publishing the dependent application package.

Native Validation archives still contain only the final executable payload and
its declared aliases. They do not distribute the supporting crate as a second
runtime file.

## Exit Criteria

- [ ] `liknon-cli-ui` builds, tests, documents, and packages independently.
- [ ] Its public API and dependency graph contain no application or resource
      domain.
- [ ] Validation uses the crate for every shared profile, style, and help
      primitive.
- [ ] Validation-specific roles and renderers remain local.
- [ ] No duplicate generic theme or help implementation remains in Validation.
- [ ] Existing Validator unit, CLI, fixture, doctest, and package tests pass.
- [ ] ANSI-free, low-vision, help, JSON, stream, signal, and exit behavior are
      preserved.
- [ ] Both Validation binary targets remain equivalent.
