# Liknon

`liknon` runs declarative validation graphs for heterogeneous
workspaces. It validates the complete configuration before starting a process,
checks tool versions, executes argument vectors without an implicit shell,
applies timeouts, preserves deterministic ordering, detects Git-visible
mutations, and emits accessible human output or a versioned JSON report.

The name comes from the ancient liknon, a winnowing basket used to separate
grain from chaff, reflecting the tool's role in separating accepted validation
evidence from failures and blockers.

The same pipeline is available as a CLI and as a Rust library.

## Installation

### From crates.io

Install the published binary with Cargo:

```sh
cargo install liknon --locked
```

Rust 1.87 or newer is required.

### From a source checkout

Install the exact code from a cloned repository without waiting for a crates.io
release:

```sh
git clone https://github.com/migmoroni/liknon.git
cd liknon
cargo install --path . --locked --force
liknon --version
```

`--force` replaces an existing installed copy even when it has the same crate
version. Cargo copies the compiled binary into its installation root, normally
`~/.cargo/bin`; it does not keep the command linked to the checkout. Run the
`cargo install --path . --locked --force` command again after updating or
modifying the cloned source. Ensure Cargo's binary directory is present in
`PATH` before invoking `liknon` directly.

## Quick Start

Author a complete candidate such as `validation.config.json` at the workspace
boundary:

