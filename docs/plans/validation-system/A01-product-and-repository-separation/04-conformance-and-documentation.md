# Phase A01.4: Prove Conformance And Reconcile Documentation

[Back to the product and repository separation plan](README.md)

## Purpose

Prove that repository boundaries preserve validation behavior, establish the
inputs for later resource interoperability, and leave every maintained document
and test aligned with the current structural product model.

## Conformance Matrix

Verify independently:

- validation-core configuration, planning, execution, cancellation, timeout,
  repository, and report semantics;
- Validation CLI arguments, help, streams, progress, themes, signals, aliases,
  initialization, and exit statuses;
- Manager and Tool package isolation, binary-name equivalence, dependency
  direction, and absence of undeclared validation or authored payloads;
- exact program-owned AI bootstrap source and metadata consistency, including
  version grammar and digest over raw entry-point bytes;
- local source installation of all three application packages, with complete
  behavior claimed only for Validation CLI;
- local package inspection of supporting crates and application candidates;
- validation artifacts excluding authored resources;
- Manager and Tool artifacts excluding resource payloads and the validation
  engine;
- the repository-only knowledge-maintenance package retaining editorial checks
  while remaining absent from every application artifact;
- neutral knowledge, skill, collection, dependency, and workflow fixture sources
  with one canonical owner and deterministic assembly evidence;
- source and binary inspection proving that no application branches on an
  official workflow, skill, tool, process, node, action, transition, or
  agent-profile ID;
- repository inventory proving each authored resource has one owner.

Existing outcome fixtures compare structured validation meaning before and
after extraction. Neutral resource fixtures prove source ownership, exact
assembly inputs, and absence of application-specific identity branches. B02 and
B03 later test catalog, lock, Manager, and Tool semantics.

## Documentation Reconciliation

Update maintained documentation and plans so they consistently state:

- `crates/liknon-workspace/contracts/ai/ENTRYPOINT.md` and its adjacent
  `manifest.json` are the package-contained, program-owned first Liknon
  instruction and identity source; B03 implements the exact `likt bootstrap`
  projection;
- `liknon-cli` is the Cargo application package whose commands are `liknon`
  and `likx`;
- `liknon-manager` provides `liknon-manager` and `likm`;
- `liknon-tool` provides `liknon-tool` and `likt`;
- `.liknon/` is the only consumer workspace root;
- B01 distributes validation, B02 establishes Hub and Manager, B03 distributes
  Tool, and B04 governs future programmatic APIs;
- knowledge is a selectable target resource whose installation and consumption
  are implemented by B02 and B03;
- Phase 3 may establish AI-Tool as the official default skill of initial
  workflows through the ordinary replaceable-resource contract, with no
  application privilege;
- workflow orchestration, agent profiles, skills, tool descriptors, and
  knowledge are distinct resource contracts organized as
  `resources/<kind-directory>/<namespace>/`;
- package registries never distribute authored resource payloads or copied CLI
  launchers; crates.io application packages contain source and resolve only
  published supporting crates.

Public README and reference documentation describe the complete Validation CLI
and the structural existence of Manager and Tool. They do not present
`bootstrap`, Hub access, local-resource lookup, AI policy, workflow persistence,
or official workflows as available behavior before their owning phases. Future
plans may describe those targets when clearly labeled as plans.

Search the repository for obsolete command names, `.validation/`, embedded
corpus claims, package-distributed resource claims, stale track links, old
include rules, outdated installation commands, AI instructions that bypass the
bootstrap entry point, and base documents that still present an official
workflow as a platform root.

Use the A01.0 replacement ledger to update or remove each superseded maintained
document in its owning phase. Do not preserve the pre-separation wording as a
fallback or describe an expected intermediate mismatch as a second supported
contract.

## Required Gates

- `cargo fmt --all -- --check`;
- `cargo clippy --workspace --all-targets --locked -- -D warnings`;
- `cargo test --workspace --all-targets --locked`;
- `cargo test --workspace --doc --locked`;
- ignored outcome fixtures;
- exact bootstrap source-versus-metadata digest checks;
- knowledge editorial and neutral assembly exact-byte checks;
- `liknon-knowledge-maintenance` tests and package-boundary checks;
- MSRV validation;
- isolated local installation smoke tests for all three packages and six command
  names, without claiming unimplemented Manager or Tool operations;
- local `cargo package` inventory checks for publishable members;
- package-boundary, artifact-content, and local-link checks;
- repository searches for removed runtime knowledge code, stale skill paths,
  old workspace roots, root-package build inputs, and accidental pre-B04 Rust
  API examples;
- `cargo metadata --no-deps` and dependency inspection against the approved
  A01 graph.

## Exit Criterion

All structural and preserved-validation evidence passes, maintained
documentation has no stale product, bootstrap, policy, workflow-root, path, or
ownership claim, and B01 through B03 can implement their assigned contracts
without moving responsibilities again.
