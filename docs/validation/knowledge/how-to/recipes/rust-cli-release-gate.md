# Rust CLI Release Gate

## Goal

Create a schema-valid, non-publishing gate for a Cargo-managed Rust CLI before
assembling its source package.

## Applicability And Preconditions

Use this recipe when a workspace ships one Rust CLI, commits `Cargo.lock`, has
maintained tests and Rustdoc, declares its supported Rust range, and packages
through Cargo. Install the active stable toolchain, rustfmt, Clippy, Git, and the
declared MSRV toolchain before the gate. Adapt workspace and feature selection
when the package contract differs.

## Risks And Invariants

- Authored Rust must remain formatted and free of denied selected lints.
- Selected targets, tests, and documentation examples must compile and pass.
- The declared MSRV must compile the promised target selection.
- The Cargo source archive must contain the intended public inputs and build
  without publishing.
- Validation must expose unintended Git-visible mutation.

Cargo package verification assembles and builds packaged source, observing a
different failure surface from testing only the checkout.
[Source: rust.cargo#cargo-package](https://doc.rust-lang.org/cargo/commands/cargo-package.html)

## Inputs And Workspace Assumptions

The example assumes `.validation/config.json` is inside the CLI workspace and
uses `workspaceRoot: ".."`. It assumes default features represent the release
surface and that the crate does not require a private registry or external
service. Replace working directories, package/feature flags, timeouts, and tool
version requirements to match the consumer. The example contains no dependency
on workspace-validator's source tree, skills, or embedded assets.

## Procedure

1. Review `Cargo.toml`, `Cargo.lock`, package include/exclude rules, build
   scripts, test effects, feature promises, and `rust-version`.
2. Run `cargo +<declared-msrv> check --workspace --all-targets --locked` after
   replacing `<declared-msrv>` with the manifest's toolchain version.
3. Save the adapted configuration below and run `workspace-validator config
   validate`.
4. Inspect the resolved `release` group, then run `workspace-validator validate
   release --format=json`.
5. Inspect package contents and every repository mutation before accepting the
   result. Do not add publication to the gate.

## Complete Configuration Example

The following configuration is complete for the declared default-feature
workspace shape:

```json
{
  "schemaVersion": 6,
  "workspaceRoot": "..",
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
      "versionRequirement": ">=1, <2"
    },
    {
      "id": "git",
      "program": "git",
      "requiresTools": [],
      "versionArgs": ["--version"],
      "versionParser": "firstSemver",
      "versionRequirement": ">=2, <3"
    }
  ],
  "checks": [
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
      "id": "rust.check",
      "label": "Rust compilation",
      "description": "Checks selected workspace targets with the committed lockfile.",
      "toolId": "cargo",
      "args": ["check", "--workspace", "--all-targets", "--locked"],
      "requiresTools": [],
      "timeoutSeconds": 600
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
      "description": "Runs tests for selected workspace targets.",
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
      "id": "rust.package-list",
      "label": "Package contents",
      "description": "Lists the exact Cargo source package contents.",
      "toolId": "cargo",
      "args": ["package", "--allow-dirty", "--list", "--locked"],
      "requiresTools": [],
      "timeoutSeconds": 300
    },
    {
      "id": "rust.package",
      "label": "Package verification",
      "description": "Assembles and verifies the source package without publishing.",
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
      "description": "Produces bounded source, lint, test, documentation, and package evidence.",
      "workingDirectory": ".",
      "checks": [
        { "checkId": "rust.format", "dependsOn": [] },
        { "checkId": "rust.check", "dependsOn": ["rust.format"] },
        { "checkId": "rust.clippy", "dependsOn": ["rust.check"] },
        { "checkId": "rust.test", "dependsOn": ["rust.clippy"] },
        { "checkId": "rust.doctest", "dependsOn": ["rust.test"] },
        { "checkId": "rust.package-list", "dependsOn": ["rust.doctest"] },
        { "checkId": "rust.package", "dependsOn": ["rust.package-list"] }
      ]
    }
  ],
  "groups": [
    {
      "id": "release",
      "label": "Release gate",
      "description": "Runs the bounded Rust CLI release example.",
      "members": [{ "kind": "suite", "id": "rust.release" }]
    }
  ]
}
```

## Expected Evidence

A successful run reports formatting, selected compilation and lint status,
behavioral and documentation-test results, exact package paths, package build
verification, and Git-visible integrity relative to the initial state. The
separate MSRV step reports compatibility with the declared minimum toolchain.

## Failure Interpretation

Treat format, compilation, lint, test, documentation, package-content, package
build, and repository-integrity failures as distinct observations. A package
failure can expose omitted files or build-script assumptions that checkout tests
cannot. An MSRV failure means the declared minimum promise and selected code or
dependencies disagree.

## Limitations And Residual Risk

The example omits cross-platform execution, non-default feature combinations,
installer smoke tests, dependency vulnerability and license analysis,
performance, signing, publication, and bit-for-bit binary reproducibility.
Offline verification requires populated Cargo inputs. Build scripts and tests
can mutate state outside Git visibility. Add evidence only when product promises
or observed failures make those gaps material.
