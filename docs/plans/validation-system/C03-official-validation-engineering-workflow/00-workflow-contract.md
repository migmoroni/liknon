# Phase C03.0: Define The Official Validation-Engineering Workflow Contract

## Status

`Provisional`. Exact manifests and operations depend on C01.0, C01.3, B02.0,
and B03.0.

[Back to Plan C03](README.md)

## Resource Identity And Graph

The official workflow is an ordinary versioned workflow resource. Its manifest
declares graph nodes, edges, continuation eligibility, profiles, skills, tool
descriptors, knowledge dependencies, capabilities, effects, artifact schemas,
and application compatibility.

Its initial local graph is:

```text
Process 0: Domain ----eligible----> Process 1: Sensorium

Process 2: Incident --eligible----> Process 3: Drift
```

The two sequences do not imply automatic startup. A human selects an entry
node. `processContinuation` applies only after an eligible source node has
completed, its artifact has been persisted, and the target preconditions pass.

## Host And Application Boundaries

- The external host selects models, applies node profiles, and conducts edges.
- Tool returns exact installed resources without executing them.
- Manager mutates resource state only through a separate governed operation.
- The Validation CLI executes validation and emits canonical reports.
- C01-assigned deterministic operations validate policy and persist artifacts.
- No application contains this workflow's IDs in control flow.

## Evidence Model

Each completed node creates a new immutable UUIDv7 artifact under the generic
C01 envelope. Inputs are explicit paths and digests; no operation selects a
latest file, reconstructs authority from chat history, or updates a mutable
state registry.

The workflow manifest, rather than generic application code, declares concrete
artifact kinds, payload schemas, destinations, relationships, limits, and
repository treatment.

## Exit Criterion

The graph is fully described by locked resources and can be replaced by a
workflow with unrelated IDs, profiles, artifacts, and topology without changing
application code.
