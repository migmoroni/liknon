# Phase A02.2: Existing And Future CLI Adoption

> **Pre-B04 evidence:** alias-specific adoption checks below record the
> implemented input state. B04 retains direct capability executables but
> removes their compact executable aliases.

## Status

`Implemented`. Manager, Tool's structural shell, and launcher-owned surfaces
consume the shared foundation while retaining their product boundaries.

[Back to Plan A02](README.md)

## Purpose

Adopt the shared experience in Manager, establish Tool's mandatory foundation
before B03 behavior is implemented, and prepare the launcher to use the same
terminal policy without deciding B03 or B04 product semantics.

## Manager Adoption

Add `liknon-cli-ui` to `apps/liknon-manager` and replace independent terminal
presentation in these layers:

- visual option declaration and pre-parse resolution;
- Clap help, usage, and parser diagnostics;
- compact and complete help presentation;
- human headings, sections, paths, metadata, states, and diagnostics;
- `NO_COLOR` behavior; and
- generic terminal-write behavior that is already part of the shared API.

Manager maps its own concepts to neutral tokens. Candidate mappings include:

- completed and retained state to success/complete roles;
- planned additions, updates, and removals to explicit action roles whose text
  remains understandable without color;
- blocked conflicts and integrity failures to distinct textual statuses;
- trust, review, origin, versions, paths, and digests to metadata/path roles;
  and
- warnings or unavailable online evidence to warning/blocked roles according
  to Manager's existing domain semantics.

The exact mapping is Manager-owned and covered by Manager tests. It does not
extend the shared token vocabulary with resource-domain names.

### Manager Behavior That Must Not Change

- domain command names, arguments, defaults, and effects; A02 adds only the
  shared global visual options defined by its terminal contract;
- workspace discovery and mutation boundaries;
- Hub, trust, resolution, lock, transaction, and recovery behavior;
- human output content and ordering, except explicitly approved visual layout;
- JSON schema, field names, values, and deterministic ordering;
- error classes and exit codes; and
- stdout/stderr contracts.

`--color` and `--presentation` style Manager help at any command depth. They
style command execution only for human output. Their combination with JSON
execution is rejected as a usage error. JSON success and error output remain
ANSI-free.

Manager tests must cover both `liknon-manager` and `likm`.

## Tool Foundation Before B03

A02 does not implement Tool's command families or resource-consumption logic.
It changes the structural shell so B03 cannot start from an independent visual
system:

- add the exact workspace dependency on `liknon-cli-ui`;
- construct the Tool Clap command through the shared adapter;
- use the shared visual options, help rules, and terminal policy;
- retain a minimal shell until B03.0 approves the final grammar; and
- add a structural test proving the Tool package does not define its own
  palette or presentation types.

B03.0 remains responsible for deciding which Tool operations support styled
human execution and for defining Tool's human and JSON contracts. B03.1 and
B03.2 implement Tool-local resource roles and renderers by mapping them to the
shared neutral tokens.

The B03 implementation must not copy source from Validation, introduce another
palette table, or wrap the shared crate in a compatibility facade.

## Launcher Foundation

Add the shared crate to the launcher package only for launcher-owned terminal
surfaces:

- root `-h` and `--help`;
- root parser diagnostics;
- root `--version` presentation where B04.0 permits human styling; and
- generic branding and low-vision behavior.

The launcher does not style or reinterpret delegated capability output. It
forwards capability arguments and streams according to B04. A visual option
that appears after a capability namespace belongs to that selected capability;
the launcher must not consume it accidentally.

A02 must not decide executable discovery, compatibility, dispatch, timeout,
component version semantics, or aggregate-suite behavior. Those remain B04.0
decisions.

## Shared Experience Without Shared Domain Output

After this phase, identical visual options do not imply identical reports.
Each product keeps the structure appropriate to its work:

| Product | Shared experience | Local presentation |
| --- | --- | --- |
| Validation | Profiles, help, terminal policy, neutral styles | Execution tree, checks, gates, progress, report result |
| Manager | Profiles, help, terminal policy, neutral styles | Catalog, resolution plans, trust, integrity, mutations |
| Tool | Profiles, help, terminal policy, neutral styles | Resource catalog, retrieval, context, provenance |
| Launcher | Profiles, root help, parser diagnostics | Namespace summary, delegation guidance, component versions |

Do not force all applications into one report layout or one error enum merely
to increase code reuse.

## Documentation

Update each adopted application's README and CLI reference to state:

- default output is plain and ANSI-free;
- accepted palette and presentation names come from the shared contract;
- `NO_COLOR` and explicit-option precedence;
- where visual options affect command execution versus requested help;
- machine output is ANSI-free and incompatible with execution styling; and
- low-vision changes presentation, not domain semantics.

Link to one canonical CLI experience reference instead of copying the full
palette and accessibility explanation into every product document. Product
documents still describe their own command applicability and output formats.

## Verification

For every adopted binary and compact alias, verify:

- root and focused compact help;
- root and focused complete help;
- default/plain output;
- every explicit palette;
- standard and low-vision presentation;
- `NO_COLOR` with and without an explicit override;
- malformed visual options;
- visual options applied to help for commands whose execution does not use
  them;
- human execution where available;
- machine output where available; and
- ANSI-stripped semantic completeness.

Tests should reuse a conformance harness where inputs and guarantees are truly
identical. Product-specific assertions remain in product test suites.

## Exit Criteria

- [x] Manager uses the shared implementation and has no local palette,
      presentation, or generic Clap-style copy.
- [x] Manager's human output is accessible and its JSON contract is unchanged
      and ANSI-free.
- [x] Tool's structural shell depends on the shared crate before B03 begins.
- [x] B03 can add domain renderers without defining terminal policy.
- [x] Launcher-owned help uses the shared foundation without consuming or
      restyling delegated capability output.
- [x] Every direct and compact command has equivalent visual behavior.
- [x] No application invokes another capability or gains a new domain
      dependency as a result of UI sharing.
