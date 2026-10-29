# Plan A02: Shared CLI Experience Foundation

## Status

`Design direction approved; implementation planned`. This plan must complete
before B03 implementation begins and before B01, B02, B03, or B04 produces a
final crates.io candidate that depends on the shared CLI surface.

The [validation-system roadmap](../README.md) owns cross-track order. This plan
owns the product-neutral terminal experience and the supporting
`liknon-cli-ui` crate. It does not own any capability grammar, domain renderer,
machine-readable schema, exit-code taxonomy, or product publication authority.

## Dependencies

- [A01](../A01-product-and-repository-separation/README.md) provides the Cargo
  workspace and independent application boundaries.
- The implemented Validation CLI provides the mature accessible theme, help,
  and terminal behavior that this plan will separate without changing.
- The implemented Manager provides the second concrete consumer required to
  prove that the boundary is shared rather than speculative.
- [B03](../B03-tool-cli/README.md) and
  [B04](../B04-unified-launcher-and-cli-suite/README.md) consume the completed
  foundation; they do not define it independently.

A02 does not depend on a public Hub, published native artifacts, B03 command
grammar, official workflows, or programmatic language bindings.

## Objective

Create one focused, publishable Rust crate for the terminal-facing experience
shared by Liknon applications, migrate existing applications to it without
changing their domain contracts, and make every future CLI consume that same
implementation from its first functional phase.

The foundation must make these properties consistent:

- palette and presentation selection;
- accessible, non-color-only semantic styling;
- `NO_COLOR` and explicit command-line precedence;
- compact and complete help behavior;
- low-vision help and human-output presentation;
- Clap help, usage, and parser-diagnostic styling;
- generic branded help composition;
- ANSI-free machine output; and
- terminal-oriented conformance tests.

Code reuse and behavioral consistency are related but distinct. Reusable
terminal primitives belong to `liknon-cli-ui`. Product-specific behavior stays
inside each application and is checked against one cross-CLI experience
contract.

## Why This Boundary Exists Now

A01 deliberately kept Validation presentation internal until another
implemented product demonstrated a real shared need. That condition now
exists:

- `liknon-validator` contains a mature accessible theme and custom help system;
- `liknon-manager` has a complete command and human/JSON output surface but
  currently uses an independent default terminal presentation;
- B03 already requires human, JSON, colored, plain, standard, low-vision, and
  `NO_COLOR` behavior; and
- B04 requires a coherent launcher help and version surface.

Leaving the implementation inside Validation would force Manager, Tool, and
the launcher either to drift or to copy it. Moving the complete Validation
theme unchanged would instead leak `Group`, `Suite`, `Check`, `Gate`, progress,
and execution concepts into unrelated products. A02 extracts only the neutral
layer and keeps a local adapter in every product.

## Product Boundary

The target package is:

```text
crates/liknon-cli-ui/
```

`liknon-cli-ui` is a supporting library crate. It:

- owns no binary target;
- executes no capability operation;
- knows no Validation, Manager, Tool, launcher, Hub, resource, workflow, or
  official-resource identity;
- depends on no application package and no `liknon-resource-contracts` type;
- contains no report schema or resource schema;
- performs no network, process, workspace, or resource mutation; and
- may be published to crates.io only as an exact supporting dependency of the
  application packages that consume it.

`liknon-resource-contracts` remains the source of resource-domain contracts.
It must not gain Clap, terminal, color, presentation, or help dependencies.

## Shared And Product-Owned Responsibilities

| Concern | Shared `liknon-cli-ui` ownership | Application ownership |
| --- | --- | --- |
| Palette profiles | Names, parsing, style tokens, accessibility invariants | Selection of tokens for domain concepts |
| Presentation profiles | `standard`, `low-vision`, generic density and emphasis | Domain layout such as validation trees or Manager plans |
| Visual CLI options | Argument definitions, pre-parse resolution, `NO_COLOR` rules | Which operations support styled execution output |
| Clap integration | Help, usage, parser diagnostics, generic spacing | Command grammar, descriptions, examples, and value semantics |
| Root help shell | Generic command index, hierarchy separators, branding hooks | Product title, commands, usage, and product-specific annotations |
| Status styling | Neutral success, failure, blocked, warning, metadata, path, and accent tokens | Meaning of each domain state and its textual label |
| Human output | Safe style application and generic terminal helpers | Exact content, ordering, hierarchy, and domain explanation |
| Machine output | Contract that styling never reaches machine output | JSON schema, envelope, fields, and serialization |
| Process behavior | None | Signals, subprocesses, progress, cancellation, and capability execution |
| Exit behavior | None | Product-specific error classes and stable exit codes |

## Target Repository Shape

