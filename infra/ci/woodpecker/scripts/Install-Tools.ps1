# Downloads the pinned Windows binaries into the project runtime folder and
# verifies the published SHA256 before anything is extracted or executed.
# cloudflared is not handled here: it is the global winget install (see versions.json).
[CmdletBinding()]
param()
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'Common.ps1')

$downloads = Join-Path $WoodpeckerRuntime 'downloads'
New-Item -ItemType Directory -Force $downloads, $WoodpeckerBin | Out-Null
foreach ($tool in (Get-WoodpeckerPins).windows) {
    $name = Split-Path -Leaf $tool.url
    $archive = Join-Path $downloads $name
    if (-not (Test-Path $archive)) {
        Write-Host "download $($tool.url)"
        Invoke-WebRequest -Uri $tool.url -OutFile "$archive.part"
        Move-Item -Force "$archive.part" $archive
    }
    Assert-FileSha256 -Path $archive -Sha256 $tool.sha256
    $target = Join-Path $WoodpeckerBin $tool.file
    if ($name -like '*.zip') {
        $extract = Join-Path $downloads ([IO.Path]::GetFileNameWithoutExtension($name))
        Expand-Archive -Force -Path $archive -DestinationPath $extract
        Copy-Item -Force (Join-Path $extract $tool.file) $target
    } else {
        Copy-Item -Force $archive $target
    }
    Write-Host "ok $($tool.name) sha256 $($tool.sha256)"
}
