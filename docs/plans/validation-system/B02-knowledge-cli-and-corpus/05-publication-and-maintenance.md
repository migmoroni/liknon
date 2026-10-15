# Phase B02.5: Publish And Maintain Liknon Knowledge

[Back to the knowledge CLI and corpus plan](README.md)

## Purpose

Publish verified knowledge candidates through isolated identities, verify
public readback, and permit corpus updates independently from validation CLI
releases.

## Publication Sequence

1. Verify the signed `liknon-knowledge-v<VERSION>` tag and retained candidates.
2. Obtain credential-free human approval.
3. Publish the inspected source package to crates.io.
4. Read the accepted crate back, rebuild it, and compare its corpus inventory.
5. Create the metadata-only immutable GitHub Release.
6. Publish immutable native objects under the knowledge-specific Cloudflare
   prefix.
7. Read every object back through the production domain.
8. Publish the signed version manifest only after all referenced objects pass.
9. Run public source, crates, manual-archive, and installer checks.
10. Update the knowledge stable pointer only after complete success.

Each registry and storage job receives only its own credential. Build,
assembly, verification, and human approval receive none.

## Recovery

A retry reauthenticates retained bytes and skips a destination only after
public readback proves equality. It never rebuilds, resigns, replaces an
accepted crate version, or overwrites an immutable Cloudflare object. If exact
recovery is impossible, release a new patch version.

## Maintenance

- Review time-sensitive source metadata and links on their declared schedule.
- A document change, catalog change, or retrieval-contract change advances the
  knowledge application version.
- Knowledge releases do not require validation CLI or ecosystem package
  releases unless their explicit compatibility contracts change.
- Removed document IDs follow an approved current API decision; no fallback
  reader or silent alias is added.
- Public documentation distinguishes source, crates.io, and native
  installation and explains both command names.

## Exit Criterion

The public knowledge application is reproducible through every approved
channel, public bytes match retained candidates, and future corpus maintenance
has an independent, recoverable release path.
