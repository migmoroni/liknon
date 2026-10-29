# Liknon Tool

`liknon-tool` is the offline, read-only interface to resources already installed
in a Liknon workspace. It lets humans, editors, agents, and automation discover
resource metadata first, retrieve exact content deliberately, and assemble a
bounded context without contacting the Hub or executing resource instructions.

The package installs two equivalent commands:

- `liknon-tool`, the explicit command;
- `likt`, the compact command.

## Responsibilities

Tool reads the nearest `.liknon/resources/` store, or a workspace selected with
`--workspace`. Before returning content, it validates the manifest and lock,
locked metadata, path containment, compatibility, revocation state, payload
length, and SHA-256 digest. Hub-backed state is also authenticated against the
catalog evidence retained by Liknon Manager.

Tool does not:

- contact a Hub or any other network service;
- install, update, remove, or materialize resources;
- execute validation, workflows, commands, scripts, agents, or models;
- write resource, policy, persistence, or workspace files;
- grant instruction authority to retrieved content.

Use `liknon-manager` when resource state must change. Use `liknon-validator`
when configured workspace checks must run.

## Install From Source

From the Liknon repository root:

```sh
cargo install --path apps/liknon-tool --locked --force
liknon-tool --version
likt --version
```

Rust 1.87 or newer is required. Public registry and native installation are not
advertised until their protected release and public readback have completed.

## Commands

```text
liknon-tool catalog [filters] [--workspace <path>]
liknon-tool search <query> [filters] [--workspace <path>]
liknon-tool show <resource-id> [--file <path>] [--encoding text|base64] [--workspace <path>]
liknon-tool resolve <resource-id>... [--workspace <path>]
liknon-tool context <resource-id>... [--include-dependencies] [--max-files <count>] [--max-bytes <bytes>] [--workspace <path>]
```

`catalog` returns bounded routing metadata without reading payload files.
`search` performs deterministic case-insensitive matching over installed
metadata. `show` returns one exact resource and an unambiguous or explicitly
selected payload. Binary payloads require `--encoding=base64`.

`resolve` returns the locked dependencies-first closure of explicit roots.
`context` returns verified text, binary references, provenance, trust,
compatibility, dependency metadata, and explicit omissions under caller-owned
file and byte limits. A collection root always resolves its locked members;
other roots include dependencies only with `--include-dependencies`.

Examples:

```sh
liknon-tool catalog --kind knowledge
liknon-tool search reproducible --namespace official
liknon-tool show knowledge:official/release-evidence
liknon-tool resolve collection:official/rust-validation
liknon-tool context collection:official/rust-validation \
  --max-files 32 \
  --max-bytes 1048576 \
  --format=json
```

Run `liknon-tool --help` or `liknon-tool <command> --help` for the complete
grammar and accepted filters.

## Output Contract

Human output is plain by default and supports the shared Liknon `--color` and
`--presentation=standard|low-vision` controls. `NO_COLOR` is honored unless an
explicit color option is supplied. Untrusted payload control characters are
escaped before terminal rendering.

`--format=json` emits a versioned, ANSI-free envelope. JSON preserves exact
UTF-8 payload content and keeps resource content separate from provenance,
trust, compatibility, integrity, and the explicit instruction boundary.
Visual options cannot be combined with machine-readable output.

Successful context assembly reports `complete` or `partial`. `partial` means
caller-selected limits omitted payloads; each omission remains explicit and no
security metadata is silently removed.

## Safety And Trust

An `official`, `verified`, `community`, or `local` trust class describes origin
and review evidence. It does not make payload text a system, host, policy,
repository, or human instruction. Tool returns resources as untrusted data and
never follows instructions contained in them.

Discovery operations intentionally do not read payload bytes. Content
operations verify every returned payload. Missing, modified, unsafe,
incompatible, or security-revoked content fails closed.

## Exit Codes

| Code | Meaning |
| ---: | --- |
| `0` | The requested read completed; inspect `outcome` for complete or partial context |
| `2` | Retrieval is blocked, or Clap rejected command-line syntax |
| `3` | Tool rejected the invocation or workspace selection |
| `4` | Persisted metadata or content failed integrity validation |
| `5` | Tool could not complete an internal or operating-system operation |

The resource protocol and store layout are documented in
[`../../docs/resources/README.md`](../../docs/resources/README.md).
