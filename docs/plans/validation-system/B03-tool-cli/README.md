# Plan B03: Liknon Tool CLI

## Status And Relationship To Other Tracks

The [validation-system index](../README.md) owns cross-track order, milestone
gates, and contract ownership. This plan owns only the Tool track.

This plan defines the independent local resource-consumption application created
by [Plan A01](../A01-product-and-repository-separation/README.md).

[B02](../B02-resource-hub-and-manager/README.md) defines the Resource Hub,
resource protocol, and canonical Manager. Tool consumes valid local resource
state but does not depend on the Manager executable. [B01](../B01-cli-distribution/README.md)
remains the independent validation executor.

## Objective

Provide humans, agents, editors, and automation with a fast, safe, offline
interface for discovering and consuming installed Liknon resources.

Tool operates over knowledge, skills, workflows, agent profiles, tool
descriptors, prompts, guides, templates, and collections. It does not privilege knowledge as a
special embedded corpus.

Tool also exposes the program-owned AI bootstrap entry point. That built-in
contract is not an installed resource or corpus: it is the stable first
instruction a human supplies to an external AI before governed resource use.

## Product Boundary

The Cargo application package is `liknon-tool`. It installs:

- `liknon-tool`, the explicit command;
- `likt`, the compact command.

Tool reads the nearest valid `.liknon/resources/` store or an explicit
workspace. It performs no Hub request, download, installation, update, removal,
materialization, validation execution, AI-policy mutation, or resource-provided
command execution.

## Command Families

The final grammar is approved in B03.0 and must cover:

| Operation | Responsibility |
| --- | --- |
| `bootstrap` | Return the exact program-owned AI entry contract, version, and digest without requiring a workspace |
| `catalog` | List installed consumable resources and routing metadata |
| `search` | Deterministically search local metadata and declared searchable text |
| `show` | Return one resource, file, or canonical document by stable identity |
| `resolve` | Return the locked local dependency or collection closure |
| `context` | Assemble an explicit bounded set of resources for human or agent consumption |

Tool operations describe and return content. Manager `list` and `info`
remain administrative views of desired, locked, installed, update, and
integrity state.

`bootstrap` is the sole built-in content route. It verifies
`crates/liknon-workspace/contracts/ai/ENTRYPOINT.md` against its adjacent
`manifest.json`, returns the entry bytes exactly with their canonical metadata, and never consults or
classifies either file as part of `.liknon/resources/`.

For workflows, Tool returns the exact declared graph and resolved resource
closure. It does not interpret transitions, instantiate an agent profile,
select a model, execute a node, or apply AI decision policy.

## Local Consumption Contract

Before returning payload content, Tool validates:

- manifest and lock schema versions;
- resource identity and kind;
- locked path containment;
- payload inventory, lengths, and digests;
- compatibility metadata;
- yanked, revoked, and trust state available in the lock;
- requested file and output-size boundaries.

Human output clearly separates metadata from payload content. Versioned JSON
returns typed metadata and exact content without ANSI.

## Progressive Disclosure

Tool supports low-cost discovery before content retrieval:

1. catalog summaries expose stable IDs, kinds, versions, summaries, trust, and
   routing metadata;
2. search narrows candidates without loading unrelated payloads;
3. show returns only the requested resource or file;
4. resolve returns an explicit dependency closure;
5. context assembles only caller-selected resources under declared size limits.

This contract lets an agent obtain the minimum relevant context without reading
every installed resource.

## Safety Boundary

Tool authenticates local bytes against the lock but cannot declare an
instruction semantically safe. Resource text remains untrusted data until the
human or agent deliberately chooses to use it.

Tool never:

- executes a command found in content;
- treats payload text as higher-priority instructions;
- hides origin, trust, revocation, or integrity state;
- selects a resource merely because it requests its own activation;
- writes content into an agent directory;
- contacts a remote model or service.

## Distribution Contract

Tool has its own version and release train and is available through exact
tagged GitHub source, the `liknon-tool` crates.io source package, and
authenticated target-native archives through the project download domain.

Its crates.io application package resolves only exact published supporting
crates. Required supporting versions are published and verified first when they
are not already available.

The website offers Tool separately and may include it in a convenience bundle
with Validation CLI and Manager. Tool artifacts contain no complete resource
corpus; inclusion of the single program-owned bootstrap contract does not turn
that contract into a Hub payload.

## Implementation Order

| Phase | Document | Depends on | Primary result |
| ---: | --- | --- | --- |
| B03.0 | [Product And Command Contract](00-product-and-command-contract.md) | A01.4 and B02.0 | Approved identity, commands, outputs, and non-mutation boundary |
| B03.1 | [Local Catalog And Retrieval](01-local-catalog-and-retrieval.md) | B03.0 | Deterministic offline discovery and exact payload retrieval |
| B03.2 | [Context And Agent Safety](02-context-and-agent-safety.md) | B03.1 | Bounded progressive disclosure and safe machine consumption |
| B03.3 | [Source And Native Distribution](03-source-and-native-distribution.md) | B03.2 | Independently installable Tool candidates |
| B03.4 | [End-To-End Verification And Publication](04-end-to-end-verification-and-publication.md) | B03.3 and the root M3 local interoperability gate | Public Tool distribution and clean-environment local-store interoperability |

B03.0-B03.2 establish local implementation readiness. The root plan's M3 gate
then combines repository-built B02.4 Manager and B03.2 Tool with neutral
fixtures. B03.3-B03.4 are distribution work and are not prerequisites for Phase
3. When B02.5 is also complete, B03.4 adds public-Manager-to-public-Tool
interoperability evidence; that cross-channel claim does not gate independent
Tool publication.

## Global Acceptance Criteria

- [ ] `liknon-tool` and `likt` are equivalent entry points.
- [ ] Tool works without network access and without Manager or Validation CLI
      installed.
- [ ] Tool consumes both Manager-produced and contract-valid fixture stores.
- [ ] `bootstrap` works without a workspace or resource store and returns the
      canonical contract bytes, version, and SHA-256 digest.
- [ ] No Tool artifact embeds authored Hub resource payloads.
- [ ] The crates.io package resolves only published supporting crates.
- [ ] Catalog and search do not load unrelated payload content.
- [ ] Exact retrieval rejects modified, missing, unsafe, incompatible, and
      unsupported locked state.
- [ ] Human and JSON output expose identity, version, origin, trust, integrity,
      and revocation state.
- [ ] Tool never executes or activates returned instructions.
- [ ] Tool presents the bootstrap contract as program authority below superior
      system, host, repository, and human instructions, while every installed
      resource remains untrusted content.
- [ ] A selected knowledge item, skill, collection, and dependency closure are
      returned deterministically.
- [ ] Project-controlled conformance and external workflow graphs, profiles,
      and dependency closures are returned through identical commands and
      schemas; later official workflows require no Tool branch.
- [ ] Tool contains no routing branch for an official workflow, skill, tool,
      node, action, transition, or agent-profile ID.
- [ ] Resource-only updates require no Tool release.
- [ ] Bootstrap-contract changes require an explicit compatible Tool release;
      ordinary resource changes do not.

## Resulting System

Tool is the stable local bridge between installed Liknon resources and the
humans or agents that deliberately consume them. It keeps resource access fast
and structured while leaving acquisition to Manager and validation execution
to `liknon` or `likx`.
