# Liknon Manager

`liknon-manager` is the networked administrative client for the
versioned Liknon resource ecosystem. It discovers and authenticates Hub
metadata, resolves exact dependency graphs, and transactionally maintains the
`.liknon/resources/` store selected by a consumer workspace.

Human output and help use the shared
[CLI experience contract](../../docs/cli-experience.md). Manager retains its
resource-state vocabulary, JSON envelopes, error classes, and exit codes.

In practical terms, it is a package manager for validation and AI-assistance
resources rather than executable programs. A workspace can use it to discover,
install, update, inspect, verify, materialize, and remove resources without
making those resources part of the Liknon binaries themselves.

## What It Manages

The resource ecosystem can contain:

- validation knowledge;
- agent skills and profiles;
- workflows and prompts;
- declarative tools;
- guides and templates;
- collections that group other resources.

Every resource has a stable identity, a semantic version, an origin, declared
dependencies and compatibility requirements, and an exact payload inventory.
Remote metadata and payloads are authenticated through signed catalogs,
trusted signing identities, byte lengths, and SHA-256 digests.

## How It Works

A typical Manager operation follows this sequence:

1. Discover the target workspace and its `.liknon/` boundary.
2. Read and authenticate the selected Hub catalog and trust root.
3. Resolve one deterministic graph of exact resource versions and dependencies.
4. Present the complete mutation plan before changing the workspace.
5. Download and verify only the objects required by that plan.
6. Commit the resource store, manifest, lock, and evidence as one recoverable
   transaction.
7. Retain enough authenticated evidence to verify the installed state offline.

The workspace manifest records the requested root resources. The lock records
the exact resolved graph, origins, compatibility facts, payloads, hashes, and
materializations. This makes resource state reproducible and allows drift or
missing content to be reported explicitly.

Manager never executes installed content, conducts workflows, evaluates AI
policy, runs validation, activates agents, or invokes another Liknon
executable. Its responsibility ends at securely managing resource bytes and
their state. Other Liknon applications may consume those resources under their
own contracts.

## Install From Source

Until the first protected public release is activated, install the complete
suite from the current checkout:

```sh
cargo install --path apps/liknon-cli --locked --force
liknon-manager --version
liknon manager --version
```

Rust 1.87 or newer is required. Reinstall after changing or updating the
checkout because Cargo installs a compiled copy rather than a link.

The `liknon-manager` Rust package is a publishable implementation dependency of
`liknon-cli`; it owns no public binary target.

## Commands

```text
liknon-manager init [--workspace <path>]
liknon-manager search [query] [--kind <kind>] [--namespace <name>] [--trust <class>]
liknon-manager info <resource> [--installed]
liknon-manager install [resource...] [--local <path>...] [--allow-community]
liknon-manager update [resource-id...] [--allow-community]
liknon-manager remove <resource-id>...
liknon-manager list
liknon-manager verify [--offline]
liknon-manager materialize <resource-id> --payload <path> --destination <path>
liknon-manager schema <resource|catalog|manifest|lock|trust-root|signature>
```

- `init` creates an empty, validated resource manifest and lock for a workspace.
- `search` discovers compatible resources exposed by the configured Hub.
- `info` inspects remote metadata or the exact locally installed record.
- `install` resolves and installs requested resources and their dependencies.
- `update` re-resolves only the selected installed resource scope.
- `remove` removes requested roots and transitive resources no longer needed.
- `list` reports desired, locked, installed, missing, and drifted state.
- `verify` authenticates retained evidence and exact installed bytes, optionally
  without network access.
- `materialize` explicitly copies one locked payload to an approved
  workspace-relative destination and records the resulting state.
- `schema` prints a public JSON Schema for one Manager contract.

Use `--workspace <path>` to select an exact consumer boundary. Otherwise,
existing-state commands discover the nearest ancestor containing `.liknon/`.
Mutations accept `--dry-run` and `--yes`; machine output is selected globally
with `--format=json`.

Human operations accept `--color[=<palette>]` and
`--presentation=<standard|low-vision>`. These options also style requested
help. They cannot be combined with JSON or schema output, which remains
ANSI-free and presentation-independent.

`--dry-run` and committed execution use the same resolved plan. A mutation is
journaled across the store, manifest, lock, retained evidence, and any declared
materializations so an interrupted operation can recover or roll back without
silently resolving a different graph.

Networked operations require an authenticated Hub origin and a separately
obtained trust root:

```sh
export LIKNON_HUB_URL=https://resources.example.invalid/
export LIKNON_HUB_TRUST_ROOT=/verified/path/trust-root.json

liknon-manager search evidence
liknon-manager install 'knowledge:example/evidence@^1' --dry-run
liknon-manager install 'knowledge:example/evidence@^1' --yes
liknon-manager verify --offline
```

No production Liknon Hub URL is advertised before its publication and public
readback gates pass. Local paths and `file://` origins are supported only for
explicit local or test use.

## Trust And State

The lock retains the exact catalog snapshot, signing identity, resource graph,
compatibility facts, payload lengths and hashes, and materializations. Catalog
bytes, their detached signature, and the public trust root are retained below
`.liknon/resources/.metadata/` for offline verification.

The complete protocol, Hub, governance, and operating documentation starts at
[`docs/resources/README.md`](../../docs/resources/README.md).
