# Phase A01.1: Create The Cargo Workspace And Extract Shared Contracts

[Back to the product and repository separation plan](README.md)

## Purpose

Create the virtual Cargo workspace through one atomic package cutover, move
deterministic validation behind a terminal-independent core API, remove the
current runtime knowledge coupling, and establish the structural workspace and
path primitives later used by Manager and Tool. The repository must build and
test at the end of this unit; A01.2 is not a repair step for an incomplete
workspace conversion. B02.0, not A01, codifies public resource manifests,
locks, Hub schemas, and Manager command semantics.

## Workspace Contract

- The root `Cargo.toml` is virtual and owns shared edition, MSRV, authorship,
  license, repository metadata, dependencies, lints, and release profiles.
  Independently released packages retain explicit package versions; the
  workspace does not impose lockstep version inheritance.
- The root uses Cargo resolver version 2 and explicit `members` for the two
  support crates, three applications, and knowledge-maintenance package. It
  explicitly excludes nested fixture workspaces such as `fixtures/outcomes`
  and `fixtures/process-tool`; product-member globs must not absorb fixture
  packages.
- The root `Cargo.lock` remains committed because the repository ships
  applications.
- `crates/liknon-core` owns validation contracts and behavior.
- `crates/liknon-workspace` owns `.liknon/` boundary discovery, path-safe
  canonical locations, and bootstrap identity and compatibility primitives.
  Public resource identities, manifests, locks, provenance schemas, workflow
  types, policy types, artifact envelopes, and resolver behavior remain work
  for their later owning phases.
- `crates/liknon-workspace/contracts/ai/ENTRYPOINT.md` is the canonical authored
  bootstrap content and its adjacent `manifest.json` is the canonical metadata.
  Keeping both below the owning package makes the source available to local
  builds and packaged dependents without copying a file from outside the Cargo
  package boundary. The manifest records schema version, SemVer contract
  version, media type, canonical path relative to that contract directory, and
  SHA-256 over the exact UTF-8 entry bytes without newline normalization.
- `apps/liknon-cli` owns binary targets `liknon` and `likx`.
- `apps/liknon-manager` owns binary targets `liknon-manager` and `likm`.
- `apps/liknon-tool` owns binary targets `liknon-tool` and `likt`.
- `tools/knowledge` becomes the repository-only Cargo package
  `liknon-knowledge-maintenance` with `publish = false`.
- Both names in each application invoke the same implementation.
- Dependency edges point from applications to approved internal crates, never
  from a contract crate to an application.
- Every future publishable member has complete inherited or explicit package
  metadata and a package-contained source inventory, README, schemas, assets,
  and contract files required to build or document that member. No Cargo
  package include rule reaches above its package root.
- The workspace MSRV is verified for every affected member.

## Atomic Package Cutover

The root package is converted to a virtual workspace only in the same change
that performs all of the following:

1. creates the two supporting crates, the three application packages, and the
   repository-only knowledge-maintenance package;
2. moves every existing source module and test to its recorded owner;
3. moves the CLI ASCII asset below `apps/liknon-cli` and the canonical config
   and report schemas below `crates/liknon-core`;
4. creates the package-contained bootstrap source and metadata below
   `crates/liknon-workspace`;
5. removes the root `build.rs`, generated embedded catalog, runtime knowledge
   module, knowledge CLI grammar/help, and every embedded-corpus test;
6. removes the superseded `skills/liknon` tree and `tests/skills.rs`, whose
   instructions depend on the deleted knowledge command; Phase 3 later authors
   the final AI-Tool resource under the approved resource contract;
7. turns the existing knowledge checker into the explicit maintainer package
   and moves its contract tests with it;
8. removes `examples/inspect.rs` and any accidental promise of the old root
   crate's Rust API; and
9. updates every immediately affected include path, schema link, help test,
   public command example, contributor command, and package inventory
   assertion. Removed `liknon knowledge` examples are not replaced with future
   Tool commands before B03 implements them; and
10. updates CI, dependency-policy configuration, and ignore rules to target the
    virtual workspace and repository-only maintainer member rather than the
    former root package.

There is no commit or phase handoff in which the root manifest is virtual while
source, `build.rs`, examples, or tests still expect the former root package.

## Future Registry Dependency Contract

Application manifests are prepared for Cargo's path-plus-version dependency
form for supporting workspace crates. Local A01 builds use workspace paths.
Each distribution track later fixes registry versions, packages the closure,
publishes in topological order when required, and proves registry-only
resolution. A01 requires neither a registry credential nor public readback.

`liknon-core` and `liknon-workspace` have their own explicit package versions and
publishable source inventories. Their public package documentation labels them
as supporting implementation crates rather than the stable programmatic SDK
planned by B04. An application release pins accepted compatible versions and
does not rely on a Git dependency, unpublished path, or generated vendored
copy.

## Validation Core Extraction

Move exact-path configuration parsing and validation, config/report schema
generation, contract types, DAG construction, planning, shell-free process
execution, repository inspection, structured outcomes, machine-readable
validation reports, and terminal-independent progress events into
`liknon-core`.

The validation core does not discover a consumer workspace, parse CLI
arguments, render a terminal, terminate the host process, access the Hub,
install resources, or interpret authored content. Application code converts
core events and structured reports into live progress, human output, JSON
streams, and process exit status.

### Required Source Split

