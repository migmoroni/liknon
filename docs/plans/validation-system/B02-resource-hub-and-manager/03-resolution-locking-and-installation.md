# Phase B02.3: Implement Resolution, Locking, And Installation

[Back to the Resource Hub and Manager plan](README.md)

## Purpose

Resolve selected resources into one reproducible graph and apply that graph to
the workspace without partial or implicit effects.

## Resolution Contract

Resolution uses only authenticated catalog metadata and explicit local sources.
It considers:

- requested exact versions and accepted constraints;
- resource-schema and CLI capability compatibility;
- required dependency constraints;
- yanked, deprecated, revoked, and unavailable versions;
- trust constraints selected by the caller;
- existing lock state and explicitly requested update scope.

Candidate ordering and conflict diagnostics are deterministic. An existing lock
is reused when it still satisfies the manifest and security rules. Manager
never silently broadens a constraint or substitutes a different namespace.

## Lock Contract

The lock records:

- the exact catalog snapshot;
- every resource identity and version;
- source origin and trust class;
- dependency and collection edges;
- compatibility facts used by the resolver;
- the AI bootstrap and decision-policy compatibility facts used for governed
  workflow reporting;
- resource manifest digest;
- every payload path, length, and digest;
- installation path;
- explicit materializations.

For workflows, the locked closure includes every exact skill, agent profile,
tool descriptor, knowledge item, prompt, and subworkflow dependency required by
the selected graph. Resolution does not activate the graph or bind profiles to
models.

The lock references the required bootstrap-contract range but never stores a
resource copy of the program contract. Manager reports incompatibility against
the installed application capability and leaves the workflow inspectable but
not activatable.

The same manifest, lock, authenticated objects, and supported Manager version
produce the same installed bytes.

## Transactional Installation

Manager stages downloads in an operating-system cache, authenticates all
metadata, verifies every payload, and preflights the complete destination set
before changing the workspace.

Commit atomically replaces the resource state as one logical operation.
Interrupted staging changes nothing. Interrupted commit is detected and either
completed from exact staged bytes or rolled back without guessing.

Installation never:

- executes a payload or lifecycle script;
- invokes a shell;
- imports resource text as Manager instructions;
- alters AI policy;
- writes an undeclared destination;
- activates an agent integration implicitly.

Materialized agent integrations remain projections of locked resources. Their
destination adapters are generic by declared integration format and never
special-case an official workflow or profile identity.

## Update And Removal

`update` changes only the requested scope and the dependencies that must move
with it. It reports every transitive change before commit.

`remove` rejects removal required by retained resources unless the caller
requests a complete valid graph change. It cleans only files owned by the exact
lock and preserves unknown files.

## Explicit Materialization

Materialization projects selected locked files to a caller-approved destination
only after containment, symlink, conflict, and complete-write-set checks. It
records source identities and digests so `verify` can detect drift.

Materialization is reversible only for bytes still proven to be Manager-owned.
It never modifies global agent configuration silently.

## Verification

- Exact, ranged, transitive, conflicting, cyclic, yanked, and revoked graphs
  have fixtures.
- Re-resolution is deterministic across input ordering.
- Lock and installed bytes survive process interruption at every boundary.
- Identical installation is idempotent.
- Update cannot change an unrelated locked resource.
- A running workflow can retain its recorded lock snapshot while a later
  Manager transaction installs a newer graph for future executions.
- Removal and materialization preserve unowned files.
- A modified cached or staged payload is rejected before commit.

## Exit Criterion

Manager can reproduce, change, and remove a complete resource graph through one
deterministic lock and one recoverable transaction contract.
