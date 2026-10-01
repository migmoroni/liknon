# Isolation And Determinism

## Question

How can validation results be made repeatable and attributable to the relevant
change?

## Apply The Pattern

Declare inputs, tool versions, environment variables, time, randomness,
locale, platform, filesystem state, network/service state, and concurrency that
can affect the result. Control or record each material input. Give each run an
isolated mutable area, stable fixtures, explicit cleanup ownership, and a
bounded working directory. Prefer locked dependencies and deterministic
generation when the ecosystem supports them.

Cargo's `--locked` option requires the existing lock file to remain unchanged,
which is useful evidence about dependency resolution but does not make compiler
output or tests universally reproducible.
[Source: rust.cargo#locked](https://doc.rust-lang.org/cargo/commands/cargo-test.html#manifest-options)

## Evidence And Limits

A repeatable result supports attribution under the controlled conditions. It
cannot establish behavior on excluded platforms, clocks, schedules, load
profiles, or external-service states. Excessive isolation can also conceal the
integration being validated.

## Flaky Results

Preserve the first result and diagnostic context. Do not convert a failure into
a pass by retry policy. Classify uncontrolled inputs, reproduce deliberately,
and either remove the source of nondeterminism, test it as a product property,
or report the evidence as unreliable. Google's published mitigation experience
shows why flakiness needs explicit detection and ownership.
[Source: google.flaky-tests#mitigation](https://research.google/pubs/flaky-tests-at-google-and-how-we-mitigate-them/)

## Cost, Mutation, And Next Step

Isolation may require temporary storage, service virtualization, containers,
or dedicated environments. Record those prerequisites and any cleanup or remote
mutation. Then use the [evidence chain](evidence-chain.md) to confirm that the
controlled environment still exposes the intended failure.
