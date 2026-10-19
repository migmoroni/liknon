# Plan C02: Official Action Workflow

## Status

`Provisional`. Implementation is blocked by C01.0, B02.0, and B03.0 and by
the corresponding implemented product contracts.

## Objective

Deliver one official, replaceable workflow for bounded AI-assisted work with
Liknon. It routes a human request to focused operational resources without
activating the validation-engineering workflow or creating analytical history.

The workflow is an ordinary resource. Its official origin grants provenance,
not additional authority. A compatible external workflow or operational skill
must be able to replace it without an application branch keyed to an official
ID.

## Deliverables

1. [Workflow Contract](00-workflow-contract.md)
2. [Resources And Operations](01-resources-and-operations.md)
3. [Policy And Safety Application](02-policy-and-safety-application.md)
4. [Verification](03-verification.md)

## Dependencies

- A01 assigns authored resource ownership and target repository paths.
- C01 defines bootstrap, policy, capability, workflow, profile, skill, and
  persistence contracts.
- B02 defines how resources are acquired, locked, updated, and removed.
- B03 defines how installed resources are discovered and returned offline.
- The Validation CLI remains the only validation executor.

## Exclusions

C02 owns only the official action workflow and its official operational
resources. It does not own generic policy semantics, CLI command grammar,
resource installation, model hosting, the validation-engineering graph, or
product publication.

## Acceptance Criteria

- [ ] The workflow is an ordinary locked resource with no application branch.
- [ ] Every operation obeys C01 policy and approved B interfaces.
- [ ] No invocation mutates policy or creates engineering persistence.
- [ ] Missing resources stop without implicit Manager or network activity.
- [ ] A compatible external operational skill can replace AI-Tool.

## Exit Criterion

A human-selected external host can load the official action workflow, route one
bounded request through declared resources, obey C01 policy, invoke only
approved product interfaces, and finish without hidden persistence or implicit
workflow activation.
