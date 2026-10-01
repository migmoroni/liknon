# Reproducible Release Evidence

## Question And Applicability

What evidence supports a claim that a source state produces the intended
release contents repeatably? Apply this to any source-to-artifact release path,
then select ecosystem-specific mechanisms only after the artifact and claim are
known.

## Risks And Required Evidence

Relevant failures include unresolved or mutable inputs, generated content
drift, omitted files, unintended secrets/build output, host-dependent bytes,
and validation that changes source state. Reproducible Builds defines a build
as reproducible when the same source, environment, instructions, and
dependencies produce bit-for-bit identical artifacts.
[Source: reproducible-builds.definition#definition](https://reproducible-builds.org/docs/definition/)

Local synthesis: choose evidence independently for input identity, dependency
immutability, deterministic generation, artifact path/content policy,
build-from-artifact verification, repeated byte comparison when claimed, and
before/after source-state observation. One mechanism rarely establishes all of
these claims.

## Limitations

A successful local artifact verifies only the selected toolchain, environment,
inputs, and artifact rules. It does not establish bit-for-bit reproducibility
unless repeated outputs are compared under the declared conditions, nor does it
establish publication, provenance, consumer installation, vulnerability
absence, or support for other platforms. Repository cleanliness cannot observe
ignored, out-of-tree, service, or remote mutation.

## Cost, Mutation, And External Services

Artifact assembly normally writes output and may execute repository or
dependency code. Offline operation is possible only when every required input
is already available. Publishing is an external mutation and is never implied
by release validation.

## Selection And Next Step

Define exact inputs, expected paths, forbidden classes, generation ownership,
and the promised reproducibility level. Generate through one canonical task,
compare paths and bytes or digests as required, inspect the deliverable, and
exercise it without publishing. See the
[Rust CLI](../recipes/rust-cli-release-gate.md) and
[JavaScript/TypeScript package](../recipes/javascript-typescript-package-gate.md)
recipes for separate bounded ecosystem applications.
