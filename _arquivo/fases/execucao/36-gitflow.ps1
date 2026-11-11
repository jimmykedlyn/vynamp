<#
    36-gitflow.ps1 - passo 36 da FASE 3

    Instala a regra de Gitflow nos repos de codigo: .githooks/pre-commit
    autocontido, core.hooksPath = .githooks, e .gitignore com .worktrees/ e
    target/.

    O hook nao chama 'cargo xtask guard-branch' porque o xtask vive no umbrella:
    num repo de modulo o comando nao existe, e um hook que falha em rodar nao
    protege nada. A regex e copia literal de xtask/src/repo.rs:225.

    Este passo NAO cria, move, apaga nem faz push de branch, NAO mergeia,
    NAO tageia, NAO configura protecao de branch no GitHub e NAO mexe em
    visibilidade, assinatura ou plano. As protecoes sao do mantenedor.

    Uso:  pwsh -NoProfile -File .\36-gitflow.ps1
          .\36-gitflow.ps1 -WhatIf
#>
[CmdletBinding(SupportsShouldProcess = $true)]
param(
    [string]$Raiz = 'D:\Projects\vynamp',
    [string]$Dono = 'jimmykedlyn',
    # Pula a secao 5 (leitura de protecao de branch via gh). Sem gh autenticado
    # no ambiente, a chamada nao falha: ela fica esperando. Como a secao e
    # apenas relatorio, e ela que fica de fora, nunca a parte que escreve.
    [switch]$SemGitHub
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

# ---------------------------------------------------------------------------
# Encoding. Alvo: PowerShell 7+ (pwsh). O proprio .ps1 e lido como UTF-8 e
# Get-Content/Set-Content usam UTF-8 por padrao, entao a compatibilidade com
# o 5.1 -- BOM obrigatorio -- foi descartada de proposito, por custo. Os
# helpers continuam porque garantem UTF-8 SEM BOM na saida, que e o que os
# parsers de YAML do GitHub e o frontmatter dos agentes exigem.
# ---------------------------------------------------------------------------
$Utf8SemBom = New-Object System.Text.UTF8Encoding($false)

function Read-Utf8 {
    param([string]$Path)
    return [System.IO.File]::ReadAllText($Path, [System.Text.Encoding]::UTF8)
}
function Write-Utf8 {
    param([string]$Path, [string]$Text, [switch]$NoTrailingNewline)
    $dir = Split-Path $Path -Parent
    if ($dir -and -not (Test-Path $dir)) { New-Item -ItemType Directory -Path $dir -Force | Out-Null }
    if (-not $NoTrailingNewline -and -not $Text.EndsWith("`n")) { $Text = $Text + "`n" }
    [System.IO.File]::WriteAllText($Path, $Text, $Utf8SemBom)
}

function Write-Passo { param($m) Write-Host "==> $m" -ForegroundColor Cyan }
function Write-Feito { param($m) Write-Host "    $m" -ForegroundColor DarkGray }
function Write-Aviso { param($m) Write-Host "    ! $m" -ForegroundColor Yellow }

# ---------------------------------------------------------------------------
# Catalogo: os 8 repos de codigo. A referencia fica de fora (read-only).
# Os 6 novos nascem em D:\Projects\vynamp\... como submodulos; decode e
# output tem checkout proprio em D:\Projects\vynamp-<nome>.
# ---------------------------------------------------------------------------
$Modulos = @(
    @{ nome = 'vynamp-dsp';       caminho = 'libs/dsp' }
    @{ nome = 'vynamp-decode';    caminho = 'libs/decode' }
    @{ nome = 'vynamp-encode';    caminho = 'libs/encode' }
    @{ nome = 'vynamp-output';    caminho = 'libs/output' }
    @{ nome = 'vynamp-engine';    caminho = 'libs/engine' }
    @{ nome = 'vynamp-player';    caminho = 'apps/player' }
    @{ nome = 'vynamp-converter'; caminho = 'apps/converter' }
    @{ nome = 'vynamp-restore';   caminho = 'apps/restore' }
)

function Invoke-Git {
    param([string[]]$Argumentos, [string]$Onde)
    $out = & git -C $Onde @Argumentos 2>&1
    $code = $LASTEXITCODE
    if ($code -ne 0) {
        $detalhe = ($out | Out-String).Trim()
        throw "git $($Argumentos -join ' ') falhou (exit $code) em ${Onde}:`n$detalhe"
    }
    return ($out | Out-String).Trim()
}

# ---------------------------------------------------------------------------
# O hook. Regex identica a xtask/src/repo.rs:225, em POSIX ERE.
# ---------------------------------------------------------------------------
$Hook = @'
#!/bin/sh
# Regra de Gitflow local: commit so em branch de trabalho.
#
# Portao em shell, sem dependencia do umbrella: 'cargo xtask guard-branch' nao
# existe num repo de modulo, e um hook que falha em rodar nao protege nada.
# A regex e a mesma de xtask/src/repo.rs:225.
set -eu

# 'git rev-parse --abbrev-ref HEAD' falha com exit 128 em branch sem commit, e com
# 'set -eu' o hook morreria antes da mensagem. 'branch --show-current' responde no
# branch nao-nascido e devolve vazio em detached HEAD.
branch="$(git branch --show-current)"

if [ -z "$branch" ]; then
    echo "Commit bloqueado: HEAD destacado. Crie uma branch de trabalho." >&2
    exit 1
fi

if [ "$branch" = "main" ] || [ "$branch" = "develop" ]; then
    echo "Commit bloqueado em $branch. Trabalhe em feature/*, fix/*, chore/* ou hotfix/*." >&2
    exit 1
fi

if ! echo "$branch" | grep -Eq '^(feature|fix|chore|hotfix)/task-e[0-9]{2}-[0-9]{2}-claude-[a-z0-9]+(-[a-z0-9]+)*$|^chore/claude-[a-z0-9]+(-[a-z0-9]+)*$|^release/[0-9]+\.[0-9]+\.[0-9]+([.-][a-z0-9.-]+)?$'; then
    echo "Commit bloqueado: branch fora do Gitflow: $branch" >&2
    echo "Use feature/task-<id>-<slug>, fix/..., chore/..., hotfix/... ou release/X.Y.Z." >&2
    exit 1
fi
'@

# ---------------------------------------------------------------------------
# .gitignore de repo de modulo: alvo + worktree.
# ---------------------------------------------------------------------------
$Gitignore = @'
/target
.worktrees/
'@

# ---------------------------------------------------------------------------
# Resolve o checkout de cada repo: o submodulo no umbrella, quando existe;
# senao o checkout proprio em D:\Projects\vynamp-<nome>.
# ---------------------------------------------------------------------------
function Resolve-Checkout {
    param([string]$Nome, [string]$Caminho)
    $sub = Join-Path $Raiz $Caminho
    if (Test-Path (Join-Path $sub '.git')) { return $sub }
    $own = Join-Path 'D:\Projects' $Nome
    if (Test-Path (Join-Path $own '.git')) { return $own }
    return $null
}

# ---------------------------------------------------------------------------
# Pre-checagem
# ---------------------------------------------------------------------------
Write-Passo 'pre-checagem'

if (-not (Test-Path $Raiz)) { throw "Umbrella nao encontrado: $Raiz. Rode o passo 31 antes." }

$alvos = @()
foreach ($m in $Modulos) {
    $p = Resolve-Checkout -Nome $m.nome -Caminho $m.caminho
    if ($null -eq $p) {
        Write-Aviso "$($m.nome): checkout nao encontrado (nem submodule em $Raiz nem D:\Projects\$($m.nome)); pulado"
        continue
    }
    $alvos += @{ nome = $m.nome; caminho = $m.caminho; path = $p }
}
if ($alvos.Count -eq 0) { throw 'Nenhum repo de modulo encontrado. Rode o passo 35 antes.' }
Write-Feito "$($alvos.Count) repo(s) de modulo localizado(s)"

# ---------------------------------------------------------------------------
# 1. Instalar o hook
# ---------------------------------------------------------------------------
Write-Passo '1. instalando .githooks/pre-commit'

foreach ($a in $alvos) {
    $dirHooks = Join-Path $a.path '.githooks'
    $arqHook = Join-Path $dirHooks 'pre-commit'

    if ($PSCmdlet.ShouldProcess($a.nome, 'instalar .githooks/pre-commit')) {
        Write-Utf8 -Path $arqHook -Text $Hook
        Write-Feito "$($a.nome): hook gravado"
    } else {
        Write-Feito "$($a.nome): hook seria gravado (WhatIf)"
    }

    # Bit de execucao. No Windows o git usa o bit; sem ele o hook e ignorado.
    # Sem --add, 'update-index --chmod' falha com 'missing --add option?' quando
    # o arquivo ainda nao esta no index (repo recem-criado).
    & git -C $a.path update-index --add --chmod=+x -- '.githooks/pre-commit' 2>&1 | Out-Null
    if ($LASTEXITCODE -ne 0) {
        Write-Aviso "$($a.nome): nao foi possivel marcar o bit de execucao no index (o arquivo pode ainda estar novo)."
    }
}
Write-Host ''

# ---------------------------------------------------------------------------
# 2. core.hooksPath
# ---------------------------------------------------------------------------
Write-Passo '2. configurando core.hooksPath'

foreach ($a in $alvos) {
    # 'git config <chave>' sai 1 quando a chave NAO existe; nao e falha, e o
    # caso comum. Por isso esta leitura nao passa pelo Invoke-Git.
    $atual = (& git -C $a.path config core.hooksPath 2>$null | Out-String).Trim()
    if ($atual -eq '.githooks') {
        Write-Feito "$($a.nome): core.hooksPath ja e .githooks"
        continue
    }
    if ($PSCmdlet.ShouldProcess($a.nome, 'configurar core.hooksPath = .githooks')) {
        Invoke-Git @('config', 'core.hooksPath', '.githooks') -Onde $a.path | Out-Null
        Write-Feito "$($a.nome): core.hooksPath = .githooks"
    }
}
Write-Host ''

# ---------------------------------------------------------------------------
# 3. .gitignore
# ---------------------------------------------------------------------------
Write-Passo '3. atualizando .gitignore'

foreach ($a in $alvos) {
    $arq = Join-Path $a.path '.gitignore'
    $texto = if (Test-Path $arq) { Read-Utf8 $arq } else { '' }
    $faltando = @()
    if ($texto -notmatch '(?m)^/?target/?$') { $faltando += 'target/' }
    if ($texto -notmatch '(?m)^\.worktrees/?$') { $faltando += '.worktrees/' }

    if ($faltando.Count -eq 0) {
        Write-Feito "$($a.nome): .gitignore ja cobre $($faltando -join ', ')"
        continue
    }
    if (-not $PSCmdlet.ShouldProcess($a.nome, "adicionar ao .gitignore: $($faltando -join ', ')")) { continue }

    $novo = $texto
    if ($novo.Length -gt 0 -and -not $novo.EndsWith("`n")) { $novo = $novo + "`n" }
    foreach ($f in $faltando) { $novo = $novo + "$f`n" }
    Write-Utf8 -Path $arq -Text $novo
    Write-Feito "$($a.nome): .gitignore += $($faltando -join ', ')"
}
Write-Host ''

# ---------------------------------------------------------------------------
# 4. Relatorio: branches existentes e estado de protecao. Somente leitura.
# ---------------------------------------------------------------------------
Write-Passo '4. relatorio de branches (somente leitura)'

foreach ($a in $alvos) {
    $branch = (Invoke-Git @('rev-parse', '--abbrev-ref', 'HEAD') -Onde $a.path).Trim()
    $remotas = (Invoke-Git @('branch', '-r', '--list', 'origin/*') -Onde $a.path)
    $n = @($remotas -split "`n" | Where-Object { $_ }).Count
    # 'rev-parse --abbrev-ref' responde literalmente 'HEAD' em detached. Reportar
    # 'HEAD' como se fosse o nome da branch confunde o proprio diagnostico que
    # esta secao existe para dar.
    $exibicao = if ($branch -eq 'HEAD') { 'HEAD destacado' } else { $branch }
    Write-Feito "$($a.nome): branch atual = $exibicao; $n branch(es) remota(s)"

    # Branch nao Conventional: candidatas a limpeza. NADA e apagado (D-GF-8).
    $naoConf = @()
    foreach ($r in ($remotas -split "`n" | Where-Object { $_ })) {
        # 'git branch -r' emite 'HEAD -> origin/develop' para o symref do clone.
        # Nao e branch: e a seta. Sem este filtro o relatorio acusava uma branch
        # inexistente chamada 'HEAD -> origin/develop' em todos os 8 repos.
        if ($r -match 'HEAD ->') { continue }
        $b = ($r -replace '^\s*origin/', '').Trim()
        if ($b -eq 'HEAD') { continue }
        if ($b -notmatch '^(main|develop)$' -and $b -notmatch '^(feature|fix|chore|hotfix)/task-e\d{2}-\d{2}-claude-[a-z0-9]+(-[a-z0-9]+)*$' -and $b -notmatch '^(feature|fix|chore|hotfix)/[a-z0-9]+(-[a-z0-9]+)*$' -and $b -notmatch '^release/\d+\.\d+\.\d+') {
            $naoConf += $b
        }
    }
    if ($naoConf.Count -gt 0) {
        Write-Aviso "$($a.nome): branch(es) fora do Gitflow (candidatas a limpeza pelo mantenedor): $($naoConf -join ', ')"
    }
}
Write-Host ''

# ---------------------------------------------------------------------------
# 5. Relatorio de protecao de branch no GitHub. Somente leitura.
# ---------------------------------------------------------------------------
Write-Passo '5. relatorio de protecao de branch no GitHub (somente leitura)'

if ($SemGitHub) {
    Write-Aviso 'secao 5 pulada (-SemGitHub): relatorio de protecao nao lido'
} else {
foreach ($a in $alvos) {
    foreach ($b in @('main', 'develop')) {
        $saida = & gh api "repos/$Dono/$($a.nome)/branches/$b/protection" --jq '.required_status_checks.contexts // "sem gate"' 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Feito "$($a.nome)/$b : $($saida | Out-String)".Trim()
        } else {
            $txt = ($saida | Out-String)
            if ($txt -match 'Upgrade to GitHub Pro') {
                Write-Aviso "$($a.nome)/$b : protecao indisponivel no plano atual (403 do GitHub, nao do agente)"
            } elseif ($txt -match 'Branch not protected') {
                Write-Aviso "$($a.nome)/$b : sem protecao"
            } else {
                Write-Aviso "$($a.nome)/$b : nao foi possivel ler a protecao ($($txt.Trim()))"
            }
        }
    }
}
}
Write-Host ''
Write-Host '    Protecao de branch e decisao do mantenedor: este passo nao configura nada.' -ForegroundColor Yellow
Write-Host ''

