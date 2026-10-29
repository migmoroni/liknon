<p align="center">
  <img src="https://raw.githubusercontent.com/migmoroni/liknon/main/assets/branding/liknon.svg" width="200" alt="Liknon logo">
</p>

# Liknon

Liknon is a developing suite for declarative validation of heterogeneous
workspaces. Its implemented validation application checks the complete
configuration before starting a process, verifies tool versions, executes
argument vectors without an implicit shell, applies timeouts, preserves
deterministic ordering, detects Git-visible mutations, and emits accessible
human output or a versioned JSON report.

The name comes from the ancient liknon, a winnowing basket used to separate
grain from chaff.

## Current Products

The public command-line product is the `liknon-cli` suite. One package owns
exactly four physical executables:

| Executable | Current responsibility |
| --- | --- |
| `liknon` | Capability-free launcher for one explicit suite namespace |
| `liknon-validator` | Declarative workspace validation |
| `liknon-manager` | Authenticated resource discovery, resolution, installation, verification, and materialization |
| `liknon-tool` | Verified offline discovery, retrieval, resolution, and bounded context for installed resources |

The capability executables remain independent processes and can be invoked
directly. The launcher resolves only its sibling executables and routes the
explicit namespaces `validator|v`, `manager|m`, and `tool|t`. The short forms
are parser namespaces, not additional command files.

Supporting crates preserve implementation boundaries without owning public
executables:

| Package | Current responsibility |
| --- | --- |
| `liknon-validator` | Validation engine and capability CLI composition |
| `liknon-manager` | Resource acquisition and mutable workspace-state behavior |
| `liknon-tool` | Read-only local resource consumption behavior |
| `liknon-cli-ui` | Product-neutral palettes, presentation, terminal policy, Clap styling, and generic help composition |
| `liknon-resource-contracts` | Resource, catalog, trust, manifest, lock, and signature contracts |

The applications share the terminal-facing foundation documented in
[`docs/cli-experience.md`](docs/cli-experience.md), while retaining their own
commands, domain renderers, machine contracts, errors, and exit codes. The
internal Rust surfaces are not promised programmatic integration APIs.

## Install From Source

No public package release is advertised at this stage. Clone the repository
and install the complete suite from one package:

```sh
git clone https://github.com/migmoroni/liknon.git
cd liknon
cargo install --path apps/liknon-cli --locked --force
liknon --version
liknon-validator --version
liknon-manager --version
liknon-tool --version
```

Rust 1.87 or newer is required. Cargo normally writes the executables to
`~/.cargo/bin`; ensure that directory is in `PATH`. The installed executable is
a compiled copy, not a link to the checkout, so reinstall it after updating or
modifying the source.

## Release Distribution

The repository contains one release profile for the complete CLI suite across
a signed GitHub source tag, one crates.io source package, and one authenticated
target-native archive per supported target on a project-controlled Cloudflare
domain. Every channel installs the same four executable roles at one version.
Registry and native channels are not advertised as live until their protected
release and public-readback gates complete.

The installation, update, uninstall, platform, and trust contract is documented
in [`docs/distribution/cli-suite.md`](docs/distribution/cli-suite.md). Candidate
preparation and the later B06 publication handoff are documented in
[`docs/distribution/releasing-cli-suite.md`](docs/distribution/releasing-cli-suite.md).
GitHub Releases intentionally carry metadata only and no Liknon binaries.

## Resource Manager

Manager creates and maintains the consumer-owned `.liknon/resources/` store.
It authenticates signed Hub catalogs against a separately obtained trust root,
resolves semantic-version dependency graphs, retains exact catalog evidence for
offline verification, and applies resource-store and materialization changes
as one recoverable transaction.

```sh
liknon-manager init
liknon-manager search <query> \
  --hub <https-origin> \
  --trust-root <verified-trust-root.json>
liknon-manager install 'knowledge:<namespace>/<name>@^1' \
  --hub <https-origin> \
  --trust-root <verified-trust-root.json> \
  --dry-run
liknon-manager verify --offline
```

Manager never executes resource content or activates workflows. No production
Hub origin is advertised before protected publication and public readback have
passed. The protocol, Hub layout, trust model, and complete operating contract
start at [`docs/resources/README.md`](docs/resources/README.md).

## Resource Tool

