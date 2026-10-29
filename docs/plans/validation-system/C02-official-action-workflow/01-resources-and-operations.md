# Phase C02.1: Define Official Action Resources And Operations

## Status

`Provisional`. A01 ownership and the implemented B02/B03 interfaces are fixed
inputs. Exact manifests, paths, and bootstrap-aware retrieval calls depend on
C01.3 and the remaining B02 public resource decisions.

[Back to Plan C02](README.md)

## Purpose

Separate workflow routing from reusable operational instruction. The official
workflow may select an official AI-Tool skill, but AI-Tool remains an ordinary
replaceable resource and is not the `liknon-tool` application.

## Resource Set

The initial official set is expected to contain:

- one action workflow manifest and workflow document;
- one AI-Tool skill manifest, `SKILL.md`, and operation references;
- one generated or exact policy-boundary guide owned by C01;
- only the knowledge, prompts, profiles, and tool descriptors actually required
  by a selected operation.

The current `skills/liknon/` source is classified by A01. It is moved or split
only after the A01 ledger records its authored content and target owners.

## Operation Semantics

The official operational skill may expose these resource-level operations:

| Operation | Bounded responsibility |
| --- | --- |
| `policy` | Validate and explain policy and suggest human-applied values without mutating it |
| `config` | Inspect or propose validation configuration within current authority |
| `run` | Resolve and execute one explicit validation selection and interpret its report |
| `triage` | Diagnose one concrete report or failure without assuming remediation authority |
| `audit` | Compare requested validation evidence with current workspace coverage |

These names identify skill operations, not public CLI subcommands. Product
commands are defined only by their owning B plans.

## Progressive Disclosure

The host retrieves only:

1. workflow and dependency metadata needed for compatibility checks;
2. the selected operation reference;
3. the minimum knowledge and tool descriptors declared for that operation;
4. exact public CLI references needed for an uncovered deterministic capability.

A missing dependency produces a typed stop with its stable identity. The host
may recommend a separate governed Manager operation but cannot perform one
silently.

## Exit Criterion

Routing, operational instruction, policy guidance, and shared knowledge have
distinct resources and can evolve independently without changing CLI control
flow.
