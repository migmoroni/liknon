# Phase 1: Human Flow Completion

## 1. Objective

Close the Human Flow as a complete product surface before adding AI operation.
A user must be able to author or review one configuration, inspect its execution
graph, run any supported selection, understand every outcome, and identify
repository mutations introduced by a validation run without relying on AI
components.

## 2. Implementation Status

This document is a continuation plan over an implemented and verified baseline.
The status markers are normative:

- `[x]` records accepted behavior that already exists. It is not an
  implementation task and must not be rebuilt or replaced.
- All phase items below are `[x]`; no implementation work remains in this
  phase.

The two final corrections complete filesystem equivalence during initialization
and preserve the documented exit-code boundary for every inspection command.

Preserve all accepted public contracts and regression coverage. A completed
area may be touched only when a remaining item requires a narrowly scoped
correction. Do not add compatibility layers, deprecated aliases, migrations, or
parallel implementations.

### 2.1 Accepted Baseline

#### Configuration

- [x] The versioned configuration schema, semantic validation, and rejection of
      unknown fields are implemented.
- [x] IDs, references, graph cycles, dependencies, parameters, tool cycles, and
      unsafe working directories are validated.
- [x] The ownership chain `tool -> check -> suite -> group` is implemented.
- [x] Suites own contextual parameters and effective working directories.
- [x] `workspaceRoot` defines one filesystem boundary with rejection of absolute
      escapes, parent traversal, and symlink escape in the covered paths.
- [x] Commands remain shell-free program and argument vectors.
- [x] Direct check execution supports omitted optional placeholders.

#### Initialization

- [x] `workspace-validator init --config <candidate-path>` provisions the
      canonical configuration.
- [x] The command requires a provisioning capability, supports an explicit
      `--workspace`, and otherwise uses exactly the current working directory.
- [x] Initialization does not discover a root through Git, parents, or candidate
      content.
- [x] A candidate must be a contained regular file; symlinks, traversal, and
      workspace escape are rejected.
- [x] The candidate is validated for its canonical destination at
      `<workspace>/.validation/config.json`.
- [x] The exact validated bytes are published atomically without overwriting a
      different destination.
- [x] Identical bytes produce `reused`; different or invalid existing bytes
      produce a conflict.
- [x] Human and JSON initialization output derive from one versioned typed
      `InitResult` with ordered resources, paths, statuses, digests, and bounded
      diagnostics.
- [x] Initialization runs no tool preflight, check, suite, group, installation,
      or repair.

#### Inspection

- [x] `config validate` validates the whole document without executing configured
      programs.
- [x] `list --tree` renders hierarchy and reuse deterministically.
- [x] Inspection of tools and suites exposes current resolved execution data
      without starting processes.
- [x] `explain suite` exposes its effective directory, parameters, resolved
      commands, prerequisites, and check dependencies.

#### Execution, Repository Evidence, And Reports

- [x] Execution planning is deterministic and deduplicates shared suites and
      groups for one run.
- [x] Timeout, interruption, process-tree termination, output truncation, and
      dependency propagation are implemented.
- [x] The executor does not install tools, invoke an implicit shell, or repair a
      workspace.
- [x] Repository status and content fingerprints are captured before and after
      execution.
- [x] The typed repository result distinguishes pre-existing state from
      `introduced`, `removed`, and `changed` paths.
- [x] JSON reporting preserves those classified path sets and represents an
      unavailable provider explicitly.
- [x] Human and JSON reports derive from completed typed execution results.
- [x] JSON mode emits one versioned document without progress, prose, or terminal
      control bytes.
- [x] Pass, fail, blocked, skipped, timeout, invalid usage or configuration,
      internal failure, and interruption are represented by the runtime
      contracts.

#### Documentation, Distribution, And Verification

- [x] Human Flow, reference, and example documentation exists under
      `docs/validation/`.
- [x] Configuration and report schemas are owned and distributed by the crate.
- [x] `manifest.json` is the single source for skill bundle version, source crate
      version, and compatible CLI range.
- [x] Package inspection includes the intended documentation and schemas.
- [x] The current baseline passes formatting, Clippy with warnings denied, all
      regular Rust tests, Rustdoc tests, ignored outcome fixtures, package
      inspection, MSRV `1.87.0`, and the checked Windows target.

### 2.2 Accepted Completion Work

- [x] Ordinary configuration loading preserves filesystem-aware semantics when
      `..` follows an existing symlink.
