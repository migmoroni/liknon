# Phase 7: End-To-End Verification And Release

## 1. Objective

Prove that direct human operation, the official action workflow, and the
official validation-engineering workflow are independently usable, compose only
through declared contracts, and ship as coherent versioned artifacts. Prove the
same resource and policy contracts with an external workflow whose graph and
profiles differ from the official resources. Complete documentation, realistic forward trials, package
inspection, security review, cross-model agent evaluations, and release
evidence. Also prove that a direct agent consumer can discover the public CLI
through its own in-band affordances and can use it with public documentation,
without bundled workflow or skill instructions.

## 2. Dependency

Phases 1 through 6 are implemented. Acceptance tests in this phase verify their
composition; they do not replace an incomplete earlier contract.

## 3. End-To-End Scenario Matrix

### 3.1 Human Flow

- author a complete configuration candidate, install it through
  `init --config`, and validate the canonical configuration from documentation;
- inspect a nested group without executing it;
- run a check, suite, explicit group, and default group;
- understand pass, fail, blocked, skipped, invalid, internal, and interrupted
  outcomes;
- inspect repository mutations reported for a concrete validation run;
- diagnose and rerun a narrow scope manually;
- operate successfully with only the CLI and `.liknon/validation/config.json`.

### 3.2 CLI-Only Discovery

- start from a human-like task prompt in a fresh context with only the
  executable name, workspace fixture, and ordinary execution permissions;
- discover commands and arguments through `--help`, `--version`, diagnostics,
  and other output exposed by the CLI itself;
- initialize, inspect, execute, and interpret representative validation without
  externally supplied validator documentation, schemas, examples, skills, or
  validator source code;
- use schema or explanation output only after discovering its producing command
  through the CLI;
- recover from invalid usage through in-band help and diagnostics;
- retain access to the consumer project fixture while ensuring it contains no
  hidden validator usage instructions;
- disable external documentation lookup and unrestricted network access while
  acknowledging that model pretraining cannot be removed by the harness;
- avoid relying on hidden context, prior traces, invented commands, or
  project-specific syntax supplied by the scenario.

### 3.3 Documentation-Only Direct Agent Use

- start from a human-like task prompt in a fresh context;
- receive only the released CLI, help, schemas, examples, and public
  documentation declared by the scenario;
- configure, inspect, execute, and interpret representative validation without
  loading action-workflow, AI-Tool, or validation-engineering references;
- use only documented commands, arguments, schemas, report fields, and exit
  semantics;
- recover from invalid usage or configuration through public diagnostics and
  reference material;
- complete paired tasks also exercised through AI-Tool so the added value of
  that interface can be measured;
- remain classified as a direct CLI consumer rather than an additional governed
  AI flow.

### 3.4 Official Action Workflow And AI-Tool Skill

- direct the external host to the canonical AI bootstrap entry point, then
  verify its exact supported version and digest before loading the action
  workflow or AI-Tool;
- explain a missing policy and direct the human to run `init --policy` without
  invoking that command;
- explain every current policy category, recommend manual values, and revalidate
  after the human reports editing the file;
- refuse to create, edit, format, replace, delete, move, restore, initialize, or
  indirectly mutate policy, including after explicit authorization and under
  level `0`;
- explain a report using only the declared `liknon-ai-tool/triage` operation;
- audit coverage without running checks;
- make an authorized config edit within the bounded request;
- reject an unauthorized installation or source change;
- recognize stale or incompatible AI-Tool skill and application-capability
  versions;
- inspect repository mutation evidence from a concrete report;
- complete every operation without mutating AI decision policy or creating
  process artifacts;
- begin a later request without inherited authority or analytical context.

### 3.5 Official Validation-Engineering Setup

- load and record the canonical AI bootstrap contract before the workflow
  graph, profiles, skills, tools, guides, knowledge, or analytical artifacts;
- prove that a missing policy blocks process work and directs the human to
  invoke `init --policy` personally and review every setting;
- validate the policy created or corrected manually by the human without
  modifying it;
