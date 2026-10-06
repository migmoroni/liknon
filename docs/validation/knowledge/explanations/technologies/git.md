# Git Repository Evidence

## Role And Scope

Git can observe changes to tracked files, the index, and untracked paths around
a validation run. It supplies repository-state evidence; it is not a sandbox or
a complete filesystem monitor.

## Failure Modes

A check may rewrite source, update a lockfile, generate a new tracked or
untracked artifact, alter the index, or leave a partial file. It may also mutate
ignored files, caches, data stores, external services, or paths outside the work
tree that Git does not expose.

## Validation Surfaces

Capture a stable porcelain status before and after the run, preserve path bytes,
and compare the two observations. Git porcelain version 1 is intended for
script-oriented stable status output.
[Source: git.status#porcelain](https://git-scm.com/docs/git-status#_porcelain_format_version_1)

Supplement status with artifact checksums or isolated temporary roots when
ignored or external effects matter.

## Evidence And Limitations

An unchanged status supports only the claim that Git-visible state did not
change between observations. It cannot prove that a command was read-only,
that ignored files are clean, or that remote state, databases, environment, and
processes are unchanged. Concurrent repository activity weakens attribution.

## Version Scope

The described porcelain contract is stable across maintained Git lines. Record
the active version and confirm behavior before depending on newer options; an
exact reviewed documentation snapshot is provenance rather than reader scope.

## Related Knowledge

Use the [Git](../../reference/tools/git.md) reference for invocation and
[Isolation And Determinism](../patterns/isolation-and-determinism.md) for a
broader mutation boundary.

## Next Step

Define which state must remain unchanged, then combine Git status with direct
observations for every material state Git cannot see.
