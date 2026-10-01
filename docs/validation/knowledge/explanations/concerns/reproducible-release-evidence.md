# Reproducible Release Evidence

## Risk And Applicability

Apply this concern when source becomes a package, binary, image, generated
bundle, or other deliverable. The risk spans ecosystems: the released artifact
may omit inputs, include unintended state, differ between equivalent builds,
or fail when consumed outside the checkout.

## Consequences

Incomplete artifacts can make installation fail, remove documentation or
licenses, ship stale generated output, or expose undeclared files. Uncontrolled
inputs can produce irreproducible artifacts, weaken provenance, and prevent
reviewers from attributing a difference to source. These failures can block a
release or create downstream security and support costs.

## Required Evidence

Identify canonical source inputs, dependency identities, tool version lines,
build instructions, environment, and generated assets. Inspect the package
manifest or archive, build from the packaged source in isolation, install or
open the deliverable through its public surface, compare critical bytes or
semantics with an independent oracle, and check for unintended repository
mutation. Rebuild comparison is useful when byte reproducibility is a declared
claim.

The Reproducible Builds definition requires the same source, build environment,
instructions, and dependencies for identical output.
[Source: reproducible-builds.definition#definition](https://reproducible-builds.org/docs/definition/)

## Limitations

Archive completeness does not prove runtime correctness. One successful build
does not establish repeatability. Two identical outputs do not prove absence of
malicious inputs. A clean Git status cannot see every cache, external store, or
ignored file. Platform, installer, signing, provenance, and publication claims
need their own evidence when applicable.

## Cost Mutation And External Services

Packaging can execute build scripts, populate caches, create artifacts, and
consume substantial compute and storage. Dependency retrieval, signing,
registries, transparency logs, and remote builders add network, credentials,
privileges, cost, and mutable external state. Separate validation from
publication and authorize each external effect explicitly.

## Response And Next Step

Start with the least costly archive inspection and isolated consumer smoke test
that observes the material consequence. Add cross-platform or repeated builds
only for declared promises. Use [Isolation And Determinism](../patterns/isolation-and-determinism.md),
then adapt an applicable [Recipe](../../how-to/recipes/README.md). Record any
unverified release property as residual risk.
