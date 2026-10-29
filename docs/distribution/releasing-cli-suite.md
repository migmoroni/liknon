# Preparing The CLI Suite Candidate

Plan B04 prepares one retained, verified CLI-suite candidate. It does not
publish to crates.io, create a public GitHub Release, upload to Cloudflare R2,
or advance a stable pointer. Plan B06 later promotes the exact retained bytes
without rebuilding them.

## Canonical Inputs

- package and version authority: `apps/liknon-cli/Cargo.toml`;
- distribution descriptor: `distribution/cli/suite/product.json`;
- candidate workflow: `.github/workflows/candidate-cli-suite.yml`;
- reusable native build: `.github/workflows/_cli-native-candidate.yml`;
- reusable direct assembly: `.github/workflows/_cli-direct-assembly.yml`;
- public-readback workflow reserved for B06:
  `.github/workflows/_cli-public-verification.yml`.

The source tag has the form `liknon-cli-v<VERSION>` and must be annotated,
signed, and point at the exact candidate commit.

## Authority Checkpoints

Before dispatching the protected workflow, provision:

1. exactly one approved tag trust root in
   `distribution/cli/suite/direct/`;
2. `distribution/cli/suite/direct/public-key.pem`, matching the detached
   metadata signing key held by the protected signing environment;
3. the Windows signing identity, pinned fingerprint, and timestamp origin;
4. `LIKNON_DOWNLOAD_BASE_URL` for deterministic installer rendering;
5. the protected environments named by the suite descriptor.

Private keys, certificates, and publication credentials never enter the
repository or build jobs. The detailed authority model is maintained in
[`../plans/validation-system/B04-unified-launcher-and-cli-suite/accounts-and-permissions.md`](../plans/validation-system/B04-unified-launcher-and-cli-suite/accounts-and-permissions.md).

## Candidate Workflow

Dispatch **Prepare CLI suite candidate** with the exact signed tag. The
workflow:

1. authenticates the tag and derives one suite identity from Cargo metadata,
   the commit, and `Cargo.lock`;
2. runs formatting, Clippy, tests, doctests, and intentional outcome fixtures;
3. packages the complete publishable dependency closure in isolation;
4. installs the packaged `liknon-cli` source and verifies exactly four command
   files with no compact executable aliases;
5. builds all four executables for every descriptor target;
6. applies and verifies target-specific platform authenticity;
7. assembles one archive per target and one signed direct manifest;
8. tests direct and launcher-routed execution, target selection, installation,
   failed-update rollback, and uninstall;
9. attests exact source and native subjects;
10. retains a `cli-suite-b06-handoff` record identifying the verified candidate
    workflow run.

The retained source contract, native candidates, direct candidate, build
evidence, attestations, and handoff record all refer to the same package,
version, tag, commit, target matrix, and executable inventory.

## Failure And Handoff

A failed job does not publish or promote anything. Fix the owning source,
create a new signed candidate identity when bytes change, and rerun the affected
contract. Do not patch, rename, or recombine an accepted artifact.

B06 may consume a candidate only after all candidate jobs pass and the exact
artifacts remain available for its freeze and rehearsal. B06 owns human release
approval, destination credentials, immutable publication, public readback, and
stable-pointer promotion.
