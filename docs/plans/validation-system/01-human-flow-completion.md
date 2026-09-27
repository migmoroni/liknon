# Phase 1: Human Flow Completion

## 1. Objective

Close the Human Flow as a complete product surface before adding AI operation.
A user must be able to author or review one configuration,
inspect its execution graph, run any supported selection, understand every
outcome, and verify workspace integrity against an explicit hash baseline
without relying on AI persistence.

## 2. Entry Conditions

- The current configuration and report contracts are the implementation source
  of truth.
- The CLI executes shell-free program and argument vectors.
- Existing human and JSON reporting remain available.

Do not change public schema versions unless this phase identifies and
implements an intentional public contract change with matching types, schemas,
tests, examples, and documentation.

## 3. Human Workflow Contract

The supported workflow is:

```text
author config
  -> validate config
  -> initialize or verify the integrity baseline
  -> inspect graph and resolved commands
  -> execute selection
  -> interpret report and exit code
  -> correct environment, config, or source under human control
  -> rerun the smallest affected scope
  -> run the final requested gate
```

The default path uses:

```sh
workspace-validator config validate
workspace-validator integrity init
workspace-validator integrity check
workspace-validator integrity diff
workspace-validator integrity accept
workspace-validator list --tree
workspace-validator explain group <group-id>
workspace-validator explain suite <suite-id>
workspace-validator explain check <check-id>
workspace-validator validate <group-or-suite>
workspace-validator check <check-id>
```

Machine consumers use `--format=json`. Human output remains accessible without
requiring color and supports the declared color and presentation profiles.

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

### 4.2 Inspection Closure

- Ensure `config validate` validates the entire document without running
  preflight or checks.
- Ensure `list --tree` communicates group reuse and hierarchy deterministically.
- Ensure every `explain` command shows the exact relevant program, argument
  vector, working directory, parameters, prerequisites, and dependency
  relationship without executing it.
- Add golden or structural tests that keep inspection output aligned with the
  configuration contract.

### 4.3 Workspace Integrity State

- Add `.validation/state.json` as the versioned shared workspace-state contract
  used by deterministic integrity commands, humans, and both AI flows.
- Keep the tool-owned `integrity` namespace separate from the optional
  AI-owned `aiEngineering` namespace. Integrity commands validate and preserve
  the AI namespace but never interpret it as executable input.
- Resolve the integrity scope deterministically from the validated
  configuration, including explicit path inclusion, exclusion, workspace-root,
  symlink, file-size, and file-count rules.
- Always exclude `.validation/state.json` itself from its hash baseline. Dynamic
  report, persistence, cache, and build paths are excluded unless the validated
  integrity scope explicitly includes them.
- Hash raw file bytes with SHA-256, normalize contained relative paths, sort
  entries deterministically, and record the resolved scope and aggregate
  baseline digest.
- Define `integrity init` as explicit creation of the first baseline,
  `integrity check` as comparison without baseline promotion, `integrity diff`
  as structured inspection of recorded changes, and `integrity accept` as the
  only command that promotes the inspected state to the new baseline.
- Let `integrity check` atomically record its last inspection, including added,
  modified, removed, and hash-equivalent renamed paths, while leaving source
  files and baseline hashes unchanged.
- Protect state updates with a lock or compare-and-swap revision so concurrent
  human or agent sessions cannot silently overwrite a newer state.
- Produce human and versioned JSON output for every integrity command. Missing,
  invalid, conflicting, or uninitialized state remains distinct from a clean or
  changed state.
- Document that hashes verify equality against the recorded baseline; they do
  not authenticate origin when an actor can modify both workspace files and the
  baseline.

### 4.4 Execution Closure

- Verify tool preflight, version parsing, and version requirements produce
  distinct observable failures.
- Verify deterministic ordering and one-run deduplication of shared suites and
  groups.
- Verify timeout, interruption, process-tree termination, output truncation,
  and dependency propagation on every supported platform boundary.
- Verify repository integrity distinguishes pre-existing state from mutations
  introduced during validation.
- Ensure the executor never installs tools, invokes an implicit shell, or
  repairs a workspace.

### 4.5 Report And Exit-Code Closure

- Treat the versioned JSON report as the canonical execution record.
- Verify reports include selection, tools, groups, suites, checks, commands,
  context, durations, statuses, exit codes, timeout state, truncation state,
  useful diagnostics, repository state, and aggregate summary.
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

### 4.6 Human Documentation

- Add `docs/validation/flows/human/README.md` as the focused Human Flow document
  covering configuration, inspection, execution, result interpretation, and
  narrow revalidation.
- Place CLI, configuration, report, schema, and exit-status reference material
  under `docs/validation/reference/` so it remains usable by every flow.
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

## 5. Tests

Add or complete tests for:

- every CLI command and exit code;
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
- integrity initialization, clean checks, added, modified, removed, and renamed
  paths, explicit baseline acceptance, and uninitialized state;
- deterministic integrity output across supported hosts;
- state path traversal, symlink escape, oversized scope, malformed state,
  interrupted writes, revision conflicts, and concurrent updates;
- preservation of the AI-owned state namespace without interpreting it.

Run the ignored outcome fixtures whenever execution-state or reporting behavior
is touched.

## 6. Deliverables

- closed runtime and CLI contracts;
- aligned checked-in JSON Schemas;
- versioned workspace-state schema and integrity commands;
- human workflow documentation;
- representative configuration examples;
- complete deterministic outcome fixtures.

## 7. Acceptance Criteria

- [ ] A new user can configure and run the validator from documentation alone.
- [ ] A human can initialize, inspect, compare, and explicitly accept workspace
      integrity without an AI agent.
- [ ] `integrity check` never modifies workspace source or promotes its observed
      hashes to the baseline.
- [ ] Concurrent state updates fail explicitly instead of losing a newer write.
- [ ] Inspection exposes the planned operation without starting processes.
- [ ] Human and JSON outputs agree for every result class.
- [ ] JSON output can be captured byte-for-byte, validated against the report
      schema, and hashed without stripping terminal output.
- [ ] Every nonzero exit code has one documented, tested meaning.
- [ ] The validator does not read `.validation/policy.json`,
      `.validation/reports/`, or `.validation/persistence/` as execution input.
- [ ] The execution planner never treats `.validation/state.json` as a command
      source; only integrity commands interpret its tool-owned integrity data.
- [ ] No human command requires an AI agent or skill.
- [ ] Existing trusted configuration can be run without hidden installation or
      mutation.
- [ ] The complete Rust validation gate, outcome fixtures, package inspection,
      and MSRV checks pass.

## 8. Handoff To Phase 2

Phase 2 starts only after the Human Flow is independently usable and its
public contracts are stable enough to support source-backed educational
guidance and agent routing.
