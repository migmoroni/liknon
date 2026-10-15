# Phase B02.3: Build The JavaScript And TypeScript Ecosystem Package

[Back to the ecosystem packs plan](README.md)

## Purpose

Publish an npm package that exposes the shared and JavaScript/TypeScript pack
roots without installing the Liknon CLI or running a workspace-mutating install
hook.

## Package Contract

The package contains:

- the exact approved shared pack;
- the JavaScript/TypeScript pack;
- a small ESM and/or CommonJS API, according to the approved support matrix,
  that returns package version, manifests, and immutable pack roots;
- accurate TypeScript declarations;
- no `bin` entry, CLI executable, native validation payload, or hidden CLI
  download.

The ecosystem pack owns JavaScript, TypeScript, Node.js, package-manager, test
runner, compiler, and stack-specific recipe content. Shared validation
foundations remain in the shared pack.

## Verification

- npm and pnpm install the package in isolated consumer projects.
- The supported module systems return the same canonical pack roots.
- Package scripts perform no network fetch or consumer-workspace mutation.
- Package inspection contains only declared locator code and pack bytes.
- Generic pack validation accepts exact installed bytes.
- One representative JavaScript/TypeScript workspace explicitly activates the
  package and retrieves stack-specific guidance through the CLI.
- Registry readback matches the approved package candidate.

## Exit Criterion

A JavaScript or TypeScript project can install one normal dependency, locate
immutable shared and stack packs, and explicitly supply them to Liknon without
receiving another CLI or validation engine.
