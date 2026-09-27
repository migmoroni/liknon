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

Chat identity and chat history are not sources of truth. They are not stored in
`.validation/persistence/`, referenced by `.validation/state.json`, or required
to resume a process. The agent reconstructs the required working context from
current user instructions, the installed skill, repository evidence, the
latest deterministic integrity inspection, and validated persistence
artifacts.

## 3. Recommended Organization

For sustained work, recommend two conversational contexts:

### Setup Context

Use one focused context for Processes 0 and 1:

- domain understanding;
- consequences and invariants;
- uncertainty and human-provided domain decisions;
- evidence coverage;
- reconciliation with `.validation/config.json`.

These processes benefit from sharing one conversational context because
coverage design directly depends on the domain model it is evaluating. After
Process 0 is persisted, the agent applies `processContinuation`: it proceeds for
`auto`, asks for `human`, or ends without a continuation prompt for `stop`.

### Operational Context

Use another focused context for Processes 2 and 3:

- interpretation of concrete validator reports;
- incident diagnosis;
- authorized remediation and focused verification;
- comparison against repository change;
- drift assessment and recalibration recommendations.

These processes benefit from sharing one conversational context because
incident evidence, remediation results, passing reports, and drift analysis
form one operational sequence. After Process 2 reaches and persists a passing
verified outcome, the agent applies the same continuation policy before Process
3.

This two-context organization is a quality recommendation. It is not a
required topology and does not change process semantics.

The two handoffs explain why these pairings are useful, but they do not bind a
handoff to the current chat. Automatic continuation may remain in the current
context, a human decision may continue or postpone it, and the next process may
always be invoked later from a fresh context when its preconditions hold.

## 4. Unified Context

A human may run Processes 0 through 3 in one chat. This remains fully supported
and produces the same artifacts, authorization boundaries, CLI behavior, and
report semantics.

A unified context can be practical for short work, initial exploration, or an
incident that immediately exposes a coverage assumption requiring review. The
human may start a fresh chat whenever the conversation becomes long, changes
purpose, or accumulates irrelevant assumptions.

## 5. Invocation And Continuity

The human invokes the required AI-Engineering process and states the current
goal.
The agent then:

1. loads the workspace-validator skill, the AI-Engineering process reference,
   and only the AI-Tool and knowledge references needed for that process;
2. runs `workspace-validator integrity check`;
3. when that inspection reports changes, inspects the deterministic diff and
   records a bounded AI assessment linked to the exact inspection before using
   existing analytical context, without repairing files or accepting a new
   baseline;
4. validates `.validation/policy.json` and loads the normative AI
   decision-policy contract;
5. reads current validated context from `.validation/persistence/` and the
   AI-owned namespace of `.validation/state.json` when the
   invoked process is persistent;
6. verifies linked repository, configuration, and exact report evidence,
   including the path and digest in local operational state;
7. performs the invoked process;
8. saves the completed bounded redacted process result, subject to
   `sensitiveDataPersistence` when protected content must remain;
9. applies `processContinuation` after Process 0 or after a passing verified
   Process 2 result;
10. classifies every other governed effect under all applicable decision
   categories and requests a free-text human decision exactly when the
   restrictive result is `human`.

The human does not need to reproduce an earlier chat or manually copy its full
history. If required state is missing, stale, or inconsistent, the agent
reports that condition and gathers only the evidence needed to continue safely.
Ordinary bounded redacted process-result persistence needs no additional
prompt; report persistence and protected content follow their dedicated
decision categories.

When the current domain has replaced the domain referenced by the preceding
sensorium, the agent treats the explicit `revalidation_required` state as a
Process 1 prerequisite. It does not use the preceding sensorium to begin an
operational process.

## 6. Human Responsibilities

The guide explains that the human:

- chooses whether to separate or combine conversational contexts;
- invokes the desired AI-Engineering process and supplies its immediate
  objective;
- owns and explicitly authorizes changes to `.validation/policy.json`;
- explicitly initializes or accepts an integrity baseline when desired;
- provides domain decisions that repository evidence cannot establish;
- answers policy-required free-text decisions and may constrain or perform the
  proposed action manually;
- may correct AI context without approving its creation;
- decides `0 -> 1` and `2 -> 3` handoffs when
  `processContinuation` is `human`.

The human does not manage artifact IDs or report digests, select the lexically
newest file, or keep a chat alive solely to preserve continuity.

## 7. Agent Responsibilities

This section tells the human what behavior to expect. It is descriptive
operator guidance, not an operational skill route. The guide explains that the
agent:

- does not infer process semantics or authority from which chat it is running
  in;
- does not require access to previous chat history;
- uses persistence as the continuity mechanism between conversations;
- begins every operation with the deterministic integrity check;
- never repairs detected changes or accepts a new integrity baseline merely
  because the check found them;
- records a bounded assessment tied to the exact changed inspection before
  relying on existing AI-Engineering context;
- validates the decision policy and follows its category tables without
  process-specific reinterpretation;
- validates current references before relying on them;
- resumes operational work from the exact report reference in local
  operational state rather than searching the report directory;
- rejects operational state whose domain, sensorium, or config references no
  longer match current setup state;
- writes bounded redacted process results without a separate persistence
  prompt, while applying `sensitiveDataPersistence` to protected content that
  remains;
- treats a persisted result as context and applies `processContinuation` before
  either related next process;
- preserves normal authorization boundaries outside persistence;
- can recommend changing conversational context for clarity but never makes it
  a prerequisite for validation.

## 8. Documentation Requirements

The public guide must:

- label the two-context arrangement as recommended, not mandatory;
- present the unified context as fully supported;
- distinguish conversational context from execution environment and consumer
  workspace;
- explain that every AI operation checks workspace integrity first and that
  baseline initialization or acceptance is always explicit;
- show separate-context and unified-context diagrams;
- explain that persistence, not chat history, provides continuity;
- explain all three continuation states and that only `human` produces a
  free-text handoff decision;
- link to the normative AI decision-policy document without duplicating or
  paraphrasing its category-level tables;
- avoid provider-specific chat features or assumptions;
- avoid implying that the CLI reads AI persistence;
- link to the AI-Engineering Flow, authorization model, Human Flow, and AI-Tool
  Flow without duplicating their contracts;
- remain outside the distributed skill routes so agent behavior never depends
  on human-only conversational guidance.

## 9. Verification

Documentation and forward trials verify that:

- one session can complete all four processes;
- a fresh session can continue with Process 1 after Process 0;
- a separate operational session can continue with Processes 2 and 3 after
  setup;
- a fresh setup session can perform recalibration after a drift recommendation;
- a fresh operational session resolves the exact stored report without human
  report selection;
- each fresh or continued AI operation checks integrity before loading
  analytical context;
- a changed integrity inspection is assessed without implicit repair or
  baseline acceptance;
- domain replacement requires a fresh Process 1 result before Process 2 or
  Process 3;
- no artifact or schema requires a chat or session identifier;
- changing conversational organization does not change executable policy or
  validator results;
- `auto`, `human`, and `stop` handoffs preserve the completed preceding process
  and behave consistently across conversational contexts;
- decision-category outcomes remain identical when the same action and policy
  are evaluated in a different chat.