```json
{
  "schemaVersion": 6,
  "workspaceRoot": "..",
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

Validate and install that exact candidate, inspect it, and run the default
group:

```sh
liknon init --config validation.config.json
liknon config validate
liknon list --tree
liknon validate
```

Use `--format=json` when a machine-readable report is required:

```sh
liknon validate --format=json
```

## Embedded Validation Knowledge

The binary includes a read-only, source-backed knowledge base for selecting and
interpreting validation evidence. Inspect its compact catalog, then retrieve
only the documents relevant to the current question:

```sh
liknon knowledge catalog --format=json
liknon knowledge show <document-id>
```

These commands work without configuration or workspace initialization and
never execute a configured program. Catalog entries marked `draft` may contain
only stable identity, kind, path, and status while their routing metadata is
being authored. Entries marked `reviewed` contain the complete catalog
metadata required by the knowledge schema. The status describes editorial
readiness; it does not make guidance universal project policy.

## Agent Skill

The source distribution includes one versioned, agent-neutral skill at
[`skills/liknon`](skills/liknon). Its `SKILL.md` is a
small router that checks CLI compatibility and loads only the requested
workflow reference: execution, triage, configuration, or coverage auditing.
When a task needs source-backed reasoning about evidence, the router uses
`liknon knowledge catalog --format=json` and then
`liknon knowledge show <document-id>` for only the exact matching
guides. It does not infer metadata omitted from a draft entry. The canonical
human-readable source remains available at
[`docs/validation/knowledge`](docs/validation/knowledge/README.md); the skill
contains no copied knowledge tree.

Copy the complete directory from the source release that matches the installed
CLI into the skill directory recognized by the consumer workspace. For an
agent that discovers workspace skills under `.agents/skills`, run this from a
checkout of `liknon`:

```sh
workspace_root=/path/to/workspace
mkdir -p "$workspace_root/.agents/skills"
cp -R skills/liknon "$workspace_root/.agents/skills/"
```

The source path is not tied to a specific model or agent. Use a different
destination when the consumer uses another skill discovery convention.
Installing the Cargo binary does not register the skill automatically.

Copy the whole directory so `manifest.json` and the `references` remain on the
same version. `manifest.json` is the only source for the bundle version, source
crate version, and compatible CLI range. The router warns about a stale but
compatible copy and stops workflows outside its declared compatibility range.
Replace the copied directory from the matching release to update it;
keep project-specific commands and approval rules in a separate local skill or
agent policy.

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

## Commands

```text
liknon init --config <candidate-path> [--workspace <path>] [--format=human|json]
liknon config validate [--config <path>]
liknon validate [group-or-suite] [--config <path>] [--format=human|json]
liknon check <check-id> [--config <path>] [--format=human|json]
liknon list [--tree] [--config <path>]
liknon explain group <group-id> [--config <path>]
liknon explain suite <suite-id> [--config <path>]
liknon explain check <check-id> [--config <path>]
liknon schema config
liknon schema report
liknon knowledge catalog [--format=human|json]
liknon knowledge show <document-id>
```

Without `--config`, the CLI discovers the nearest
`.validation/config.json` by walking from the current directory upward.
`validate` without a target selects `defaultGroup`. `config validate`, `list`,
and `explain` inspect configuration without running preflight or checks.
`init` performs no discovery and installs only explicitly requested resources;
an omitted `--workspace` means exactly the process current directory.
Knowledge commands read only assets embedded in the binary and do not discover
configuration or require an initialized workspace.

The focused [Human Flow](docs/validation/flows/human/README.md) walks through
authoring, deterministic initialization, inspection, execution, interpretation,
and narrow revalidation. Stable command and contract details live under
[the validation reference](docs/validation/reference/README.md).

Exit codes are stable CLI behavior:

| Code | Meaning |
| ---: | --- |
| `0` | The request completed with a positive result |
| `1` | The request completed with negative validation evidence or an initialization conflict/partial result |
| `2` | Validation was blocked or skipped without a failure |
| `3` | Usage, configuration, candidate input, or a provisioning precondition was rejected before configured execution |
| `4` | An internal execution, provisioning, or reporting operation failed |
| `130` | An interrupt was observed during validation, regardless of the report aggregate |

The [exit-status reference](docs/validation/reference/exit-status.md) defines
these process-composition outcomes precisely.

## Human And JSON Output

Human CLI output is plain by default, including short help, long help, parser
diagnostics, and validation reports. This default follows `NO_COLOR` without
requiring terminal detection. An explicit global `--color` overrides
`NO_COLOR` and selects the standard palette; `--color=<palette>` selects
`high-contrast`, `protanopia`, `deuteranopia`, `tritanopia`, or
`achromatopsia`. Layout and emphasis are independently selected through
`--presentation=<mode>`; `low-vision` expands spacing, removes dim styling, and
may use ANSI typographic emphasis without adding hue. Color never carries the
only indication of status, node type, hierarchy, or errors.

With `-h` or `--help`, visual options style the requested help. The root help
documents these renderer controls in full. Focused `validate` and `check` help
only indicates that they are available because they also style normal human
execution. Each focused help documents its own `-h, --help`; the consolidated
root reference documents that option once instead of repeating it in every
embedded command section. Other commands accept visual options only when
rendering help, preventing successful command output from silently ignoring
them.

```sh
liknon --color -h
liknon --color=high-contrast --help
liknon config --presentation=low-vision --help
liknon validate --color=deuteranopia --help
liknon validate --color=high-contrast
liknon validate --color=deuteranopia --presentation=low-vision
liknon validate --presentation=low-vision
```

`--format=json` emits only the version 4 `ValidationReport`; visual options are
therefore rejected in JSON mode. Visual options without help are also rejected
for commands other than `validate` and `check`. Reports keep tools, groups,
suites, concrete checks, and the optional repository gate structurally
distinct. Captured output is bounded by `outputLimitBytes` for each stream.

## Library Usage

Loading and planning establish invariants before execution. Their resulting
types expose read-only inspection rather than public constructors or mutable
fields.

```rust,no_run
use std::{
    path::Path,
    sync::{atomic::AtomicBool, Arc},
};
use liknon::{config, execution, planning, reporting};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let current = std::env::current_dir()?;
let validated = config::load(Some(Path::new(".validation/config.json")), &current)?;
let plan = planning::target(&validated, None)?;
let outcome = execution::run(&validated, &plan, Arc::new(AtomicBool::new(false)));
let json = reporting::result::json::render(&outcome.report)?;
println!("{json}");
# Ok(())
# }
```

`execution::run_with_progress` accepts an implementation of
`execution::progress::ProgressReporter` for typed lifecycle events. The
checked-in [`examples/inspect.rs`](examples/inspect.rs) demonstrates
non-destructive configuration and plan inspection. Human renderers consume
`theme::Theme`; palette and presentation profiles remain composable and usable
independently from the CLI.

The Rust API, CLI commands and exit codes, schema shapes, JSON report, execution
semantics, palette names, and presentation names are observable contracts.
While the crate is `0.x`, incompatible public API changes raise the minor
version and compatible fixes raise the patch version. Cargo versions and JSON
schema versions evolve independently.

## Security And Trust Model

Configuration is trusted executable input. It selects programs and arguments
that run with the permissions of the process invoking `liknon`.
Execution without an implicit shell prevents accidental shell interpretation;
it does not make an untrusted configuration safe.

Review `.validation/config.json` before validating a third-party repository.
The validator does not install dependencies, modify source intentionally, or
apply fixes. Optional Git-integrity detection reports Git-visible mutations,
but it is not an operating-system sandbox and cannot observe every external
side effect.

Private vulnerability reports follow [`SECURITY.md`](SECURITY.md).

## Compatibility And Platforms

The supported platforms are Linux, macOS, and Windows. Functional behavior
includes executable resolution, suite working directories, bounded stream
capture, exit codes, timeout, cancellation, descendant-process termination,
human reports, JSON reports, and Git repository integrity.

The minimum supported Rust version is 1.87. Configuration schema version 6 and
report schema version 4 are exact contracts; unsupported schema versions are
rejected rather than inferred or converted.

## Schemas, Design, And License

Published package contents include:

- the configuration, report, and knowledge contracts under the
  [`schemas` directory](schemas/);
- the human flow, stable reference, embedded knowledge source, and maintainer
  guidance under [`docs/validation`](docs/validation/README.md);
- the agent-neutral workflow bundle under
  [`skills/liknon`](skills/liknon/SKILL.md);
- [`CONFIG_DESIGN.md`](CONFIG_DESIGN.md);
- [`CHANGELOG.md`](CHANGELOG.md);
- [`SECURITY.md`](SECURITY.md);
- [`LICENSE`](LICENSE).

The crate is distributed under the MIT License.

## Repository Development

From the source repository, replace the installed binary in examples with:

```sh
cargo run --locked -- <command>
```

CI treats the committed `Cargo.lock` as the reproducible dependency graph for
the CLI. `cargo audit` rejects known RustSec advisories, while `cargo deny`
applies the license, source, and duplicate-dependency policy in
[`deny.toml`](deny.toml). The package job verifies the isolated crate archive
and smoke-tests the packaged binary. The CI workflow never publishes a package.

Knowledge maintainers verify the machine-readable contracts, declared assets,
references, recipe configurations, generated source index, and deterministic
tree digest with:

```sh
cargo run --locked --example knowledge_release -- --check
```

Use `--write` only when `sources.json` changed and `SOURCES.md` must be
regenerated. Editorial quality remains a human review responsibility.
