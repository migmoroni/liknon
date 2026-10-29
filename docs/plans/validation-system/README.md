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
   `Approved suite contract` for the B04 source and native release
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
| [A02 Shared CLI Experience Foundation](A02-shared-cli-experience-foundation/README.md) | Repository implementation complete; hosted verification pending | Publishable `liknon-cli-ui` crate, accessible terminal policy, generic help presentation, existing-CLI adoption, and B03 runtime-UI handoff |

A01 established independent application boundaries and deliberately retained
Validation presentation until a second implemented consumer demonstrated a
shared boundary. A02 now extracts that demonstrated boundary from the current
Validator and Manager surfaces. It is a runtime experience foundation, not a
capability application and not a replacement for the shared release
foundation.

### B: Product Implementation And Distribution

| Plan | Status | Owns |
| --- | --- | --- |
| [B01 Validation CLI Distribution](B01-validation-cli-distribution/README.md) | Validator behavior and reusable release foundation implemented; standalone publication superseded by B04 | Validator behavior, contracts, and focused release evidence consumed by the suite |
| [B02 Resource Hub And Manager](B02-resource-hub-and-manager/README.md) | Repository implementation in place; Hub authority and suite integration pending | Resource protocol, Hub publication, Manager behavior, and focused release evidence consumed by the suite |
| [B03 Tool CLI](B03-tool-cli/README.md) | Store-backed repository implementation in place; C01 bootstrap and suite integration pending | Offline resource consumption, agent-facing retrieval, and focused release evidence consumed by the suite |
| [B04 Unified Launcher And CLI Suite](B04-unified-launcher-and-cli-suite/README.md) | `Decision gate` at B04.0 | Package `liknon-cli`, executables `liknon`, `liknon-validator`, `liknon-manager`, and `liknon-tool`, plus one native suite archive per target |
| [B05 Programmatic APIs And Language Bindings](B05-programmatic-api-and-language-bindings/README.md) | `Future` | Deliberate programmatic APIs and explicit workspace integrations with demonstrated consumers |
| [B06 Coordinated Public Release](B06-coordinated-public-release/README.md) | `Future`; blocked by B04 suite, B02 Hub, E01 site, D01 evidence, and selected B05 inputs | Frozen first-release set, external authority, coordinated public activation, readback, and closure |

The three capability CLIs remain independent executable processes and behavior
boundaries, but they share one public CLI distribution. B04 owns the
`liknon-cli` source package, the four physical executable targets, and one
complete native suite archive per supported target. B01-B03 provide focused
behavior, security, and conformance evidence; they do not retain standalone
public CLI packages or native download channels after the B04 cutover.

B02 continues to own Hub publication because resources are external data.
B05 owns any activated programmatic packages. B06 coordinates the first public
activation of the B04 suite, Hub, site, and selected B05 outputs without
rebuilding their candidates.

The [shared CLI distribution foundation](cli-distribution-foundation.md)
defines the reusable build, manifest, archive, installer, signing, verification,
publication, and recovery mechanisms. B01 supplied the first implementation of
many of those mechanisms. B04 now adapts them to one suite candidate and builds
the four executables once per target; it does not compose separately published
capability products.

[A02](A02-shared-cli-experience-foundation/README.md) separately defines the
runtime terminal experience shared by those applications: palettes,
presentation, help styling, `NO_COLOR`, low-vision behavior, and neutral
semantic style primitives. B01-B03 retain product command grammars, domain
renderers, machine schemas, errors, and exit codes; B04 owns the shared suite
version and CLI publication authority.

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

D01 does not publish products. CLI publication and recovery remain in B04, Hub
publication remains in B02, and activated programmatic packages remain in B05.
B06 coordinates their first public activation without taking over those
authorities.

### E: Public Web And Delivery Surfaces

