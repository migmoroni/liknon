# Initialization Reference

`workspace-validator init` provisions consumer-owned resources. It accepts the
capability flag `--config <candidate-path>` and requires at least one
capability flag. `--workspace <path>` selects the workspace; when omitted, the
workspace is exactly the process current directory. Initialization never walks
to a parent configuration or Git root.

The candidate must be a contained regular file. Symlinks, parent traversal,
workspace escape, and non-files are rejected. Its complete bytes are validated
as though their location were `<workspace>/.validation/config.json`, so
`workspaceRoot`, suite directories, graph references, and every other semantic
rule have their installed meaning before any write occurs.
This includes configurations whose `workspaceRoot` is the canonical
`.validation` directory itself: current-directory components, an empty path,
and paths that leave and return to that future directory resolve exactly as
they do after installation. Required descendants that publication does not
create remain invalid.

Successful publication is atomic and never overwrites a destination. Exact
existing bytes produce `reused`; different bytes or an unsafe destination
produce `conflict`. Initialization does not run preflight, checks, installation,
repair, or workspace discovery.
Temporary files use exclusive creation and collision-tolerant names. An
artifact abandoned by an interrupted process is never reused, truncated, or
removed by a later attempt and cannot block publication through another name.

`--format=json` emits exactly one version 1 `InitResult`. It contains the
canonical workspace, aggregate status, and one ordered resource entry with its
kind, canonical destination, status, SHA-256 digest when readable, and bounded
diagnostics. Resource statuses are `created`, `reused`, `conflict`, `failed`,
and `not_written`; aggregate statuses are `success`, `conflict`, `failed`, and
`partial`. The shared result keeps provisioning capabilities composable within
one initialization namespace.
