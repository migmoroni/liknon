# Future Linux x86-64 Build Scope

## Status

This scope reduction is deferred and is not active. The current cross-platform
CI remains intentional while Liknon is under active development because it
provides early evidence that process handling, paths, terminal behavior, and
packaging remain portable.

Apply this document only after an explicit maintainer decision to reduce hosted
runner cost. The change must be made as one reviewed unit so that release
artifacts, CI coverage, public claims, and required credentials remain aligned.

## Intended Result

After activation, native releases and routine CI cover only Linux x86-64.
Native distribution retains two separate artifacts:

| Rust target | Purpose |
| --- | --- |
| `x86_64-unknown-linux-gnu` | Default artifact for compatible GNU systems |
| `x86_64-unknown-linux-musl` | Static MUSL artifact for compatible environments |

Linux ARM64, macOS, and Windows produce no release candidate and start no
hosted runner. GNU remains preferred when both GNU and MUSL are detected.

Restricting the active build matrix does not require deleting portable code.
Platform models, installers, signing support, process implementations, and
platform-neutral unit tests remain available for a later expansion. Platforms
without active native and CI evidence must not be presented as currently
supported.

## Activation Preconditions

Before reducing the matrices:

1. Require an explicit maintainer approval for the scope change.
2. Record one green baseline run of the existing Linux, macOS, and Windows CI.
3. Confirm that both Linux x86-64 release targets build and pass installation,
   alias-equivalence, mutation-rejection, and uninstall checks.
4. Review branch protection so removed jobs are no longer required checks.
5. Review GitHub Actions budgets and runner policies after the new matrix is
   known.

## Required Changes

### 1. Make The Product Descriptor Authoritative

In `distribution/cli/validation/product.json`, retain only:

```text
x86_64-unknown-linux-gnu
x86_64-unknown-linux-musl
```

Remove the active target entries for Linux ARM64, macOS, and Windows. Do not add
an independent allowlist to a workflow: the reviewed descriptor remains the
source of truth for release targets, manifests, archives, and public
verification.

### 2. Make Optional Release Jobs Follow The Descriptor

Update `.github/workflows/release-validation.yml` so removing a target from the
descriptor does not leave a hard-coded job waiting for an artifact that will
never exist.

- Derive the native and public verification matrices from `product.json`, as
  today.
- Derive whether a Windows target is active and expose that result from the
  `contract` job.
- Run `sign-windows` and `verify-windows-platform` only when the descriptor
  contains a Windows target.
- Allow `assemble-direct` to proceed when the Windows signing job is skipped,
  while still failing if an active signing job fails.
- Derive the additional POSIX verification matrix from active descriptor
  targets instead of hard-coding ARM64 and macOS runners.
- Exercise `x86_64-unknown-linux-musl` with `--libc musl`; the primary GNU
  installer check continues to exercise `x86_64-unknown-linux-gnu`.
- Allow `attest` to proceed after intentionally skipped platform jobs, while
  still requiring every active verification job to succeed.
- Keep the reusable Windows and macOS implementation intact. Re-adding a
  reviewed target to the descriptor should be sufficient to reactivate the
  corresponding release path after its credentials and runner policy are
  restored.

The common reusable workflows must continue to reject a missing artifact for
any target that remains in the descriptor.

### 3. Reduce Routine CI To Linux

In `.github/workflows/liknon.yml`:

- change the `platforms` matrix to `ubuntu-latest` only;
- remove the MSRV step that installs `x86_64-pc-windows-gnu`;
- remove the MSRV Windows cross-target check;
- retain formatting, Clippy, Rustdoc, tests, MSRV, dependency policy, package
  boundary, local installation, fixture, and knowledge checks on Linux;
- keep platform-specific unit tests that can run on Linux and validate generic
  contracts without pretending to be native runtime evidence.

Review any other workflow added after this document was written. No enabled
matrix, runner, or cross-compilation step should select Windows, macOS, or
Linux ARM64 after activation.

### 4. Align Public Documentation And Security Claims

Update current documentation in the same change:

- `README.md`;
- `docs/distribution/validation-cli.md`;
- `docs/distribution/releasing-validation-cli.md`;
- `SECURITY.md`;
- the active B01 distribution contract and readiness runbook.

The supported-target table must contain only the two Linux x86-64 targets.
Remove active Windows and macOS installation, signing, notarization, and
credential prerequisites. Historical changelog entries remain historical and
must not be rewritten.

Do not describe an untested platform as supported merely because its code is
still present. A source build attempted elsewhere is outside the maintained
platform contract until native CI and release verification are restored.

### 5. Preserve Focused Contract Coverage

Add or adjust tests to prove that:

- the Validation product descriptor has exactly the two active Linux x86-64
  targets;
- direct assembly requires exactly one final artifact for each active target;
- installer auto-detection prefers GNU when compatible GNU and MUSL options
  coexist;
- `--libc musl` selects and verifies the MUSL archive;
- no active release matrix schedules Windows, macOS, or ARM64;
- skipped optional platform jobs cannot conceal a failure from an active job.

Keep generic tests for Windows/macOS target validation, authenticity evidence,
installer generation, and process behavior when they remain meaningful on
Linux. Their presence preserves implementation quality without making a public
support claim.

### 6. Update Repository Settings

After merging the code change:

1. Remove obsolete macOS and Windows job names from required status checks.
2. Disable or archive Windows-signing environments and secrets only after no
   active workflow references them.
3. Retain recovery copies of signing material according to the security
   policy; reducing CI scope is not permission to destroy release identities.
4. Confirm that a pull request starts no non-Linux hosted runner.
5. Confirm that a release dispatch creates only the GNU and MUSL x86-64 native
   candidates.

## Verification Gate

The activation change is complete only when all of the following are true:

```sh
jq -r '.targets[].rustTarget' distribution/cli/validation/product.json
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --all-targets --locked
cargo test --workspace --doc --locked
```

The descriptor query must print exactly:

```text
x86_64-unknown-linux-gnu
x86_64-unknown-linux-musl
```

Run the release workflow through candidate assembly and verification before
approving publication. Inspect the Actions run to confirm that only Linux x64
jobs consumed hosted runners and that both retained archives appear in the
signed manifest.

## Restoring A Platform

Platform restoration is a new support decision, not an automatic rollback.
Restore the descriptor row, native runner coverage, credentials or signing
mode, public documentation, and required checks together. A platform becomes
supported again only after its native candidate and public installation pass
the complete release verification path.