| Plan | Status | Owns |
| --- | --- | --- |
| [E01 Liknon Public Site And Hub Delivery](E01-public-site-and-hub-delivery/README.md) | `Decision gate` | `liknon.org`, `apps/liknon-site`, the human resource catalog, public domain topology, and verified presentation of Hub and product-release outputs |

E01 does not replace B02 Hub ownership or any B01-B04 release authority. It
keeps the human site independent from the machine Hub while making their public
identities and verified metadata discoverable together.

### F: Toolchain Modernization

| Plan | Status | Owns |
| --- | --- | --- |
| [F01 Rust 1.90 MSRV Modernization](F01-rust-1.90-msrv-modernization/README.md) | `Future`; begins only after the current roadmap and B06 closure | One reviewed Rust 1.87-to-1.90 compatibility transition, opportunity ledger, standard-library adoption, toolchain/release cutover, and post-transition evidence |

F01 is deliberately outside the current implementation and first-release
sequence. It does not change today's Rust 1.87 contract. It provides one place
to accumulate concrete 1.88-1.90 opportunities discovered during the active
plans without applying them prematurely or turning the MSRV into a moving
target.

## Dependency Graph

```text
current-state assessment + approved target summary
                         |
                         v
          A01 product/repository boundaries
             |                         |
             v                         v
 B01 Validator behavior +        C01 generic AI/workflow
 release-foundation evidence             governance contracts
             |                           |
             |                           v
             +------------------> B02.0-B02.4 Hub/Manager
                                      local implementation
             |                           |
             +-------------+-------------+
                           v
                A02 shared CLI experience
              (Validator + Manager evidence)
                           |
                  +--------+--------+
                  v                 v
          B01/B02 behavior      B03.0-B03.4 Tool
          closure               implementation and evidence
                  |                 |
                  +--------+--------+
                           |
                    stable contracts
                    and candidates
                           |
                           v
                 B04.0 launcher/suite contract
                           |
                           v
          B04 liknon-cli package and one suite archive per target

 C01 + B02/B03 contracts --> C02/C03 official workflows
 B01-B04 + C01-C03 evidence --> D01 integrated conformance

B05 programmatic APIs and language bindings: future, independently activated
                                  |
                                  v
B04 CLI-suite candidate + B02 Hub + selected B05 candidates
          + selected D01 evidence
          + E01 site/Hub delivery candidate
                                  |
                                  v
                  B06 coordinated first public release
                                  |
                                  v
                  F01 Rust 1.90 MSRV modernization
```

Capability readiness remains diagnosable through B01-B03, while public CLI
distribution readiness belongs to B04. `D01` may verify an integrated
candidate, but it does not publish it. `B06` coordinates the owning plans
without rebuilding or mutating their accepted outputs.

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

### B02.5: Manager, Hub, And Suite Handoff Readiness

The repository-side Manager and Hub contracts are implemented. Before B02
release readiness closes, complete the human-owned external setup, public Hub
authority, A02 Manager presentation adoption, focused Manager evidence for
B04, and clean Hub readback defined by B02.5 and B06.

### B03.0: Tool Contract

Before Tool implementation, approve local discovery, read-only behavior,
command grammar, machine outputs, integrity handling, the bootstrap projection
interface, and the policy/persistence inspection boundary. Existing command
names in B03 are candidates until this gate passes.

### B04.0: Launcher And Suite Contract

Before launcher implementation or suite publication, approve executable
resolution, compatibility, process forwarding, four-target package ownership,
one-archive-per-target composition, transactional installation, help, version,
provenance, publication, and recovery contracts. The four physical executables
and `validator|v`, `manager|m`, and `tool|t` namespaces are fixed inputs.

### B05.0: Programmatic API Activation

Do not activate B05 until at least one real programmatic consumer justifies a
deliberate API. A configuration integration can satisfy this gate only with a
complete contract and representative consumer. CLI syntax and internal Rust
modules are not sufficient API design evidence.

### B06.0: Coordinated Release Set

