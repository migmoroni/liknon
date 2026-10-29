# Phase C02.0: Define The Official Action Workflow Contract

## Status

`Provisional`. The implemented B02/B03 resource interfaces and B04 suite
topology are fixed inputs. The resource contract waits for the generic C01
contracts and focused Tool bootstrap follow-up.

[Back to Plan C02](README.md)

## Purpose

Define an official workflow that helps an external AI host complete one bounded
Liknon-related task selected by a human. The workflow is point-in-time
assistance, not a persistent engineering process.

## Workflow Shape

The resource must declare:

- one stable workflow identity and independently versioned manifest;
- its compatible bootstrap, policy, Manager, Tool, and Validation contracts;
- one bounded entry node that routes the current request;
- exact resource dependencies and capability/effect declarations;
- supported operation references and their input/output boundaries;
- no continuation-eligible edge;
- no analytical artifact declarations;
- no implicit dependency acquisition or workflow activation.

Application references identify logical Validator, Manager, and Tool
capabilities. Runtime evidence binds each reference to an exact sibling
executable in one B04 suite; a launcher namespace is only an alternate route to
that same process.

The exact resource ID and manifest grammar are selected by B02.0 and C01.3.
They must not be embedded as special cases in any CLI.

## Invocation

```text
human selects a bounded task
  -> external host loads the C01 bootstrap contract
  -> host resolves the exact locked workflow and dependencies through Tool
  -> host validates compatibility and the human-owned policy
  -> workflow routes to the minimum required operation and context
  -> host classifies each proposed effect under policy
  -> approved product interfaces perform deterministic work
  -> host returns one bounded result and ends the invocation
```

Each later request starts from current workspace evidence, a fresh policy
evaluation, and the new human instruction. Prior conversation does not carry
authority or unresolved work implicitly.

## Non-Responsibilities

The workflow must not:

- initialize, edit, replace, move, or remove policy;
- install or update a missing resource;
- create validation-engineering domain, coverage, incident, or drift artifacts;
- write to the workflow persistence area;
- define validation checks or report semantics;
- select a model or provide an agent runtime;
- infer permission from official origin, installation, or successful lookup.

## Exit Criterion

The workflow graph is independently versioned, replaceable, finite, and free of
application-specific privileges or persistent-process semantics.
