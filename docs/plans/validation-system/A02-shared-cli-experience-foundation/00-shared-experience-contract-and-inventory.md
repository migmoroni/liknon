# Phase A02.0: Shared Experience Contract And Inventory

## Status

`Implemented`. The inventory is reflected in the shared crate, local product
adapters, conformance tests, and the canonical CLI experience reference.

[Back to Plan A02](README.md)

## Purpose

Distinguish reusable terminal policy from Validation-specific presentation,
identify the concrete Manager and future Tool consumers, and approve one
behavioral contract that can be implemented without speculative abstraction.

## Evidence Inventory

The implementation must inspect and classify at least these current surfaces:

| Current surface | Evidence to preserve or separate |
| --- | --- |
| `apps/liknon-validator/src/theme/` | Palette profiles, presentation profiles, style primitives, Clap adaptation, Validation roles, and Validation metrics |
| `apps/liknon-validator/src/cli/arguments.rs` | Visual arguments, accepted values, pre-parse help resolution, and format incompatibilities |
| `apps/liknon-validator/src/cli/help/` | Compact/full help distinction, root composition, command index, separators, logo, and command-specific text |
| `apps/liknon-validator/src/reporting/` | Domain renderers, hierarchy, progress, result trees, diagnostics, and local theme mappings |
| `apps/liknon-validator/tests/cli.rs` | Existing cross-palette, low-vision, `NO_COLOR`, malformed-option, root-help, and focused-help behavior |
| `apps/liknon-manager/src/cli/` | Existing command grammar, global format option, and default Clap behavior |
| `apps/liknon-manager/src/output.rs` | Human report content, JSON report contract, error streams, and domain states |
| `apps/liknon-tool/` | Structural shell that must consume the shared foundation before B03 adds behavior |
| `apps/liknon-cli/` | Structural launcher whose local help remains B04-owned |

The inventory records each item as one of:

- shared code moved to `liknon-cli-ui`;
- product adapter rewritten to consume the shared API;
- product-specific code retained in place;
- product-specific test retained and adapted; or
- obsolete duplication removed only after its consumer uses the shared owner.

No current behavior is removed merely because the new crate does not need it.

## Approved Visual Profiles

The shared contract preserves the current profile axes independently.

### Palette

- `plain`;
- `standard`;
- `high-contrast`;
- `protanopia`;
- `deuteranopia`;
- `tritanopia`; and
- `achromatopsia`.

### Presentation

- `standard`; and
- `low-vision`.

Palette selects color and typographic style. Presentation selects density,
spacing, and emphasis. A palette must not change presentation metrics, and a
presentation must not silently select a hue palette.

## Terminal And ANSI Contract

1. Default human output uses `plain` plus `standard` and contains no ANSI.
2. Bare `--color` selects the standard palette.
3. Named palettes require `--color=<PALETTE>`.
4. Named presentation modes require `--presentation=<MODE>`.
5. An explicit command-line palette overrides `NO_COLOR`.
6. Without an explicit palette, `NO_COLOR` prevents hue output.
7. Low-vision presentation may use non-hue typographic emphasis while still
   obeying `NO_COLOR`; documentation must distinguish ANSI color from other
   terminal attributes.
8. Human status and hierarchy never rely on hue alone.
9. Machine-readable output contains no ANSI or terminal-dependent layout.
10. A command rejects visual execution options when its selected output format
    is machine-readable rather than silently discarding them.

TTY detection and redirected-output behavior must be explicit and tested. The
implementation must not infer that terminal color is safe merely because a
dependency's default changed.

## Help Contract

- `-h` produces the compact help for the selected root or command.
- `--help` produces the complete help for the selected root or command.
- Visual options may style requested help at any command depth even when they
  do not apply to that command's execution.
- Focused command help advertises `--color` and `--presentation` as execution
  options only when that command's human execution output supports them.
- Root help explains the global visual system once.
- Focused help owns only the selected command's grammar and explanations.
- Complete root help preserves the command index before options and may render
  detailed command blocks through a generic shared shell.
- Main commands and nested subcommands remain visually distinguishable.
- Low-vision help increases separation without changing content, order, or
  command semantics.
- Root branding is shared data and rendering policy; product names, summaries,
  command text, and examples remain application inputs.

The shared implementation must receive command metadata through typed inputs.
It must not contain branches for `validate`, `check`, `install`, `show`, or any
other product command name.

## Neutral Style Contract

The shared crate exposes product-neutral semantic tokens, not application
roles. The exact public names are finalized in A02.1, but the model must cover:

- heading and section emphasis;
- borders and separators;
- metadata, paths, values, and subdued text;
- primary, secondary, tertiary, and quaternary accents;
- success, failure, blocked, warning, skipped, active, complete, and pending
  states; and
- diagnostic labels and diagnostic content.

Applications map their domain concepts to those tokens. For example, Validation
may map group, suite, check, and gate roles to neutral accents, while Manager
may map trust, planned change, retained state, and integrity roles. The shared
crate does not know either vocabulary.

## Explicitly Product-Owned Behavior

The following remain outside `liknon-cli-ui`:

- Validation report trees, execution hierarchy, progress bars, spinner state,
  signal handling, and process termination;
- Manager catalog, plan, trust, integrity, transaction, and recovery content;
- Tool resource projection, context, provenance, and integrity content;
- launcher discovery, dispatch, compatibility, and component version output;
- every JSON schema and serialized envelope;
- every capability error taxonomy and exit-code mapping;
- command grammar, examples, domain labels, and domain ordering; and
- interactive confirmation or mutation policy.

`indicatif`, `reqwest`, resource contracts, process APIs, and application
packages therefore do not become dependencies of `liknon-cli-ui`.

## Behavior Baseline

Before extraction, add or identify regression coverage for:

- all palette and presentation combinations;
- plain/default ANSI absence;
- explicit palette behavior with and without `NO_COLOR`;
- low-vision spacing and removal of dim styling;
- root and focused compact/full help;
- help styled by visual options at unsupported execution commands;
- visual option rejection for JSON execution;
- invalid palette and presentation syntax;
- hierarchy and status comprehension after ANSI stripping;
- Validator human report and execution progress output; and
- Manager human and JSON output before it gains shared styling.

Tests should assert stable semantic properties and exact contract text where it
is public. They should not freeze incidental terminal widths or dependency
formatting that the project does not intend to own.

## Deliverables

- An approved source-to-owner inventory for every current visual/help surface.
- One documented terminal, ANSI, help, and neutral-style contract.
- Regression tests sufficient to distinguish extraction from behavior change.
- A public API sketch containing only demonstrated Validator and Manager needs.
- An explicit list of product behavior that must not enter the shared crate.

## Exit Criteria

- [x] Every current Validator theme/help component has one target owner.
- [x] Manager requirements are represented without importing Manager domain
      types into the shared design.
- [x] B03 requirements can be fulfilled without adding a second theme system.
- [x] The contract preserves current Validation behavior intentionally.
- [x] No proposed shared API exists only for an imagined future consumer.
- [x] The extraction proceeded in a buildable sequence with no duplicate
      long-term source of truth.
