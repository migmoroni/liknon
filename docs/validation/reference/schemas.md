# Schema Reference

Configuration and report schemas are owned and distributed by the tool. Inspect
the schema that matches the running binary without keeping copied files:

```sh
workspace-validator schema config
workspace-validator schema report
```

Redirect these commands only when an editor or repository explicitly needs a
checked-in copy. Configuration schema version 6 and report schema version 4 are
independent of the crate version. Unsupported versions are rejected; the tool
does not infer, convert, or maintain compatibility with replaced contracts.

The source package also contains canonical copies at `schemas/config.schema.json`
and `schemas/report.schema.json`, verified against the Rust contract types.
