# B06.4: Public Verification And Closure

## Status

`Future`. Begins after all selected immutable publication steps have completed.

[Back to Plan B06](README.md)

## Objective

Prove the released system from public interfaces, promote stable discovery only
after that proof, and close the temporary authority used for the first release.

## Public Readback

From clean, unauthenticated environments:

1. fetch and verify every signed source tag and metadata-only source release;
2. install `liknon-cli` and each selected B05 source package from its public
   registry;
3. fetch native manifests and archives through the public project domain,
   authenticate metadata first, and verify installed command hashes;
4. acquire selected resources through Manager from the public Hub and consume
   them through Tool;
5. verify all three direct capability commands and both long and short launcher
   namespaces, versions, exit codes, and supported target claims;
6. install and exercise every activated B05 package through its normal package
   manager and representative consumer;
7. run the blocking integrated scenarios against public artifacts only;
8. compare all public bytes and metadata with the frozen ledger.

## Promotion Order

After all immutable readback passes:

1. promote Hub catalog references and selected B05 stable references whose
   dependencies are already verified;
2. publish or activate the corresponding website installation paths;
3. promote the B04 CLI-suite pointer last;
4. repeat a concise clean-environment smoke test through every promoted path.

## Closure

- Retain the release ledger, approval, provenance, signatures, digests,
  publication state, public-readback report, and incident notes.
- Revoke one-time bootstrap tokens and remove temporary secrets.
- Prefer trusted publishing or the narrow recurring credentials approved by
  each product plan.
- Confirm retention and immutability policies on public objects.
- Update status pages and documentation with exact supported products,
  channels, targets, trust modes, and exclusions.
- Return later releases to their owning trains; the four CLI executables remain
  one B04 suite train.

## Acceptance Criteria

- [ ] Public installation succeeds without private credentials or repository
      state.
- [ ] Public bytes and metadata match retained evidence.
- [ ] Stable pointers were promoted only after complete verification.
- [ ] The CLI-suite pointer was promoted last.
- [ ] Temporary bootstrap credentials were revoked or rotated.
- [ ] Documentation makes no claim for an excluded target or ecosystem.
- [ ] Recovery owners can reconstruct the complete first-release state from
      retained evidence.
