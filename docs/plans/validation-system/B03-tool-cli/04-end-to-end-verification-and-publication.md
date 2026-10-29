# Phase B03.4: Verify Tool And Prepare Publication End To End

## Status

`Provisional`. Blocked by B03.3, local B02/B03 interoperability evidence, and
explicit human candidate approval. B06 coordinates the first public invocation
of the retained Tool workflow.

Operation labels in this document are placeholders for the exact grammar
selected by B03.0.

[Back to the Tool CLI plan](README.md)

## Purpose

Prove Tool installation candidates and local-store interoperability while
retaining Tool's independent offline contract. This phase prepares the
product-owned publication and readback procedures; B06 executes their first
production use.

## Verification Matrix

From clean environments:

1. install Tool from exact GitHub source;
2. install the crates.io candidate;
3. install each authenticated native target;
4. exercise both command names;
5. run the approved bootstrap projection with no workspace and verify exact
   source bytes, version, and digest;
6. consume a contract-valid fixture store without Manager installed;
7. use the repository-built B02.4 Manager to install one knowledge item, skill,
   collection, and dependency graph, then remove Manager from `PATH`;
8. run Tool catalog, search, show, resolve, and context offline;
9. reject modified, unsupported, revoked, unsafe, partial, and oversized state;
10. run the complete A02 help, palette, presentation, `NO_COLOR`, ANSI-free
    machine-output, and alias-equivalence matrix in addition to Tool-specific
    human and JSON assertions;
11. prove Validation CLI absence does not affect resource consumption;
12. resolve and assemble context for project-controlled conformance and external
    workflows with different node identifiers, transitions, and agent profiles;
13. prove that Tool returns both without executing, activating, or applying
    policy to either graph. When official workflows have completed their owning
    phases, repeat the same case without changing Tool code.

During B06 public verification, repeat Step 7 from a clean environment with the
exact public Manager channel and retain a separate cross-channel
interoperability result. The repository-built interoperability evidence remains
the Tool candidate prerequisite.

## Publication Procedure

Build, verification, approval, crates.io publication, GitHub metadata release,
native signing, and Cloudflare publication use isolated jobs and credentials.
No publication job rebuilds a previously verified candidate.

For the first public release, B06 supplies the frozen release-set identity and
coordinates when these B03-owned jobs run. B06 never receives Tool credentials.

The Tool release workflow is a thin product orchestrator over shared
credential-free mechanisms. Reusable workflows receive no broad inherited
secret set; each Tool publication job receives only its destination-specific
protected environment.

crates.io publication follows dependency order: any required unpublished
supporting-crate candidate, including the exact accepted `liknon-cli-ui`
version, is published and read back before `liknon-tool`.

The Tool stable pointer changes only after public readback and clean-environment
installation pass for every supported channel and target.

Before that transition, publication verifies that the source package, every
native archive, installers, checksums, detached signatures, and public manifest
all name the same Tool package-derived version, signed tag, and full source
commit. It rejects any retained candidate whose final hash or provenance
reference differs from the internal build manifest.

The product stable pointer does not permanently depend on a Manager release.
The first coordinated release advances it only after B06 completes the
conditional public cross-channel case above.

Publication also proves Tool storage authorization cannot alter Validation,
Manager, or suite candidates, stable pointers, or release state.

## Website Integration

The site:

- offers Tool independently;
- explains that Manager is optional for Tool runtime but is the canonical Hub
  client;
- explains that Validation CLI is independently selectable;
- may offer all three as a convenience bundle;
- never describes the bundle as a shared executable or runtime dependency;
- documents supported resource schemas and offline behavior.

## Evidence

Retain versioned evidence for source package, signed tag, full commit, Tool
version, source-package digest and closure, target, final native hash,
public-manifest identity, stable-pointer transition, installation channel,
resource-store producer and version, resource-store digest, commands exercised,
semantic results, network isolation, bootstrap-contract version and digest,
artifact inventory, and public readback. Label repository-built and
public-channel Manager evidence distinctly.

## Exit Criterion

B06 can publish every Tool channel from the exact retained candidate without
rebuilding it. After activation, every public Tool channel consumes the same
verified local resources offline, and the application remains independent from
Manager, Validation CLI, and authored resource releases. Public cross-channel
Manager interoperability has its own clean-environment evidence.
