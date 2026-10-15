# B03 Registry Accounts And Publication Authority

[Back to the ecosystem AI packages plan](README.md)

## Intended Destinations

| Ecosystem | Package | Registry |
| --- | --- | --- |
| Rust | `liknon` | crates.io |
| JavaScript/TypeScript | `liknon` | npm |
| Ruby | `liknon` | RubyGems |
| Python | `liknon` | PyPI |
| .NET | `liknon` | NuGet |

These are library/resource identities. None grants permission to publish a CLI.

## Activation Rules

For each registry independently:

1. confirm name ownership, accountable owners, recovery, and strong MFA;
2. approve exact package inventory and supported runtime;
3. pass package inspection, consumer, mutation, and polyglot tests;
4. create a narrow protected GitHub environment;
5. configure trusted publishing where available;
6. use a narrow bootstrap credential only when the first publication requires
   it, then revoke it;
7. perform credential-free human approval;
8. verify public readback before closing the release.

Build and conformance jobs cannot read publication credentials. Registries do
not share credentials merely because package names match.

## Environment Identities

- `publish-crates-liknon`
- `publish-npm-liknon`
- `publish-rubygems-liknon`
- `publish-pypi-liknon`
- `publish-nuget-liknon`

Exact OIDC claims, owners, recovery, and rotation procedures are finalized
against current registry policy immediately before each package phase.

## Official Starting Points

- [Cargo publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [crates.io trusted publishing](https://crates.io/docs/trusted-publishing)
- [npm trusted publishing](https://docs.npmjs.com/trusted-publishers/)
- [RubyGems trusted publishing](https://guides.rubygems.org/trusted-publishing/)
- [PyPI trusted publishers](https://docs.pypi.org/trusted-publishers/)
- [NuGet trusted publishing](https://learn.microsoft.com/en-us/nuget/nuget-org/trusted-publishing)