# ---------------------------------------------------------------------------
# 6. Commitar dentro de cada repo (commit local, sem push)
# ---------------------------------------------------------------------------
Write-Passo '6. commitando em cada repo (local, sem push)'

foreach ($a in $alvos) {
    $sujo = (Invoke-Git @('status', '--short') -Onde $a.path)
    if (-not $sujo) {
        Write-Feito "$($a.nome): nada a commitar"
        continue
    }

    $branch = (Invoke-Git @('rev-parse', '--abbrev-ref', 'HEAD') -Onde $a.path).Trim()
    # 'HEAD' e o que o git responde em detached, e e o estado real dos 8 submodulos:
    # eles ficam pinados na tag v0.1.0, por desenho (D-GH-4). Sem tratar detached
    # aqui, o commit cairia fora de qualquer branch -- e o hook que este mesmo passo
    # instala o bloquearia, deixando o passo pela metade com o hook ja configurado.
    # Nao e o caso de main/develop apenas: e o caso de qualquer HEAD nao-nomeado.
    if ($branch -eq 'main' -or $branch -eq 'develop' -or $branch -eq 'HEAD') {
        $setup = 'chore/claude-gitflow-36'
        $existe = (& git -C $a.path rev-parse --verify --quiet "refs/heads/$setup") 2>$null
        $de = if ($branch -eq 'HEAD') { 'HEAD destacado' } else { $branch }
        if (-not $PSCmdlet.ShouldProcess($a.nome, "criar/trocar para $setup (estava em $de)")) { continue }
        if ($existe) { Invoke-Git @('switch', $setup) -Onde $a.path | Out-Null }
        else { Invoke-Git @('switch', '-c', $setup) -Onde $a.path | Out-Null }
        $branch = $setup
        Write-Feito "$($a.nome): branch $setup (estava em $de)"
    }

    if (-not $PSCmdlet.ShouldProcess($a.nome, 'commitar as regras de gitflow')) { continue }

    Invoke-Git @('add', '.githooks/pre-commit', '.gitignore') -Onde $a.path | Out-Null
    $msg = 'chore(gitflow): instala a regra de branch local (passo 36)'
    Invoke-Git @('commit', '-q', '-m', $msg) -Onde $a.path | Out-Null
    Write-Feito "$($a.nome): commit criado em $branch (sem push)"
}
Write-Host ''

