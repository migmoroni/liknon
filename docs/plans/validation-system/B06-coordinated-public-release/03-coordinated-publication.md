# B06.3: Coordinated Publication

## Status

`Future`. Blocked by an approved B06.2 rehearsal and explicit human release
approval.

[Back to Plan B06](README.md)

## Objective

Publish the frozen release set through product-owned workflows in dependency
order, with explicit partial-release state and no rebuilds.

## Publication Order

The exact order is generated from the B06.0 dependency graph. A typical order
is:

1. supporting Rust crates and other package prerequisites required by
   `liknon-cli` or selected language packages;
2. immutable Hub payloads and versioned metadata, without advancing mutable
   catalog aliases or stable pointers;
3. the `liknon-cli` source package and immutable complete native suite
   archives;
4. activated B05 language packages in registry dependency order;
5. metadata-only source releases and other immutable public records not already
   created by the product workflows.

This list is a default shape, not authority to violate an actual dependency.

## Execution Contract

- One human approves the exact frozen release set before step one.
- Each publication job obtains only its destination-specific authority through
  its product-owned protected environment.
- The coordinator passes immutable candidate and ledger identities; it does not
  pass raw secrets between workflows.
- Every destination records `pending`, `published`, `verified`, `failed`, or
  `excluded` with the public immutable identity and readback evidence.
- A destination marked `published` is never republished with different bytes.
- Publication stops when a dependency fails. Independent already published
  objects remain recorded; they are not deleted to simulate rollback.

## Partial Release And Resume

Cross-registry publication cannot be atomic. On failure:

1. preserve the exact ledger, candidates, and destination state;
2. classify whether the failed destination can be retried without mutation;
3. resume only unpublished or unverified steps from the same candidates;
4. do not advance any stable pointer;
5. if an accepted immutable package is wrong, stop and prepare a new product
   version under its owning plan rather than overwriting it.

## Acceptance Criteria

- [ ] No publication begins without the frozen-ledger approval.
- [ ] Every product workflow uses its own protected authority.
- [ ] Every public immutable object matches the candidate digest.
- [ ] Dependency failures stop dependent publications.
- [ ] Partial publication state is complete enough to resume safely.
- [ ] No stable pointer or suite default is advanced in this phase.
