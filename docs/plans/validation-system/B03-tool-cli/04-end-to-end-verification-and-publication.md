# Phase B03.4: Verify And Publish Tool End To End

[Back to the Tool CLI plan](README.md)

## Purpose

Prove public Tool installation and Manager-to-Tool interoperability while
retaining Tool's independent offline contract.

## Verification Matrix

From clean environments:

1. install Tool from exact GitHub source;
2. install the crates.io candidate;
3. install each authenticated native target;
4. exercise both command names;
5. consume a contract-valid fixture store without Manager installed;
6. use Manager to install one knowledge item, skill, collection, and dependency
   graph, then remove Manager from `PATH`;
7. run Tool catalog, search, show, resolve, and context offline;
8. reject modified, unsupported, revoked, unsafe, partial, and oversized state;
9. verify human, JSON, color, standard, and low-vision output;
10. prove Validation CLI absence does not affect resource consumption.

## Publication

Build, verification, approval, crates.io publication, GitHub metadata release,
native signing, and Cloudflare publication use isolated jobs and credentials.
No publication job rebuilds a previously verified candidate.

crates.io publication follows dependency order: any required unpublished
supporting-crate candidate is published and read back before `liknon-tool`.

The Tool stable pointer changes only after public readback and clean-environment
installation pass for every supported channel and target.

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

Retain versioned evidence for source commit, Tool version, target, installation
channel, resource-store digest, commands exercised, semantic results, network
isolation, and artifact inventory.

## Exit Criterion

Every public Tool channel consumes the same verified local resources offline,
and the published application remains independent from Manager, Validation CLI,
and authored resource releases.
