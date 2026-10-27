# Plan B04: Unified Launcher And CLI Suite

## Status

`Decision gate` at B04.0. No launcher, aggregate package, native suite, or
publication work begins until the contract is approved.

The [validation-system roadmap](../README.md) owns cross-track order. This plan
is the sole owner of the unified launcher, aggregate Cargo package, and complete
native-suite composition. It does not own the behavior or individual release
of any capability CLI.

## Dependencies

- [A01](../A01-product-and-repository-separation/README.md) must establish the
  shared implementation boundaries and buildable application packages.
- The [shared CLI distribution foundation](../cli-distribution-foundation.md)
  must have a proven credential-free implementation before native launcher or
  suite candidates are produced.
- The command contracts selected by B01-B03 must be stable before integrated
  launcher verification.
- A complete native suite may be assembled only from verified release
  candidates owned by B01-B03.
- [D01](../D01-system-conformance-and-agent-evaluation/README.md) later verifies
  the integrated candidate without taking publication ownership.

## Product Boundary

Cargo package `liknon-cli` is the promoted complete-suite installation route.
It owns these binary targets:

```text
liknon             likx
liknon-validator    likv
liknon-manager     likm
liknon-tool        likt
```

`liknon` and `likx` are equivalent launcher entry points. The other six
targets expose the same implementations as their independently installable
application packages:

| Capability | Standalone Cargo package | Explicit command | Compact command |
| --- | --- | --- | --- |
| Validation | `liknon-validator` | `liknon-validator` | `likv` |
| Resource management | `liknon-manager` | `liknon-manager` | `likm` |
| Local resource access | `liknon-tool` | `liknon-tool` | `likt` |

The aggregate package declares every target itself and reuses shared
implementation crates. It does not assume Cargo installs binary targets from
dependencies. Package `liknon` is reserved for the future programmatic
integration plan B05 and owns no CLI command by implication.

## Launcher Contract

Capability dispatch uses one explicit namespace:

```text
liknon validate <args> -> liknon-validator <args>
liknon manager <args>  -> liknon-manager <args>
liknon tool <args>     -> liknon-tool <args>
```

`likx` has exactly the same grammar. The launcher removes only the selected
namespace, preserves every remaining argument boundary, forwards the working
directory, environment and process streams, and returns the selected
component's outcome.

There is no default namespace and no compatibility route such as
`liknon check ...`. Dispatch is shell-free. The launcher never validates,
downloads, installs, updates, repairs, mutates resources, interprets content,
or performs AI work.

## Help And Version Contract

Root `--help` for `liknon` and `likx`:

- identifies the program as a launcher;
- briefly lists `validate`, `manager`, and `tool`;
- shows the unified grammar;
- directs the user to `liknon <namespace> --help`; and
- remains available when a component is absent or invalid.

Capability-specific help is emitted by the selected component. The launcher
does not merge or reproduce its command tree.

Root `--version` reports the launcher first and then
`liknon-validator`, `liknon-manager`, and `liknon-tool`, in that order. Version
inspection uses the same trusted executable resolution as dispatch. Missing,
invalid, incompatible, and unresponsive components are reported explicitly.
B04.0 decides the exact output, timeout, and exit status, not this ordering or
coverage.

## Distribution Ownership

B04 owns:

- crates.io publication of aggregate package `liknon-cli`;
- commands `liknon` and `likx`;
- aggregate installation and command-collision handling;
- complete native suites and exact component manifests;
- cross-channel component-location and compatibility metadata;
- launcher and suite provenance, recovery, and publication evidence.

B04 instantiates the shared distribution foundation with a launcher/suite
descriptor. It uses the common machinery to build one final launcher payload
per target, but it does not build capability payloads. A complete direct native
suite combines the exact retained launcher payload with exact verified
B01-B03 candidates for the same target and records every component identity and
hash.

The suite archive and installer expose all eight command names, authenticate
the complete component manifest before extraction, verify every payload and
alias, and update the installed suite transactionally. A missing or
incompatible component fails before replacement of an existing suite.

The aggregate crates.io source package is a different distribution form. It
owns and compiles its eight declared binary targets from the accepted source
closure; it does not package prebuilt B01-B03 native candidates.

## Version And Provenance Contract

The exact `liknon-cli` Cargo package selected by B04.0 is the sole version
authority for the launcher and complete-suite release. Structured Cargo
metadata, a B04-owned signed tag, and the full source commit define its release
identity. B04.0 records the exact package path and tag prefix; no second suite
version is authored and no capability version is borrowed.

