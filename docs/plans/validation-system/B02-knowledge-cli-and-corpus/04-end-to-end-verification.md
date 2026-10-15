# Phase B02.4: Verify Knowledge Distribution End To End

[Back to the knowledge CLI and corpus plan](README.md)

## Purpose

Prove before publication that every channel exposes the same command,
version, catalog, and document bytes.

## Verification Matrix

For GitHub source, packaged crates.io source, and every native target:

- install in an isolated environment;
- ensure neither command resolves to another installation on `PATH`;
- run both aliases and compare versions;
- compare human and JSON catalog semantics;
- retrieve representative documents from every editorial family;
- compare the full ordered ID and digest inventory;
- exercise unknown IDs, invalid formats, broken output streams, and help;
- prove operation without `liknon`, `.validation/`, or network access;
- update from an older fixture release and uninstall cleanly.

## Security And Artifact Checks

- Authenticate source tags, package checksums, native manifests, archives, and
  extracted executables.
- Reject modified catalog, document, archive, manifest, installer, or binary
  bytes.
- Verify Windows and macOS trust claims exactly as implemented.
- Verify Linux GNU/MUSL selection on each architecture.
- Prove generated state remains ignored and tracked files unchanged.
- Prove no verification job can access publication credentials.

## Evidence Contract

Retain machine-readable evidence with version, commit, channel, target, command
alias, catalog digest, corpus inventory digest, artifact hashes, and result.
Evidence contains no credential or host-secret path.

## Exit Criterion

Source, crates.io, and Cloudflare candidates are behaviorally and
content-equivalent, independently installable, and completely verified before
any publication credential becomes available.
