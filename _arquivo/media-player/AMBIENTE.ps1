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

function Invoke-VynampAuthenticatedAgent {
    param([Parameter(Mandatory = $true)][scriptblock]$Action)
    # Read the current user's existing credential before the agent changes
    # Windows identity. Never put the token in command arguments or files.
    $previousGitHubToken = $env:GH_TOKEN
    $tokenLines = $null
    try {
        try {
            $tokenLines = @(& gh auth token --hostname github.com 2>$null)
            $tokenExit = $LASTEXITCODE
        } catch {
            throw 'Nao foi possivel ler a credencial GitHub existente. Abra o atalho no PowerShell normal do usuario autenticado; nao repita login dentro do sandbox.'
        }
        $sessionToken = ($tokenLines -join '').Trim()
        if ($tokenExit -ne 0 -or [string]::IsNullOrWhiteSpace($sessionToken)) {
            throw 'Credencial GitHub indisponivel neste usuario. Abra o atalho no PowerShell normal autenticado; confira gh auth status nesse terminal.'
        }
        $env:GH_TOKEN = $sessionToken
        $sessionToken = $null
        $tokenLines = $null
        try {
            $visibleRepo = @(& gh api repos/jimmykedlyn/vynamp-mediaplayer --jq .full_name 2>$null)
            $repoExit = $LASTEXITCODE
        } catch {
            throw 'Nao foi possivel validar o acesso ao repositorio GitHub. Confira credencial/rede no PowerShell normal; nenhum agente foi iniciado.'
        }
        if ($repoExit -ne 0 -or ($visibleRepo -join '').Trim() -ne 'jimmykedlyn/vynamp-mediaplayer') {
            throw 'A credencial nao confirmou acesso a jimmykedlyn/vynamp-mediaplayer. Confira permissoes/rede no terminal normal; nenhum agente foi iniciado.'
        }
        & $Action
    } finally {
        $env:GH_TOKEN = $previousGitHubToken
        $sessionToken = $null
        $tokenLines = $null
    }
}

# Ferramenta de desenvolvimento (oraculo de testes; nao entra em codigo entregue)
$env:FFMPEG = Join-Path $PSScriptRoot '.cache\tools\ffmpeg\ffmpeg-n8.1.3-23-g5c4f5744cd-win64-gpl-8.1\bin\ffmpeg.exe'
