# Evidence Chain

## Question

How can a validation mechanism be justified from a real risk or invariant?

## Apply The Pattern

1. Name the consequence or invariant in product terms.
2. Form one bounded claim whose truth would reduce the uncertainty.
3. List plausible ways the claim could be false.
4. Choose an observable signal for those failure modes.
5. Define an oracle and representative success, boundary, and negative inputs.
6. Select a mechanism that produces the signal at the relevant boundary.
7. Record prerequisites, execution context, cost, mutation, and external state.
8. Interpret the result only within those conditions.
9. State blind spots and residual uncertainty.

NIST SSDF tasks pair outcomes with examples of evidence rather than treating a
named tool as the outcome. That supports the direction of this local pattern,
not every detail of it.
[Source: nist.ssdf-1.1#tasks-and-evidence](https://csrc.nist.gov/pubs/sp/800/218/final)

## Example

Consequence: a release archive omits the distributed skill. Claim: the archive
contains every routed skill file. Failure mode: package include rules omit the
generated knowledge projection. Observable: archive path list. Oracle: every
manifest route and projected file is present exactly once. Mechanism: inspect
the package list without publishing. Limitation: presence does not establish
byte identity, so a separate projection comparison is required.

## Failure Modes

Avoid chains that begin with a fashionable tool, assert “all tests pass” as the
claim, reuse the implementation as its own oracle, or omit what a pass cannot
establish. If the necessary observation is unavailable, record the gap rather
than substitute convenient but unrelated evidence.

## Useful Combinations

Combine with [isolation and determinism](isolation-and-determinism.md) when the
signal is environment-sensitive and [oracles and failure paths](oracles-and-failure-paths.md)
when success-only evidence could hide the important failure.
