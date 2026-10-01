# Source Policy

## Question

How should local validation guidance use external knowledge without hiding
uncertainty or becoming dependent on live network access?

## Applicability

Apply this policy to every substantive guide in this knowledge tree. It does
not govern executable workspace configuration or make an external publication
project policy.

## Source Selection

Prefer, in order, specifications and standards bodies, maintainer
documentation, and primary research. Record the exact edition or living
revision, publication status, scope, review date, and narrow locations actually
used. Restricted standards may be summarized and linked but must not be copied.
Drafts remain labeled drafts and do not silently replace final editions.

Use citation labels that combine the `Source:` prefix, source ID, `#`, and the
location ID, linked to the registered deep URI. A cited claim should use the
narrowest registered location. The source register records
what the location supports; it is not permission to generalize beyond that
scope. JSON Schema Draft 2020-12 supplies the vocabulary used for the two local
registry schemas, but schema validity does not establish editorial accuracy.
[Source: json-schema.2020-12#core](https://json-schema.org/draft/2020-12/json-schema-core)

## Sourced Fact, Synthesis, Example, And Inference

- **Sourced fact** restates a bounded point supported by a registered location.
- **Local synthesis** combines sources and project reasoning and is labeled as
  such; it is not attributed verbatim to a source.
- **Example** demonstrates one valid composition and carries no universal
  requirement.
- **Inference** is a contextual conclusion from observed workspace evidence and
  must expose the observations and uncertainty that led to it.

## Offline Use And Gaps

Each guide must state relevance, evidence, limitations, and a safe next step in
its own words. Links offer depth, not implicit network authority. If local
coverage is missing or requested depth is inaccessible, report that limitation,
identify the exact missing question, and stop before inventing an answer.

## Maintenance

Review a source when guidance depending on it changes, a cited edition is
superseded, or link-health monitoring reports a persistent move. Update
`sources.json`, regenerate `SOURCES.md` and the skill projection, then run the
knowledge verification suite. A transient unavailable publisher does not alter
the deterministic local package.
