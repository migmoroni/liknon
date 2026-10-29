# Resource Hub Publication Boundary

The production public trust root is committed here as `trust-root.json` only
after its offline review and recovery procedure are complete. Private signing
seeds never enter this directory, the repository, workflow artifacts, or
resource payloads.

`.github/workflows/release-resource-hub.yml` requires that reviewed trust root
and a signed `liknon-hub-v<SNAPSHOT>` source tag. It assembles and signs the
exact tagged resource tree in the protected `sign-liknon-hub` environment, then
publishes immutable objects and advances only the Hub stable pointer through
the isolated `publish-liknon-hub` environment.

No production Hub URL is advertised while `trust-root.json`, protected
environments, storage boundaries, and public readback are incomplete.
