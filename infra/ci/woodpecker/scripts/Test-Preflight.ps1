# Static checks for the Woodpecker pilot configuration. -SelfTest proves each
# check rejects a known-bad input before trusting it on the real files.
[CmdletBinding()]
param([switch]$SelfTest)
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'Common.ps1')

function Assert-Rejects([string]$Name, [scriptblock]$Check) {
    $failed = $false
    try { & $Check | Out-Null } catch { $failed = $true }
    if (-not $failed) { throw "Self-test '$Name': check accepted a bad input" }
    Write-Host "self-test ok: $Name"
}

if ($SelfTest) {
    $tmp = Join-Path $WoodpeckerRuntime 'selftest'
    New-Item -ItemType Directory -Force $tmp | Out-Null
    $file = Join-Path $tmp 'sample.bin'
    Set-Content -Path $file -Value 'vynamp' -NoNewline
    Assert-Rejects 'sha256 mismatch' { Assert-FileSha256 -Path $file -Sha256 ('0' * 64) }
    Assert-Rejects 'unpinned image' { Test-ComposeText "services:`n  s:`n    image: woodpeckerci/woodpecker-server:v3.19.0`n" }
    Assert-Rejects 'latest image' { Test-ComposeText "services:`n  s:`n    image: woodpeckerci/woodpecker-server:latest@sha256:$('a' * 64)`n" }
    Assert-Rejects 'public port' { Test-ComposeText "services:`n  s:`n    ports:`n      - `"8000:8000`"`n" }
    Assert-Rejects 'grpc in tunnel' { Test-TunnelText "ingress:`n  - hostname: x`n    service: http://woodpecker-server:9000`n  - service: http_status:404`n" }
    Assert-Rejects 'tunnel without 404 fallback' { Test-TunnelText "ingress:`n  - service: http://woodpecker-server:8000`n" }
    Assert-Rejects 'secret in text' { Test-NoSecretText ('WOODPECKER_GITHUB_SECRET=' + ('0' * 40)) }
    Assert-Rejects 'github token in text' { Test-NoSecretText ('token ghp_' + ('A' * 36)) }
    Assert-Rejects 'native exit code' { Invoke-Native { cmd /c exit 3 } }
    Remove-Item -Recurse -Force $tmp
    Write-Host 'self-test passed'
    exit 0
}

$compose = Join-Path $WoodpeckerConfig 'compose.yaml'
Test-ComposeText (Get-Content -Raw $compose)
Test-TunnelText (Get-Content -Raw (Join-Path $WoodpeckerConfig 'cloudflared\config.template.yml'))
$pins = Get-WoodpeckerPins
foreach ($image in $pins.images.PSObject.Properties.Value) {
    if ($image -notmatch '@sha256:[0-9a-f]{64}$') { throw "versions.json image not pinned: $image" }
}
foreach ($tool in $pins.windows) {
    $path = Join-Path $WoodpeckerBin $tool.file
    if (Test-Path $path) {
        if ($tool.url -notmatch '\.zip$') { Assert-FileSha256 -Path $path -Sha256 $tool.sha256 }
    } else { Write-Warning "tool not installed yet: $($tool.file)" }
}
$repoRoot = Resolve-Path (Join-Path $WoodpeckerConfig '..\..\..')
$tracked = Invoke-Native { git -C $repoRoot ls-files -- infra .woodpecker }
foreach ($rel in $tracked) { Test-NoSecretText (Get-Content -Raw (Join-Path $repoRoot $rel)) -Source $rel }
Write-Host 'preflight passed'
