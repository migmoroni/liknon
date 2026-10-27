# Phase B05.6: Verify And Publish Language Libraries

## Status

`Future`. Blocked until at least one binding phase has an approved release
candidate.

[Back to the programmatic API and bindings plan](README.md)

## Purpose

Apply one conformance, provenance, security, and publication discipline to each
language package that has independently completed its binding phase.

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
- Public package behavior matches the retained candidate and shared fixtures.
- Partial publication recovery never rebuilds or replaces an accepted version.
- Credentials do not enter packages, logs, or retained evidence.

## Exit Criterion

Every published language package is a real, independently useful programmatic
interface with public readback, conformance evidence, and isolated publication
authority.
