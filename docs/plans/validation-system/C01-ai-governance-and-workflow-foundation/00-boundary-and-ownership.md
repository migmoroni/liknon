# Phase C01.0: Approve AI Governance Ownership

## Status

`Decision gate`.

[Back to Plan C01](README.md)

## Purpose

Assign deterministic operations to products without changing the approved
human, host, validation, Manager, or Tool boundaries.

## Fixed Responsibility Baseline

| Actor or product | Fixed responsibility |
| --- | --- |
| Human | Selects workflows, owns policy, supplies decisions, and may operate any CLI directly |
| External AI host | Selects models, binds profiles, conducts graphs, classifies proposed effects, and presents decisions |
| Validator (`liknon-validator`) | Executes declarative validation from validation configuration and emits structured evidence |
| Manager (`liknon-manager`) | Contacts the Hub and mutates installed resource state through approved operations |
| Tool (`liknon-tool`) | Reads and returns local resources offline and projects the exact program-owned bootstrap through approved operations |
| Unified launcher (`liknon`) | Forwards one explicitly selected capability invocation to a sibling executable; owns no policy, persistence, resource, validation, or AI operation |
| Hub | Publishes metadata and immutable resource payloads; it grants no instruction authority |
| C01 contracts | Define authority, compatibility, policy, and artifact semantics without hosting cognition |

## Operations Requiring An Owner

Record one owner and one machine contract for each operation:

| Operation | Constraints |
| --- | --- |
| Bootstrap projection | Tool-owned, read-only, exact bytes, no workspace or Hub required; C01 selects the operation and envelope for focused B03 implementation |
| Policy initialization | Human-operated only, conservative content, no overwrite |
| Policy validation | Read-only and deterministic |
| Policy explanation | Advisory and read-only |
| Policy evaluation | Read-only; receives explicit workflow stack and concrete classification inputs |
| Artifact validation | Read-only validation against locked workflow declaration and generic envelope |
| Artifact persistence | Exact declared destination, append-only creation, policy-gated |
| Validation-report persistence | Exact report bytes, report contract, policy-gated when requested by AI |
| Artifact/report removal | Explicit governed mutation; never automatic retention cleanup |

Except for the approved Tool-owned bootstrap projection, the owner may be the
existing Validator, Manager, Tool, a supporting library invoked behind one of
those capability applications, or an external-host responsibility only when
that choice preserves existing product boundaries. A supporting library is not
another application, executable, command surface, or release identity.

The B04 suite contains exactly `liknon`, `liknon-validator`, `liknon-manager`,
and `liknon-tool`. The unified launcher is never an operation owner, and C01
must not introduce a fifth executable. Do not select an owner merely because
an earlier plan used a particular command prefix.

## Evaluation Criteria

For each candidate owner, evaluate:

- conceptual fit with its approved product role;
- whether network or mutation capability would be widened;
- whether direct human use remains complete;
- whether an AI can use the operation through the same public interface;
- whether policy evaluation creates circular authorization;
- whether the operation can be tested without an AI model;
- whether another product would become a hidden runtime dependency;
- whether machine contracts can evolve explicitly while every shipped
  executable remains bound to the exact suite version;
- whether direct and launcher-routed invocation remain semantically equivalent;
- whether sensitive inputs cross a new trust boundary.

## Required Decision Record

The exit record must state:

- selected owner and rationale for every operation;
- command family only where the owning B plan has approved it;
- machine input, output, exit, and side-effect class;
- policy applicability and the narrow pre-policy exception, if any;
- direct and launcher-routed invocation paths plus the required suite identity
  and component-coherence evidence;
- required shared contracts and package owner;
- direct-human and AI-host invocation paths;
- rejected alternatives and the boundary each would violate.

## Exit Criterion

Every deterministic policy and persistence operation has one approved owner,
while the human still owns policy and the external host still owns cognition.
C01.1-C01.5 can then replace provisional operation names with exact contracts,
and the focused B03 follow-up can implement the selected bootstrap projection
without changing the B04 executable topology.