```text
crates/
|-- liknon-cli-ui/
|   |-- Cargo.toml
|   |-- README.md
|   |-- assets/
|   |   `-- branding/
|   |       `-- liknon-ascii.txt
|   `-- src/
|       |-- lib.rs
|       |-- arguments.rs
|       |-- clap.rs
|       |-- help.rs
|       |-- palette.rs
|       |-- presentation.rs
|       |-- style.rs
|       `-- terminal.rs
`-- liknon-resource-contracts/

apps/
|-- liknon-validator/  # local validation roles and renderers
|-- liknon-manager/    # local resource-state roles and renderers
|-- liknon-tool/       # local resource-consumption roles and renderers
`-- liknon-cli/        # local launcher composition
```

The final module split may combine files when that reduces ceremony, but it
must preserve the ownership boundaries above. A generic `common` module or
crate that accumulates unrelated helpers is not an acceptable substitute.

## Implementation Units

| Unit | Document | Depends on | Primary result |
| ---: | --- | --- | --- |
| A02.0 | [Shared Experience Contract And Inventory](00-shared-experience-contract-and-inventory.md) | A01 and current Validator/Manager evidence | Frozen shared boundary and behavior baseline |
| A02.1 | [CLI UI Crate And Validator Extraction](01-cli-ui-crate-and-validator-extraction.md) | A02.0 | Publishable product-neutral crate and behavior-preserving Validator adoption |
| A02.2 | [Existing And Future CLI Adoption](02-existing-and-future-cli-adoption.md) | A02.1 | Manager migration plus Tool and launcher consumption boundaries |
| A02.3 | [Conformance, Plan Alignment, And Handoff](03-conformance-plan-alignment-and-handoff.md) | A02.2 | Cross-CLI evidence, updated plans, package closure, and B03 handoff |

Every unit leaves the workspace buildable. A later unit is not a repair phase
for a knowingly broken intermediate state.

## Cross-Track Handoff

- B01 retains Validation distribution authority. Its final source closure must
  include an exact accepted `liknon-cli-ui` source candidate rather than a
  workspace-only path. Prepublication rehearsal uses a controlled local
  registry; B06 publishes and reads back that exact supporting version before
  publishing the dependent application package.
- B02 retains Manager and Hub authority. A02 migrates Manager presentation;
  B02.5 packages and verifies the resulting application.
- B03 depends on completed A02 and must use `liknon-cli-ui` from B03.0 onward.
  It may add Tool-local renderers but may not recreate palettes, help styling,
  low-vision behavior, or terminal policy.
- B04 uses the shared help and theme foundation for launcher-owned surfaces
  while retaining its brief launcher-specific help contract.
- B05 programmatic APIs do not inherit terminal presentation merely because a
  CLI uses this crate.
- B06 publishes and verifies the exact supporting crate before any selected
  crates.io application package that depends on it.
- D01 may test cross-product consistency, but it does not define or publish the
  shared crate.

## Global Acceptance Criteria

- [ ] `liknon-cli-ui` is the only product-neutral implementation of Liknon
      palettes, presentation profiles, Clap styling, and generic help layout.
- [ ] The crate contains no product command name, domain role, report schema,
      resource contract, process execution, network access, or mutation logic.
- [ ] Validation retains its existing command grammar, report semantics,
      visual profiles, help behavior, and exit codes after extraction.
- [ ] Validation-specific roles and progress behavior remain inside
      `liknon-validator`.
- [ ] Manager uses the shared visual options and help system while retaining
      its command, JSON, error, and exit contracts.
- [ ] Tool consumes the shared crate from the first B03 implementation unit.
- [ ] The launcher consumes shared presentation without absorbing capability
      help or execution behavior.
- [ ] Plain/default output and every JSON output are ANSI-free.
- [ ] `NO_COLOR`, explicit palettes, standard presentation, and low-vision
      presentation behave consistently across every adopted CLI.
- [ ] Status, hierarchy, trust, and integrity remain understandable without
      color.
- [ ] Root and focused `-h`/`--help` behavior follows one documented contract
      without duplicating product help content.
- [ ] Every crates.io application candidate resolves `liknon-cli-ui` from a
      controlled registry rehearsal with no unpublished path dependency, and
      B06 records crates.io publication/readback before dependent packages.
- [ ] Formatting, Clippy, tests, doctests, package inspection, and MSRV pass for
      the complete workspace and extracted package.
- [ ] Active B01-B04 and B06 plans refer to this foundation consistently before
      B03 implementation begins.

## Result

Liknon applications retain independent capabilities and domain contracts while
presenting one coherent, accessible terminal experience. Fixes to palette,
help, low-vision, or terminal policy are implemented and tested once, and B03
begins on that established foundation instead of creating a third presentation
system.
