# Validation Model

## Question

Which independent evidence dimensions may be needed, and what can each answer?

## Applicability And Limits

Use this map before choosing tools. Select dimensions from concrete
consequences, invariants, and uncertainty. No dimension, tool, passing gate, or
universal checklist establishes complete correctness. NIST's SSDF similarly
organizes secure-development practices as outcomes and tasks that organizations
adapt to risk rather than as one tool prescription.
[Source: nist.ssdf-1.1#practice-model](https://csrc.nist.gov/pubs/sp/800/218/final)

## Evidence Dimensions

| Dimension | Question it helps answer | Principal limitation and adjacent evidence |
| --- | --- | --- |
| Formatting and linting | Does authored text follow selected mechanical and diagnostic rules? | Style is not behavior; combine with static analysis and tests. |
| Type and static analysis | Can declared types, control/data-flow rules, or analyzers reject known defect classes without executing the product? | Model and rule coverage are bounded; dynamic behavior remains. |
| Unit tests | Does a small owned unit satisfy examples and invariants at a controlled boundary? | Test doubles and narrow scope may hide integration failures. |
| Component tests | Does a deployable or cohesive component behave through its supported boundary? | External integration and complete user journeys remain outside. |
| Integration tests | Do selected real collaborators interoperate under a representative setup? | Setup fidelity and unexercised combinations limit inference. |
| Contract tests | Do producer and consumer observations conform to an agreed interface? | Agreement can still be incomplete or semantically wrong. |
| End-to-end tests | Does a representative user journey traverse the assembled system? | High cost and broad failures make diagnosis and exhaustive coverage difficult. |
| Smoke tests | Can a deployed or packaged system perform a minimal critical operation? | Breadth is deliberately small; success is not release completeness. |
| Build, package, install, artifact inspection | Can source become the intended deliverable and does that deliverable contain expected assets? | A build says little about runtime correctness or deployability elsewhere. |
| Persistence, schema, generated output | Do stored/generated structures satisfy current shape, integrity, and reproducibility expectations? | Shape alone does not prove semantic correctness or safe migration. |
| Accessibility | Can users with relevant disabilities perceive, operate, understand, and robustly access the interface under the selected criteria? | Automation covers only a subset; conformance needs scoped human and technical evaluation. |
| Dependency, license, supply-chain security | Are inputs identified and checked against selected vulnerability, policy, provenance, and integrity claims? | Databases and attestations are incomplete and time-sensitive. |
| Repository consistency and reproducibility | Did validation preserve tracked state and can inputs produce repeatable results under declared conditions? | Clean state does not prove correct output; environment parity still matters. |
| Platform and runtime support | Does the product build and behave on each promised platform/runtime combination? | One host cannot establish support for untested targets or services. |
| Performance and load | Are latency, throughput, resource, and saturation claims met under a stated workload? | Results depend on workload, environment, warm-up, and measurement error. |
| Property-based testing | Do generated cases preserve declared properties across a broad input space? | Generators and properties are themselves oracles and may omit important regions. |
| Fuzz testing | Do high-volume generated/mutated inputs expose crashes, hangs, or sanitizer findings? | Absence of findings is time- and corpus-bounded, not proof of safety. |
| Mutation testing | Do tests detect selected synthetic changes to implementation behavior? | Mutants approximate faults and can be equivalent or irrelevant. |

## Selection Guidance

Start with the cheapest evidence that directly observes the important failure,
then add an orthogonal dimension where a blind spot matters. Apparent overlap
can be useful when evidence has different mechanisms or boundaries; duplicate
commands that establish the same bounded claim add cost without much confidence.
Record what was not tested and the residual uncertainty.

## Next Steps

Use [validation strategy](validation-strategy.md) to select dimensions,
[test types](test-types.md) to choose a boundary, and
[evidence chain](../patterns/evidence-chain.md) to connect each check to a claim.
