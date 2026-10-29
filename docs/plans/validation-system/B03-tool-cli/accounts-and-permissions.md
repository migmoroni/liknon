# B03 Tool Accounts, Permissions, And Publication Authority

## Status

`External setup pending`. Repository-owned names and isolation boundaries are
defined; each account, environment, key, and credential is created just in time
for its delivery phase rather than as one broad credential setup.

[Back to the Tool CLI plan](README.md)

## Purpose

Stage only the external ownership, signing, and publication access needed for
the Tool release phase currently being implemented.

For the first public release, this runbook supplies B03's exact requirements to
the [B06 consolidated readiness checklist](../B06-coordinated-public-release/accounts-and-permissions.md).
B06 records readiness and coordinates invocation without receiving Tool
credentials.

## Canonical Identities

| Purpose | Identity |
| --- | --- |
| Release workflow | `.github/workflows/release-tool.yml` |
| Cargo packages | `liknon-tool` and supporting packages selected by A01 |
| Signed tag | `liknon-tool-v<VERSION>` |
| GitHub metadata release | Exact signed tag |
| Native download channel | Tool-specific immutable Cloudflare prefix |
| Human approval | Credential-free protected environment |

Tool publication uses no Hub resource-publication or namespace credential.

## Credential Boundaries

- Build, test, package inspection, and end-to-end verification have no
  publication credential.
- Reusable CLI-distribution workflows receive no inherited product secret; the
  B03 caller binds only the isolated Tool authority required by the selected
  publication job.
- crates.io, GitHub, Cloudflare, detached metadata signing, and platform signing
  use separate protected environments.
- The human release owner controls the signed tag and final approval.
- Bootstrap tokens are narrow, temporary, and revoked after use.
- No secret enters a binary, archive, log, cache key, or retained public
  evidence.

## Checkpoints

1. Confirm `liknon-tool` and supporting-package ownership plus tag
   ownership before B03.0.
2. Create credential-free release candidates during B03.3.
3. Configure protected publication environments only when candidates pass.
4. Activate destination credentials immediately before B06 invokes the B03.4
   publication procedure for the first release.
5. Let B06 coordinate public readback, then revoke temporary credentials and
   retain B03-owned evidence.

Tool storage authorization cannot write Validation, Manager, or suite release
state. Use a separate bucket or an equally strong boundary when authorization
cannot be scoped to a product prefix.

## Exit Criterion

Tool publication has independent ownership, minimal credentials, and no ability
to alter Hub resources, Manager releases, or Validation CLI releases.
