# Validation Model

## Question

Which independent evidence dimensions may be relevant, and what can each
establish?

## Applicability And Boundaries

Use this model before choosing tools. Select dimensions from concrete
consequences, invariants, and uncertainty. No dimension, tool, passing gate, or
universal checklist establishes complete correctness.

## Conceptual Model

Validation dimensions include formatting and linting; static analysis; unit,
component, integration, contract, end-to-end, and smoke tests; build, package,
installation, and artifact inspection; persistence, schema, and generated
output; accessibility; dependency, license, and supply-chain security;
repository reproducibility; platform and runtime support; performance; and
property-based, fuzz, and mutation testing.

Each dimension answers a different bounded question. Static analysis observes
modeled properties without running the product. Behavioral tests observe
selected executions. Packaging observes deliverable assembly. Accessibility
combines machine-observable and human-dependent criteria. Supply-chain and
reproducibility evidence depend on time-sensitive inputs and declared
environments.

## Decision Guidance

Start with the least costly evidence that directly observes the important
failure. Add an orthogonal dimension when a blind spot matters. Preserve
apparent overlap only when mechanisms, boundaries, or oracles differ. Record
why an expensive, privileged, mutating, networked, or external-service check is
proportionate.

## Evidence And Limitations

NIST SSDF expresses secure-development practices as adaptable outcomes and
tasks rather than one required tool, which supports contextual selection.
[Source: nist.ssdf-1.1#practice-model](https://csrc.nist.gov/pubs/sp/800/218/final)

The model is an inventory for reasoning, not a requirement to run every
dimension. Missing evidence remains explicit residual uncertainty; duplicate
commands aimed at the same observation add cost without equivalent confidence.

## Related Knowledge

Use [Test Types](test-types.md) for test boundaries, [Validation Strategy](validation-strategy.md)
for selection, and the [Evidence Chain](../patterns/evidence-chain.md) to justify
each mechanism.
