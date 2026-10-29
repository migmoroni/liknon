# Resource Trust And Governance

## Trust Classes

| Class | Meaning |
| --- | --- |
| `official` | Published by the Liknon project identity |
| `verified` | Publisher identity and documented review evidence were accepted |
| `community` | Listed without Liknon endorsement and selected only with explicit consent |
| `local` | Supplied directly by the consumer and never published by the public Hub |

Trust describes provenance and review state. It does not make instructions
correct for a workspace and never grants execution, policy, or activation
authority.

## Namespace Governance

One publisher identity controls a namespace. A reviewed publication records the
publisher, source repository, exact revision when available, license,
authorship, declared capabilities, compatibility, dependencies, payload
inventory, and review evidence.

Namespace transfer requires evidence from the current owner or a documented
abandonment and dispute process. The previous and new owners, effective
snapshot, and review decision remain auditable. A name cannot be reused to
replace immutable historical versions.

Third-party submissions are schema-validated and reviewed as data. Publication
automation never executes submitted instructions, lifecycle hooks, tools, or
workflows.

## Yanking, Revocation, And Correction

A yanked version remains identifiable for an existing lock but is not selected
for a new resolution or requested update. A security-revoked version is never
selected and must have a matching signed advisory. Verification reports a
current revocation without silently deleting the installed evidence.

Trust may be downgraded when review evidence is withdrawn or publisher control
is lost. Corrections publish a new immutable version or catalog snapshot;
historical bytes are not overwritten. Publishers may appeal a moderation
decision with new evidence, but appeal does not suppress the active advisory.

Malicious content should be reported privately through the project security
channel. The response records affected identities and ranges, severity,
mitigation, revocation state, and corrected versions when available.

## Signing-Key Lifecycle

The trust root declares key IDs, algorithms, validity windows, and active,
retired, or revoked status. A new key is distributed through a separately
established trust-root update before it signs a catalog. Retired keys may still
verify historical evidence within their accepted interval; revoked keys fail
closed according to the trust-root contract.

Manager checks signatures, authenticated publication times, expiration,
monotonic snapshot sequence, catalog hashes, and exact payload hashes. TLS
protects transport but is not the artifact identity.
