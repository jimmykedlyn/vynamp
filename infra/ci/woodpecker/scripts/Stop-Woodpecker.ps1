# Stops the Windows agent and the Compose services. Volumes, secrets, claims
# and worktrees are preserved (no `down -v`).
[CmdletBinding()]
param([switch]$WindowsAgentOnly)
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'Common.ps1')

$pidFile = Join-Path $WoodpeckerRuntime 'windows-agent\agent.pid'
if (Test-Path $pidFile) {
    $p = Get-Process -Id (Get-Content $pidFile) -ErrorAction SilentlyContinue
    if ($p -and $p.ProcessName -eq 'woodpecker-agent') { Stop-Process -Id $p.Id; Write-Host "windows agent stopped pid $($p.Id)" }
    Remove-Item $pidFile
}
if ($WindowsAgentOnly) { return }
Invoke-Native { docker compose -f (Join-Path $WoodpeckerConfig 'compose.yaml') --profile tunnel stop }
