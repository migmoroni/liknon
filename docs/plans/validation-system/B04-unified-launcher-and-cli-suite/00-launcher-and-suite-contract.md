# B04.0 - Launcher And Suite Contract Gate

## Purpose

Freeze the package, process, routing, archive, and release contracts required
to consolidate the three capability CLIs into one distributable suite without
merging their responsibilities.

No B04 implementation begins before every decision and prototype in this gate
is recorded and accepted.

## Fixed Inputs

The gate starts from these approved decisions:

- public source package: `liknon-cli`;
- physical executables: `liknon`, `liknon-validator`, `liknon-manager`, and
  `liknon-tool`;
- no compact executable aliases;
- launcher namespaces: `validator|v`, `manager|m`, and `tool|t`;
- capability behavior remains owned by B01, B02, and B03;
- one native suite archive per supported target;
- every native archive contains all four executable roles for that target;
- source and native installation expose the same command set;
- Hub resources remain external data and are not bundled into the suite.

This gate refines implementation mechanics. It does not reopen those product
decisions.

## Required Decisions

### 1. Cargo Ownership And Source Closure

Record:

- the exact `apps/liknon-cli` package manifest;
- four and only four `[[bin]]` targets;
- which supporting crates remain publishable dependencies;
- which implementation must move or be folded to avoid unpublished path
  dependencies;
- the committed lockfile and supported Rust toolchain;
- package inventory rules proving that no Hub corpus or AI-resource payload is
  included.

Capability implementation crates must own no executable targets after the
cutover.

### 2. Launcher Resolution And Process Semantics

Specify:

- sibling executable resolution for Cargo and native installations;
- Windows executable suffix handling;
- deterministic failure when a sibling executable is absent or mismatched;
- exact namespace stripping and argument forwarding;
- working-directory, environment, standard-stream, signal, interruption, and
  exit-code behavior;
- recursion prevention and rejection of unknown namespaces;
- long and short namespace equivalence;
- prohibition on fallback to PATH entries from another suite installation.

The launcher must execute the sibling capability binary from its own installed
suite. It must not import or call capability implementation in-process.

### 3. Help And Version Contract

Record:

- concise root help for the three launcher namespaces;
- focused help ownership in each capability executable;
- suite and component version output;
- mismatch detection and failure semantics;
- JSON or other machine-readable version output, if required by B06.

### 4. Native Target And Archive Contract

For every supported target, record:

- target triple and runtime baseline;
- archive format and filename;
- the four executable filenames;
- platform-authenticity mode;
- archive and executable hash fields;
- provenance references;
- installer and uninstaller behavior;
- target-detection rules, including GNU preference when GNU and MUSL both
  appear viable;
- atomic replacement and rollback semantics for all four files.

Each target produces one archive. An archive contains no binaries for another
target and no separately versioned capability payload.

### 5. Release Identity And Publication

Record one canonical identity:

```text
(product=cli-suite, package=liknon-cli, version, tag, commit, target)
```

Define:

- the sole version authority;
- tag syntax;
- source package digest and dependency closure;
- one immutable object prefix and one stable pointer for the suite;
- candidate, approval, publication, rollback, and public-readback states;
- how B06 consumes the suite candidate without rebuilding it;
- how the site resolves one target to one suite archive.

B01, B02, and B03 must not publish independent public CLI stable pointers after
this cutover.

## Required Prototypes

Run the following prototypes before accepting implementation:

1. package `liknon-cli` and inspect its exact four binary targets;
2. install the packaged source into an isolated Cargo root and prove all four
   command files exist while compact aliases do not;
3. execute each capability directly and through both launcher namespace forms;
4. prove argument, environment, working-directory, stream, signal, and exit-code
   preservation;
5. prove the launcher rejects a missing, foreign, or version-mismatched sibling;
6. build one target-native archive containing the four executables and inspect
   its complete inventory;
7. verify every executable and archive hash from authenticated metadata;
8. install, interrupt, update, roll back, and uninstall the four-file suite in
   an isolated destination;
9. prove GNU and MUSL selection requests exactly one matching Linux archive;
10. prove B01, B02, and B03 behavior suites remain green through direct and
    routed execution;
11. rehearse B06 candidate handoff without rebuilding or renaming payloads.

## Failure Cases

The gate must define deterministic rejection for:

- missing or extra Cargo binary targets;
- compact executable aliases reintroduced by metadata or packaging;
- unresolved or unpublished source dependencies;
- missing, foreign, recursive, or version-mismatched sibling executables;
- unknown or missing launcher namespaces;
- changed argument ordering or exit status;
- archive inventory drift;
- duplicate, missing, or cross-target payloads;
- manifest, tag, commit, target, size, or hash mismatch;
- partial installation or mixed component versions;
- unsafe extraction paths, links, or permissions;
- unsigned, expired, or rolled-back stable metadata;
- a public site link resolving more than one target artifact.

## Exit Criteria

- [ ] Package ownership is fixed at exactly four binary targets.
- [ ] Compact executable aliases are explicitly absent.
- [ ] Launcher namespaces and process behavior are frozen.
- [ ] Supporting-crate publication or consolidation is decided.
- [ ] Help and version contracts are frozen.
- [ ] Every supported target has one reviewed suite archive contract.
- [ ] Installer atomicity and rollback are specified for all four files.
- [ ] One suite release identity and stable pointer are approved.
- [ ] Site target selection and R2 object layout are approved.
- [ ] Required prototypes pass on the available platform and CI covers the
  remaining supported targets.
- [ ] B01, B02, B03, and B06 handoffs are explicit.

## Exit Evidence

B04.0 exits with a reviewed decision record, package inventory, source-closure
proof, launcher prototype, target/archive matrix, installer prototype, release
identity, and B06 handoff contract. Later B04 units implement that exact
contract rather than reopening product topology.
