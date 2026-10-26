# Validation Knowledge Authoring

This directory contains maintainer contracts. It is not embedded in the
Validation CLI or part of its runtime package.

## Authoring Workflow

1. State the reader's primary question and select the profile that serves that
   intent.
2. Create or revise the canonical Markdown file and register its stable `id`,
   `kind`, `path`, and `status`. A work in progress may remain `draft` while its
   remaining catalog metadata is absent.
3. Author the smallest sufficient guide under the profile's canonical path
   prefix. Add applicability facets, evidence dimensions, relationships,
   version scope, and authoritative sources as they become established.
4. Update `knowledge/sources.json` and navigation indexes when the document
   requires them.
5. Run `cargo run -p liknon-knowledge-maintenance --locked -- --write` when the
   generated source index needs regeneration, then run it with `--check` to
   verify contracts, references, declared files, recipes, source inventory,
   and the deterministic tree digest.
6. Review the rendered guide, stable-ID routing, operational effects, source
   support, and duplication with the [review rubric](review-rubric.md). Complete
   every catalog metadata field before changing its status to `reviewed`.

Draft status relaxes editorial metadata completeness only. Stable identifiers,
safe paths, declared UTF-8 files, supplied references, generated artifacts, and
packaged assets remain technically valid. A document whose `kind` is `recipe`
must contain one complete configuration example accepted by the ordinary
configuration schema and loader, including while it is a draft.

Use the [editorial standard](editorial-standard.md), consult the
[editorial bases](editorial-bases.md), and follow the profile named in
[`editorial-profiles.json`](editorial-profiles.json). The JSON registry records
the authoring model for family mapping, path prefixes, and H2 order. The
Markdown profiles explain editorial intent. These editorial materials guide
review and are not release-blocking validation rules.

The [corpus refoundation review](corpus-review.md) records the completed
classification and substantive review. Future changes use the same
[review rubric](review-rubric.md) without treating that artifact as a runtime
catalog.

The preserved skill's non-active C02 boundary is recorded in the
[skill handoff](skill-handoff.md).

Do not create a new family or profile just to classify a subject. Languages,
tools, frameworks, technologies, evidence dimensions, and workspace facts are
applicability facets unless they define the reader purpose represented by an
existing profile.
