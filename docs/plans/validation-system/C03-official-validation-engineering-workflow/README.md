# Plan C03: Official Validation-Engineering Workflow

## Status

`Provisional`. Implementation is blocked by C01.0, B02.0, B03.0, the resulting
product implementations, and the official action resources used as declared
dependencies.

## Objective

Deliver one official, replaceable workflow for sustained validation-engineering
work. Its local graph models domain, coverage, incidents, and drift while using
generic C01 policy and persistence contracts.

## Dependencies

C03 depends on implemented C01 governance, approved and implemented B02/B03
resource contracts, the Validation CLI, and the C02 operational resources it
declares. D01 is a downstream evaluator, not a prerequisite.

The workflow may bind the direct capability commands or their explicit B04
launcher namespaces. `liknon-validator <args>` and
`liknon validate <args>` reach the same validation capability; launcher use
does not change policy, operation ownership, or the workflow graph.

## Deliverables

1. [Workflow Contract](00-workflow-contract.md)
2. [Domain And Coverage Setup](01-domain-and-coverage-setup.md)
3. [Triage And Drift Loop](02-triage-and-drift-loop.md)
4. [Human Work-Context Guidance](03-human-work-context-guidance.md)
5. [Verification](04-verification.md)

## Exclusions

The process labels `0` through `3`, their profile bindings, and the transitions
`0 -> 1` and `2 -> 3` belong only to this workflow resource. They are not
platform phases, CLI branches, policy categories, or conventions imposed on
external workflows.

C03 does not own generic policy levels, product command grammars, resource
installation, model hosting, product publication, or cross-model scoring.

## Acceptance Criteria

- [ ] The graph and all profile/skill bindings come from locked resources.
- [ ] Every completed node creates immutable, explicitly related evidence.
- [ ] Continuation applies only to the two workflow-declared eligible edges.
- [ ] No CLI contains official node, transition, profile, or skill IDs in
      control flow.
- [ ] A neutral workflow with a different graph passes the same contracts.

## Exit Criterion

An external host can conduct the complete official graph from exact locked
resources and explicit immutable evidence, while an unrelated workflow with a
different graph passes the same generic application contracts.
