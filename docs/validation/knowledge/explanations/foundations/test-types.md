# Test Types

## Question

Which test boundary can observe the relevant failure?

## Applicability And Boundaries

Use these local practical distinctions when selecting test evidence. Community
terminology varies, so state the concrete boundary, real and replaced
collaborators, execution context, and owner whenever a label could change the
decision.

## Conceptual Model

| Type | Primary boundary | Principal limitation |
| --- | --- | --- |
| Unit | One small unit through a controlled interface | Doubles may hide integration. |
| Component | One cohesive component through supported boundaries | “Component” varies by architecture. |
| Integration | Selected real collaborators | It may omit user journeys and external contracts. |
| Contract | Producer and consumer observations against an agreement | The agreement may omit product semantics. |
| End-to-end | A representative journey through an assembled system | It is broad, costly, and non-exhaustive. |
| Smoke | Minimal critical behavior after build, install, or deploy | It is deliberately shallow. |

The SWEBOK Guide organizes testing by levels, techniques, measures, and
process; these local boundaries are an editorial synthesis rather than
universal definitions. [Source: ieee.swebok-v4#testing](https://www.computer.org/education/bodies-of-knowledge/software-engineering)

## Decision Guidance

Choose the narrowest boundary that exposes the material consequence without
hiding the relevant interaction. Add a broader test for assembly or journey
risk that narrower evidence cannot establish. Add a narrower test when a broad
failure is too slow or ambiguous to diagnose. Treat mechanisms as
complementary only when their observables or blind spots materially differ.

## Evidence And Limitations

Record inputs, environment, oracle, result, duration, mutation, external
services, and excluded boundaries. Unit success does not establish packaging;
one end-to-end journey does not establish every branch; a smoke test does not
replace deeper behavioral evidence.

## Related Knowledge

Use [Validation Model](validation-model.md) to select evidence dimensions and
[Oracles And Failure Paths](../patterns/oracles-and-failure-paths.md) to design
acceptance and negative cases.
