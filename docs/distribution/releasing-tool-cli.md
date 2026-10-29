# Releasing Tool CLI

This runbook prepares and publishes one exact Tool release identity. The first
public execution is coordinated by B06; candidate preparation itself is owned
by B03 and does not publish Resource Hub content.

## Preconditions

1. `distribution/cli/tool/product.json` passes descriptor and package
   inspection.
2. `liknon-cli-ui`, `liknon-resource-contracts`, and `liknon-tool` pass
   formatting, Clippy, tests, doctests, package inventory checks, semantic
   offline smoke tests, and MSRV 1.87.
3. The exact supporting-crate versions have been published and read back
   byte-for-byte through their own release workflows.
4. Every target-native Tool payload passes alias, offline retrieval,
   non-mutation, terminal, archive, and installer verification.
5. The detached-manifest public key is committed from the protected key setup;
   the private key remains only in the Tool signing environment.
6. Tool-specific crates.io, GitHub, Cloudflare, signing, Windows-signing, and
   approval environments are configured as described by the B03 accounts
   runbook.

## Candidate Identity

The package version in `apps/liknon-tool/Cargo.toml` is the sole version
authority. Create the signed annotated tag `liknon-tool-v<VERSION>` at the
exact release commit. The workflow rejects a lightweight, unsigned, mismatched,
or noncanonical tag.

The retained identity binds product `tool`, package `liknon-tool`, version,
tag, full commit, committed lockfile, source-package digest,
supporting-package closure, command targets, native payload hashes, and target
authenticity evidence.

## Workflow

Run `.github/workflows/release-tool.yml` with the exact tag. Its
credential-free jobs inspect source packages, build target-native candidates,
assemble direct archives and installers, exercise both command names against a
contract-valid offline fixture, and retain provenance. Publication jobs use
isolated protected environments.

Publication order is:

1. supporting crates.io packages through their own retained-candidate workflows;
2. `liknon-tool` crates.io package;
3. metadata-only GitHub Release;
4. immutable Tool Cloudflare objects;
5. public readback and installer verification;
6. Tool stable-pointer advancement.

The workflow is idempotent only when an existing public version or object is
byte-identical to the retained candidate. Immutable bytes are never replaced.

## Recovery

A failed publication resumes from retained candidates and evidence. Do not
rebuild under the same version. If public verification fails, preserve the
prior stable pointer, diagnose the isolated destination, and retry only the
failed publication/readback stage with the same hashes. Tool release
credentials cannot publish Hub catalogs, Validation artifacts, Manager
artifacts, or a complete-suite pointer.
