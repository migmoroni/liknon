# Phase B03.4: Verify Tool And Hand Off To The CLI Suite

## Status

`Partially implemented`. Local Tool behavior and store interoperability are
present. This phase completes focused evidence for B04; it does not publish a
standalone Tool product.

[Back to Plan B03](README.md)

## Objective

Prove Tool end to end from installed resource state through human and
machine-readable retrieval, then attach that evidence to the exact B04 suite
candidate.

## Verification Sequence

1. run formatting, linting, tests, documentation tests, and package checks;
2. build Tool from the exact `liknon-cli` source closure;
3. verify direct `liknon-tool` help, version, JSON, errors, and exit codes;
4. verify `liknon tool` and `liknon t` route to the same executable;
5. consume Manager-produced state and neutral fixture state;
6. exercise catalog, search, show, resolve, context, and any C01-approved
   bootstrap operation;
7. reject integrity drift, unsafe paths, resource cycles, unsupported schemas,
   revocation, and oversized context requests;
8. prove offline and read-only behavior under observation;
9. run target-native checks for every B04 target;
10. bind reports and digests to the exact B04 suite candidate.

## Cross-Product Proof

The joined fixture proves:

- Manager may create valid resource state without granting instruction
  authority;
- Tool may read that state without invoking Manager;
- Validator does not depend on either resource application;
- the launcher routes without performing resource or validation behavior;
- resource-only updates require no CLI rebuild;
- all four executables in an archive share one B04 version and provenance chain.

## Publication Boundary

B03 creates no registry publication, native object prefix, stable pointer, or
publication credential. B04 owns CLI-suite publication. B06 coordinates the
first public readback. B02 independently owns Hub publication.

## Acceptance Criteria

- [ ] All focused Tool tests pass against the exact suite source closure.
- [ ] Direct and routed Tool invocations are equivalent.
- [ ] No physical compact alias exists.
- [ ] Every selected B04 target has reproducible Tool evidence.
- [ ] Manager-to-Tool interoperability passes without hidden process coupling.
- [ ] Tool performs no network or mutation effect.
- [ ] Every evidence record binds to the B04 version, tag, commit, target, and
  executable hash.
- [ ] B04 can reject a suite candidate when Tool evidence is absent or stale.
- [ ] B03 owns no standalone publication authority.

## Exit

B03.4 exits when Tool is behaviorally ready and its exact evidence can gate the
complete B04 source and native suite candidates.
