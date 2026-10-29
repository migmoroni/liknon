# Manager CLI Distribution Profile

This directory instantiates the product-neutral CLI distribution foundation
for `liknon-manager` and `likm`. `product.json` is the reviewed product
descriptor used by release inspection, target-native builds, direct archive
assembly, installers, publication, and public readback.

Manager owns its package, signed-tag prefix, immutable object prefix, stable
pointer, protected environments, semantic smoke tests, and recovery evidence.
It shares mechanisms with other CLIs but inherits none of their identities,
candidates, credentials, or publication authority.

The profile prepares release candidates only. The first public activation is
coordinated by B06 after supporting crates and every retained native candidate
are independently verified.
