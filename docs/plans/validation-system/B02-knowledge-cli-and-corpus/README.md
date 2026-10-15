# Plan B02: Liknon Knowledge CLI And Corpus

## Status And Relationship To Other Tracks

This plan distributes the complete validation-knowledge corpus through the
independent `liknon-knowledge` application created by
[A01](../A01-product-and-repository-separation/README.md).

[B01](../B01-cli-distribution/README.md) distributes validation execution.
B02 distributes read-only knowledge. Neither application depends on, invokes,
installs, or updates the other. [B03](../B03-ecosystem-ai-packages/README.md)
distributes ecosystem AI resources, while
[B04](../B04-programmatic-api-and-language-bindings/README.md) governs future
in-process APIs.

## Objective

Distribute one complete knowledge product through three complementary channels:

| Channel | Distributed form | User experience | Rust required |
| --- | --- | --- | --- |
| GitHub source | Signed tagged source for the canonical monorepo | Clone, select a version, and compile `apps/liknon-knowledge` | Yes |
| crates.io | Source application package `liknon-knowledge` | `cargo install liknon-knowledge --locked` | Yes |
| Cloudflare Direct | Versioned native archive through the project domain | Run an authenticated installer or verify and extract an archive | No |

The source repository remains the canonical authored location. crates.io and
Cloudflare distribute release forms of the same approved application and exact
corpus bytes.

## Product Boundary

The package installs two command names backed by one implementation:

- `liknon-knowledge`, the canonical command;
- `liknon-k`, the compact command.

The initial command contract is:

```text
liknon-knowledge catalog [--format=human|json]
liknon-knowledge show <DOCUMENT_ID>
```

The equivalent `liknon-k` commands produce the same results.

The application includes:

- the complete approved catalog and canonical Markdown documents;
- stable document IDs and deterministic discovery;
- human-readable and machine-readable catalog output;
- exact Markdown retrieval by ID;
- version and source provenance;
- knowledge-specific help, diagnostics, color, and presentation where
  applicable.

It does not include:

- configuration planning or validation execution;
- `.validation/` initialization or mutation;
- `liknon-core` merely to reuse unrelated CLI infrastructure;
- skills, prompts, AI-Tool operations, or AI-Engineering processes;
- ecosystem package discovery or workspace activation state;
- network retrieval during normal knowledge use;
- a `liknon knowledge` compatibility route.

## Corpus Contract

The corpus is one product with an internal taxonomy. Language, tool,
technology, concern, pattern, reference, and recipe facets guide selection but
do not create optional runtime packs.

Every release binds:

- one application version;
- one source commit and signed tag;
- one deterministic catalog;
- the exact digest of every canonical document;
- the editorial profile and source-register versions used to approve it.

`catalog` and `show` are read-only, work without a workspace configuration, and
perform no network access. `show` writes the exact canonical Markdown bytes to
standard output.

## Release Identity

The knowledge application has an independent release train:

- Cargo package: `liknon-knowledge`;
- signed tag: `liknon-knowledge-v<VERSION>`;
- GitHub Release identity: the same tag;
- Cloudflare prefix:
  `releases/liknon-knowledge/v<VERSION>/...`;
- stable channel:
  `channels/liknon-knowledge/stable.json`.

A validation-CLI release and a knowledge release may use different versions.
Compatibility is documented only where a skill or workflow requires a minimum
capability; version equality is never assumed.

## Distribution Invariants

1. Source, crates.io, and Cloudflare candidates come from one tagged commit.
2. The crates.io package and native archives contain the same approved corpus
   bytes and command behavior.
3. Both command aliases invoke one implementation and report one version.
4. Cloudflare archives retain separate target identities, including GNU and
   MUSL Linux variants.
5. Native signing, detached metadata, immutable object, credential isolation,
   and recovery rules follow the security discipline established by B01 while
   retaining independent artifact identities.
6. Registry publication and Cloudflare publication consume already verified
   candidates; neither rebuilds them.
7. Generated artifacts remain ignored and no release step edits authored
   knowledge.
8. A content-only corpus change still requires a new knowledge application
   version because it changes distributed bytes.

## Proposed Repository Layout

```text
apps/
└── liknon-knowledge/

docs/validation/
├── knowledge/
└── authoring/

distribution/cli/
└── knowledge/
    ├── direct/
    │   ├── install.sh
    │   ├── install.ps1
    │   └── public-key.txt
    └── crates/

target/distribution/knowledge/       # Generated and ignored
```

Authoring guidance and release tooling are not part of the runtime corpus
unless the catalog explicitly identifies a public knowledge document.

## Implementation Order

| Phase | Document | Depends on | Primary result |
| ---: | --- | --- | --- |
| B02.0 | [Product And Command Contract](00-product-and-command-contract.md) | A01.4 | Approved identity, aliases, commands, corpus boundary, and channels |
| B02.1 | [Corpus And Retrieval](01-corpus-and-retrieval.md) | B02.0 | Deterministic complete corpus and exact read-only retrieval |
| B02.2 | [Version And Provenance](02-version-and-provenance.md) | B02.1 | Independent version authority and artifact evidence |
| B02.3 | [Native And Source Distribution](03-native-and-source-distribution.md) | B02.2 | GitHub, crates.io, and Cloudflare release candidates |
| B02.4 | [End-To-End Verification](04-end-to-end-verification.md) | B02.3 | Cross-channel command and corpus equivalence |
| B02.5 | [Publication And Maintenance](05-publication-and-maintenance.md) | B02.4 | Isolated publication, readback, recovery, and corpus lifecycle |

## Global Acceptance Criteria

- [ ] `liknon-knowledge` and `liknon-k` are equivalent entry points.
- [ ] `liknon knowledge` does not exist.
- [ ] The complete catalog is available without configuration, network access,
      or validation CLI installation.
- [ ] Every catalog ID resolves to exactly one canonical document.
- [ ] `show` returns byte-identical Markdown across all channels.
- [ ] The application contains no validation runner or agent-resource bundle.
- [ ] GitHub source installation targets `apps/liknon-knowledge`.
- [ ] crates.io installs both command names from `liknon-knowledge`.
- [ ] Cloudflare installs exactly one target archive and verifies metadata,
      archive, and executable hashes.
- [ ] Public readback matches retained candidates.
- [ ] Knowledge updates can release without rebuilding or republishing
      `liknon-cli`.

## Resulting System

Users and agents can install a focused, complete knowledge application without
the validation executor. Validation execution and knowledge discovery remain
composable at the workflow level but independent at the process, package,
release, and update levels.
