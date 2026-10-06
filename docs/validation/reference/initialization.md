# Initialization Reference

`liknon init` provisions consumer-owned resources. It accepts the
capability flag `--config <candidate-path>` and requires at least one
capability flag. `--workspace <path>` selects the workspace; when omitted, the
workspace is exactly the process current directory. Initialization never walks
to a parent configuration or Git root.

The candidate must be a contained regular file. Symlinks, parent traversal,
workspace escape, and non-files are rejected. Its complete bytes are first
parsed as the closed configuration contract. Malformed JSON and unknown fields
are rejected without creating `.validation/`.

After parsing, `init` creates the canonical `.validation/` directory when it is
absent, using a single-directory operation and rejecting a file, symlink, or
other unsafe entry at that path. The directory is still empty when the candidate
is validated as `<workspace>/.validation/config.json` through the ordinary
filesystem-aware configuration path. Consequently `workspaceRoot`, suite
directories, symlinks, parent traversal, graph references, and operating-system
path errors have exactly their installed meaning. `config.json` and atomic
temporary files are created only after complete validation succeeds.

If validation or publication fails after the current invocation created the
directory, cleanup attempts only `remove_dir`; it can remove the directory only
while it remains empty. A pre-existing real directory and every unrelated entry
inside it are preserved.

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
its own explicit flag, and a flag materializes only that resource. Adding
another capability does not make `--config` install it implicitly. Shared
validation knowledge is versioned inside the CLI and is never copied into
`.validation/` by initialization.
