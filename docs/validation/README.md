# Validation Documentation

Use the [Human Flow](flows/human/README.md) to operate the validator, the
[reference](reference/README.md) for stable CLI and contract details, and the
[shared knowledge](knowledge/README.md) to reason about risks, evidence, and
validation choices.

In this source repository, people and agents may read the canonical knowledge
tree directly. An installed CLI exposes the same versioned content through
`workspace-validator knowledge catalog` and `workspace-validator knowledge
show <document-id>` without requiring workspace initialization.

Catalog status is part of the consumption contract. A `draft` identifies
work in progress and may omit routing metadata; consumers must not invent the
missing values. A `reviewed` entry provides the complete metadata required by
the catalog schema.

These areas have different responsibilities. Reference pages describe what the
product does. Flow pages describe how a person uses it. Shared knowledge helps
people and agents decide what evidence is relevant; it does not activate a flow,
grant authority, or define workspace policy.

Contributors who maintain the shared corpus use the separate
[authoring contracts](authoring/README.md). Those contracts are packaged for
maintainers but are not embedded knowledge and are never returned by
`knowledge show`.