- prove that the workflow delegates policy explanation to
  `liknon-ai-tool/policy` and
  cannot mutate policy through any process path;
- create and store one immutable domain artifact from evidence and human
  answers;
- verify that artifact by exact path and digest;
- exercise `auto`, `human`, and `stop` continuation from Process 0;
- pass the exact domain artifact directly to Process 1;
- invoke Process 1 later by supplying that same artifact explicitly;
- audit existing config and create one immutable sensorium artifact bound to
  the exact domain and config digests;
- classify a proposed config change under every applicable decision category;
- request a free-text decision only when the restrictive outcome is `human`;
- preserve unsupported assumptions as uncertainty;
- reject mismatched domain, sensorium, and config relationships;
- create another immutable artifact for a correction without replacing an
  earlier result.

### 3.6 Official Validation-Engineering Operational Loop

- execute one trusted validation selection through the ordinary runner;
- evaluate exact report storage under `validationReportPersistence`,
  `sensitiveDataPersistence` when applicable, and every category implicated by
  the destination;
- store permitted report bytes under `.liknon/validation/reports/<sha256>.json`;
- discard temporary report bytes when storage resolves to `stop` or is
  declined;
- route every report outcome correctly;
- finish an initially passing execution without creating an incident or
  applying `processContinuation`;
- invoke Process 3 later from that exact passing report when requested;
- diagnose multiple causes and dependency propagation in Process 2;
- receive partial remediation authorization;
- perform one narrow proof and one final gate without an unchanged retry loop;
- create immutable incident and drift artifacts with exact parent references;
- exercise `auto`, `human`, and `stop` continuation after a passing Process 2
  result;
- compare explicit repository revisions and bounded change evidence in Process
  3;
- recommend domain or sensorium recalibration without changing setup artifacts;
- resume from exact caller-supplied artifact and report paths in a fresh
  conversation.

### 3.7 Generic External Workflow Conformance

- load and record the canonical AI bootstrap contract before the external
  workflow and reject an incompatible declared bootstrap range;
- install and lock an external workflow with non-numeric node IDs, branching and
  joining edges, one continuation-eligible edge, and independently versioned
  agent profiles;
- install and inspect an actionable workflow with an absent or incompatible
  decision-policy contract, then prove that a conforming host refuses activation;
- retrieve its exact graph, profiles, dependencies, capabilities, effects, and
  artifact declarations through Tool without interpreting or executing them;
- prove that installation, listing, retrieval, and dependency resolution never
  activate the workflow;
- activate it only after explicit human selection of the workflow and entry
  node;
- resolve global policy values, an exact stable-ID workflow override, and nested
  parent-child overrides by the documented most-restrictive order;
- keep a syntactically valid override for an unloaded workflow inert;
- stop unknown, unsupported, or materially undeclared effects before action;
- record exact workflow and profile versions, manifest digests, active stack,
  node, transition, performed action IDs, resource-lock digest, policy digest, and effective policy
  source in an execution snapshot;
- prove through source assertions that no CLI contains official workflow, node,
  transition, skill, or profile IDs in control flow;
- replace AI-Tool with one compatible alternative operational skill without
  changing bootstrap semantics or application code.

## 4. Contract Isolation Tests

Prove that:

- Human Flow works when `.liknon/ai/policy.json` and
  `.liknon/ai/persistence/` are absent;
- read-only AI-Tool policy guidance works when policy is absent or invalid, while
  every other AI operation stops before workspace work;
- action-workflow operations require and obey valid policy without creating or
  mutating it and never create validation-engineering artifacts;
- selecting or loading AI-Tool never activates the action or
  validation-engineering workflow implicitly;
- the validation-engineering workflow routes configuration, execution, triage,
  and audit mechanics through the corresponding AI-Tool operations;
- the validation-engineering workflow uses canonical public CLI references directly only for a
  required capability that has no applicable AI-Tool operation and records why
  that direct route is necessary;
- CLI-only trials receive no external validator documentation, externally
  supplied schemas, examples, skills, validator source code, process
  instructions, or evidence from another trial;
