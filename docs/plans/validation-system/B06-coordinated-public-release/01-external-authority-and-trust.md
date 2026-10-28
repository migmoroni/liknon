# B06.1: External Authority And Trust

## Status

`Future`. Blocked by an approved B06.0 ledger.

[Back to Plan B06](README.md)

## Objective

Prove that every destination, identity, key, protected environment, and public
trust root needed by the frozen release set is ready before candidates are
published.

## Authority Model

Each product and destination retains an isolated authority. B06 coordinates
their use but never copies all secrets into one job or environment.

Required controls include, where selected by the ledger:

- GitHub repository, protected environments, immutable releases, signed tags,
  Actions permissions, and artifact retention;
- crates.io package ownership and trusted publishing or a short-lived scoped
  bootstrap token;
- Cloudflare account, R2 buckets or prefixes, custom domains, bucket locks,
  cache policy, CORS policy, and least-privilege API tokens;
- Hub metadata-signing identity and immutable payload storage authority;
- B05 registry namespaces and trusted-publishing identities for every activated
  ecosystem;
- release-manifest signing keys and public-key publication;
- platform-signing identities for targets included in the release set;
- website authority for installation and release documentation.

Use [Accounts And Permissions](accounts-and-permissions.md) as the consolidated
human checklist. Product plans remain authoritative for the exact scope and
secret names of their own environments.

## Preflight

For every destination:

1. verify ownership and name availability;
2. verify that the protected workflow identity can perform a non-mutating
   authentication check;
3. verify that unrelated products cannot use the authority;
4. verify public trust roots from an unauthenticated client;
5. verify expiration, rotation, revocation, and recovery owners;
6. verify that logs and retained evidence cannot expose secret values.

No candidate publication starts while one selected destination remains
unprovisioned or while a trust root exists only on the release operator's
machine.

## Acceptance Criteria

- [ ] Every selected destination has one accountable owner.
- [ ] Every credential is destination-scoped and stored only in its protected
      environment.
- [ ] Build and rehearsal jobs have no publication credentials.
- [ ] Public verification needs no private credential.
- [ ] Signing public keys and trust instructions are available before use.
- [ ] Revocation and recovery procedures are executable.
