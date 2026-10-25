# A00.1: Unified Launcher, CLI Suite, And Command Plan Realignment

## 1. Status

`Implementation-ready` documentation-only meta-plan.

This plan reconciles the active validation-system plans with the approved
command architecture. It does not authorize Rust, schema, CI, packaging,
publication, workspace-state, or public-documentation changes. Those changes
remain with their owning implementation plans.

Completion means that the active plan set describes one coherent route from
the current single validation executable to three capability CLIs plus one
capability-free launcher, with `liknon-cli` as the promoted aggregate Cargo
package. It does not mean that any target executable exists.

## 2. Objective

Realign the active plans so that:

- each capability CLI has a name that states its responsibility;
- the unified `liknon` entry point is a transparent launcher rather than the
  validation implementation;
- direct and unified command forms have explicit, non-overlapping namespaces;
- the Cargo package `liknon-cli` installs the launcher and all three capability
  command pairs as binary targets owned by that package;
- the package name `liknon` remains reserved for future programmatic
  integration libraries, including the Rust crate, and never identifies the
  launcher package;
- no deprecated command, fallback route, or compatibility alias is retained;
- the launcher has one owning plan instead of being distributed implicitly by
  the validation plan;
- direct applications remain independently installable and operable;
- integrated routing and bundle concerns do not leak into Manager, Tool, or
  future programmatic APIs.

## 3. Approved Command Architecture

### 3.1 Capability CLIs

| Capability | Cargo application package | Explicit command | Compact command |
| --- | --- | --- | --- |
| Validation | `liknon-validate` | `liknon-validate` | `likv` |
| Resource management | `liknon-manager` | `liknon-manager` | `likm` |
| Local resource access | `liknon-tool` | `liknon-tool` | `likt` |

These applications own behavior. They may share libraries, but they do not
invoke one another and do not depend on the unified launcher.

### 3.2 Unified Launcher And Aggregate Package

| Role | Aggregate Cargo package | Explicit command | Compact command |
| --- | --- | --- | --- |
| Transparent dispatcher | `liknon-cli` | `liknon` | `likx` |

The launcher owns no validation, network, resource mutation, resource
interpretation, policy, persistence, or AI capability. It selects one exact
capability executable, forwards the remaining arguments and process streams,
and returns that executable's outcome.

The launcher is a fourth executable boundary, not a fourth capability product.

`liknon-cli` is also the promoted complete-suite installation route. The
package declares these binary targets itself:

```text
liknon             likx
liknon-validate    likv
liknon-manager     likm
liknon-tool        likt
```

The six capability targets reuse the same shared implementation crates as the
three standalone Cargo application packages. They are not obtained by relying
on Cargo to install executable targets from dependencies: ordinary Cargo
dependencies provide library targets to a build and do not install dependency
binaries. Consequently, `cargo install liknon-cli --locked` installs the
complete suite because every listed executable belongs to `liknon-cli`.

The standalone packages remain selective installation routes. For one Cargo
installation root, the aggregate and standalone routes are alternative owners
of the same capability command names; installation and recovery documentation
must detect or explain that ownership collision instead of silently replacing
executables.

The future Cargo package `liknon` belongs to the programmatic integration plan,
as do corresponding integration packages in other language ecosystems. These
packages own no CLI executable merely because the terminal launcher is named
`liknon`.

### 3.3 Final Unified Grammar

The first argument to the launcher always selects a capability namespace:

```text
liknon validate validate fast -> liknon-validate validate fast
liknon validate check ...     -> liknon-validate check ...
liknon validate config ...    -> liknon-validate config ...
liknon manager install ...    -> liknon-manager install ...
liknon tool show ...          -> liknon-tool show ...
```

`likx` has exactly the same launcher grammar as `liknon`. The direct compact
commands retain the grammar of their respective applications:

```text
likv validate fast
likm install ...
likt show ...
```

There is no default capability namespace. In particular:

- `liknon check ...` is not forwarded implicitly.

Component discovery, process replacement, signal forwarding, and
missing-component behavior belong to the launcher contract created by this
meta-plan.

### 3.4 Launcher Help And Version

`liknon --help` and `likx --help` describe only the launcher contract. They
must:

- identify the launcher as a dispatcher;
- present the `validate`, `manager`, and `tool` namespaces with one brief
  description each;
- show the unified invocation grammar;
- direct users to `liknon <namespace> --help` for capability-specific help;
- avoid reproducing command trees, options, or reference text owned by a
  capability CLI; and
