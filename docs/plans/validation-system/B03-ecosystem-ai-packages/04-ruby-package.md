# Phase B03.4: Publish The Ruby AI Package

[Back to the ecosystem AI packages plan](README.md)

## Purpose

Publish RubyGem `liknon` as an importable resource library.

## Package Contract

- Expose shared and Ruby resource inventory and exact bytes through Ruby APIs.
- Provide explicit safe materialization into caller-selected destinations.
- Declare no executable and invoke no CLI internally.
- Use no extension merely to distribute static resources.
- Reserve coherent modules for future B04 Ruby APIs.

## Verification

- gem and Bundler install in isolated consumers.
- `require "liknon"` exposes the approved resource contract.
- Gem inspection contains no executable, knowledge corpus, or undeclared file.
- Import and inspection mutate no workspace.
- Public readback matches the retained candidate.

## Exit Criterion

Ruby consumers receive idiomatic access to Liknon AI resources without a global
command or hidden process.
