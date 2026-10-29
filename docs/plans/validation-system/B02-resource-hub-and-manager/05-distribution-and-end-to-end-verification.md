# Phase B02.5: Prepare Manager Handoff And Verify Hub-To-Workspace Behavior

## Status

`Partially implemented`. B02.0-B02.4 behavior is present in the repository.
This phase closes focused Manager evidence for B04 and prepares the separately
versioned Hub candidate. It does not publish a standalone Manager package or
native archive.

[Back to Plan B02](README.md)

## Objective

Prove that Manager can enter the B04 CLI suite without losing its network,
mutation, trust, recovery, and resource-state contracts, while independently
preparing the public Hub that Manager consumes.

## Ownership

B02 owns:

- Hub protocol, objects, catalog, signatures, revocation, publication, and
  recovery;
- Manager command behavior, machine contracts, security properties, and
  focused conformance tests;
- Manager-to-Hub-to-workspace end-to-end evidence;
- the handoff record consumed by B04.

B04 owns:

- the public `liknon-manager` executable target;
- the `liknon-cli` source package and suite version;
- target-native suite archives;
- CLI provenance, installers, stable pointer, publication, and recovery.

B06 coordinates the first public activation of the exact B02 Hub and B04 suite
candidates.

## Manager Handoff Contract

The handoff records:

- exact repository commit and Manager implementation source closure;
- required supporting-crate versions;
- direct command `liknon-manager`;
- launcher routes `liknon manager` and `liknon m`;
- absence of a physical `likm` target;
- command, help, version, JSON, error, and exit-code fixtures;
- runtime dependencies and supported-target evidence;
- network and filesystem effect boundaries;
- semantic smoke commands for B04;
- compatibility with the exact Hub protocol and local resource-state schema.

The handoff does not define a Manager version, tag, R2 prefix, stable pointer,
or registry credential independent from B04.

## Source-Closure Verification

Before B04 consumes Manager:

1. enumerate every source and supporting crate required to build Manager;
2. reject unpublished or undeclared path dependencies;
3. prove the implementation builds through the `liknon-cli` package closure;
4. inspect the final package inventory for unintended Hub payloads, resources,
   fixtures, secrets, or generated state;
5. prove direct Manager help and machine contracts remain product-owned;
6. prove `liknon-cli-ui` supplies only shared terminal behavior.

## Target And Smoke Evidence

For every target selected by B04, Manager evidence proves:

- the executable starts and reports the expected suite version;
- help and JSON output satisfy the B02 contract;
- local fixture operations work without a public Hub;
- networked fixture operations contact only the explicit test Hub;
- target-specific path, permission, rename, lock, and recovery semantics pass;
- direct and launcher-routed execution are equivalent;
- interruption and cancellation preserve transaction guarantees.

A failed Manager row blocks the complete suite archive for that target.

## Hub Candidate

The Hub candidate remains a B02 product independent from the CLI suite. It
contains:

- one immutable catalog snapshot;
- immutable resource payloads and metadata;
- signatures and trust-root references;
- revocation data;
- cache and rollback policy;
- a public-readback plan;
- exact compatibility metadata for the selected Manager behavior.

Hub publication never rebuilds Manager or any other CLI executable.

## End-To-End Matrix

Exercise at minimum:

1. initialize an empty workspace through Manager;
2. search and inspect against the immutable test Hub;
3. preview an install without mutation;
4. install a resource graph transactionally;
5. consume resulting state through Tool fixtures or the implemented Tool;
6. update, remove, materialize, interrupt, and recover;
7. reject invalid signatures, revocation, digest drift, dependency cycles,
   unsafe paths, archive expansion abuse, lock contention, and incompatible
   contracts;
8. run the same Manager operation directly and through both launcher
   namespaces;
9. prove Validator remains independent from Hub availability and resource
   state;
10. prove no installed resource executes during acquisition.

## External Readiness

Human-owned setup is required only for the destinations actually retained by
the target architecture:

- Hub domain and R2 boundary;
- Hub signing and trust roots;
- protected Hub publication environment;
- B04/B06 CLI-suite publication authority;
- E01 site presentation and public links.

Do not create standalone Manager registry ownership, native-object prefixes, or
stable pointers.

## Acceptance Criteria

- [ ] Manager behavior and security tests pass from the exact B04 source closure.
- [ ] `liknon-manager` is the only physical Manager executable.
- [ ] `manager|m` route through `liknon` without changing semantics.
- [ ] Every selected B04 target has focused Manager native evidence.
- [ ] The Manager handoff contains no independent release identity.
- [ ] The Hub candidate is immutable, authenticated, revocable, and recoverable.
- [ ] Hub-to-workspace-to-Tool scenarios pass from clean state.
- [ ] Resource publication requires no CLI rebuild.
- [ ] E01 can present the exact Hub identity and B04 suite download without
  inventing another source of truth.
- [ ] B06 can consume the Hub and suite candidates without rebuilding either.

## Exit

B02.5 exits when B04 can build and verify Manager as one member of the complete
CLI suite, while B02 can independently publish and recover the exact Hub that
Manager consumes.