- [x] Initialization uses a dedicated resolution context for the future
      canonical destination without changing the ordinary loader.
- [x] Atomic publication has a deterministic pre-publication test seam and
      protects the canonical destination during recoverable failures.
- [x] `explain group`, `explain suite`, and `explain check` share invocation
      rendering and expose effective directories, parameters, commands,
      prerequisites, and dependencies.
- [x] Human repository reporting exposes `introduced`, `removed`, and `changed`
      paths from the same typed evidence used by JSON, including informational
      mutation detection and unavailable providers.
- [x] Distributed examples validate exactly as stored at their documented
      canonical location.
- [x] Human Flow and reference documentation use current product contracts and
      do not require copied schemas.
- [x] Exit codes `0`, `1`, `2`, `3`, and `130` have real CLI coverage; exit code
      `4` is covered for `list`, initialization output, and final validation
      report writer failures.

### 2.3 Completed Scope

These two final corrections are complete:

1. [x] Complete initialization equivalence when an existing symlink resolves to
       the future `.validation/` directory only after that directory is created.
2. [x] Distinguish semantic `explain` failures from output failures so all three
       `explain` commands return exit code `4` when their output cannot be
       written.

Collision-tolerant temporary allocation and fallible schema, initialization,
and final-report output are accepted and must remain unchanged except where a
remaining correction requires shared error plumbing.

## 3. Human Workflow Contract

The supported first-time workflow is:

```text
author a complete config candidate
  -> initialize the canonical config
  -> validate config
  -> inspect graph and resolved commands
  -> execute selection
  -> interpret report and exit code
  -> correct environment, config, or source under human control
  -> rerun the smallest affected scope
  -> run the final requested gate
```

The default path uses:

```sh
workspace-validator init --config <candidate-path>
workspace-validator config validate
workspace-validator list --tree
workspace-validator explain group <group-id>
workspace-validator explain suite <suite-id>
workspace-validator explain check <check-id>
workspace-validator validate <group-or-suite>
workspace-validator check <check-id>
```

Machine consumers use `--format=json`. Human output remains accessible without
requiring color and supports the declared color and presentation profiles. An
existing canonical configuration enters the workflow at `config validate`;
`init` is provisioning, not a replacement or edit command.

Do not change public schema versions unless a remaining item requires an
intentional public contract change with matching types, schemas, tests,
examples, and documentation.

## 4. Final Completion Workstreams

### 4.1 Complete Future-Destination Resolution

Accepted foundation:

- [x] Ordinary loading delegates path semantics to filesystem
      canonicalization.
- [x] Existing symlinks adjacent to `..` retain operating-system semantics.
- [x] Initialization resolution is isolated from the ordinary loader.
- [x] Paths such as `../link/..` resolve identically before and after
      installation.

Completed work:

- [x] Support `workspaceRoot: "."` when the canonical `.validation/` directory
      does not yet exist. The candidate must resolve to that future directory,
      just as the installed configuration does after initialization creates it.
- [x] Preserve equivalent current-contract forms, including repeated current
      directory components and the empty path currently accepted by ordinary
      loading and the versioned configuration schema.
- [x] Support relative paths that leave and then resolve back to the future
      `.validation/` directory when their installed filesystem interpretation is
      valid.
- [x] Reject paths whose required descendants will still not exist after the
      canonical parent is created.
- [x] Do not lexically collapse existing symlink components or alter ordinary
      configuration loading.
- [x] Test the same candidate with `.validation/` absent and pre-created, then
      load the installed bytes and assert identical workspace roots and suite
      directories.

Completed work:

- [x] Resolve an existing symlink whose target is the future canonical
      `.validation/` directory even though that target does not exist at
      candidate-validation time. For example, with
      `future-link -> .validation`, `workspaceRoot: "../future-link"` must have
      the same meaning before publication and after installation.
- [x] Preserve filesystem-aware traversal around that symlink. Paths that use
      the link and then `..` must resolve as the operating system resolves the
      installed path, rather than through lexical component collapse.
- [x] Keep genuinely broken links and links to descendants that publication
      does not create invalid. Recognizing the one future canonical directory
      must not make arbitrary missing targets virtual.
- [x] Keep candidate validation free of filesystem mutations. Do not create the
      future directory merely to make canonicalization pass, and do not change
      ordinary installed-configuration loading.
