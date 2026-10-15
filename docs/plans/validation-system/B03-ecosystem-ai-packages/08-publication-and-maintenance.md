# Phase B03.8: Publish And Maintain Ecosystem AI Packages

[Back to the ecosystem AI packages plan](README.md)

## Purpose

Publish each proven `liknon` package independently and maintain AI resources
without rebuilding either CLI.

## Publication Contract

- Build shared resources once and retain their manifest and digest.
- Assemble each package from exact canonical resources.
- Inspect packages and run isolated consumer tests before approval.
- Keep build and conformance jobs credential-free.
- Use a separate protected publication identity per registry.
- Prefer registry-native trusted publishing.
- Read accepted packages back and verify version, inventory, resource digests,
  import behavior, and absence of commands.
- Never replace an accepted version or rebuild bytes during retry.

## Maintenance Contract

- Shared changes trigger each package that carries the changed resource set.
- Ecosystem-only changes trigger only that package.
- Resource compatibility is checked against both CLIs independently.
- Skills and prompts are tested through the agent evaluation program before
  promotion.
- B04 additions extend the package contract without moving or duplicating B03
  resources.
- No ecosystem package is published without maintained resources, fixtures,
  ownership, and a tested consumption path.

## Exit Criterion

Every published package is useful without either CLI, installs no command,
contains exact approved AI resources, and can advance independently.
