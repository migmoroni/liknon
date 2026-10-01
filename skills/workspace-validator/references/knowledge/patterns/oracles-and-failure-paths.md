# Oracles And Failure Paths

## Question

How should validation distinguish correct behavior and exercise meaningful
failure handling?

## Oracle Selection

Prefer an oracle independent from the implementation under test: a stated
invariant, protocol/schema contract, known example, model, property, or
externally visible consequence. When exact output is unstable, assert the
stable semantic fields and explicitly exclude incidental presentation.

The Rust testing documentation distinguishes ordinary assertions, expected
panic behavior, and `Result`-returning tests; each observes a different outcome
and none automatically covers failure recovery.
[Source: rust.book-testing#test-organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html)

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
