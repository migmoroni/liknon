# Phase A01.1: Create The Cargo Workspace And Extract Shared Contracts

[Back to the product and repository separation plan](README.md)

## Purpose

Create the virtual Cargo workspace, move deterministic validation behind a
terminal-independent core API, and establish the shared workspace/resource
contract used by Manager and Tool.

## Workspace Contract

- The root `Cargo.toml` is virtual and owns shared package metadata,
  dependencies, lints, release profile, and version policy.
- The root `Cargo.lock` remains committed because the repository ships
  applications.
- `crates/liknon-core` owns validation contracts and behavior.
- `crates/liknon-workspace` owns `.liknon/` boundary discovery, path-safe
  resource locations, resource identities, manifests, locks, provenance, and
  compatibility types.
- `apps/liknon-cli` owns binary targets `liknon` and `likx`.
- `apps/liknon-manager` owns binary targets `liknon-manager` and `likm`.
- `apps/liknon-tool` owns binary targets `liknon-tool` and `likt`.
- Both names in each application invoke the same implementation.
- Dependency edges point from applications to approved internal crates, never
  from a contract crate to an application.
- The workspace MSRV is verified for every affected member.

## Registry Dependency Contract

Application manifests use Cargo's path-plus-version dependency form for
supporting workspace crates. Local builds use the path; packaged crates resolve
the declared registry version. Before any application is published to
crates.io, its supporting-crate closure is packaged, inspected, published in
topological order when not already available byte-for-byte, and read back from
the registry.

`liknon-core` and `liknon-workspace` have their own package versions and
publishable source inventories. Their public package documentation labels them
as supporting implementation crates rather than the stable programmatic SDK
planned by B04. An application release pins accepted compatible versions and
does not rely on a Git dependency, unpublished path, or generated vendored
copy.

## Validation Core Extraction

Move configuration, contract types, DAG construction, planning, shell-free
process execution, repository inspection, structured outcomes, and
machine-readable validation reports into `liknon-core`.

The validation core does not parse process arguments, render a terminal,
terminate the host process, access the Hub, install resources, or interpret
authored content.

## Workspace Contract Extraction

`liknon-workspace` provides deterministic values and operations for:

- nearest-`.liknon/` discovery and explicit-root validation;
- canonical validation, resource, and AI paths;
- stable resource IDs and kinds;
- manifest and lock parsing;
- exact path containment and symlink rejection;
- version, compatibility, origin, length, and digest metadata.

It performs no network access, command execution, terminal rendering, resource
selection policy, or validation execution.

## Application Boundaries

Move Clap definitions, validation help, themes, human report rendering,
progress, signal adaptation, stream selection, and exit-code mapping into
`apps/liknon-cli`.

Manager owns Hub transport, resolution, installation, and resource-state
mutation. Tool owns local read-only discovery, retrieval, and context
projection. Neither application invokes another executable.

## Verification

- Validation fixtures pass through `liknon-core`.
- Workspace and resource fixtures pass through `liknon-workspace`.
- Every application has isolated argument, stream, help, and exit tests.
- Dependency inspection proves both internal crates are terminal-independent.
- Manager network tests require no validation engine.
- Tool tests require no network or validation configuration.
- `cargo test --workspace --all-targets --locked` exercises all members.
- Isolated installation proves both command names of each application.
- `cargo package` verification for every publishable member proves that each
  application resolves only published supporting dependencies.

## Exit Criterion

The workspace builds with one validation implementation, one shared workspace
contract, three independent applications, and no circular or hidden executable
dependency.
