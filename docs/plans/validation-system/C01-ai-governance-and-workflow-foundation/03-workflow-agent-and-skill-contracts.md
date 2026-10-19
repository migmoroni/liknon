# Phase C01.3: Define Workflow, Agent, Skill, And Tool Contracts

## Status

`Provisional`. Blocked by C01.0 and the B02.0 public resource contract.

[Back to Plan C01](README.md)

## Purpose

Define compositional resource responsibilities so any compatible workflow can
use Liknon products without application branches for official content.

## Resource Responsibilities

| Kind | Responsibility | Must not own |
| --- | --- | --- |
| Workflow | Graph, nodes, actions, transitions, preconditions, continuation-eligible edges, referenced resources, and declared artifacts | Model hosting, policy values, implicit installation, or application routing |
| Agent profile | Role, posture, scope, and behavioral constraints selected for a node | Graph transitions, tool authority, or policy |
| Skill | Reusable operational instructions and declared capabilities/effects | Automatic activation, credentials, or policy exemption |
| Tool descriptor | Stable description of an external capability and its typed operation contract | Executable payload, credential, installation, or authority |
| Knowledge | Bounded explanatory or reference content | Workflow control or command execution |
| Prompt | Optional authored interaction material below superior instructions | System authority or hidden activation |
| Guide | Human/agent guidance with declared audience and compatibility | Executable policy |
| Template | Declarative starting material requiring explicit destination and mutation handling | Silent workspace mutation |
| Collection | Versioned references to resources | Copied payload ownership or implicit activation |

## Generic Workflow Contract

A workflow candidate must be able to declare, through the B02-approved schema:

- stable identity and version;
- entry and terminal nodes;
- stable node, action, transition, and edge IDs;
- transition preconditions and continuation eligibility;
- referenced resources and version constraints;
- per-node agent-profile selection;
- typed tool operations and declared effects;
- bounded inputs and outputs;
- declared artifact types and producing actions;
- compatible bootstrap, policy, resource, and application capabilities.

The platform does not require numeric nodes, a fixed number of phases, one
persona, a particular graph shape, or one official operational skill.

## Authority Rules

- A resource manifest describes content and compatibility; it cannot authorize
  itself.
- Installation, trust class, namespace, signature, or review status never
  activates instructions.
- The human selects a workflow or bounded operation.
- The external host evaluates policy before effects and conducts the selected
  graph.
- Tool may return exact graph data but does not interpret transitions.
- Manager may install graph resources but does not execute them.
- Validation CLI may be referenced as a capability but does not become a
  workflow engine.

## Neutral Fixtures

Define fixtures with:

- nonnumeric and irregular nodes;
- multiple profiles within one workflow;
- an alternate operational skill;
- parent and child workflows with overlapping effects;
- absent optional resources;
- incompatible bootstrap/policy ranges;
- artifact declarations with distinct repository treatments;
- an external namespace whose content is otherwise equivalent to an official
  fixture.

## Exit Criterion

The resource contracts compose generically, official and external fixtures use
the same paths, and no application needs identity-specific routing.
