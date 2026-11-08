$VynampRoot = [IO.Path]::GetFullPath($PSScriptRoot)
$VynampCache = Join-Path $VynampRoot '.cache'
$VynampTemp = Join-Path $VynampCache 'tmp'
$VynampRepo = Join-Path $VynampRoot 'repo'
$VynampPackage = Join-Path $VynampRoot 'pacote'
if (($env:OS -eq 'Windows_NT') -and (-not $VynampRoot.TrimEnd('\').Equals('D:\Projects\vynamp-media-player', [StringComparison]::OrdinalIgnoreCase))) {
    throw 'Extraia a raiz em D:\Projects\vynamp-media-player e execute o atalho externo dessa pasta.'
}
foreach ($folder in @($VynampRoot, $VynampCache, $VynampRepo, $VynampPackage, $VynampTemp, (Join-Path $VynampCache 'cargo'), (Join-Path $VynampCache 'bootstrap-build'), (Join-Path $VynampRoot 'logs'))) {
    if ((Test-Path -LiteralPath $folder) -and ((Get-Item -LiteralPath $folder -Force).Attributes -band [IO.FileAttributes]::ReparsePoint)) {
        throw "Pasta com junction/symlink nao suportada no bootstrap: $folder"
    }
}
foreach ($folder in @($VynampTemp, (Join-Path $VynampCache 'cargo'), (Join-Path $VynampRoot 'logs'))) {
    New-Item -ItemType Directory -Path $folder -Force | Out-Null
}
$env:CARGO_HOME = Join-Path $VynampCache 'cargo'
$env:CARGO_TARGET_DIR = Join-Path $VynampCache 'bootstrap-build'
$env:TEMP = $VynampTemp
$env:TMP = $VynampTemp
function Invoke-VynampCommand {
    param([string]$Program, [string[]]$Arguments)
    & $Program @Arguments
    if ($LASTEXITCODE -ne 0) { throw "Falhou: $Program $($Arguments -join ' ') (exit $LASTEXITCODE)" }
}
