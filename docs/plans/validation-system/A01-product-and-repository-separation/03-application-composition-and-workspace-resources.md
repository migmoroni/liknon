# Phase A01.3: Compose Applications And Establish The Workspace Boundary

## Status

`Provisional`. Blocked by the approved A01.0 ledger and completed A01.1-A01.2
handoffs.

[Back to Plan A01](README.md)

## Purpose

Complete the approved application-package boundaries and replace the current
validation workspace path with the approved `.liknon/validation/` location.

## Validation CLI

The standalone Validation CLI package provides the approved
`liknon-validate` and `likv` entry points over one implementation. It owns
command parsing, terminal adaptation, human presentation, JSON stream
selection, signals, exit codes, and explicit validation initialization.

It reads only:

```text
.liknon/validation/config.json
```

It does not contact the Hub, browse installed resources, host an AI model, or
gain policy/persistence behavior merely because those files share `.liknon/`.

## Launcher And Aggregate Package Boundary

Create `apps/liknon-cli` as the aggregate Cargo package selected by the A01.0
ledger. It owns launcher targets `liknon` and `likx` plus capability targets
`liknon-validate`, `likv`, `liknon-manager`, `likm`, `liknon-tool`, and `likt`.

The launcher targets contain only namespace selection and process dispatch.
They do not import capability APIs to execute work. Aggregate capability
targets reuse the same shared implementations as the standalone application
packages; no validation, Manager, or Tool behavior is copied. A01 establishes
these package and dependency boundaries, while B04.0 decides discovery,
compatibility, process forwarding, installation collisions, native suites, and
release behavior.

Do not create a Cargo application package named `liknon`. That package identity
is reserved for future B05 programmatic integration.

## Manager And Tool Structural Boundaries

Create the Manager and Tool application packages and approved binary identities
only if the A01.0 ledger selected package-shell creation as part of the
structural cutover.

Before B02.0/B03.0, a shell may expose only identity, version, concise help, and
equivalent dispatch for its long and compact names. It must not:

- advertise candidate feature commands as implemented;
- create resource, policy, or persistence state;
- contact a Hub;
- inspect or execute validation;
- embed authored resource payloads.

If the ledger defers package creation until B02/B03, A01 records the destination
and dependency boundary without manufacturing empty applications.

## Atomic Validation-Path Replacement

Replace `.validation/config.json` with `.liknon/validation/config.json` in one
behavioral unit:

1. workspace discovery and explicit-root handling use the approved boundary;
2. initialization preflights and creates only the requested validation path;
3. configuration discovery and validation use only the new location;
4. unit, integration, symlink, containment, and initialization tests use it;
5. fixtures and examples use it;
6. help, README, config design, and maintained references use it;
7. repository search proves no active implementation or maintained public
   documentation still presents `.validation/` as current.

No fallback discovery, dual read, dual write, migration utility, or deprecated
alias is introduced.

## Shared Workspace Boundary

Shared primitives may define:

- nearest `.liknon/` ancestor discovery;
- explicit workspace selection;
- canonical child paths;
- containment and unsafe-symlink rejection;
- no-overwrite and atomic-placement operations required by implemented
  validation initialization.

A command creates only explicitly requested paths. A01 does not create
`.liknon/resources/`, `.liknon/ai/policy.json`, reports, or persistence
directories merely to match the target tree.

## Ownership Deferred To Later Plans

| Concern | Owning gate |
| --- | --- |
| Resource manifest, lock, payload layout, and Manager mutation | B02.0 |
| Tool discovery, retrieval, and machine output | B03.0 |
| Bootstrap, policy, authority, and persistence contracts | C01 |
| Official action workflow | C02 |
| Official validation-engineering workflow | C03 |

No application branches on an official workflow, skill, process, node, action,
profile, or namespace identity.

## Verification

- Validation works when Manager and Tool are absent.
- Approved application names dispatch one implementation per product.
- Aggregate and standalone capability targets resolve the same shared
  implementations without circular application dependencies.
- Launcher-only targets contain no validation, network, resource mutation,
  resource interpretation, or AI behavior.
- Structural shells, when created, expose no unapproved feature grammar.
- Workspace discovery, explicit roots, nested roots, unsafe links, and boundary
  escapes have deterministic tests.
- JSON output remains ANSI-free and human themes retain current behavior.
- The old validation path appears only in historical records or the reviewed
  replacement ledger.
- No command creates future resource or AI state.
- Full workspace and relevant outcome-fixture gates pass.

## Exit Criterion

Validation uses the sole target workspace path, product packages have the
approved structural boundaries, and B02/B03/B04/C01 can define their behavior
without A01 having preassigned it.
