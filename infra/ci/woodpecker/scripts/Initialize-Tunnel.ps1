# Creates (or reuses) the named tunnel and its DNS route, then renders the
# runtime config. Requires a prior `cloudflared tunnel login` by the maintainer.
# Never overwrites an existing DNS record that points elsewhere.
[CmdletBinding()]
param([string]$TunnelName = 'vynamp-woodpecker')
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'Common.ps1')

$cloudflared = Get-GlobalCloudflared
$dir = Join-Path $WoodpeckerSecrets 'cloudflared'
$cert = Join-Path $env:USERPROFILE '.cloudflared\cert.pem'
if (-not (Test-Path $cert)) { throw "Cloudflare login missing ($cert): run cloudflared tunnel login first" }
Protect-SecretsFolder
New-Item -ItemType Directory -Force $dir | Out-Null

$tunnels = Invoke-Native { & $cloudflared tunnel list --output json --name $TunnelName } | ConvertFrom-Json
$tunnel = @($tunnels | Where-Object { -not $_.deleted_at -or $_.deleted_at -like '0001-*' })[0]
if (-not $tunnel) {
    Invoke-Native { & $cloudflared tunnel create --credentials-file (Join-Path $dir 'pending.json') $TunnelName } | Out-Null
    $tunnel = @(Invoke-Native { & $cloudflared tunnel list --output json --name $TunnelName } | ConvertFrom-Json)[0]
    Move-Item -Force (Join-Path $dir 'pending.json') (Join-Path $dir "$($tunnel.id).json")
}
$id = $tunnel.id
if (-not (Test-Path (Join-Path $dir "$id.json"))) {
    throw "Tunnel $TunnelName ($id) exists but its credentials file is not in $dir; recover it instead of creating a duplicate"
}
# route dns fails (non-zero) when a conflicting record already exists; it is not forced.
Invoke-Native { & $cloudflared tunnel route dns $id $WoodpeckerHost }
$template = Get-Content -Raw (Join-Path $WoodpeckerConfig 'cloudflared\config.template.yml')
Set-Content -Path (Join-Path $dir 'config.yml') -Encoding ascii -Value $template.Replace('__TUNNEL_ID__', $id)
Write-Host "tunnel $TunnelName id $id routed to $WoodpeckerHost"
