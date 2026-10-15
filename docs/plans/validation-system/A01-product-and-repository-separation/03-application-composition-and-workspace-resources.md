# Phase A01.3: Compose Applications And Workspace Resources

[Back to the product and repository separation plan](README.md)

## Purpose

Compose the two independent command-line applications over their approved
owners and preserve explicit, safe workspace resource creation.

## Validation CLI

`liknon` and `liknon-cli` expose the same validation application. It owns:

- arguments and complete help;
- color and presentation profiles;
- human progress and report rendering;
- stdin, stdout, and stderr routing;
- Ctrl+C and process-exit adaptation;
- validation-owned schemas;
- generic `.validation/` bootstrap;
- explicit provisioning of validation configuration and policy candidates.

It delegates deterministic configuration, planning, execution, and structured
outcomes to `liknon-core`. It has no `knowledge` command and does not locate,
download, activate, or render the knowledge corpus.

## Knowledge CLI

`liknon-knowledge` and `liknon-k` expose the same read-only knowledge
application. It owns:

- catalog discovery in human and machine-readable forms;
- exact lookup by stable document ID;
- canonical Markdown output;
- knowledge version and provenance inspection;
- knowledge-specific help and diagnostics.

It does not read `.validation/config.json`, execute checks, initialize a
workspace, install AI resources, or invoke the validation CLI.

## Ecosystem Package Interaction

AI resources may instruct a human or agent to invoke either application.
Resource materialization into a workspace, when B03 defines it, is always an
explicit package operation with a caller-selected destination, full preflight,
path containment, atomic writes, and conflict reporting.

Neither CLI recursively searches Cargo caches, `node_modules`, gem paths,
Python environments, NuGet caches, or agent configuration directories.

## Verification

- Both aliases for each application produce equivalent version, help, and
  command behavior.
- Validation commands use core rather than duplicate domain logic.
- Knowledge commands resolve only the complete canonical corpus.
- A validation-only installation works without `liknon-knowledge`.
- A knowledge-only installation works without `liknon`.
- JSON output contains no ANSI and remains stable under terminal themes.
- Package installation produces no workspace or global-state mutation.
- Explicit materialization, when implemented, is path-safe, transactional, and
  idempotent for identical bytes.

## Exit Criterion

Two independent CLIs expose validation and knowledge without hidden
cross-execution, while ecosystem resources remain explicit non-CLI products.
