# CLI Suite Trust Material

This directory is the canonical source location for the public verification
material used by the complete CLI suite candidate.

Before the protected candidate workflow can run, provision exactly one signed
tag trust mechanism:

- `tag-signing-public-key.asc`; or
- `tag-signing-allowed-signers`.

Provision `public-key.pem` with the public half of the detached metadata key.
Private keys and certificates never belong in this directory or repository.

The files are intentionally absent until the authority checkpoints in
`docs/plans/validation-system/B04-unified-launcher-and-cli-suite/accounts-and-permissions.md`
are completed.