# ---------------------------------------------------------------------------
# Verificacao
# ---------------------------------------------------------------------------
Write-Passo 'verificacao'

$erros = @()

foreach ($a in $alvos) {
    # 'git config <chave>' sai 1 quando a chave nao existe -- que e o estado
    # NORMAL antes deste passo. Passar pelo Invoke-Git, que lanca em exit != 0,
    # transformava "falta configurar" em excecao nao tratada na verificacao.
    $hooksPath = (& git -C $a.path config core.hooksPath 2>$null | Out-String).Trim()
    if ($hooksPath -ne '.githooks') {
        $erros += "$($a.nome): core.hooksPath = '$hooksPath' (esperado .githooks)"
    }

    $arqHook = Join-Path $a.path '.githooks\pre-commit'
    if (-not (Test-Path $arqHook)) {
        $erros += "$($a.nome): .githooks/pre-commit ausente"
        continue
    }
    $txtHook = Read-Utf8 $arqHook
    if ($txtHook -notmatch 'branch --show-current') {
        $erros += "$($a.nome): o hook nao usa 'branch --show-current' (unica forma que funciona em branch sem commit)"
    }
    # So conta linha EXECUTAVEL: o proprio comentario do hook cita 'cargo xtask'
    # para explicar por que ele nao e usado, e isso nao e defeito.
    $executaveis = @($txtHook -split "`n" | Where-Object { $_ -match 'cargo\s+xtask' -and $_ -notmatch '^\s*#' })
    if ($executaveis.Count -gt 0) {
        $erros += "$($a.nome): o hook ainda chama cargo xtask, que nao existe no repo de modulo"
    }
    $executable = (& git -C $a.path ls-files -s -- '.githooks/pre-commit') -match '^100755'
    if (-not $executable) {
        $erros += "$($a.nome): .githooks/pre-commit sem bit de execucao no index"
    }

    $arqGi = Join-Path $a.path '.gitignore'
    if (-not (Test-Path $arqGi)) {
        $erros += "$($a.nome): .gitignore ausente"
    } else {
        $gi = Read-Utf8 $arqGi
        if ($gi -notmatch '(?m)^\.worktrees/?$') { $erros += "$($a.nome): .gitignore sem .worktrees/" }
        if ($gi -notmatch '(?m)^/?target/?$') { $erros += "$($a.nome): .gitignore sem target/" }
    }
}

foreach ($e in $erros) { Write-Aviso $e }
if ($erros.Count -gt 0) {
    Write-Host ''
    Write-Host '    Verificacao com pendencias. Nada foi excluido e nada foi publicado.' -ForegroundColor Yellow
} else {
    Write-Host ''
    Write-Host '    Verificacao do passo 36 passou.' -ForegroundColor Green
}

Write-Host ''
Write-Host '    Commits locais, sem push. Nenhuma protecao de branch foi configurada.' -ForegroundColor Yellow
Write-Host ''
