$ErrorActionPreference = "Stop"

if ($args.Count -ne 2) {
  throw "Usage: verify.ps1 <canonical-command> <alias-command>"
}

$Canonical = (Resolve-Path $args[0]).Path
$Alias = (Resolve-Path $args[1]).Path
$CanonicalVersion = (& $Canonical --version | Out-String).Trim()
$AliasVersion = (& $Alias --version | Out-String).Trim()
if ($CanonicalVersion -ne $AliasVersion) {
  throw "installed commands report different versions"
}

$Temporary = Join-Path ([System.IO.Path]::GetTempPath()) ("liknon-manager-smoke-" + [guid]::NewGuid())
New-Item -ItemType Directory -Path $Temporary | Out-Null
try {
  $CanonicalHelp = (& $Canonical -h | Out-String)
  $AliasHelp = (& $Alias -h | Out-String)
  if ($CanonicalHelp -ne $AliasHelp.Replace("likm", "liknon-manager")) {
    throw "installed commands expose different help"
  }
  & $Canonical schema resource | Out-Null
  New-Item -ItemType Directory -Path (Join-Path $Temporary "workspace") | Out-Null
  & $Canonical init --workspace (Join-Path $Temporary "workspace") --format=json | Out-Null
  & $Alias list --workspace (Join-Path $Temporary "workspace") --format=json | Out-Null
  & $Canonical verify --offline --workspace (Join-Path $Temporary "workspace") --format=json | Out-Null
} finally {
  Remove-Item -Recurse -Force $Temporary -ErrorAction SilentlyContinue
}
