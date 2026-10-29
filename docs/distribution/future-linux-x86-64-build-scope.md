# Future Linux x86-64 Build Scope

## Status

This cost-reduction profile is deferred and inactive. Cross-platform CI and the
complete suite target matrix remain intentional while portability and packaging
are still being established.

Apply this document only after an explicit maintainer decision. The descriptor,
candidate workflow, routine CI, public claims, branch protection, and required
credentials must change as one reviewed unit.

## Intended Result

Retain only these native suite targets:

```text
x86_64-unknown-linux-gnu
x86_64-unknown-linux-musl
```

Each target still produces one archive containing `liknon`,
`liknon-validator`, `liknon-manager`, and `liknon-tool`. GNU remains preferred
when both GNU and MUSL appear viable. Linux ARM64, macOS, and Windows start no
hosted native-build or routine-CI runner and are no longer advertised as
supported.

Portable implementation code, target models, installers, signing support, and
platform-neutral tests remain available for a later support expansion.

## Required Changes

1. In `distribution/cli/suite/product.json`, retain exactly the two Linux x64
   target rows.
2. Make optional Windows signing and verification in
   `.github/workflows/candidate-cli-suite.yml` conditional on the descriptor;
   intentionally skipped platform jobs must not block assembly, while a failed
   active job must block it.
3. Keep `_cli-native-candidate.yml` and the distribution tooling generic so a
   reviewed descriptor row can reactivate a platform later.
4. Reduce the platform matrix in `.github/workflows/liknon.yml` to Linux x64
   and remove Windows cross-target setup.
5. Update `README.md`, `docs/distribution/cli-suite.md`, `SECURITY.md`, B04, and
   B06 to claim only the active targets.
6. Remove obsolete required status checks and archive unused protected signing
   environments only after no enabled workflow references them.
7. Add contract coverage proving the descriptor, generated matrices, target
   selection, GNU preference, MUSL override, and archive inventory all match
   the reduced scope.

## Verification Gate

```sh
jq -r '.targets[].rustTarget' distribution/cli/suite/product.json
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --all-targets --locked
cargo test --workspace --doc --locked
```

The descriptor query must print exactly the two approved targets. A candidate
workflow rehearsal must schedule only those native targets and retain one
complete archive for each.

## Restoring A Platform

Platform restoration is a new support decision. Restore its descriptor row,
native runner evidence, credentials or authenticity mode, CI coverage, public
documentation, and required checks together. Code presence alone is not a
support claim.
