# Phase A01.4: Prove Conformance And Reconcile Documentation

## Status

`Complete`. Workspace tests, doctests, strict Rustdoc, Clippy, MSRV and Windows
cross-checks, outcome fixtures, package inventories, local installations,
editorial checks, dependency policy, stale-reference searches, and maintained
documentation verify the A01 exit.

[Back to Plan A01](README.md)

## Purpose

Prove that the structural separation preserves current validation behavior,
enforces the approved ownership ledger, and leaves documentation describing the
actual A01 exit rather than later planned features.

## Conformance Matrix

Verify independently:

- configuration, planning, execution, cancellation, timeout, repository
  evidence, and structured report semantics;
- Validation CLI arguments, help, streams, progress, themes, signals, aliases,
  initialization, and exit statuses;
- package and dependency direction selected by A01.0;
- application isolation and absence of undeclared payloads;
- schema ownership and generation;
- repository-only knowledge maintenance and editorial checks;
- canonical authored-source ownership;
- `.liknon/validation/` discovery, containment, and initialization;
- package inventories and local installation for products actually created by
  A01;
- absence of application branches for official resource identities.

Existing outcome fixtures compare structured validation meaning before and
after extraction. Structural fixtures may prepare later interoperability tests,
but they do not prove Manager, Tool, Hub, policy, persistence, or workflow
behavior.

## Documentation Reconciliation

Update maintained documentation to distinguish:

- behavior implemented at A01 exit;
- product identities and boundaries approved but not yet feature-complete;
- B01 distribution work;
- B02/B03 decision gates;
- B04 launcher and aggregate-suite work;
- C01 governance decisions;
- C02/C03 official workflow work;
- B05 future APIs.

Do not document candidate Manager/Tool commands as available. Do not describe a
future bootstrap, policy, persistence, Hub, resource, or workflow contract as
implemented. Do not retain the old validation path as an alternative.

## Required Gates

- formatting check;
- Clippy with warnings denied;
- all-target workspace tests;
- workspace doctests;
- ignored outcome fixtures;
- MSRV validation;
- package and dependency-boundary inspection;
- local installation smoke tests for applications actually present;
- editorial and authored-source checks;
- repository searches for stale package, embedded-content, workspace-path, and
  ownership claims;
- `git diff --check`.

## Exit Criterion

The complete A01 ledger is satisfied, all preserved validation behavior passes,
every package and authored source has one owner, documentation matches the
implemented boundary, and later plans can proceed without correcting an A01
ownership contradiction.
