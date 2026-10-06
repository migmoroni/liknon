# Evidence Quality

## Question

When does a check result meaningfully support a validation claim?

## Applicability And Boundaries

Use this foundation before treating any command, review, test, or external
attestation as evidence. It explains evidence quality, not which tool a
workspace must run or whether a product is correct in every respect.

## Conceptual Model

Trace an important decision through the following chain:

```text
consequence or invariant
  -> bounded claim
  -> plausible failure mode
  -> observable signal
  -> mechanism and oracle
  -> result and conditions
  -> limitation and residual uncertainty
```

An observation is what the mechanism reports. An oracle distinguishes an
acceptable observation from an unacceptable one. Evidence is the observation
interpreted against a bounded claim under recorded conditions. An inference is
the conclusion a reviewer draws; it remains no stronger than the boundary,
inputs, environment, and oracle.

## Decision Guidance

Prefer direct invariants, independently computed expectations, public
contracts, or externally visible consequences over assertions that repeat the
implementation. Select representative success, boundary, and failure inputs.
Preserve an unchanged failure before retrying it, then classify uncontrolled
time, randomness, concurrency, environment, product, test, or service inputs.

Record runtime, resource cost, privileges, network and external-service use,
and mutations to source, generated output, data, caches, or remote state.
Increase evidence depth with consequence and uncertainty, not with a universal
checklist.

## Evidence And Limitations

NIST SSDF describes review or analysis of human-readable code as a scoped way
to identify vulnerabilities; one analyzer therefore does not prove that
software is secure. [Source: nist.ssdf-1.1#verify-software](https://csrc.nist.gov/pubs/sp/800/218/final)

Primary research at Google reports flaky tests as an operational problem that
requires detection and mitigation; retries do not establish correctness.
[Source: google.flaky-tests#mitigation](https://research.google/pubs/flaky-tests-at-google-and-how-we-mitigate-them/)

A pass can miss excluded platforms, inputs, failure paths, or observables. A
failure can reflect a stale oracle or contaminated environment. Neither volume
nor tool prestige repairs an evidence chain aimed at the wrong claim.

## Related Knowledge

Apply the [Evidence Chain](../patterns/evidence-chain.md), control inputs with
[Isolation And Determinism](../patterns/isolation-and-determinism.md), and
strengthen observations with [Oracles And Failure Paths](../patterns/oracles-and-failure-paths.md).
