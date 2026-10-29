# B03 Tool Handoff Accounts And Permissions

## Status

`Approved target`. Tool has no standalone publication authority.

[Back to the Tool CLI plan](README.md)

## Purpose

Record that B03's handoff to B04 is credential-free and prevent obsolete
standalone Tool publication access from being provisioned.

## Authority Boundary

B03 owns Tool behavior, local safety contracts, tests, and evidence. It does
not own:

- a crates.io `liknon-tool` publication identity;
- a Tool release tag;
- a Tool-specific R2 prefix or stable pointer;
- platform-signing authority;
- a Tool publication environment.

B04 owns those effects for the complete `liknon-cli` suite. B06 coordinates
their first use.

## Handoff Evidence

B03 may generate and retain:

- source-closure reports;
- package inventory reports;
- direct and routed smoke-test results;
- target-native Tool evidence;
- machine-contract fixtures;
- offline and read-only effect observations;
- compatibility evidence for Manager-produced state.

These artifacts contain no secret and grant no publication authority.

## Required Protections

- Tool test jobs have no network or publication credential.
- Evidence is bound to commit, suite candidate, target, executable hash, and
  test revision.
- A stale or mismatched report blocks the B04 target.
- No B03 workflow can write Hub, CLI-suite, or site production state.
- No secret enters fixtures, resource stores, logs, caches, or retained reports.

## Exit Criterion

B03 can prove Tool readiness to B04 without possessing any destination
credential or independent public release identity.
