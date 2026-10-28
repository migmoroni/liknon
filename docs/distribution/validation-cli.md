# Validation CLI Distribution

`liknon-validator` is one standalone application exposed through two
byte-equivalent commands:

- `liknon-validator`, the explicit command;
- `likv`, the compact command.

Every release uses one package-derived version, one signed annotated Git tag,
one source commit, and one committed lockfile. GitHub and crates.io distribute
source. The project-controlled download domain distributes target-native
archives. GitHub Releases record source identity and intentionally contain no
binary assets.

## Availability

Source installation from the repository is available during development. The
crates.io and native channels become public only after their first release has
completed the protected `Release Validation CLI` workflow. Until then, the
absence of a registry version, public manifest, or stable pointer is expected
and must not be bypassed with an unverified mirror.

## Install From A Signed Git Tag

Replace `<VERSION>` with one canonical `MAJOR.MINOR.PATCH` release:

```sh
git clone https://github.com/migmoroni/liknon.git
cd liknon
git fetch --tags --force
git verify-tag "liknon-validator-v<VERSION>"
git checkout --detach "liknon-validator-v<VERSION>"
cargo install --path apps/liknon-validator --locked
liknon-validator --version
likv --version
```

`git verify-tag` must succeed under a maintainer identity trusted independently
by the user. Do not substitute the default branch when reproducibility matters.
Rust 1.87 or newer is required to build from source.

Update by verifying and checking out another exact tag, then reinstalling the
same package. Remove a Cargo installation with:

```sh
cargo uninstall liknon-validator
```

## Install From crates.io

Install an exact accepted source package and its locked dependency graph:

```sh
cargo install liknon-validator --version "=<VERSION>" --locked
liknon-validator --version
likv --version
```

Cargo compiles this channel locally. It does not download a Liknon native
archive. Use `cargo uninstall liknon-validator` to remove both command targets.

## Install A Native Archive

The production download origin is configured before the first release as
`LIKNON_DOWNLOAD_BASE_URL`. For an exact version, the immutable release root is:

```text
https://<download-domain>/releases/liknon-validator/v<VERSION>/
```

The release contains `manifest.json`, `manifest.sig`, `SHA256SUMS`, signed
POSIX and PowerShell installers, and one archive per supported target. Obtain
the public manifest key from the exact verified source tag at:

```text
distribution/cli/validation/direct/public-key.pem
```

Authenticate an installer before executing it:

```sh
release=https://<download-domain>/releases/liknon-validator/v<VERSION>
curl --fail --max-redirs 0 --proto '=https' --proto-redir '=https' --tlsv1.2 \
  --output install.sh "$release/install.sh"
curl --fail --max-redirs 0 --proto '=https' --proto-redir '=https' --tlsv1.2 \
  --output install.sh.sig "$release/install.sh.sig"
openssl dgst -sha256 \
  -verify public-key.pem \
  -signature install.sh.sig \
  install.sh
sh install.sh
```

The installer authenticates the exact manifest before parsing it, detects the
supported host, downloads one archive, verifies the archive and both command
hashes, and replaces an existing installation transactionally. Its default
POSIX destination is `~/.local/bin`. Select another user-owned directory with
`--install-dir`.

On Linux, automatic selection chooses GNU when compatible GNU and MUSL
implementations are both detected. Override only with an implementation that
is actually available:

```sh
sh install.sh --libc gnu
sh install.sh --libc musl
```

PowerShell 7 or newer is required by the Windows installer for built-in RSA
verification:

```powershell
$release = "https://<download-domain>/releases/liknon-validator/v<VERSION>"
Invoke-WebRequest -MaximumRedirection 0 "$release/install.ps1" -OutFile install.ps1
Invoke-WebRequest -MaximumRedirection 0 "$release/install.ps1.sig" -OutFile install.ps1.sig
$rsa = [System.Security.Cryptography.RSA]::Create()
try {
  $rsa.ImportFromPem((Get-Content -LiteralPath public-key.pem -Raw))
  $valid = $rsa.VerifyData(
    [System.IO.File]::ReadAllBytes((Resolve-Path ./install.ps1).Path),
    [System.IO.File]::ReadAllBytes((Resolve-Path ./install.ps1.sig).Path),
    [System.Security.Cryptography.HashAlgorithmName]::SHA256,
    [System.Security.Cryptography.RSASignaturePadding]::Pkcs1
  )
} finally {
  $rsa.Dispose()
}
if (-not $valid) { throw "Installer signature verification failed." }
pwsh -File ./install.ps1
```

The primary instructions never pipe a network response directly into a shell.
Use `--uninstall` on the same versioned installer to remove its two commands.
The installers also verify native platform evidence before replacing an
existing installation: strict ad hoc signatures on macOS, and the pinned
Authenticode fingerprint plus timestamp on Windows.

## Manual Verification

For a manual archive installation:

1. Download `manifest.json`, `manifest.sig`, and the independently trusted
   `public-key.pem`.
2. Run `openssl dgst -sha256 -verify public-key.pem -signature manifest.sig
   manifest.json` before reading any manifest field.
3. Select exactly one declared target archive.
4. Compare its SHA-256 with the authenticated manifest.
5. Extract only the declared command and license inventory.
6. Compare both extracted command hashes with the manifest before execution.

`SHA256SUMS` is a convenient projection, but its own hash is trusted only
because it appears in the signed manifest.

## Supported Native Targets

| Target | Initial runtime baseline | Archive |
| --- | --- | --- |
| `x86_64-unknown-linux-gnu` | glibc 2.39 or newer | `.tar.gz` |
| `x86_64-unknown-linux-musl` | static MUSL; Linux kernel 4.18 or newer | `.tar.gz` |
| `aarch64-unknown-linux-gnu` | glibc 2.39 or newer | `.tar.gz` |
| `aarch64-unknown-linux-musl` | static MUSL; Linux kernel 4.18 or newer | `.tar.gz` |
| `x86_64-apple-darwin` | macOS 15 or newer | `.tar.gz` |
| `aarch64-apple-darwin` | macOS 15 or newer | `.tar.gz` |
| `x86_64-pc-windows-msvc` | Windows 10 x64 or newer | `.zip` |

Every target is a separate archive. Linux GNU and MUSL payloads are never
combined or selected by distribution name.

## Platform Trust

- Linux relies on the signed release manifest, exact hashes, the signed source
  identity, and GitHub artifact provenance. It claims no native publisher.
- Windows initially uses one persistent project-controlled self-signed
  Authenticode certificate with an RFC 3161 timestamp. Verification pins its
  published fingerprint without importing it into the system trust store.
- macOS initially uses an ad hoc code signature and is not notarized. Prefer
  the user-mediated **Open Anyway** flow if Gatekeeper intervenes. After
  cryptographic verification, remove quarantine only from an exact installed
  path if required, never recursively:

  ```sh
  xattr -d com.apple.quarantine "$HOME/.local/bin/liknon-validator"
  xattr -d com.apple.quarantine "$HOME/.local/bin/likv"
  ```

Report a signing, manifest, archive, or download-domain discrepancy through the
private process in [SECURITY.md](../../SECURITY.md). Do not continue an install
after an integrity warning.
