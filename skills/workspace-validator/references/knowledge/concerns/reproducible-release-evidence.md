# Reproducible Release Evidence

## Question And Applicability

What evidence supports a claim that a source state produces the intended
release contents repeatably? Apply this to distributable crates or archives;
adapt the artifact mechanism for other ecosystems.

## Risks And Required Evidence

Relevant failures include unlocked dependency resolution, generated content
drift, omitted files, unintended secrets/build output, host-dependent bytes,
and validation that changes tracked state. Use locked dependency evidence,
deterministic generation checks, archive path inspection, build-from-package
verification, and before/after repository comparison.

Cargo distinguishes listing package contents from assembling and verifying the
package, so both can be needed for different claims.
[Source: rust.cargo#cargo-package](https://doc.rust-lang.org/cargo/commands/cargo-package.html)

## Limitations

A successful local package verifies only the selected host/toolchain and Cargo
rules. It does not establish bit-for-bit reproducible binaries, registry
publication, provenance, consumer installation, vulnerability absence, or
support for other platforms. Git cleanliness cannot observe ignored or remote
mutation.

## Cost, Mutation, And External Services

Package assembly writes build output and can run build scripts. Offline mode
avoids registry access only when dependencies are already available. Publishing
is an external mutation and is never implied by package validation.

## Selection And Next Step

Define exact expected paths and forbidden classes, regenerate through one
canonical task, compare bytes/digests, inspect the actual archive, and build it
without publishing. See the [Rust CLI release recipe](../recipes/rust-cli-release-gate.md)
for one bounded composition.
