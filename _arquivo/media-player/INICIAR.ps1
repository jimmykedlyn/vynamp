$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'AMBIENTE.ps1')
if ($env:OS -eq 'Windows_NT') {
    $identity = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($identity)
    if ($principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
        throw 'Abra um PowerShell normal, sem Executar como administrador, para evitar ownership incorreto do Git.'
    }
}
foreach ($program in @('cargo', 'git', 'gh', 'claude')) {
    if (-not (Get-Command $program -ErrorAction SilentlyContinue)) { throw "Ferramenta ausente no PATH: $program" }
}
& gh auth status
if ($LASTEXITCODE -ne 0) { Invoke-VynampCommand -Program 'gh' -Arguments @('auth', 'login') }
$gitName = [string](& git config user.name)
$gitEmail = [string](& git config user.email)
if ([string]::IsNullOrWhiteSpace($gitName)) { $gitName = Read-Host 'Nome para os commits Git' }
if ([string]::IsNullOrWhiteSpace($gitEmail)) { $gitEmail = Read-Host 'Email para os commits Git' }
if ([string]::IsNullOrWhiteSpace($gitName) -or [string]::IsNullOrWhiteSpace($gitEmail)) { throw 'Nome/email Git sao necessarios.' }
$env:GIT_AUTHOR_NAME = $gitName.Trim()
$env:GIT_COMMITTER_NAME = $gitName.Trim()
$env:GIT_AUTHOR_EMAIL = $gitEmail.Trim()
$env:GIT_COMMITTER_EMAIL = $gitEmail.Trim()
$log = Join-Path (Join-Path $VynampRoot 'logs') ('inicio-' + (Get-Date -Format 'yyyyMMdd-HHmmss') + '.log')
$transcript = $false
try {
    Start-Transcript -Path $log | Out-Null
    $transcript = $true
    $receiptPath = Join-Path $VynampCache 'import-completed.json'
    if (-not (Test-Path -LiteralPath $VynampRepo)) {
        Push-Location $VynampPackage
        try { Invoke-VynampCommand -Program 'cargo' -Arguments @('xtask', 'github-import', '--destination', $VynampRepo, '--push') }
        finally { Pop-Location }
        # Written only after the importer completed ALL requested pushes/PR creation.
        @{ spec_version = '0.7.0'; repository = 'jimmykedlyn/vynamp-mediaplayer'; publication_completed = $true } | ConvertTo-Json | Set-Content -LiteralPath $receiptPath -Encoding UTF8
    } else {
        if (-not (Test-Path -LiteralPath (Join-Path $VynampRepo '.git'))) { throw 'A pasta repo existe mas nao e checkout Git. Preserve-a; nao sobrescrever.' }
        $remote = [string](& git -C $VynampRepo remote get-url origin)
        if ($LASTEXITCODE -ne 0 -or $remote.Trim() -ne 'https://github.com/jimmykedlyn/vynamp-mediaplayer.git') { throw 'Origin diferente do repositorio esperado; nao modificar automaticamente.' }
        $config = Get-Content -LiteralPath (Join-Path $VynampRepo 'config/project.json') -Raw -Encoding UTF8 | ConvertFrom-Json
        if ($config.spec_version -ne '0.7.0') { throw 'Checkout de outra versao; atualizar por PR, sem sobrescrever trabalho.' }
        if (-not (Test-Path -LiteralPath $receiptPath)) {
            throw 'Importacao interrompida: publicacao/PR nao confirmados. Nao iniciar agentes. Preserve repo renomeando para repo-interrompido dentro desta raiz e execute INICIAR novamente. Historico remoto e arquivos preservados; consulte o guia docs/engineering/09-windows-restart.md no pacote.'
        }
        $receipt = Get-Content -LiteralPath $receiptPath -Raw -Encoding UTF8 | ConvertFrom-Json
        if ($receipt.spec_version -ne '0.7.0' -or $receipt.repository -ne 'jimmykedlyn/vynamp-mediaplayer' -or $receipt.publication_completed -ne $true) { throw 'Recibo de importacao invalido; nao declarar preparacao concluida.' }
    }
    Invoke-VynampCommand -Program 'git' -Arguments @('-C', $VynampRepo, 'config', 'user.name', $gitName.Trim())
    Invoke-VynampCommand -Program 'git' -Arguments @('-C', $VynampRepo, 'config', 'user.email', $gitEmail.Trim())
    Write-Host ''
    Write-Host 'Preparacao concluida. Abra um terminal normal nesta pasta:'
    Write-Host $VynampRoot
    Write-Host 'powershell -NoProfile -ExecutionPolicy Bypass -File .\ABRIR-CLAUDE.ps1'
    Write-Host 'O prompt e enviado automaticamente; Claude e autor e revisor.'
} finally {
    if ($transcript) { Stop-Transcript | Out-Null }
}
