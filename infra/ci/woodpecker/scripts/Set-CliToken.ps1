# Run by the maintainer: stores the Woodpecker personal API token (UI → User
# settings → Personal access token) for woodpecker-cli. Never printed.
[CmdletBinding()]
param()
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'Common.ps1')

Protect-SecretsFolder
$secure = Read-Host 'Woodpecker personal access token (hidden)' -AsSecureString
$token = [Net.NetworkCredential]::new('', $secure).Password.Trim()
if (-not $token) { throw 'Token is required' }
Set-Content -Path (Join-Path $WoodpeckerSecrets 'cli.env') -Encoding ascii -Value @(
    'WOODPECKER_SERVER=http://127.0.0.1:8000',
    "WOODPECKER_TOKEN=$token"
)
Write-Host "cli token written to $WoodpeckerSecrets (value not shown)"
