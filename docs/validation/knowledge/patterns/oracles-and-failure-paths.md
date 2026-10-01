# Oracles And Failure Paths

## Question

How should validation distinguish correct behavior and exercise meaningful
failure handling?

## Oracle Selection

Prefer an oracle independent from the implementation under test: a stated
invariant, protocol/schema contract, known example, model, property, or
externally visible consequence. When exact output is unstable, assert the
stable semantic fields and explicitly exclude incidental presentation.

The SWEBOK testing knowledge area treats test techniques and measures as
separate concerns within a wider testing process. Local synthesis: choosing an
input technique does not supply an oracle, and recording a result does not show
that the oracle observed the intended consequence.
[Source: ieee.swebok-v4#testing](https://www.computer.org/education/bodies-of-knowledge/software-engineering)

## Failure Paths

Derive negative cases from plausible failure modes: malformed or boundary
input, unavailable prerequisites, permission denial, timeouts, interruption,
partial writes, stale state, dependency failure, and cleanup failure. Validate
both the primary result and safety invariants such as no unintended mutation,
no leaked secret, and recoverable state.

## Evidence And Limits

A success-only path shows that one expected case can work. It cannot establish
diagnostic quality, atomicity, rollback, or safe behavior under failure. A
negative test establishes only its chosen fault and observation boundary.
Fault injection itself may be unrealistic or may bypass the behavior under
study.

## Cost And Selection

Begin with deterministic local faults at the narrowest meaningful boundary.
Use real service or platform failures when simulation would hide the relevant
contract, and record external mutation and recovery ownership. Pair this pattern
with [isolation and determinism](isolation-and-determinism.md) and report
unexercised failures as residual uncertainty.
