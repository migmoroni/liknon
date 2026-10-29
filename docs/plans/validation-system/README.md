# Liknon Validation-System Roadmap

## Status

`Approved target`.

This file is the concise end-to-end index for the active plan set. It defines
plan ownership, status, dependencies, and decision gates. Detailed product,
workflow, policy, resource, distribution, and evaluation contracts belong to
their owning folders.

The roadmap describes a target under development. Only the repository code,
schemas, tests, and the current-state record prove implemented behavior.

## Foundation

Read these documents in order before using an implementation plan:

1. [Current-State Assessment](avaliacao-estado-atual.md) - `Current-state record`
2. [Target-State Summary](resumo-estado-alvo.md) - `Approved target` with
   explicit open decisions
3. [Shared CLI Distribution Foundation](cli-distribution-foundation.md) -
   `Approved cross-product contract` for B01-B04 release mechanisms
4. [Plan Coherence And Reorganization](A00-plan-coherence-and-reorganization/README.md)
   - `Historical` completed documentation meta-plan
5. [Unified Launcher, CLI Suite, And Command Plan Realignment](A00.1-unified-launcher-plan-realignment/README.md)
   - `Historical` completed documentation meta-plan

The current-state assessment answers what exists. The target summary answers
what has been approved and what remains undecided. A subordinate plan may add
detail, but it may not silently contradict either document.

## Plan Tracks

### A: Product And Repository Boundaries

| Plan | Status | Owns |
| --- | --- | --- |
| [A01 Product And Repository Separation](A01-product-and-repository-separation/README.md) | `Implementation complete`; hosted verification pending | Current-to-target inventory, package boundaries, source ownership, repository layout, and structural cutover |

### B: Product Implementation And Distribution

| Plan | Status | Owns |
| --- | --- | --- |
| [B01 Validation CLI Distribution](B01-validation-cli-distribution/README.md) | `Provisional`; blocked by final A01.5 hosted verification | Source-package and native distribution of `liknon-validator` / `likv` |
| [B02 Resource Hub And Manager](B02-resource-hub-and-manager/README.md) | `Decision gate` at B02.0 | Resource protocol, Hub, Manager behavior, and Manager distribution |
| [B03 Tool CLI](B03-tool-cli/README.md) | `Decision gate` at B03.0 | Offline resource consumption, agent-facing retrieval, and Tool distribution |
| [B04 Unified Launcher And CLI Suite](B04-unified-launcher-and-cli-suite/README.md) | `Decision gate` at B04.0 | Package `liknon-cli`, launcher commands `liknon` / `likx`, aggregate Cargo installation, and complete native suites |
| [B05 Programmatic APIs And Language Bindings](B05-programmatic-api-and-language-bindings/README.md) | `Future` | Deliberate programmatic APIs and explicit workspace integrations with demonstrated consumers |
| [B06 Coordinated Public Release](B06-coordinated-public-release/README.md) | `Future`; blocked by release-ready B01-B05 inputs | Frozen first-release set, external authority, coordinated public activation, readback, and closure |

The three capability CLIs use the same categories of distribution channel:
exact GitHub source, crates.io source packages, and project-hosted native
binaries through Cloudflare/site delivery. They retain independent package
identities, release artifacts, readiness gates, credentials, and publication
authority. B04 composes verified component candidates into the promoted
aggregate Cargo package and complete native suites without taking over their
individual publication.

B01-B05 own product-specific candidate creation, publication workflows,
credentials, recovery contracts, and recurring release trains. B06 does not
replace those responsibilities. It coordinates their first public activation
after the selected products are ready, and it excludes any unimplemented B05
ecosystem instead of publishing placeholders.

The [shared CLI distribution foundation](cli-distribution-foundation.md)
defines the reusable build, manifest, archive, installer, signing, verification,
publication, and recovery mechanisms. B01 implements those mechanisms first
with Validation as the first product profile. B02 and B03 instantiate them for
independent products. B04 uses them to build the launcher and then composes
exact verified B01-B03 native candidates without rebuilding capability CLIs.

### C: AI Governance And Official Workflows