- [x] Add a Unix regression matrix that validates the exact same candidate with
      `.validation/` absent and present, installs it, and compares the resolved
      workspace root and suite directories. Cover both the direct symlink and a
      symlink followed by parent traversal, plus rejection of an unrelated
      dangling target.

The current configuration contract is the source of truth. Do not solve this
by newly rejecting `.` or another path form already accepted after installation
unless an explicit public contract change is separately requested.

### 4.2 Make Atomic Temporaries Collision-Tolerant

Accepted foundation:

- [x] Candidate bytes are fully written and synchronized before publication.
- [x] Publication is atomic and does not replace an existing destination.
- [x] Recoverable pre-publication failures remove their temporary artifact.
- [x] Controlled interruption tests prove that partial canonical bytes are not
      exposed and existing destinations are not overwritten.

Completed work:

- [x] Replace the single predictable temporary-path attempt with a
      collision-tolerant allocator. An existing temporary candidate must cause
      another safe name to be attempted rather than fail initialization.
- [x] Preserve `create_new` or equivalent exclusive creation; never truncate,
      reuse, or delete an artifact whose ownership is unknown.
- [x] Keep best-effort cleanup for artifacts created by the current attempt.
- [x] Ensure an abrupt interruption may leave an inert artifact but cannot make
      a later attempt fail, including after process identifier reuse.
- [x] Add a deterministic regression test that pre-creates the first temporary
      candidate, initializes successfully through another candidate, and leaves
      the pre-existing artifact untouched.
- [x] Preserve the current hard-link no-overwrite publication semantics and
      existing typed outcomes.

### 4.3 Make CLI Output Fallible

Accepted foundation:

- [x] Inspection commands already write through fallible `Write` interfaces.
- [x] JSON rendering itself is fallible and returns typed serialization errors.
- [x] Exit code `4` is documented as an internal execution, provisioning, or
      reporting failure.

Completed work:

- [x] Route schema output, initialization output, final human reports, and final
      JSON reports through fallible writers instead of `println!`.
- [x] Convert standard-output write failures for those outputs into
      `ValidatorError::Internal` and exit code `4` without panicking.
- [x] Keep standard error best-effort and panic-free when reporting a failure;
      failure to emit its diagnostic must not replace the selected exit code
      with Rust's panic code.
- [x] Preserve exactly one JSON document on successful machine-output paths and
      preserve the current human rendering byte-for-byte where practical.
- [x] Add real CLI tests that direct `validate --format=json` and
      `init --format=json` to a failing output sink and assert exit code `4`, no
      panic message, and no accidental second document.
- [x] Retain the existing inspection writer-failure test and rename it if needed
      so its scope is explicit rather than implying coverage of final reports.

Completed work:

- [x] Stop representing every error returned by `explain group`,
      `explain suite`, and `explain check` as `InvalidConfig`. Unknown IDs and
      invalid selections remain semantic failures with exit code `3`; failures
      from the output writer are internal reporting failures with exit code `4`.
- [x] Establish an explicit typed boundary between inspection resolution and
      inspection rendering, or an equivalent structure that cannot lose the
      error category in a shared `String`.
- [x] Preserve the current successful human output exactly and keep inspection
      free of configured process execution.
- [x] Add real CLI tests for `explain group`, `explain suite`, and
      `explain check` with a failing standard-output sink. Assert exit code `4`,
      an internal reporting diagnostic when standard error is available, and no
      panic. Retain coverage proving an unknown target still returns `3`.

## 5. Implementation Order

1. Complete future-directory symlink resolution and its pre-install/post-install
   equivalence tests.
2. Separate semantic inspection failures from writer failures and complete
   `explain` exit-code coverage.
3. Run targeted tests after each correction.
4. Repeat the complete final gate and update every remaining status marker in
   this plan.

## 6. Verification

### 6.1 Baseline Already Verified

- [x] `cargo fmt --all -- --check`
- [x] `cargo clippy --workspace --all-targets --locked -- -D warnings`
- [x] `cargo test --workspace --all-targets --locked`
- [x] `cargo test --doc --locked`
- [x] `cargo test --test outcome_fixtures --locked -- --ignored`
- [x] Rustdoc with warnings and missing documentation denied
- [x] `cargo package --allow-dirty --list`
- [x] `cargo package --allow-dirty --locked`
- [x] MSRV `1.87.0` workspace checks and tests
- [x] MSRV `1.87.0` check for `x86_64-pc-windows-gnu`

