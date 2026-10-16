# Plan B03: Liknon Tool CLI

## Status And Relationship To Other Tracks

This plan defines the independent local resource-consumption application created
by [Plan A01](../A01-product-and-repository-separation/README.md).

[B02](../B02-resource-hub-and-manager/README.md) defines the Resource Hub,
resource protocol, and canonical Manager. Tool consumes valid local resource
state but does not depend on the Manager executable. [B01](../B01-cli-distribution/README.md)
remains the independent validation executor.

## Objective

Provide humans, agents, editors, and automation with a fast, safe, offline
interface for discovering and consuming installed Liknon resources.

Tool operates over knowledge, skills, workflows, agent profiles, prompts,
guides, templates, and collections. It does not privilege knowledge as a
special embedded corpus.

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
| `catalog` | List installed consumable resources and routing metadata |
| `search` | Deterministically search local metadata and declared searchable text |
| `show` | Return one resource, file, or canonical document by stable identity |
| `resolve` | Return the locked local dependency or collection closure |
| `context` | Assemble an explicit bounded set of resources for human or agent consumption |

Tool operations describe and return content. Manager `list` and `info`
remain administrative views of desired, locked, installed, update, and
integrity state.

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
corpus.

## Implementation Order

| Phase | Document | Depends on | Primary result |
| ---: | --- | --- | --- |
| B03.0 | [Product And Command Contract](00-product-and-command-contract.md) | A01.4 and B02.0 | Approved identity, commands, outputs, and non-mutation boundary |
| B03.1 | [Local Catalog And Retrieval](01-local-catalog-and-retrieval.md) | B03.0 | Deterministic offline discovery and exact payload retrieval |
| B03.2 | [Context And Agent Safety](02-context-and-agent-safety.md) | B03.1 | Bounded progressive disclosure and safe machine consumption |
| B03.3 | [Source And Native Distribution](03-source-and-native-distribution.md) | B03.2 | Independently installable Tool candidates |
| B03.4 | [End-To-End Verification And Publication](04-end-to-end-verification-and-publication.md) | B02.5 and B03.3 | Public Tool distribution and Manager-to-Tool interoperability |

## Global Acceptance Criteria

- [ ] `liknon-tool` and `likt` are equivalent entry points.
- [ ] Tool works without network access and without Manager or Validation CLI
      installed.
- [ ] Tool consumes both Manager-produced and contract-valid fixture stores.
- [ ] No Tool artifact embeds authored Hub resource payloads.
- [ ] The crates.io package resolves only published supporting crates.
- [ ] Catalog and search do not load unrelated payload content.
- [ ] Exact retrieval rejects modified, missing, unsafe, incompatible, and
      unsupported locked state.
- [ ] Human and JSON output expose identity, version, origin, trust, integrity,
      and revocation state.
- [ ] Tool never executes or activates returned instructions.
- [ ] A selected knowledge item, skill, collection, and dependency closure are
      returned deterministically.
- [ ] Resource-only updates require no Tool release.

## Resulting System

Tool is the stable local bridge between installed Liknon resources and the
humans or agents that deliberately consume them. It keeps resource access fast
and structured while leaving acquisition to Manager and validation execution
to `liknon` or `likx`.
