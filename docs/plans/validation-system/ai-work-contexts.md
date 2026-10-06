# Human Guidance: AI Work Contexts

## 1. Status And Audience

This is cross-cutting human-only operator guidance for the AI-Engineering Flow,
not an implementation phase, AI process, runtime contract, skill-routing rule,
or persistence schema. It is not part of the Human Flow or AI-Tool Flow.

It specifies the public guide to be authored at:

```text
docs/validation/human/ai-work-contexts.md
```

The guide helps a human organize conversations with an AI agent for clearer,
more focused work. The validator and agent behavior remain independent from
that organization.

## 2. Definition

An AI work context is the conversational scope a human chooses for a body of
work. It may be one chat, a sequence of chats, or another agent session model.
It is not a workspace directory, execution environment, persisted artifact
type, or security boundary.

Chat identity and chat history are not process inputs. Continuity comes from
the exact domain, sensorium, incident, drift, and report artifacts supplied to
an invocation. Each artifact carries its own explicit references and digests.

## 3. Recommended Organization

For sustained work, recommend two conversational contexts.

### 3.1 Setup Context

Use one focused context for Processes 0 and 1:

- domain understanding;
- consequences and invariants;
- uncertainty and human-provided domain decisions;
- evidence coverage;
- reconciliation with `.validation/config.json`.

These processes benefit from sharing one conversational context because
coverage design directly depends on the domain model it evaluates. After
Process 0 persists its result, the agent revalidates the current policy and
Process 1 preconditions before applying `processContinuation`: it starts Process
1 for `auto`, asks for `human`, or ends without a continuation prompt for
`stop`.

### 3.2 Operational Context

Use another focused context for Processes 2 and 3:

- interpretation of concrete validator reports;
- incident diagnosis;
- authorized remediation and focused verification;
- comparison against repository change;
- drift assessment and recalibration recommendations.

These processes benefit from sharing one conversational context because
incident evidence, remediation results, passing reports, and drift analysis
form one operational sequence. After Process 2 persists a passing verified
outcome, the agent revalidates the current policy and Process 3 preconditions
before applying the same continuation policy.

This two-context organization is a quality recommendation. It is not a
required topology and does not change process semantics. Automatic continuation
may remain in the same conversation. A human may continue immediately,
postpone the next process, or invoke it later from another conversation by
supplying its exact prerequisites.

## 4. Unified Context

A human may run Processes 0 through 3 in one conversation. This remains fully
supported and produces the same artifacts, authorization boundaries, CLI
behavior, and report semantics.

A unified context can be practical for short work, initial exploration, or an
incident that exposes a coverage assumption requiring immediate review. The
human may start a fresh conversation whenever the discussion becomes long,
changes purpose, or accumulates irrelevant assumptions.

## 5. Invocation And Continuity

The human invokes the required AI-Engineering process, states the immediate
goal, and supplies the exact prerequisite artifact paths. A direct process
handoff may pass paths returned by the preceding process without asking the
human to transcribe them.

The agent then:

1. loads the workspace-validator skill, the requested AI-Engineering process
   reference, the immutable policy boundary, and only the AI-Tool references
   needed for it, then retrieves selected shared knowledge through the CLI;
2. verifies CLI and skill compatibility;
3. invokes `policy validate` for the explicit active workspace root and records
   the exact path, schema version, and digest of the existing valid policy;
4. verifies every explicitly supplied artifact and report through the
   deterministic persistence commands;
5. verifies the relationships and digests required by the requested process;
6. performs only the invoked process and its authorized effects;
7. validates and stores one new immutable UUIDv7 process artifact, applying
   `sensitiveDataPersistence` when protected content remains;
8. revalidates the current policy and the next process preconditions, then
   applies `processContinuation` after Process 0 or after a passing verified
   Process 2 result;
9. classifies every other governed effect under all applicable decision
   categories and requests a free-text human decision exactly when the
   restrictive result is `human`.

If Step 3 reports a missing policy, the process stops before Step 4. The agent
routes to the advisory AI-Tool `policy` operation, asks the human to run
`workspace-validator init --policy` personally, and explains that the human must
review `processContinuation` and every decision level. An invalid policy also
stops the process; the agent may explain diagnostics and recommend values but
never edits or replaces the file. After the human reports manual initialization
or correction, the process is invoked or resumed and validates the policy again.

