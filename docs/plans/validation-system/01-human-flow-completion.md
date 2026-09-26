# Phase 1: Human Flow Completion

## 1. Objective

Close the human-operated workflow as a complete product surface before adding
AI orchestration. A user must be able to author or review one configuration,
inspect its execution graph, run any supported selection, understand every
outcome, and preserve repository integrity without relying on AI persistence.

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

### 4.3 Execution Closure

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

### 4.4 Report And Exit-Code Closure

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
  exact bytes are the digest source when a later persistent AI workflow stores
  the report.
- Document that a pass means all counted evidence in the selected graph passed;
  it is not a certification of complete product correctness.

### 4.5 Human Documentation

- Add a focused human workflow document covering configuration, inspection,
  execution, result interpretation, and narrow revalidation.
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

### 4.6 Optional Point-In-Time AI Assistance

The bundled skill continues to support isolated requests such as explaining a
failure or editing configuration. This does not invoke a persistent AI process
or write AI context. Verify that the existing `run`, `triage`, `config`, and
`audit` references:

- match the current CLI exactly;
- require structured reports for machine interpretation;
- preserve user approval for installation and mutation;
- avoid duplicate execution and unchanged-failure loops;
- remain usable without `.validation/persistence/`.

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
- source package contents and generated schemas.

Run the ignored outcome fixtures whenever execution-state or reporting behavior
is touched.

## 6. Deliverables

- closed runtime and CLI contracts;
- aligned checked-in JSON Schemas;
- human workflow documentation;
- representative configuration examples;
- updated operational skill references;
- complete deterministic outcome fixtures.

## 7. Acceptance Criteria

- [ ] A new user can configure and run the validator from documentation alone.
- [ ] Inspection exposes the planned operation without starting processes.
- [ ] Human and JSON outputs agree for every result class.
- [ ] JSON output can be captured byte-for-byte, validated against the report
      schema, and hashed without stripping terminal output.
- [ ] Every nonzero exit code has one documented, tested meaning.
- [ ] The validator does not read `.validation/persistence/`.
- [ ] No human command requires an AI agent or skill.
- [ ] Existing trusted configuration can be run without hidden installation or
      mutation.
- [ ] The complete Rust validation gate, outcome fixtures, package inspection,
      and MSRV checks pass.

## 8. Handoff To Phase 2

Phase 2 starts only after the human workflow is independently usable and its
public contracts are stable enough to support source-backed educational
guidance and agent routing.
