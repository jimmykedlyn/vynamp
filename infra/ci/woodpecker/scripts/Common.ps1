# Shared paths and guards for the Woodpecker pilot scripts. Runtime state never
# lives in the repository: everything mutable goes under $WoodpeckerRuntime.
$ErrorActionPreference = 'Stop'
$WoodpeckerConfig = Resolve-Path (Join-Path $PSScriptRoot '..')
$WoodpeckerRuntime = 'D:\Projects\vynamp-media-player\.cache\woodpecker'
$WoodpeckerBin = Join-Path $WoodpeckerRuntime 'bin'
$WoodpeckerSecrets = Join-Path $WoodpeckerRuntime 'secrets'
$WoodpeckerLogs = Join-Path $WoodpeckerRuntime 'logs'
$WoodpeckerHost = 'ci-cd-vynamp.jkca.lat'

function Get-WoodpeckerPins {
    Get-Content -Raw (Join-Path $WoodpeckerConfig 'versions.json') | ConvertFrom-Json
}

# cloudflared is the maintainer's global winget install, not a project copy.
function Get-GlobalCloudflared {
    $pin = (Get-WoodpeckerPins).cloudflared_global
    if (-not (Test-Path $pin.path)) { throw "cloudflared not found at $($pin.path): winget install Cloudflare.cloudflared --version $($pin.version)" }
    Assert-FileSha256 -Path $pin.path -Sha256 $pin.sha256
    $pin.path
}

function Assert-FileSha256([string]$Path, [string]$Sha256) {
    $actual = (Get-FileHash -Algorithm SHA256 -Path $Path).Hash.ToLowerInvariant()
    if ($actual -ne $Sha256.ToLowerInvariant()) {
        throw "SHA256 mismatch for ${Path}: expected $Sha256, got $actual"
    }
}

# Runs a native command and fails on a non-zero exit code, so a later command
# can never hide an earlier failure.
function Invoke-Native([scriptblock]$Command) {
    $global:LASTEXITCODE = 0
    & $Command
    if ($LASTEXITCODE -ne 0) { throw "Native command failed with exit code ${LASTEXITCODE}: $Command" }
}

function Test-ComposeText([string]$Text) {
    foreach ($m in [regex]::Matches($Text, '(?m)^\s*image:\s*(\S+)')) {
        $image = $m.Groups[1].Value.Trim('"', "'")
        if ($image -notmatch '@sha256:[0-9a-f]{64}$' -or $image -match ':latest[@]') {
            throw "Compose image not pinned by version and digest: $image"
        }
    }
    foreach ($m in [regex]::Matches($Text, '(?m)^\s*-\s*["'']?([^"''\s]+)["'']?\s*$')) {
        $entry = $m.Groups[1].Value
        if ($entry -match '^\d+:\d+$' -or $entry -match '^0\.0\.0\.0:') {
            throw "Compose port not bound to 127.0.0.1: $entry"
        }
    }
}

function Test-TunnelText([string]$Text) {
    $services = [regex]::Matches($Text, '(?m)service:\s*(\S+)') | ForEach-Object { $_.Groups[1].Value }
    foreach ($s in $services) {
        if ($s -ne 'http://woodpecker-server:8000' -and $s -ne 'http_status:404') {
            throw "Tunnel ingress exposes an unexpected service: $s"
        }
    }
    if ($services.Count -eq 0 -or $services[-1] -ne 'http_status:404') {
        throw 'Tunnel ingress must end with http_status:404'
    }
}

function Test-NoSecretText([string]$Text, [string]$Source = 'text') {
    $patterns = @(
        '(?i)(secret|token|password)[A-Z_]*\s*[=:]\s*["'']?[A-Za-z0-9_\-+/]{20,}',
        'gh[pousr]_[A-Za-z0-9]{36}',
        'github_pat_[A-Za-z0-9_]{20,}',
        '"TunnelSecret"\s*:'
    )
    foreach ($p in $patterns) {
        if ($Text -match $p) { throw "Possible secret in ${Source} (pattern $p)" }
    }
}
