# Workspace Validator Competitive Positioning And Readiness

## 1. Purpose

This document evaluates the current `workspace-validator` product, compares it
with four established alternatives, and defines the evidence required for the
project to become a validation-governance layer over specialized validation
systems.

The assessment is a strategic guide, not a public runtime contract. Public
behavior remains defined by the CLI, schemas, Rust API, and reference
documentation. The competitive research snapshot is dated **2026-09-30** and
must be reviewed when a material competitor capability or project release
changes.

The assessment distinguishes three different questions:

1. **Is the architectural idea coherent and differentiated?**
2. **How complete is the implemented product today?**
3. **What must be proven before the intended market position can be claimed?**

Planned behavior is never scored as implemented behavior.

## 2. Product Category

The intended category is:

> A deterministic validation-governance control plane for heterogeneous
> workspaces, operated directly by humans or through policy-governed AI flows.

The project does not need to become a linter catalog, package manager,
container runtime, distributed CI service, IDE analysis daemon, or autonomous
code-repair platform. It coordinates tools that already solve those problems.

Its long-term responsibility is to govern:

- what validation evidence a workspace requires;
- how external validators are composed and invoked;
- which exact configuration produced a result;
- how outcomes and repository effects are represented;
- how humans and agents may interpret and act on evidence;
- whether passing evidence still covers the changing domain.

A concise positioning statement is:

> `workspace-validator` does not provide every validator. It governs how
> validation is designed, executed, evidenced, interpreted, and evolved by
> humans and agents.

## 3. Executive Assessment

| Assessment | Score | Interpretation |
| --- | ---: | --- |
| Architectural thesis | **8.5/10** | Coherent, relevant, and meaningfully differentiated, but its strongest differentiation still needs implementation and empirical validation. |
| Implemented product today | **6.8/10** | A strong deterministic local runner and Human Flow, with the governance and validation-engineering layers still planned. |
| Evidence-based target | **8.7/10** | Achievable after the planned flows, external-runner conformance, structured evidence, and agent evaluations are implemented and verified. |

The earlier architectural score of `8.5/10` evaluates the design direction. It
must not be read as a claim that the current `0.1.0` implementation already
delivers the complete AI-governed system.

The current implementation is already stronger than an ordinary task wrapper
in deterministic configuration, execution safety, report contracts,
repository mutation evidence, accessibility, and cross-platform verification.
Its present limitation is product breadth: the implemented surface ends at a
well-defined Human Flow plus an initial agent skill. The capabilities that make
the category distinctive are largely represented by the remaining validation
system plans.

## 4. Scoring Method

### 4.1 Scale

Each criterion uses the same scale:

| Score | Meaning |
| ---: | --- |
| `0-2` | Absent, conceptual, or represented only by early scaffolding |
| `3-4` | Partial implementation with important product gaps |
| `5-6` | Useful and credible, but not yet complete or broadly proven |
| `7-8` | Strong production-oriented design and implementation |
| `9` | Leading capability with extensive verification and clear operational evidence |
| `10` | Category-defining, broadly adopted, and proven under diverse real-world use |

The weighted score is calculated as:

```text
sum(criterion score * criterion weight) / 100
```

The target score is not a promise. It is the score expected only when the
listed evidence exists.

### 4.2 Weighted Criteria

