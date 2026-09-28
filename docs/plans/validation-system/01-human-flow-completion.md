# Phase 1: Human Flow Completion

## 1. Objective

Close the Human Flow as a complete product surface before adding AI operation.
A user must be able to author or review one configuration, inspect its execution
graph, run any supported selection, understand every outcome, and identify
repository mutations introduced by a validation run without relying on AI
components.

## 2. Entry Conditions

- The current configuration and report contracts are the implementation source
  of truth.
- The CLI executes shell-free program and argument vectors.
- Existing human and JSON reporting remain available.

Do not change public schema versions unless this phase identifies and
implements an intentional public contract change with matching types, schemas,
tests, examples, and documentation.

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
requiring color and supports the declared color and presentation profiles.
An existing canonical configuration enters the workflow at `config validate`;
`init` is provisioning, not a replacement or edit command.

## 4. Workstreams

### 4.1 Configuration Closure

- Verify JSON Schema and semantic validation cover every public field and
  reject unknown fields, invalid IDs, duplicate IDs, graph cycles, invalid
  dependencies, unresolved parameters, unsafe directories, and tool cycles.
- Preserve the ownership chain `tool -> check -> suite -> group`.
- Preserve suite ownership of working directories and contextual parameters.
- Preserve one active `workspaceRoot` filesystem boundary with symlink-escape,
  absolute-path, and parent-traversal rejection.
- Verify direct check execution remains valid when optional placeholders are
  omitted.
- Keep configuration declarative and free from implicit shell behavior.

### 4.2 Deterministic Initialization

- Add one extensible `workspace-validator init` command for provisioning
  consumer-owned resources through explicit capability flags.
- Phase 1 introduces `--config <candidate-path>`. The command requires at least
  one provisioning flag and accepts `--workspace <workspace-path>`; omission of
  `--workspace` means exactly the process current working directory.
- Resolve and canonicalize that workspace once. Do not discover another root
  from a parent directory, Git, or configuration content.
- Require the candidate to be a contained regular file and reject symlinks,
  parent traversal, and workspace escape.
- Validate the complete candidate as though its canonical location were
  `<workspace>/.validation/config.json`. Relative configuration semantics,
  including `workspaceRoot`, therefore resolve from the canonical destination
  rather than from the candidate's temporary location.
- Install the exact validated bytes through an atomic, no-overwrite operation.
  Reuse an existing destination only when its bytes are identical; reject an
  invalid or different existing destination as a conflict.
- Do not run tool preflight, checks, suites, groups, installation, or repair as
  part of initialization.
- Define one versioned `InitResult` shared by every provisioning capability. It
  contains the normalized workspace root, overall status, and one ordered entry
  per requested resource with kind, normalized canonical path, status, digest
  when available, and bounded diagnostics. Resource status is `created`,
  `reused`, `conflict`, `failed`, or `not_written`; overall status is `success`,
  `conflict`, `failed`, or `partial`.
- Derive human and JSON output from that same typed result. JSON mode emits
  exactly one document without progress or prose.
- Keep provisioning flags capability-owned and composable. A later capability
  extends this same command with another flag instead of adding a separate
  initialization namespace.

### 4.3 Inspection Closure

- Ensure `config validate` validates the entire document without running
  preflight or checks.
- Ensure `list --tree` communicates group reuse and hierarchy deterministically.
- Ensure every `explain` command shows the exact relevant program, argument
  vector, working directory, parameters, prerequisites, and dependency
  relationship without executing it.
- Add golden or structural tests that keep inspection output aligned with the
  configuration contract.

### 4.4 Repository Mutation Evidence

- Preserve the optional repository provider as execution-scoped evidence.
- Capture repository status and content fingerprints immediately before and
  after a validation run through the configured provider.
- Distinguish repository changes that precede the run from mutations introduced
  during that run.
- Keep the observation bounded, deterministic, and represented in both human
  and JSON reports.
- When the repository provider is unavailable, report that condition through
  the existing typed result.
- Document that this evidence detects run-time mutations; it does not certify
  repository origin or provide historical tamper detection.

### 4.5 Execution Closure

- Verify tool preflight, version parsing, and version requirements produce
  distinct observable failures.
- Verify deterministic ordering and one-run deduplication of shared suites and
  groups.
- Verify timeout, interruption, process-tree termination, output truncation,
  and dependency propagation on every supported platform boundary.
- Verify repository mutation evidence distinguishes pre-existing changes from
  mutations introduced during validation.
- Ensure the executor never installs tools, invokes an implicit shell, or
  repairs a workspace.

