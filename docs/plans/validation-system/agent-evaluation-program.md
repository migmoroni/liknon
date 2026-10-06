# Agent Evaluation Program

## 1. Purpose

Define the model-neutral evaluation program used to measure how agents interact
with `workspace-validator` through four evaluation families:

1. the public CLI through only its executable and in-band output;
2. the public CLI with public documentation but without bundled agent
   instructions;
3. the AI-Tool operational interface;
4. the complete AI-Engineering Flow.

These are evaluation families, not runtime modes, release phases, or additional
workspace contracts. Phase 7 implements and executes the complete program after
the Human, AI-Tool, and AI-Engineering contracts are available. Earlier phases
define representative scenarios and observable acceptance criteria for the
surface they introduce.

The program produces evidence for improving CLI ergonomics, built-in help,
diagnostics, public documentation, skills, prompts, routing, and process
contracts. It does not train models, select one provider as normative, or treat
one successful run as proof of general agent reliability.

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
│   ├── cli-only-discovery/
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

The family order is analytical: Family A measures CLI self-discovery, Family B
measures the incremental value of public documentation, Family C measures the
incremental value of AI-Tool, and Family D evaluates the composed engineering
system. It does not define runtime dependencies or require one family to
execute another.

## 3. Evaluation Family A: CLI-Only Discovery

### 3.1 Objective

Measure whether an agent can discover and use the executable through the CLI's
own affordances, without external documentation or bundled agent instructions.
This evaluates command naming, help structure, diagnostics, introspection, and
self-description rather than the agent's ability to recall project-specific
commands from prior knowledge.

The scenario provides the executable name, workspace fixture, task prompt, and
ordinary execution permissions. The agent may use `--help`, `--version`, error
messages, and any inspection, explanation, or schema output it discovers by
interacting with the CLI itself.

### 3.2 Context Isolation

Withhold all `workspace-validator` README files, reference documentation,
externally supplied schemas, examples, skills, source code, implementation
plans, expected answers, and traces from other evaluation runs. The consumer
project fixture and any domain context declared identically for paired trials
remain available, but they must not contain hidden validator usage
instructions. A schema or explanation emitted by a CLI command discovered
during the trial is in-band CLI output and remains allowed.

Disable external documentation lookup and unrestricted network access for this
family. If the fixture requires network behavior, expose only controlled
scenario endpoints that cannot provide validator instructions.

Do not provide command syntax beyond the executable name. Keep the prompt,
fixture, permissions, and expected outcome aligned with comparable Family B and
Family C scenarios whenever the task is supported in all three contexts.

Context isolation controls only evidence supplied or reachable during the
trial; it cannot erase knowledge already present in a model. Record that
limitation, prefer version-specific behavior unlikely to exist in prior
training, and never claim that the family proves complete absence of model
prior knowledge.

### 3.3 Required Scenarios

Include at minimum:

- discover the top-level command structure and relevant subcommand help;
- identify how to initialize, validate, inspect, and execute a representative
  configuration workflow;
- discover groups, suites, checks, resolved commands, and effective directories
  through CLI affordances;
- interpret representative result classes and exit behavior from in-band
  output;
- recover from invalid usage or configuration through help and diagnostics;
- discover and use machine-readable schema or explanation commands when the
  task requires them;
- avoid source inspection, external documentation, hidden skill context, and
  invented commands or flags.

### 3.4 Evaluation Questions

Measure task completion, the path to the first valid command, help traversal,
command and argument accuracy, corrective attempts, interpretation of output
and exit status, unsupported assumptions, unnecessary workspace changes, and
the point at which CLI self-description no longer supplies enough context.

## 4. Evaluation Family B: Documentation-Only Direct CLI

### 4.1 Objective

Measure whether an agent with no AI-Tool or AI-Engineering instructions can use
the public executable correctly from public documentation. This evaluates CLI
discoverability, reference quality, examples, diagnostics, and model-neutral
usability.

This family is not an additional governed AI flow. The evaluated agent receives
no claim to AI-Tool policy enforcement, AI-Engineering persistence, or bundled
agent routing. It operates as a direct CLI consumer under the evaluation
harness's ordinary permissions and human controls.

### 4.2 Context Isolation

Provide only the released CLI, `--help`, schemas, examples, and public
documentation declared by the scenario. Withhold the distributed skill,
AI-Tool references, AI-Engineering references, internal implementation plans,
expected answers, and traces from other evaluation runs.

Use paired scenarios with Families A and C whenever the same task can be
expressed across all three contexts. The comparisons separately measure the
value added by public documentation over CLI-only discovery and by AI-Tool over
public documentation without changing the workspace, prompt intent,
permissions, or expected CLI outcome.

### 4.3 Required Scenarios

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

### 4.4 Evaluation Questions

Measure successful task completion, documentation lookup path, command and
argument accuracy, number of corrective attempts, interpretation of reports and
exit codes, unsupported assumptions, unnecessary workspace changes, and the
point at which documentation or CLI feedback fails to supply enough context.