The launcher build manifest records that identity and each final launcher
target, command, toolchain, size, hash, and platform-authenticity result. A
direct complete-suite manifest additionally records every retained component's
product, package, version, signed tag, commit, target, final hash, provenance
reference, and compatibility result. Component versions may differ and remain
owned by B01-B03.

The aggregate crates.io source candidate has its own package-inventory,
supporting-closure, and digest evidence because it compiles all eight targets
from source. The direct suite has composition evidence because it reuses exact
native candidates. These evidence forms are not interchangeable.

Versioned suite objects are immutable. The B04 stable pointer advances only
after exact public readback and complete-suite verification, and B04 recovery
cannot move a B01-B03 pointer or rewrite component provenance.

B01-B03 retain source-package and native publication of their standalone
products. B04 consumes verified candidates and does not republish an individual
component under a second owner.

## Architectural Invariants

1. Capability CLIs remain usable when the launcher and sibling CLIs are absent.
2. Capability applications neither invoke nor depend on the launcher.
3. Only the launcher invokes one selected capability executable.
4. Launcher targets import no capability API merely to dispatch.
5. Aggregate capability targets and standalone targets use the same shared
   implementation and satisfy the same command contract.
6. Product separation is defense in depth, not an operating-system sandbox.
7. Aggregate and standalone installations in one Cargo root are alternative
   owners of the same capability command names; collisions are explicit.
8. No package, executable, or native artifact named `liknon-cli` is described
   as the Validation CLI alone.
9. Shared release machinery is reused, while launcher and suite versions,
   tags, candidates, namespaces, credentials, approval, and recovery remain
   B04-owned.
10. Direct suite assembly cannot rebuild, resign, or mutate a retained
    capability candidate.
11. Launcher and suite identity is package-derived and each composed component
    retains its independently verified version, tag, commit, target, hash, and
    provenance.

## Decision Gate And Later Units

| Unit | Document | Status | Primary result |
| --- | --- | --- | --- |
| B04.0 | [Launcher And Suite Contract](00-launcher-and-suite-contract.md) | `Decision gate` | Approved discovery, compatibility, process, composition, collision, and release contracts |
| B04.1 | Launcher implementation | `Future`, defined after B04.0 | Shell-free launcher and direct/unified parity |
| B04.2 | Aggregate Cargo package | `Future`, defined after B04.0 | Package-owned eight-target suite |
| B04.3 | Launcher native builds and complete native suites | `Future`, defined after B04.0 and B01-B03 candidates | One final launcher payload plus verified target-specific suite artifacts composed from exact component candidates |
| B04.4 | Integrated publication and recovery | `Future`, defined after B04.0 | Isolated aggregate release and recovery evidence |

Later unit documents are created only after B04.0 resolves their contract.

## Verification Requirements

Future B04 implementation must prove:

- direct and unified argument, stream, working-directory, interruption, and
  exit-status equivalence;
- `liknon` and `likx` help and dispatch equivalence;
- brief root help and delegated capability help;
- launcher-first version output with all three components and explicit
  inspection failures;
- deterministic handling of absent, incompatible, shadowed, or replaced
  executables;
- resistance to argument-boundary and executable-path attacks;
- parity between aggregate capability targets and standalone packages;
- independent capability operation when the launcher is absent;
- rejection of removed implicit command forms; and
- absence of network, validation, resource, or AI behavior in the launcher.

## Acceptance Criteria

- [ ] B04.0 is approved before implementation or publication begins.
- [ ] Package `liknon-cli` owns exactly the approved eight binary targets.
- [ ] Package `liknon` remains outside this plan and provides no launcher.
- [ ] Every capability dispatch uses an explicit namespace.
- [ ] Root help is launcher-focused and detailed help is delegated.
- [ ] Root version covers the launcher and every resolved capability.
- [ ] Aggregate installation does not rely on dependency-owned executables.
- [ ] Standalone products preserve independent release and operation.
- [ ] Complete suites contain only verified B01-B03 candidates.
- [ ] Every supported target contains one newly built and verified launcher
      payload plus unchanged compatible capability candidates.
- [ ] The launcher and complete suite use one B04-owned package-derived release
      identity with no handwritten or inherited version.
- [ ] Aggregate source and direct native-suite provenance remain explicit and
      non-interchangeable.
- [ ] The launcher/suite profile uses shared mechanisms without inheriting a
      capability product's publication authority.
- [ ] D01 can verify direct and unified behavior without owning publication.

## Result

Users can install each capability selectively or install one complete native
suite. The unified command improves ergonomics without merging capability
authority, runtime logic, or release ownership.
