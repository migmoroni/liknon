# Phase B04.0: Approve The Launcher And Suite Contract

## Status

`Decision gate`. This phase changes no product code, package, CI workflow,
artifact, public documentation, or published channel.

[Back to Plan B04](README.md)

## Purpose

Resolve the operational and release questions required to implement the
capability-free launcher and aggregate suite without weakening the independent
product boundaries established by A01 and B01-B03.

## Required Inputs

- the approved A01 package and dependency graph;
- stable direct command contracts from B01-B03;
- the supported platform and process guarantees of each component;
- evidence from current shell-free execution and platform-specific process
  handling;
- the approved eight-target list from the target summary and B04 README.

The target list is fixed input. This gate does not reopen it.

## Decisions Required

### Executable Resolution And Compatibility

Decide:

- trusted executable locations and exact lookup precedence;
- protection against self-resolution, path shadowing, unsafe replacement, and
  launcher recursion;
- component identity and compatibility metadata;
- coordinated or independent version policy;
- behavior for absent, incompatible, invalid, or unresponsive components;
- inspection timeout and diagnostics.

### Process Semantics

Decide and prototype:

- POSIX replacement or forwarding behavior;
- Windows process, signal, interruption, and exit-status parity;
- exact preservation of arguments, environment, working directory, stdin,
  stdout, and stderr;
- launcher-owned usage failures versus component-owned outcomes;
- shell-free execution on every supported platform.

### Aggregate Package And Installation

Decide:

- source mapping from all eight binary targets to shared implementation crates;
- dependency direction that prevents launcher capability coupling;
- package inventory and supporting-crate publication order;
- aggregate versus standalone ownership in a shared Cargo installation root;
- collision detection, replacement, rollback, and recovery;
- whether partial native suites are supported.

### Native Suites And Publication

Decide:

- exact suite composition for each supported target and channel;
- component-location rules across aggregate and standalone channels;
- compatibility and provenance manifest contracts;
- how B04 consumes immutable B01-B03 candidates without assuming publication
  ownership;
- release versioning, tags, artifact names, credentials, retention, retry, and
  recovery boundaries;
- whether publication waits for all three standalone candidates or supports a
  documented narrower readiness rule.

### Help And Aggregate Version

Decide only the details left open by the approved contract:

- exact human and machine-readable version format, if both are needed;
- component inspection timeout;
- exit status when one or more component versions cannot be obtained;
- diagnostics for unavailable, incompatible, invalid, and unresponsive states.

The gate may not expand root help into capability documentation, change the
component list or ordering, or hide an inspection failure.

## Required Prototypes

Before approval, produce disposable evidence for:

1. exact argument and stream forwarding without a shell;
2. POSIX and Windows interruption/exit behavior;
3. deterministic trusted resolution and self-recursion prevention;
4. missing, invalid, incompatible, shadowed, and unresponsive components;
5. package-owned installation of all eight Cargo binary targets;
6. aggregate/standalone collision detection in an isolated Cargo root;
7. brief root help plus delegated component help;
8. launcher-first version output with complete failure reporting;
9. one representative complete native-suite composition from retained
   component candidates.

Prototype code is not production code and is removed unless its ownership is
approved for B04.1 or later.

## Verification

- Every decision has an owner, test strategy, and recovery behavior.
- Resolution never falls back silently to validation or another component.
- Dispatch never reconstructs a command string.
- Root help works without installed components.
- Version inspection uses the normal trusted resolution contract.
- Aggregate installation proves targets belong to `liknon-cli` itself.
- Capability implementations remain independently buildable and installable.
- Native composition consumes exact verified candidates and does not rebuild
  them under B04.
- Package `liknon` is absent from launcher and suite ownership.
- D01 receives an explicit future scenario matrix for the approved contract.

## Exit Criterion

The user has approved one implementable launcher, aggregate-package, native-
suite, collision, compatibility, publication, and recovery contract. Only then
may B04.1-B04.4 receive implementation-ready phase documents.
