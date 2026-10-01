# Source Policy

## Question

How should local validation guidance use external knowledge without hiding
uncertainty or depending on live network access?

## Applicability And Boundaries

Apply this policy to every substantive shared guide. It governs traceability
and synthesis, not executable workspace configuration, network authority, or a
consumer's project policy.

## Conceptual Model

Prefer specifications and standards, then maintainer documentation, then
primary research. Record authority, edition or reviewed living snapshot,
publication status, scope, review date, authoritative URI, and only the narrow
locations used. Keep restricted material summarized and linked rather than
copied.

A sourced fact restates a bounded point. Local synthesis combines source facts
and project reasoning. An example demonstrates one declared context. An
inference connects observed workspace evidence to a contextual conclusion.
Label these roles so a reader can inspect their authority and uncertainty.

## Decision Guidance

Place a source-location citation next to the supported claim and do not
generalize beyond its registered `supports` statement. Exact snapshots provide
provenance; they narrow reader applicability only when the guide explains a
material version difference. Use local prose for the essential decision and an
external link for authoritative depth.

When coverage is absent, identify the missing question and stop before
inventing guidance. Review a record when its dependent claim changes, its
edition is superseded, or a persistent link move is confirmed.

## Evidence And Limitations

JSON Schema Draft 2020-12 defines the dialect used to validate local registry
shape, but schema validity cannot establish editorial or technical accuracy.
[Source: json-schema.2020-12#core](https://json-schema.org/draft/2020-12/json-schema-core)

Publisher availability, citation resolution, and a valid source record do not
prove that the adjacent synthesis is complete. Human review must confirm that
the location actually supports the claim.

## Related Knowledge

Use [Validation Strategy](validation-strategy.md) to turn authoritative context
into proportional local evidence. Maintainers update `sources.json`, regenerate
`SOURCES.md`, and run the release check; ordinary consumers remain offline.
