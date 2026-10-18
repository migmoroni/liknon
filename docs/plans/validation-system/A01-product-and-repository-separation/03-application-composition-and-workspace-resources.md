# Phase A01.3: Compose Applications And Establish The Workspace Boundary

[Back to the product and repository separation plan](README.md)

## Purpose

Complete the Validation CLI composition, establish isolated package boundaries
for all three command-line applications, and establish one safe consumer-owned
`.liknon/` root. Complete Manager and Tool command behavior belongs to B02 and
B03 after A01.

## Validation CLI

`liknon` and `likx` expose the same validation application. It owns:

- validation arguments and complete help;
- color and presentation profiles;
- human progress and report rendering;
- stdin, stdout, stderr, signal, and exit adaptation;
- projection of the config and report schemas owned by `liknon-core`;
- explicit provisioning and inspection of validation configuration.

The Validation CLI package is also the reserved future owner of thin command
adapters over deterministic AI-policy and generic workflow-persistence
contracts. Phase 3 implements policy initialization, validation, and evaluation;
Phase 4 implements persistence adapters over caller-supplied locked workflow
contracts. A01 does not implement or accept either feature.

It reads `.liknon/validation/config.json` for execution. It neither contacts
the Hub nor browses, selects, or renders installed resources. A01 implements no
workflow-artifact verification. When Phase 4 later adds that capability, it may
read only the exact caller-supplied workflow manifest and must verify it against
the resource lock before using a declared schema or destination template.

### Atomic Validation-Path Replacement

A01.3 replaces the old root in one complete change:

1. workspace discovery targets the nearest ancestor containing `.liknon/`;
2. validation config selection targets only
   `.liknon/validation/config.json`;
3. `init` preflights and creates only the new directory and file path;
4. core, CLI, integration, symlink, containment, and initialization tests use
   the new root;
5. outcome fixtures and their explicit commands use the new path;
6. examples, help, `CONFIG_DESIGN.md`, public validation references, README
   snippets, and comments use the new path; and
7. repository search proves that no executable source, active fixture, test, or
   maintained public documentation still names `.validation/`.

There is no fallback discovery, dual read, dual write, migration command, or
compatibility alias for `.validation/`.

## Manager Package Boundary

`liknon-manager` and `likm` are two binary names for the same isolated
application package. The package is reserved to own:

- local resource-state initialization;
- Hub catalog access and authenticated downloads;
- dependency and collection resolution;
- manifest and lock updates;
- transactional installation, update, removal, and verification;
- explicit materialization to a caller-selected integration destination.

These capabilities are implemented and accepted in B02. A01 proves only that
the package contains no validation engine or Tool implementation and that both
names dispatch the same package. Manager will treat every payload as data and
never execute installed instructions or validation commands.

During A01 the package exposes only identity, version, and help. It does not
create `.liknon/resources/`, advertise final subcommands, or imply that a
Manager operation is available before B02.0 approves its grammar.

## Tool Package Boundary

`liknon-tool` and `likt` are two binary names for the same isolated application
package. The package is reserved to own:

- exact read-only projection of the program-owned AI bootstrap entry point;
- installed-resource catalog and search;
- exact resource lookup and rendering;
- dependency-aware local resolution;
- bounded context assembly in human and machine-readable formats;
- provenance, trust, compatibility, and integrity visibility.

These capabilities are implemented and accepted in B03. Tool performs no
network access and no resource, validation, policy, or persistence mutation.

B03.0 defines the bootstrap output envelope and B03.1 implements the projection.
It must work without a workspace or resource store and remain separate from
installed content. A01 provides only the canonical bytes, metadata, package
boundary, and shared primitives required by that later implementation.

During A01 the package exposes only identity, version, and help. It does not
implement `bootstrap`, inspect `.liknon/resources/`, or advertise the final
Tool grammar before B03.0.

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

The completed system's ownership is divided by subtree as follows. A01 creates
only `.liknon/validation/config.json` when the human explicitly requests
validation initialization; later owning phases create the other entries:

| Path | Owner |
| --- | --- |
| `.liknon/validation/config.json` | Human and Validation CLI initialization contract |
| `.liknon/validation/reports/` | Validation/AI evidence contract |
| `.liknon/resources/manifest.json` | Human intent, updated only by explicit Manager operations |
| `.liknon/resources/lock.json` | Exact Manager resolution |
| `.liknon/resources/<kind-directory>/` | Manager-installed immutable payloads |
| `.liknon/ai/policy.json` | Human operator exclusively |
| `.liknon/ai/persistence/` | Declared workflow persistence contracts |

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

At A01 exit, `liknon-workspace` owns only workspace discovery, canonical path
and containment primitives, plus generic AI-bootstrap identity and
compatibility types. B02 adds resource identity, manifest, lock, and provenance
contracts. Phase 3 adds policy contracts. Phase 4 adds generic workflow and
artifact-envelope contracts. Application adapters may later validate or expose
those types, but they never branch on `liknon-action`,
`liknon-validation-engineering`, `liknon-ai-tool`, an official namespace, a
process number, or an agent-profile identity.

The bootstrap contract contains no AI-Tool or official-workflow identity;
official workflows declare AI-Tool through their ordinary locked dependencies.

## Verification

- Both names for each application have equivalent version and command behavior.
- Manager and Tool equivalence in A01 covers only identity, version, help,
  stream, and exit behavior; B02 and B03 own feature-command equivalence.
- Validation operates with Manager and Tool absent.
- Manager and Tool package shells build without a validation config or another
  Liknon executable.
- B02/B03 conformance fixtures are structurally available without being treated
  as completed application behavior.
- JSON output contains no ANSI.
- Workspace discovery chooses the nearest boundary deterministically.
- Explicit roots, nested workspaces, missing roots, unsafe links, and boundary
  escapes have deterministic tests.
- Neutral resource fixtures include different workflow node IDs, transitions,
  profiles, and operational-skill references for later B02/B03 tests.
- No application source branches on those fixture identities.
- A repository-wide stale-path check permits `.validation/` only in the A01
  replacement ledger or deliberately retained historical Git data, never in a
  maintained runtime, test, fixture, example, or public reference.

## Exit Criterion

Validation remains complete, three application packages have isolated
responsibilities, and one workspace boundary is ready for B02 and B03 public
contracts without hidden executable dependencies or premature feature claims.
