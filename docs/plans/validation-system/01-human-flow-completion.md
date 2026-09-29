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
- `[ ]` identifies the only implementation work that remains in this phase.

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

### 2.2 Remaining Scope

Only these five completion areas remain:

1. [ ] Preserve filesystem-aware semantics when `..` appears adjacent to a
       symlink, while keeping the future canonical destination used by `init`
       isolated from ordinary configuration loading.
2. [ ] Prove interrupted initialization cannot publish partial bytes, overwrite
       a destination, or leave a temporary artifact that blocks a retry.
3. [ ] Complete `explain check` and `explain group` with the same parameter,
       command, directory, prerequisite, and dependency detail already available
       through suite inspection.
4. [ ] Render concrete repository path classifications in human reports with
       semantic parity to JSON, including informational mutation detection and
       unavailable-provider evidence.
5. [ ] Close documentation and test gaps: canonical examples without repair,
       valid schema guidance, present-tense product references, and real CLI
       coverage for exit codes `4` and `130`.

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

## 4. Completion Workstreams

### 4.1 Filesystem-Aware Configuration Resolution

Accepted foundation:

- [x] Workspace and suite paths are bounded by the selected workspace.
- [x] Initialization validates candidate content for the future canonical
      destination.

Remaining work:

- [ ] Remove global lexical normalization that collapses `..` before existing
      symlink components are resolved.
- [ ] Resolve ordinary configuration paths with operating-system filesystem
      semantics. A path such as `link/..` must retain those semantics or be
      rejected; it must not be silently reinterpreted as though `link` were an
      ordinary directory.
- [ ] Introduce a dedicated initialization resolution context for the known
      future `.validation/` destination.
- [ ] Permit that context to derive the not-yet-created destination from the
      already canonical selected workspace while preserving real filesystem
      semantics for every existing component.
- [ ] Keep this virtual destination behavior out of the ordinary loader.
- [ ] Add regression tests for symlink components adjacent to `..`, destination
      equivalence, and ordinary loading after initialization.

### 4.2 Interrupted Atomic Initialization

Accepted foundation:

- [x] Publication uses atomic no-overwrite behavior.
- [x] Existing identical and conflicting destinations are classified correctly.
- [x] The typed initialization result reports canonical paths and resource
      outcomes.

Remaining work:

- [ ] Add a deterministic test seam at the publication boundary without adding
      runtime-only environment flags to the production interface.
- [ ] Cover interruption or injected write failure before publication.
- [ ] Assert that no partial canonical file becomes visible.
- [ ] Assert that no existing destination is overwritten.
- [ ] Assert that temporary artifacts are cleaned or cannot block the next
      initialization attempt.
- [ ] Keep the implementation atomic and no-overwrite on every supported
      platform boundary.

### 4.3 Inspection Parity

Accepted foundation:

- [x] Inspection commands do not execute configured programs.
- [x] Suite inspection exposes the complete effective execution context.

Remaining work:

- [ ] Make `explain check` show each suite invocation separately, including its
      effective directory, parameter bindings, resolved argument vector,
      prerequisites, and `dependsOn` relationship.
- [ ] Make `explain group` retain the reachable hierarchy while showing the
      suite context of every resolved command, including parameters and check
      dependencies.
- [ ] Render explicit empty states when the absence of parameters or dependencies
      affects interpretation.
- [ ] Share resolution and rendering data structures across group, suite, and
      check inspection so their semantics cannot drift.
- [ ] Add structural tests for populated and empty parameter and dependency
      relationships.

### 4.4 Human Repository Evidence

Accepted foundation:

- [x] `RepositoryReport` owns the classified mutation evidence.
- [x] JSON output exposes exact `introduced`, `removed`, and `changed` path sets.
- [x] Mutation gating and unavailable-provider states are typed.

Remaining work:

- [ ] Render `introduced`, `removed`, and `changed` paths under stable, explicit
      headings in human output.
- [ ] Keep observed paths visible when `detectMutations` is `false`; that option
      controls the gate result, not disclosure of evidence.
- [ ] Render an unavailable provider as its typed status and diagnostic rather
      than as an empty successful section.
- [ ] Derive human content directly from the same `RepositoryReport` used by
      JSON instead of replacing path evidence with only a generic gate reason.
- [ ] Add parity tests proving both formats contain identical classified path
      sets for enabled and informational mutation detection.
- [ ] Document that this evidence detects run-time mutation and does not certify
      repository origin or historical integrity.

### 4.5 Documentation, Examples, And Exit Codes

Accepted foundation:

- [x] The Human Flow has dedicated documentation, reference material, and
      representative examples.
- [x] Inspection and execution commands are distinct in the CLI and current
      documentation structure.
- [x] Skills and schemas remain tool-owned distribution resources.