Tool reads installed state without contacting the Hub or changing the
workspace. Discovery commands expose bounded metadata without loading payload
files; content commands verify locked paths, compatibility, revocation, byte
lengths, and SHA-256 digests before returning content.

```sh
liknon-tool catalog --kind knowledge
liknon-tool search reproducible
liknon-tool show knowledge:<namespace>/<name>
liknon-tool resolve collection:<namespace>/<name>
liknon-tool context collection:<namespace>/<name> \
  --max-files 32 \
  --max-bytes 1048576 \
  --format=json
```

Returned resource text remains untrusted data and never gains system, host,
policy, repository, or human authority through retrieval. The complete command,
output, safety, and exit contract is documented in
[`apps/liknon-tool/README.md`](apps/liknon-tool/README.md).

## Quick Start

Author a complete candidate such as `validation.config.json` at the workspace
root. Because initialization validates it at its canonical destination under
`.liknon/validation/`, `workspaceRoot` uses `../..` to select the consumer
workspace root:

```json
{
  "schemaVersion": 6,
  "workspaceRoot": "../..",
  "defaultGroup": "all",
  "outputLimitBytes": 1048576,
  "tools": [
    {
      "id": "cargo",
      "program": "cargo",
      "requiresTools": [],
      "versionArgs": ["--version"],
      "versionParser": "firstSemver",
      "versionRequirement": ">=1.87"
    }
  ],
  "checks": [
    {
      "id": "rust.cargo.check",
      "label": "Cargo check",
      "description": "Checks selected Rust targets.",
      "toolId": "cargo",
      "args": ["check", "{target}"],
      "requiresTools": [],
      "timeoutSeconds": 600
    }
  ],
  "suites": [
    {
      "id": "rust",
      "label": "Rust",
      "description": "Checks the Rust workspace.",
      "workingDirectory": ".",
      "checks": [
        {
          "checkId": "rust.cargo.check",
          "parameters": {
            "target": ["--workspace", "--all-targets"]
          },
          "dependsOn": []
        }
      ]
    }
  ],
  "groups": [
    {
      "id": "all",
      "label": "All validations",
      "description": "Runs the complete workspace gate.",
      "members": [
        { "kind": "suite", "id": "rust" }
      ]
    }
  ]
}
```

Validate and install those exact bytes, inspect the resulting configuration,
and run its default group:

```sh
liknon validator init --config validation.config.json
liknon validator config validate
liknon validator list --tree
liknon validator validate
```

Use `--format=json` for a machine-readable report:

```sh
liknon validator validate --format=json
```

## Configuration Model

The version 6 configuration has four ownership layers:

```text
tool -> check -> suite -> group
```

- A **tool** declares executable discovery and version validation.
- A **check** declares one reusable program and argument template.
- A **suite** binds parameters, dependencies, and one working directory to
  check invocations.
- A **group** composes suites and other groups into an ordered DAG.

An argument formed entirely as `{name}` is a structured placeholder. A suite
parameter string emits one argument, an array emits multiple arguments, and an
unbound placeholder emits no argument. Values are never split, recursively
expanded, interpolated inside another string, or interpreted by a shell.

Only suites own `workingDirectory`. Paths are resolved below `workspaceRoot`;
absolute paths, parent traversal, missing directories, files, and symlink
escapes are rejected before preflight. Group references form a DAG, and shared
groups or suites execute once per validation run.

The normative ownership and extension rules are documented in
[`CONFIG_DESIGN.md`](CONFIG_DESIGN.md).

## Validation Commands

```text
liknon-validator init --config <candidate-path> [--workspace <path>] [--format=human|json]
liknon-validator config validate [--config <path>]
liknon-validator validate [group-or-suite] [--config <path>] [--format=human|json]
liknon-validator check <check-id> [--config <path>] [--format=human|json]
liknon-validator list [--tree] [--config <path>]
liknon-validator explain group <group-id> [--config <path>]
liknon-validator explain suite <suite-id> [--config <path>]
liknon-validator explain check <check-id> [--config <path>]
liknon-validator schema config
liknon-validator schema report
```

The same commands are available through the launcher by inserting the
`validator` namespace after `liknon`, for example
`liknon validator validate --format=json`. Direct and routed execution select
the same installed capability executable.

Without `--config`, the CLI discovers the nearest
`.liknon/validation/config.json` by walking from the current directory upward.
`validate` without a target selects `defaultGroup`. `config validate`, `list`,
and `explain` inspect configuration without running preflight or checks. `init`
performs no discovery and installs only the explicitly requested configuration;
an omitted `--workspace` means exactly the process current directory.