Before any registry package, Hub object, native object, metadata release, or
stable pointer is published, approve one immutable ledger containing the exact
B04 CLI suite candidate, B02 Hub candidate, selected B05 candidates, site
candidate, targets, destinations, dependencies, exclusions, retention windows,
and recovery owners. A changed candidate or destination invalidates that
approval. B06 does not publish a placeholder for a B05 ecosystem whose
activation gate has not passed.

### F01.0: Rust 1.90 Activation

Do not change the current Rust 1.87 declaration while the active roadmap or
first public release is still being completed. After B06 closure, freeze the
exact package, release, target, and documentation inventory; approve the first
versions that require Rust 1.90; and select only modernizations with concrete
Liknon consumers and verification evidence.

## Execution Order

1. A01 has implemented the structural cutover in buildable units; complete its
   final hosted verification.
2. Use the implemented B01 release machinery and Validator evidence as inputs
   to the shared suite foundation; do not complete its superseded standalone
   publication path.
3. The repository-side B02.0-B02.4 implementation supplied the second concrete
   CLI surface and A02 extracted `liknon-cli-ui`, migrated Validator and
   Manager, and established Tool and launcher consumption boundaries.
4. Complete the remaining C01-dependent B03 contract and the B03.3-B03.4 Tool
   handoff evidence without creating another theme or help system.
5. Resolve B04.0 after A02 and the capability command contracts are stable;
   move executable ownership to `liknon-cli`, remove physical compact aliases,
   and build one authenticated four-executable archive per target.
6. Implement C02 and C03 as resources over approved generic contracts.
7. Execute the D01 integrated conformance and agent evaluation required before
   public release, including cross-CLI experience checks from A02.
8. Activate the future B05 programmatic-API phases only through their
   independent consumer-driven gates and prepare candidates for every language
   ecosystem selected for the first public release.
9. Execute B06 only after B04 has handed off one exact CLI-suite candidate,
   B02 has handed off the Hub candidate, selected B05 outputs are ready, E01 is
   ready, and D01 has produced the designated blocking evidence. B06 publishes
   dependencies in order, activates the complete set, verifies public readback,
   promotes stable pointers, and revokes bootstrap credentials.
10. After the current roadmap has reached a terminal state and B06 has closed
    the first public release, activate F01 as one deliberate Rust 1.90
    compatibility transition. Reinventory the completed system before changing
    any MSRV declaration.

Each plan owns its own detailed handoff and acceptance criteria. Completion of
one plan never implicitly marks another plan complete.

## Canonical Ownership Rules

- Current implementation facts belong to code, schemas, tests, and the
  current-state assessment.
- Approved target direction belongs to the target summary.
- Product behavior and distribution belong to A/B plans.
- Shared runtime terminal experience and the `liknon-cli-ui` supporting package
  belong to A02; application-specific presentation remains with its product.
- CLI release mechanisms belong to the shared distribution foundation; the
  public CLI package, native archives, and release authority belong to B04.
- Programmatic package releases and registry authorities remain in B05.
- The coordinated first public activation, release-set ledger, cross-product
  publication order, and closure evidence belong to B06.
- Generic AI governance belongs to C01.
- Official workflow definitions belong to C02 or C03.
- Integrated conformance and agent evaluation belong to D01.
- Public documentation may summarize these contracts but must not redefine
  them independently.
- Post-roadmap Rust MSRV and toolchain modernization belong to F01; active
  product plans continue to use Rust 1.87 until that transition is activated.
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
- shared release mechanisms are implemented once while the complete CLI suite
  keeps one release identity and authority;
- shared terminal behavior is implemented once without merging product command
  grammars, reports, machine schemas, errors, or exit codes;
- first public activation is coordinated by B06 from the exact B04 suite, Hub,
  site, and selected programmatic candidates without rebuilding them;
- all active links resolve and the next executable step is unambiguous.