- remain available even when a capability executable is absent or invalid.

`liknon <namespace> --help` dispatches to the selected capability executable's
own help. The launcher does not synthesize or merge that detailed output.

`liknon --version` and `likx --version` report the launcher version first,
followed by the version of `liknon-validate`, `liknon-manager`, and
`liknon-tool`. Component inspection must use the same trusted executable
resolution that normal dispatch would use. An absent, incompatible, or
unresponsive component is reported explicitly rather than omitted or replaced
with manifest metadata that does not describe the selected executable.

B04.0 still decides the exact output format, inspection timeout, and exit code
when one or more component versions cannot be obtained. It may not reopen the
help scope, component list, ordering, or requirement to expose failures.

## 4. Architectural Invariants

1. `liknon` and `likx` are equivalent launcher entry points, not validation
   aliases.
2. `liknon-validate` and `likv` are equivalent validation entry points.
3. Every unified invocation names `validate`, `manager`, or `tool` explicitly.
4. The launcher removes only the selected namespace before dispatch.
5. Dispatch is shell-free and never reconstructs arguments as a command
   string.
6. The launcher never downloads, installs, updates, repairs, or silently
   selects a missing component.
7. Direct application commands remain usable when the launcher and the other
   applications are absent.
8. Capability applications contain no launcher-only routing branch.
9. The `liknon-cli` aggregate package and complete native suite install several
   executables, but they create no in-process runtime coupling among capability
   applications.
10. Product separation is defense in depth; the plans must not describe it as
    an operating-system sandbox when all processes run with the same user
    authority.
11. Cargo aggregation is implemented with binary targets owned by
    `liknon-cli` and shared implementation libraries, never by assuming that a
    package dependency installs another package's binaries.
12. Package `liknon` remains reserved for programmatic integration in each
    language ecosystem. In Cargo, it is distinct from both package
    `liknon-cli` and command `liknon`.
13. Launcher help summarizes namespaces but delegates all capability-specific
    documentation to the selected executable.
14. Launcher version output identifies the launcher first and then every
    capability executable selected through the normal discovery contract.

## 5. Authority And Inputs

Apply this precedence while executing the meta-plan:

1. repository code, schemas, tests, manifests, and CI remain the source of
   truth for current implementation;
2. `../avaliacao-estado-atual.md` remains the current-state record;
3. the approved decisions in this meta-plan amend the target direction that
   must be incorporated into `../resumo-estado-alvo.md`;
4. the revised target summary and root roadmap govern subordinate active
   plans;
5. archived plans remain historical evidence and are not rewritten.

Do not change the current-state assessment merely to describe this target. It
may change only when repository evidence changes.

## 6. Target Plan Topology

The realigned active plan set must use this topology:

