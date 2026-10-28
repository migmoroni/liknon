param(
    [Parameter(Mandatory = $true)]
    [string]$Canonical,

    [Parameter(Mandatory = $true)]
    [string]$Alias
)

$ErrorActionPreference = "Stop"

if (-not (Test-Path -LiteralPath $Canonical -PathType Leaf)) {
    throw "Canonical command does not exist: $Canonical"
}
if (-not (Test-Path -LiteralPath $Alias -PathType Leaf)) {
    throw "Alias command does not exist: $Alias"
}

$CanonicalVersion = (& $Canonical --version | Out-String).Trim()
if ($LASTEXITCODE -ne 0) { throw "Canonical version command failed." }
$AliasVersion = (& $Alias --version | Out-String).Trim()
if ($LASTEXITCODE -ne 0) { throw "Alias version command failed." }
if ($CanonicalVersion -cne $AliasVersion) {
    throw "Installed commands report different versions."
}

$CanonicalHelp = (& $Canonical -h | Out-String)
if ($LASTEXITCODE -ne 0) { throw "Canonical help command failed." }
$AliasHelp = (& $Alias -h | Out-String)
if ($LASTEXITCODE -ne 0) { throw "Alias help command failed." }
if ($CanonicalHelp -cne $AliasHelp) {
    throw "Installed commands produced different help."
}

& $Canonical schema config | Out-Null
if ($LASTEXITCODE -ne 0) { throw "Schema command failed." }

$TemporaryDirectory = Join-Path ([System.IO.Path]::GetTempPath()) "liknon-validation-smoke-$([guid]::NewGuid())"
New-Item -ItemType Directory -Path $TemporaryDirectory | Out-Null
try {
    $Configuration = [ordered]@{
        schemaVersion = 6
        workspaceRoot = "."
        defaultGroup = "release.smoke"
        outputLimitBytes = 4096
        tools = @([ordered]@{
            id = "installed-validator"
            program = $Canonical
            requiresTools = @()
            versionArgs = @("--version")
            versionParser = "firstSemver"
        })
        checks = @([ordered]@{
            id = "installed-validator.version"
            label = "Installed validator version"
            description = "Executes the installed validation CLI."
            toolId = "installed-validator"
            args = @("--version")
            requiresTools = @()
            timeoutSeconds = 30
        })
        suites = @([ordered]@{
            id = "release.smoke.suite"
            label = "Release smoke"
            description = "Validates the installed command."
            checks = @([ordered]@{checkId = "installed-validator.version"; dependsOn = @()})
        })
        groups = @([ordered]@{
            id = "release.smoke"
            label = "Release smoke"
            description = "Runs the release smoke suite."
            members = @([ordered]@{kind = "suite"; id = "release.smoke.suite"})
        })
    }
    $ConfigurationPath = Join-Path $TemporaryDirectory "config.json"
    $Configuration | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $ConfigurationPath -Encoding utf8NoBOM
    & $Canonical validate --config $ConfigurationPath | Out-Null
    if ($LASTEXITCODE -ne 0) { throw "Installed validator smoke validation failed." }
} finally {
    Remove-Item -LiteralPath $TemporaryDirectory -Recurse -Force -ErrorAction SilentlyContinue
}
