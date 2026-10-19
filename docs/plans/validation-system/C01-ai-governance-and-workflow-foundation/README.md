# Plan C01: AI Governance And Workflow Foundation

## Status

`Decision gate`. Generic authority, policy, workflow, resource, and persistence
semantics are defined here, but C01 is not implementation-ready until product
ownership for policy and persistence operations is approved.

## Objective

Define the product-neutral contracts required for governed external AI use
without turning Liknon into a model host, hard-coding an official workflow, or
assigning future behavior to a CLI by accident.

## Dependencies

- [Current-State Assessment](../avaliacao-estado-atual.md)
- [Target-State Summary](../resumo-estado-alvo.md)
- [A01 Product And Repository Separation](../A01-product-and-repository-separation/README.md)
- approved B02/B03 product boundaries where a generic contract needs a CLI
  capability

C01 may define product-neutral semantics before B02/B03 implementation. It may
not finalize command syntax or package ownership that their decision gates have
not approved.

## Ownership

C01 owns:

- the program-owned AI bootstrap and authority hierarchy;
- the human-owned decision-policy contract;
- generic workflow, agent-profile, skill, tool-descriptor, and resource
  distinctions;
- generic continuation and policy evaluation semantics;
- generic artifact and report-persistence boundaries;
- the division of responsibility among the human, external AI host, workspace,
  and Liknon products;
- conformance requirements for any compatible workflow.

C01 does not own:

- validation configuration or execution semantics;
- Hub transport, resource installation, or Manager command grammar;
- Tool command grammar or machine envelopes;
- official action-workflow content;
- official validation-engineering nodes or transitions;
- model selection, conversation hosting, or cognitive orchestration;
- product publication.

## Exclusions

C01 does not define Manager or Tool command grammar, install resources, conduct
models, define an official workflow graph, execute validation, or publish any
product. It assigns generic semantics and requires C01.0 to assign deterministic
operation owners.

## Deliverables

| Unit | Document | Status | Result |
| --- | --- | --- | --- |
| C01.0 | [Boundary And Ownership](00-boundary-and-ownership.md) | `Decision gate` | Product-neutral responsibility matrix and resolved operational owners |
| C01.1 | [Bootstrap And Conformance](01-bootstrap-and-conformance.md) | `Provisional` | Stable entry and loading-order contract |
| C01.2 | [Decision Policy](02-decision-policy.md) | `Provisional` | Human-owned policy shape and deterministic evaluation semantics |
| C01.3 | [Workflow, Agent, And Skill Contracts](03-workflow-agent-and-skill-contracts.md) | `Provisional` | Generic compositional resource distinctions |
| C01.4 | [Persistence And Artifact Contracts](04-persistence-and-artifact-contracts.md) | `Provisional` | Generic immutable history and report/artifact boundaries |
| C01.5 | [Conformance](05-conformance.md) | `Provisional` | Neutral compatibility and abuse evidence |

## Blocking Decisions

Before C01 becomes implementation-ready, approve:

1. the product that exposes deterministic policy initialization, validation,
   explanation, and evaluation;
2. the product that exposes deterministic artifact/report persistence
   operations, if any CLI owns them;
3. the exact boundary between those operations and external-host orchestration;
4. the bootstrap-projection semantics that B03.0 must realize through its
   approved operation and envelope;
5. which shared package owns the machine contracts selected by A01;
6. which policy and artifact schemas are public compatibility surfaces;
7. the minimum pre-policy read-only operations required to evaluate policy
   without circular authorization.

## Architectural Invariants

1. The human owns policy and remains the only actor allowed to create, edit,
   move, restore, or delete it.
2. An external host conducts workflow cognition; no Liknon CLI selects a model
   or executes a workflow graph autonomously.
3. Resource installation never activates instructions.
4. Resource identity, namespace, trust, or official status never raises
   instruction priority.
5. Workflows select profiles, skills, tools, knowledge, prompts, and
   subworkflows through generic resource references.
6. Agent profiles define roles, not graph transitions or authority.
7. Skills define reusable operational guidance, not policy exemptions.
8. Tool descriptors declare capabilities; they do not contain credentials or
   grant execution permission.
9. Official resources use the same contracts as external resources.
10. No application branches on official workflow IDs, node names, process
    numbers, profile IDs, or skill IDs.
11. `processContinuation` applies only to edges declared continuation-eligible
    by the selected workflow.
12. Policy decision outcomes combine restrictively.
13. Every completed persistent workflow result is append-only and separately
    identifiable; no mutable state or integrity registry is introduced.

## Acceptance Criteria

- [ ] C01.0 records one owner for each deterministic operation.
- [ ] Bootstrap and policy loading contain no official workflow identity.
- [ ] Policy semantics apply equally to official and external workflows.
- [ ] The generic resource model distinguishes workflow, profile, skill, tool,
      knowledge, prompt, guide, template, and collection responsibilities.
- [ ] Persistence contracts are workflow-neutral and preserve immutable
      history without `state.json` or a general integrity subsystem.
- [ ] Validation reports and analytical artifacts remain distinct contracts.
- [ ] Neutral fixtures prove irregular graphs, nonnumeric nodes, multiple
      profiles, alternate skills, composed workflows, and restrictive policy
      combination.
- [ ] No C01 document claims an unresolved CLI command exists.
- [ ] C02 and C03 can consume C01 without receiving special platform branches.

## Result

C01 provides one generic governance substrate that products and external AI
hosts can implement consistently. Official workflows become replaceable
consumers instead of hidden roots of platform behavior.
