# Git

## Identity And Reviewed Scope

- Executable/package: `git` from Git SCM.
- Reviewed command: Git 2.x `status --porcelain=v1` behavior.
- Evidence class: Git-visible work-tree/index state before and after validation.

## Useful Command Structure

```text
git status --short
git status --porcelain=v1 --untracked-files=all
```

Porcelain v1 is designed as a stable scripting format and fixes paths relative
to the repository root unless `-z` rules apply.
[Source: git.status#porcelain](https://git-scm.com/docs/git-status#_porcelain_format_version_1)

## Output, Exit, Cost, And Mutation

Successful status inspection exits zero and is read-only. Output classifies
index/work-tree changes and untracked files according to options and ignore
rules. Large repositories or untracked trees can make inspection expensive.

## Blind Spots And Overlap

Ignored/out-of-tree files, permissions not tracked by Git, external services,
databases, and running processes remain invisible. Status text alone may not
detect content transitions within the same status class; use a bounded content
fingerprint where that matters. Never erase pre-existing changes to create a
clean baseline.

## Next Step

Use [Git repository evidence](../technologies/git.md) to interpret the result
and combine it with direct evidence for the product claim.
