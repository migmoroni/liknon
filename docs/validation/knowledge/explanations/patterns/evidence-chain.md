# Evidence Chain

## Problem And Context

A named tool or passing command is often presented as validation without a
traceable reason that its observation addresses a product consequence.

## Forces

Evidence must be specific enough to support a decision, cheap enough to run at
the right time, and independent enough to expose the target failure. Available
tools may produce convenient signals that do not observe the important claim.
Stronger boundaries often cost more and produce broader failures.

## Pattern

1. Name the consequence or invariant in product terms.
2. Form one bounded claim that reduces the uncertainty.
3. List plausible ways the claim could be false.
4. Choose an observable signal and an independent oracle.
5. Select success, boundary, and negative inputs.
6. Select a mechanism at the relevant boundary.
7. Record execution conditions, effects, and limitations.
8. Interpret the result only within those conditions.

NIST SSDF presents tasks and notional implementation examples rather than
treating a named tool as the practice outcome. That supports this local
synthesis without making its exact sequence normative.
[Source: nist.ssdf-1.1#tasks-and-evidence](https://csrc.nist.gov/pubs/sp/800/218/final)

## Consequences And Tradeoffs

The chain makes relevance and residual risk reviewable, helps remove redundant
checks, and exposes missing observations. It adds design and maintenance work,
and an apparently precise chain can still encode a false premise or weak
oracle.

## Example

Consequence: an installed CLI lacks required documentation. Claim: the source
archive contains the documented asset and embedding entrypoint. Failure mode:
package rules omit the asset. Observable: archive paths and installed CLI
output. Oracle: returned bytes equal canonical bytes. Mechanism: build and query
the package without publishing. Limitation: this says nothing about whether the
documentation answers every future question.

## Failure Modes

The pattern fails when it begins with a fashionable tool, uses “all tests pass”
as the claim, copies implementation logic into the oracle, ignores material
mutation or services, or replaces an unavailable observation with unrelated
evidence. Record a gap when the required signal does not exist.

## Related Knowledge

Start with [Evidence Quality](../foundations/evidence-quality.md). Combine this
pattern with [Isolation And Determinism](isolation-and-determinism.md) for
environment-sensitive signals and [Oracles And Failure Paths](oracles-and-failure-paths.md)
for negative behavior.
