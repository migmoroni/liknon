# Manager Operating Contract

`liknon-manager` and `likm` are equivalent commands. Manager is the only Liknon
application that contacts the Resource Hub or mutates installed resource state.
It does not validate workspaces, answer knowledge questions, interpret content,
evaluate AI policy, or invoke another Liknon executable.

## Discovery And Initialization

Existing-state commands discover the nearest ancestor containing `.liknon/`,
or use an explicit `--workspace <path>`. Discovery does not depend on Git,
language manifests, environment managers, or agent directories.

`init` creates only an empty `.liknon/resources/manifest.json` and `lock.json`
foundation at the selected boundary. It is a no-op when a valid resource store
already exists and never resets populated state.

## Hub Access

Networked operations require:

- `--hub <https-origin>` or `LIKNON_HUB_URL`;
- `--trust-root <path>` or `LIKNON_HUB_TRUST_ROOT`.

`file://` and local paths exist for explicit local testing. HTTP is rejected.
HTTPS redirects stay on the original origin and are bounded. Metadata and
payload responses have size limits and request timeouts. The optional
`LIKNON_CACHE_DIR` is content-addressed, rejects unsafe links, and is always
reverified. Network requests have a bounded retry budget; initial transfer
concurrency is deliberately limited to one request.

## Mutation Contract

`install`, `update`, `remove`, and `materialize` resolve and present a complete
plan before commit. Use `--dry-run` to stop after planning. Use `--yes` only
after reviewing the plan when no interactive confirmation is available.

The plan identifies the selected catalog, requested constraints, exact graph,
resource and file actions, manifest and lock changes, potential downloads,
verified result bytes, declared effects, required decisions, conflicts, and
rollback boundary. Dry-run and commit use the same resolved plan.

The resource store, manifest, lock, retained authentication evidence, and
materializations change through one journaled transaction. Concurrent Manager
mutations are rejected. An interrupted operation is completed from exact
staged bytes or rolled back to the prior complete state; recovery never
re-resolves.

Materialization is explicit and workspace-relative. It rejects `.liknon/`,
unsafe ancestors, unowned destination bytes, and drifted Manager-owned bytes.
`list` reports resource and materialization state separately, including missing
and drifted bytes.

## Outputs And Exit Statuses

Human output is the default. `--format=json` emits a versioned typed report on
stdout with diagnostics on stderr and no terminal styling.

| Code | Meaning |
| ---: | --- |
| `0` | Operation completed or a plan was produced |
| `2` | Resolution or mutation was blocked |
| `3` | Usage, workspace discovery, or caller input was invalid |
| `4` | Network, authentication, contract, or integrity verification failed |
| `5` | Internal I/O or serialization failure |

AI workflows must evaluate their applicable policy before invoking Manager.
Manager intentionally does not parse or enforce conversational authority or
`.liknon/ai/policy.json`.
