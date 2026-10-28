# Validation CLI Distribution Profile

`product.json` is the reviewed B01 profile for `liknon-validator` and `likv`.
It owns Validation package, tag, command, target, namespace, protected
environment, and stable-pointer identities while consuming only generic
release primitives from `distribution/cli/common`.

The `smoke/` scripts own Validation-specific public-install behavior. Reusable
workflows receive those reviewed paths as product inputs and remain unaware of
Validation commands or configuration semantics.

The production detached-signature public key is activated during checkpoint
AP-3 and stored as `direct/public-key.pem`. Release preflight fails closed while
that trust root or any required protected environment is absent.
