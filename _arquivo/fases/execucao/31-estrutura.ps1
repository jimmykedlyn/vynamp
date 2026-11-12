<#
    31-estrutura.ps1 — passo 31 da FASE 3

    Move o repo para D:\Projects\vynamp e reorganiza a arvore pelo mapa de destino.
    Sem submódulos (passo 35), sem push, sem merge, sem tag, sem gh repo edit.
    Arquivo de projeto so se move com git mv (historico preservado).

    Uso:  pwsh -NoProfile -File .\31-estrutura.ps1
          .\31-estrutura.ps1 -WhatIf

    -RepoAtual / -Destino existem para ENSAIO: apontam para uma copia do repo e
    um diretorio temporario, e o passo roda inteiro sem tocar no repo real. Foi
    assim que o passo foi verificado antes de ser executado.
#>
[CmdletBinding(SupportsShouldProcess = $true)]
param(
    [switch]$Forcar,
    [string]$RepoAtual = 'D:\Projects\vynamp-media-player\repo',
    [string]$Destino   = 'D:\Projects\vynamp'
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$ArquivoOld = Join-Path $RepoAtual '_arquivo'

function Write-Passo { param($m) Write-Host "==> $m" -ForegroundColor Cyan }
function Write-Feito { param($m) Write-Host "    $m" -ForegroundColor DarkGray }
function Write-Aviso { param($m) Write-Host "    ! $m" -ForegroundColor Yellow }

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
function Append-Utf8 {
    param([string]$Path, [string]$Text)
    # se o arquivo existe e nao termina em newline, o append colaria na ultima linha
    if ((Test-Path $Path) -and (Get-Item $Path).Length -gt 0) {
        $fs = [System.IO.File]::OpenRead($Path)
        try {
            $fs.Seek(-1, [System.IO.SeekOrigin]::End) | Out-Null
            if ($fs.ReadByte() -ne 10) { $Text = "`n" + $Text }
        } finally { $fs.Dispose() }
    }
    if (-not $Text.EndsWith("`n")) { $Text = $Text + "`n" }
    [System.IO.File]::AppendAllText($Path, $Text, $Utf8SemBom)
}

if (-not (Test-Path $RepoAtual)) { throw "Repo nao encontrado: $RepoAtual" }

# ------------------------------------------------------------------ 0. pre-checagem
Write-Passo 'pre-checagem'

Push-Location $RepoAtual
try {
    $branch = (git rev-parse --abbrev-ref HEAD).Trim()
    if ($branch -ne 'develop') {
        # O passo 30 e substancial e pode chegar via PR de branch de passo, como
        # os passos 30-38 inteiros. O que a mudanca do diretorio NAO pode levar
        # embora e commit que nao esteja em develop -- senao o umbrella nasce
        # com historia que o develop do GitHub nunca viu.
        $foraDoDevelop = git rev-list --count "origin/develop..$branch" 2>$null
        if (-not $foraDoDevelop -or [int]$foraDoDevelop -ne 0) {
            throw "Branch '$branch' tem $foraDoDevelop commit(s) fora de origin/develop. Faca o merge do PR do passo 30 (o mantenedor, pelo GitHub) e rode de novo a partir de develop."
        }
        Write-Aviso "branch '$branch', nao develop, mas todo commit ja esta em origin/develop"
    }
    Write-Feito "branch: $branch"

    $dirty = git status --short
    if ($dirty) { throw "Arvore suja. Commite o passo 30 antes de mover o repo.`n$($dirty -join "`n")" }
    Write-Feito 'arvore limpa'

    $wts = git worktree list --porcelain |
        Where-Object { $_ -like 'worktree *' } |
        ForEach-Object { $_.Substring(9).Trim() } |
        Where-Object { $_ -ne $RepoAtual.Replace('\', '/') -and $_ -ne $RepoAtual }
    if ($wts) {
        Write-Aviso 'worktrees ativas:'
        $wts | ForEach-Object { Write-Host "      $_" }
        throw 'Remova as worktrees antes de mover o repo (git worktree remove <path>).'
    }
    Write-Feito 'nenhuma worktree ativa'

    # Uma branch de worktree que existe so no disco morre com o worktree. Antes de
    # mandar remover, confere que cada branch nao-trancada ja esta no remoto.
    $branches = git for-each-ref --format='%(refname:short)' refs/heads
    $naoPublicadas = @()
    foreach ($b in $branches) {
        if ($b -in @('develop', 'main')) { continue }
        $remoto = git rev-parse --verify --quiet "origin/$b"
        if ($LASTEXITCODE -ne 0 -or -not $remoto) {
            # Sem origin/<b> pode ser branch local avulsa OU branch cujo PR ja
            # foi mergeado e apagado no remoto. A segunda nao se perde: os
            # commits estao em origin/develop. So a primeira e perda.
            $emDevelop = git rev-list --count "origin/develop..$b" 2>$null
            $soLocal = (git rev-list --count "$b" --not --remotes) 2>$null
            if ($emDevelop -and [int]$emDevelop -eq 0) {
                Write-Aviso "$b sem origin/$b, mas todo commit ja esta em origin/develop (PR mergeado); nao se perde"
            } elseif ($soLocal -and [int]$soLocal -eq 0) {
                Write-Aviso "$b sem origin/$b, mas nenhum commit e exclusivo dela; nao se perde"
            } else {
                $naoPublicadas += "$b (sem origin/$b e $soLocal commit(s) fora de todo remoto)"
            }
        } else {
            $pend = git rev-list --count "origin/$b..$b"
            if ([int]$pend -gt 0) { $naoPublicadas += "$b ($pend commit(s) nao empurrados)" }
        }
    }
    if ($naoPublicadas) {
        Write-Aviso 'branches locais que se perderiam:'
        $naoPublicadas | ForEach-Object { Write-Host "      $_" }
        if (-not $Forcar) {
            throw 'Publique ou descarte essas branches antes de mover o repo (git push -u origin <branch>).'
        }
        Write-Aviso 'seguindo com -Forcar'
    } else {
        Write-Feito 'toda branch local tem origin correspondente e nada pendente'
    }

    # os 5 PRs mergeados?
    $adr014 = Test-Path (Join-Path $RepoAtual 'docs/architecture/adr/ADR-014-public-github-actions-and-in-app-dsp.md')
    $adr015 = Test-Path (Join-Path $RepoAtual 'docs/architecture/adr/ADR-015-ai-restoration-future-phase.md')
    $tools03 = Test-Path (Join-Path $RepoAtual 'docs/tools/03-vynamp-restore-ia.md')
    if (-not ($adr014 -and $adr015 -and $tools03)) {
        Write-Aviso "ADR-014: $adr014 | ADR-015: $adr015 | tools/03: $tools03"
        if (-not $Forcar) {
            throw 'Os 5 PRs (lista D2) nao estao todos mergeados. Rode -Forcar so se souber o efeito.'
        }
        Write-Aviso 'seguindo com -Forcar'
    } else {
        Write-Feito 'ADR-014, ADR-015 e tools/03 presentes'
    }
} finally { Pop-Location }

# ------------------------------------------------------------------ 1. mover o repo
Write-Passo "movendo o repo para $Destino"

if (Test-Path $Destino) {
    $conteudo = Get-ChildItem -Path $Destino -Force -ErrorAction SilentlyContinue
    if ($conteudo -and -not $Forcar) {
        throw "Destino nao esta vazio: $Destino ($($conteudo.Count) item(ns)). Use -Forcar se for resto de tentativa anterior."
    }
    if ($conteudo) { Write-Aviso "destino tem $($conteudo.Count) item(ns); seguindo com -Forcar" }
}

# Inventario da arvore ANTIGA, colhido antes do Move-Item. Vai ser lido pela
# reescrita de links (6b), que roda depois de todo git mv -- quando o caminho
# antigo ja nao existe mais nem no disco nem no indice.
$arvoreAntiga = New-Object 'System.Collections.Generic.HashSet[string]'
Push-Location $RepoAtual
try {
    foreach ($f in (git ls-tree -r --name-only HEAD)) {
        [void]$arvoreAntiga.Add($f)
        $d = $f
        while ($d.Contains('/')) {
            $d = $d.Substring(0, $d.LastIndexOf('/'))
            [void]$arvoreAntiga.Add($d)
        }
    }
} finally { Pop-Location }

if ($PSCmdlet.ShouldProcess($RepoAtual, "mover para $Destino")) {
    if (-not (Test-Path $Destino)) { New-Item -ItemType Directory -Path $Destino -Force | Out-Null }
    # Move-Item do diretorio inteiro: o .git vai junto, historico intacto.
    # Duas excecoes, ambas de build: /target/ (3,1 GB) e .worktrees/ (11 GB).
    # Elas ficam para tras e o passo 38 as apaga. Mover 14 GB para apagar no
    # destino custaria duas vezes o mesmo tempo e o mesmo espaco em disco.
    # Nao sao versionadas: o .git nao perde nada com a ausencia delas.
    $deixarParaTras = @('target', '.worktrees')
    Get-ChildItem -Path $RepoAtual -Force |
        Where-Object { $deixarParaTras -notcontains $_.Name } |
        Move-Item -Destination $Destino -Force
    foreach ($d in $deixarParaTras) {
        if (Test-Path (Join-Path $RepoAtual $d)) {
            Write-Aviso "ficou em $RepoAtual\$d (build); o passo 38 apaga"
        }
    }
    Write-Feito 'conteudo movido; o .git foi junto'
}

Push-Location $Destino
try {

# ------------------------------------------------------------------ 2. estrutura nova
Write-Passo 'criando a arvore nova'

$dirs = @(
    'architecture', 'adr', 'product', 'delivery', 'design', 'engineering',
    'extensions', 'library', 'platforms', 'quality', 'security', 'streaming',
    'evidence', 'tasks', 'tasks/items', 'tasks/epics',
    'libs', 'apps', 'reference', 'xtask', 'config', 'schemas', 'packaging', 'assets', 'tests',
    '_arquivo', '.github/workflows'
)
foreach ($d in $dirs) {
    $p = Join-Path $Destino $d
    if (-not (Test-Path $p)) { New-Item -ItemType Directory -Path $p -Force | Out-Null }
}
Write-Feito "$($dirs.Count) diretorios"

# ------------------------------------------------------------------ 3. movimentos
Write-Passo 'movendo arquivos pelo mapa de destino'

# destino explicito por arquivo: origem -> destino (vazio = nao mover, ja arquivado)
$mapa = [ordered]@{
    'docs/INDEX.md'                                = 'INDEX.md'
    'docs/GLOSSARY.md'                             = 'architecture/GLOSSARY.md'

    'docs/architecture/01-system.md'               = 'architecture/01-sistema.md'
    'docs/architecture/02-stack.md'                = 'architecture/02-stack.md'
    'docs/architecture/03-data.md'                 = 'architecture/03-dados.md'
    'docs/architecture/04-contracts.md'            = 'architecture/04-contratos.md'

    'docs/product/01-vision-scope.md'              = 'product/01-visao-escopo.md'
    'docs/product/02-reference-benchmark.md'       = 'product/02-benchmark.md'
    'docs/product/03-requirements.md'              = 'product/03-requisitos.md'
    'docs/product/04-user-journeys.md'             = 'product/04-jornadas.md'
    'docs/product/05-decisions.md'                 = 'product/05-decisoes.md'
    'docs/product/06-feature-parity.md'            = 'product/06-paridade.md'

    'docs/delivery/01-roadmap.md'                  = 'delivery/01-roadmap.md'
    'docs/delivery/02-release-devops.md'           = 'delivery/02-release-devops.md'
    'docs/delivery/03-risks-open-questions.md'     = 'delivery/03-riscos.md'
    'docs/delivery/04-capability-release-gates.md' = 'delivery/04-gates.md'

    'docs/design/01-visual-system.md'              = 'design/01-sistema-visual.md'
    'docs/design/02-layouts.md'                    = 'design/02-layouts.md'
    'docs/design/03-accessibility-i18n.md'         = 'design/03-acessibilidade.md'
    'docs/design/04-brand-identity.md'             = 'design/04-marca.md'
    'docs/design/04-mobile-tablet.md'              = 'design/05-mobile-tablet.md'
    'docs/design/05-dsp-panel.md'                  = 'design/06-painel-dsp.md'
    'docs/design/06-mvp-reference.md'              = 'design/07-pin-mvp.md'

    'docs/engineering/02-worktrees.md'             = 'engineering/02-worktrees.md'
    'docs/engineering/03-gitflow-github.md'        = 'engineering/03-gitflow.md'
    'docs/engineering/04-ci-cd.md'                 = 'engineering/04-ci.md'
    'docs/engineering/05-tests.md'                 = 'engineering/05-testes.md'
    'docs/engineering/07-secrets-activation.md'    = 'engineering/07-segredos.md'
    'docs/engineering/09-windows-restart.md'       = 'engineering/08-reinicio-windows.md'

    'docs/extensions/01-themes.md'                 = 'extensions/01-temas.md'
    'docs/extensions/02-plugins.md'                = 'extensions/02-plugins.md'
    'docs/extensions/04-sdk-contract.md'           = 'extensions/04-sdk.md'
    'docs/extensions/04-visualizer-contract.md'    = 'extensions/05-visualizador.md'

    'docs/library/01-library-playlists.md'         = 'library/01-biblioteca-playlists.md'
    'docs/library/02-tags-lyrics-tools.md'         = 'library/02-tags-letras.md'

    'docs/platforms/01-support-matrix.md'          = 'platforms/01-matriz-suporte.md'
    'docs/platforms/02-android.md'                 = 'platforms/02-android.md'
    'docs/platforms/03-system-integration.md'      = 'platforms/03-integracao-sistema.md'
    'docs/platforms/04-activation-ownership.md'    = 'platforms/04-ativacao-ownership.md'

    'docs/quality/01-test-strategy.md'             = 'quality/01-estrategia-testes.md'
    'docs/quality/02-performance-observability.md' = 'quality/02-performance.md'
    'docs/quality/03-multiplatform-matrix.md'      = 'quality/03-matriz-multiplataforma.md'
    'docs/quality/05-dsp-test-matrix.md'           = 'quality/04-matriz-dsp.md'

    'docs/security/01-threat-model.md'             = 'security/01-threat-model.md'
    'docs/security/02-privacy-licensing.md'        = 'security/02-privacidade-licencas.md'

    'docs/streaming/01-client.md'                  = 'streaming/01-cliente.md'
    'docs/streaming/02-podcasts-recording.md'      = 'streaming/02-podcasts-gravacao.md'
    'docs/streaming/03-platform.md'                = 'streaming/03-plataforma.md'
    'docs/streaming/05-network-proxy-and-containers.md' = 'streaming/04-rede-proxy.md'
}

$movidos = 0
foreach ($origem in $mapa.Keys) {
    $dstRel = $mapa[$origem]
    $src = Join-Path $Destino $origem
    $dst = Join-Path $Destino $dstRel
    if (-not (Test-Path $src)) { Write-Aviso "inexistente, pulando: $origem"; continue }
    if (Test-Path $dst) { Write-Aviso "destino ja existe, pulando: $dstRel"; continue }
    $dstDir = Split-Path $dst -Parent
    if (-not (Test-Path $dstDir)) { New-Item -ItemType Directory -Path $dstDir -Force | Out-Null }

    if ($PSCmdlet.ShouldProcess($origem, "git mv -> $dstRel")) {
        git mv -- $origem $dstRel
        if ($LASTEXITCODE -ne 0) { throw "git mv falhou: $origem -> $dstRel" }
        $movidos++
    }
}
Write-Feito "$movidos arquivo(s) movido(s)"

# ADRs
Write-Passo 'movendo ADRs'
$adrMapa = [ordered]@{
    'ADR-001-independent-product.md'                = 'ADR-001-produto-independente.md'
    'ADR-002-audio-core.md'                         = 'ADR-002-audio-core.md'
    'ADR-003-ui-themes.md'                          = 'ADR-003-temas.md'
    'ADR-004-extensions.md'                         = 'ADR-004-extensoes.md'
    'ADR-005-streaming-platform.md'                 = 'ADR-005-streaming.md'
    'ADR-006-multiplatform.md'                      = 'ADR-006-multiplataforma.md'
    'ADR-007-development-delivery.md'               = 'ADR-007-entrega.md'
    'ADR-008-vynamp-brand.md'                       = 'ADR-008-marca.md'
    'ADR-009-engineering-baseline.md'               = 'ADR-009-baseline.md'
    'ADR-011-own-audio-stack.md'                    = 'ADR-011-pilha-propria.md'
    'ADR-012-shell-integration.md'                  = 'ADR-012-integracao-shell.md'
    'ADR-013-vynamp-restore.md'                     = 'ADR-013-restore.md'
    'ADR-014-public-github-actions-and-in-app-dsp.md' = 'ADR-014-publico-dsp-interno.md'
}
foreach ($de in $adrMapa.Keys) {
    $src = "docs/architecture/adr/$de"
    $dst = "adr/$($adrMapa[$de])"
    if (-not (Test-Path (Join-Path $Destino $src))) { Write-Aviso "ADR ausente: $de"; continue }
    if ($PSCmdlet.ShouldProcess($src, "git mv -> $dst")) {
        git mv -- $src $dst
        if ($LASTEXITCODE -ne 0) { throw "git mv falhou: $src" }
    }
}
# ADR-015 vem do merge, pode existir
$adr015 = 'docs/architecture/adr/ADR-015-ai-restoration-future-phase.md'
if (Test-Path (Join-Path $Destino $adr015)) {
    git mv -- $adr015 'adr/ADR-015-restauracao-ia.md'
    Write-Feito 'ADR-015 movida'
}
Write-Feito 'ADRs movidas'

# evidencia
Write-Passo 'movendo evidencia'
$evid = git ls-files 'docs/evidence/*'
foreach ($f in $evid) {
    $rel = $f -replace '^docs/evidence/', 'evidence/'
    $dstDir = Split-Path (Join-Path $Destino $rel) -Parent
    if (-not (Test-Path $dstDir)) { New-Item -ItemType Directory -Path $dstDir -Force | Out-Null }
    if ($PSCmdlet.ShouldProcess($f, "git mv -> $rel")) {
        git mv -- $f $rel
        if ($LASTEXITCODE -ne 0) { Write-Aviso "falhou: $f" }
    }
}
Write-Feito 'evidencia movida'

# ------------------------------------------------------------------ 3b. fusões
Write-Passo 'aplicando as fusoes'

# O passo 30 deixou 17 arquivos vivos DE PROPOSITO, porque este passo le o
# conteudo deles. Medido em 2026-11-07: das 5 "fusoes" do plano original so
# UMA e duplicata de verdade. As outras quatro tem conteudo unico que se
# perderia -- por isso aqui elas sao ACRESCIMO ao destino, nunca substituicao.
#
# Como o conteudo e lido antes de o arquivo ir para _arquivo/, e preciso que
# a origem exista. Se o passo 30 ja arquivou, le de _arquivo/repo/.
function Get-ConteudoOrigem {
    param([string]$Rel)
    $vivo = Join-Path $Destino $Rel
    if (Test-Path $vivo) { return @{ Texto = (Read-Utf8 $vivo); Origem = $vivo } }
    $arquivado = Join-Path $Destino "_arquivo/repo/$Rel"
    if (Test-Path $arquivado) {
        Write-Aviso "lendo do arquivado: $Rel"
        return @{ Texto = (Read-Utf8 $arquivado); Origem = $arquivado }
    }
    return $null
}

function Add-Secao {
    param([string]$DestRel, [string]$Titulo, [string]$Corpo)
    $p = Join-Path $Destino $DestRel
    if (-not (Test-Path $p)) { Write-Aviso "destino da fusao ausente: $DestRel"; return }
    $atual = Read-Utf8 $p
    # idempotente: nao duplica secao que ja esta la
    if ($atual.Contains($Titulo)) { Write-Aviso "secao ja existe em $DestRel : $Titulo"; return }
    $bloco = "`n## $Titulo`n`n" + $Corpo.Trim() + "`n"
    if ($PSCmdlet.ShouldProcess($DestRel, "acrescentar secao: $Titulo")) {
        Append-Utf8 -Path $p -Text $bloco
        Write-Feito "secao acrescentada em $DestRel : $Titulo"
    }
}

# --- 1. UNICA fusao real: as 12 linhas QA-022..033 sao byte-identicas as de
#        quality/01. O resto do arquivo (3 secoes de prosa, 2227 chars) e unico.
$q04 = Get-ConteudoOrigem 'docs/quality/04-integration-codecs-dsp.md'
if ($q04) {
    $todas = @(($q04.Texto -split "`n") | Where-Object { $_ -match '^\| QA-0\d\d \|' })
    $alvo  = @($todas | Where-Object { $_ -match '^\| QA-0(2[2-9]|3[0-3]) \|' })
    Write-Feito "quality/04: $($alvo.Count) de $($todas.Count) linhas QA a fundir"
    $prosa = @()
    $cap = $false
    foreach ($l in ($q04.Texto -split "`n")) {
        if ($l -match '^## (Entrada pelos sistemas|Corpus|DSP, render e rede)') { $cap = $true; $prosa += $l; continue }
        if ($l -match '^## ') { $cap = $false }
        if ($cap) { $prosa += $l }
    }
    if ($prosa.Count -gt 0) {
        $corpo = ($prosa -join "`n") -replace '(?m)^## (Entrada pelos sistemas|Corpus[^\n]*|DSP, render e rede)\s*$', '### $1'
        Add-Secao -DestRel 'quality/01-estrategia-testes.md' `
                  -Titulo 'Cenarios complementares de integracao, codecs e DSP' -Corpo $corpo
        Write-Feito "quality/04: $($prosa.Count) linha(s) de prosa unica preservada(s)"
    }
}

# --- 2. engineering/06-installers.md: NAO e duplicata. delivery/04 tem o
#        manifesto; o 06 tem formatos, recipes e assinatura (2618 chars).
$i06 = Get-ConteudoOrigem 'docs/engineering/06-installers.md'
if ($i06) {
    $linhas = @(($i06.Texto -split "`n") | Where-Object {
        $_ -notmatch '^# Instaladores' -and $_ -notmatch '^Vers.o: 0\.7\.0'
    })
    Add-Secao -DestRel 'delivery/04-gates.md' `
              -Titulo 'Instaladores: formatos, recipes e assinatura' -Corpo ($linhas -join "`n")
}

# --- 3. streaming/04-api-semantics.md: os endpoints JA estao em
#        architecture/04 secao Provider API v1; as 6 secoes de semantica sao unicas.
$s04 = Get-ConteudoOrigem 'docs/streaming/04-api-semantics.md'
if ($s04) {
    $linhas = @(($s04.Texto -split "`n") | Where-Object {
        $_ -notmatch '^# Sem.ntica da API' -and $_ -notmatch '^Vers.o: 0\.7\.0'
    })
    Add-Secao -DestRel 'architecture/04-contratos.md' `
              -Titulo 'Semantica da API da plataforma (M4)' -Corpo ($linhas -join "`n")
}

# --- 4. extensions/03-visualizations-integrations.md: 113 dos 162 termos sao
#        exclusivos. Cobre SMTC/MPRIS/MediaSession, Discord, VST3, remote control.
$e03 = Get-ConteudoOrigem 'docs/extensions/03-visualizations-integrations.md'
if ($e03) {
    $linhas = @(($e03.Texto -split "`n") | Where-Object {
        $_ -notmatch '^# Visualiza..es e integra' -and $_ -notmatch '^Vers.o: 0\.7\.0'
    })
    Add-Secao -DestRel 'extensions/05-visualizador.md' `
              -Titulo 'Integracoes de sistema e visualizacoes' -Corpo ($linhas -join "`n")
}

# --- 5. audio/05-platform-backends.md: 0 linhas comuns com audio/04. Nao funde
#        em nada aqui: e spec de libs/output, no passo 35. So avisa.
if (Test-Path (Join-Path $Destino 'docs/audio/05-platform-backends.md')) {
    Write-Aviso 'audio/05 nao e duplicata de audio/04 (0 linhas comuns); vira spec de libs/output no passo 35'
}

# --- 6. destino dos 13 de docs/audio e docs/tools: reference/, NAO _arquivo/.
#        247 links vivos em 71 arquivos apontam para eles, e o check-docs do
#        xtask reprova qualquer link que caia em _arquivo/. reference/ e
#        versionado, fica fora do build (exclude no Cargo.toml) e e onde o
#        passo 35/37 vai ler a spec congelada de cada modulo. Nada se perde e
#        nenhum link vira texto morto.
$moverArquivo = @()
$paraReference = @{}
foreach ($r in @('audio', 'tools')) {
    $p = Join-Path $Destino "docs/$r"
    if (-not (Test-Path $p)) { continue }
    foreach ($f in (Get-ChildItem -Path $p -Recurse -File -Filter *.md)) {
        $rel = $f.FullName.Substring($Destino.Length + 1).Replace('\', '/')
        $destinoRef = $rel -replace '^docs/', 'reference/'
        $paraReference[$rel] = $destinoRef
    }
}
foreach ($rel in $paraReference.Keys) {
    $dst = $paraReference[$rel]
    $dstDir = Split-Path (Join-Path $Destino $dst) -Parent
    if (-not (Test-Path $dstDir)) { New-Item -ItemType Directory -Path $dstDir -Force | Out-Null }
    if ($PSCmdlet.ShouldProcess($rel, "git mv -> $dst")) {
        git mv -- $rel $dst
        if ($LASTEXITCODE -ne 0) { Write-Aviso "falhou ao mover para reference/: $rel" }
    }
}
Write-Feito "$($paraReference.Count) spec(s) de audio/tools movida(s) para reference/"

# As 5 origens de fusao vao para _arquivo/: o conteudo ja foi copiado para os
# destinos acima, e nenhum .md vivo pode apontar para _arquivo/.
foreach ($f in @(
    'docs/engineering/01-agents-superpowers.md'
    'docs/engineering/06-installers.md'
    'docs/extensions/03-visualizations-integrations.md'
    'docs/quality/04-integration-codecs-dsp.md'
    'docs/streaming/04-api-semantics.md'
)) {
    if (Test-Path (Join-Path $Destino $f)) { $moverArquivo += $f }
}
foreach ($rel in $moverArquivo) {
    $dst = "_arquivo/repo/$rel"
    $dstDir = Split-Path (Join-Path $Destino $dst) -Parent
    if (-not (Test-Path $dstDir)) { New-Item -ItemType Directory -Path $dstDir -Force | Out-Null }
    if ($PSCmdlet.ShouldProcess($rel, "git mv -> $dst")) {
        git mv -- $rel $dst
        if ($LASTEXITCODE -ne 0) { Write-Aviso "falhou ao arquivar: $rel" }
    }
}
Write-Feito "$($moverArquivo.Count) origem(ns) de fusao arquivada(s) em _arquivo/repo/"
Write-Feito 'fusoes aplicadas'

if (Test-Path (Join-Path $Destino 'docs')) {
    $resto = git ls-files 'docs/*'
    if (-not $resto) {
        if ($PSCmdlet.ShouldProcess('docs/', 'remover diretorio vazio')) {
            Remove-Item -Path (Join-Path $Destino 'docs') -Recurse -Force -ErrorAction SilentlyContinue
        }
        Write-Feito 'docs/ removido (vazio)'
    } else {
        Write-Aviso "docs/ nao esta vazio, restam $($resto.Count) arquivo(s):"
        $resto | ForEach-Object { Write-Host "      $_" }
    }
}

# ------------------------------------------------------------------ 4. libs/apps/reference
Write-Passo 'criando marcadores de submodulo'

$marcadores = @(
    @{ p = 'libs/LEIA-ME.md'; t = @'
# libs

Cada subdiretorio e um submodulo fixado em tag de release.
Criados no passo 35, depois dos remotes.

- `dsp/` -> vynamp-dsp
- `decode/` -> vynamp-decode
- `encode/` -> vynamp-encode
- `output/` -> vynamp-output
- `engine/` -> vynamp-engine
'@ },
    @{ p = 'apps/LEIA-ME.md'; t = @'
# apps

Cada subdiretorio e um submodulo fixado em tag de release.
Criados no passo 35, depois dos remotes.

- `player/` -> vynamp-player
- `converter/` -> vynamp-converter
- `restore/` -> vynamp-restore
'@ },
    @{ p = 'reference/LEIA-ME.md'; t = @'
# reference

Referencia read-only, fora do workspace e do build.

- `vynamp-mvp/` -> vynamp-mvp-reference (pin 741316f, rev.38) — submodulo, passo 35
- `audio/` e `tools/` — specs dos modulos de audio e das ferramentas, congeladas

`audio/` e `tools/` vieram de `docs/audio` e `docs/tools` no passo 31. Nao sao
dominio do umbrella: viram spec de `libs/*` e `apps/*` quando cada repositorio
de modulo for criado, e o arquivo correspondente sai daqui naquele passo. Ate
la ficam aqui porque 247 links vivos de `architecture/`, `product/`, `quality/`
e `tasks/` apontam para eles, e `_arquivo/` nao pode receber link.

Nunca editar. Se a referencia divergir da spec, a spec registra a divergencia e o motivo.
'@ }
)
foreach ($m in $marcadores) {
    $p = Join-Path $Destino $m.p
    if (-not (Test-Path $p)) { Write-Utf8 -Path $p -Text $m.t }
}
Write-Feito 'marcadores criados'

# ------------------------------------------------------------------ 5. Cargo.toml
Write-Passo 'workspace virtual'

$cargoPath = Join-Path $Destino 'Cargo.toml'
if (Test-Path $cargoPath) {
    # Sem backup .bak: o Cargo.toml antigo e 5 linhas e esta no historico do git
    # (`git show HEAD:Cargo.toml`). Uma copia .bak ficaria untracked na raiz do
    # umbrella para sempre, e o passo 38 teria de limpa-la.
    $cargo = @'
[workspace]
resolver = "3"
members = ["xtask"]
default-members = ["xtask"]

# libs/* e apps/* NAO sao membros: sao repositorios independentes em submodulos.
# Cada um tem seu proprio Cargo.toml e compila sozinho (ver AGENTS.md secao 1.2).
exclude = ["libs/*", "apps/*", "reference/*"]

# Sem [workspace.package] e sem [workspace.lints]: o xtask/Cargo.toml declara
# edition, rust-version, license e as dependencias por extenso, sem nenhum
# `.workspace = true`. Hipotese de workspace que ninguem herda e configuracao
# morta: passa no cargo metadata e nao faz nada.
'@
    if ($PSCmdlet.ShouldProcess($cargoPath, 'reescrever como workspace virtual')) {
        Write-Utf8 -Path $cargoPath -Text $cargo
        Write-Feito 'Cargo.toml reescrito; o antigo continua em git show HEAD:Cargo.toml'
    }
}

# ------------------------------------------------------------------ 6. README
Write-Passo 'reescrevendo README.md'

$readme = @'
# Vynamp

Player de audio multiplataforma em Rust, com ferramentas de conversao e restauracao.
O projeto e dividido em **um repositorio por modulo**, cada um com gitflow, spec e CI
proprios.

## Estado

Spec avancada; o codigo real existe em `vynamp-decode` e `vynamp-output`, em
branches de feature. As specs de modulo ainda nao publicadas ficam em
`reference/` (`audio/` e `tools/`), congeladas, e o passo correspondente do
plano de migracao as leva para o repositorio de cada modulo.

## Estrutura

| Caminho | Repo | Tipo |
|---|---|---|
| `libs/dsp` | `vynamp-dsp` | primitivas de sinal, sem dominio |
| `libs/decode` | `vynamp-decode` | decodificacao para PCM |
| `libs/encode` | `vynamp-encode` | codificacao |
| `libs/output` | `vynamp-output` | saida para device |
| `libs/engine` | `vynamp-engine` | integracao, dominio, DSP |
| `apps/player` | `vynamp-player` | player |
| `apps/converter` | `vynamp-converter` | conversor em lote |
| `apps/restore` | `vynamp-restore` | restauracao |
| `reference/vynamp-mvp` | `vynamp-mvp-reference` | referencia read-only |

Este repositorio contem as specs que atravessam modulos (`architecture/`, `product/`,
`design/`, ...), as ADRs, as TASKs cross-modulo e a ferramentaria (`xtask`).

## Direcao de dependencia

```
apps -> engine -> { decode, output, encode, dsp }
```

`decode`, `encode` e `output` nao dependem de nenhum crate `vynamp-*` de dominio.
Cada lib compila sozinha: `git clone` + `cargo test`, sem este repositorio.

## Comecar

Leia [AGENTS.md](AGENTS.md) — o contrato do agente, e tambem o melhor mapa da
arquitetura. Depois [SPEC.md](SPEC.md) e [tasks/NEXT.md](tasks/NEXT.md).

## Licenca

MIT ou Apache-2.0, a escolher por quem usa.
'@
Write-Utf8 -Path (Join-Path $Destino 'README.md') -Text $readme
Write-Feito 'README.md reescrito'

# ------------------------------------------------------------------ 6b. links
Write-Passo 'reescrevendo links relativos'

# Medido antes de escrever: sem esta secao, 1648 links relativos apontam para
# caminho que nao existe mais e cargo xtask check-docs reprova em 261 arquivos.
# A reescrita e mecanica: para cada link relativo de cada .md vivo, resolve o
# caminho na arvore ANTIGA, aplica o mapa e recalcula o link relativo a partir
# do caminho NOVO do arquivo que contem o link.
#
# Roda depois de todo git mv e antes de qualquer verificacao. E idempotente:
# link cujo alvo ja existe no destino nao e tocado.

# mapa completo origem->destino, incluindo o que foi para _arquivo/
$mapaCompleto = @{}
foreach ($k in $mapa.Keys) { $mapaCompleto[$k] = $mapa[$k] }
foreach ($de in $adrMapa.Keys) { $mapaCompleto["docs/architecture/adr/$de"] = "adr/$($adrMapa[$de])" }
if (Test-Path (Join-Path $Destino 'adr/ADR-015-restauracao-ia.md')) {
    $mapaCompleto['docs/architecture/adr/ADR-015-ai-restoration-future-phase.md'] = 'adr/ADR-015-restauracao-ia.md'
}
foreach ($f in $moverArquivo) { $mapaCompleto[$f] = "_arquivo/repo/$f" }
foreach ($f in @(
    'docs/engineering/01-agents-superpowers.md'
    'docs/engineering/06-installers.md'
    'docs/extensions/03-visualizations-integrations.md'
    'docs/quality/04-integration-codecs-dsp.md'
    'docs/streaming/04-api-semantics.md'
)) { $mapaCompleto[$f] = "_arquivo/repo/$f" }

# docs/audio/* e docs/tools/* foram para reference/, nao para _arquivo/.
# Este mapa tem de vir DEPOIS do de cima para prevalecer.
foreach ($k in $paraReference.Keys) { $mapaCompleto[$k] = $paraReference[$k] }

# Diretorios. O mapa de arquivos e por arquivo; 4 links do repo apontam para
# DIRETORIO (`tasks/items/`, `docs/architecture/adr/`). Deriva a entrada do
# diretorio a partir do arquivo: docs/architecture/adr/ADR-001-x.md -> adr/ADR-001-y.md
# implica docs/architecture/adr -> adr. So entra quando todos os filhos do
# diretorio vao para o MESMO diretorio novo (um deles divergindo torna o
# mapeamento ambiguo, e entao o diretorio fica como esta).
# Diretorio: voto de MAIORIA dos filhos. Unanimidade era a regra antiga e
# deixava `docs/quality` e `docs/engineering` sem destino so porque UM filho
# (o 04 de cada) foi para _arquivo/ -- citacao de diretorio deve seguir o
# diretorio que hoje tem a maioria, nao desaparecer por causa da excecao.
# Empate nao mapeia: ai a escolha seria arbitraria.
$filhos = @{}
foreach ($k in @($mapaCompleto.Keys)) {
    if (-not $k.EndsWith('.md')) { continue }   # so arquivo real, nao diretorio ja posto
    $v = $mapaCompleto[$k]
    if (-not $k.Contains('/')) { continue }
    $kd = $k.Substring(0, $k.LastIndexOf('/'))
    $vd = if ($v.Contains('/')) { $v.Substring(0, $v.LastIndexOf('/')) } else { '' }
    if (-not $filhos.ContainsKey($kd)) { $filhos[$kd] = @{} }
    if (-not $filhos[$kd].ContainsKey($vd)) { $filhos[$kd][$vd] = 0 }
    $filhos[$kd][$vd]++
}
$dirMapeados = 0
$dirEmpate = @()
foreach ($kd in @($filhos.Keys)) {
    $votos = $filhos[$kd]
    if ($votos.Count -eq 0) { continue }
    $ordenado = @($votos.GetEnumerator() | Sort-Object -Property Value -Descending)
    if ($ordenado.Count -gt 1 -and $ordenado[0].Value -eq $ordenado[1].Value) {
        $dirEmpate += $kd
        continue
    }
    $vd = $ordenado[0].Key
    if ($vd -eq '' -or $vd -eq $kd) { continue }
    if (-not $mapaCompleto.ContainsKey($kd)) { $mapaCompleto[$kd] = $vd; $dirMapeados++ }
}
Write-Feito "$dirMapeados diretorio(s) no mapa de links"
if ($dirEmpate.Count -gt 0) {
    Write-Aviso "diretorio(s) com voto empatado, deixados como estao: $($dirEmpate -join ', ')"
}

# evidencia: docs/evidence/x -> evidence/x
$evidTodos = git ls-tree -r --name-only HEAD -- docs/evidence 2>$null
foreach ($f in $evidTodos) { $mapaCompleto[$f] = ($f -replace '^docs/evidence/', 'evidence/') }

$novoCaminhoDe = {
    param([string]$Rel)
    if ($mapaCompleto.ContainsKey($Rel)) { return $mapaCompleto[$Rel] }
    return $Rel
}
$novo = & $novoCaminhoDe 'docs/INDEX.md'
Write-Feito "mapa de links: $($mapaCompleto.Count) origem(ns) (INDEX.md -> $novo)"

# Mapa reverso destino->origem, para saber de onde cada .md veio. Necessario
# porque o link relativo depende do diretorio do arquivo: quando os DOIS lados
# se movem, o link tem de ser recalculado mesmo que o alvo nao esteja no mapa.
$origemDe = @{}
foreach ($k in $mapaCompleto.Keys) {
    $v = $mapaCompleto[$k]
    if (-not $origemDe.ContainsKey($v)) { $origemDe[$v] = $k }
}

# O que existia na arvore ANTIGA vem de $arvoreAntiga, capturada do HEAD ANTES
# do Move-Item (git ls-tree -r HEAD e agnostico ao caminho). Ler do disco ou do
# indice agora nao serve: o git mv ja aconteceu e o caminho antigo sumiu; sem a
# lista, todo link seria classificado como "ja quebrado" e nenhum reescrito.
# Diretorios entram porque 4 links apontam para diretorio (tasks/items/, adr/).
Write-Feito "arvore antiga: $($arvoreAntiga.Count) caminho(s) no HEAD"

# Alvos que o proprio passo cria: um link para eles nao estava quebrado hoje.
$criadosAqui = @('libs/LEIA-ME.md', 'apps/LEIA-ME.md', 'reference/LEIA-ME.md')
foreach ($c in $criadosAqui) { [void]$arvoreAntiga.Add($c) }

$mdVivos = @(git ls-files '*.md' | Where-Object { $_ -notlike '_arquivo/*' })
$linkRe = '\[[^\]]*\]\((?<alvo>[^)\s]+)\)'
$reescritos = 0
$naoResolvidos = @()
$porArquivo = @{}

foreach ($novoRel in $mdVivos) {
    $path = Join-Path $Destino $novoRel
    if (-not (Test-Path $path)) { continue }
    $texto = Read-Utf8 $path
    $novoDir = if ($novoRel.Contains('/')) { $novoRel.Substring(0, $novoRel.LastIndexOf('/')) } else { '' }

    # caminho ANTIGO deste arquivo, via mapa reverso
    $relAntigo = if ($origemDe.ContainsKey($novoRel)) { $origemDe[$novoRel] } else { $novoRel }
    $dirAntigo = if ($relAntigo.Contains('/')) { $relAntigo.Substring(0, $relAntigo.LastIndexOf('/')) } else { '' }

    $out = $texto
    $alterou = $false
    $ms = [regex]::Matches($texto, $linkRe)
    # de tras para frente: substituir por indice nao desloca os anteriores
    for ($i = $ms.Count - 1; $i -ge 0; $i--) {
        $m = $ms[$i]
        $alvo = $m.Groups['alvo'].Value
        if ($alvo -match '^[a-zA-Z][a-zA-Z0-9+.-]*:' -or $alvo.StartsWith('#')) { continue }
        # o check-docs do xtask faz trim_matches(['<','>']); preserva o formato
        $entreAngulos = $alvo.StartsWith('<') -and $alvo.EndsWith('>')
        $nucleo = if ($entreAngulos) { $alvo.Substring(1, $alvo.Length - 2) } else { $alvo }
        $partes = $nucleo.Split('#', 2)
        $cam = $partes[0]
        $anc = if ($partes.Count -gt 1) { '#' + $partes[1] } else { '' }
        if (-not $cam) { continue }
        $alvo = $nucleo

        # 1. resolve na arvore ANTIGA (lista do HEAD), normalizando ./ e ../
        $absAntigo = if ($dirAntigo) { "$dirAntigo/$cam" } else { $cam }
        try {
            $norm = [System.IO.Path]::GetFullPath((Join-Path $Destino $absAntigo))
        } catch { continue }
        if (-not $norm.StartsWith($Destino)) { continue }
        $normRel = $norm.Substring($Destino.Length + 1).Replace('\', '/')
        # link para diretorio vem com barra final (`adr/`); a chave do mapa nao tem
        if ($normRel.EndsWith('/')) { $normRel = $normRel.TrimEnd('/') }

        # link ja quebrado ANTES do passo: nao e defeito deste passo, nao toca
        if (-not $arvoreAntiga.Contains($normRel)) { $naoResolvidos += "$novoRel -> $alvo"; continue }

        # 2. aplica o mapa
        $alvoNovo = & $novoCaminhoDe $normRel

        # 3. recalcula relativo ao diretorio NOVO do arquivo
        $baseDir = if ($novoDir) { Join-Path $Destino $novoDir } else { $Destino }
        $novoLink = [System.IO.Path]::GetRelativePath($baseDir, (Join-Path $Destino $alvoNovo)).Replace('\', '/')
        if ($novoLink -eq '.') { $novoLink = '' }

        # alvo que o mapa nao conhece: o link so muda se o PROPRIO arquivo mudou
        # de diretorio. E o caso de links para Cargo.toml, .github/, tests/.
        if ($alvoNovo -eq $normRel -and $novoDir -eq $dirAntigo) { continue }

        $novoTrecho = $m.Value.Replace($alvo, $novoLink + $anc)
        if ($novoTrecho -ne $m.Value) {
            $out = $out.Substring(0, $m.Index) + $novoTrecho + $out.Substring($m.Index + $m.Length)
            $alterou = $true
            $reescritos++
        }
    }

    if ($alterou) {
        $porArquivo[$novoRel] = $true
        if ($PSCmdlet.ShouldProcess($novoRel, 'reescrever links relativos')) {
            Write-Utf8 -Path $path -Text $out
        }
    }
}
Write-Feito "$reescritos link(s) reescrito(s) em $($porArquivo.Count) arquivo(s)"
if ($naoResolvidos.Count -gt 0) {
    Write-Aviso "$($naoResolvidos.Count) link(s) para alvo inexistente ANTES da mudanca (nao sao defeito deste passo):"
    $naoResolvidos | Select-Object -First 10 | ForEach-Object { Write-Host "      $_" }
}

# ------------------------------------------------------------------ 6c. caminhos fora dos .md
Write-Passo 'corrigindo caminhos docs/ fora dos .md'

# A reescrita acima cobre links markdown. Quatro lugares carregam caminho
# `docs/...` como STRING, e todos quebram no destino:
#   1. validate.rs   -- le INDEX, a matriz de requisitos, o QA e o design
#   2. release.rs    -- le a evidencia de release
#   3. backlog.json  -- validate.rs:213 exige root.join(spec).exists()
#   4. config/, assets/, .github/ -- caminho citado em texto/atributo
# Sem esta secao, `cargo xtask check-docs` falha no destino por arquivo ausente,
# e o CI Gate reprova no primeiro push.

$substituicoes = @(
    @{ Rel = 'xtask/src/validate.rs'; De = 'docs/INDEX.md';                     Para = 'INDEX.md' }
    @{ Rel = 'xtask/src/validate.rs'; De = 'docs/product/03-requirements.md';   Para = 'product/03-requisitos.md' }
    @{ Rel = 'xtask/src/validate.rs'; De = 'docs/quality/01-test-strategy.md';  Para = 'quality/01-estrategia-testes.md' }
    @{ Rel = 'xtask/src/validate.rs'; De = 'docs/design/01-visual-system.md';   Para = 'design/01-sistema-visual.md' }
    @{ Rel = 'xtask/src/release.rs';  De = 'docs/evidence/release/';            Para = 'evidence/release/' }
    @{ Rel = 'config/superpowers.lock.json'; De = '"docs/evidence/';            Para = '"evidence/' }
    @{ Rel = 'assets/layout-reference.html'; De = '../docs/design/04-brand-identity.md'; Para = '../design/04-marca.md' }
    @{ Rel = '.github/workflows/release.yml'; De = 'docs/delivery/02-release-devops.md'; Para = 'delivery/02-release-devops.md' }
    @{ Rel = '.github/ISSUE_TEMPLATE/docs_problem.yml'; De = 'docs/engineering/04-ci-cd.md'; Para = 'engineering/04-ci.md' }
)
$corrigidos = 0
foreach ($s in $substituicoes) {
    $p = Join-Path $Destino $s.Rel
    if (-not (Test-Path $p)) { Write-Aviso "ausente, pulando: $($s.Rel)"; continue }
    $t = Read-Utf8 $p
    if (-not $t.Contains($s.De)) { Write-Aviso "sem '$($s.De)': $($s.Rel)"; continue }
    $novoTexto = $t.Replace($s.De, $s.Para)
    if ($PSCmdlet.ShouldProcess($s.Rel, "$($s.De) -> $($s.Para)")) {
        Write-Utf8 -Path $p -Text $novoTexto -NoTrailingNewline
        $corrigidos++
    }
}
Write-Feito "$corrigidos caminho(s) corrigido(s) em codigo/config"

# backlog.json: cada TASK aponta sua spec. As 37 que apontam para docs/audio,
# docs/tools e para os 5 arquivados passam a apontar para onde o arquivo
# realmente esta (reference/... e _arquivo/repo/...). O passo 37 reescreve os
# itens e repoe spec de modulo; aqui so torna o caminho verdadeiro. Sem isto,
# validate.rs:213 (`root.join(spec).exists()`) reprova em 5 TASKs.
$backlogPath = Join-Path $Destino 'tasks/backlog.json'
if (Test-Path $backlogPath) {
    $textoBacklog = Read-Utf8 $backlogPath
    $specs = [regex]::Matches($textoBacklog, '"spec":\s*"(docs/[^"]+)"')
    $distintas = $specs | ForEach-Object { $_.Groups[1].Value } | Sort-Object -Unique
    $mapaSpecs = 0
    # nao reaproveitar o nome $paraReference: ele e o hashtable da secao 6, e um
    # contador de mesmo nome o sobrescreve. Funciona hoje por ordem, quebra amanha.
    $nRef = 0
    $nArq = 0
    foreach ($sp in $distintas) {
        $novoSp = & $novoCaminhoDe $sp
        if ($novoSp -eq $sp) { continue }
        if ($novoSp.StartsWith('reference/')) { $nRef++ }
        if ($novoSp.StartsWith('_arquivo/')) { $nArq++ }
        $textoBacklog = $textoBacklog.Replace("`"spec`": `"$sp`"", "`"spec`": `"$novoSp`"")
        $mapaSpecs++
    }
    if ($mapaSpecs -gt 0 -and $PSCmdlet.ShouldProcess('tasks/backlog.json', "remapear $mapaSpecs spec(s)")) {
        Write-Utf8 -Path $backlogPath -Text $textoBacklog -NoTrailingNewline
        Write-Feito "backlog.json: $mapaSpecs spec(s) remapeadas ($nRef -> reference/, $nArq -> _arquivo/repo/)"
    }
}

# INDEX.md: a coluna final `— docs/caminho.md` e texto, nao link -- o bloco 6b
# nao a alcanca. Reescreve pelo mapa para que nenhum caminho docs/ sobreviva.
# Tambem acrescenta as 3 entradas que faltam (libs, apps, reference).
$indexPath = Join-Path $Destino 'INDEX.md'
if (Test-Path $indexPath) {
    $idx = Read-Utf8 $indexPath
    $colunas = [regex]::Matches($idx, '(?m)^(- \[[^\]]*\]\([^)]+\) — )(docs/[^\s;]+)')
    $nCol = 0
    for ($i = $colunas.Count - 1; $i -ge 0; $i--) {
        $m = $colunas[$i]
        $antigo = $m.Groups[2].Value
        $novoIdx = & $novoCaminhoDe $antigo
        if ($novoIdx -eq $antigo) { continue }
        $trecho = $m.Groups[1].Value + $novoIdx
        $idx = $idx.Substring(0, $m.Index) + $trecho + $idx.Substring($m.Index + $m.Groups[0].Length)
        $nCol++
    }

    $faltam = @(
        @{ Rel = 'libs/LEIA-ME.md';      Titulo = 'libs — submodulos de audio (vazio ate o passo 35)' }
        @{ Rel = 'apps/LEIA-ME.md';      Titulo = 'apps — submodulos de aplicacao (vazio ate o passo 35)' }
        @{ Rel = 'reference/LEIA-ME.md'; Titulo = 'reference — referencia read-only, fora do build' }
    )
    $adicionadas = 0
    foreach ($f in $faltam) {
        if (-not (Test-Path (Join-Path $Destino $f.Rel))) { continue }
        if ($idx.Contains("($($f.Rel))")) { continue }
        $linha = "- [$($f.Titulo)]($($f.Rel)) — $($f.Rel)`n"
        if (-not $idx.EndsWith("`n")) { $idx += "`n" }
        $idx += $linha
        $adicionadas++
    }

    if ($PSCmdlet.ShouldProcess('INDEX.md', "reescrever $nCol coluna(s) e acrescentar $adicionadas entrada(s)")) {
        Write-Utf8 -Path $indexPath -Text $idx -NoTrailingNewline
        Write-Feito "INDEX.md: $nCol coluna(s) remapeada(s), $adicionadas entrada(s) nova(s)"
    }
}

# ------------------------------------------------------------------ 6d. docs/ em prosa
Write-Passo 'reescrevendo caminhos docs/ citados em prosa'

# O bloco 6b reescreve LINK markdown `[x](caminho)`. Nao alcanca caminho citado
# em TEXTO -- `docs/audio/03-dsp.md` dentro de crase, de uma linha de tabela ou
# de um paragrafo. Medido no ensaio: 55 citacoes em 13 arquivos vivos ficavam
# apontando para um diretorio que deixa de existir, e a spec §7 exige 0.
#
# Reescreve pelo mesmo $mapaCompleto, com tres excecoes deliberadas:
#
#   1. CHANGELOG.md -- registro do que foi feito na data. Reescrever o nome que
#      o arquivo tinha na epoca falsifica o historico. Fica na grafia de entao.
#   2. evidence/ -- dumps de CI e relatorios de PR: registro, nao spec.
#   3. Diretorios e caminhos que NUNCA existiram neste repo (`docs/contracts`,
#      `docs/schema`, `docs/tooling`, `docs/superpowers/plans`, `docs/config`).
#      O mapa nao os conhece e inventar destino seria pior que deixar. Ficam
#      listados na saida para o passo 37/32 decidir, e nao viram link quebrado
#      porque nao sao link.
$semProsa = @('CHANGELOG.md')

# por prefixo de diretorio novo, resolvido pelo mapa (documenta a intencao)
$mapaProsa = @{}
foreach ($k in $mapaCompleto.Keys) { $mapaProsa[$k] = $mapaCompleto[$k] }
# docs/evidence/... nao esta no mapa de arquivos (so no de links); entra aqui,
# com os diretorios derivados dos arquivos -- a prosa cita `docs/evidence/PR/`
# com barra final, e sem a entrada de diretorio ela ficaria sem destino.
foreach ($f in (git ls-files 'evidence/*')) {
    $mapaProsa["docs/$f"] = $f
    $dd = $f
    while ($dd.Contains('/')) {
        $dd = $dd.Substring(0, $dd.LastIndexOf('/'))
        $mapaProsa["docs/$dd"] = $dd
    }
}
# A regra de "o que reescrever" nao e lista manual, e a arvore do HEAD: se o
# caminho citado existia no repo antes do passo, ele TEM destino e cita-lo no
# nome antigo e link morto em prosa. Se nunca existiu -- `docs/contracts`,
# `docs/schema`, `docs/tooling` -- nao ha destino a inventar e o texto fica.
# Uma lista manual acerta hoje e envelhece amanha; a arvore nao.
#
# O que o passo 30 arquivou em _arquivo/repo/ tambem tem destino: ARQUIVO
# citado em prosa segue a mesma convencao que os LINKS ja seguem desde o passo
# 30 -- ADR-014:5, TASK-E15-04, TASK-E11-01 e TASK-E15-01/02/03 apontam para
# ../../_arquivo/repo/... e a prosa ao lado deixou o caminho velho. Sem isso,
# `ADR-010` e `08-folder-structure` (que existe e e citado como "Spec" em
# REQ-E19-05) apontam para um diretorio que deixa de existir.
#
# Diretorio NAO entra: `docs/superpowers/plans` aparece em AGENTS.md,
# CLAUDE.md e docs/engineering/01 como o lugar ONDE SE ESCREVEM planos novos;
# mandar a instrucao para o arquivo morto seria pior que a grafia antiga. O
# _arquivo/repo/ que existe e de planos antigos, nao do diretorio vivo.
# `git ls-tree HEAD:_arquivo/repo` ja devolve caminho COM `docs/`, porque o
# passo 30 arquivou `repo/docs/x` em `_arquivo/repo/docs/x`. Prefixar de novo
# daria a chave `docs/docs/x`, que nunca casa com o texto.
foreach ($f in (git ls-tree -r --name-only HEAD:_arquivo/repo 2>$null)) {
    $mapaProsa[$f] = "_arquivo/repo/$f"
}

# docs/evidence/ -> evidence/ e regra de PREFIXO, nao de arquivo: a subarvore
# inteira mudou de lugar. Sem isso, `docs/evidence/release/<versao>.json` --
# caminho de uma evidencia que a TASK vai PRODUZIR, citado em
# engineering/05-testes.md:15 -- ficaria apontando para o passado. O mapa de
# arquivos nao resolve: `evidence/release/` nao existe, e criar a pasta aqui
# seria inventar estrutura que nenhum passo pediu.
$prefixosProsa = [ordered]@{ 'docs/evidence/' = 'evidence/' }

$prosaRe = '(?<![\w/.\-])(docs/[A-Za-z0-9_./\-]*[A-Za-z0-9_\-])'
$ativosProsa = 0
$naoMapeados = [System.Collections.Generic.HashSet[string]]::new()
$historicos = [System.Collections.Generic.HashSet[string]]::new()
foreach ($rel in @(git ls-files '*.md' | Where-Object { $_ -notlike '_arquivo/*' -and $_ -notlike 'evidence/*' -and $semProsa -notcontains $_ })) {
    $p = Join-Path $Destino $rel
    if (-not (Test-Path $p)) { continue }
    $t = Read-Utf8 $p
    if (-not $t.Contains('docs/')) { continue }
    $ms = [regex]::Matches($t, $prosaRe)
    if ($ms.Count -eq 0) { continue }
    $out = $t; $mudou = $false
    for ($i = $ms.Count - 1; $i -ge 0; $i--) {
        $alvo = $ms[$i].Groups[1].Value
        # `/` final e parte do alvo (link para diretorio); pontuacao nao e
        $sufl = ''
        $nucleo = $alvo
        if (-not $alvo.EndsWith('/')) {
            $nucleo = $alvo.TrimEnd('.', ',', ';', ':', '`')
            $sufl = $alvo.Substring($nucleo.Length)
        }
        $novoP = $null
        if ($mapaProsa.ContainsKey($nucleo)) {
            $novoP = $mapaProsa[$nucleo]
        } else {
            # prefixo antes de desistir: docs/evidence/x -> evidence/x vale
            # para caminho que ainda nao existe (evidencia futura)
            foreach ($pfx in $prefixosProsa.Keys) {
                if ($nucleo.StartsWith($pfx)) {
                    $novoP = $prefixosProsa[$pfx] + $nucleo.Substring($pfx.Length)
                    break
                }
            }
        }
        if (-not $novoP) {
            if ($arvoreAntiga.Contains($nucleo.TrimEnd('/'))) {
                # existia antes e o mapa nao cobre: lacuna do mapa, nao historico
                [void]$naoMapeados.Add($nucleo)
            } else {
                [void]$historicos.Add($nucleo)
            }
            continue
        }
        if ($novoP -eq $nucleo) { continue }
        $novoTrecho = $novoP + $sufl
        $out = $out.Substring(0, $ms[$i].Index) + $novoTrecho + $out.Substring($ms[$i].Index + $alvo.Length)
        $mudou = $true
        $ativosProsa++
    }
    if ($mudou -and $PSCmdlet.ShouldProcess($rel, 'reescrever docs/ citado em prosa')) {
        Write-Utf8 -Path $p -Text $out -NoTrailingNewline
    }
}
Write-Feito "$ativosProsa caminho(s) docs/ reescrito(s) em prosa"
if ($naoMapeados.Count -gt 0) {
    Write-Aviso "$($naoMapeados.Count) caminho(s) que existiam e ficaram fora do mapa (corrigir aqui, nao no passo 32):"
    @($naoMapeados) | Sort-Object | Select-Object -First 15 | ForEach-Object { Write-Host "      $_" }
}
if ($historicos.Count -gt 0) {
    Write-Feito "$($historicos.Count) caminho(s) docs/ citados que nunca existiram neste repo (preservados):"
    @($historicos) | Sort-Object | Select-Object -First 8 | ForEach-Object { Write-Host "      $_" }
}

# Varredura final da spec §7: nenhum `docs/` sobrevive fora de _arquivo/ e do
# CHANGELOG (registro historico, reescrito seria falso).
#
# NAO usar `:!caminho` como pathspec: o git le `_` como magic e aborta com
# "Unimplemented pathspec magic '_'". Com `2>$null` o erro ia embora e a
# varredura "passava" com 25 ocorrencias no disco -- verificacao que mente.
# Aqui o filtro de caminho e feito em PowerShell, sobre a lista que o proprio
# git devolve, e a funcao DEVOLVE FALHA quando o git falha.
#
# Definir ANTES do primeiro uso: em PowerShell nao ha hoisting de funcao dentro
# do mesmo bloco -- chamar antes da definicao falha com "not recognized".
function Find-DocsRefs {
    param([string]$Padrao, [string[]]$Excluir, [switch]$SoArquivos)
    $gp = @('grep', '--no-color')
    if ($SoArquivos) { $gp += '-l' } else { $gp += '-n' }
    $gp += @('-E', $Padrao, '--', '*.md')
    $saida = & git @gp 2>&1
    $codigo = $LASTEXITCODE
    # git grep devolve 1 quando nao ha nenhuma ocorrencia: nao e erro
    if ($codigo -notin @(0, 1)) {
        throw "git grep falhou ($codigo): $($saida | Select-Object -First 3)"
    }
    $linhas = @($saida | Where-Object { $_ -is [string] })
    if ($Excluir) {
        $linhas = @($linhas | Where-Object {
            $caminho = if ($SoArquivos) { $_ } else { ($_ -split ':', 2)[0] }
            $manter = $true
            foreach ($e in $Excluir) { if ($caminho -like $e) { $manter = $false } }
            $manter
        })
    }
    return , $linhas
}

# ------------------------------------------------------------------ 7. verificacao
Write-Passo 'verificacao'

# Mede ARQUIVO sob docs/ (o diretorio tem de sumir), nao citacao em prosa --
# citacao e o bloco seguinte, que lista as 7 linhas remanescentes. Dizer so
# "nenhum caminho docs/ restante" ao lado de 7 linhas que citam docs/ e o tipo
# de resumo que faz a revisao passar por cima.
$sobraDocs = git ls-files 'docs/*'
if ($sobraDocs) { Write-Aviso "sobrou em docs/: $($sobraDocs.Count)"; $sobraDocs | ForEach-Object { Write-Host "      $_" } }
else { Write-Feito 'nenhum ARQUIVO sob docs/ (diretorio removido; citacoes em prosa listadas abaixo)' }

# Nenhum .md vivo pode citar docs/audio ou docs/tools como CAMINHO VALIDO.
# As 13 specs foram para reference/audio e reference/tools. Citacao em prosa
# historica (CHANGELOG, evidence/PR/) e registro do que foi feito na epoca e
# fica como esta -- reescrever historico seria falso. O que nao pode existir e
# link markdown apontando para docs/audio|tools, porque o alvo nao existe mais.
$linksMortos = Find-DocsRefs -Padrao '\]\(\.{0,2}/?(docs/(audio|tools)/[^)]+)\)' -Excluir @('_arquivo/*')
if ($linksMortos.Count -gt 0) {
    Write-Aviso "$($linksMortos.Count) link(s) vivo(s) para docs/audio ou docs/tools (o alvo nao existe mais):"
    $linksMortos | Select-Object -First 10 | ForEach-Object { Write-Host "      $_" }
} else { Write-Feito 'nenhum link vivo para docs/audio ou docs/tools' }

$citacoes = Find-DocsRefs -Padrao 'docs/(audio|tools)/' -Excluir @('_arquivo/*') -SoArquivos
if ($citacoes.Count -gt 0) {
    Write-Feito "$($citacoes.Count) arquivo(s) ainda citam docs/audio|tools em prosa:"
    $citacoes | Select-Object -First 8 | ForEach-Object { Write-Host "      $_" }
}

$sobrouDocs = Find-DocsRefs -Padrao '(^|[^A-Za-z0-9_/.-])docs/' -Excluir @('_arquivo/*', 'CHANGELOG.md', 'evidence/*')
if ($sobrouDocs.Count -gt 0) {
    Write-Aviso "$($sobrouDocs.Count) linha(s) ainda citam docs/ fora de _arquivo/, CHANGELOG e evidence:"
    $sobrouDocs | Select-Object -First 15 | ForEach-Object { Write-Host "      $_" }
} else { Write-Feito 'nenhum caminho docs/ em .md vivo (fora de CHANGELOG e evidence)' }

Push-Location $Destino
try {
    $meta = cargo metadata --no-deps --format-version 1 2>&1
    if ($LASTEXITCODE -eq 0) {
        $so = ($meta | Out-String)
        if ($so -match '"xtask"' -and $so -notmatch '"vynamp-desktop"') { Write-Feito 'cargo metadata ok; xtask e o unico membro' }
        else { Write-Aviso 'cargo metadata ok, mas o conjunto de membros nao e so xtask' }
    }
    else { Write-Aviso "cargo metadata falhou: $($meta | Out-String)" }

    # A verificacao de verdade: o mesmo gate que o CI roda (ADR-014).
    # check-docs exige titulo, entrada no INDEX e que todo link relativo exista.
    # O subcomando e `check-docs`; `cargo xtask validate` NAO existe.
    $docs = cargo xtask check-docs 2>&1
    if ($LASTEXITCODE -eq 0) { Write-Feito 'cargo xtask check-docs: 0 erro' }
    else {
        Write-Aviso 'cargo xtask check-docs REPROVOU -- o passo nao esta pronto:'
        ($docs | Out-String) -split "`n" | Where-Object { $_ -match '\S' } |
            Select-Object -First 25 | ForEach-Object { Write-Host "      $_" }
    }
    $deliv = cargo xtask check-delivery 2>&1
    if ($LASTEXITCODE -eq 0) { Write-Feito 'cargo xtask check-delivery: 0 erro' }
    else {
        Write-Aviso 'cargo xtask check-delivery REPROVOU:'
        ($deliv | Out-String) -split "`n" | Where-Object { $_ -match '\S' } |
            Select-Object -First 15 | ForEach-Object { Write-Host "      $_" }
    }
} finally { Pop-Location }

Write-Host ''
Write-Host '    Conferencia a mao (nao commitado, nao enviado):' -ForegroundColor Cyan
Write-Host '      git -C D:\Projects\vynamp status --short | head -30'
Write-Host '      git -C D:\Projects\vynamp log --oneline -3'
Write-Host '      ls D:\Projects\vynamp'
Write-Host '      cargo xtask quality-gate   # so depois de check-docs verde'
Write-Host ''
Write-Host '    Nada foi commitado nem enviado. PARE AQUI e confira a arvore.' -ForegroundColor Yellow
Write-Host ''

} finally {
    Pop-Location
}