| Current responsibility | Target responsibility |
| --- | --- |
| Config parsing, parameter substitution, semantic validation, schema generation, and config digest | `liknon-core` APIs that receive explicit paths or bytes |
| Ancestor discovery, canonical `.liknon/` locations, containment, symlink rejection, no-overwrite checks, and atomic file placement | `liknon-workspace` primitives |
| Config candidate parsing/validation inside current `initialization.rs` | `liknon-core` |
| Initialization destination safety and atomic placement inside current `initialization.rs` | `liknon-workspace` |
| Init arguments, user-facing result, streams, and exit mapping | `liknon-cli` |
| DAG, planning, prerequisites, process-tree execution, timeout/cancellation state, repository fingerprinting, and structured outcomes | `liknon-core` |
| Terminal-independent progress event contract | `liknon-core` |
| Indicatif progress bars, tree geometry, color/presentation, human rendering, and terminal help | `liknon-cli` |
| Versioned report types and canonical JSON serialization/schema semantics | `liknon-core` |
| JSON stream selection and stdout/stderr/process behavior | `liknon-cli` |
| Global error enum | Split into package-local errors; no support crate depends on an application error type |

Member manifests declare only the dependencies their moved code uses. Clap,
console styling, terminal progress, and signal-handler adapters stay in
`liknon-cli`; serialization, schema, SemVer validation, hashing, and
platform process-tree dependencies follow the owning core/workspace code. A
dependency listed centrally in `[workspace.dependencies]` does not authorize an
unused member dependency.

## Workspace Contract Extraction

`liknon-workspace` initially provides deterministic values and operations for:

- nearest-`.liknon/` discovery and explicit-root validation;
- canonical validation, future resource-root, and AI-root paths without
  defining the future files stored below them;
- exact path containment and symlink rejection;
- AI bootstrap-contract identity, supported version, exact-byte digest, and
  metadata loading without interpreting its instructions.

B02.0 extends these foundations with approved public resource, manifest, lock,
provenance, trust, and Hub contracts. B03.0 defines the Tool machine-result
envelope. A01 does not guess either contract in advance.

It performs no network access, command execution, terminal rendering, resource
selection policy, or validation execution.

## Application Boundaries

Move Clap definitions, validation help, themes, human report rendering,
progress, signal adaptation, stream selection, and exit-code mapping into
`apps/liknon-cli`.

The Validation CLI resolves one workspace boundary through
`liknon-workspace`, passes the exact configuration path to `liknon-core`, and
renders only core events and results. It must not regain validation logic
through a CLI-only copy.

Each application package has one small library entry point and two binary
targets whose `main` functions delegate to it. The long and compact commands do
not compile separate command trees or copy behavior.

Create isolated Manager and Tool application packages and their two binary names
without implementing their later feature sets. B02 owns Hub transport,
resolution, installation, and resource-state mutation. B03 owns bootstrap
projection, local discovery, retrieval, and context assembly. Neither
application may acquire those responsibilities accidentally during extraction.

Until B02 and B03 approve and implement their public command contracts, the
Manager and Tool packages are non-published structural shells. They expose only
their application identity, version, concise help, and equivalent long and
compact binary dispatch. They create no workspace state, accept no final
feature grammar, and include no authored payload.

## Repository-Only Knowledge Maintenance

`liknon-knowledge-maintenance` preserves catalog, source, editorial, recipe,
link, and release checks without becoming a user-facing application. Its tests
read the canonical repository sources explicitly. It may use `liknon-core` to
validate recipe configuration, but no application depends on it and it is
excluded from every publishable runtime inventory.

The maintainer interface preserves the existing explicit `--check` and
`--write` responsibilities under a package command such as
`cargo run -p liknon-knowledge-maintenance -- --check`. It is not installed as
a user command, does not receive a public CLI alias, and never enters native
distribution plans.

The current knowledge build script and `liknon knowledge` routes are deleted,
not redirected to the maintainer package. A01.2 later relocates editorial
schemas and completes neutral resource assembly inputs after this workspace is
already green.

## Verification

- Validation fixtures pass through `liknon-core`.
- Workspace-boundary and bootstrap-metadata fixtures pass through
  `liknon-workspace`.
- Validation CLI retains isolated argument, stream, help, and exit tests.
- Manager and Tool package shells compile, expose both approved binary names,
  expose only identity/version/help, and contain no undeclared behavior claim.
- Dependency inspection proves both internal crates are terminal-independent.
- Bootstrap metadata verification proves the recorded digest equals the exact
  `crates/liknon-workspace/contracts/ai/ENTRYPOINT.md` bytes.
- `cargo metadata --no-deps` proves the expected member and dependency graph;
  `cargo tree` proves that support crates do not depend on applications or
  terminal rendering.
- Metadata proves nested fixture workspaces are not product members; their own
  committed lockfiles and explicit fixture commands continue to operate
  independently.
- `cargo test --workspace --all-targets --locked` exercises all members.
- Isolated local installation proves the two names of each package dispatch the
  same package implementation; feature behavior for Manager and Tool remains a
  B02/B03 gate.
- `cargo package --list` and local candidate-archive inspection prove intended
  source inventories and no repository-only leakage. Packages whose exact
  supporting versions are already registry-resolvable also pass normal package
  verification. A01 does not require an application package to resolve an
  unpublished support crate from crates.io; registry-only package verification
  remains the owning distribution track's gate after topological publication.
- Repository search proves no runtime source, application test, or package
  manifest references `OUT_DIR`, the removed root `build.rs`, `src/knowledge`,
  `skills/liknon`, or a `liknon knowledge` command.
- Maintained public and contributor documentation contains no invocation of the
  removed knowledge command and does not claim an unimplemented Tool command as
  its replacement.
- The knowledge-maintenance checks run from their own package and are absent
  from every application package inventory.

## Exit Criterion

The workspace builds with one validation implementation, foundational workspace
and bootstrap contracts, three isolated application packages, and no circular
or hidden executable dependency. No exit claim implies that Manager or Tool
features already exist.
