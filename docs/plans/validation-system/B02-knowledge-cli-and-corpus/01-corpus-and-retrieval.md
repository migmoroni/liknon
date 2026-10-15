# Phase B02.1: Build The Complete Corpus And Retrieval Contract

[Back to the knowledge CLI and corpus plan](README.md)

## Purpose

Turn the canonical authored knowledge into one deterministic read-only
application payload without creating a second authored copy.

## Canonical Source

`docs/validation/knowledge` remains the authored source. The build consumes its
validated catalog, source register, editorial metadata, and canonical Markdown
documents. Generated runtime indexes are derived artifacts.

The catalog records stable identity, kind, title, summary, path, status,
applicability facets, and relationships sufficient for progressive discovery.
It does not expose maintainer-only filesystem details as a public selector.

## Build Contract

- Parse and validate the corpus through the same editorial tooling used by
  release gates.
- Sort all generated indexes deterministically.
- Record byte length and SHA-256 for every public document.
- Reject duplicate IDs, missing bytes, unsafe paths, unresolved local links,
  invalid status metadata, and catalog/document disagreement.
- Embed or package only the approved runtime inventory.
- Rebuild from the same commit to byte-identical catalog and document payloads.

## Retrieval Contract

- `catalog --format=human` is readable and bounded.
- `catalog --format=json` is versioned, deterministic, and unstyled.
- `show <DOCUMENT_ID>` emits exact canonical Markdown to standard output.
- Diagnostics use standard error and stable exit statuses.
- Unknown IDs suggest catalog discovery without guessing another document.
- Retrieval remains offline and independent of current directory, workspace,
  package manager, and validation configuration.

## Verification

- Every public catalog entry resolves exactly once.
- Every returned document digest equals its canonical source digest.
- Draft metadata remains honest and does not fabricate missing sections.
- Reviewed documents satisfy their complete editorial profile.
- Authoring guidance, plans, fixtures, and tools are absent from the payload.
- Golden fixtures cover ordering, formats, unknown IDs, exact bytes, and
  corpus-integrity failures.

## Exit Criterion

One deterministic complete corpus can be queried by stable ID without network,
workspace, validation engine, or duplicate authored content.
