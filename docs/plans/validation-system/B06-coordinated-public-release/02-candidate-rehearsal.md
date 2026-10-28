# B06.2: Candidate Rehearsal

## Status

`Future`. Blocked by B06.0 and B06.1.

[Back to Plan B06](README.md)

## Objective

Exercise the complete release set and publication sequence without mutating a
public destination.

## Rehearsal Inputs

Use only the exact candidates and evidence named by the frozen ledger:

- crates.io source-package archives and dependency closures;
- target-native Validator, Manager, Tool, launcher, and suite archives;
- Hub catalog, metadata, payload, and signature candidates;
- aggregate `liknon-cli` source and native-suite candidates;
- every activated B05 language-package candidate;
- installers, manifests, checksums, signatures, attestations, and public
  documentation candidates.

## Required Rehearsal

1. Reauthenticate every retained input.
2. Execute each product's isolated package and native smoke tests.
3. Install standalone capability CLIs and the aggregate suite in clean
   environments without mixing their installation roots.
4. Exercise Manager-to-Hub acquisition and Tool consumption from a local or
   test origin that preserves immutable object semantics.
5. Verify launcher forwarding, version reporting, help, aliases, exit codes,
   signal handling, and component compatibility.
6. Exercise each activated B05 API against its shared conformance fixtures and
   at least one representative consumer.
7. Run the D01 scenarios selected as first-release blockers.
8. Simulate interruption after every irreversible publication boundary and
   prove resume uses the same candidate.
9. Generate a publication-order plan from the resolved dependency graph.

Rehearsal may use test registries or local registry substitutes, but it must not
claim that a substitute proves production ownership. B06.1 separately proves
the real destination authorities.

## Failure Rule

A changed candidate is not a rehearsal fix. Return it to the owning B plan,
produce new evidence, revise the release-set ledger, and repeat the affected
rehearsal. B06 never patches an accepted artifact in place.

## Acceptance Criteria

- [ ] Every selected candidate passes its product-owned verification.
- [ ] Cross-product scenarios pass using the exact frozen versions.
- [ ] Publication ordering is derived from explicit dependencies.
- [ ] Interruption and resume are proven without rebuilding.
- [ ] No rehearsal job can mutate a production destination.
- [ ] One retained rehearsal report is approved before publication.