The focused [Human Flow](docs/validation/flows/human/README.md) covers
authoring, deterministic initialization, inspection, execution,
interpretation, and narrow revalidation. Stable command and contract details
live under [the validation reference](docs/validation/reference/README.md).

Exit codes are stable validation-CLI behavior:

| Code | Meaning |
| ---: | --- |
| `0` | The request completed with a positive result |
| `1` | The request completed with negative validation evidence or an initialization conflict/partial result |
| `2` | Validation was blocked or skipped without a failure |
| `3` | Usage, configuration, candidate input, or a provisioning precondition was rejected before configured execution |
| `4` | An internal execution, provisioning, or reporting operation failed |
| `130` | An interrupt was observed during validation, regardless of the report aggregate |

## Human And JSON Output

Human output is plain by default, including help, parser diagnostics, and
validation reports. An explicit global `--color` overrides `NO_COLOR` and
selects the standard palette; `--color=<palette>` also supports
`high-contrast`, `protanopia`, `deuteranopia`, `tritanopia`, and
`achromatopsia`. Layout and emphasis are selected independently through
`--presentation=standard|low-vision`. Color never carries the only indication
of status, node type, hierarchy, or errors.

With `-h` or `--help`, visual options style the requested help. During command
execution they apply only to human `validate` and `check` output. JSON mode
rejects visual options and emits only the version 4 `ValidationReport`.

```sh
liknon-validator --color=high-contrast --help
liknon validator config --presentation=low-vision --help
liknon validator validate --color=deuteranopia --presentation=low-vision
liknon validator validate --format=json
```

## Knowledge And AI Material

The canonical validation guidance remains under
[`docs/validation/knowledge`](docs/validation/knowledge/README.md). It is
authored repository content and is not embedded in `liknon-validator` or exposed
through a validation command at the A01 boundary.

The source under [`skills/liknon`](skills/liknon) is preserved only as
non-active design input for C02. Its explicit handoff is recorded in
[`docs/validation/authoring/skill-handoff.md`](docs/validation/authoring/skill-handoff.md).
Do not distribute it as current CLI guidance before that plan reconciles and
activates it.

Repository maintainers validate the knowledge contracts, references, recipes,
generated source index, and deterministic tree digest with:

```sh
cargo run -p liknon-knowledge-maintenance --locked -- --check
```

Use `--write` only after changing `sources.json` when `SOURCES.md` must be
regenerated. Editorial quality remains a human review responsibility.

## Security And Trust

`.liknon/validation/config.json` is trusted executable input. It selects
programs and arguments that run with the caller's operating-system
permissions. Execution without an implicit shell prevents accidental shell
interpretation; it does not make an untrusted configuration safe.

The validator does not install dependencies, modify source intentionally, or
apply fixes. Optional Git-integrity detection reports Git-visible mutations,
but it is not an operating-system sandbox and cannot observe every external
side effect. Review third-party configuration before execution.

Private vulnerability reports follow [`SECURITY.md`](SECURITY.md).

## Compatibility And Schemas

The supported platforms are Linux, macOS, and Windows. The minimum supported
Rust version is 1.87. Configuration schema version 6 and report schema version
4 are exact contracts; unsupported versions are rejected rather than inferred
or converted.

The validation-owned schemas live with their owner at
[`apps/liknon-validator/schemas`](apps/liknon-validator/schemas). Inspect
the contract generated by the running executable with:

```sh
liknon-validator schema config
liknon-validator schema report
```

Knowledge authoring schemas live under
[`docs/validation/authoring/schemas`](docs/validation/authoring/schemas), while
evaluation-only schemas live under [`evals/schemas`](evals/schemas).

## Repository Development

Run the suite launcher from the workspace with:

```sh
cargo run -p liknon-cli --bin liknon --locked -- validator <command>
```

The committed root `Cargo.lock` is the reproducible dependency graph. The root
workspace owns formatting, lint, tests, doctests, and package-boundary checks;
nested fixtures remain excluded workspace inputs.

Release maintainers use the reviewed suite profile under
[`distribution/cli/suite`](distribution/cli/suite) and its runbook under
[`docs/distribution`](docs/distribution).
Generated release candidates remain below `target/distribution/`.

Liknon is licensed under the [MIT License](LICENSE).