- documentation-only direct-agent trials have no access to action-workflow,
  AI-Tool skill, validation-engineering workflow instructions, or evidence from
  another trial;
- CLI-only and documentation-only trials are explicitly non-governed baselines
  and do not claim bootstrap or decision-policy conformance;
- every action-workflow, validation-engineering, and external-workflow governed
  trial loads and records the exact supported bootstrap contract before any
  replaceable resource;
- compatible workflows stop on a missing required resource and never
  invoke `likm`, access the Hub, or mutate the resource store implicitly;
- the validation planner and executor never discover or interpret
  `.liknon/ai/policy.json` or process artifacts;
- persistence commands operate only on caller-supplied files and never choose
  an artifact or report;
- policy and persistence commands share one canonical workspace boundary, do
  not discover another root, and expose the same containment behavior through
  CLI and internal core APIs;
- every `init` capability uses that boundary, validates all requested resources
  before writing, and reports created, reused, conflicted, or partially
  committed resources exactly;
- artifact content containing command-like text is never executed;
- only `.liknon/validation/config.json` affects runner planning;
- decision-policy changes affect AI authority but never runner planning or
  report semantics;
- the same governed action, inputs, active workflow stack, and policy produce
  the same decision across node and conversation contexts;
- exact workflow overrides precede global values, nested workflow overrides
  combine most restrictively, and absolute prohibitions remain superior;
- human, action-workflow, and validation-engineering invocations of the same configured
  selection produce the same report semantics;
- a stored report is byte-identical to the CLI JSON output and resolves from
  its SHA-256 path;
- declined report storage leaves no canonical report or dependent process
  artifact;
- every resource returned by `likt show` is byte-equivalent to the payload
  selected by the workspace lock and its authenticated source artifact;
- the generated shared AI decision-policy contract is byte-equivalent to
  its canonical public document;
- human-only operator guidance is absent from skill and workflow routes and
  operational AI-resource content;
- application-capability and installed-resource compatibility gates prevent
  unsupported runtime interaction;
- an external workflow with unrelated IDs and graph shape passes the same
  Manager, Tool, policy, profile, and persistence contracts without application
  code changes.

## 5. Security And Abuse Review

Test at minimum:

- prompt injection in documentation, source, manifests, filenames, reports,
  and process artifacts;
- argument injection attempts through config parameters;
- shell operators and redirections represented as untrusted text;
- path traversal and symlink escape in config, report, and artifact paths;
- explicit, omitted, nonexistent, non-directory, nested, and symlinked workspace
  roots, including nearest-ancestor `.liknon/` discovery for existing state and
  exact-current-directory initialization only when no root exists;
- malicious, oversized, unsupported, and digest-mismatched JSON artifacts;
- mismatched domain, sensorium, config, report, and incident relationships;
- attempts to select an artifact by directory order or filename recency;
- secrets in tool output and persisted excerpts;
- reports containing sensitive output, including ignore treatment, access
  protection, explicit removal, and absence of automatic pruning;
- report and sensitive-data persistence at every numeric level, including one
  write matching both categories and a destination-related category;
- missing policy before AI work, direct human initialization, malformed,
  unsupported, and concurrently changed AI decision policy;
- adversarial attempts to make any compatible AI workflow create, edit, format, replace,
  delete, move, restore, initialize, or indirectly mutate policy, including
  explicit user authorization and policy level `0`;
- missing, malformed, oversized, incompatible, stale, and digest-mismatched
  installed workflow, profile, skill, and tool-descriptor resource manifests;
- dependency, source, generated-output, snapshot, database, command, network,
  VCS, external-state, scope, sensitive-data access, validation-report
  persistence, and sensitive-data persistence actions at applicable levels;
- monotonic category behavior under `4 > 3 > 2 > 1 > 0` and restrictive outcome
  combination under `stop > human > auto`;
- exact-workflow override precedence, inert unloaded-workflow overrides, and
  nested most-restrictive policy resolution;