```text
validation-system/
|-- README.md
|-- avaliacao-estado-atual.md
|-- resumo-estado-alvo.md
|-- A00-plan-coherence-and-reorganization/        # historical
|-- A00.1-unified-launcher-plan-realignment/      # this meta-plan
|-- A01-product-and-repository-separation/
|-- B01-validation-cli-distribution/
|-- B02-resource-hub-and-manager/
|-- B03-tool-cli/
|-- B04-unified-launcher-and-cli-suite/
|-- B05-programmatic-api-and-language-bindings/
|-- C01-ai-governance-and-workflow-foundation/
|-- C02-official-action-workflow/
|-- C03-official-validation-engineering-workflow/
`-- D01-system-conformance-and-agent-evaluation/
```

Required plan moves:

- rename `B01-cli-distribution/` to
  `B01-validation-cli-distribution/` and restrict it to
  `liknon-validate` / `likv`;
- move the current `B04-programmatic-api-and-language-bindings/` to
  `B05-programmatic-api-and-language-bindings/` without changing its future
  activation principle;
- create `B04-unified-launcher-and-cli-suite/` as the sole owner of package
  `liknon-cli`, commands `liknon` / `likx`, dispatch, component discovery, the
  aggregate Cargo installation, and complete native bundles.

The B04-to-B05 move and its reference repair must finish before the new B04 is
created or registered in the active roadmap. The active plan set must never
contain two normative plans identified as B04.

## 7. Workstream 0: Build The Plan-Reference Inventory

Before editing normative text, produce an evidence-based inventory of active
references to:

- `liknon-cli`;
- `apps/liknon-cli`;
- package `liknon` in any ecosystem where it could be confused with command
  `liknon`;
- `liknon` and `likx` as validation aliases;
- "three CLIs" or "three applications" where launcher treatment matters;
- B01 and B04 paths, links, statuses, and ownership;
- distribution artifacts, tags, package names, archive names, manifests,
  installers, account requirements, and verification matrices;
- direct and integrated command examples in A, B, C, and D tracks.

Classify each match as one of:

| Class | Required treatment |
| --- | --- |
| Current-state fact | Preserve until implementation changes the repository |
| Approved target or active plan | Reconcile through this meta-plan |
| Historical evidence | Preserve unchanged |
| Maintained public documentation | Leave current until the owning implementation changes behavior |

Do not perform a blind repository-wide replacement.

## 8. Workstream 1: Reconcile Foundation Documents

### 8.1 Target-State Summary

Update `../resumo-estado-alvo.md` first:

- describe three capability CLIs and one capability-free launcher;
- move the current validation-only package identity from `liknon-cli` to
  `liknon-validate`;
- replace validation commands `liknon` / `likx` with
  `liknon-validate` / `likv`;
- redefine `liknon-cli` as the aggregate source package that owns launcher and
  capability binary targets;
- add `liknon` / `likx` as launcher commands without assigning package name
  `liknon` to them;
- reserve package `liknon` for future programmatic integration libraries owned
  by B05, including the Rust crate;
- record the explicit namespace grammar without renaming validation
  subcommands;
- record the approved launcher help and aggregate version behavior;
- update the repository shape to contain four application packages;
- distinguish independent application distribution from the promoted
  `liknon-cli` aggregate route and complete native suite;
- preserve Manager, Tool, Hub, AI-resource, and API boundaries;
- state that no legacy command grammar is retained;
- add unresolved launcher release details to an explicit B04 decision gate
  rather than inventing them in the summary.

### 8.2 Root Roadmap

Update `../README.md` after the target summary:

- list B01 as Validation CLI distribution for `liknon-validate` / `likv`;
- retain B02 and B03 as the Manager and Tool owners;
- add B04 as unified launcher, aggregate package, and complete-suite ownership;
- move future programmatic APIs to B05;
- update the dependency graph and execution order;
- make B04 contract work depend on A01 boundaries and approved component
  command contracts;
- make B04 integrated verification and distribution depend on the relevant
  B01-B03 release candidates;
- keep D01 as integrated conformance rather than publication ownership.

### 8.3 Foundation Checkpoint

Do not update subordinate plans until the target summary and roadmap agree on:

- product names and command identities;
- package names, executable names, and their distinct ownership;
- three capability boundaries plus one launcher boundary;
- explicit namespace routing;
- B01-B05 ownership;
- decision gates and execution order.

## 9. Workstream 2: Reconcile A01 Product Separation

Update the A01 README and phases so that the structural ledger can distinguish
implemented state from newly approved targets:

- map the current validation implementation to `apps/liknon-validate`;
- map the current validation semantics to one shared implementation;
- record `apps/liknon-cli` as the new aggregate package and launcher
  application rather than pretending that target already exists;
- require its capability binary targets and the three standalone packages to
  reuse shared implementation crates rather than duplicate behavior;
- keep launcher dispatch in its own binary targets and out of capability
  libraries;
- reserve package `liknon` for B05 instead of creating an application package
  with that name;
- treat `liknon-manager` and `liknon-tool` according to their existing B02/B03
  gates;
- define package and dependency direction so the launcher binary target imports
  no capability API merely to dispatch, while sibling targets use the approved
  shared implementations;
- preserve shell-free process handling and platform-specific signal/exit
  guarantees;
- update structural verification for direct and unified invocation;
- retain the rule that capability applications do not invoke one another;
- explicitly permit only the launcher to invoke a selected capability binary.

A01 must decide the buildable cutover units without introducing an
intermediate public command contract.

## 10. Workstream 3: Refound B01 Around Validation Only

Move and rewrite B01 as `B01-validation-cli-distribution/`.

Across its README, phases, runbook, examples, and artifact contracts:

- use package `liknon-validate`;
- install commands `liknon-validate` and `likv`;
- use tag `liknon-validate-v<VERSION>`;
- use `liknon-validate` in archive, manifest, channel, installer, and evidence
  identities;
- remove launcher, Manager, Tool, and bundle ownership;
- verify direct long/compact command equivalence;
- retain source, crates.io, Cloudflare, platform, authenticity, recovery, and
  publication requirements that remain applicable to this product;
- update every internal and external link after the directory move.

B01 must not publish package `liknon-cli`, commands `liknon` / `likx`, or
package `liknon`; it must not require the launcher for installation or
verification.

## 11. Workstream 4: Relocate Programmatic APIs To B05

Move the current B04 API plan to B05 and repair every link, status table,
dependency graph, gate name, and track reference before creating the new B04.
Keep it `Future`. Clarify that language APIs neither expose the launcher as an
in-process API nor package CLI copies by default. Reserve package identity
`liknon` in each supported ecosystem for the integration API activated by this
plan. In Cargo, crate `liknon` must not provide command `liknon`.

This workstream is complete only when no active reference still identifies the
programmatic API plan as B04 and the B04 identifier is free for reassignment.

## 12. Workstream 5: Create B04 Launcher And CLI Suite Plan

Create an active B04 plan with an initial decision gate. It is the sole owner
of:

- aggregate Cargo package `liknon-cli`;
- commands `liknon` and `likx`;
- all launcher and capability binary targets owned by the aggregate package;
- reuse of the same shared implementation crates used by standalone packages;
- namespace parsing for `validate`, `manager`, and `tool`;
- deterministic executable discovery;
- safe argument, environment, working-directory, stdin, stdout, stderr,
  signal, interruption, and exit-status forwarding;
- the approved launcher help and aggregate version contract;
- diagnostics and missing-component behavior;
- aggregate crates.io installation as the promoted complete-suite route;
- complete native suites and their exact component manifests;
- composition rules that consume independently published standalone
  components without taking ownership of their publication;
- cross-channel component-location rules;
- launcher/component compatibility metadata;
- launcher-specific provenance, recovery, and publication evidence.

B04.0 must resolve, rather than assume:

1. executable lookup precedence and trusted locations;
2. behavior when a selected component is absent or incompatible;
3. whether launcher and components have independent or coordinated release
   versions;
4. the exact `liknon-cli` binary target set and shared implementation-crate
   dependency graph;
5. source-package and native-suite composition per channel;
6. whether partial native suites are supported;
7. exact aggregate-version output format, component inspection timeout, and
   exit status when inspection cannot obtain every required version;
8. POSIX process replacement and Windows process/signal parity;
9. installation-root ownership, collision detection, replacement, and recovery
   when aggregate and standalone packages expose the same command names;
10. exact boundaries between aggregate publication and standalone component
    publication.

The plan must require shell-free dispatch, exact argument preservation, no
implicit installation, no hidden fallback to validation, and proof that
`cargo install liknon-cli` installs package-owned targets rather than relying
on executable artifacts from dependencies.

The new B04 directory, roadmap entry, and links may be created only after
Workstream 4 has completed.

## 13. Workstream 6: Reconcile B02, B03, C, And D Tracks

### 13.1 Manager And Tool

Preserve the direct products and their existing decision gates. Update only
cross-product statements needed to establish that:

- `liknon manager <args>` is equivalent to `liknon-manager <args>` after
  launcher dispatch;
- `liknon tool <args>` is equivalent to `liknon-tool <args>` after launcher
  dispatch;
- Manager and Tool do not depend on, invoke, or embed the launcher;
- their individual releases remain valid without the aggregate package;
- B04, not B02 or B03, owns unified routing, aggregate Cargo publication, and
  complete native-suite publication.

### 13.2 AI Governance And Workflows

Use capability roles in generic contracts. Where an executable form is
necessary, distinguish:

- direct deterministic use of `liknon-validate`, `liknon-manager`, or
  `liknon-tool`;
- equivalent unified use through the explicit launcher namespace;
- AI-Tool the resource from `liknon-tool` the executable.

Do not alter policy, workflow, profile, skill, persistence, or authority
semantics merely because command routing changed.

### 13.3 Integrated Conformance

Extend D01's scenario and isolation matrices to cover:

- every direct long and compact command;
- every unified namespace through both `liknon` and `likx`;
- root help that remains launcher-focused and capability help reached through
  namespace dispatch;
- launcher-first aggregate version output for all three resolved capability
  executables, including explicit unavailable and invalid states;
- installation and behavior equivalence between aggregate capability targets
  and their standalone-package counterparts;
- direct/unified stdout, stderr, JSON, exit-code, interruption, and working-
  directory equivalence;
- missing, incompatible, shadowed, or replaced component executables;
- argument-boundary and executable-path attacks;
- proof that the launcher performs no network, validation, resource mutation,
  resource interpretation, or AI action itself;
- independent operation when the launcher is absent;
- explicit rejection of removed command forms.

D01 verifies the integrated candidate but does not publish the launcher or any
capability product.

## 14. Workstream 7: Terminology And Link Sweep

After all owning documents are reconciled:

- use **Validation CLI** for the capability role and `liknon-validate` for its
  concrete package or command;
- use **launcher** or **unified launcher** for `liknon` / `likx`;
- use **aggregate CLI package** for Cargo package `liknon-cli`;
- use **integration package** for package `liknon`, qualified by language or
  ecosystem when ambiguity exists;
- never call the launcher the validation CLI;
- use **three capability CLIs plus one launcher**, not an unexplained "four
  CLIs";
- reserve **native suite** or **bundle** for native artifact composition, not
  Cargo dependency semantics or runtime coupling;
- update relative links, plan tables, dependency graphs, status summaries,
  acceptance criteria, package paths, tags, examples, and account runbooks;
- preserve current-state and historical language where it remains factually
  correct.

## 15. Verification

The meta-plan execution must include:

1. an active-plan search proving every normative `liknon-cli` target identity
   denotes the aggregate package rather than Validation CLI alone;
2. a search proving active target text no longer presents `liknon` / `likx` as
   validation aliases;
3. a search proving package identity `liknon` is reserved for programmatic
   integration and never owns a CLI binary;
4. a search proving final examples use explicit launcher namespaces and the
   approved command grammar;
5. tests proving root launcher help remains brief, capability-specific help is
   delegated, and `liknon` / `likx` expose equivalent help;
6. tests proving launcher-first version ordering, use of normal component
   resolution, complete component coverage, and explicit inspection failures;
7. a search proving removed command forms occur only in explicit rejection
   tests or historical/current-state evidence;
8. link validation for every moved or newly created plan path;
9. status and dependency validation for A01 and B01-B05;
10. confirmation that the programmatic API plan was fully relocated to B05
    before the new B04 was created or linked;
11. confirmation that archived files are byte-for-byte unchanged;
12. confirmation that maintained public documentation was not rewritten ahead
   of implementation;
13. `git diff --check` and repository documentation checks that apply to these
   files.

Rust gates are not required for execution of this documentation-only
meta-plan unless a build input or product source is changed accidentally.

## 16. Acceptance Criteria

- [ ] The target summary defines three capability CLIs and one launcher.
- [ ] `liknon-validate` / `likv` exclusively identify direct validation.
- [ ] `liknon` / `likx` exclusively identify the unified launcher.
- [ ] Cargo package `liknon-cli` installs launcher and capability command
      targets backed by the same implementations as standalone packages.
- [ ] Package identity `liknon` is reserved for future programmatic integration
      in each ecosystem and owns no CLI command.
- [ ] Unified invocations always name `validate`, `manager`, or `tool`.
- [ ] Launcher help briefly identifies all three namespaces without duplicating
      capability-specific help, which remains reachable through dispatch.
- [ ] Launcher version output identifies the launcher first, reports every
      resolved capability version, and exposes every inspection failure.
- [ ] A01 maps current implementation to the target without claiming
      unimplemented products already exist.
- [ ] B01 owns only Validation CLI distribution.
- [ ] B02 owns only Hub and Manager behavior/distribution.
- [ ] B03 owns only Tool behavior/distribution.
- [ ] B04 solely owns launcher routing, aggregate Cargo distribution, and
      complete native-suite distribution.
- [ ] Programmatic APIs are consistently owned by future B05.
- [ ] B05 relocation and reference repair precede creation or registration of
      the new B04, with no transient duplicate B04 ownership.
- [ ] C plans consume command capabilities without embedding product routing.
- [ ] D01 verifies direct and unified paths without taking publication
      ownership.
- [ ] Current-state and historical records remain factually intact.
- [ ] Every active link resolves and the next implementation step is
      unambiguous.

## 17. Execution Order

1. Complete the reference inventory.
2. Reconcile the target summary and root roadmap.
3. Reconcile A01 product and repository boundaries.
4. Move and refound B01 around `liknon-validate` / `likv`.
5. Move the future API plan from B04 to B05 and repair all references.
6. Create B04 launcher and CLI-suite ownership only after B05 is established.
7. Reconcile B02, B03, C01-C03, and D01 references and tests.
8. Perform the terminology, link, status, and acceptance sweep.
9. Mark this meta-plan `Historical` only after all verification succeeds.

After this meta-plan is complete, the next executable product step returns to
A01.0's evidence-based structural inventory.
