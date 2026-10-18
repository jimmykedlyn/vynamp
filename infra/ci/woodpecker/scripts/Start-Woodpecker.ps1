# Starts the Compose stack (server, Linux agent, tunnel when configured) and the
# native Windows agent with a sanitized environment.
[CmdletBinding()]
param([switch]$NoWindowsAgent, [switch]$NoTunnel)
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'Common.ps1')

& (Join-Path $PSScriptRoot 'Test-Preflight.ps1')
foreach ($f in 'server.env', 'agent.env') {
    if (-not (Test-Path (Join-Path $WoodpeckerSecrets $f))) { throw "Missing ${f}: run Initialize-Secrets.ps1" }
}
$compose = Join-Path $WoodpeckerConfig 'compose.yaml'
$profileArgs = @()
if (-not $NoTunnel -and (Test-Path (Join-Path $WoodpeckerSecrets 'cloudflared\config.yml'))) { $profileArgs = '--profile', 'tunnel' }
Invoke-Native { docker compose -f $compose @profileArgs config --quiet }
Invoke-Native { docker compose -f $compose @profileArgs up -d }

$deadline = (Get-Date).AddSeconds(60)
while ($true) {
    try { if ((Invoke-WebRequest -UseBasicParsing http://127.0.0.1:8000/healthz).StatusCode -in 200..299) { break } } catch {}
    if ((Get-Date) -gt $deadline) { throw 'Woodpecker server not healthy on 127.0.0.1:8000 after 60s' }
    Start-Sleep -Seconds 2
}
Write-Host 'server healthy'
if ($NoWindowsAgent) { return }

$home_ = Join-Path $WoodpeckerRuntime 'windows-agent'
$pidFile = Join-Path $home_ 'agent.pid'
if ((Test-Path $pidFile) -and (Get-Process -Id (Get-Content $pidFile) -ErrorAction SilentlyContinue)) {
    Write-Host 'windows agent already running'; return
}
foreach ($d in 'tmp', 'cargo-home') { New-Item -ItemType Directory -Force (Join-Path $home_ $d) | Out-Null }
New-Item -ItemType Directory -Force $WoodpeckerLogs | Out-Null

# Credentials of the interactive session must not reach pipeline steps.
Get-ChildItem env: | Where-Object { $_.Name -match '(?i)(TOKEN|SECRET|PASSWORD|API_KEY|CREDENTIAL)' } |
    ForEach-Object { Remove-Item "env:$($_.Name)" }
$agentSecret = ((Get-Content (Join-Path $WoodpeckerSecrets 'agent.env')) -split '=', 2)[1]
$env:WOODPECKER_AGENT_SECRET = $agentSecret
$env:WOODPECKER_SERVER = '127.0.0.1:9000'
$env:WOODPECKER_BACKEND = 'local'
$env:WOODPECKER_HOSTNAME = 'vynamp-windows'
$env:WOODPECKER_AGENT_LABELS = 'host=vynamp-windows'
$env:WOODPECKER_MAX_WORKFLOWS = '1'
$env:WOODPECKER_AGENT_CONFIG_FILE = Join-Path $home_ 'agent.conf'
# Short base path: MSVC link.exe fails with LNK1104 once build paths pass MAX_PATH.
$env:WOODPECKER_BACKEND_LOCAL_TEMP_DIR = 'D:\Projects\vynamp-media-player\.cache\wpw\w'
New-Item -ItemType Directory -Force $env:WOODPECKER_BACKEND_LOCAL_TEMP_DIR | Out-Null
$env:TEMP = $env:TMP = Join-Path $home_ 'tmp'
$env:CARGO_HOME = Join-Path $home_ 'cargo-home'
Remove-Item env:CARGO_TARGET_DIR -ErrorAction SilentlyContinue
$env:PATH = "$WoodpeckerBin;$env:PATH"
$stamp = Get-Date -Format 'yyyyMMdd-HHmmss'
$p = Start-Process -PassThru -WindowStyle Hidden -FilePath (Join-Path $WoodpeckerBin 'woodpecker-agent.exe') `
    -RedirectStandardOutput (Join-Path $WoodpeckerLogs "windows-agent-$stamp.out.log") `
    -RedirectStandardError (Join-Path $WoodpeckerLogs "windows-agent-$stamp.err.log")
Set-Content $pidFile $p.Id
Write-Host "windows agent started pid $($p.Id)"
