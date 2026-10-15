# Phase B04.6: Verify And Publish Language Libraries

[Back to the programmatic API and bindings plan](README.md)

## Purpose

Apply one conformance, provenance, security, and publication discipline to each
language package that has independently completed its binding phase.

## Dependencies

- [Phase B04.0](00-programmatic-api-contract.md) is complete.
- At least one binding phase has an approved release candidate.
- The corresponding account checkpoint in
  [the B04 runbook](accounts-and-permissions.md) is complete.

## Conformance Contract

Each participating language runs the same domain fixtures for:

- configuration validation;
- inspection and plan resolution;
- successful, failed, blocked, skipped, timed-out, and cancelled execution;
- typed diagnostics and report data;
- explicit workspace and mutation boundaries;
- native resource cleanup and host-runtime interruption.

Idiomatic representation may differ, but observable domain meaning may not.

## Publication Contract

- Build programmatic native artifacts independently from B01 and B02 CLI
  artifacts, then assemble them into the existing B03 ecosystem packages.
- Keep build jobs credential-free.
- Inspect final source and native package contents before approval.
- Use one protected publication identity per registry.
- Prefer registry-native trusted publishing where supported.
- Read accepted packages back and verify identity, version, contents, native
  hashes, and import behavior.
- Publish only the language phases included in the approved release set.
- Never publish a placeholder or a package whose only behavior invokes the CLI.

## Verification

- Clean consumer projects import the public package through normal dependency
  declarations.
- Existing B03 resource inventories and explicit materialization remain
  conformant after the API extension.
- No package installs a global `liknon` command.
- Platform selection retrieves only compatible native library artifacts.
- Public package behavior matches the retained candidate and shared fixtures.
- Partial publication recovery never rebuilds or replaces an accepted version.
- Credentials do not enter packages, logs, or retained evidence.

## Exit Criterion

Every published language package is a real, independently useful programmatic
interface with public readback, conformance evidence, and isolated publication
authority.
