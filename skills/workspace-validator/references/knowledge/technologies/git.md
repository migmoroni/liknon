# Git Repository Evidence

## Question And Applicability

How can Git-visible state help detect unintended validation mutations? Apply
this guidance only when the workspace is inside the intended Git work tree and
the relevant files are not intentionally invisible to Git.

## Evidence

Capture the initial status, execute the bounded validation, then compare the
final status or a stronger fingerprint. `git status --porcelain` provides a
stable script-oriented format across Git versions, subject to the documented
version rules.
[Source: git.status#porcelain](https://git-scm.com/docs/git-status#_porcelain_format_version_1)

This evidence can reveal tracked changes and selected untracked files. A
content fingerprint can additionally distinguish files whose status class is
unchanged.

## Limitations

Git status does not prove filesystem purity: ignored files, files outside the
work tree, metadata, remote services, databases, and process state can change
without appearing. Pre-existing changes must be preserved and separated from
new mutations. A clean repository says nothing about behavioral correctness.

## Cost, Mutation, And Next Step

Status inspection is read-only and usually cheap; hashing large trees costs
more. Confirm repository boundaries and ignore rules, then pair repository
evidence with the mechanism that observes the actual product claim. Use the
[Git tool guide](../tools/git.md) for command identity and the
[reproducible-release concern](../concerns/reproducible-release-evidence.md) for
release composition.
