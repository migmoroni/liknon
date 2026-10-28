# Phase B04.0: Approve The Launcher And Suite Contract

## Status

`Decision gate`. This phase changes no product code, package, CI workflow,
artifact, public documentation, or published channel.

[Back to Plan B04](README.md)

## Purpose

Resolve the operational and release questions required to implement the
capability-free launcher and aggregate suite without weakening the independent
product boundaries established by A01 and B01-B03.

The approved contract must also define a B06 handoff: one immutable B04
candidate identity, exact B01-B03 component closure, product-owned workflow
entry point, protected authority boundary, public-readback procedure, and
resumable destination state. B06 coordinates the first invocation but cannot
rebuild the suite or receive B04 credentials.

## Required Inputs

- the approved A01 package and dependency graph;
- the [shared CLI distribution foundation](../cli-distribution-foundation.md)
  and its credential-free conformance evidence;
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

- the launcher/suite product descriptor, release workflow, target rows, tag,
  Cloudflare storage boundary and prefix, stable pointer, protected
  environments, and recovery owner;
- how one final launcher payload is built and verified per target through the
  shared foundation;
- exact suite composition for each supported target and channel;
- component-location rules across aggregate and standalone channels;
- compatibility and provenance manifest contracts;
- multi-payload archive, installer, transactional replacement, rollback, and
  uninstall behavior for all eight command names;
- how B04 consumes immutable B01-B03 candidates without assuming publication
  ownership;
- release versioning, tags, artifact names, credentials, retention, retry, and
  recovery boundaries;
- whether publication waits for all three standalone candidates or supports a
  documented narrower readiness rule.

### Version And Provenance

Record the exact `liknon-cli` Cargo package path as the sole version authority
for the launcher and complete-suite release, plus its signed-tag prefix. The
implementation must select the package and version through structured Cargo
metadata and must not add a handwritten suite version or inherit a capability
CLI version.

Decide and document:

- the canonical B04 release identity `(suite, liknon-cli, version, tag,
  commit)` and exact annotated-tag signature requirements;
- package, binary-target, lockfile, source-closure, tag, and commit checks that
  precede all candidate creation;
- the internal launcher build manifest and the public complete-suite manifest;
- immutable object naming, stable-pointer transition, retry, rollback, and
  recovery rules for that exact identity;
- source-package inventory and digest evidence for the aggregate crates.io
  package; and
- public-readback and provenance evidence retained for every launcher and suite
  target.

The direct-suite manifest records the exact product, package, version, signed
tag, source commit, target, final hash, and provenance reference for the
launcher and every retained B01-B03 component. It also records the compatibility
decision used for that composition. Components may have different versions;
compatibility is never inferred from equality and a capability candidate is
never relabeled with the suite version.

The aggregate crates.io source package and the direct native suite remain
different provenance subjects. The source package proves the accepted package
closure from which it compiles all eight targets. The direct suite proves the
exact immutable native candidates it composes. Evidence for one cannot stand in
for the other.

The decision must distinguish the aggregate crates.io source package from a
direct native suite. The source package compiles its owned targets from source.
The direct suite combines a newly built launcher payload with exact retained
capability payloads and cannot rebuild, resign, or mutate them.

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
   component candidates;
10. one launcher native candidate produced through the shared build lane and a
    rejected cross-product candidate or namespace substitution;
11. deterministic launcher/suite manifests plus rejection of a changed version,
    tag, commit, target, component hash, provenance reference, or source-package
    closure.

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
- The launcher and suite release identity comes from the exact `liknon-cli`
  package, signed tag, and source commit without a second authored version.
- Every direct-suite component retains its original release identity, target,
  hash, and provenance reference.
- Aggregate source-package provenance is not substituted for direct native-suite
  provenance or vice versa.
- Launcher native production uses the shared foundation but B04-owned identity,
  tests, namespaces, credentials, approval, and recovery.
- Package `liknon` is absent from launcher and suite ownership.
- D01 receives an explicit future scenario matrix for the approved contract.

## Exit Criterion

The user has approved one implementable launcher, aggregate-package, native-
suite, collision, compatibility, publication, and recovery contract. Only then
may B04.1-B04.4 receive implementation-ready phase documents.
