# Phase D01.0: Execute The End-To-End Scenario Matrix

## Status

`Provisional`. Scenarios become executable only after their owning product and
workflow contracts are implemented.

[Back to Plan D01](README.md)

## Scenario Families

| Scenario | Context supplied | Primary proof |
| --- | --- | --- |
| Direct human operation | Validation CLI and public contract | Complete human use without AI resources or policy |
| CLI-only agent discovery | Executable name and in-band CLI output | Self-discovery through help, diagnostics, schemas, and introspection |
| Documentation-only direct agent | Public CLI documentation, no AI resources | Direct use from public contracts without workflow guidance |
| Official action workflow | C01 bootstrap/policy plus exact C02 resources | Bounded governed assistance through replaceable resources |
| Validation-engineering setup | C03 Domain and Sensorium nodes | Domain evidence, coverage design, artifacts, and continuation |
| Validation-engineering operation | C03 Incident and Drift nodes | Report routing, remediation discipline, drift, and resumption |
| External workflow conformance | Neutral workflow and operational resources | Generic contracts without official-ID branches |

Each scenario declares exact versions, fixture digest, context boundary,
permissions, network policy, human-decision script, expected deterministic
assertions, and evidence destination.

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
