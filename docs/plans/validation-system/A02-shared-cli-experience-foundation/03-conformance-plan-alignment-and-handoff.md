# Phase A02.3: Conformance, Plan Alignment, And Handoff

> **Pre-B04 evidence:** the command inventory below records the completed A02
> handoff. B04 owns the final four-target command inventory and launcher-only
> namespace aliases.

## Status

`Repository implementation complete; hosted verification pending`. Local
conformance, MSRV, source-package closure, documentation, and plan alignment
are implemented. Protected hosted jobs and public registry readback require a
committed release input and remain external completion evidence.

[Back to Plan A02](README.md)

## Purpose

Prove that the extracted foundation remains product-neutral, verify consistent
behavior across applications and platforms, reconcile every active plan that
consumes the new package, and hand one unambiguous foundation to B03.

## Conformance Matrix

The final gate covers every applicable combination of:

### Executables

- `liknon-validator` and `likv`;
- `liknon-manager` and `likm`;
- the current `liknon-tool` and `likt` structural surface before B03, followed
  by B03's functional surface when implemented; and
- `liknon` and `likx` launcher-owned root surfaces.

### Help And Diagnostics

- root `-h`;
- root `--help`;
- focused command `-h`;
- focused command `--help`;
- nested subcommand help where present;
- unknown command;
- missing required argument;
- invalid option value; and
- visual options before and after the selected command where grammar permits.

### Visual Modes

- default plain/standard;
- explicit standard palette;
- high-contrast;
- protanopia;
- deuteranopia;
- tritanopia;
- achromatopsia;
- plain low-vision;
- colored low-vision;
- `NO_COLOR`; and
- explicit palette overriding `NO_COLOR`.

### Output Modes

- human success;
- human domain failure;
- usage failure;
- internal or integrity failure where the product defines one;
- JSON success where supported; and
- JSON error where supported.

The harness asserts contract properties, not visual preference:

- no ANSI in default/plain or machine output;
- low-vision retains the same semantic content and order;
- ANSI stripping never removes status or hierarchy information;
- direct and compact aliases are equivalent;
- product command grammar and exit codes remain product-owned; and
- shared help contains no product-specific branch hidden in the library.

## Platform And MSRV Evidence

Run the complete shared-crate and adopted-application matrix on the repository's
supported Linux, macOS, and Windows CI targets. Platform-specific assertions
must normalize line endings and executable suffixes without weakening semantic
coverage.

Validate the declared workspace MSRV for:

- `liknon-cli-ui` alone;
- every application depending on it; and
- the resolved crates.io source-package closure.

No extraction may raise the MSRV accidentally through a language feature or
new dependency.

## Package And Dependency Evidence

Before handoff:

1. package `liknon-cli-ui` independently;
2. inspect its file inventory, license, README, Rustdoc inputs, and dependency
   closure;
3. install or test each dependent application from its extracted package
   against a controlled local registry containing the exact supporting
   candidate;
4. prove no application candidate relies on a workspace-only path;
5. prove native archives still contain only their declared executable payload
   and aliases; and
6. record the exact supporting-package dependency in release manifests.

The supporting crate has its own source-package identity but no native archive,
Cloudflare stable pointer, installed command, or runtime file.

## Required Plan Alignment

Reconcile these maintained documents as part of A02 implementation:

### Roadmap And Target

- `docs/plans/validation-system/README.md`: place A02 after A01 and before B03,
  show B01/B02 release-readiness and B03/B04 dependencies, and assign the
  shared runtime experience to A02.
- `docs/plans/validation-system/resumo-estado-alvo.md`: add
  `crates/liknon-cli-ui` and distinguish shared terminal policy from local
  domain presentation.
- `docs/plans/validation-system/avaliacao-estado-atual.md`: update only after
  the code exists; never describe the planned crate as current behavior.

### Product Plans

- B01: make the exact shared package part of the inspected Validation source
  closure while leaving native artifacts unchanged.
- B02: require A02 adoption before B02.5 Manager release readiness and keep Hub
  publication independent from UI releases.
- B03: make completed A02 a prerequisite from B03.0 onward, require Tool to use
  the shared crate, and prohibit a second palette/help implementation.
- B04: use the shared crate for launcher-owned help while preserving delegated
  capability streams and B04-owned launcher semantics.
- B05: state no dependency unless a future concrete terminal consumer justifies
  one; programmatic APIs must remain independent from CLI presentation.
- B06: include the exact `liknon-cli-ui` source candidate and publish/read back
  it before selected dependent crates.io application packages.

### Shared And Integrated Plans

- `cli-distribution-foundation.md`: continue to own release machinery only and
  explicitly defer runtime terminal experience to A02.
- D01: add cross-CLI accessibility, ANSI, help, and product-isolation scenarios
  without transferring crate publication authority.
- public CLI references and application READMEs: link to one canonical
  experience contract and retain product-specific applicability.

Completed A01 documents remain evidence of the earlier structural decision.
They may receive a concise forward handoff note, but must not be rewritten as
if the shared crate existed during A01.

## Publication Ownership

A02 owns the `liknon-cli-ui` supporting source package, its version, package
candidate, crates.io publication workflow, and recurring release evidence.
It owns no capability CLI candidate and no native artifact.

Application plans retain authority over their own packages and may require one
exact compatible shared version. A shared-crate change triggers application
conformance but does not automatically force release of every application.
Conversely, an application release does not require a new shared version when
its accepted dependency is already public and byte-identical.

For the first public release, B06 freezes the exact dependency graph and
coordinates publication in topological order. It publishes and reads back the
exact supporting crate before invoking publication of a dependent application
package, without rebuilding either candidate. It does not take A02 credentials
or recurring release authority.

## Documentation And Changelog

After implementation:

- add a crate README explaining scope, accessibility contract, examples, and
  non-goals;
- add Rustdoc for every public type and function;
- update the repository README's architecture and package inventory;
- update each application README and CLI reference;
- record the new supporting package and migrations in the changelog; and
- avoid historical narration or compatibility instructions for the private
  module layout that was replaced.

## Final Verification

Run at minimum:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --all-targets --locked
cargo test --doc --locked
cargo package -p liknon-cli-ui --locked
distribution/cli/common/scripts/rehearse-source-packages.sh \
  liknon-cli-ui liknon-resource-contracts -- \
  liknon-validator liknon-manager
```

Also run the ignored outcome fixtures because Validation reporting and terminal
presentation are touched:

```sh
cargo test -p liknon-validator --test outcome_fixtures -- --ignored
```

Add Tool and launcher package checks once their manifests become publishable
under B03/B04. Validate MSRV and hosted platform jobs before marking A02
complete.

## Handoff Criteria

- [x] All repository-verifiable A02 global acceptance criteria pass.
- [x] The shared package is independently packageable and has one release
      owner.
- [x] Validator and Manager consume the same implementation without sharing
      domain renderers.
- [x] Tool is structurally bound to the shared foundation before B03.0 begins.
- [x] Launcher-owned help has a defined shared dependency without changing
      dispatch semantics.
- [x] B01, B02, B03, B04, B06, D01, the roadmap, and the target summary agree
      on ownership and dependency order.
- [x] Current-state documentation is updated only after implementation.
- [x] B03 has no unresolved theme, help, ANSI, or low-vision architecture
      decision that duplicates A02.
- [ ] Hosted Linux, macOS, Windows, and protected publication/readback jobs pass
      from the committed release revision.

## Result

B03 receives a tested, publishable CLI experience foundation and needs to
design only Tool's resource-consumption grammar and domain presentation. The
existing products retain their independent behavior while future terminal
improvements have one implementation and one conformance matrix.