| Criterion | Weight | Current | Target | Current evidence and principal gap |
| --- | ---: | ---: | ---: | --- |
| Category clarity and scope discipline | 7 | 8.5 | 9.0 | The project is explicitly project-agnostic and separates execution from validators. Public positioning still needs real integration cases. |
| Deterministic execution correctness | 12 | 8.8 | 9.2 | Strict preflight, ordered planning, deduplication, timeouts, cancellation, bounded output, and process-tree termination exist. Broader operational use remains to be proven. |
| Declarative composition and inspectability | 10 | 8.5 | 9.0 | The `tool -> check -> suite -> group` model, placeholders, DAG validation, `list`, and `explain` are implemented. Larger heterogeneous configurations need usability evidence. |
| Safety and trust boundaries | 10 | 8.2 | 8.8 | Shell-free execution, path containment, symlink checks, explicit trust documentation, and Git-visible mutation detection exist. This is not an OS sandbox and must never be marketed as one. |
| Evidence and reporting contracts | 9 | 8.0 | 9.2 | Human output and versioned JSON reports derive from typed results. Cross-run provenance, persistent evidence, and external report interoperability remain planned or undefined. |
| Human experience and accessibility | 7 | 9.0 | 9.2 | Plain output, independent color and presentation profiles, color-vision palettes, low-vision spacing, and semantic labels are implemented. Real accessibility feedback remains valuable. |
| Heterogeneous integration and governance readiness | 10 | 7.2 | 9.0 | Any host executable can be modeled without an implicit shell. Named external-runner conformance fixtures and richer evidence handoff do not yet exist. |
| Execution efficiency and scale | 8 | 4.5 | 6.5 | Execution is deterministic but local and sequential, without native cache, affected analysis, or distribution. The target intentionally delegates much of this work. |
| Validation knowledge and coverage engineering | 8 | 2.0 | 8.8 | The knowledge foundation, domain model, sensorium, and drift methodology are planned but not implemented. |
| AI interface and human-owned governance | 8 | 3.0 | 9.0 | A versioned skill router and bounded references exist. The decision policy, AI-Tool Flow, AI-Engineering Flow, and process persistence remain planned. |
| Internal engineering and release quality | 6 | 8.5 | 9.0 | Three-platform tests, MSRV, warning-free Rustdoc, audit, deny, package isolation, schemas, and fixtures are present. The planned system must preserve this standard. |
| Ecosystem, adoption, and external proof | 5 | 3.0 | 6.5 | The crate is young, with no demonstrated integration catalog, adoption evidence, or long operational history. This score cannot be raised by implementation alone. |
| **Weighted total** | **100** | **6.8** | **8.7** | The target depends mainly on governance, evidence, coverage engineering, and external proof rather than a larger execution engine. |

## 5. Criterion Analysis

### 5.1 Category Clarity And Scope Discipline

**Current strengths**

- The runtime is explicitly project-agnostic.
- Consumer-specific commands stay in consumer configuration.
- The trust model states that configuration is trusted executable input.
- The validator does not install dependencies, intentionally repair source, or
  claim to sandbox configured programs.

**Target evidence**

- Publish integrations where Dagger, Nx, Trunk, and MegaLinter remain the
  specialized executors while `workspace-validator` owns the outer validation
  contract.
- Demonstrate that adding a specialized validator does not require embedding
  its domain logic in the core crate.

### 5.2 Deterministic Execution Correctness

**Current strengths**

- The whole configuration is validated before configured execution begins.
- Tool requirements and versions are checked explicitly.
- Programs receive argument vectors without an implicit shell.
- Shared suites and groups are deduplicated for one run.
- Dependency propagation, timeout, cancellation, interruption, bounded output,
  and descendant-process termination have explicit contracts.

**Target evidence**

- Maintain the same semantics across Linux, macOS, and Windows as the product
  surface grows.
- Add integration fixtures that prove external orchestrators receive the exact
  arguments, directory, environment assumptions, and lifecycle outcomes
  described by configuration.

### 5.3 Declarative Composition And Inspectability

**Current strengths**

- Tools own executable discovery and version requirements.
- Checks own reusable command templates.
- Suites own parameters, dependencies, and working directories.
- Groups compose suites and groups into an ordered DAG.
- Inspection commands expose effective commands before execution.

**Target evidence**

- Validate the model against large polyglot workspaces without turning the
  configuration into an opaque maintenance burden.
- Let AI assistance improve configuration authoring without making AI a
  prerequisite for understanding or running the configuration.

### 5.4 Safety And Trust Boundaries

**Current strengths**

- Absolute escapes, parent traversal, missing directories, files where
  directories are required, and covered symlink escapes are rejected.
- Initialization validates exact candidate bytes and publishes atomically.
- Repository evidence distinguishes pre-existing state from introduced,
  removed, and changed paths.
