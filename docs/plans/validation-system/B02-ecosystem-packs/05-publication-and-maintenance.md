# Phase B02.5: Publish And Maintain Ecosystem Packs

[Back to the ecosystem packs plan](README.md)

## Purpose

Publish each proven ecosystem package independently and establish a maintenance
model that permits content updates without rebuilding the CLI.

## Dependencies

- Phases B02.0 through B02.4 are complete.
- The applicable account checkpoint in
  [the B02 runbook](accounts-and-permissions.md) is complete.
- Each package has an inspected immutable release candidate and public name.

## Publication Contract

- Build packs once from tracked sources and retain their manifests and digests.
- Assemble each ecosystem package from exact retained pack bytes.
- Keep build and verification jobs credential-free.
- Use a separate protected publication identity for every registry.
- Prefer registry-native trusted publishing where available.
- Read accepted packages back and verify version, inventory, pack digests,
  locator API, and absence of global commands or install mutations.
- Never replace an accepted version or rebuild bytes during retry.
- Publish only ecosystems with substantive content and maintained fixtures.

## Maintenance Contract

- Shared changes trigger every package that carries the changed shared pack.
- Ecosystem-only changes trigger only that ecosystem's package.
- Pack compatibility is evaluated before publication, independently from CLI
  release cadence.
- Source links and time-sensitive tool guidance are reviewed according to their
  editorial metadata without pinning arbitrary patch versions in prose.
- Removal or replacement of a resource follows an explicit pack contract; it
  is not hidden by a locator API fallback.

## Exit Criterion

Every published package can be installed and read back through its native
registry, exposes exact approved packs, and can advance independently without
republishing or changing the Liknon CLI.