### 4.6 Report And Exit-Code Closure

- Treat the versioned JSON report as the canonical execution record.
- Verify reports include selection, tools, groups, suites, checks, commands,
  context, durations, statuses, exit codes, timeout state, truncation state,
  useful diagnostics, repository observations, and aggregate summary.
- Preserve the distinct meanings of pass, fail, blocked, skipped, invalid
  configuration or usage, internal failure, and interruption.
- Confirm human output and JSON output derive from the same completed result
  without semantic disagreement.
- Ensure JSON mode writes exactly one complete report document to standard
  output without prose, progress rendering, or terminal control bytes. These
  exact bytes are the digest source when a later AI-Engineering process stores
  the report.
- Document that a pass means all counted evidence in the selected graph passed;
  it is not a certification of complete product correctness.

### 4.7 Human Documentation

- Add `docs/validation/flows/human/README.md` as the focused Human Flow document
  covering configuration, inspection, execution, result interpretation, and
  narrow revalidation.
- Place CLI, initialization, configuration, report, schema, and exit-status
  reference material under `docs/validation/reference/` so it remains usable by
  every flow.
- Keep the root README as the concise entry point and link to deeper contracts.
- Provide complete examples for a single crate, a heterogeneous workspace, and
  a workspace with shared suites across groups.
- Explain which commands inspect only and which commands execute configured
  programs.
- Explain that checked-in configuration is trusted executable policy and must
  be reviewed like code.
- Keep configuration and report schemas owned and distributed by the tool.
  Schema inspection commands must not require consumers to track copied schema
  files in their workspaces.
- Keep `manifest.json` as the single source for the skill bundle version, source
  crate version, and compatible CLI range. Workflow documents must not duplicate
  those values.
- Align skill bundle tests with that ownership: verify manifest values,
  entrypoint routing, declared resources, and document completeness without
  requiring repeated version headers in each workflow document.

## 5. Tests

Add or complete tests for:

- every CLI command and exit code;
- `init` with no capability, one config candidate, an explicit or omitted
  workspace root, identical reuse, conflicting or invalid destinations,
  canonical-destination-relative resolution, symlink escape, and interrupted
  atomic writes;
- valid and invalid configuration fixtures;
- nested and convergent group graphs;
- contextual parameter expansion without shell interpretation;
- working-directory containment and symlink escape;
- pass, fail, blocked, skipped, timeout, interruption, and internal-error
  reports;
- stdout and stderr truncation;
- repository mutation introduction, removal, and modification;
- plain, colored, high-contrast, color-vision, and low-vision output;
- cross-platform process-tree termination;
- source package contents and generated schemas;
- skill manifest metadata, entrypoint and workflow routing, declared resources,
  and absence of duplicated version metadata in workflow documents;
- repository mutation evidence for clean, pre-dirty, modified, added, removed,
  renamed, copied, ignored, and unavailable-provider cases.

Run the ignored outcome fixtures whenever execution-state or reporting behavior
is touched.

## 6. Deliverables

- closed runtime and CLI contracts;
- deterministic config provisioning through the shared `init` command;
- aligned checked-in JSON Schemas;
- human workflow documentation;
- representative configuration examples;
- complete deterministic outcome fixtures.

## 7. Acceptance Criteria

- [ ] A new user can configure and run the validator from documentation alone.
- [ ] A new user can install one fully authored config candidate without the
      tool inventing project-specific tools, checks, suites, or groups.
- [ ] Config initialization validates canonical path semantics, never
      overwrites a different destination, and is idempotent for identical
      bytes.
- [ ] A human can identify repository mutations introduced by a validation run
      from that run's report.
- [ ] Inspection exposes the planned operation without starting processes.
- [ ] Human and JSON outputs agree for every result class.
- [ ] JSON output can be captured byte-for-byte, validated against the report
      schema, and hashed without stripping terminal output.
- [ ] Every nonzero exit code has one documented, tested meaning.
- [ ] The validator does not read `.validation/policy.json`,
      `.validation/reports/`, or `.validation/persistence/` as execution input.
- [ ] No human command requires an AI agent or skill.
- [ ] The skill manifest is the only bundle-version and CLI-compatibility source,
      and bundle tests do not require those values to be copied into workflow
      documents.
- [ ] Existing trusted configuration can be run without hidden installation or
      mutation.
- [ ] The complete Rust validation gate, outcome fixtures, package inspection,
      and MSRV checks pass.

## 8. Handoff To Phase 2

Phase 2 starts only after the Human Flow is independently usable and its
public contracts are stable enough to support source-backed educational
guidance and agent routing.