| Plan | Status | Owns |
| --- | --- | --- |
| [C01 AI Governance And Workflow Foundation](C01-ai-governance-and-workflow-foundation/README.md) | `Decision gate` | Generic bootstrap, authority, policy, workflow/resource distinctions, persistence envelopes, and cross-product ownership |
| [C02 Official Action Workflow](C02-official-action-workflow/README.md) | `Provisional`, blocked by C01 and B02/B03 contracts | Replaceable official workflow for bounded assisted operations |
| [C03 Official Validation-Engineering Workflow](C03-official-validation-engineering-workflow/README.md) | `Provisional`, blocked by C01 and B02/B03 contracts | Replaceable official workflow for domain, coverage, incident, and drift work |

C01 defines platform-neutral contracts. C02 and C03 define ordinary official
resources that consume those contracts. Process numbers, node names, selected
profiles, and selected skills remain local to their workflow resources and are
not platform constants.

### D: Integrated Conformance And Evaluation

| Plan | Status | Owns |
| --- | --- | --- |
| [D01 System Conformance And Agent Evaluation](D01-system-conformance-and-agent-evaluation/README.md) | `Provisional`, blocked by implemented B/C capabilities | Cross-product scenarios, isolation, abuse review, agent evaluation, and integrated readiness evidence |

D01 does not publish products. Product publication and recovery remain in
B01-B05. B06 coordinates their first public activation without taking over
those authorities.

### E: Public Web And Delivery Surfaces

| Plan | Status | Owns |
| --- | --- | --- |
| [E01 Liknon Public Site And Hub Delivery](E01-public-site-and-hub-delivery/README.md) | `Decision gate` | `liknon.org`, `apps/liknon-site`, the human resource catalog, public domain topology, and verified presentation of Hub and product-release outputs |

E01 does not replace B02 Hub ownership or any B01-B04 release authority. It
keeps the human site independent from the machine Hub while making their public
identities and verified metadata discoverable together.

## Dependency Graph

```text
current-state assessment + approved target summary
                         |
                         v
          A01 product/repository boundaries
             |                         |
             v                         v
 B01 Validation profile +       C01 generic AI/workflow
 shared distribution            governance contracts
 foundation implementation              |
             |                    +------+------+
             |                    v             v
             |               B02.0 Hub/     B03.0 Tool
             |               Manager        contract
             |                    |             |
             +--------------------+-------------+
                                  |
                     B02/B03 product implementation
                     and independent distribution
                                  |
                    stable contracts and candidates
                                  |
                                  v
                    B04.0 launcher/suite contract
                                  |
                                  v
                   B04 launcher and native suites

 C01 + B02/B03 contracts --> C02/C03 official workflows
 B01-B04 + C01-C03 evidence --> D01 integrated conformance

B05 programmatic APIs and language bindings: future, independently activated
                                  |
                                  v
B01-B05 release-ready candidates + selected D01 evidence
          + E01 site/Hub delivery candidate
                                  |
                                  v
                  B06 coordinated first public release
```

Distribution readiness remains product-specific. `D01` may verify an
integrated candidate, but it must not own publication already assigned to
`B01`, `B02`, `B03`, `B04`, or `B05`. `B06` coordinates those owners without
acquiring their destination credentials or recurring release authority.

## Active Decision Gates

### A01.0: Structural Inventory

Before repository restructuring, approve an evidence-based ledger that maps
every implemented source, package surface, schema, test, asset, fixture, and
maintained document to one target owner and one operation. An unimplemented
proposal cannot appear as a removal task.

### C01: Cross-Product Ownership

Before AI-governance implementation, decide:

- which product validates and initializes the human-owned policy;
- which product provides deterministic persistence operations;
- which responsibilities remain with the external AI host;
- how these operations remain generic and independent from official workflow
  identities.

### B02.0: Manager And Hub Contract

Before Manager implementation, approve resource identity, Hub behavior,
manifest/lock ownership, command grammar, machine outputs, trust, recovery, and
publication boundaries. Existing command names in B02 are candidates until
this gate passes.

