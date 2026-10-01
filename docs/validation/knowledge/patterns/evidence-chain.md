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

NIST SSDF presents practices with tasks and notional implementation examples
rather than treating a named tool as the practice outcome. That supports the
direction of this local synthesis, not every detail of it.
[Source: nist.ssdf-1.1#tasks-and-evidence](https://csrc.nist.gov/pubs/sp/800/218/final)

## Example

Consequence: an installed CLI cannot retrieve a required guide. Claim: the
source archive contains the canonical knowledge tree and build entrypoint.
Failure mode: package include rules omit a guide or the embedding build logic.
Observable: archive path list plus queries made with a binary built from that
archive. Oracle: every catalog document ID returns bytes equal to its canonical
Markdown. Mechanism: inspect and verify the package without publishing.
Limitation: this does not establish that the guidance is sufficient for every
future validation question.

## Failure Modes

Avoid chains that begin with a fashionable tool, assert “all tests pass” as the
claim, reuse the implementation as its own oracle, or omit what a pass cannot
establish. If the necessary observation is unavailable, record the gap rather
than substitute convenient but unrelated evidence.

## Useful Combinations

Combine with [isolation and determinism](isolation-and-determinism.md) when the
signal is environment-sensitive and [oracles and failure paths](oracles-and-failure-paths.md)
when success-only evidence could hide the important failure.
