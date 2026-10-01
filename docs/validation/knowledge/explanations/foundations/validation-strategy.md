# Validation Strategy

## Question

How should a contextual validation strategy be selected?

## Applicability And Boundaries

Use this foundation for a product change, release, or operation whose evidence
must be proportional and explainable. It does not define one risk score, tool
stack, schedule, or assurance level for every workspace.

## Conceptual Model

A strategy connects affected users and assets, consequences, invariants,
uncertainty, plausible failures, evidence dimensions, mechanisms, placement,
authority, and residual risk. External standards retain their own scope and
native levels. For example, WCAG A, AA, and AAA remain WCAG conformance levels;
they are not universal product assurance levels.
[Source: w3c.wcag-2.2#conformance-levels](https://www.w3.org/TR/WCAG22/#levels-of-conformance)

## Decision Guidance

1. Identify the product surface and affected users, data, assets, and platforms.
2. State consequences, invariants, and uncertainty without inventing a
   universal score.
3. Select applicable evidence dimensions and test boundaries.
4. Choose mechanisms only after defining the evidence they must produce.
5. Record prerequisites, timing, owner or authority, cost, mutation, privilege,
   network, and external services.
6. State gaps, meaningful overlap, limitations, and accepted residual risk.

This local synthesis follows the SSDF direction to adapt practices to risk; it
is not an SSDF conformance profile.
[Source: nist.ssdf-1.1#risk-based-customization](https://csrc.nist.gov/pubs/sp/800/218/final)

## Evidence And Limitations

A usable strategy names each material claim, mechanism, execution context,
expected observation, failure interpretation, limitation, and remaining
uncertainty. Placement changes feedback time and exposure, not what a check can
prove. A strategy can still be wrong if it overlooks a consequence or relies
on an unsupported oracle.

## Related Knowledge

Select dimensions with [Validation Model](validation-model.md), justify checks
with the [Evidence Chain](../patterns/evidence-chain.md), and use a
[Recipe](../../how-to/recipes/README.md) only as an adaptable example.
