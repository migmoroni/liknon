# Phase B02.3: Build Source And Native Knowledge Distributions

[Back to the knowledge CLI and corpus plan](README.md)

## Purpose

Produce verified GitHub-source, crates.io-source, and Cloudflare-native
candidates without changing the approved corpus between channels.

## GitHub Source Candidate

- Install from an isolated clone at the exact signed tag.
- Use `cargo install --path apps/liknon-knowledge --locked`.
- Prove both command names, version, catalog, and representative document
  retrieval.

## crates.io Candidate

- Package `liknon-knowledge` from tracked sources only.
- Include both binary targets and the exact approved corpus.
- Depend on no unpublished path package.
- Inspect `cargo package --list` and the extracted crate.
- Build and install the packaged crate in an isolated Cargo home before
  publication.

## Native Candidates

Build one final application artifact for every supported target approved by
B02.0. Initial targets mirror B01 only where target-native execution evidence
exists:

- Linux x64 and arm64, each split into GNU and MUSL;
- Windows x64;
- macOS x64 and arm64.

Each archive contains both command names backed by byte-identical executable
content plus only declared metadata. Platform authenticity, archive formats,
Linux libc selection, immutable Cloudflare keys, and installer verification
follow B01's approved security mechanics under the independent
`liknon-knowledge` identity.

## Channel Isolation

- Build once per target; later jobs copy verified bytes.
- crates.io packages source and never reuses a native archive.
- Cloudflare receives native archives and never triggers source compilation.
- GitHub Releases carry metadata only and no native binary assets.
- Publication jobs do not rebuild or rewrite candidates.

## Verification

- All channels return identical catalog JSON and document bytes.
- Both aliases work after each installation form.
- Archives contain no validation CLI, agent resources, plans, or fixtures.
- The crates package contains no generated native archives.
- GNU and MUSL selection requests exactly one appropriate archive.
- Unsupported targets fail before download or installation.

## Exit Criterion

All three channel candidates are independently installable, share one exact
knowledge contract, and are ready for credential-free end-to-end verification.
