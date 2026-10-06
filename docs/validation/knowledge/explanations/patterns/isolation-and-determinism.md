# Isolation And Determinism

## Problem And Context

Validation results become hard to attribute when dependency identities,
workspace state, time, randomness, concurrency, platform, or external services
change independently of the product change.

## Forces

Complete isolation can be expensive or unrealistic. Shared caches improve
speed but carry hidden state. Networked services improve fidelity but introduce
availability and mutable data. Strict input control improves repeatability but
can hide production variability that must instead be exercised deliberately.

## Pattern

Declare material inputs and tool version lines. Pin or record dependency
identity, locale, environment, time, randomness, concurrency, filesystem,
network, and service state. Give each run a bounded mutable area and explicit
cleanup ownership. Compare repository state before and after authorized
effects. Preserve original failures and classify nondeterminism rather than
retrying toward a pass.

The Reproducible Builds definition includes source, build environment,
instructions, and dependencies among the inputs needed for repeatable output.
[Source: reproducible-builds.definition#definition](https://reproducible-builds.org/docs/definition/)

## Consequences And Tradeoffs

Controlled inputs make results more repeatable and failures more attributable.
They require storage, fixtures, environment setup, and periodic review. An
isolated result may still omit a supported platform or external interaction.

## Example

A package check uses a committed lockfile, a populated offline cache, an empty
temporary install root, and byte comparison against canonical assets. The
check records the tool major line and fails on repository mutation. This
controls dependency and output inputs but does not establish behavior on
another operating system.

## Failure Modes

Ignoring hidden caches, shared ports, clock or random inputs, background
processes, and mutable service data makes repeated outcomes misleading.
Primary research describes systematic flaky-test detection and mitigation;
pass-seeking retries remain weak evidence.
[Source: google.flaky-tests#mitigation](https://research.google/pubs/flaky-tests-at-google-and-how-we-mitigate-them/)

The pattern is disproportionate when control costs exceed the consequence and
the residual variability can be recorded and interpreted safely.

## Related Knowledge

Use the [Evidence Chain](evidence-chain.md) to decide which inputs are material
and [Reproducible Release Evidence](../concerns/reproducible-release-evidence.md)
for source-to-artifact risk.
