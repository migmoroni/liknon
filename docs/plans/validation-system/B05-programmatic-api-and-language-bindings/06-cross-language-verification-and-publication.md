# Phase B05.6: Verify And Prepare Language Libraries For Publication

## Status

`Future`. Blocked until at least one binding phase has an approved release
candidate. B06 coordinates the first public invocation for the exact language
packages selected in its frozen release set.

[Back to the programmatic API and bindings plan](README.md)

## Purpose

Apply one conformance, provenance, security, and publication-readiness
discipline to each language package that has independently completed its
binding phase. Retain exact candidates and product-owned registry workflows for
B06 without requiring unimplemented ecosystems to participate.

## Dependencies

- [Phase B05.0](00-programmatic-api-contract.md) is complete.
- At least one binding phase has an approved release candidate.
- The corresponding account checkpoint in
  [the B05 runbook](accounts-and-permissions.md) is complete.

## Conformance Contract

Each participating language runs the same domain fixtures for:

- configuration validation;
- inspection and plan resolution;
- successful, failed, blocked, skipped, timed-out, and cancelled execution;
- typed diagnostics and report data;
- explicit workspace and mutation boundaries;
- native resource cleanup and host-runtime interruption.

When Git-hook integration is included, each participating language also runs
the same fixtures for clean creation, merge, update, no-op reapply, removal,
stale plans, malformed provider input, absent or incompatible CLIs, unrelated
configuration preservation, and atomic apply.

Idiomatic representation may differ, but observable domain meaning may not.

## Publication Contract

For the first public release, B06 invokes the destination-specific workflows
defined here after candidate rehearsal and human approval. Registry credentials
remain B05-owned and never enter the B06 coordinator.

- When selected APIs require native artifacts, build them independently from
  all three CLI artifacts, then assemble only the approved language packages.
- Keep build jobs credential-free.
- Inspect final source and native package contents before approval.
- Use one protected publication identity per registry.
- Prefer registry-native trusted publishing where supported.
- Read accepted packages back and verify identity, version, contents, native
  hashes, and import behavior.
- Publish only the language phases included in the approved release set.
- Never publish a placeholder or a package whose only behavior uses the CLI as
  hidden programmatic transport. An approved API that configures a hook to
  invoke the native Validation command directly remains valid.

## Verification

- Clean consumer projects import the public package through normal dependency
  declarations.
- No package installs a global `liknon` command.
- No package embeds authored Hub resources or performs implicit Hub access.
- No package install, restore, import, or require step mutates hooks, downloads
  a CLI, or executes validation.
- Approved provider integrations produce equivalent plans and diagnostics
  across languages and preserve unrelated configuration.
- Platform selection retrieves only compatible native library artifacts.
- Before publication, an isolated installed candidate matches shared fixtures.
  During B06 public readback, the accepted package must match that candidate.
- Partial publication recovery never rebuilds or replaces an accepted version.
- Credentials do not enter packages, logs, or retained evidence.

## Exit Criterion

Every selected language package is a real, independently useful programmatic
interface with an exact retained candidate, conformance evidence, isolated
publication authority, and a public-readback procedure that B06 can execute.
