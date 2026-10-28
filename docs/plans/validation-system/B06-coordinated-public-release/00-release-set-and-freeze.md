# B06.0: Release Set And Freeze

## Status

`Future`. This is the entry gate for B06.

[Back to Plan B06](README.md)

## Objective

Freeze one exact, reviewable set of candidates before any registry package,
Hub object, native object, metadata release, or stable pointer is published.

## Release-Set Ledger

Record, without secret values:

- release-set ID and creation time;
- human release owner and approvers;
- repository commit and signed product tags;
- each included product, package, version, command, platform target, and public
  destination;
- each retained candidate ID, digest, provenance statement, signature, and
  retention deadline;
- package, Hub-resource, runtime, and aggregate-suite dependencies;
- compatibility claims between Validator, Manager, Tool, launcher, resources,
  and activated language APIs;
- every excluded product, target, channel, or B05 ecosystem with a reason;
- required publication and readback workflows;
- recovery owner for every irreversible destination.

Product versions are not normalized. The ledger binds their independently
chosen versions into one coordinated release set.

## Freeze Rules

1. Resolve the dependency graph and reject cycles or unpublished prerequisites.
2. Authenticate every candidate from retained evidence.
3. Confirm that tags and package metadata still point to the recorded commits.
4. Confirm that candidate retention exceeds the release and recovery window.
5. Reject mutable references such as branches, floating package ranges, or Hub
   aliases where an immutable identity is required.
6. Freeze the ledger and obtain explicit human approval.
7. Any candidate, dependency, destination, or version change invalidates the
   approval and creates a new ledger revision before publication.

## B05 Inclusion Rule

Include only language ecosystems with an approved consumer-driven API,
registry-ready package, conformance evidence, and destination authority. An
ecosystem that has not reached that state is marked `excluded`, not `pending`
inside an otherwise approved release set.

## Acceptance Criteria

- [ ] Every included candidate has one owner and immutable digest.
- [ ] Every dependency resolves to an included or already public immutable
      version.
- [ ] Versions and tags match product-owned metadata.
- [ ] Exclusions are explicit and do not create broken advertised behavior.
- [ ] The human release owner approved the exact frozen ledger.