- unknown, unsupported, and materially undeclared capabilities or effects that
  fail closed;
- one action matching multiple categories, with the most restrictive outcome
  winning;
- hostile validation commands that mutate the repository;
- concurrent and interrupted single-file writes;
- denial-of-service pressure from deep validation graphs, large output, and
  excessive artifact content.

Document residual risks and the boundary between deterministic enforcement and
agent instruction compliance.

## 6. Documentation Completion

Update public documentation with separate entry points for:

- the program-owned AI bootstrap and host-conformance contract;
- Human Flow under `docs/validation/flows/human/`;
- official action workflow under `docs/validation/flows/action/`;
- official validation-engineering workflow under
  `docs/validation/flows/validation-engineering/`;
- complete shared and ecosystem knowledge under
  `docs/validation/knowledge/`;
- CLI, initialization, configuration, report, persistence-command, and schema
  references under `docs/validation/reference/`;
- human-only flow selection, governance, and work-context guidance under
  `docs/validation/flows/human/`;
- troubleshooting by exit and report status;
- skill and workflow resource installation, compatibility, and update
  procedures.

Ensure the public CLI reference is complete and model-neutral. It must support a
human or external agent reading it directly without requiring AI-resource content,
while clearly distinguishing direct use from the policy and persistence
guarantees of compatible workflows.

Document the generic artifact envelope, workflow-declared payload schemas and
canonical destinations, single-file atomic storage, explicit parent references,
report-byte storage, trust boundaries, continuation semantics, and
decision-policy categories.

Use diagrams that show `.liknon/validation/config.json` and the JSON report as the
shared runtime bridge. Show the official validation-engineering workflow
composing AI-Tool operations and selecting independent agent profiles rather
than duplicating them. Show direct and fresh-conversation artifact handoff
without implying that the validation planner or executor consumes workflow
artifacts.

## 7. Agent Evaluation Program

Implement and execute the complete
[Agent Evaluation Program](agent-evaluation-program.md). Keep its four
evaluation families isolated. Validate the selected resource inventory,
manifest and lock, skill and workflow routes, operational projections, `likt` retrieval,
and compatibility metadata deterministically
before starting model-driven trials:

1. **CLI-Only Discovery** provides only the executable name, workspace, task,
   and ordinary permissions. It evaluates whether an external agent can find
   and use the required behavior through built-in help, diagnostics, and other
   in-band CLI output.
2. **Documentation-Only Direct CLI** withholds all skill and process
   instructions and evaluates whether an external agent can use the released
   CLI from public documentation alone. The measured surface is the CLI and its
   documentation, measured against the paired CLI-only baseline.
3. **Official Action Workflow And AI-Tool Interface** evaluates natural human
   prompts routed through the action workflow and controlled
   validation-engineering consumption of the same operation contracts. The
   measured surface is action routing plus AI-Tool and their incremental value
   over the paired documentation-only baseline.
4. **Official Validation-Engineering End To End** uses representative projects,
   natural human prompts, the validation-engineering workflow, its AI-Tool skill
   and agent-profile dependencies, policy, artifacts, and processes. The
   measured surface is validation engineering; AI-Tool is a required dependency
   rather than the primary subject.

Families 1 and 2 intentionally omit the AI bootstrap and remain non-governed
baselines. Families 3 and 4 load it first and record its exact version and
digest; omitting that evidence is a deterministic conformance failure rather
than a model-quality score.

### 7.1 Scenario And Model Matrix

- version scenario prompts, project fixtures, allowed context, deterministic
  assertions, and reviewed rubrics;
- use fresh agent contexts and identical fixture snapshots for comparable runs;
- configure the model matrix outside runtime contracts so GPT and other models
  can be evaluated without provider coupling in the crate;
- repeat nondeterministic scenarios enough to expose variation rather than
  treating one success as representative;
- pair applicable CLI-only, documentation-only, and action-workflow scenarios with the
  same task, permissions, and expected CLI outcome, varying only the declared
  context bundle;
