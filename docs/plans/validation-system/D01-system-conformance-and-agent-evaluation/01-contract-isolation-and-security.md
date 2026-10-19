# Phase D01.1: Verify Contract Isolation And Security

## Status

`Provisional`. This phase consumes integrated candidates and does not define
their contracts.

[Back to Plan D01](README.md)

## Isolation Assertions

Prove that:

- direct Validation use works without policy, Manager, Tool, or AI persistence;
- Validation planning reads only validation configuration;
- Tool is offline and read-only;
- Manager installation never activates content;
- workflow selection is explicit and separate from resource lookup;
- action-workflow operations create no engineering artifacts;
- policy changes affect AI authority but not validation graph or report
  semantics;
- workflow artifacts cannot become executable validation input;
- official resources can be replaced without application changes;
- product versions, packages, and release gates remain independently operable.

## Abuse Matrix

Exercise prompt and instruction injection, argument injection, shell-like text,
path traversal, symlink escape, oversized or malformed resources, digest
mismatch, dependency confusion, stale locks, unauthorized network activity,
sensitive output, policy mutation attempts, scope expansion, ambiguous artifact
selection, and interrupted atomic writes.

Verify absolute policy prohibitions under every level, restrictive combination
of overlapping categories, exact human authority, redaction, containment, and
absence of hidden retries or fallback sources.

## Evidence

Every scenario records exact inputs, product and resource identities, policy
digest and effective source where applicable, invoked operations, filesystem
effects, structured results, and cleanup. Secrets and unredacted protected data
are excluded.

## Exit Criterion

No product or resource gains undeclared authority through composition, and all
tested hostile inputs fail at the owning contract boundary.
