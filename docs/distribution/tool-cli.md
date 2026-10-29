# Tool CLI Distribution

`liknon-tool` and `likt` are equivalent executables built from the same
`liknon-tool` Cargo package and version. The project prepares three channels:

- exact signed source tags named `liknon-tool-v<VERSION>`;
- the `liknon-tool` crates.io source package;
- authenticated target-native archives on the project download domain.

No channel is advertised as live before protected publication and public
readback pass. During development, install from a reviewed checkout:

```sh
cargo install --path apps/liknon-tool --locked --force
```

The native target matrix, runtime baselines, archive formats, authenticity
modes, object namespace, stable pointer, and protected environments are defined
by [`distribution/cli/tool/product.json`](../../distribution/cli/tool/product.json).
Tool uses the common archive and installer contracts without sharing a
candidate, stable pointer, signing identity, or publication credential with
another Liknon product.

## Installation Trust

The direct installer authenticates a detached signature over the exact public
manifest before parsing it, selects one declared target, verifies the archive
hash, and installs both command names transactionally without elevated
privileges. An existing installation remains intact on failure.

GitHub Releases contain source and release metadata, not Liknon native
binaries. The native download origin is the project-controlled Cloudflare
domain. Windows self-signed Authenticode and macOS ad hoc signatures are
supplementary platform evidence; the signed direct manifest and exact hashes
remain mandatory on every platform.

Installing Tool does not contact the Resource Hub and does not install or
modify resources. Tool reads an existing `.liknon/resources/` store only when
the user later invokes a resource command. The release smoke matrix exercises
catalog, search, exact retrieval, dependency resolution, bounded context, alias
equivalence, and non-mutation against a contract-valid offline fixture.

## Removal And Updates

Source installations are managed by Cargo. Direct installations are managed by
the authenticated installer and the install directory selected by the user.
Updating replaces only the Tool executables. Uninstalling Tool does not delete
consumer-owned `.liknon/resources/` state.