### B03.0: Tool Contract

Before Tool implementation, approve local discovery, read-only behavior,
command grammar, machine outputs, integrity handling, the bootstrap projection
interface, and the policy/persistence inspection boundary. Existing command
names in B03 are candidates until this gate passes.

### B04.0: Launcher And Suite Contract

Before launcher implementation or aggregate publication, approve executable
resolution, compatibility, process forwarding, aggregate target mapping,
installation collision, native-suite composition, help, version, provenance,
publication, and recovery contracts. The approved eight binary targets and
three explicit namespaces are fixed inputs to this gate.

### B05.0: Programmatic API Activation

Do not activate B05 until at least one real programmatic consumer justifies a
deliberate API. A configuration integration can satisfy this gate only with a
complete contract and representative consumer. CLI syntax and internal Rust
modules are not sufficient API design evidence.

### B06.0: Coordinated Release Set

Before any registry package, Hub object, native object, metadata release, or
stable pointer is published, approve one immutable ledger containing the exact
included products, independently owned versions, signed tags, commits,
candidates, targets, destinations, dependencies, exclusions, retention windows,
and recovery owners. A changed candidate or destination invalidates that
approval. B06 does not publish a placeholder for a B05 ecosystem whose
activation gate has not passed.

## Execution Order

1. A01 has implemented the structural cutover in buildable units; complete its
   final hosted verification.
2. Begin B01 delivery work and C01 contract work from the verified A01 boundary.
   B01 first establishes the shared distribution primitives and its Validation
   profile; this does not publish or version another product.
3. Resolve B02.0 and B03.0; only then implement their provisional phases.
   Their distribution phases instantiate the shared foundation under separate
   identities, tests, credentials, and approval.
4. Resolve B04.0 after A01 boundaries and component command contracts are
   stable; build the launcher through the shared foundation and implement
   native-suite composition only from verified B01-B03 release candidates.
5. Implement C02 and C03 as resources over approved generic contracts.
6. Execute the D01 integrated conformance and agent evaluation required before
   public release.
7. Activate the future B05 programmatic-API phases only through their
   independent consumer-driven gates and prepare candidates for every language
   ecosystem selected for the first public release.
8. Execute B06 only after B01-B05 have handed off the selected exact candidates
   and D01 has produced the designated blocking evidence. B06 rehearses and
   activates the complete set, verifies public readback, promotes stable
   pointers, and revokes bootstrap credentials.

Each plan owns its own detailed handoff and acceptance criteria. Completion of
one plan never implicitly marks another plan complete.

## Canonical Ownership Rules

- Current implementation facts belong to code, schemas, tests, and the
  current-state assessment.
- Approved target direction belongs to the target summary.
- Product behavior and distribution belong to A/B plans.
- Cross-product release mechanisms belong to the shared CLI distribution
  foundation; product releases and authorities remain in B01-B04.
- Programmatic package releases and registry authorities remain in B05.
- The coordinated first public activation, release-set ledger, cross-product
  publication order, and closure evidence belong to B06.
- Generic AI governance belongs to C01.
- Official workflow definitions belong to C02 or C03.
- Integrated conformance and agent evaluation belong to D01.
- Public documentation may summarize these contracts but must not redefine
  them independently.
- Generated documentation must identify its canonical source.
- Archived plans are evidence, never normative dependencies.

## Historical Plans

Completed and superseded plans are retained under
[docs/plans/old](../old/README.md). They explain how the current implementation
was reached but do not override the current-state assessment, target summary,
or active plans.

## Roadmap Completion

This roadmap is coherent when:

- every active capability has one owning plan;
- every open design question is visible as a gate;
- no active plan depends normatively on the historical archive;
- no proposal that was never implemented is described as something to remove;
- generic platform contracts remain separate from official workflow content;
- product release work remains separate from integrated-system evaluation;
- shared release mechanisms are implemented once while every product keeps an
  independent release identity and authority;
- first public activation is coordinated by B06 from exact product-owned
  candidates without creating a permanent coupled release train;
- all active links resolve and the next executable step is unambiguous.
