# Plan E01: Liknon Public Site And Hub Delivery

## Status

`Decision gate`. The domain `liknon.org` is available, the repository-side Hub
implementation exists, and this plan selects the public topology that connects
the human site, the Resource Hub, and existing product-download channels.

No production DNS, trust root, resource snapshot, or site deployment is implied
until E01.0 is approved and the applicable B02 and B06 gates pass.

[Back to the validation-system roadmap](../README.md)

## Purpose

Provide two deliberately separate public surfaces:

1. a human-facing website for product identity, documentation, downloads,
   resource discovery, trust information, and installation guidance; and
2. a machine-facing authenticated static Hub from which `liknon-manager`
   obtains exact catalogs and immutable resource payloads.

The site improves discovery. It is not the Hub, a package registry, a Manager
proxy, or a second source of resource truth.

## Recommended Initial Architecture

```text
reviewed source in the Liknon repository
                  |
                  v
      deterministic Hub assembly + signing
                  |
                  v
       immutable Cloudflare R2 objects
                  |
          +-------+-------+
          |               |
          v               v
 hub.liknon.org      verified site build
          |               |
          v               v
  liknon-manager        liknon.org
```

Use these public boundaries:

| Surface | Initial destination | Responsibility |
| --- | --- | --- |
| `https://liknon.org` | `apps/liknon-site` deployed as static assets on Cloudflare Workers | Human documentation, product discovery, resource catalog presentation, and links |
| `https://www.liknon.org` | Permanent redirect | Redirect to the apex site |
| `https://hub.liknon.org` | Dedicated Cloudflare R2 custom domain | Signed Hub pointer, snapshots, and immutable payload objects |
| `https://downloads.liknon.org` | B04 CLI-suite R2 boundary | One complete native suite archive per target, manifests, checksums, signatures, and installers |

The Hub and download origins may share one Cloudflare account, but they must
not share buckets, object prefixes, publication credentials, stable pointers,
or rollback authority.

Cloudflare currently recommends Workers Static Assets for new static-site
deployments. R2 production access uses a custom domain rather than `r2.dev`,
which is intended for development and lacks the same cache and security
controls:

