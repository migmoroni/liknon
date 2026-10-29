# Phase D01.0: Execute The End-To-End Scenario Matrix

## Status

`Provisional`. Scenarios become executable only after their owning product and
workflow contracts are implemented.

[Back to Plan D01](README.md)

## Scenario Families

| Scenario | Context supplied | Primary proof |
| --- | --- | --- |
| Direct human operation | Validation CLI and public contract | Complete human use without AI resources or policy |
| Direct and unified CLI parity | B01-B04 integrated candidates | Equivalent capability behavior with launcher isolation and explicit failure states |
| CLI-only agent discovery | Executable name and in-band CLI output | Self-discovery through help, diagnostics, schemas, and introspection |
| Documentation-only direct agent | Public CLI documentation, no AI resources | Direct use from public contracts without workflow guidance |
| Official action workflow | C01 bootstrap/policy plus exact C02 resources | Bounded governed assistance through replaceable resources |
| Validation-engineering setup | C03 Domain and Sensorium nodes | Domain evidence, coverage design, artifacts, and continuation |
| Validation-engineering operation | C03 Incident and Drift nodes | Report routing, remediation discipline, drift, and resumption |
| External workflow conformance | Neutral workflow and operational resources | Generic contracts without official-ID branches |

Each scenario declares exact versions, fixture digest, context boundary,
permissions, network policy, human-decision script, expected deterministic
assertions, and evidence destination.

## Direct And Unified Route Matrix

When B04 is implemented, require future tests for:

- every direct capability command: `liknon-validator`, `liknon-manager`, and
  `liknon-tool`;
- every `validator|v`, `manager|m`, and `tool|t` launcher namespace through
  `liknon`;
- exact argument-boundary, environment, working-directory, stdin, stdout,
  stderr, JSON, exit-status, timeout, and interruption equivalence;
- root help that remains launcher-focused and reports missing suite components
  deterministically;
- capability-specific help reached through namespace dispatch and emitted by
  the selected component;
- launcher-first root version output covering all three components in the
  approved order through the same trusted resolution used for dispatch;
- explicit absent, incompatible, invalid, unresponsive, shadowed, and replaced
  component states;
- parity between each direct capability executable and its launcher route;
- argument-boundary and executable-path attacks;
- direct capability operation without routing through the launcher;
- proof that the launcher performs no network, validation, resource mutation,
  resource interpretation, policy, persistence, or AI action; and
- explicit rejection of unknown or implicit routes such as
  `liknon validate ...` and `liknon check ...`.

These tests consume B04 candidates. D01 neither chooses the launcher contract
nor publishes the products under test.

## Cross-Scenario Rules

- Baseline scenarios do not receive hidden workflow or skill instructions.
- Governed scenarios load and record the bootstrap contract before replaceable
  resources.
- Comparable prompts, fixtures, and permissions remain aligned where the same
  task is measured across families.
- No scenario consumes transcripts or artifacts from another unless the
  declared handoff itself is under test.
- Product publication success is evidence supplied by B plans, not performed
  here.

## Exit Criterion

Every supported consumer shape has at least one passing representative scenario
and every unsupported or unsafe path has one deterministic failure scenario.