Remaining work:

- [ ] Make every distributed example valid exactly as stored and at its
      documented canonical location.
- [ ] Remove test-side repairs such as rewriting `workspaceRoot` before example
      validation.
- [ ] Do not include a relative `$schema` URI in a copy-and-run example unless
      that exact schema is installed at the referenced location. Prefer omitting
      the optional field when no stable distributable URI exists.
- [ ] Remove implementation-phase wording from product documentation and write
      references entirely in terms of current commands and contracts.
- [ ] Document one unambiguous meaning for every CLI exit code.
- [ ] Add real CLI translation tests for internal failure `4` and interruption
      `130` through deterministic seams or controlled child processes.

## 5. Implementation Order

1. Correct ordinary path resolution and isolate initialization destination
   resolution.
2. Add the interrupted-publication seam and atomicity coverage.
3. Complete group and check inspection.
4. Complete human repository evidence and format parity.
5. Correct documentation, examples, and exit-code coverage.
6. Run targeted tests after each workstream, then repeat the complete final
   gate.

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

- [ ] Symlink-aware resolution when an existing symlink is adjacent to `..`.
- [ ] Candidate validation and installed loading produce equivalent roots and
      suite directories.
- [ ] Interrupted atomic publication and a successful retry afterward.
- [ ] Structural inspection output for group, suite, and check contexts.
- [ ] Human and JSON repository path parity for introduction, removal,
      modification, rename, copy, pre-dirty state, ignored files, and unavailable
      providers.
- [ ] Human mutation evidence with `detectMutations` enabled and disabled.
- [ ] Real CLI exit-code translation for `0`, `1`, `2`, `3`, `4`, and `130`.
- [ ] Distributed examples loaded byte-for-byte from their documented canonical
      locations.

### 6.3 Final Gate To Repeat

- [ ] Formatting and Clippy with warnings denied.
- [ ] All regular tests and Rustdoc tests.
- [ ] Ignored outcome fixtures.
- [ ] Rustdoc with warnings and missing documentation denied.
- [ ] Source package listing and package construction.
- [ ] MSRV `1.87.0` checks and tests.
- [ ] Checked Windows target.

## 7. Deliverable Status

- [x] Runtime and CLI foundation.
- [x] Deterministic configuration provisioning through `init`.
- [x] Checked-in configuration and report schemas.
- [x] Versioned human and JSON report contracts.
- [x] Human workflow documentation structure and representative examples.
- [x] Typed JSON repository mutation evidence.
- [ ] Filesystem-aware loader and isolated initialization destination semantics.
- [ ] Interruption-proof initialization coverage.
- [ ] Complete inspection context in every view.
- [ ] Complete human repository mutation evidence.
- [ ] Exact distributed examples and complete exit-code coverage.
- [ ] Final regression and release-readiness gate after the remaining changes.

## 8. Acceptance Criteria

- [ ] A new user can configure and run the validator from documentation alone.
- [x] A user can install one fully authored config candidate without the tool
      inventing project-specific tools, checks, suites, or groups.
- [ ] Config initialization preserves canonical filesystem semantics, never
      overwrites a different destination, and is idempotent for identical bytes.
- [ ] Candidate validation and post-install loading resolve the same workspace
      and suite directories without changing ordinary symlink-aware semantics.
- [ ] Interrupted initialization cannot expose partial canonical bytes or leave
      a temporary artifact that blocks the next attempt.
- [x] Inspection exposes planned operations without starting processes.
- [ ] Every inspection view exposes all effective directories, parameters,
      commands, prerequisites, and dependencies relevant to its selection.
- [ ] A human can identify exact repository mutations introduced by a run.
- [ ] Repository path classifications agree between human and JSON output when
      mutation detection is gating or informational.
- [ ] Human and JSON outputs agree for every result class.
- [x] JSON output can be captured byte-for-byte, validated against its schema,
      and hashed without removing terminal output.
- [ ] Every nonzero exit code has one documented and tested meaning.
- [ ] Every distributed example validates byte-for-byte at its documented
      canonical location.
- [x] The validator does not read `.validation/policy.json`,
      `.validation/reports/`, or `.validation/persistence/` as execution input.
- [x] No Human Flow command requires an AI agent or skill.
- [x] The skill manifest is the only bundle-version and CLI-compatibility source.
- [x] Existing trusted configuration runs without hidden installation or
      workspace repair.
- [ ] The complete Rust gate, outcome fixtures, package inspection, MSRV checks,
      and checked Windows target pass after all remaining changes.

## 9. Handoff To Phase 2

Phase 2 starts only after every unchecked item in this document is complete and
the Human Flow is independently usable. The accepted baseline remains the
foundation for source-backed educational guidance and agent routing.
