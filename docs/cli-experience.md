# CLI Experience Contract

Liknon command-line applications share one accessible terminal presentation
implemented by the `liknon-cli-ui` crate. This contract applies to
application-owned help, parser diagnostics, and supported human output. It does
not merge command grammars, domain renderers, machine-readable schemas, errors,
or exit codes.

## Defaults

- Human output is plain and ANSI-free unless a visual option is explicit.
- TTY detection never enables styling automatically; terminal and redirected
  output follow the same explicit-option policy.
- `--color` selects the standard semantic palette.
- `--color=<palette>` selects `standard`, `high-contrast`, `protanopia`,
  `deuteranopia`, `tritanopia`, or `achromatopsia`.
- `--presentation=standard` uses compact spacing and regular emphasis.
- `--presentation=low-vision` expands spacing, removes dim styling, and may use
  ANSI typographic emphasis without adding hue.
- An explicit palette overrides `NO_COLOR`. Without an explicit palette,
  `NO_COLOR` never enables color.
- Status, hierarchy, trust, integrity, and errors remain readable after ANSI
  styling is removed.

Visual options always style the requested help. They style command execution
only when that application documents support for the selected operation.
Machine-readable output rejects explicit visual options and never contains
ANSI or presentation-dependent whitespace.

## Help

`-h` prints a compact command-oriented summary. `--help` prints complete
details. Both forms accept the same visual controls and retain the same command
semantics.

Complete root help contains the canonical Liknon ASCII mark, a compact command
index, the root options, and detailed command blocks with generated usage.
Top-level commands use full-width separators; nested commands use half-width
separators. Low-vision help adds explicit spacing between related items and
sections without changing their content or order.

Focused help remains application-owned. It documents visual options when those
options also affect that command's human execution, but it does not repeat the
root explanation of palettes and presentation modes.

## Application Ownership

`liknon-cli-ui` owns:

- palette and presentation values;
- neutral semantic style roles;
- Clap help, usage, and diagnostic styling;
- generic complete-root help composition;
- the canonical ASCII mark;
- visual-option parsing and machine-output compatibility policy.

Each application owns:

- commands, arguments, descriptions, examples, and aliases;
- which operations support styled human execution;
- domain roles, hierarchy, progress, and exact output ordering;
- machine-readable contracts and serialization;
- errors, streams, and exit codes.

Direct capability invocation and delegation through the launcher must produce
equivalent terminal behavior. Short names such as `v`, `m`, and `t` are
launcher namespaces, not additional executables. Launcher-owned visual options
apply only before a capability namespace; options after that namespace belong
to the delegated capability.

## Library Scope

`liknon-cli-ui` is a supporting Rust library with no binary, process execution,
network access, workspace mutation, resource model, or application command
name. Applications map their local domain vocabulary to its neutral roles and
retain all domain rendering locally.
