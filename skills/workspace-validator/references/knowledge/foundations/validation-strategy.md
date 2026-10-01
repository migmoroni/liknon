# Validation Strategy

## Question

How should validation evidence be selected proportionally and transparently?

## Decision Process

1. Identify the product, change, release, or operation.
2. Identify affected users, assets, data, platforms, and obligations.
3. State concrete consequences, invariants, and uncertainty.
4. Assess relevant likelihood and impact in context; do not invent a universal
   score.
5. Identify applicable quality characteristics and domain standards.
6. Select required evidence dimensions, reusable patterns, and test boundaries.
7. Select tools only after the evidence is understood.
8. Decide where and when each validation runs, including prerequisites,
   authority, mutation, cost, and external services.
9. Record gaps, meaningful overlap, uncertainty, and accepted residual risk.

This is local synthesis. It is compatible with the SSDF direction to select and
adapt practices according to organizational risk, but it is not an SSDF
conformance profile.
[Source: nist.ssdf-1.1#risk-based-customization](https://csrc.nist.gov/pubs/sp/800/218/final)

## Standards And Native Levels

Keep each external framework in its own version and scope. WCAG conformance
levels A, AA, and AAA apply to Web content under WCAG's conformance model;
SLSA tracks and levels apply to supply-chain threats; ASVS verification levels,
SAMM maturity, and OSPS maturity belong to their respective models. Never merge
these into a universal “assurance level” or infer equivalence.
[Source: w3c.wcag-2.2#conformance-levels](https://www.w3.org/TR/WCAG22/#levels-of-conformance)

## Evidence Placement

Fast deterministic checks can protect each change. Slower cross-platform,
integration, packaging, performance, accessibility, and supply-chain evidence
may run at merge, release, scheduled, or controlled-environment boundaries.
Placement changes feedback time and exposure; it does not change what a check
can prove.

## Output

A usable strategy names every material claim, its mechanism, context, owner or
authority, expected result, limitation, and residual uncertainty. Rejected
alternatives matter when they leave a known gap or explain a cost tradeoff.

## Next Steps

Build an [evidence chain](../patterns/evidence-chain.md), consult only applicable
topic guides, and use a [recipe](../recipes/README.md) solely as a worked example.
