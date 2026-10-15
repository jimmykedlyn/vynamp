$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'AMBIENTE.ps1')
$worktree = Join-Path $VynampRepo '.worktrees/codex/task-e15-01-bootstrap'
if (-not (Test-Path -LiteralPath (Join-Path $worktree '.git'))) { throw 'Execute INICIAR.ps1 primeiro.' }
$prompt = (Get-Content -LiteralPath (Join-Path $PSScriptRoot 'PROMPT-CODEX.txt') -Raw -Encoding UTF8) -replace '\r?\n', ' '
Write-Host 'Iniciando CODEX com o prompt de bootstrap/revisao.'
$env:CARGO_TARGET_DIR = Join-Path $worktree 'target/driver'
Push-Location $worktree
try { Invoke-VynampCommand -Program 'codex' -Arguments @($prompt) }
finally { Pop-Location }
