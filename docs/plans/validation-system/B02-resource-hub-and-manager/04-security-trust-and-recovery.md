# Phase B02.4: Establish Security, Trust, And Recovery

[Back to the Resource Hub and Manager plan](README.md)

## Purpose

Prove that untrusted catalogs, metadata, payloads, filesystems, networks, and
resource instructions cannot escape the approved Manager contract.

## Trust Verification

Manager authenticates catalog and resource metadata from a separately
established trust root before accepting object locations or hashes. TLS is
transport protection, not the sole artifact identity.

Key activation, rotation, expiry, retirement, compromise, and revocation are
documented and covered by fixtures. Expired or unavailable trust metadata fails
closed without discarding a previously valid local installation.

## Payload Safety

Reject:

- absolute paths, traversal, duplicate normalized paths, and unsafe encodings;
- symlink, junction, and reparse-point escapes;
- undeclared files, lengths, media types, or digests;
- archive bombs and configured size/count/ratio limit violations;
- device files and unsupported filesystem entry types;
- executable lifecycle hooks and implicit command declarations;
- resource metadata that attempts to override Manager, Tool, policy, or agent
  authority.

Executable text or files may be declared as resource data, but Manager never
runs them and always exposes that capability metadata.

## Network And Cache Safety

Use bounded redirects, timeouts, retries, response sizes, and concurrent
downloads. Catalog metadata controls no credential destination. Authentication
headers cross no unapproved origin boundary.

Global caches are content-addressed and untrusted until reverified. Workspace
locks contain no cache path dependency. Offline verification succeeds from
installed bytes and retained authenticated metadata.

## AI-Initiated Operations

An agent may propose or invoke Manager only under the applicable
`.liknon/ai/policy.json` categories for network access, workspace mutation,
external-state effects, dependency management, and sensitive data. Manager
itself does not read, parse, or enforce AI policy and does not infer
conversational authority. Policy evaluation belongs to the invoking AI workflow
before it calls Manager.

This rule applies to every compatible agent workflow. Manager has no knowledge
of official action or validation-engineering workflows, workflow-specific
policy overrides, process IDs, or agent-profile identities.

Direct human use remains governed by the explicit command and operating-system
permissions rather than AI policy.

## Recovery

Every mutation records enough private local transaction state to detect an
interruption. Recovery either completes the exact authenticated plan or restores
the prior complete state. It never re-resolves during recovery.

A failed update preserves the previous valid installation and lock. A security
revocation is reported prominently but does not silently delete local evidence.

## Adversarial Verification

Cover forged metadata, replay, rollback, mirror mismatch, key rotation,
revocation, path escape, unsafe links, archive expansion, resource cycles,
prompt-injection payloads, interrupted writes, disk exhaustion, and concurrent
Manager processes.

## Exit Criterion

Manager fails closed on unverifiable inputs, preserves the last complete state,
and never converts resource content into execution or authority.
