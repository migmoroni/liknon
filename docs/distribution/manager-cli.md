# Manager CLI Distribution

`liknon-manager` and `likm` are equivalent executables built from the same
`liknon-manager` Cargo package and version. The project prepares three channels:

- exact signed source tags named `liknon-manager-v<VERSION>`;
- the `liknon-manager` crates.io source package;
- authenticated target-native archives on the project download domain.

No channel is advertised as live before protected publication and public
readback pass. During development, install from a reviewed checkout:

```sh
cargo install --path apps/liknon-manager --locked --force
```

The native target matrix, runtime baselines, archive formats, authenticity
modes, object namespace, stable pointer, and protected environments are defined
by [`distribution/cli/manager/product.json`](../../distribution/cli/manager/product.json).
Manager uses the common archive and installer contracts without sharing a
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

Installing Manager does not contact the Resource Hub and installs no resources.
Hub access occurs only when the user later invokes a networked Manager command
with an independently obtained Hub trust root.

## Removal And Updates

Source installations are managed by Cargo. Direct installations are managed by
the authenticated installer and the install directory selected by the user.
Updating resolves a new immutable Manager release; it never advances the
resource Hub catalog. Uninstalling Manager does not delete consumer-owned
`.liknon/resources/` state.