A later conversation does not need the preceding chat transcript. It needs the
same explicit prerequisite files that the process contract requires. If a path,
digest, relationship, or policy is missing or invalid, the agent reports the
specific prerequisite and stops before relying on that evidence.

Process artifacts are immutable. A correction creates another artifact and the
caller chooses that exact file in a later invocation. The agent never selects
an artifact from directory order or filename recency.

## 6. Human Responsibilities

The guide explains that the human:

- chooses whether to separate or combine conversational contexts;
- invokes the desired AI-Engineering process and supplies its immediate
  objective;
- supplies exact prerequisite artifact paths when starting outside a direct
  process handoff;
- initializes `.validation/policy.json` directly through
  `workspace-validator init --policy`, then reviews and selects its continuation
  and decision levels;
- opens and changes `.validation/policy.json` manually; no agent performs that
  action on the human's behalf;
- provides domain decisions that repository evidence cannot establish;
- answers policy-required free-text decisions and may constrain or perform the
  proposed action manually;
- decides `0 -> 1` and `2 -> 3` handoffs when
  `processContinuation` is `human`.

The human does not need to manage artifact IDs or digests manually during a
direct handoff, select files by recency, or keep a conversation alive solely to
preserve continuity.

## 7. Agent Responsibilities

This section tells the human what behavior to expect. It is descriptive
operator guidance, not an operational skill route. The guide explains that the
agent:

- does not infer process semantics or authority from the conversation in which
  it runs;
- does not require access to previous chat history;
- verifies every explicit prerequisite before loading its analytical content;
- validates the existing decision policy through the deterministic read-only
  command for the same explicit workspace root, stops when it is absent or
  invalid, and follows its category tables without process-specific
  reinterpretation;
- never creates, edits, formats, replaces, deletes, moves, renames, restores,
  initializes, or indirectly mutates the policy, even after an explicit request
  or approval;
- resumes work from exact artifact and report paths rather than searching
  persistence directories;
- rejects a sensorium whose domain or config references do not match the
  invocation;
- writes one bounded redacted process result without a separate ordinary
  persistence prompt, while applying `sensitiveDataPersistence` to protected
  content that remains;
- treats a persisted result as evidence and applies `processContinuation`
  before either related next process;
- preserves normal authorization boundaries outside declared process-result
  persistence;
- may recommend changing conversational context for clarity but never makes it
  a prerequisite for validation.

## 8. Documentation Requirements

The public guide must:

- label the two-context arrangement as recommended, not mandatory;
- present the unified context as fully supported;
- distinguish conversational context from execution environment and consumer
  workspace;
- show separate-context and unified-context diagrams;
- explain explicit artifact handoff within one conversation and across fresh
  conversations;
- explain all three continuation modes and that only `human` produces a
  free-text handoff decision;
- link to the normative AI decision-policy document without duplicating or
  paraphrasing its category-level tables;
- avoid provider-specific chat features or assumptions;
- avoid implying that the validation planner or executor reads AI persistence;
- link to the AI-Engineering Flow, authorization model, Human Flow, and AI-Tool
  Flow without duplicating their contracts;
- remain outside the distributed skill routes so agent behavior never depends
  on human-only conversational guidance.

## 9. Verification

Documentation and forward trials verify that:

- one conversation can complete all four processes;
- a fresh conversation can invoke Process 1 from an explicitly supplied Process
  0 artifact;
- a separate operational conversation can invoke Process 2 or Process 3 from
  exact setup artifacts and report evidence;
- a fresh setup conversation can perform recalibration from an explicitly
  supplied drift artifact;
- missing or mismatched prerequisite paths stop with a precise explanation;
- a missing policy stops before process work and produces exact
  human-controlled initialization and review guidance;
- no artifact or schema requires a chat or session identifier;
- changing conversational organization does not change executable policy or
  validator results;
- `auto`, `human`, and `stop` handoffs preserve the completed preceding process
  and behave consistently across conversational contexts;
- every `0 -> 1` and `2 -> 3` handoff revalidates the current policy and the next
  process preconditions before continuation;
- no conversational context, request, or approval permits an agent to create,
  edit, format, replace, delete, move, rename, restore, initialize, or indirectly
  mutate policy;
- decision-category outcomes remain identical when the same action, inputs,
  and policy are evaluated in a different conversation.