- [Cloudflare Workers Static Assets](https://developers.cloudflare.com/workers/static-assets/)
- [SvelteKit adapters](https://svelte.dev/docs/kit/adapters)
- [Cloudflare R2 public buckets and custom domains](https://developers.cloudflare.com/r2/buckets/public-buckets/)

## Repository Decision

For the first public release, keep the canonical official resource sources in
this repository under a dedicated `resources/official/` source root. Keep
`fixtures/resource-hub/source/` exclusively for neutral tests.

GitHub is the authoring, review, and provenance plane. Neither the site nor
Manager may use raw GitHub files, mutable branches, or repository archive URLs
as the production Hub. The protected Hub workflow reads one exact signed source
tag, assembles one authenticated snapshot, and publishes the generated object
tree to R2.

A separate resource repository is deferred until independent maintainers,
third-party submissions, release cadence, repository size, or governance make
that boundary useful. Such a split changes only the reviewed source input and
cross-repository provenance. It must not change resource IDs, Hub URLs, object
layout, signatures, Manager state, or the public protocol.

## Site Application Boundary

Create the website at `apps/liknon-site/` as a static-first SvelteKit
application managed with pnpm. Prerender it with the official
`@sveltejs/adapter-static` adapter and deploy the generated assets through
Cloudflare Workers Static Assets. Introduce server-side behavior only when a
measured feature cannot be provided safely at build time.

The first site release should provide:

- a concise overview of the unified CLI suite and its three capability
  executables;
- installation and download pages that select one exact suite archive for the
  user's target;
- versioned user and protocol documentation selected from canonical repository
  sources;
- a resource catalog generated from one authenticated Hub snapshot;
- resource pages showing identity, version, publisher, trust class, license,
  compatibility, dependencies, capabilities, review state, and payload hashes;
- copyable Manager commands that begin with inspection or `--dry-run`;
- a security and trust page explaining the Hub root, signed catalogs, immutable
  objects, revocation, and reporting channels.

The site must not:

- copy or transform release binaries;
- proxy Manager traffic through a site API;
- publish unsigned resource metadata;
- infer trust from an `official` namespace alone;
- execute, activate, or silently materialize resource content;
- render untrusted payload HTML or scripts;
- require the site to be available for Manager to continue using the Hub.

## Hub Production Boundary

The generated Hub retains the existing static contract:

```text
stable.json
stable.json.sig.json
snapshots/<snapshot-id>/catalog.json
snapshots/<snapshot-id>/catalog.json.sig.json
objects/sha256/<first-two-hex>/<sha256>
```

Production publication must:

- use the existing deterministic assembler and public contracts;
- sign snapshots and pointers only inside the protected signing environment;
- publish immutable objects before advancing the stable pointer;
- reject replacement of a versioned object with different bytes;
- read every object back through `hub.liknon.org` before promotion;
- publish immutable objects with long-lived immutable caching;
- publish pointer objects with explicit revalidation and no stale promotion;
- disable the public `r2.dev` endpoint;
- expose no bucket listing or writable public operation;
- retain exact publication and rollback evidence without private key material.

Because Cloudflare cache can serve stale overwritten or deleted keys, immutable
objects are never repaired in place. Stable-pointer advancement and cache
purging follow one reviewed procedure. See
[R2 consistency](https://developers.cloudflare.com/r2/reference/consistency/).

## Trust Bootstrap

The Hub trust root must be distributed independently from the Hub bucket. E01.0
must select a versioned public path below `liknon.org`, the accepted key and
fingerprint representation, and the B02 Manager bootstrap contract.

The recommended contract is:

- publish versioned public trust-root bytes through the separately deployed
  site origin;
- include or pin the accepted root identity and digest in each applicable
  Manager release;
- show the same identity and fingerprint in human documentation;
- never allow a mutable site response by itself to grant a new Hub signing key;
- require an explicitly authorized rotation procedure that can verify both the
  previous and replacement root evidence.

This plan does not silently change the current Manager requirement for an
explicit Hub origin and trust root. B02 owns that CLI contract and must approve
any production default.

## Verified Site Integration

The site build consumes a retained or public Hub snapshot, not authored source
files directly. Before generating catalog pages, CI verifies the pointer,
catalog signature, resource contracts, payload inventory, and exact snapshot
identity with repository-owned tooling.

The generated site records the displayed snapshot ID and generation time. A
Hub publication may trigger a new site build only after the stable pointer has
passed public readback. A failed site rebuild does not roll back an already
valid Hub publication; it reports that the human catalog view is behind the
machine endpoint.

Initially, catalog pages expose authenticated metadata only. Displaying full
payload content is a separate safety decision and requires format-aware
sanitization, strict output limits, origin labeling, and tests against active
content.

## Security And Deployment Isolation

Use separate GitHub environments and least-privilege credentials for:

- `deploy-liknon-site`;
- `sign-liknon-hub`;
- `publish-liknon-hub`;
- each existing CLI publication boundary.

The site job receives no Hub signing seed or R2 write credential. The Hub
signing job receives no site deployment or R2 publication credential. The Hub
publication job receives only bucket-scoped object authority and cannot deploy
the site or publish CLI archives.

The site must ship a restrictive Content Security Policy, HSTS, MIME sniffing
protection, referrer and permissions policies, accessible semantic navigation,
and no third-party analytics or scripts without a separate privacy review.

## Implementation Units

| Unit | Scope | Exit result |
| --- | --- | --- |
| E01.0 | Approve domains, source ownership, trust bootstrap, framework, deployment authority, and release relationship | One accepted public topology with no competing source of truth |
| E01.1 | Create `apps/liknon-site`, pnpm workspace support, static build, preview deployment, design foundation, and security headers | Reproducible site candidate with no production credentials |
| E01.2 | Establish `resources/official/`, author the minimum first-release corpus, and prove deterministic Hub assembly | Reviewed production-form source candidate distinct from fixtures |
| E01.3 | Provision the dedicated R2 boundary, `hub.liknon.org`, caching, trust-root publication, protected environments, signing, and public readback | Authenticated production Hub candidate ready for coordinated activation |
| E01.4 | Generate resource catalog, documentation, trust, and download views from exact verified inputs | Site and Hub expose the same snapshot and product identities |
| E01.5 | Exercise failure, rollback, stale-site, cache, origin, accessibility, and clean-client scenarios | Release-ready site and Hub delivery evidence for B06 |

## Relationship To Existing Plans

- B02 continues to own the resource protocol, Manager, Hub assembly,
  signatures, publication mechanics, and Hub recovery.
- E01 owns the public domain topology, site application, human catalog view,
  and verified presentation of B02 outputs.
- B01-B03 own capability behavior and evidence; B04 owns the CLI package and
  native suite artifacts. E01 links only to exact B04 manifests and one
  complete archive per target.
- C02 and C03 may supply official resources after their own contracts pass;
  E01 grants them no special execution authority.
- B06 freezes and coordinates the first public activation of the exact E01 site
  candidate and B02 Hub candidate with the selected Liknon products.

## Deferred Work

- a dedicated resource-source repository;
- public publisher accounts and self-service submissions;
- dynamic catalog search services;
- browser-side installation or resource execution;
- arbitrary payload rendering;
- personalized accounts, telemetry, or analytics;
- a general-purpose package registry API.

## Acceptance Criteria

- [ ] `apps/liknon-site` is the sole site application source.
- [ ] `liknon.org` is human-facing and `hub.liknon.org` is machine-facing.
- [ ] Manager never depends on site HTML, JavaScript, or availability.
- [ ] GitHub source is reviewed provenance, not the runtime Hub origin.
- [ ] Official resource sources and neutral fixtures are physically distinct.
- [ ] One signed Hub snapshot produces the Manager view and site catalog view.
- [ ] The site publishes no metadata that failed Hub contract and signature
      verification.
- [ ] The trust root is independently delivered and cannot be replaced by an
      unauthenticated Hub or mutable site response.
- [ ] Site, Hub signing, Hub publication, and CLI publication credentials are
      isolated.
- [ ] Immutable Hub objects cannot be overwritten through the normal workflow.
- [ ] Public readback verifies exact bytes through the custom domains.
- [ ] Site failure cannot corrupt or disable a valid Hub.
- [ ] The first public activation is recorded and verified through B06.

## Result

Liknon has one content-focused public site and one independent authenticated
resource origin. Humans discover products and resources through `liknon.org`;
Manager obtains exact signed objects through `hub.liknon.org`; GitHub preserves
reviewed source and provenance without becoming a production CDN or a second
Hub.
