# Changelog

All notable changes to `workspace-validator` are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and the project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Versioned agent skill bundle for safe validation execution, report triage,
  configuration, and coverage auditing with progressive disclosure.
- Read-only embedded validation knowledge with catalog discovery and exact
  document retrieval through the CLI.
- A technical knowledge release checker for schemas, references, declared
  assets, recipe configurations, generated source indexes, embedded inventory,
  and deterministic tree evidence.

### Changed

- Visual themes are shared across the CLI, so explicit accessible palettes
  style short help, complete help, parser diagnostics, and validation reports
  while plain ANSI-free output remains the default.
- CLI help describes every command, nested command, positional argument,
  option, default, interaction, and accepted enum value at its corresponding
  help level; root `--help` consolidates the complete command tree while root
  `-h` remains compact.
- Knowledge catalog entries use minimal structural metadata while `draft` and
  require complete routing and review metadata when marked `reviewed`.
- Recipe release validation accepts relative `workspaceRoot` layouts while
  confining materialized examples to an isolated temporary workspace.

## [0.1.0]

### Added

- Declarative version 6 configuration for tools, checks, suites, and groups.
- Shell-free command execution with structured parameters and suite working
  directories.
- Deterministic validation DAGs with timeout, cancellation, bounded output,
  dependency handling, and process-tree termination.
- Accessible human progress and completed reports with composable palettes and
  low-vision presentation.
- Version 4 JSON reports and checked-in JSON Schemas.
- Optional Git repository-integrity gate.
- Rust library API for configuration, planning, execution, progress events,
  reports, and themes.
- Supported execution on Linux, macOS, and Windows with portable process
  fixtures and descendant-process termination.
