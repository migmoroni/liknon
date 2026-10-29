# Liknon Resource Ecosystem

Liknon resources are versioned, authenticated data selected by a consumer and
installed below `.liknon/resources/`. They can carry knowledge, skills,
workflows, agent profiles, tool descriptions, prompts, guides, templates, or
collections. Installing a resource never executes it, activates a workflow, or
changes AI policy.

The ecosystem has three distinct boundaries:

- the [resource protocol](protocol.md) defines identities, metadata, state,
  paths, compatibility, and exact-byte integrity;
- the [static Hub](hub-publication.md) publishes immutable objects and signed
  catalog snapshots;
- `liknon-manager` applies an authenticated selection to a consumer workspace
  under the [Manager operating contract](manager.md).

Publisher identity and moderation are described in
[trust and governance](trust-and-governance.md). Public JSON Schemas are
generated from `liknon-resource-contracts` and checked in under
`crates/liknon-resource-contracts/schemas/`.

No production Hub origin is advertised until its protected publication and
public readback gates have passed.
