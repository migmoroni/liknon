# Git

## Purpose And Scope

Git provides stable observations of tracked, staged, and untracked work-tree
state around validation. This reference covers status output used for mutation
evidence.

## Version Lines And Sources

Porcelain version 1 is stable across maintained Git lines. Confirm the active
tool with `git --version` before depending on options not present in older
consumer environments. The living `git-status` documentation is the reviewed
source.

## Command Model

Use a NUL-delimited porcelain observation when path bytes must remain
unambiguous:

```text
git status --porcelain=v1 -z --untracked-files=all
```

The porcelain format is designed for scripts and remains stable across Git
versions. [Source: git.status#porcelain](https://git-scm.com/docs/git-status#_porcelain_format_version_1)

## Inputs Outputs And Exit Status

Inputs are repository metadata, index, work tree, ignore rules, and status
options. Output encodes entry state and paths; successful observation returns
zero. A zero exit does not mean the work tree is clean—the output is the oracle.

## Mutation Cost And External Services

Status is local and normally non-mutating, but it reads the work tree and may be
costly in large repositories. It needs no network or remote service. Other Git
commands may mutate state and are not implied by this reference.

## Evidence Produced

Before-and-after observations can identify Git-visible changes attributable to
a run when concurrent repository activity is excluded.

## Blind Spots

Ignored files, caches, external paths, databases, remote services, process
state, and content restored between observations may remain invisible. Git is
not a sandbox and cannot prove that a command was read-only.

## Related Knowledge

Use [Git Repository Evidence](../../explanations/technologies/git.md) for the
reasoning boundary and [Isolation And Determinism](../../explanations/patterns/isolation-and-determinism.md)
for non-Git state.
