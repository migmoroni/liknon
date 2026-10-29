# CLI Distribution Foundation

## Status

`Approved target`. This document defines the reusable mechanics for one
public CLI suite. B04 owns the public package and native distribution contract;
B01, B02, and B03 supply capability behavior and verification inputs.

The earlier per-capability publication model is superseded. Its implemented
build, provenance, signing, installer, and readback work remains reusable only
where it conforms to this suite contract.

## Distribution Subject

There is one public CLI distribution subject:

| Field | Value |
| --- | --- |
| Product | Liknon CLI suite |
| Source package | `liknon-cli` |
| Version authority | `apps/liknon-cli/Cargo.toml` |
| Physical executables | `liknon`, `liknon-validator`, `liknon-manager`, `liknon-tool` |
| Compact executable aliases | None |
| Native topology | One archive per supported target containing all four executables |
| Public owner | B04 |
| Coordinated publication | B06 |

Validator, Manager, and Tool remain separate executable processes and domain
boundaries. They are not independent public source packages or native download
products after B04.

The resource Hub remains a separate data publication subject owned by B02.

## Product Descriptor

The shared distribution tooling consumes a reviewed descriptor whose canonical
fields include:

- product ID `cli-suite`;
- package `liknon-cli`;
- package path and version authority;
- exactly four binary target names;
- launcher namespace map;
- source-package tag and commit;
- supported target rows;
- archive format and filename per target;
- executable filename, size, and SHA-256 per target;
- runtime baseline and platform-authenticity mode;
- provenance references;
- immutable R2 prefix and stable pointer;
- installer compatibility;
- semantic smoke commands for direct and routed execution.

The descriptor rejects:

- missing or extra executable targets;
- compact executable aliases;
- per-capability public versions or stable pointers;
- target archives with a partial command set;
- cross-target payloads;
- unpublished source dependencies;
- reconstructed or renamed release payloads.

## Version And Provenance

One suite candidate binds:

```text
package + version + tag + commit + source digest + target +
archive digest + four executable digests
```

The same version is observable from the source package, launcher, capability
executables, native manifests, installers, documentation, and public stable
pointer.

The build lane compiles each executable once for a target. Later assembly,
signing, candidate storage, approval, and publication copy or wrap those
verified bytes; they do not rebuild them.

Every target manifest records all four executable hashes. A component cannot be
promoted, rolled back, or replaced independently from the suite version.

## Source Package

`cargo package` and isolated `cargo install` must prove that
`liknon-cli`:

- builds outside the repository;
- has a complete publishable dependency closure;
- installs exactly four command files;
- installs no compact executable aliases;
- contains no Hub corpus or managed AI-resource payload;
- preserves B01, B02, and B03 behavior;
- uses the committed lockfile and supported MSRV.

Supporting crates can remain separate implementation units, but they publish no
binary targets and do not create independent CLI release identities.

## Native Target Archives

Each supported Rust target produces one archive. The archive contains:

- the four target-compatible executables;
- the authenticated suite manifest;
- only the minimal target-specific installation metadata approved by B04.

GNU and MUSL are distinct Linux targets and distinct archives. Target detection
selects one archive; GNU wins only when both environments are genuinely
compatible.

The site and installer request one archive for one target. They never download
unrelated operating-system, architecture, or libc payloads.

## Storage Layout

R2 uses one suite namespace, for example:

```text
cli/
  v<VERSION>/
    <TARGET>/
      <ARCHIVE>
      manifest.json
      manifest.sig
      provenance.json
  stable.json
  stable.json.sig
```

Exact names remain an implementation detail frozen by B04.0, but the following
invariants are mandatory:

- immutable versioned objects;
- one stable pointer for the suite;
- one archive per target;
- no Validator, Manager, or Tool stable pointer;
- cache rules that never make mutable metadata indistinguishable from immutable
  payloads;
- authenticated rollback to a complete previous suite.

## Installation And Recovery

Installers:

1. resolve the target;
2. retrieve authenticated metadata;
3. select exactly one suite archive;
4. verify archive and four executable hashes;
5. stage all files outside the active destination;
6. replace the complete suite atomically;
7. verify direct and routed execution;
8. retain or restore the previous complete suite on failure.

Partial versions and mixed component versions are invalid states. Uninstall
removes the four suite-owned command files without deleting unrelated user
resources.

## Shared Verification

The reusable verification layer proves:

- descriptor schema and target matrix integrity;
- source-package closure;
- deterministic archive inventory;
- native execution on every supported target;
- direct/routed semantic equivalence;
- namespace aliases without physical alias files;
- provenance, checksum, signature, and stable-pointer verification;
- extraction, path, link, permission, interruption, rollback, and replay
  defenses;
- exact public readback of previously reviewed bytes.

B01, B02, and B03 contribute focused domain fixtures. B04 joins them under the
suite. B06 publishes the already joined candidate.

## Ownership Boundaries

- B01 owns Validator behavior, not public Validator packaging.
- B02 owns Manager behavior and Hub publication, not public Manager packaging.
- B03 owns Tool behavior, not public Tool packaging.
- B04 owns `liknon-cli`, launcher routing, source closure, target archives,
  installers, and suite release identity.
- B06 owns coordinated external approval, publication, and readback.
- E01 owns the site surface that selects and documents the suite download.

## Acceptance Criteria

- [ ] One descriptor represents the complete CLI suite.
- [ ] One source package installs exactly four executables.
- [ ] One native archive exists per supported target.
- [ ] Every native archive contains all four executable roles.
- [ ] No compact executable alias or standalone capability download exists.
- [ ] All files in one suite candidate share one version, tag, commit, and
  provenance chain.
- [ ] B01, B02, and B03 domain gates remain independently diagnosable.
- [ ] Installation and rollback are atomic across the four executables.
- [ ] B06 can promote the reviewed candidate without rebuilding it.
