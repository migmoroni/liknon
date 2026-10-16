# Phase A01.3: Compose Applications And Workspace Resources

[Back to the product and repository separation plan](README.md)

## Purpose

Compose the three independent command-line applications over their approved
contracts and establish one safe consumer-owned `.liknon/` root.

## Validation CLI

`liknon` and `likx` expose the same validation application. It owns:

- validation arguments and complete help;
- color and presentation profiles;
- human progress and report rendering;
- stdin, stdout, stderr, signal, and exit adaptation;
- validation-owned schemas;
- explicit provisioning and inspection of validation configuration;
- deterministic validation of the human-owned AI policy and AI artifacts
  required by the existing validation-system contract.

It reads `.liknon/validation/config.json` for execution. It neither contacts
the Hub nor discovers or renders installed resources.

## Manager CLI

`liknon-manager` and `likm` expose the same resource-management
application. It owns:

- local resource-state initialization;
- Hub catalog access and authenticated downloads;
- dependency and collection resolution;
- manifest and lock updates;
- transactional installation, update, removal, and verification;
- explicit materialization to a caller-selected integration destination.

Manager treats every payload as data and never executes installed instructions
or validation commands.

## Tool CLI

`liknon-tool` and `likt` expose the same offline resource-consumption
application. It owns:

- installed-resource catalog and search;
- exact resource lookup and rendering;
- dependency-aware local resolution;
- bounded context assembly in human and machine-readable formats;
- provenance, trust, compatibility, and integrity visibility.

Tool performs no network access and no resource, validation, policy, or
persistence mutation.

## Workspace Boundary

The canonical consumer root is `.liknon/`. Default discovery walks from the
process current directory to the nearest ancestor containing that directory.
An explicit `--workspace` path selects a boundary directly and must pass the
same containment rules.

Commands that require existing state fail when discovery finds no `.liknon/`
root. An explicit initialization operation instead uses the selected
`--workspace`, or the nearest existing root, and falls back to the exact process
current directory only when it is creating a new `.liknon/` root. Initialization
never guesses a Git, package-manager, or language-project root.

Ownership is divided by subtree:

| Path | Owner |
| --- | --- |
| `.liknon/validation/config.json` | Human and Validation CLI initialization contract |
| `.liknon/validation/reports/` | Validation/AI evidence contract |
| `.liknon/resources/manifest.json` | Human intent, updated only by explicit Manager operations |
| `.liknon/resources/lock.json` | Exact Manager resolution |
| `.liknon/resources/<kind>/` | Manager-installed immutable payloads |
| `.liknon/ai/policy.json` | Human operator exclusively |
| `.liknon/ai/persistence/` | AI-Engineering persistence contract |

A command creates only explicitly requested paths. Existing conflicting bytes,
unsafe links, or boundary escapes fail before mutation.

## Integration Boundary

Agent-native materialization, when requested, is separate from installation.
Manager records the exact source resource, destination, bytes, and digest. It
does not silently edit `.agents/`, `.codex/`, editor settings, or global
agent configuration.

Tool can consume a manually populated store when its manifest, lock, payloads,
and hashes satisfy the public contract. Manager is therefore convenient but
not a runtime dependency of Tool.

## Verification

- Both names for each application have equivalent version and command behavior.
- Validation operates with Manager and Tool absent.
- Manager operates without a validation config or Tool executable.
- Tool operates offline against Manager-produced and fixture-produced stores.
- JSON output contains no ANSI.
- Workspace discovery chooses the nearest boundary deterministically.
- Explicit roots, nested workspaces, missing roots, unsafe links, and boundary
  escapes have deterministic tests.
- Installation and materialization are transactional and idempotent for
  identical bytes.

## Exit Criterion

Three independent CLIs interoperate through one versioned workspace contract
without hidden execution, hidden network access, or implicit activation.
