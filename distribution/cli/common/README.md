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
