# Phase B01.6: Assemble PyPI Wheels

[Back to the distribution plan](README.md)

## Purpose

Package Liknon as platform-specific Python wheels that install the
already-built native executable directly, without compiling Rust or adding a
Python implementation, launcher, import API, or runtime dependency.

## Dependencies

- [Phase B01.2](02-native-builds.md) is complete.
- The PyPI project name, publishing owner, Python, pip, pipx, and uv baselines,
  and exact wheel compatibility tags from Phase B01.0 are confirmed.
- Every native input matches `build-manifest.json`.

## Distribution Model

PyPI exposes one project named `liknon`. The same project version may contain
multiple platform wheels, and the Python installer selects the wheel compatible
with the current environment.

This channel publishes wheels only. It does not publish an sdist or another
source package that could cause Cargo to run on the user machine. It also does
not contain Python modules or define an importable `liknon` package.

Python is an installation frontend for this channel, not a Liknon runtime. The
installed command is the native Rust executable itself.

## Wheel Layout

Each staged wheel tree contains only the native command, required notices, and
standard Wheel metadata:

```text
liknon-<version>.data/
└── scripts/
    └── liknon[.exe]
liknon-<version>.dist-info/
├── licenses/
├── METADATA
├── WHEEL
└── RECORD
```

The wheel assembler places the manifest-verified executable under
`.data/scripts/`. Installation moves it into the environment's script
directory. Unix wheels preserve executable behavior, and the Windows wheel
uses the exact `liknon.exe` filename.

`RECORD` covers every wheel member according to the Wheel specification. The
native executable hash is additionally compared with `build-manifest.json`;
Wheel metadata hashes do not replace release provenance.

## Metadata And Compatibility Contract

- The distribution name is `liknon` and its version is generated from Cargo
  metadata during staging.
- Wheels use the generic Python tag `py3` and ABI tag `none` only after tests
  prove that no Python implementation or ABI is involved.
- `Requires-Python` records the installer baseline approved in Phase B01.0.
- Every platform tag accurately represents the operating system, architecture,
  libc contract, and macOS deployment baseline of the contained binary.
- A Linux binary may appear in more than one wheel envelope only when each
  manylinux or musllinux tag has independent compatibility evidence.
- Wheel filenames, `WHEEL` tags, and staged platform identity must agree.
- No wheel declares Python dependencies, build dependencies, entry-point
  wrappers, or installation hooks.
- No wheel contacts Cloudflare, GitHub, or another host during installation or
  execution.

The channel does not use Maturin, PyO3, CFFI, or another Rust build backend.
Assembly uses maintained Wheel-format tooling capable of packing a pre-staged
tree without invoking Cargo. A candidate packer that cannot preserve this
boundary is not acceptable merely because a prior Cargo result may exist in a
local cache.

## Implementation Tasks

1. Add version-neutral Wheel metadata templates and an assembly entry point.
2. Select and pin a maintained Wheel-format packer that accepts a staged tree
   without compiling source.
3. Derive distribution version, filenames, and metadata from Cargo metadata and
   the approved platform matrix.
4. Stage one tree for every supported wheel compatibility tag under
   `target/distribution/pypi/`.
5. Copy only build-manifest-verified native bytes into `.data/scripts/`.
6. Generate standards-compliant `METADATA`, `WHEEL`, and `RECORD` files.
7. Pack and inspect each wheel without producing an sdist.
8. Prove assembly leaves tracked files unchanged and invokes no Rust build.

## Verification

- Inspect every wheel filename, compatibility tag, metadata field, member,
  permission, and `RECORD` entry.
- Compare the installed executable SHA-256 with `build-manifest.json`.
- Install each matching local wheel with pip's user installation scheme under
  a clean temporary user base and expose its scripts directory on `PATH`.
- Install the same wheel globally for the isolated user with `pipx install` and
  `uv tool install`, using a separate clean home for each frontend.
- Remove or isolate each installation before testing the next frontend so a
  previously installed command cannot satisfy another frontend's smoke test.
- Run `liknon --version`, `liknon --help`, and a representative
  `liknon validate` fixture after installation.
- Test arguments, current directory, environment, standard streams, successful
  completion, nonzero status, and supported signal behavior.
- Test installation and execution paths containing spaces.
- Confirm no Python interpreter, wrapper, or module is needed after the native
  executable is installed.
- Confirm unsupported platforms report that no compatible distribution exists
  rather than compiling or selecting another binary.
- Confirm no sdist, build hook, Maturin invocation, Cargo invocation, or
  Cloudflare request occurs.

## Exit Criterion

Every local platform wheel installs a globally invocable exact release binary
through the pip user scheme, pipx, and `uv tool`, without Rust, source
compilation, a Python runtime wrapper, a secondary download, or an importable
Python API.
