# Phase B02.1: Build The Hub Catalog And Publication Pipeline

[Back to the Resource Hub and Manager plan](README.md)

## Purpose

Publish authenticated metadata and immutable payloads for official and approved
third-party resources without making the Hub an execution service.

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

The public catalog supports metadata discovery by ID, kind, namespace, title,
summary, compatibility, trust class, and version. Search results are stable for
one catalog snapshot and identify that snapshot exactly.

The Hub publishes:

- signed versioned catalog snapshots;
- immutable resource metadata;
- immutable payload objects;
- revocation and security-advisory metadata;
- a narrowly mutable stable catalog pointer.

Consumers authenticate the snapshot before trusting object locations or
digests.

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
- The production origin cannot overwrite immutable versioned objects through
  the normal publication identity.

## Exit Criterion

The Hub exposes authenticated, queryable metadata and immutable resource
payloads with explicit ownership, trust, moderation, and recovery semantics.
