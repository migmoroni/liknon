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

Successful publication is atomic and never overwrites a destination. Exact
existing bytes produce `reused`; different bytes or an unsafe destination
produce `conflict`. Initialization does not run preflight, checks, installation,
repair, or workspace discovery.

`--format=json` emits exactly one version 1 `InitResult`. It contains the
canonical workspace, aggregate status, and one ordered resource entry with its
kind, canonical destination, status, SHA-256 digest when readable, and bounded
diagnostics. Resource statuses are `created`, `reused`, `conflict`, `failed`,
and `not_written`; aggregate statuses are `success`, `conflict`, `failed`, and
`partial`. The shared result keeps provisioning capabilities composable within
one initialization namespace.