- The documentation correctly avoids treating shell-free execution as a
  sandbox.

**Target evidence**

- Implement and adversarially test the human-owned AI decision policy.
- Preserve a strict distinction between enforceable CLI guarantees and
  behavioral instructions supplied to agents.
- Verify policy behavior across multiple agent implementations rather than one
  model or one client.

### 5.5 Evidence And Reporting Contracts

**Current strengths**

- Human and JSON renderers consume the same typed completed results.
- JSON output is versioned and free of progress text or terminal control bytes.
- Tools, groups, suites, checks, and the repository gate remain structurally
  distinct.
- Standard output and standard error are bounded independently.

**Target evidence**

- Implement exact report persistence and immutable analytical artifacts from
  the validation-system plans.
- Bind persisted evidence to explicit paths and digests.
- Establish an external-evidence strategy using real integrations. Start with
  exit status and bounded output; add structured formats such as SARIF, JUnit,
  or provider JSON only when concrete consumers justify a versioned contract.

### 5.6 Human Experience And Accessibility

**Current strengths**

- The default output does not depend on color.
- Color-vision palettes and high contrast are separate from the low-vision
  presentation profile.
- Status, hierarchy, and node kind remain textual.
- Human reports distinguish execution progress from final results.

**Target evidence**

- Test the output with real users and common terminal combinations.
- Preserve equivalent information in plain, colored, low-vision, and JSON
  modes as report capabilities expand.

### 5.7 Heterogeneous Integration And Governance Readiness

**Current strengths**

- Any locally available executable can participate as a tool and check.
- Tool versions, arguments, dependencies, and working directories are explicit.
- The runner does not assume one language, package manager, or monorepo model.

**Current limitation**

The validator can already launch the four compared systems, but it sees them
primarily as processes with status and bounded output. That is enough for
top-level orchestration, but not yet enough for rich cross-tool evidence
analysis.

**Target evidence**

- Add maintained configuration examples and conformance fixtures for Dagger,
  Nx, Trunk, and MegaLinter.
- Record enough provenance to identify which external tool version and exact
  invocation produced each result.
- Decide structured evidence ingestion from integration evidence, not from a
  speculative universal adapter abstraction.

### 5.8 Execution Efficiency And Scale

**Current strengths**

- Planning is deterministic and repeated suites execute once per run.
- A caller can select a group, suite, or check for narrow validation.

**Current limitation**

The runtime does not provide native parallel execution, task-result caching,
affected-project analysis, remote cache, or distributed workers.

**Target boundary**

The project should add native optimization only when governance correctness
requires it or measurements justify it. Dagger and Nx can supply cache,
parallelism, and distribution beneath a governed check. Reimplementing their
engines would weaken the product focus.

### 5.9 Validation Knowledge And Coverage Engineering

This is the largest distinction between current and target capability.

The planned shared knowledge and AI-Engineering processes introduce:

- explicit domain consequences and invariants;
- a sensorium mapping risks to evidence;
- reconciliation between expected evidence and configured IDs;
- incident diagnosis from canonical reports;
- drift analysis even when every configured check passes.

These capabilities must be implemented, sourced, and evaluated before the
project claims to engineer coverage rather than merely execute configured
checks.

### 5.10 AI Interface And Human-Owned Governance

**Current strengths**

- The skill bundle is versioned separately from the CLI contract.
- The router loads focused execution, configuration, triage, or audit guidance.
- The CLI remains usable without an agent.

**Target evidence**

- Implement the Human Flow, AI-Tool Flow, and AI-Engineering Flow as separate
  boundaries.
- Enforce the rule that the human exclusively creates and mutates the decision
  policy.
- Test policy classification, free-text human decisions, stop outcomes,
  persistence authority, and process continuation.
- Evaluate CLI-only discovery, documentation-assisted direct CLI use, AI-Tool,
  and AI-Engineering end to end across multiple models.

### 5.11 Internal Engineering And Release Quality

The current repository already has a strong internal baseline:

