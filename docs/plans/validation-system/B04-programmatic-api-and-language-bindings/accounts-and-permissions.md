# B04 Registry Accounts And Publication Authority

[Back to the programmatic API and bindings plan](README.md)

## Status

`Future`. Deferred until Phase B04.0 approves the public API and at least one language
binding has a release candidate. Do not create placeholder packages or
long-lived publication credentials merely to satisfy this document.

## Intended Destinations

| Language product | Intended registry | Intended consumption |
| --- | --- | --- |
| Rust library | crates.io | Cargo dependency |
| JavaScript/TypeScript library | npm | Imported package |
| Ruby library | RubyGems | Required gem |
| Python library | PyPI | Imported module |
| .NET library | NuGet | `PackageReference` |

Each identity is created only when its binding is releasable and a
representative consumer exists. B04 does not publish placeholder packages or
parallel names. None of these identities grants permission to publish a CLI or
authored Hub resources through that registry.

## Activation Rules

For each language independently:

1. confirm the `liknon` package name, owner, supported runtimes, consumer, and
   artifact inventory;
2. verify account recovery and strong multi-factor authentication;
3. complete local package inspection and conformance tests;
4. create the narrow protected GitHub environment for that registry;
5. configure registry-native trusted publishing when available;
6. use a destination-scoped bootstrap credential only when the first
   publication cannot use trusted publishing, then revoke it immediately;
7. perform human approval without exposing the publication credential;
8. verify public readback before closing the release.

Registry phases do not share credentials. Build and conformance jobs cannot
read any publication environment.

## Required Future Detail

Before implementation, expand this runbook with exact repository, workflow,
environment, package, owner, OIDC-claim, recovery, and rotation identities for
each approved language phase. Requirements must follow the registry's current
official policy at implementation time.

## Official Starting Points

- [Cargo publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [crates.io trusted publishing](https://crates.io/docs/trusted-publishing)
- [npm trusted publishing](https://docs.npmjs.com/trusted-publishers/)
- [RubyGems trusted publishing](https://guides.rubygems.org/trusted-publishing/)
- [PyPI trusted publishers](https://docs.pypi.org/trusted-publishers/)
- [NuGet trusted publishing](https://learn.microsoft.com/en-us/nuget/nuget-org/trusted-publishing)