These checks describe the accepted baseline. They must remain passing, and the
complete gate must be repeated after the pending work.

### 6.2 Coverage To Add

- [x] Symlink-aware ordinary resolution when an existing symlink is adjacent to
      `..`.
- [x] Candidate and installed loading equivalence for `../link/..`.
- [x] Recoverable interrupted publication and a successful retry.
- [x] Structural inspection output for group, suite, and check contexts.
- [x] Human and JSON repository path parity for gating and informational modes.
- [x] Distributed examples loaded byte-for-byte from documented locations.
- [x] Real CLI exit-code translation for `0`, `1`, `2`, `3`, and `130`.
- [x] Candidate and installed loading equivalence for the future canonical
      directory itself, including `workspaceRoot: "."`.
- [x] Successful publication when the first exclusive temporary candidate
      already exists.
- [x] Final validation-report and initialization-result write failures translate
      to exit code `4` without panic.
- [x] Candidate and installed loading remain equivalent when an existing
      symlink points to the not-yet-created canonical `.validation/` directory,
      including parent traversal after the symlink.
- [x] An unrelated dangling symlink or a link to a descendant not created by
      initialization remains invalid.
- [x] Writer failures from `explain group`, `explain suite`, and
      `explain check` return exit code `4`; semantic selection failures continue
      to return `3`.

### 6.3 Final Gate To Repeat

- [x] Formatting and Clippy with warnings denied.
- [x] All regular tests and Rustdoc tests.
- [x] Ignored outcome fixtures.
- [x] Rustdoc with warnings and missing documentation denied.
- [x] Source package listing and package construction.
- [x] MSRV `1.87.0` checks and tests.
- [x] Checked Windows target.

## 7. Deliverable Status

- [x] Runtime and CLI foundation.
- [x] Deterministic configuration provisioning through `init`.
- [x] Checked-in configuration and report schemas.
- [x] Versioned human and JSON report contracts.
- [x] Human workflow documentation structure and representative examples.
- [x] Typed JSON repository mutation evidence.
- [x] Complete inspection context in every view.
- [x] Complete human repository mutation evidence.
- [x] Exact distributed examples and final-report exit-code coverage through
      `130`.
- [x] Complete future canonical-directory semantics during initialization,
      including an existing symlink whose target becomes valid at publication.
- [x] Collision-tolerant atomic temporary allocation.
- [x] Panic-free schema, initialization, and final-report output.
- [x] Correct exit-code `4` classification for writer failures from every
      `explain` command.
- [x] Final regression and release-readiness gate after the remaining changes.

## 8. Acceptance Criteria

- [x] A new user can configure and run the validator from documentation alone.
- [x] A user can install one fully authored config candidate without the tool
      inventing project-specific tools, checks, suites, or groups.
- [x] Config initialization preserves canonical filesystem semantics, never
      overwrites a different destination, and is idempotent for identical bytes.
- [x] Candidate validation and post-install loading resolve the same workspace
      and suite directories without changing ordinary symlink-aware semantics.
- [x] Interrupted initialization cannot expose partial canonical bytes or leave
      a temporary artifact that blocks the next attempt.
- [x] Inspection exposes planned operations without starting processes.
- [x] Every inspection view exposes all effective directories, parameters,
      commands, prerequisites, and dependencies relevant to its selection.
- [x] A human can identify exact repository mutations introduced by a run.
- [x] Repository path classifications agree between human and JSON output when
      mutation detection is gating or informational.
- [x] Human and JSON report contents agree for every result class.
- [x] JSON output can be captured byte-for-byte, validated against its schema,
      and hashed without removing terminal output.
- [x] Every nonzero exit code has one documented and tested meaning across
      execution, initialization, and inspection output failures.
- [x] Every distributed example validates byte-for-byte at its documented
      canonical location.
- [x] The validator does not read `.validation/policy.json`,
      `.validation/reports/`, or `.validation/persistence/` as execution input.
- [x] No Human Flow command requires an AI agent or skill.
- [x] The skill manifest is the only bundle-version and CLI-compatibility source.
- [x] Existing trusted configuration runs without hidden installation or
      workspace repair.
- [x] The complete Rust gate, outcome fixtures, package inspection, MSRV checks,
      and checked Windows target pass after all remaining changes.

## 9. Handoff To Phase 2

Phase 2 starts only after every unchecked item in this document is complete and
the Human Flow is independently usable. The accepted baseline remains the
foundation for source-backed educational guidance and agent routing.
