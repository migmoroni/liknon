# Phase B02.0: Confirm The Knowledge Product And Command Contract

[Back to the knowledge CLI and corpus plan](README.md)

## Purpose

Resolve every public identity, command, corpus, platform, and installation
decision before implementation or publication.

## Required Decisions

- Confirm package, binary, alias, tag, Cloudflare-prefix, and stable-channel
  identities.
- Confirm `catalog` and `show` arguments, formats, streams, exit statuses, and
  diagnostics.
- Confirm that `show` emits exact canonical bytes and that catalog ordering is
  deterministic.
- Confirm the complete initial corpus and which authoring or maintainer files
  remain excluded.
- Confirm color and presentation behavior without coupling to validation help.
- Confirm supported source and native platforms.
- Confirm independent versioning from `liknon-cli`.
- Confirm crates.io ownership and Cloudflare publication authority.

## Hard Boundaries

- No `liknon knowledge` command or implicit delegation from `liknon`.
- No validation configuration discovery, process execution, or report
  generation.
- No package-manager discovery or workspace activation record.
- No skills, prompts, AI policy, or process instructions in the corpus binary.
- No network access during catalog or document retrieval.
- No hidden installation of the validation CLI.

## Prototype Matrix

Prototype:

- both aliases and both commands;
- human and JSON catalog output;
- exact Markdown retrieval for each editorial family;
- unknown and malformed document IDs;
- narrow and low-vision help and output;
- source installation, crates.io package assembly, and one native archive;
- operation with no `.validation/` directory and no `liknon` on `PATH`.

## Exit Criterion

The knowledge product has one unambiguous identity, complete corpus boundary,
independent commands, independent release train, and feasible publication
model through all three approved channels.