## 5. Evaluation Family C: AI-Tool Interface

### 5.1 Objective

Measure whether AI-Tool is a clear, safe, and composable agent interface over
the deterministic CLI. The evaluated surface is AI-Tool, not the underlying
model's general ability to discover the CLI independently and not the complete
AI-Engineering reasoning system.

### 5.2 Consumer Shapes

Exercise AI-Tool through both supported consumer shapes:

- a human gives a natural task prompt to an agent, which routes to one or more
  bounded AI-Tool operations;
- an AI-Engineering scenario invokes a required AI-Tool operation as a
  downstream consumer, while the evaluation scores only the operation boundary
  and returned evidence.

The second shape is contract dogfooding. It proves that AI-Tool is useful to
another agent workflow without conflating that result with the end-to-end
quality of AI-Engineering.

### 5.3 Required Scenarios

Cover `policy`, `config`, `run`, `triage`, and `audit` with ordinary, ambiguous,
blocked, and adversarial prompts. Include at minimum:

- selecting the correct primary operation from a natural human request;
- loading only the required operation and retrieving only relevant shared
  knowledge through `workspace-validator knowledge`;
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

### 5.4 Evaluation Questions

Measure routing accuracy, task completion, policy compliance, command accuracy,
unnecessary commands, human interventions, context loaded, report
interpretation, safety violations, unsupported claims, and usefulness of the
bounded result to both human and agent consumers.

## 6. Evaluation Family D: AI-Engineering End To End

### 6.1 Objective

Measure the complete AI-Engineering product on representative projects and
natural human prompts. AI-Tool is infrastructure in this family; the evaluated
surface is the engineering orchestration, analytical quality, persistence,
continuation, remediation discipline, and human collaboration of Processes 0
through 3.

### 6.2 Required Scenarios

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

### 6.3 Evaluation Questions

Measure process selection, prerequisite validation, AI-Tool reuse, justified
direct CLI use, domain and coverage reasoning, evidence traceability,
uncertainty preservation, policy decisions, artifact validity, continuation
behavior, human-decision quality, remediation scope, duplicate work, report
persistence, fresh-context recovery, and usefulness of the final engineering
result.

## 7. Assertions, Rubrics, And Interpretation

Use deterministic assertions for facts such as:

- selected routes and loaded references;
- selected knowledge document IDs and exact CLI retrieval commands;
- exact executable, arguments, working directory, and exit status;
- schema-valid reports and process artifacts;
- filesystem and repository mutations;
- policy validation and effective decision;
- forbidden policy mutation or undeclared persistence;
- artifact IDs, paths, digests, relationships, and immutability;
- use of AI-Tool when an applicable operation exists;
- absence of external documentation, pre-supplied schemas, examples, skills,
  validator source code, and prior traces in CLI-only trials;
- absence of AI-Tool and AI-Engineering context in documentation-only trials.

Use reviewed rubrics for qualities that cannot be reduced to exact equality,
including diagnostic usefulness, clarity, proportionality, uncertainty,
evidence-to-conclusion alignment, and quality of human decision requests.
Rubrics define anchors and failure examples before evaluating model output.

Do not rank a model or product surface from one run. Report per-scenario
outcomes, repeated-run variation, deterministic failures, rubric distributions,
human-intervention counts, and unsupported behavior. Compare models only across
equivalent inputs and declared capabilities.

## 8. Feedback Boundaries

Classify every observed failure before changing the product:

- Family A primarily informs command naming, built-in help, introspection,
  diagnostics, and CLI self-description;
- Family B primarily informs examples, schemas, and public documentation;
- Family C primarily informs AI-Tool routing and operation references;
- Family D primarily informs AI-Engineering process prompts, persistence,
  handoffs, and orchestration;
- a failure shared across all families may indicate a runtime, schema, report,
  or public-contract problem;
- a provider-specific failure remains identified as such until reproduced on
  another model or explained by a declared capability difference.

After a change, rerun the smallest affected scenario set and the cross-family
regressions that share its contract. Preserve prior run evidence according to
the declared retention policy so quality changes remain measurable.

## 9. Completion Criteria

The evaluation program is complete when:

- all four families have versioned scenarios, fixtures, deterministic
  assertions, and reviewed rubrics;
- evaluation source assets are isolated from generated runs and from normal
  crate runtime dependencies;
- the declared model matrix runs from fresh contexts with reproducible inputs;
- CLI-only agents complete representative workflows using only in-band CLI
  affordances and output;
- direct documentation-only agents complete representative CLI workflows
  without hidden skill context and are compared with paired CLI-only baselines
  where applicable;
- AI-Tool is evaluated through human prompts and AI-Engineering consumption and
  compared with paired documentation-only baselines where applicable;
- AI-Engineering is evaluated independently as the end-to-end product;
- results contain enough evidence to attribute failures to CLI
  self-description, public documentation, AI-Tool, AI-Engineering, the runtime,
  or a model capability;
- release conclusions identify model coverage, repetitions, residual failures,
  and uncertainty instead of claiming universal agent correctness.
