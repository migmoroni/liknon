# Schema Reference

Configuration and report schemas are owned and distributed by the tool. Inspect
the schema that matches the running binary without keeping copied files:

```sh
liknon-validate schema config
liknon-validate schema report
```

Redirect these commands only when an editor or repository explicitly needs a
checked-in copy. Configuration schema version 6 and report schema version 4 are
independent of the crate version. Unsupported versions are rejected; the tool
does not infer, convert, or maintain compatibility with replaced contracts.

The validation package contains canonical copies at
`apps/liknon-validate/schemas/config.schema.json` and
`apps/liknon-validate/schemas/report.schema.json`, verified against the
Rust contract types.

The repository additionally owns Draft 2020-12 schemas for the shared
validation-knowledge catalog, source register, and maintainer-only editorial
profile registry at
`docs/validation/authoring/schemas/knowledge-catalog.schema.json`,
`docs/validation/authoring/schemas/knowledge-sources.schema.json`, and
`docs/validation/authoring/schemas/knowledge-editorial-profiles.schema.json`.
The routing fixture,
forward-trial registry, and evaluation rubric are separately described by
`evals/schemas/knowledge-routing.schema.json`,
`evals/schemas/knowledge-forward-trials.schema.json`, and
`evals/schemas/knowledge-rubric.schema.json`.

The knowledge release task compiles the catalog and source-register schemas
with format assertions, then validates recipe examples through the ordinary
configuration schema and loader. Editorial profiles, routing fixtures, forward
trials, and rubrics support authoring or evaluation workflows; they are not
release-blocking knowledge checks and are not runtime CLI contracts.

A catalog document with `status: "draft"` requires only its stable identity,
kind, path, and status. Metadata may be added incrementally while it is being
authored. A document with `status: "reviewed"` requires the complete catalog
metadata declared by the schema. Recipe examples may use any relative
`workspaceRoot` that remains inside the isolated release-check sandbox; the
ordinary configuration loader remains the authority for its final validation.
