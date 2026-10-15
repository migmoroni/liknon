# Phase B03.3: Publish The Node AI Package

[Back to the ecosystem AI packages plan](README.md)

## Purpose

Publish npm package `liknon` as an importable JavaScript/TypeScript resource
library.

## Package Contract

- Expose shared and JavaScript/TypeScript resource metadata and exact bytes.
- Provide supported ESM and/or CommonJS exports and accurate declarations.
- Provide explicit safe materialization through the approved API.
- Define no `bin` entry and run no mutating install script.
- Download no native payload or other Liknon product.
- Reserve coherent exports for future B04 Node APIs.

## Verification

- npm and pnpm install in isolated consumers.
- Supported module systems return the same manifest and bytes.
- Package lifecycle scripts perform no network or workspace mutation.
- Package inspection contains only approved code, metadata, and resources.
- Public readback matches the retained candidate.

## Exit Criterion

Node consumers import `liknon` for AI resources without receiving a command,
knowledge corpus, native executable, or hidden validation process.
