# Phase B03.1: Implement Local Catalog And Retrieval

## Status

`Repository implementation complete` for local store loading, catalog, search,
exact retrieval, and locked closure resolution. The bootstrap-specific clauses
remain pending C01 and are excluded from this completion claim.

[Back to the Tool CLI plan](README.md)

## Purpose

Implement deterministic offline discovery and exact retrieval over one valid
locked resource store.

## Store Loading

The bootstrap projection operation selected by B03.0 bypasses store loading
and returns the program-owned entry contract through the approved source
boundary. Every ordinary resource operation requires the explicit or
discovered store contract.

Tool discovers or receives one workspace root, parses the resource manifest and
lock, validates schema compatibility, and builds a local immutable index for
the command invocation.

It does not scan Cargo caches, `node_modules`, language environments, global
agent directories, or arbitrary sibling workspaces.

## Catalog

Catalog entries expose only bounded routing metadata by default:

- stable ID, kind, and version;
- title and concise summary;
- namespace, authorship, license, and source;
- trust, yanked, and revoked state;
- compatibility facts;
- declared capabilities;
- for workflows, bounded graph summary, entry points, referenced resource
  kinds, and declared effects;
- dependency and collection counts;
- available payload files and media types.

Ordering is stable and independent from filesystem enumeration.

## Search

Local search is deterministic and versioned. The initial implementation uses
declared metadata and explicitly searchable text fields. It does not silently
contact a semantic service, embed documents, or send workspace content
elsewhere.

Results identify the matching fields and retain stable ranking for the same
store and query.

## Exact Retrieval

`show` validates the selected locked resource and requested file before
writing content. Canonical text bytes remain exact. Binary payloads require an
explicit supported output mode and are never emitted accidentally to a human
terminal.

Missing, modified, unsafe, unsupported, incompatible, yanked, and revoked
states are distinct typed outcomes.

## Resolution

`resolve` traverses only exact lock edges and returns a stable topological
closure or a typed invalid-lock diagnostic. It performs no version solving and
does not amend the lock.

Workflow closure traversal treats skills, profiles, tools, knowledge, prompts,
and subworkflows as ordinary typed dependencies. It neither chooses a profile
nor executes a transition.

## Verification

- Results are identical with network disabled.
- Bootstrap bytes, when that operation is approved, match the canonical C01
  source and metadata exactly and remain usable with no workspace.
- Catalog ordering survives filesystem and manifest key reordering.
- Search has positive, negative, Unicode, case, and limit fixtures.
- Exact Markdown bytes match canonical installed payloads.
- Binary and oversized payload safeguards are exercised.
- Modified payloads fail before content output.
- Manually populated contract-valid stores behave like Manager-produced stores.

## Exit Criterion

Tool can return its exact bootstrap contract and discover, search, retrieve, and
resolve installed resources exactly and offline without loading unrelated
content or changing state.
