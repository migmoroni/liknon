# Shared CLI Distribution Foundation

This directory contains product-neutral release contracts, installer templates,
and publication scripts used by native Liknon CLIs. Product identity and
publication authority enter only through a reviewed descriptor under a sibling
product directory.

`liknon-cli-distribution` validates descriptors and Cargo metadata, creates
internal and public manifests, assembles deterministic archives, renders
versioned installers, selects supported hosts, and verifies retained bytes.
It never contains destination credentials or product behavior.

The current standalone-product contract exposes one canonical command and
exactly one compact alias. Product-specific semantic smoke tests live beside
the product descriptor and are injected into the reusable public-verification
workflow as reviewed paths.

Generated candidates belong under `target/distribution/<product>/` and are not
tracked. Production public keys, signing keys, certificates, registry tokens,
and Cloudflare credentials are provisioned through protected release
environments; no placeholder credential is committed here.

`scripts/rehearse-source-packages.sh` packages supporting workspace crates,
injects their exact archives into a temporary Cargo directory source, and then
packages dependent applications with crates.io replaced by that isolated
source. This proves that source candidates resolve versioned package bytes
rather than workspace paths before any supporting crate is publicly available.
By default, the temporary source is deleted after the rehearsal. A workflow may
set `LIKNON_SOURCE_REHEARSAL_ROOT` to retain it inside ephemeral runner storage
while assembling dependent source contracts. It is never a publication
destination.