- tests on Linux, macOS, and Windows;
- explicit Rust `1.87` MSRV verification;
- formatting and Clippy with warnings denied;
- warning-free Rustdoc with missing documentation denied;
- dependency advisory, license, source, and duplicate checks;
- isolated package construction and installed-binary smoke tests;
- committed JSON Schemas and exact schema-version handling.

This criterion is intentionally separate from ecosystem maturity. A project
can be engineered carefully before it is broadly adopted.

### 5.12 Ecosystem, Adoption, And External Proof

This is the criterion most resistant to planning. It requires time and use:

- independent consumer workspaces;
- documented integration feedback;
- issue and release history;
- upgrade experience;
- examples beyond the originating repository;
- evidence that users understand the configuration and reports;
- measured agent behavior across models and clients.

The target remains below `9` because broad adoption cannot be manufactured by a
single implementation cycle.

## 6. Competitor Selection

There is no exact substitute that combines the complete intended scope. The
four comparators represent the nearest overlapping product categories:

1. **Dagger**: portable DAG execution and automation infrastructure.
2. **Nx with Nx Cloud**: monorepo task graph, optimized CI, and AI-assisted CI.
3. **Trunk Code Quality with CI Autopilot**: hermetic code-quality tooling and
   CI diagnosis.
4. **MegaLinter**: broad open-source aggregation of static-analysis tools with
   agent workflows.

Task runners and pre-commit frameworks overlap with smaller portions of the
runtime, but these four provide a stronger benchmark for the intended product
position.

## 7. Competitive Research

### 7.1 Dagger

#### What It Does

Dagger describes itself as a portable DAG execution engine for repeatable CI
pipelines. Its pipeline logic can run locally or in cloud environments with
container composition, automatic caching, and end-to-end tracing. Dagger
checks run concurrently and can be invoked locally, in CI, by an agent, or
before deployment. Its current `1.0-beta` API also exposes LLM conversations,
tools, skills, workspaces, and experimental agent operations.

Primary sources:

