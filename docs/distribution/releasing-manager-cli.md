# Releasing Manager CLI

This runbook prepares and publishes one exact Manager release identity. The
first public execution is coordinated by B06; candidate preparation itself is
owned by B02 and does not publish Resource Hub content.

## Preconditions

1. `distribution/cli/manager/product.json` passes descriptor and package
   inspection.
2. `liknon-resource-contracts` and `liknon-manager` pass formatting, Clippy,
   tests, doctests, schema checks, package inventory checks, and MSRV 1.87.
3. The supporting `liknon-resource-contracts` version has been published and
   read back byte-for-byte by
   `.github/workflows/release-resource-contracts.yml`.
4. Every target-native Manager payload and semantic smoke test passes.
5. The detached-manifest public key is committed from the protected key setup;
   the private key remains only in the Manager signing environment.
6. Manager-specific crates.io, GitHub, Cloudflare, signing, Windows-signing,
   and approval environments are configured as described by the B02 accounts
   runbook.

## Candidate Identity

The package version in `apps/liknon-manager/Cargo.toml` is the sole version
authority. Create the signed annotated tag `liknon-manager-v<VERSION>` at the
exact release commit. The workflow rejects a lightweight, unsigned, mismatched,
or noncanonical tag.

The retained identity binds product `manager`, package `liknon-manager`,
version, tag, full commit, committed lockfile, source-package digest,
supporting-package closure, command targets, native payload hashes, and target
authenticity evidence.

## Workflow

Run `.github/workflows/release-manager.yml` with the exact tag. Its
credential-free jobs inspect source packages, build target-native candidates,
assemble direct archives and installers, exercise both command names, and
retain provenance. Publication jobs use isolated protected environments.

Publication order is:

1. supporting crates.io packages through their own retained-candidate workflow;
2. `liknon-manager` crates.io package;
3. metadata-only GitHub Release;
4. immutable Manager Cloudflare objects;
5. public readback and installer verification;
6. Manager stable-pointer advancement.

The workflow is idempotent only when an existing public version or object is
byte-identical to the retained candidate. Immutable bytes are never replaced.

## Recovery

A failed publication resumes from retained candidates and evidence. Do not
rebuild under the same version. If public verification fails, preserve the
prior stable pointer, diagnose the isolated destination, and retry only the
failed publication/readback stage with the same hashes. Manager release
credentials cannot publish Hub catalogs, Validation artifacts, Tool artifacts,
or a complete-suite pointer.
