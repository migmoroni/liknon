# Plan D01: System Conformance And Agent Evaluation

## Status

`Provisional`. Execution is blocked until the required A, B, and C contracts
exist as integrated candidates.

## Objective

Verify the complete Liknon system across direct human use, direct agent use,
official governed workflows, and an external compatible workflow. Measure agent
behavior without transferring product publication ownership into this plan.

## Dependencies

D01 depends on the implemented product and workflow surfaces exercised by each
scenario. A family may begin only when its own prerequisites exist; independent
product release evidence is supplied by B01-B03, while launcher and complete-
suite evidence is supplied by B04.

## Deliverables

1. [End-To-End Scenario Matrix](00-end-to-end-scenario-matrix.md)
2. [Contract Isolation And Security](01-contract-isolation-and-security.md)
3. [Agent Evaluation Program](02-agent-evaluation-program.md)
4. [Documentation And Readiness](03-documentation-and-readiness.md)

## Exclusions

D01 owns integrated evidence, not product release. B01-B03 retain their
individual packaging and release gates; B04 retains launcher, aggregate
package, complete-suite, publication, recovery, and channel-readback gates.

## Acceptance Criteria

- [ ] All eight scenario shapes have versioned deterministic coverage.
- [ ] Every direct long/compact command and unified launcher namespace has
      deterministic parity, isolation, help, version, and failure coverage.
- [ ] The four agent families remain context-isolated and model-neutral.
- [ ] Security and contract-isolation scenarios pass before scores are used.
- [ ] Evidence attributes failures to the owning product, resource, or model
      boundary.
- [ ] D01 publishes no product artifact and holds no publication credential.

## Exit Criterion

The integrated candidate passes deterministic conformance and security
scenarios, and the evaluation program produces reviewable evidence for all four
agent-consumer families.
