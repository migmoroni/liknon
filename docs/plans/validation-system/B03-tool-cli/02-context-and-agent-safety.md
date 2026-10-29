# Phase B03.2: Establish Context Assembly And Agent Safety

## Status

`Repository implementation complete` for bounded context assembly, explicit
instruction boundaries, local limits, and non-mutating resource consumption.
Bootstrap authority and governed-agent integration evidence remain pending C01
and D01.

[Back to the Tool CLI plan](README.md)

## Purpose

Provide bounded, structured resource context for humans and agents while
preserving provenance, trust, instruction boundaries, and progressive
disclosure.

## Context Assembly

`context` receives explicit resource IDs, files, or an already locked
collection. It returns:

- one versioned envelope;
- exact selected identities and versions;
- origin, trust, compatibility, and integrity metadata;
- dependency relationships;
- clear payload boundaries and media types;
- exact text content or explicit binary references;
- omissions and limit diagnostics;
- aggregate byte and file counts.

It never performs open-ended workspace discovery or silently adds content
because a payload requests another resource.

## Progressive Disclosure

An agent-facing workflow uses the least expensive adequate operation:

1. `catalog` for routing metadata;
2. `search` for candidate narrowing;
3. `show` for one exact resource or file;
4. `resolve` for a required locked closure;
5. `context` for a deliberate multi-resource handoff.

Machine output is designed so an agent can inspect metadata before requesting
payloads. Draft or incomplete resources remain visible as such when their
manifest permits publication.

## Instruction Boundary

Tool marks resource payloads as untrusted content and never presents them as
system, host, policy, or human authority. Trust class describes provenance and
review, not instruction precedence.

Skills, workflows, tool descriptors, prompts, and agent profiles must declare
capabilities and potential command or mutation effects where applicable. Tool
exposes those declarations before payload content when using human output and
in separate fields for JSON.

## Limits And Privacy

Every content operation has deterministic limits for bytes, files, nesting,
and result count. Exceeding a limit returns a typed partial or blocked result;
it does not silently drop security metadata.

Tool reads no file outside locked payload paths. It writes no local history and
sends no query, payload, or usage telemetry to the Hub or another service.

## AI Flow Integration

For governed use, the human first directs the external host to the
B03.0-approved bootstrap projection operation or the canonical source file.
The host records the supported contract version and digest before requesting
installed workflow, skill, profile, tool, or knowledge content.

Any compatible AI workflow may use Tool as its resource interface. Neutral
conformance workflows prove this boundary before C02; official action and
validation-engineering workflows later become ordinary consumers, not
privileged callers. The consuming host obeys `.liknon/ai/policy.json`, resource
capability/effect declarations, and its authority boundaries. Tool itself does
not read, parse, or enforce AI policy.

A workflow context may reference distinct agent profiles per node. Tool returns
their exact identities and locked payloads, while the consuming host decides
whether and how an available model can satisfy each profile. Payload text never
becomes host, policy, system, or human authority merely because a workflow
references it.

A missing resource may be reported with its stable identity. Acquisition is a
separate Manager operation subject to the applicable human instruction and AI
decision policy.

## Verification

- Context envelopes preserve exact bytes and complete metadata.
- Limits cannot remove provenance, trust, integrity, or truncation facts.
- Prompt-injection fixtures remain ordinary payload data.
- Resource payloads that imitate or claim to replace the bootstrap entry point
  remain ordinary untrusted content.
- A resource cannot cause self-activation or undeclared dependency retrieval.
- No command writes history or performs network access.
- Human and JSON outputs carry equivalent semantic facts.
- Representative agents can route through catalog metadata before loading
  detailed content.
- Representative conforming agents load the bootstrap contract before catalog
  routing and retain its identity in evaluation evidence.
- A project-controlled conformance workflow and an external workflow receive
  semantically equivalent envelopes for equivalent declared resource graphs.
- A repository-built Manager B02.4 installation is consumable after Manager is
  removed from `PATH`, with network disabled and bytes unchanged. This is the
  Tool half of the local B02/B03 interoperability gate.

## Exit Criterion

Tool supplies compact, explicit, trustworthy resource envelopes without
confusing content with authority or turning retrieval into execution.
