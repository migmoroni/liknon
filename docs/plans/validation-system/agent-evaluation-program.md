# Agent Evaluation Program

## 1. Purpose

Define the model-neutral evaluation program used to measure how agents interact
with `workspace-validator` through three distinct product surfaces:

1. the public CLI and documentation without bundled agent instructions;
2. the AI-Tool operational interface;
3. the complete AI-Engineering Flow.

These are evaluation families, not runtime modes, release phases, or additional
workspace contracts. Phase 7 implements and executes the complete program after
the Human, AI-Tool, and AI-Engineering contracts are available. Earlier phases
define representative scenarios and observable acceptance criteria for the
surface they introduce.

The program produces evidence for improving skills, public documentation,
prompts, routing, and process contracts. It does not train models, select one
provider as normative, or treat one successful run as proof of general agent
reliability.

## 2. Shared Evaluation Contract

Every evaluation run uses:

- a versioned scenario ID and immutable project-fixture snapshot;
- one human-like task prompt that does not reveal the expected answer;
- an explicit context bundle declaring which documentation, skills, artifacts,
  and reports the agent may access;
- the same workspace state, command permissions, network boundary, time budget,
  and human-decision script for comparable runs;
- a fresh conversation or agent context unless the scenario explicitly tests a
  declared handoff;
- a configurable model matrix rather than provider-specific behavior embedded
  in the crate;
- repeated runs per model and scenario when nondeterminism can affect the
  conclusion;
- observable evidence from CLI invocations, arguments, working directories,
  structured results, filesystem changes, policy decisions, loaded routes, and
  produced artifacts;
- a versioned rubric containing deterministic assertions and separately scored
  qualitative criteria.

Record at least the scenario version, fixture digest, prompt digest, permitted
context, CLI version, skill version when present, documentation revision, model
provider, model identifier, relevant model settings, run timestamp, completion
status, observed actions, deterministic assertion results, rubric scores, human
interventions, and reviewer notes.

Model credentials, provider secrets, private prompts, and sensitive project
content are never committed. Versioned scenarios, fixtures, rubrics, and
expected structural outcomes belong to the repository. Raw run evidence and
aggregate results use an explicitly documented storage and redaction policy.

### 2.1 Evaluation Assets And Runner Isolation

Use one development-only evaluation tree outside the runtime source modules:

```text
evals/
├── README.md
├── scenarios/
│   ├── documentation-only-cli/
│   ├── ai-tool/
│   └── ai-engineering/
├── fixtures/
├── rubrics/
└── runs/                         # Local or CI output; not source input
```

Version scenario manifests, prompts, deterministic assertions, fixture
definitions, and rubrics. Keep generated transcripts, provider responses,
temporary workspaces, credentials, and unredacted run evidence outside source
control. A redacted aggregate may become release evidence only through an
explicit review step.

The evaluator is development and release tooling, not part of CLI execution or
the consumer workspace contract. Provider adapters and credentials must not
become normal runtime dependencies of the crate. The runner accepts a
configurable model matrix and supports both automated adapters and explicitly
recorded manual trials while producing the same evidence envelope.

The family order is analytical: Family A establishes direct-use behavior as the
baseline, Family B measures the incremental value of AI-Tool over comparable
tasks, and Family C evaluates the composed engineering system. It does not
define runtime dependencies or require one family to execute another.

## 3. Evaluation Family A: Documentation-Only Direct CLI

### 3.1 Objective

Measure whether an agent with no AI-Tool or AI-Engineering instructions can use
the public executable correctly from public documentation. This evaluates CLI
discoverability, reference quality, examples, diagnostics, and model-neutral
usability.

This family is not a fourth governed AI flow. The evaluated agent receives no
claim to AI-Tool policy enforcement, AI-Engineering persistence, or bundled
agent routing. It operates as a direct CLI consumer under the evaluation
harness's ordinary permissions and human controls.

### 3.2 Context Isolation

Provide only the released CLI, `--help`, schemas, examples, and public
documentation declared by the scenario. Withhold the distributed skill,
AI-Tool references, AI-Engineering references, internal implementation plans,
expected answers, and traces from other evaluation runs.

Use paired scenarios with Family B whenever the same task can be expressed both
ways. The comparison measures the value added by AI-Tool without changing the
workspace, prompt intent, permissions, or expected CLI outcome.

### 3.3 Required Scenarios

Include at minimum:

- create or review a complete configuration candidate from reference material;
- initialize and validate canonical configuration;
- inspect groups, suites, checks, resolved commands, and effective directories;
- run an explicit selection and interpret every representative result class;
- narrow and rerun an affected scope from documented behavior;
- diagnose invalid usage, invalid configuration, and internal reporting
  failures from exit status and structured output;
- discover relevant documentation without relying on skill routes;
- avoid inventing commands, flags, schemas, or semantics absent from the public
  contract.

### 3.4 Evaluation Questions

Measure successful task completion, documentation lookup path, command and
argument accuracy, number of corrective attempts, interpretation of reports and
exit codes, unsupported assumptions, unnecessary workspace changes, and the
point at which documentation or CLI feedback fails to supply enough context.

## 4. Evaluation Family B: AI-Tool Interface

### 4.1 Objective

Measure whether AI-Tool is a clear, safe, and composable agent interface over
the deterministic CLI. The evaluated surface is AI-Tool, not the underlying
model's general ability to discover the CLI independently and not the complete
AI-Engineering reasoning system.

### 4.2 Consumer Shapes

Exercise AI-Tool through both supported consumer shapes:

