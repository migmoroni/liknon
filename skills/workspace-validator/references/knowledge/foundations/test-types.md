# Test Types

## Question

Which test boundary best observes the failure in question?

## Practical Distinctions

Testing terminology varies across communities. The SWEBOK Guide organizes
software testing by levels, techniques, measures, and process while noting the
broader engineering context; this guide uses local practical boundaries and
does not claim universal definitions.
[Source: ieee.swebok-v4#testing](https://www.computer.org/education/bodies-of-knowledge/software-engineering)

| Type | Primary purpose and boundary | Typical ownership and relative cost | Main limitation/overlap |
| --- | --- | --- | --- |
| Unit | Verify one small unit through a controlled interface. | Code owner; low. | Doubles may hide integration; overlaps component tests in small modules. |
| Component | Verify one cohesive/deployable component through supported boundaries. | Component team; low to medium. | Meaning of “component” varies; may include real local infrastructure. |
| Integration | Verify interaction among selected real collaborators. | Collaborating owners; medium. | Does not necessarily cover a user journey or external contract. |
| Contract | Verify observations against a producer/consumer agreement. | Interface owners; medium. | A mutually accepted contract can omit product semantics. |
| End-to-end | Verify a representative journey through the assembled system. | Product/system owners; high. | Slow, diagnostically broad, and impractical for exhaustive combinations. |
| Smoke | Verify minimal critical availability after build/deploy/install. | Release/operations owners; low to medium. | Deliberately shallow and never a substitute for deeper tests. |

“Integration” may mean any test with a real dependency, a test across internal
modules, or a separately deployed-system test. “Component” may mean a UI
component or an independently deployable service. When the distinction changes
a decision, state the concrete boundary, real and replaced collaborators,
execution context, and owner rather than relying on the label.

## Selection And Composition

Choose the narrowest boundary that can observe the consequence without hiding
the relevant interaction. Add a broader test only for assembly or journey risk
that narrower tests cannot establish. Add a narrower test when a broad failure
would be too slow or ambiguous to diagnose. Count different mechanisms as
complementary only when their observables or blind spots differ meaningfully.

## Evidence And Limits

Record inputs, environment, oracle, result, duration, mutation, and excluded
boundaries. A passing unit suite cannot establish packaging or deployment; a
passing end-to-end journey cannot establish all branches or malformed inputs.

## Next Steps

Apply [oracles and failure paths](../patterns/oracles-and-failure-paths.md), then
select implementation-specific guidance through the [catalog](../catalog.json).
