# Phase A01.3: Compose The CLI And Workspace Resources

[Back to the product and repository separation plan](README.md)

## Purpose

Rebuild the command-line application over the extracted core and generic pack
contract, keeping the executable lean while preserving explicit access to
active pack resources.

## CLI Composition

The CLI owns:

- argument parsing and complete help;
- color and presentation profiles;
- human progress and report rendering;
- stdin, stdout, and stderr routing;
- Ctrl+C and process-exit adaptation;
- schema output for CLI-owned public contracts;
- generic `.validation/` bootstrap;
- generic pack inspection, activation, and explicit resource materialization.

The CLI delegates configuration, planning, execution, and structured outcomes
to core. It delegates pack parsing, validation, composition, and file inventory
to `liknon-pack`.

## Pack Selection

Define one deterministic, package-manager-neutral way to supply active pack
roots or sealed pack artifacts to the CLI. The contract must support:

- explicit caller-supplied paths;
- workspace-owned activation metadata when persistence is requested;
- stable precedence and conflict diagnostics;
- paths independent from the process current directory after resolution;
- no recursive search through Cargo, `node_modules`, gem, Python, or NuGet
  installations;
- no network access or dependency installation.

Exact command names and activation-file shape are approved here before code is
changed. They become public CLI and schema contracts.

## Knowledge And Skill Behavior

`knowledge catalog` and `knowledge show` operate over the composed active pack
catalog. They remain read-only and return stable resource identities. With no
active knowledge pack, they return a documented empty state or selection error
and never use compiled-in content.

Initialization creates only explicitly requested base resources owned by the
CLI. Pack-provided knowledge, skills, recipes, or templates are materialized
only through an explicit pack operation. Materialization preflights the full
set, writes atomically, refuses conflicting destinations, and reports exact
created, unchanged, skipped, or conflicting resources.

Installing an ecosystem package does not activate it or mutate a workspace.

## Verification

- All CLI commands use core rather than duplicate domain logic.
- CLI-only tests retain help, theme, report-layout, stream, signal, and status
  coverage.
- Knowledge lookup uses only explicitly active test packs.
- A no-pack fixture proves the binary has no hidden fallback.
- Pack activation and materialization are path-safe, deterministic,
  transactional, and idempotent for identical bytes.
- Package-manager directories are never scanned implicitly.
- JSON output contains no ANSI and remains stable under terminal themes.

## Exit Criterion

One lean CLI exposes the validation core and generic pack capabilities without
embedding ecosystem content, downloading dependencies, or mutating a workspace
as a side effect of package installation.
