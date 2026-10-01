# Evidence Quality

## Question

When does a check result meaningfully support a validation claim?

## The Evidence Chain

Trace every important decision through:

```text
consequence or invariant
  -> bounded claim
  -> plausible failure mode
  -> observable signal
  -> validation mechanism and oracle
  -> result
  -> limitation
  -> residual uncertainty
```

Executing a command establishes only that the mechanism ran under recorded
conditions. A result supports a claim only when the boundary, inputs, oracle,
environment, and failure interpretation are fit for that claim. NIST SSDF asks
for review and/or analysis of human-readable code to identify vulnerabilities;
that scoped outcome does not imply that one analyzer proves software secure.
[Source: nist.ssdf-1.1#verify-software](https://csrc.nist.gov/pubs/sp/800/218/final)

## Oracles And Representative Inputs

An oracle distinguishes acceptable from unacceptable observations. Prefer
direct invariants, independently computed expected values, protocol contracts,
or externally visible consequences over assertions that repeat implementation
logic. Fixtures should represent relevant partitions, boundaries, failure
conditions, and protected assets. A large fixture count cannot compensate for
an oracle that observes the wrong thing.

## False Confidence And Classification Error

A false negative misses a real problem; a false positive reports a problem that
is not present under the claim. Both consume trust. Passing evidence can still
mislead when important inputs, platforms, failure paths, or observables were
excluded. Failing evidence can mislead when an oracle is stale, the environment
is contaminated, or a heuristic is outside its intended scope.

## Determinism And Flakiness

Evidence should be reproducible from controlled inputs and an explicit
environment. A flaky check sometimes passes and sometimes fails without a
relevant product change, which weakens both outcomes and can train operators to
ignore real failures. Primary research at Google describes flaky tests as a
material operational problem and reports systematic detection and mitigation;
it does not imply that retries establish correctness.
[Source: google.flaky-tests#mitigation](https://research.google/pubs/flaky-tests-at-google-and-how-we-mitigate-them/)

Do not retry an unchanged failure merely to obtain a pass. First preserve the
original result, identify uncontrolled inputs, and classify whether the
nondeterminism belongs to the product, test, environment, or external service.

## Cost And Mutation

Record prerequisites, runtime, resource use, external services, privileged
access, and whether the mechanism can mutate source, generated output, data, or
remote state. Expensive or mutating evidence may be appropriate, but its timing
and authority must be explicit.

## Next Steps

Use the [evidence-chain pattern](../patterns/evidence-chain.md) to design a
check, [isolation and determinism](../patterns/isolation-and-determinism.md) for
reproducibility, and [oracles and failure paths](../patterns/oracles-and-failure-paths.md)
to strengthen observations.
