# Phase B01.5: Assemble RubyGems Packages

[Back to the distribution plan](README.md)

## Purpose

Package Liknon as platform-specific gems that embed the already-built native
binary behind a transparent Ruby executable.

## Dependencies

- [Phase B01.2](02-native-builds.md) is complete.
- The gem name, Ruby, RubyGems, and Bundler baselines, platform identifiers,
  and owner from Phase B01.0 are confirmed.
- Every native input matches `build-manifest.json`.

## Gem Layout

A gem executable is a Ruby script rather than the native binary itself:

```text
exe/liknon                         # listed in spec.executables
lib/liknon/runner.rb               # launcher implementation
libexec/liknon/liknon[.exe]        # verified Rust binary
```

The launcher resolves the binary relative to its installed gem, invokes it
without a shell, and preserves arguments, current directory, environment,
standard streams, exit status, and signals. It performs no download and has no
Cargo fallback.

## Platform And Metadata Contract

- Build one platform gem for each RubyGems platform declared in the [shared
  matrix](README.md#initial-platform-contract).
- Stage gems in deterministic directories under `target/distribution/rubygems/`.
- Do not derive package identity from mutable environment such as
  `ENV["GEM_PLATFORM"]` inside the gemspec.
- Supply the intended platform through deterministic staging/build input and
  inspect the resulting package metadata.
- Publish no generic `ruby` source gem as a fallback.
- Encode the Phase B01.0 baselines with `required_ruby_version` and
  `required_rubygems_version`.
- Never contact Cloudflare, GitHub, or another binary host during installation.

## Implementation Tasks

1. Add the Ruby executable, runner, and deterministic gemspec/staging metadata.
2. Stage one tree for every supported RubyGems platform.
3. Copy only build-manifest-verified bytes into `libexec/liknon`.
4. Build every platform gem with strict validation.
5. Inspect platform, version, files, permissions, executable metadata, and
   runtime requirements.

## Verification

- Install each matching local gem globally into a clean user-owned `GEM_HOME`
  without Cargo and expose its executable directory on `PATH`.
- Run `liknon --version`, `liknon --help`, and a representative
  `liknon validate` fixture from that global installation.
- In a separate Bundler fixture, resolve the same version from an isolated
  local gem source and confirm Bundler selects the matching platform variant.
- Run the same smoke tests through `bundle exec liknon` to verify Bundler
  compatibility without presenting Bundler as the global installation path.
- Compare the extracted native binary with `build-manifest.json`.
- Test arguments, current directory, environment, standard streams, success,
  nonzero status, and signals.
- Test paths containing spaces and unsupported platforms.
- Prove there are no runtime downloads, source compilation, or Cloudflare
  requests.
- Prove staging leaves tracked files unchanged.

## Exit Criterion

Every local platform gem installs globally through RubyGems and resolves
correctly through Bundler with the exact release binary, without Rust, Cargo,
runtime downloads, or a generic source fallback.
