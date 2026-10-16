# B03 Tool Accounts, Permissions, And Publication Authority

[Back to the Tool CLI plan](README.md)

## Purpose

Stage only the external ownership, signing, and publication access needed for
the Tool release phase currently being implemented.

## Canonical Identities

| Purpose | Identity |
| --- | --- |
| Cargo packages | `liknon-tool` and `liknon-workspace` |
| Signed tag | `liknon-tool-v<VERSION>` |
| GitHub metadata release | Exact signed tag |
| Native download channel | Tool-specific immutable Cloudflare prefix |
| Human approval | Credential-free protected environment |

Tool publication uses no Hub resource-publication or namespace credential.

## Credential Boundaries

- Build, test, package inspection, and end-to-end verification have no
  publication credential.
- crates.io, GitHub, Cloudflare, detached metadata signing, and platform signing
  use separate protected environments.
- The human release owner controls the signed tag and final approval.
- Bootstrap tokens are narrow, temporary, and revoked after use.
- No secret enters a binary, archive, log, cache key, or retained public
  evidence.

## Checkpoints

1. Confirm `liknon-tool` and `liknon-workspace` package ownership plus tag
   ownership before B03.0.
2. Create credential-free release candidates during B03.3.
3. Configure protected publication environments only when candidates pass.
4. Activate destination credentials immediately before B03.4 publication.
5. Verify public readback, revoke temporary credentials, and retain evidence.

## Exit Criterion

Tool publication has independent ownership, minimal credentials, and no ability
to alter Hub resources, Manager releases, or Validation CLI releases.