- a human gives a natural task prompt to an agent, which routes to one or more
  bounded AI-Tool operations;
- an AI-Engineering scenario invokes a required AI-Tool operation as a
  downstream consumer, while the evaluation scores only the operation boundary
  and returned evidence.

The second shape is contract dogfooding. It proves that AI-Tool is useful to
another agent workflow without conflating that result with the end-to-end
quality of AI-Engineering.

### 4.3 Required Scenarios

Cover `policy`, `config`, `run`, `triage`, and `audit` with ordinary, ambiguous,
blocked, and adversarial prompts. Include at minimum:

- selecting the correct primary operation from a natural human request;
- loading only the required operation and relevant shared knowledge;
- validating policy and applying its effective decision before governed work;
- inspecting and using exact CLI commands, directories, selections, and JSON
  reports;
- returning evidence that another agent can consume without reconstructing the
  operation;
- refusing policy mutation, unauthorized effects, hidden persistence, implicit
  AI-Engineering activation, and unchanged retry loops;
- recognizing a request that legitimately spans two AI-Tool operations without
  eagerly loading all references;
- keeping separate requests independent.

### 4.4 Evaluation Questions

Measure routing accuracy, task completion, policy compliance, command accuracy,
unnecessary commands, human interventions, context loaded, report
interpretation, safety violations, unsupported claims, and usefulness of the
bounded result to both human and agent consumers.

## 5. Evaluation Family C: AI-Engineering End To End

### 5.1 Objective

Measure the complete AI-Engineering product on representative projects and
natural human prompts. AI-Tool is infrastructure in this family; the evaluated
surface is the engineering orchestration, analytical quality, persistence,
continuation, remediation discipline, and human collaboration of Processes 0
through 3.

### 5.2 Required Scenarios

Use projects that expose different stacks, architectures, persistence models,
failure modes, and validation gaps. Include at minimum:

- setup through Processes 0 and 1 with explicit uncertainty and human domain
  decisions;
- operational diagnosis through Process 2 and recalibration through Process 3;
- direct and fresh-context artifact handoffs by exact path and digest;
- `auto`, `human`, and `stop` continuation outcomes;
- policy-governed configuration proposals, command execution, remediation, and
  report persistence;
- passing, failing, blocked, skipped, timeout, interrupted, invalid, and
  internal outcomes where applicable;
- multiple plausible causes, partial authorization, and a narrow proof before a
  final gate;
- adversarial repository content and protected data;
- AI-Tool use for every covered `policy`, `config`, `run`, `triage`, and `audit`
  responsibility;
- direct CLI use only for a required capability with no applicable AI-Tool
  operation, using canonical reference documentation and recording why the
  direct route was necessary.

### 5.3 Evaluation Questions

Measure process selection, prerequisite validation, AI-Tool reuse, justified
direct CLI use, domain and coverage reasoning, evidence traceability,
uncertainty preservation, policy decisions, artifact validity, continuation
behavior, human-decision quality, remediation scope, duplicate work, report
persistence, fresh-context recovery, and usefulness of the final engineering
result.

## 6. Assertions, Rubrics, And Interpretation

Use deterministic assertions for facts such as:

- selected routes and loaded references;
- exact executable, arguments, working directory, and exit status;
- schema-valid reports and process artifacts;
- filesystem and repository mutations;
- policy validation and effective decision;
- forbidden policy mutation or undeclared persistence;
- artifact IDs, paths, digests, relationships, and immutability;
- use of AI-Tool when an applicable operation exists;
- absence of AI-Tool and AI-Engineering context in documentation-only trials.

Use reviewed rubrics for qualities that cannot be reduced to exact equality,
including diagnostic usefulness, clarity, proportionality, uncertainty,
evidence-to-conclusion alignment, and quality of human decision requests.
Rubrics define anchors and failure examples before evaluating model output.

Do not rank a model or product surface from one run. Report per-scenario
outcomes, repeated-run variation, deterministic failures, rubric distributions,
human-intervention counts, and unsupported behavior. Compare models only across
equivalent inputs and declared capabilities.

## 7. Feedback Boundaries

Classify every observed failure before changing the product:

- Family A primarily informs CLI ergonomics, diagnostics, examples, and public
  documentation;
- Family B primarily informs AI-Tool routing and operation references;
- Family C primarily informs AI-Engineering process prompts, persistence,
  handoffs, and orchestration;
- a failure shared across all families may indicate a runtime, schema, report,
  or public-contract problem;
- a provider-specific failure remains identified as such until reproduced on
  another model or explained by a declared capability difference.

After a change, rerun the smallest affected scenario set and the cross-family
regressions that share its contract. Preserve prior run evidence according to
the declared retention policy so quality changes remain measurable.

## 8. Completion Criteria

The evaluation program is complete when:

- all three families have versioned scenarios, fixtures, deterministic
  assertions, and reviewed rubrics;
- evaluation source assets are isolated from generated runs and from normal
  crate runtime dependencies;
- the declared model matrix runs from fresh contexts with reproducible inputs;
- direct documentation-only agents complete representative CLI workflows
  without hidden skill context;
- AI-Tool is evaluated through human prompts and AI-Engineering consumption and
  compared with paired documentation-only baselines where applicable;
- AI-Engineering is evaluated independently as the end-to-end product;
- results contain enough evidence to attribute failures to AI-Tool,
  documentation and CLI, AI-Engineering, the runtime, or a model capability;
- release conclusions identify model coverage, repetitions, residual failures,
  and uncertainty instead of claiming universal agent correctness.