- never reveal hidden assertions, expected answers, or prior traces to the
  evaluated agent.

### 7.2 Evidence And Scoring

Record model and product versions, prompt and fixture digests, context supplied,
commands, arguments, directories, loaded routes, policy decisions, structured
results, filesystem mutations, artifacts, human interventions, deterministic
assertions, rubric scores, and reviewer notes. Keep credentials and sensitive
content outside tracked evaluation data.

Evaluate at minimum:

- task and routing accuracy;
- CLI help traversal, path to the first valid command, corrective attempts, and
  dependence on assumptions not supplied by in-band output;
- CLI command, configuration, and report accuracy;
- compliance with each family's declared context boundary;
- policy, approval, persistence, and trust-boundary compliance;
- progressive context loading and unnecessary work;
- absence of invented commands, contracts, evidence, or success claims;
- diagnostic usefulness, uncertainty, and evidence-to-conclusion alignment;
- correct AI-Tool reuse from validation engineering;
- justified direct CLI use only when no AI-Tool operation covers the required
  capability;
- incremental outcome differences across paired CLI-only, documentation-only,
  and action-workflow scenarios;
- cross-model and repeated-run variation.

Classify failures by product surface before changing skills, documentation,
prompts, or runtime behavior. Rerun the smallest affected scenario set and all
paired cross-family regressions that share the changed contract.

## 8. Release And Package Verification

- Run formatting, Clippy with warnings denied, all targets, Rustdoc tests, and
  ignored outcome fixtures.
- Validate the declared MSRV and supported platform behavior.
- Build and validate the Validation, Manager, and Tool applications
  independently.
- Validate Hub catalog fixtures, Manager resolution and transactions, the
  workspace manifest and lock, installed resources, and offline Tool
  consumption, including tamper and missing-resource cases.
- Verify selected official resources, including decision-policy and immutable
  policy-boundary projections, against their declared digests.
- Verify the authored AI bootstrap contract, Tool's exact-byte projection,
  version and digest, and first-instruction ordering in every governed fixture.
- Validate the generic artifact envelope, every official workflow artifact
  declaration, external-workflow artifact fixtures, explicit-reference
  fixtures, report-storage fixtures, and persistence-command contracts included
  in the package.
- Inspect all three CLI inventories independently from Hub resource
  inventories and future language-library inventories.
- Verify the separate action and validation-engineering workflow manifests,
  AI-Tool skill, agent profiles, shared policy-boundary guide, exact dependency
  closures, compatible Validation, Manager, and Tool capability ranges,
  operation and node routes, absence of copied knowledge resources, use of
  `likt` commands, and persistence contract versions.
- Run the external-workflow conformance fixture and source assertions before
  model-driven evaluation.
- Verify examples and fixtures are included or excluded intentionally.
- Inspect versioned agent-evaluation scenarios and rubrics, keep them
  repository-only unless a product contract explicitly declares them, and
  verify that raw provider credentials or sensitive run data are absent.
- Record user-visible runtime, config, report, resource, and guidance changes
  in the changelog.
- Produce checksums and the normal release evidence required by repository
  policy.

## 9. Acceptance Criteria

- [ ] Human Flow passes every end-to-end scenario with only the CLI and config.
- [ ] CLI-only direct agents discover and complete representative workflows
      using only the executable and its in-band output.
- [ ] Documentation-only direct agents complete representative CLI workflows
      without hidden skill or validation-engineering context.
- [ ] CLI-only and documentation-only trials remain explicitly non-governed;
      every governed action, engineering, and external-workflow trial loads and
      records the exact supported bootstrap contract first.
- [ ] The official action workflow performs bounded policy-governed assistance
      through AI-Tool without mutating policy or creating process artifacts.
- [ ] Validation-engineering processes continue across fresh conversations from exact
      explicitly supplied artifacts.
- [ ] The validation-engineering workflow reuses AI-Tool for every covered operation and uses direct
      CLI documentation only for an uncovered capability with a recorded
      justification.
