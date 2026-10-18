# Run by the maintainer in a local PowerShell. Prompts for the GitHub OAuth App
# credentials and generates the agent secret. Values are written only to the
# runtime secrets folder (ACL limited to the current user) and never printed.
[CmdletBinding()]
param([switch]$RotateAgentSecret)
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'Common.ps1')

New-Item -ItemType Directory -Force $WoodpeckerSecrets | Out-Null
Invoke-Native { icacls $WoodpeckerSecrets /inheritance:r /grant:r "${env:USERNAME}:(OI)(CI)F" | Out-Null }

$serverEnv = Join-Path $WoodpeckerSecrets 'server.env'
$agentEnv = Join-Path $WoodpeckerSecrets 'agent.env'
$agentSecret = $null
if ((Test-Path $agentEnv) -and -not $RotateAgentSecret) {
    $agentSecret = ((Get-Content $agentEnv | Where-Object { $_ -like 'WOODPECKER_AGENT_SECRET=*' }) -split '=', 2)[1]
}
if (-not $agentSecret) {
    $bytes = [byte[]]::new(32)
    [Security.Cryptography.RandomNumberGenerator]::Fill($bytes)
    $agentSecret = [Convert]::ToHexString($bytes).ToLowerInvariant()
}

Write-Host 'GitHub OAuth App: homepage https://ci-cd-vynamp.jkca.lat, callback https://ci-cd-vynamp.jkca.lat/authorize'
$client = (Read-Host 'OAuth Client ID').Trim()
$secure = Read-Host 'OAuth Client Secret (hidden)' -AsSecureString
$secret = [Net.NetworkCredential]::new('', $secure).Password.Trim()
if (-not $client -or -not $secret) { throw 'Client ID and Client Secret are required' }

Set-Content -Path $serverEnv -Encoding ascii -Value @(
    "WOODPECKER_GITHUB_CLIENT=$client",
    "WOODPECKER_GITHUB_SECRET=$secret",
    "WOODPECKER_AGENT_SECRET=$agentSecret"
)
Set-Content -Path $agentEnv -Encoding ascii -Value "WOODPECKER_AGENT_SECRET=$agentSecret"
Write-Host "secrets written to $WoodpeckerSecrets (values not shown)"
