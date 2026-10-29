# A00.2: Launcher-Primary CLI Interface Realignment

## 1. Status And Authority

`Approved target; implementation pending`.

This meta-plan makes the `liknon` launcher the primary user-facing interface
without removing or merging the three independent capability CLIs.

Repository code remains the source of truth for implemented behavior. For
target direction, this document supersedes active text that presents direct
capability commands and launcher routes as equally promoted user workflows.

[A00.1](../A00.1-unified-launcher-plan-realignment/README.md) remains
historical evidence. The implemented B04 four-executable topology remains the
approved technical architecture.

This is one small cross-plan realignment. It does not authorize a new product
topology or a sequence of additional corrective plans.

## 2. Approved Contract

The suite continues to contain four physical executables:

| Executable | Responsibility |
| --- | --- |
| `liknon` | Capability-free launcher and primary user entry point |
| `liknon-validator` | Independent validation CLI |
| `liknon-manager` | Independent resource-management CLI |
| `liknon-tool` | Independent local-resource CLI |

The primary user-facing routes are:

```text
liknon validator <args...>
liknon v <args...>
liknon manager <args...>
liknon m <args...>
liknon tool <args...>
liknon t <args...>
```

The launcher continues to resolve the exact sibling executable from its own
installed suite, remove only the selected namespace, preserve arguments and
process semantics, and return the selected CLI's outcome.

## 3. Meaning Of Primary Interface

Documentation, onboarding, examples, workflows, and ordinary user guidance
must teach the launcher routes first. A user should not need to choose among
four command names to use the suite normally.

The direct commands remain real and operational because they are the
independent processes used by the launcher. They retain:

- their own parsers, focused help, version output, errors, and exit codes;
- their own domain implementation and tests;
- their own process and dependency boundaries;
- direct conformance tests needed to distinguish component failures from
  launcher failures;
- availability for advanced diagnostics and explicitly constrained
  integrations.

Direct invocation is therefore a component-level interface, not the promoted
everyday entry point. This distinction is documentation and product hierarchy;
it is not an attempt to make installed executable files impossible to invoke.

Cargo installs all package-owned binaries into the same binary directory.
Preventing direct execution would require a different private executable
layout or an artificial runtime restriction. Neither is part of this plan.

## 4. Architecture That Must Not Change

This realignment does **not**:

- remove `liknon-validator`, `liknon-manager`, or `liknon-tool`;
- convert capability execution to in-process library calls;
- merge the three CLIs into the launcher;
- remove direct component help, version, or tests;
- change `liknon-cli` from four Cargo binary targets;
- change the one-archive-per-target topology;
- change manifests from four executable digests;
- change atomic installation, update, rollback, or uninstall across the suite;
- introduce compact executable aliases such as `likv`, `likm`, or `likt`;
- create compatibility shims or deprecated commands;
- change the B04 or B06 release identity.

Validator, Manager, and Tool remain independent CLIs. The launcher remains a
fourth, capability-free process that invokes exactly one of them.

## 5. Required Plan Adjustments

Review active normative plans and distinguish two concerns:

1. **Technical topology:** four physical executables, direct component
   conformance, sibling verification, and one atomic suite distribution.
2. **Primary product interface:** ordinary use through `liknon validator`,
   `liknon manager`, or `liknon tool`.

Apply the following focused adjustments:

| Plan area | Required treatment |
| --- | --- |
| Target-state summary | Identify `liknon` routes as primary while preserving the four-executable architecture |
| Root roadmap | Describe one launcher-primary suite with three independent capability CLIs |
| A01 and A02 | Preserve implemented package, process, and shared-UI boundaries |
| B01 | Use `liknon validator` in ordinary user examples; retain direct Validator conformance evidence |
| B02 | Use `liknon manager` in ordinary user examples; retain direct Manager conformance evidence |
| B03 | Use `liknon tool` in ordinary user examples; retain direct Tool conformance evidence |
| B04 | Preserve package, routing, sibling, archive, installer, and release contracts; clarify interface priority |
| B05 | Preserve package `liknon` for future programmatic APIs |
| B06 | Preserve the exact four-executable suite candidate and coordinated release contract |
| C01-C03 | Use launcher routes in workflow-facing instructions while retaining exact component identity in evidence |
| D01 | Test both the primary launcher route and direct component boundaries for diagnosis |
| E01 | Present one suite download and teach the launcher as its entry point |

Historical plans remain unchanged. Current-state documents change only when
their statements are inaccurate about repository behavior; the current
four-executable implementation itself is not being replaced.

## 6. Required Code And Help Adjustments

No routing redesign is required. Code changes are limited to places where the
current user experience incorrectly promotes direct commands over the
launcher.

The final help contract is:

- `liknon --help` briefly presents Validator, Manager, and Tool namespaces;
- `liknon <namespace> --help` reaches the corresponding component help;
- each direct executable keeps its own focused `--help` and `--version`;
- root help does not duplicate complete component references;
- errors produced before dispatch belong to `liknon`;
- errors produced after dispatch remain owned by the selected component.

If routed help displays the physical component command name, documentation may
identify it as the executing component. This plan does not require environment
markers, hidden arguments, help synthesis, or in-process parser composition
merely to rewrite that display name.

## 7. Documentation Rules

Maintained user documentation should normally show:

```text
liknon validator ...
liknon manager ...
liknon tool ...
```

Direct names may appear when documenting:

- suite internals;
- component-specific diagnostics;
- executable inventory and integrity;
- direct conformance testing;
- process isolation and signal behavior;
- manifest, archive, installer, or provenance records.

Do not describe direct commands as removed, deprecated, unsupported, or legacy.
Do not advertise them as alternative onboarding paths either. They are stable
suite components whose normal orchestration belongs to `liknon`.

## 8. Verification

The realignment must preserve existing B04 evidence and add only the checks
needed for interface priority:

- `liknon validator`, `liknon manager`, and `liknon tool` remain functional;
- `v`, `m`, and `t` remain parser aliases, not executable files;
- each launcher route reaches the expected sibling executable;
- direct component tests remain green;
- launcher and direct routes remain semantically equivalent;
- source installation still installs exactly four executables;
- native archives and manifests still contain exactly four executable roles;
- maintained onboarding examples use launcher routes;
- no active plan accidentally requires in-process composition or one physical
  executable.

Run the repository validation gates after implementation:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --all-targets --locked
cargo test --doc --locked
```

Run package and native-distribution checks only when affected files change.

## 9. Acceptance Criteria

- [ ] The suite still contains `liknon`, `liknon-validator`,
  `liknon-manager`, and `liknon-tool`.
- [ ] Validator, Manager, and Tool remain independent CLI processes.
- [ ] `liknon` remains capability-free and dispatches only to exact sibling
  executables.
- [ ] Ordinary documentation and examples use launcher namespaces first.
- [ ] Direct commands remain available for component operation, diagnosis,
  testing, and integrity verification without being promoted as onboarding
  alternatives.
- [ ] No binary target, archive entry, executable digest, installer operation,
  or rollback guarantee is removed.
- [ ] No in-process capability composition is introduced.
- [ ] B01-B03 behavior and B04 distribution gates remain green.
- [ ] Active plans consistently distinguish primary interface from technical
  executable topology.

## 10. Exit

A00.2 exits when the repository still ships the same four-executable suite,
all three capability CLIs remain independently testable and executable, and
the maintained product experience consistently presents `liknon` namespaces
as the normal way to use them.
