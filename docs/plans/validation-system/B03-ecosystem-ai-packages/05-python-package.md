# Phase B03.5: Publish The Python AI Package

[Back to the ecosystem AI packages plan](README.md)

## Purpose

Publish PyPI project `liknon` as an importable Python resource library.

## Package Contract

- Expose shared and Python resource inventory and exact bytes.
- Provide typed APIs and explicit safe materialization.
- Define no console-script entry point.
- Require no native extension for the initial resource-only package.
- Reserve coherent modules for future B04 Python APIs.

## Verification

- pip and uv resolve the approved package without creating a Liknon command.
- `import liknon` exposes the approved resource contract.
- Wheel and source-distribution inventories contain no CLI or knowledge corpus.
- Installation and import mutate no consumer workspace.
- Public readback matches retained artifacts.

## Exit Criterion

Python consumers import `liknon` for AI resources without receiving a console
application or placeholder native binding.