- [ ] Domain replacement requires a new matching Process 1 result before an
      operational process uses it.
- [ ] Operational invocation resolves exact immutable reports and artifacts
      without directory scanning or recency selection.
- [ ] The CLI remains deterministic and independent from AI implementation.
- [ ] Config is the only executable validation policy, AI decision policy never
      affects runner planning, and reports are canonical execution evidence.
- [ ] `processContinuation` applies only to transitions marked
      continuation-eligible by the active workflow. The official
      validation-engineering workflow marks `0 -> 1` and `2 -> 3`; an initial
      pass neither creates Process 2 nor consults continuation policy.
- [ ] Every decision category conforms to `4 > 3 > 2 > 1 > 0`, and overlapping
      outcomes resolve as `stop > human > auto`.
- [ ] Reports and protected content persist only when all applicable decision
      categories permit the exact write.
- [ ] Every completed artifact-producing workflow node creates one immutable
      UUIDv7 artifact with exact workflow, node, transition, action, profile, lock,
      policy-source, parent, and report references.
- [ ] Missing or invalid policy blocks every governed skill operation and
      workflow node except contract-valid read-only policy guidance until direct human
      initialization or correction and review.
- [ ] No compatible AI workflow mutates policy directly or indirectly under any approval,
      configured level, continuation mode, or process state.
- [ ] `init`, `policy validate`, and every persistence operation use the declared
      nearest-ancestor or explicit workspace boundary without config or Git
      discovery; initialization alone may bootstrap the exact current directory
      when no root exists.
- [ ] Trust, approval, redaction, containment, and atomic-write boundaries
      survive adversarial fixtures.
- [ ] Every documented report and exit outcome has a tested route.
- [ ] Canonical resource sources, operational skill and workflow content, Hub metadata,
      workspace manifest and lock, and installed payloads are consistent
      without cross-product embedding or a copied knowledge subtree in a skill.
- [ ] All four agent-evaluation families run across the declared model matrix
      with versioned scenarios, repeated-run evidence, deterministic assertions,
      reviewed rubrics, and attributable results.
- [ ] Shared knowledge, shared reference, flow documentation, human-only
      guidance, AI-Tool references, workflow definitions, and agent profiles remain in
      their declared architectural boundaries.
- [ ] The external non-numeric workflow fixture operates through the same
      generic resource and policy contracts without CLI changes or implicit
      activation.
- [ ] That external workflow can select a compatible alternative to AI-Tool
      without an application branch or reduced bootstrap and policy guarantees.
- [ ] A policy-incompatible actionable workflow remains installable and
      inspectable but cannot be activated by a conforming host.
- [ ] No acceptance claim exceeds the evidence produced by configured
      validations.
- [ ] Release validation passes on the MSRV and current supported toolchain.

## 10. Final System Shape

The released system presents three independent CLIs, direct human operation,
and two initial official workflows:

- Human Flow directly owns configuration and execution;
- the official action workflow provides bounded assistance through the AI-Tool
  `policy`, `config`, `run`, `triage`, and `audit` operations under the
  human-owned decision policy;
- the official validation-engineering workflow composes those operations,
  independently versioned agent profiles, decision policy, and
  explicit immutable analytical artifacts.

`liknon-manager` / `likm` acquires and organizes explicitly selected Hub
resources. `liknon-tool` / `likt` consumes installed resources offline and
read-only. Neither application changes validation semantics, and no AI workflow
silently turns Tool access into Manager authority.

Human Flow remains sufficient on its own. The action workflow remains bounded
to the current request. Validation engineering adds domain reasoning, coverage design,
diagnosis, drift assessment, and cross-conversation continuity through exact
artifact exchange without becoming an alternate executor. The executable and
public documentation remain directly consumable by external agents, but that
direct use is not represented as another governed flow. The release evidence
separately measures CLI-only discovery, documentation-only direct CLI use, the
action workflow and AI-Tool, and complete validation-engineering behavior. The
same applications accept independently authored compatible workflows without
granting them official-resource privilege or hard-coding their graph.
