$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'AMBIENTE.ps1')
$worktree = Join-Path $VynampRepo '.worktrees/claude/review-bootstrap'
if (-not (Test-Path -LiteralPath (Join-Path $worktree '.git'))) { throw 'Execute INICIAR.ps1 primeiro.' }
$prompt = (Get-Content -LiteralPath (Join-Path $PSScriptRoot 'PROMPT-CLAUDE.txt') -Raw -Encoding UTF8) -replace '\r?\n', ' '
Write-Host 'Iniciando CLAUDE com o prompt de bootstrap/revisao.'
$env:CARGO_TARGET_DIR = Join-Path $worktree 'target/driver'
Push-Location $worktree
try { Invoke-VynampCommand -Program 'claude' -Arguments @($prompt) }
finally { Pop-Location }
