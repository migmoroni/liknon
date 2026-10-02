# Validation Knowledge Authoring

This directory contains maintainer contracts. It is packaged for contributors
but is not embedded as shared validation knowledge and cannot be selected with
`workspace-validator knowledge show`.

## Authoring Workflow

1. State the reader's primary question and select the profile that serves that
   intent.
2. Confirm applicability facets, evidence dimensions, version scope, and
   authoritative sources.
3. Author or revise the smallest sufficient guide under the profile's canonical
   path prefix.
4. Update `knowledge/catalog.json`, `knowledge/sources.json`, relationships, and
   navigation indexes.
5. Run `cargo run --locked --example knowledge_release -- --write`, then run it
   again with `--check` to verify contracts, references, files, recipes,
   generated assets, and deterministic packaging.
6. Review the rendered guide, stable-ID routing, operational effects, source
   support, and duplication with the [review rubric](review-rubric.md).

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

Do not create a new family or profile just to classify a subject. Languages,
tools, frameworks, technologies, evidence dimensions, and workspace facts are
applicability facets unless they define the reader purpose represented by an
existing profile.
