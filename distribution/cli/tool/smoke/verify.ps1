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

$Temporary = Join-Path ([System.IO.Path]::GetTempPath()) ("liknon-tool-smoke-" + [guid]::NewGuid())
New-Item -ItemType Directory -Path $Temporary | Out-Null
try {
  $CanonicalHelp = (& $Canonical -h | Out-String)
  $AliasHelp = (& $Alias -h | Out-String)
  if ($CanonicalHelp -ne $AliasHelp.Replace("likt", "liknon-tool")) {
    throw "installed commands expose different help"
  }

  $Workspace = Join-Path $Temporary "workspace"
  New-Item -ItemType Directory -Path $Workspace | Out-Null
  Copy-Item -Recurse -LiteralPath (Join-Path $PSScriptRoot "fixture/.liknon") -Destination $Workspace
  $Before = Get-ChildItem -Recurse -File -LiteralPath (Join-Path $Workspace ".liknon") |
    ForEach-Object { "{0}:{1}" -f $_.FullName.Substring($Workspace.Length), (Get-FileHash -Algorithm SHA256 -LiteralPath $_.FullName).Hash }

  & $Canonical catalog --workspace $Workspace --format=json | Out-File -Encoding utf8NoBOM (Join-Path $Temporary "catalog.json")
  & $Alias search smoke --workspace $Workspace --format=json | Out-File -Encoding utf8NoBOM (Join-Path $Temporary "search.json")
  & $Canonical show knowledge:fixture/smoke --workspace $Workspace --format=json | Out-File -Encoding utf8NoBOM (Join-Path $Temporary "show.json")
  & $Canonical resolve knowledge:fixture/smoke --workspace $Workspace --format=json | Out-File -Encoding utf8NoBOM (Join-Path $Temporary "resolve.json")
  & $Canonical context knowledge:fixture/smoke --workspace $Workspace --format=json | Out-File -Encoding utf8NoBOM (Join-Path $Temporary "context.json")

  if (-not (Select-String -SimpleMatch "Liknon Tool smoke payload." -LiteralPath (Join-Path $Temporary "show.json"))) {
    throw "show did not return the verified smoke payload"
  }
  if (-not (Select-String -SimpleMatch "untrusted-resource-content" -LiteralPath (Join-Path $Temporary "context.json"))) {
    throw "context omitted the instruction boundary"
  }
  $After = Get-ChildItem -Recurse -File -LiteralPath (Join-Path $Workspace ".liknon") |
    ForEach-Object { "{0}:{1}" -f $_.FullName.Substring($Workspace.Length), (Get-FileHash -Algorithm SHA256 -LiteralPath $_.FullName).Hash }
  if (Compare-Object $Before $After) {
    throw "Tool changed the resource store"
  }
} finally {
  Remove-Item -Recurse -Force $Temporary -ErrorAction SilentlyContinue
}
