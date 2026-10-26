# Initialization Reference

`liknon-validate init` provisions consumer-owned validation resources. It accepts the
capability flag `--config <candidate-path>` and requires at least one
capability flag. `--workspace <path>` selects the workspace; when omitted, the
workspace is exactly the process current directory. Initialization never walks
to a parent configuration or Git root.

The candidate must be a contained regular file. Symlinks, parent traversal,
workspace escape, and non-files are rejected. Its complete bytes are first
parsed as the closed configuration contract. Malformed JSON and unknown fields
are rejected without creating `.liknon/`.

After parsing, `init` prepares `.liknon/` and `.liknon/validation/` in order,
using non-recursive directory creation at each level and rejecting a file,
symlink, or other unsafe entry at either path. The validation directory is
still empty when the candidate is validated as
`<workspace>/.liknon/validation/config.json` through the ordinary
filesystem-aware configuration path. Consequently `workspaceRoot`, suite
directories, symlinks, parent traversal, graph references, and operating-system
path errors have exactly their installed meaning. `config.json` and atomic
temporary files are created only after complete validation succeeds.

If validation or publication fails after the current invocation created either
directory, cleanup attempts only `remove_dir`, from the validation directory to
the namespace. It can remove a directory only while it remains empty. Every
pre-existing real directory and unrelated entry is preserved.

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

Initialization is capability-selective: every consumer-owned resource requires
its own explicit flag, and `--config` materializes only the validation
configuration. It does not create resource, policy, persistence, report, or AI
state merely because those concerns may later share the `.liknon/` namespace.
