# Run by the maintainer in a local PowerShell. Prompts for the GitHub OAuth App
# credentials and generates the agent secret. Values are written only to the
# runtime secrets folder (ACL limited to the current user) and never printed.
[CmdletBinding()]
param([switch]$RotateAgentSecret)
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'Common.ps1')

Protect-SecretsFolder

$serverEnv = Join-Path $WoodpeckerSecrets 'server.env'
$agentEnv = Join-Path $WoodpeckerSecrets 'agent.env'
$agentSecret = $null
if ((Test-Path $agentEnv) -and -not $RotateAgentSecret) {
    $agentSecret = ((Get-Content $agentEnv | Where-Object { $_ -like 'WOODPECKER_AGENT_SECRET=*' }) -split '=', 2)[1]
}
if (-not $agentSecret) { $agentSecret = New-RandomHex }

Write-Host 'GitHub OAuth App: homepage https://ci-cd-vynamp.jkca.lat, callback https://ci-cd-vynamp.jkca.lat/authorize'
$client = (Read-Host 'OAuth Client ID').Trim()
$secure = Read-Host 'OAuth Client Secret (hidden)' -AsSecureString
$secret = [Net.NetworkCredential]::new('', $secure).Password.Trim()
if (-not $client -or -not $secret) { throw 'Client ID and Client Secret are required' }

$grpcSecret = $null
if (Test-Path $serverEnv) {
    $grpcSecret = ((Get-Content $serverEnv | Where-Object { $_ -like 'WOODPECKER_GRPC_SECRET=*' }) -split '=', 2)[1]
}
if (-not $grpcSecret) { $grpcSecret = New-RandomHex }
Set-Content -Path $serverEnv -Encoding ascii -Value @(
    "WOODPECKER_GITHUB_CLIENT=$client",
    "WOODPECKER_GITHUB_SECRET=$secret",
    "WOODPECKER_AGENT_SECRET=$agentSecret",
    "WOODPECKER_GRPC_SECRET=$grpcSecret"
)
Set-Content -Path $agentEnv -Encoding ascii -Value "WOODPECKER_AGENT_SECRET=$agentSecret"
Write-Host "secrets written to $WoodpeckerSecrets (values not shown)"
