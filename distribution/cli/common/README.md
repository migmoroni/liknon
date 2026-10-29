# Shared CLI Distribution Foundation

This directory contains product-neutral release contracts, installer templates,
and publication scripts used by the complete native Liknon CLI suite. Product
identity and publication authority enter only through the reviewed descriptor
under `distribution/cli/suite/`.

`liknon-cli-distribution` validates descriptors and Cargo metadata, creates
internal and public manifests, assembles deterministic archives, renders
versioned installers, selects supported hosts, and verifies retained bytes.
It never contains destination credentials or product behavior.

The current suite contract exposes four distinct executable payloads and no
physical compact aliases. Suite semantic smoke tests live beside the product
descriptor. Candidate workflows verify direct execution, launcher routing,
transactional replacement, rollback, and exact archive inventory.

Generated candidates belong under `target/distribution/cli-suite/` and are not
tracked. Public verification roots are tracked only after their authority is
provisioned. Private signing keys, certificates, registry tokens, and
Cloudflare credentials remain in protected environments.

`scripts/rehearse-source-packages.sh` packages supporting workspace crates,
injects their exact archives into a temporary Cargo directory source, and then
packages dependent applications with crates.io replaced by that isolated
source. This proves that source candidates resolve versioned package bytes
rather than workspace paths before any supporting crate is publicly available.
By default, the temporary source is deleted after the rehearsal. A workflow may
set `LIKNON_SOURCE_REHEARSAL_ROOT` to retain it inside ephemeral runner storage
while assembling dependent source contracts. It is never a publication
destination.
