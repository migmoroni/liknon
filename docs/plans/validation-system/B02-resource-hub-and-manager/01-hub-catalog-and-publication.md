# Phase B02.1: Build The Hub Catalog And Publication Pipeline

[Back to the Resource Hub and Manager plan](README.md)

## Purpose

Build and prove the deterministic catalog and publication pipeline against a
local or non-production test origin. It produces authenticated metadata and
immutable payload candidates for official and approved third-party resources
without making the Hub an execution service. Production deployment, public
credentials, public pointer mutation, and Internet readback belong exclusively
to B02.5.

## Canonical Assembly

Official artifacts are assembled deterministically from the canonical sources
identified by A01. Each artifact records its source repository, commit,
resource manifest, payload inventory, and digests.

Assembly:

- reads only declared source files;
- follows no unsafe links;
- normalizes no authored payload bytes unless the resource contract explicitly
  defines that transformation;
- produces no credential-bearing file;
- runs no command declared by a resource;
- leaves tracked sources unchanged.

## Catalog

The catalog contract supports metadata discovery by ID, kind, namespace, title,
summary, compatibility, trust class, and version. B02.1 implements and tests it
through local or non-production fixtures. Search results are stable for one
catalog snapshot and identify that snapshot exactly.

The Hub publishes:

- signed versioned catalog snapshots;
- immutable resource metadata;
- immutable payload objects;
- revocation and security-advisory metadata;
- a narrowly mutable stable catalog pointer.

Consumers authenticate the snapshot before trusting object locations or
digests.

## Public Deployment Boundary

B02.1 may generate production-form catalog snapshots, immutable object layouts,
and stable-pointer candidates, but it does not publish them to the production
origin. Test signing identities and local object stores must be visibly
non-production and must not require release credentials.

B02.5 alone provisions or selects the production origin, supplies protected
publication credentials, publishes immutable objects, advances the public
pointer, exercises rollback, and reads every object back through the public
domain. Passing B02.1 therefore proves implementation readiness, not public
availability.

## Namespace And Trust Governance

Document:

- official namespace ownership;
- verified publisher identity checks;
- community submission requirements;
- review and moderation evidence;
- namespace transfer and abandonment;
- malicious-content response;
- trust downgrade, yanking, and revocation;
- appeal and correction handling.

`verified` means the documented identity and review checks passed. It does
not mean that every instruction is correct or safe for every workspace.

## Third-Party Publication

A submission declares all payloads, capabilities, dependencies, compatibility,
license, source, and executable-content indicators. Publication automation
validates schemas and bytes but does not execute submitted instructions.

Workflow publication validates graph structure, declared node and edge IDs,
continuation eligibility, resource references, capability/effect vocabulary,
artifact declarations, and declared AI bootstrap and decision-policy contract
ranges. It does not evaluate the workflow's conclusions, instantiate an agent
profile, grant the resource a policy override, or permit resource content to
replace the program bootstrap entry point.

The first implementation may use reviewed repository submissions and a static
authenticated catalog. A dynamic service is not required until measured Hub
needs justify it.

## Verification

- Rebuilding one artifact from the same source produces the same payload bytes
  and metadata.
- Catalog snapshots reject mutation, missing objects, and inconsistent
  references.
- Search never needs payload execution.
- Removed, yanked, and revoked states have deterministic client behavior.
- Namespace and trust transitions retain auditable evidence.
- Official and third-party workflows with different graph shapes pass through
  the same publication checks.
- The local/test origin and publication identity reject overwrite of immutable
  versioned objects through the normal publication path.
- No B02.1 test requires a production account, public DNS, production secret,
  or Internet-visible object.

## Exit Criterion

The repository can reproducibly assemble, sign, publish, query, and recover an
authenticated catalog and immutable payloads through the approved local/test
origin, with explicit ownership, trust, moderation, and recovery semantics.
No public-Hub claim is made until B02.5 passes.
