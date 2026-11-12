$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'AMBIENTE.ps1')
$worktree = $VynampRoot
if (-not (Test-Path -LiteralPath (Join-Path $VynampRepo '.git'))) { throw 'Execute INICIAR.ps1 primeiro.' }
$promptPath = Join-Path $PSScriptRoot 'PROMPT-CLAUDE.txt'
if (-not (Test-Path -LiteralPath $promptPath -PathType Leaf)) { throw 'PROMPT-CLAUDE.txt nao encontrado.' }
$prompt = "Leia $promptPath por completo e siga suas instrucoes. Preserve o trabalho existente."
Write-Host 'Iniciando CLAUDE como dono unico do projeto.'
$env:CARGO_TARGET_DIR = Join-Path $VynampRepo 'target/driver'
Push-Location $worktree
try {
    Invoke-VynampAuthenticatedAgent -Action {
        Invoke-VynampCommand -Program 'claude' -Arguments @($prompt)
    }
}
finally { Pop-Location }
