# Static Hub Publication

## Layout

The initial Hub is a static authenticated object tree:

```text
stable.json
stable.json.sig.json
snapshots/<snapshot-id>/catalog.json
snapshots/<snapshot-id>/catalog.json.sig.json
objects/sha256/<first-two-hex>/<sha256>
```

The catalog snapshot and payload objects are immutable. Only the stable pointer
is advanced, and only after every referenced object has been written and read
back successfully.

## Deterministic Assembly

The private `liknon-resource-hub-maintenance` package assembles a snapshot from
authored `resource.json` files. It:

- reads only declared regular files below each resource directory;
- rejects symlinks and path escapes;
- preserves exact authored payload bytes;
- derives canonical content-addressed object paths;
- sorts resources, dependencies, collections, capabilities, payloads, and
  advisories deterministically;
- validates the public contracts before writing output;
- refuses to overwrite an existing Hub output;
- signs the exact catalog and pointer bytes with Ed25519;
- never executes resource content.

Checked-in neutral fixtures under `fixtures/resource-hub/source/` prove
reproducible assembly, transitive resolution inputs, collections, unrelated
workflow shapes, community content, revocation, and advisory coverage. Test
keys are generated ephemerally and are visibly separate from production trust.

## Maintainer Commands

Generate the checked-in public schemas:

```sh
cargo run -p liknon-resource-hub-maintenance --locked -- \
  schema --output crates/liknon-resource-contracts/schemas
```

Create a public trust root from a protected 32-byte Ed25519 seed encoded as
standard Base64:

```sh
cargo run -p liknon-resource-hub-maintenance --locked -- trust-root \
  --private-key <seed-file> \
  --key-id <key-id> \
  --authority <authority> \
  --not-before <unix-seconds> \
  --not-after <unix-seconds> \
  --output <trust-root.json>
```

Assemble and verify a local candidate:

```sh
cargo run -p liknon-resource-hub-maintenance --locked -- assemble \
  --source <authored-resources> \
  --output <static-hub> \
  --private-key <seed-file> \
  --key-id <key-id> \
  --snapshot <snapshot-id> \
  --sequence <positive-sequence> \
  --published-at <unix-seconds> \
  --expires-at <unix-seconds>

cargo run -p liknon-resource-hub-maintenance --locked -- verify \
  --hub <static-hub> \
  --trust-root <trust-root.json> \
  --now <unix-seconds>
```

Private signing material must never enter source control, build artifacts,
resource payloads, or public evidence. Production publication and pointer
advancement use isolated credentials and protected approval in the coordinated
release track.
