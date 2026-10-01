# Rust CLI Release Gate

## Scenario And Assumptions

This worked recipe validates a maintained Cargo workspace that ships a CLI,
declares Rust 1.87 as its MSRV, commits `Cargo.lock`, distributes documentation
and a generated agent skill, and must not publish or modify source during the
gate. It is an example, not mandatory policy.

## Risks, Invariants, And Evidence

- Source must remain formatted and free of denied selected lints.
- Current and documented Rust examples must pass on the active toolchain.
- The declared minimum toolchain must compile every selected target.
- The crate archive must contain canonical schemas, documentation, and the
  exact generated skill projection, then verify without publication.
- Validation must preserve the initial Git-visible state.

These claims use the [evidence chain](../patterns/evidence-chain.md),
[isolation and determinism](../patterns/isolation-and-determinism.md), the
[Rust guide](../languages/rust.md), and
[reproducible release evidence](../concerns/reproducible-release-evidence.md).
Cargo's package verification builds the packaged source, which directly
addresses a different failure from testing the checkout.
[Source: rust.cargo#cargo-package](https://doc.rust-lang.org/cargo/commands/cargo-package.html)

## Execution Context And Ordering

Run in the repository root with reviewed Rust and MSRV toolchains already
installed. The configuration below checks generation first, then formatting,
linting, tests, docs, and package contents. Run the MSRV suite on a separate
installed toolchain or CI matrix because a declarative tool entry cannot make a
missing toolchain available. Network-free package verification additionally
requires a populated Cargo cache. Publishing is excluded.

## Complete Configuration Example

```json
{
  "schemaVersion": 6,
  "workspaceRoot": ".",
  "defaultGroup": "release",
  "outputLimitBytes": 1048576,
  "repository": {
    "provider": "git",
    "toolId": "git",
    "detectMutations": true
  },
  "tools": [
    {
      "id": "cargo",
      "program": "cargo",
      "requiresTools": [],
      "versionArgs": ["--version"],
      "versionParser": "firstSemver",
      "versionRequirement": ">=1.87, <2.0"
    },
    {
      "id": "git",
      "program": "git",
      "requiresTools": [],
      "versionArgs": ["--version"],
      "versionParser": "firstSemver",
      "versionRequirement": ">=2.0, <3.0"
    }
  ],
  "checks": [
    {
      "id": "knowledge.check",
      "label": "Generated knowledge",
      "description": "Verifies registries, links, sources, fixtures, and the exact skill projection.",
      "toolId": "cargo",
      "args": ["run", "--locked", "--example", "knowledge_release", "--", "--check"],
      "requiresTools": [],
      "timeoutSeconds": 120
    },
    {
      "id": "rust.format",
      "label": "Rust formatting",
      "description": "Checks formatting without rewriting source.",
      "toolId": "cargo",
      "args": ["fmt", "--all", "--", "--check"],
      "requiresTools": [],
      "timeoutSeconds": 120
    },
    {
      "id": "rust.clippy",
      "label": "Rust lints",
      "description": "Denies selected compiler and Clippy warnings.",
      "toolId": "cargo",
      "args": ["clippy", "--workspace", "--all-targets", "--locked", "--", "-D", "warnings"],
      "requiresTools": [],
      "timeoutSeconds": 600
    },
    {
      "id": "rust.test",
      "label": "Rust tests",
      "description": "Runs workspace tests for all selected targets.",
      "toolId": "cargo",
      "args": ["test", "--workspace", "--all-targets", "--locked"],
      "requiresTools": [],
      "timeoutSeconds": 900
    },
    {
      "id": "rust.doctest",
      "label": "Rust documentation tests",
      "description": "Runs executable Rust documentation examples.",
      "toolId": "cargo",
      "args": ["test", "--doc", "--locked"],
      "requiresTools": [],
      "timeoutSeconds": 300
    },
    {
      "id": "rust.docs",
      "label": "Rust documentation",
      "description": "Builds workspace API documentation without dependencies.",
      "toolId": "cargo",
      "args": ["doc", "--workspace", "--no-deps", "--locked"],
      "requiresTools": [],
      "timeoutSeconds": 600
    },
    {
      "id": "rust.package-list",
      "label": "Package contents",
      "description": "Lists the exact crate source package contents.",
      "toolId": "cargo",
      "args": ["package", "--allow-dirty", "--list"],
      "requiresTools": [],
      "timeoutSeconds": 300
    },
    {
      "id": "rust.package",
      "label": "Package verification",
      "description": "Assembles and verifies the crate package without publishing.",
      "toolId": "cargo",
      "args": ["package", "--allow-dirty", "--locked", "--offline"],
      "requiresTools": [],
      "timeoutSeconds": 900
    }
  ],
  "suites": [
    {
      "id": "rust.release",
      "label": "Rust CLI release",
      "description": "Produces bounded source, test, documentation, and package evidence.",
      "workingDirectory": ".",
      "checks": [
        { "checkId": "knowledge.check", "dependsOn": [] },
        { "checkId": "rust.format", "dependsOn": ["knowledge.check"] },
        { "checkId": "rust.clippy", "dependsOn": ["rust.format"] },
        { "checkId": "rust.test", "dependsOn": ["rust.clippy"] },
        { "checkId": "rust.doctest", "dependsOn": ["rust.test"] },
        { "checkId": "rust.docs", "dependsOn": ["rust.doctest"] },
        { "checkId": "rust.package-list", "dependsOn": ["rust.docs"] },
        { "checkId": "rust.package", "dependsOn": ["rust.package-list"] }
      ]
    }
  ],
  "groups": [
    {
      "id": "release",
      "label": "Release gate",
      "description": "Runs the complete Rust CLI release example.",
      "members": [{ "kind": "suite", "id": "rust.release" }]
    }
  ]
}
```

## Known Gaps And Rejected Alternatives

The example omits cross-platform execution, installer smoke tests,
vulnerability/license analysis, performance, Web accessibility conformance, and
bit-for-bit binary reproducibility. It rejects `cargo publish` because external
state mutation is not validation evidence. It does not add every feature
combination without a demonstrated promise or failure mode.

Residual uncertainty includes cached dependency availability, host-specific
build behavior, build-script effects outside Git visibility, and untested
consumer installation. Add evidence only when product promises or observed
failures make those gaps material.
