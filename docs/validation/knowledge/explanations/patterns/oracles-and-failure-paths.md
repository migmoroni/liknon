# Oracles And Failure Paths

## Problem And Context

A mechanism may execute representative inputs yet fail to distinguish correct
behavior, or it may test only success while safety depends on rejection,
diagnostics, atomicity, cleanup, or recovery.

## Forces

Independent oracles are stronger but can be costly to construct. Real failure
injection improves confidence but may mutate data or require privileges.
Mocks make failures controllable but can diverge from real collaborators.

## Pattern

Define acceptable and unacceptable observations before execution. Prefer
public contracts, independent calculations, invariants, or externally visible
effects over implementation-derived expectations. Exercise malformed,
boundary, denied, interrupted, timeout, partial-write, and dependency-failure
paths that match the consequence. Verify exit status, diagnostics, preserved
state, cleanup, retry behavior, and absence of unauthorized effects.

The SWEBOK testing knowledge area provides terminology and context for testing
levels and techniques; this oracle-and-failure-path composition is local
synthesis. [Source: ieee.swebok-v4#testing](https://www.computer.org/education/bodies-of-knowledge/software-engineering)

## Consequences And Tradeoffs

The pattern exposes false success and makes error behavior part of the product
contract. It increases fixture and environment complexity, and injected faults
can become unsafe without isolated state and bounded authority.

## Example

For atomic configuration publication, observe that an interrupted write never
replaces an existing destination, leaves no partial canonical file, reports a
specific failure, and permits a later retry. A success-only creation test would
not establish these properties.

## Failure Modes

The pattern fails when the oracle repeats implementation logic, a mock cannot
produce the real protocol behavior, injected failure escapes its temporary
boundary, or assertions accept any error rather than the required contract.
Passing selected failures never proves exhaustive fault coverage.

## Related Knowledge

Use [Evidence Quality](../foundations/evidence-quality.md) for claim boundaries,
[Test Types](../foundations/test-types.md) for the observable boundary, and
[Isolation And Determinism](isolation-and-determinism.md) before destructive
failure injection.