- [Dagger introduction](https://docs.dagger.io/getting-started/introduction/)
- [Dagger checks](https://docs.dagger.io/core-concepts/checks/)
- [Dagger LLM API](https://docs.dagger.io/reference/api/llm/)

#### Where Dagger Is Stronger

- Containerized and portable execution environments.
- Automatic cache and concurrent execution.
- Service composition for databases and other dependencies.
- End-to-end traces and operational observability.
- Language SDKs and reusable modules or toolchains.
- A native environment in which agents can receive tools and workspaces.

#### Where Workspace Validator Differs

Dagger is an execution and automation platform. `workspace-validator` defines a
smaller validation-specific contract with accessible reports, strict local
configuration semantics, repository mutation evidence, planned human-owned AI
authority, and planned domain-to-evidence coverage analysis.

Dagger is the strongest technical substitute because custom Dagger code could
implement much of the target behavior. The distinction is whether those
behaviors are custom pipeline code or a ready validation-governance product.

#### Correct Integration Position

Use Dagger beneath a suite when isolation, services, caching, or concurrency are
valuable. Let Dagger own the inner execution graph and let
`workspace-validator` own selection, outer dependencies, invocation evidence,
policy, and the final validation decision.

### 7.2 Nx And Nx Cloud

#### What They Do

Nx models projects and tasks, computes the projects affected by a change, and
uses local caching to avoid repeated work. Nx Cloud adds remote caching,
distributed task execution, run history, analytics, and self-healing CI. Its MCP
server gives agents access to CI information, task output, self-healing fixes,
and workspace structure; current guidance combines MCP connectivity with
progressively loaded agent skills.

Primary sources:

- [Nx CI features](https://nx.dev/docs/features/ci-features)
- [Nx affected tasks](https://nx.dev/docs/features/ci-features/affected)
- [Nx self-healing CI](https://nx.dev/docs/features/ci-features/self-healing-ci)
- [Nx MCP server](https://nx.dev/docs/reference/nx-mcp)

#### Where Nx Is Stronger

- Native project and task graphs for monorepos.
- Affected-project calculation.
- Local and remote caching.
- Dynamic distribution across CI agents.
- Historical task data and CI observability.
- Mature plugin and framework integration.
- Shipped agent connectivity and self-healing CI.

#### Where Workspace Validator Differs

Nx is centered on its project and task model. `workspace-validator` is centered
on a language-neutral validation contract and can govern repositories that do
not adopt Nx as their workspace architecture.

Nx asks which known tasks are affected and how to execute them efficiently.
The target `workspace-validator` additionally asks which evidence should exist,
whether it covers domain risk, who may act on the result, and whether passing
evidence has drifted away from the product.

#### Correct Integration Position

Use Nx beneath suites for project-aware `affected` validation, cache, and CI
distribution. Do not reproduce the Nx project graph. Govern the selected Nx
commands and relate their evidence to the broader workspace validation model.

### 7.3 Trunk Code Quality And CI Autopilot

#### What They Do

Trunk Code Quality is a metalinter and static-analysis manager for polyglot
repositories. It hermetically manages tool versions and runtimes, uses a daemon
to precompute and cache results, reports in editors, and applies Git-aware
hold-the-line analysis to changed code. CI Autopilot exposes root-cause analysis
to supported coding agents through MCP. Trunk's broader AI DevOps Agent is
documented as beta.

Primary sources:

- [Trunk Code Quality overview](https://docs.trunk.io/code-quality/overview)
- [Trunk CI Autopilot MCP](https://docs.trunk.io/use-ci-autopilot/apply-fixes-with-mcp)
- [Trunk AI DevOps Agent](https://docs.trunk.io/ai-devops-agent/overview)

#### Where Trunk Is Stronger

- Hermetic installation of linters and their runtimes.
- A maintained static-analysis plugin catalog.
- Incremental Git-aware analysis and hold-the-line adoption.
- Background caching and IDE annotations.
- CI history and root-cause diagnosis.
- A polished code-quality onboarding experience.

#### Where Workspace Validator Differs

Trunk is specialized around code quality, static analysis, and CI diagnosis.
`workspace-validator` composes arbitrary validation evidence, including tests,
builds, audits, domain-specific checks, and Trunk itself.

The target decision policy is also broader than selecting which CI failures may
receive fixes. It governs categories of agent authority while preserving human
ownership of the policy document.

#### Correct Integration Position

Use Trunk as the code-quality suite. Let Trunk own linter provisioning,
incremental analysis, and editor integration. Let `workspace-validator` place
that suite beside runtime tests, builds, architecture checks, and other evidence
under one result and policy boundary.

### 7.4 MegaLinter

#### What It Does

MegaLinter is an open-source CI-oriented aggregator for code, IaC,
configuration, and script analysis. Its current documentation lists 67
languages, 23 formats, and 22 tooling formats, with Docker-based execution,
autofix, CI integrations, and multiple reporters. Version 10 adds agent skills
for setup, checking, fixing, and bounded rechecking, while its LLM Advisor
supports multiple hosted and local model providers.

Primary sources:

- [MegaLinter overview](https://megalinter.io/latest/)
- [MegaLinter coding-agent skills](https://megalinter.io/10/install-agent-skills/)
- [MegaLinter LLM Advisor](https://megalinter.io/10.1.0/llm-advisor/)
- [MegaLinter changelog](https://megalinter.io/latest/CHANGELOG/)

#### Where MegaLinter Is Stronger

- Very broad out-of-the-box linter coverage.
- Maintained Docker images and specialized flavors.
- Automatic language and file-oriented setup.
- CI provider integrations and numerous report formats.
- Autofix guidance and per-linter documentation.
- Ready agent skills and optional AI-generated fix suggestions.

#### Where Workspace Validator Differs

MegaLinter answers how to execute many linters consistently.
`workspace-validator` answers how arbitrary evidence sources are composed and
governed. Tests, builds, package audits, schema checks, architecture checks, and
MegaLinter can all be peers in one validation graph.

MegaLinter also demonstrates why skills alone are not a durable differentiator.
The intended distinction is the policy, domain model, coverage reasoning,
evidence continuity, and passing-result drift analysis that the skills enforce.

#### Correct Integration Position

Use MegaLinter as one static-analysis suite instead of reproducing its catalog.
Keep its detailed linter configuration and fix guidance under MegaLinter, while
`workspace-validator` governs when it runs and how its result participates in
the complete evidence set.

## 8. Comparative Capability Matrix

This matrix scores **capability fit**, not overall product quality. `1` means the
capability is absent or outside the product center; `5` means it is a leading or
defining capability. Competitor scores are approximate interpretations of the
official sources listed above.

| Capability | WV now | WV target | Dagger | Nx | Trunk | MegaLinter |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Arbitrary heterogeneous command orchestration | 4 | 5 | 5 | 4 | 2 | 2 |
| Deterministic validation-specific contract | 5 | 5 | 4 | 4 | 4 | 3 |
| Isolation and tool provisioning | 2 | 2 | 5 | 3 | 5 | 4 |
| Cache, affected analysis, and distribution | 2 | 3 | 5 | 5 | 4 | 3 |
| Built-in validator or plugin catalog | 1 | 2 | 4 | 5 | 5 | 5 |
| Versioned top-level evidence contract | 4 | 5 | 4 | 4 | 4 | 4 |
| Accessible local terminal presentation | 5 | 5 | 3 | 3 | 3 | 3 |
| Agent-facing operations | 2 | 5 | 5 | 5 | 4 | 4 |
| Human-owned AI authority policy | 1 | 5 | 2 | 2 | 2 | 2 |
| Domain-to-evidence coverage engineering | 1 | 5 | 2 | 2 | 1 | 1 |
| Passing-evidence drift analysis | 1 | 5 | 1 | 2 | 1 | 1 |
| Local and model-neutral core operation | 5 | 5 | 4 | 3 | 3 | 4 |

The matrix shows the intended strategy clearly:

- Dagger and Nx should continue to dominate execution optimization.
- Trunk and MegaLinter should continue to dominate ready-made static analysis.
- `workspace-validator` should dominate explicit validation governance,
  evidence relationships, human-owned agent authority, and coverage evolution.

## 9. Governance Architecture

The target relationship is:

```text
human operator
      |
      +---------------- direct Human Flow ----------------+
      |                                                   |
      +---- human-owned AI policy ---- AI-Tool / AI-Engineering
                                      |
                                      v
                          workspace-validator
                   configuration, selection, policy,
                    provenance, evidence, reporting
                                      |
                 +--------------------+--------------------+
                 |                    |                    |
                 v                    v                    v
          native validators      execution systems    quality systems
          cargo, pnpm, tests     Dagger, Nx            Trunk, MegaLinter
                 |                    |                    |
                 +--------------------+--------------------+
                                      |
                                      v
                         canonical validation result
                                      |
                       +--------------+--------------+
                       |                             |
                       v                             v
                 incident analysis            passing-result drift
```

### 9.1 Ownership Boundary

Specialized systems own:

- installation and runtime isolation when they provide it;
- internal task graphs;
- cache and distribution;
- linter, framework, or language semantics;
- native detailed reports and repair capabilities.

`workspace-validator` owns:

- the selected external invocation;
- the outer validation graph;
- dependency and selection semantics;
- top-level status and repository evidence;
- versioned human and machine reports;
- links between required evidence and configured checks;
- agent authority and continuation policy;
- analytical continuity for domain, coverage, incident, and drift processes.

### 9.2 Integration Levels

#### Level 1: Process Governance

This is substantially available today. An external product is configured as a
tool and invoked through a check. The validator records its status, duration,
bounded output, dependency outcome, and repository effects.

#### Level 2: Evidence Interoperability

The next integration level links an external product's structured report to the
top-level validation result with explicit provenance. This must be designed from
real Dagger, Nx, Trunk, and MegaLinter fixtures. A universal parser invented
before those fixtures would create complexity without proven value.

#### Level 3: Validation Engineering

The AI-Engineering Flow relates external evidence to domain consequences,
invariants, coverage gaps, incidents, and drift. This is where the project gains
its intended strategic position.

#### Level 4: Execution Optimization

Cache, containers, affected analysis, and distribution remain delegated unless
measurements demonstrate that a small native optimization materially improves
governance. The validator should expose and govern these systems, not absorb
them.

## 10. Roadmap To The Intended Position

### Priority 1: Complete Shared Validation Knowledge

Implement the foundation described by
[`02-shared-validation-knowledge.md`](../plans/validation-system/02-shared-validation-knowledge.md).
Keep it concise, sourced, and progressively disclosed. Its purpose is to guide
decisions, not become an offline encyclopedia.

**Score impact:** validation knowledge, configuration usability, and AI quality.

### Priority 2: Complete AI-Tool And Human-Owned Policy

Implement the policy, configuration, run, triage, and audit operations from
[`03-ai-tool-flow.md`](../plans/validation-system/03-ai-tool-flow.md). Preserve
the rule that agents may explain and recommend policy but never create, mutate,
move, restore, or delete it.

**Score impact:** safety, AI interface, and category differentiation.

### Priority 3: Establish Immutable Evidence Continuity

Implement the persistence and safety foundation from
[`04-ai-engineering-persistence-and-safety-foundation.md`](../plans/validation-system/04-ai-engineering-persistence-and-safety-foundation.md).
Persist exact reports only under applicable authority and bind analytical
artifacts to explicit paths and digests.

**Score impact:** evidence, auditability, and AI governance.

### Priority 4: Implement Domain And Coverage Engineering

Implement Processes 0 and 1 from
[`05-ai-engineering-domain-and-coverage-setup.md`](../plans/validation-system/05-ai-engineering-domain-and-coverage-setup.md).
The resulting sensorium must reconcile expected evidence with actual
configuration IDs rather than merely recommend generic test types.

**Score impact:** the principal unique value proposition.

### Priority 5: Implement Incident And Drift Loops

Implement Processes 2 and 3 from
[`06-ai-engineering-triage-and-drift-loop.md`](../plans/validation-system/06-ai-engineering-triage-and-drift-loop.md).
The drift process is essential because a green validation result cannot prove
that the configured evidence still represents the changed product.

**Score impact:** differentiation, operational continuity, and long-term value.

### Priority 6: Prove External Governance

Create maintained integration fixtures for:

- a Dagger check graph;
- an Nx affected task selection;
- a Trunk code-quality run;
- a MegaLinter run;
- ordinary native commands without an external orchestration platform.

Each fixture must prove configuration validation, effective invocation,
failure propagation, timeout or interruption behavior where applicable,
bounded diagnostics, JSON representation, and repository mutation reporting.

Only after these fixtures exist should the project decide whether a versioned
structured-evidence extension is required.

**Score impact:** category proof, integration readiness, and ecosystem value.

### Priority 7: Complete End-To-End And Agent Evaluation

Use
[`07-end-to-end-verification-and-release.md`](../plans/validation-system/07-end-to-end-verification-and-release.md)
and
[`agent-evaluation-program.md`](../plans/validation-system/agent-evaluation-program.md)
to test:

- humans using the CLI directly;
- agents discovering the CLI through only its built-in help, diagnostics, and
  other in-band output;
- agents using only public CLI documentation;
- humans or agents using AI-Tool operations;
- AI-Engineering using AI-Tool and the four recurring processes;
- policy decisions and prohibited actions;
- successful, failed, blocked, skipped, interrupted, and internally failed
  outcomes;
- behavior across multiple models and agent clients.

**Score impact:** credibility, safety evidence, and release maturity.

### Priority 8: Build Adoption Evidence

After the system works end to end:

- publish focused examples instead of a large integration framework;
- validate at least three independent polyglot workspaces;
- document configuration effort and recurring maintenance cost;
- collect report usability and accessibility feedback;
- measure false guidance, policy violations, and human intervention rates in
  agent evaluations;
- review competitor capabilities at every material release.

**Score impact:** ecosystem and proof. This work is required to move beyond an
internally strong implementation.

## 11. Strategic Guardrails

To preserve the intended position, do not turn the core into:

- a replacement for Dagger's container and DAG engine;
- a replacement for Nx project graphs, affected analysis, or remote cache;
- a replacement for Trunk's hermetic linter manager and IDE daemon;
- a replacement for MegaLinter's catalog and Docker distributions;
- a package or runtime installer;
- a model host;
- an autonomous fixer whose authority bypasses human policy;
- a generic workflow engine unrelated to validation evidence.

Capabilities such as a DAG, JSON output, skills, MCP connectivity, and AI fix
suggestions are already available elsewhere. They are supporting mechanisms,
not the defensible product identity.

The strongest defensible combination is:

1. deterministic, model-neutral execution contracts;
2. canonical and attributable evidence;
3. explicit domain-to-evidence coverage;
4. human-owned AI authority;
5. governed incident and drift processes;
6. ability to compose specialized external systems without absorbing them.

## 12. Definition Of The Desired Position

The project may claim to be a validation-governance layer when all of the
following are true:

- [ ] The Human Flow remains independently complete without AI.
- [ ] The AI-Tool Flow is implemented, policy-governed, and bounded to one
      requested operation.
- [ ] The AI-Engineering Flow persists immutable analytical artifacts and uses
      explicit evidence handoff.
- [ ] Humans retain exclusive mutation authority over AI decision policy.
- [ ] Processes 0 through 3 are implemented and evaluated.
- [ ] Domain invariants map to concrete configured evidence IDs.
- [ ] Passing-result drift can recommend coverage recalibration.
- [ ] Dagger, Nx, Trunk, and MegaLinter run successfully as governed external
      systems in maintained fixtures.
- [ ] External tool identity, version, invocation, and result provenance are
      inspectable.
- [ ] Human and JSON outputs remain semantically equivalent and accessible.
- [ ] Agent evaluations cover CLI-only discovery, documentation-assisted direct
      CLI use, AI-Tool, and AI-Engineering across more than one model family.
- [ ] Adversarial tests distinguish enforceable CLI boundaries from agent
      instructions.
- [ ] Independent workspaces demonstrate that the configuration and governance
      model are maintainable outside the originating project.

## 13. Principal Risks

### 13.1 Instructional Safety Presented As Enforcement

Agent skills and policy interpretation guide behavior, while CLI and operating
system boundaries enforce only what their code controls. Documentation and
marketing must preserve this distinction.

### 13.2 Configuration Burden

The four-layer model is explicit but can become verbose. Shared checks and
suites, strong inspection, examples, and AI-assisted authoring should reduce
repetition without introducing implicit behavior.

### 13.3 Universal Evidence Abstraction

Trying to normalize every external report too early could create a fragile
lowest-common-denominator schema. Integration fixtures must precede an adapter
contract.

### 13.4 Execution-Engine Scope Creep

Native cache, distribution, containers, and affected analysis are attractive
features with high implementation and maintenance cost. Delegate first.

### 13.5 Unproven Agent Behavior

A coherent policy model does not guarantee that external agents follow it.
Multi-model evaluations, forbidden-action assertions, and exact artifact
inspection are required.

### 13.6 Ecosystem Maturity

Internal test quality does not substitute for adoption, upgrade history, and
independent feedback. The maturity score must remain conservative until that
evidence exists.

## 14. Final Judgment

The current crate is already a carefully engineered deterministic validation
runner. That alone is useful, but it competes in a crowded space of task
runners, CI utilities, and metalinters.

The project becomes strategically distinctive when the remaining system turns
execution into validation engineering: domain modeling, evidence coverage,
human-owned agent authority, incident continuity, and drift analysis. The four
competitors do not invalidate that direction. They clarify it.

The correct ambition is not to defeat Dagger, Nx, Trunk, or MegaLinter at their
specialties. It is to make them governable parts of one deterministic,
auditable, human-directed validation system.
