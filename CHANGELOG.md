# Changelog

All notable changes to `liknon` are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and the project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.0] - 2026-10-06

### Added

- Canonical project artwork for package documentation and an accessible ASCII
  mark in the complete root help.
- Versioned agent skill bundle for safe validation execution, report triage,
  configuration, and coverage auditing with progressive disclosure.
- Read-only embedded validation knowledge with catalog discovery and exact
  document retrieval through the CLI.
- A technical knowledge release checker for schemas, references, declared
  assets, recipe configurations, generated source indexes, embedded inventory,
  and deterministic tree evidence.

### Changed

- The project, crate, library, CLI, workflow, fixtures, documentation, and
  bundled agent skill are now named `liknon`.
- Visual themes are shared across the CLI, so explicit accessible palettes and
  presentations style short help, complete help, parser diagnostics, and human
  validation reports while plain ANSI-free output remains the default.
- CLI help describes every command, nested command, positional argument,
  option, default, interaction, and accepted enum value at its corresponding
  help level; root `--help` consolidates the complete command tree while root
  `-h` remains compact. The consolidated tree visually separates every command,
  omits redundant generated `help` subcommands, and keeps focused command help
  uncluttered. It documents `-h, --help` once at the root while each focused
  command help retains its own entry. Root help fully documents visual renderer
  controls, while focused help indicates them only when they also affect that
  command's execution. The consolidated hierarchy uses shorter separators for
  nested commands, keeps low-vision section spacing explicit, and places root
  help guidance with its option instead of in a trailing footer. Command
  sections use concise paths as headings and include their complete generated
  invocation under a dedicated `Usage` line, while the compact command index
  remains available before the root options. Low-vision help separates adjacent
  commands, arguments, and options while keeping command descriptions beside
  their names and every other description with its item. Root usage consistently
  presents the command before global options in both short and complete help.
- Knowledge catalog entries use minimal structural metadata while `draft` and
  require complete routing and review metadata when marked `reviewed`.
- Recipe release validation accepts relative `workspaceRoot` layouts while
  confining materialized examples to an isolated temporary workspace.

### Fixed

- Human validation no longer deadlocks after tool preflight when the live
  execution renderer and CLI diagnostics share an interactive terminal.
- Configuration initialization accepts valid contained candidates on macOS and
  Windows without relying on platform-specific canonical path spelling.
- Tool preflight recognizes vendor-qualified version output such as Git for
  Windows while retaining the semantic-version core used by requirements.

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
