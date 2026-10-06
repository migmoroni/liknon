# Shared Validation Knowledge

This is the canonical, audience-neutral knowledge base for choosing and
interpreting validation evidence. Start with the compact [catalog](catalog.json)
and load only the documents that match the current question and confirmed
workspace facts.

1. Use [Explanations](explanations/README.md) to understand concepts, risks,
   patterns, languages, frameworks, and technologies.
2. Use [How-To Guides](how-to/README.md) for one bounded, adaptable procedure.
3. Use [Reference](reference/README.md) to look up tool behavior and external
   standards.

Follow a registered [source location](SOURCES.md) only when local guidance is
insufficient and external access is available and authorized.

The catalog's applicability fields are routing hints, not claims about a
workspace. Missing coverage is explicit: inspect authoritative material or
report the gap instead of inventing local guidance. Loading this tree never
selects a product flow, starts a process, executes a command, persists an
artifact, or authorizes network access.

Use `status` when interpreting catalog entries. A `draft` may contain only
`id`, `kind`, `path`, and `status`; absent summaries, questions, applicability,
evidence dimensions, relationships, sources, or review dates remain unknown.
A `reviewed` entry contains every catalog metadata field. Neither status
replaces contextual judgment about whether a document applies to a workspace.

`sources.json` is the canonical source register; `SOURCES.md` is generated.
In a source checkout, read this tree directly. Installed binaries embed the
same versioned assets and expose them without a repository checkout:

```sh
liknon knowledge catalog --format=json
liknon knowledge show <document-id>
```

The distributed operational skill uses these commands and contains no copied
knowledge tree.

Maintainer contracts live outside this embedded tree in
`docs/validation/authoring/`; ordinary knowledge consumers do not need them.
