# B02 Registry Accounts And Publication Authority

[Back to the ecosystem packs plan](README.md)

## Status

Activate a registry section only after its package phase has a complete release
candidate. Do not reserve empty placeholder packages or create long-lived
credentials merely to satisfy this runbook.

## Initial Destinations

| Ecosystem package | Registry | Initial consumption |
| --- | --- | --- |
| Rust pack package | crates.io | Dependency exposing exact sealed pack payloads or explicit materialization |
| JavaScript/TypeScript pack package | npm | Dependency exposing immutable pack roots |

Package names remain undecided until B02.0 resolves their relationship with
future B03 APIs. Neither package installs the CLI.

## Activation Rules

For each registry independently:

1. approve package name, owner, support policy, and exact inventory;
2. verify account recovery and strong multi-factor authentication;
3. inspect the package and pass isolated consumer and polyglot tests;
4. create a narrow protected GitHub environment for that registry;
5. configure registry-native trusted publishing when supported;
6. use a destination-scoped bootstrap credential only when first publication
   cannot use trusted publishing, then revoke it immediately;
7. require credential-free human approval before publication;
8. verify public readback before closing the release.

Build and conformance jobs cannot read publication credentials. Registry
identities, recovery procedures, and package ownership are not shared merely
because packages share content.

## Future Ecosystems

Add RubyGems, PyPI, NuGet, or another registry only with an approved ecosystem
pack, representative fixture, locator contract, package inventory, and
maintenance owner. Extend this runbook at that time using the registry's current
official policy.

## Official Starting Points

- [Cargo publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [crates.io trusted publishing](https://crates.io/docs/trusted-publishing)
- [npm trusted publishing](https://docs.npmjs.com/trusted-publishers/)
