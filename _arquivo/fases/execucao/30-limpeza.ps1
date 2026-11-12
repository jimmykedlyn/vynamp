<#
    30-limpeza.ps1 — passo 30 da FASE 3

    Aplica M4 (16 VUs) e M6 (remove Woodpecker) e cria _arquivo/.
    NADA E APAGADO: arquivo de projeto vai para _arquivo/, com motivo no INDEX.md.
    Sem push, sem merge, sem tag, sem gh repo edit. Nenhum token e impresso.

    Uso:  pwsh -NoProfile -File .\30-limpeza.ps1
          .\30-limpeza.ps1 -WhatIf     # so mostra o que faria
#>
[CmdletBinding(SupportsShouldProcess = $true)]
param(
    [switch]$SkipM8Gate
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$RepoRoot   = 'D:\Projects\vynamp-media-player\repo'
$ExternoRoot = 'D:\Projects\vynamp-media-player'
$ArquivoRoot = Join-Path $RepoRoot '_arquivo'
$IndexPath   = Join-Path $ArquivoRoot 'INDEX.md'

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

if (-not (Test-Path $RepoRoot)) { throw "Repo nao encontrado: $RepoRoot" }
Push-Location $RepoRoot
try {

# ------------------------------------------------------------------ 0. pre-checagem
Write-Passo 'pre-checagem'

$branch = (git rev-parse --abbrev-ref HEAD).Trim()
if ($branch -ne 'develop') { throw "Branch atual '$branch'; esperado 'develop'." }
Write-Feito "branch: $branch"

$dirty = git status --short
if ($dirty) {
    Write-Aviso 'arvore nao esta limpa:'
    $dirty | ForEach-Object { Write-Host "      $_" }
    throw 'Commit ou stash antes de rodar o passo 30.'
}
Write-Feito 'arvore limpa'

$head = (git rev-parse --short HEAD).Trim()
Write-Feito "HEAD: $head"

# ------------------------------------------------------------------ 1. _arquivo/
Write-Passo 'criando _arquivo/'

if (-not (Test-Path $ArquivoRoot)) {
    if ($PSCmdlet.ShouldProcess($ArquivoRoot, 'criar diretorio')) {
        New-Item -ItemType Directory -Path $ArquivoRoot -Force | Out-Null
    }
}
# Envolvido em ShouldProcess: com -WhatIf o diretorio nao existe e a escrita falharia.
if (-not (Test-Path $IndexPath) -and $PSCmdlet.ShouldProcess($IndexPath, 'criar indice')) {
    $cabecalho = @(
        '# _arquivo — indice de arquivamento',
        '',
        'Todo arquivo movido para ca tem uma linha. A exclusao definitiva so acontece',
        'no passo 38, sob confirmacao explicita do mantenedor.',
        '',
        '| Data | Origem | Destino | Motivo | Passo |',
        '|---|---|---|---|---|'
    ) -join "`n"
    Write-Utf8 -Path $IndexPath -Text $cabecalho
}
Write-Feito $IndexPath

function Add-Indice {
    # Idempotente pela chave (Origem + Destino + Passo): reexecutar nao duplica a linha.
    param([string]$Origem, [string]$Destino, [string]$Motivo, [string]$Passo = '30')
    $Motivo = $Motivo -replace '\|', '/'
    $data = Get-Date -Format 'yyyy-MM-dd'
    if (Test-Path $IndexPath) {
        $chave = "| {0} | `{1}` | `{2}` |" -f $data, $Origem, $Destino
        $existe = (Read-Utf8 -Path $IndexPath) -split "`r?`n" | Where-Object { $_.StartsWith($chave) -and $_.EndsWith("| $Passo |") }
        if ($existe) { return }
    }
    $linha = '| {0} | `{1}` | `{2}` | {3} | {4} |' -f $data, $Origem, $Destino, $Motivo, $Passo
    # Sob -WhatIf o INDEX.md nao existe (a criacao dele tambem respeita ShouldProcess),
    # entao escrever aqui estouraria com "Could not find a part of the path".
    if ($PSCmdlet.ShouldProcess($IndexPath, "registrar indice: $Origem")) {
        Append-Utf8 -Path $IndexPath -Text $linha
    }
}

function Move-ParaArquivo {
    <# Move um arquivo versionado para _arquivo/repo/<caminho original> usando git mv.
       Com -Desde (passo 38) o arquivo ja esta em _arquivo/ e so o indice e escrito. #>
    param([string]$RelPath, [string]$Motivo, [switch]$DesdeArquivo, [string]$Passo = '30')

    $src = Join-Path $RepoRoot $RelPath
    if (-not (Test-Path $src)) { Write-Aviso "inexistente, pulando: $RelPath"; return }

    $dstRel = "_arquivo/repo/$RelPath"
    $dst = Join-Path $RepoRoot $dstRel
    $dstDir = Split-Path $dst -Parent

    if ($DesdeArquivo) {
        # passo 38: o deslocamento aconteceu, so o INDEX.md estava vazio. Registra.
        Write-Feito "ja arquivado; indice registrado: $RelPath"
        Add-Indice -Origem $RelPath -Destino $dstRel -Motivo $Motivo -Passo $Passo
        return
    }

    if (Test-Path $dst) { Write-Aviso "ja arquivado, pulando: $RelPath"; return }

    if ($PSCmdlet.ShouldProcess($RelPath, "mover para $dstRel")) {
        if (-not (Test-Path $dstDir)) { New-Item -ItemType Directory -Path $dstDir -Force | Out-Null }
        git mv -- $RelPath $dstRel
        if ($LASTEXITCODE -ne 0) { throw "git mv falhou em $RelPath" }
        Add-Indice -Origem $RelPath -Destino $dstRel -Motivo $Motivo -Passo $Passo
        Write-Feito "arquivado: $RelPath"
    }
}

# ------------------------------------------------------------------ 2. arquivamentos
Write-Passo 'arquivando (M6: Woodpecker, historico, placeholders)'

$arquivar = @(
    @{ p = 'docs/architecture/adr/ADR-010-woodpecker-only-ci.md'; m = 'M6: Woodpecker removido por inteiro' },
    @{ p = 'docs/engineering/08-folder-structure.md';              m = 'descreve raiz de ZIP que nao existe mais' },
    @{ p = 'docs/product/08-completeness-audit.md';               m = 'auditoria historica 0.6, superada' },
    @{ p = 'docs/research/03-completeness-sources.md';            m = 'fontes da auditoria 0.6, superadas' },
    @{ p = 'START-HERE.md';                                       m = 'descreve bootstrap de ZIP e INICIAR.ps1 inexistentes' },
    @{ p = 'ARTIFACTS.json';                                      m = 'manifesto de bundle de outro produto' },
    @{ p = 'VALIDATION.json';                                     m = 'saida gerada por xtask validate; passa a ser gitignored' }
)

foreach ($a in $arquivar) { Move-ParaArquivo -RelPath $a.p -Motivo $a.m }

# Registro das origens de "fundir" e "absorver": o arquivo continua vivo neste
# momento (31 le o conteudo dele), por isso nao entra em _arquivo/ ainda. So o
# indice passa a dizer de onde veio o conteudo. Destino = o alvo.
$registrar = @(
    @{ o = 'docs/audio/05-platform-backends.md';                     d = 'libs/output (spec, passo 35)'; m = 'fundida em audio/04-outputs-hires; 9 linhas identicas' },
    @{ o = 'docs/quality/04-integration-codecs-dsp.md';              d = 'quality/01-estrategia-testes.md'; m = 'fundida; QA-022..033 byte-identicas para quality/01' },
    @{ o = 'docs/streaming/04-api-semantics.md';                     d = 'architecture/04-contratos.md'; m = 'fundida na secao Provider de architecture/04' },
    @{ o = 'docs/engineering/06-installers.md';                      d = 'delivery/04-gates.md'; m = 'fundida; mesmos instaladores' },
    @{ o = 'docs/extensions/03-visualizations-integrations.md';      d = 'extensions/05-visualizador.md'; m = 'fundida; era subconjunto' },
    @{ o = 'docs/engineering/01-agents-superpowers.md';              d = 'AGENTS.md secao 5'; m = 'absorvida por M5' },
    @{ o = 'docs/audio/01-engine.md';                                d = 'libs/engine (spec, passo 35)'; m = 'absorvida por M3' },
    @{ o = 'docs/audio/02-formats.md';                               d = 'libs/decode + libs/encode'; m = 'absorvida por M3' },
    @{ o = 'docs/audio/03-dsp.md';                                   d = 'libs/engine'; m = 'absorvida por M3; resampler vai a dsp (M9)' },
    @{ o = 'docs/audio/04-outputs-hires.md';                         d = 'libs/output'; m = 'absorvida por M3' },
    @{ o = 'docs/audio/06-codec-capability-registry.md';             d = 'libs/decode + libs/encode'; m = 'absorvida por M3' },
    @{ o = 'docs/audio/07-dsp-signal-contract.md';                   d = 'libs/engine'; m = 'absorvida por M3' },
    @{ o = 'docs/audio/08-equalizers.md';                            d = 'libs/engine'; m = 'absorvida por M3' },
    @{ o = 'docs/audio/09-dsp-catalog.md';                           d = 'libs/engine'; m = 'absorvida por M3' },
    @{ o = 'docs/audio/10-vitrola.md';                               d = 'libs/engine'; m = 'M10: vinil/tube interno; sem repo vitrola' },
    @{ o = 'docs/tools/01-vynamp-restore.md';                        d = 'apps/restore'; m = 'absorvida por M3' },
    @{ o = 'docs/tools/02-vynamp-converter.md';                      d = 'apps/converter'; m = 'absorvida por M3' },
    @{ o = 'repo/AGENTS.md';                                         d = 'AGENTS.md do umbrella'; m = 'M5: absorvido no passo 32' },
    @{ o = 'repo/CLAUDE.md';                                         d = 'CLAUDE.md do umbrella'; m = 'M5: absorvido no passo 32' },
    @{ o = 'AGENTS.md (raiz externa)';                               d = 'AGENTS.md do umbrella'; m = 'M5: absorvido no passo 32' },
    @{ o = 'LEIA-ME.md (raiz externa)';                              d = 'README.md do umbrella'; m = 'absorvido no passo 31' }
)
foreach ($r in $registrar) { Add-Indice -Origem $r.o -Destino $r.d -Motivo $r.m }

# raiz externa (00-mapa-destino.csv, passo 30). AUDIOSR e SOURCE_SEPARATION
# ficam de fora de proposito: o gate M8 logo abaixo decide por eles.
$arquivarRaiz = @(
    @{ p = 'CLAUDE-SPEC-LETRAS-SINCRONIZADAS.md'; d = 'CLAUDE-SPEC-LETRAS-SINCRONIZADAS.md'; m = 'integrado em library/02 e D-LYR' },
    @{ p = 'Prompt_Claude_Documentacao_DSP_Equalizadores_Obrigatorios_v1.4.md'; d = 'Prompt_Claude_DSP_v1.4.md'; m = 'ja integrado em D-DSP-11..18, design/06, quality/04' },
    @{ p = 'ROADMAP-rodadas.md';                  d = 'ROADMAP-rodadas.md'; m = 'conceito morto; substituido por tasks/backlog.json' },
    @{ p = 'SKIN-REQUIREMENTS-CODEX.md';          d = 'SKIN-REQUIREMENTS-CODEX.md'; m = 'descartado por D-MVP-01; 87% subconjunto do GPT' },
    @{ p = 'SKIN-REQUIREMENTS-GPT-MVP-OBRIGATORIO-v1.2.md'; d = 'SKIN-REQUIREMENTS-GPT-MVP-OBRIGATORIO-v1.2.md'; m = 'descartado por D-MVP-01' },
    @{ p = 'TAREFA-SPECS-DSP-LETRAS.md';          d = 'TAREFA-SPECS-DSP-LETRAS.md'; m = 'ordem de servico concluida' },
    @{ p = 'TAREFA-SPECS-LIBS-MVP.md';            d = 'TAREFA-SPECS-LIBS-MVP.md'; m = 'ordem de servico concluida' }
)

# evidencia de Woodpecker
$wpEvid = git ls-files 'docs/evidence/PR/task-e15-04/*' 'docs/evidence/PR/chore-woodpecker-3-20-1/*'
foreach ($f in $wpEvid) { Move-ParaArquivo -RelPath $f -Motivo 'M6: evidencia de CI morto (Woodpecker)' }

# planos executados
$planos = git ls-files 'docs/superpowers/plans/*'
foreach ($f in $planos) { Move-ParaArquivo -RelPath $f -Motivo 'plano de execucao ja concluido' }

# placeholders substituidos por submodulos
$placeholders = git ls-files 'crates/*' 'apps/*' 'bootstrap/*'
foreach ($f in $placeholders) {
    Move-ParaArquivo -RelPath $f -Motivo 'placeholder; submodulo o substitui'
}

# assets de referencia de conceito
$refAssets = git ls-files 'assets/branding/reference/*'
foreach ($f in $refAssets) { Move-ParaArquivo -RelPath $f -Motivo 'conceito A/B/C/D, nao asset de produto' }

# AGENTS/CLAUDE do repo, absorvidos por M5 no passo 32
# (movidos so no passo 32, depois de o novo contrato existir)

# arquivos da raiz externa (fora do git): Move-Item, nunca git mv. Nada e apagado:
# vao para _arquivo/raiz/ e o passo 38 so remove o que o mantenedor autorizar.
foreach ($a in $arquivarRaiz) {
    $src = Join-Path $ExternoRoot $a.p
    if (-not (Test-Path $src)) { Write-Aviso "inexistente, pulando: raiz/$($a.p)"; continue }
    $dstRel = "_arquivo/raiz/$($a.d)"
    $dst = Join-Path $ArquivoRoot "raiz\$($a.d)"
    if (Test-Path $dst) { Write-Aviso "ja arquivado, pulando: raiz/$($a.p)"; continue }
    if ($PSCmdlet.ShouldProcess($src, "mover para $dstRel")) {
        $dstDir = Split-Path $dst -Parent
        if (-not (Test-Path $dstDir)) { New-Item -ItemType Directory -Path $dstDir -Force | Out-Null }
        Move-Item -Path $src -Destination $dst -Force
        Add-Indice -Origem "raiz/$($a.p)" -Destino $dstRel -Motivo $a.m
        Write-Feito "arquivado: raiz/$($a.p)"
    }
}
# os dois da M8 so saem com ADR-015 + tools/03 em develop: tem gate proprio abaixo

# ------------------------------------------------------------------ 3. M4: 16 VUs
Write-Passo 'M4: 12 -> 16 VUs'

# M4 nao e feito por regex ampla. Medido nos arquivos, o texto separa tres casos
# que uma regex nao distingue:
#   (a) contagem VIGENTE do contrato -- MUDA para 16;
#   (b) registro historico (protótipo rev.15, "144 combinações") -- FICA, porque
#       descreve o que foi medido naquela revisao;
#   (c) citacao literal de out/themes/themes.json dentro de D-MVP-04
#       ("encaixe universal para os 12 VUs aprovados") -- FICA, senao deixa de
#       ser citacao.
# Substituicoes globais testadas e descartadas: 144 -> 192 tambem pegava
# "144 kHz" (docs/audio/08-equalizers.md) e a citacao do themes.json;
# 12 VUs -> 16 VUs tambem pegava "os 12 sao anteriores aos quatro medidores".
# Por isso cada ponto abaixo e um par (de, para) literal e unico: se o texto
# mudou e o `de` nao casa exatamente uma vez, o passo falha em vez de editar
# pela metade.
$m4 = @(
    # design/01-visual-system.md
    @{ rel = 'docs/design/01-visual-system.md'
       de  = 'Três acabamentos de apresentação dos 12 VUs estéreo'
       para = 'Três acabamentos de apresentação dos 16 VUs estéreo' },

    # design/02-layouts.md
    @{ rel = 'docs/design/02-layouts.md'
       de  = '(os 12 VUs estéreo e seus'
       para = '(os 16 VUs estéreo e seus' },

    # design/05-dsp-panel.md -- a divergencia deixa de existir: o contrato passa a 16
    @{ rel = 'docs/design/05-dsp-panel.md'
       de  = '**Contagem de modelos.** O contrato VU-INT abaixo fala em **12 VU meters aprovados**, número herdado da rev.15. No pin atual (rev.38) o pacote tem **16 medidores e os 16 são selecionáveis** ([§3 da referência](06-mvp-reference.md#3-pacote-de-referência)). A divergência está registrada em [D-MVP-04](../product/05-decisions.md#decisão-o-mvp-é-a-fonte-única-da-ui-2026-11-05); a matriz de aceite VU-INT-06 não é reescrita aqui.'
       para = '**Contagem de modelos.** O contrato VU-INT abaixo fala em **16 VU meters aprovados**, número do pin atual ([§3 da referência](06-mvp-reference.md#3-pacote-de-referência)): o pacote tem **16 medidores e os 16 são selecionáveis**. A contagem herdada da rev.15 era 12 e foi corrigida por [D-MVP-04](../product/05-decisions.md#decisão-o-mvp-é-a-fonte-única-da-ui-2026-11-05) em 2026-11-06.' },
    @{ rel = 'docs/design/05-dsp-panel.md'
       de  = 'Contrato de apresentação dos 12 VU meters estéreo aprovados, vindo da referência externa rev. 15.'
       para = 'Contrato de apresentação dos 16 VU meters estéreo aprovados, vindo da referência externa rev. 38.' },
    @{ rel = 'docs/design/05-dsp-panel.md'
       de  = 'preservar byte a byte os 12 SVGs aprovados'
       para = 'preservar byte a byte os 16 SVGs aprovados' },
    @{ rel = 'docs/design/05-dsp-panel.md'
       de  = 'botão direito para escolher entre os 12 modelos'
       para = 'botão direito para escolher entre os 16 modelos' },
    @{ rel = 'docs/design/05-dsp-panel.md'
       de  = 'validar as 144 combinações de 12 skins × 12 VUs'
       para = 'validar as 192 combinações de 12 skins × 16 VUs' },

    # quality/05-dsp-test-matrix.md -- 12 skins continuam 12; VUs passam a 16
    @{ rel = 'docs/quality/05-dsp-test-matrix.md'
       de  = 'waveform, 12 skins/VUs e abertura/fechamento do DSP'
       para = 'waveform, 12 skins e 16 VUs e abertura/fechamento do DSP' },
    @{ rel = 'docs/quality/05-dsp-test-matrix.md'
       de  = 'Comparar byte a byte os 12 SVGs aprovados'
       para = 'Comparar byte a byte os 16 SVGs aprovados' },
    @{ rel = 'docs/quality/05-dsp-test-matrix.md'
       de  = 'botão direito para os 12 modelos'
       para = 'botão direito para os 16 modelos' },
    @{ rel = 'docs/quality/05-dsp-test-matrix.md'
       de  = 'Percorrer 12 skins × 12 VUs (144 combinações)'
       para = 'Percorrer 12 skins × 16 VUs (192 combinações)' },
    @{ rel = 'docs/quality/05-dsp-test-matrix.md'
       de  = '`meter-finish-validation` (144)'
       para = '`meter-finish-validation` (192)' },

    # library/02-tags-lyrics-tools.md
    @{ rel = 'docs/library/02-tags-lyrics-tools.md'
       de  = 'nem substitui os 12 VU meters estéreo originais'
       para = 'nem substitui os 16 VU meters estéreo originais' },
    @{ rel = 'docs/library/02-tags-lyrics-tools.md'
       de  = '| LYR-AC-10 | 12 VUs originais e temas'
       para = '| LYR-AC-10 | 16 VUs originais e temas' },

    # product/05-decisions.md
    @{ rel = 'docs/product/05-decisions.md'
       de  = 'preservar byte a byte os 12 SVGs aprovados'
       para = 'preservar byte a byte os 16 SVGs aprovados' },
    @{ rel = 'docs/product/05-decisions.md'
       de  = 'sem reduzir nem distorcer os 12 VUs'
       para = 'sem reduzir nem distorcer os 16 VUs' },
    # A divergencia que D-MVP-04 registrava deixa de existir com M4 aplicado.
    # A citacao literal do themes.json ("encaixe universal para os 12 VUs
    # aprovados") fica: e citacao, e o texto do MVP que ficou para tras.
    @{ rel = 'docs/product/05-decisions.md'
       de  = 'Hoje: o contrato VU-INT fala em 12 VU meters (número herdado da rev.15), e o pin rev.38 tem **16 medidores, todos selecionáveis**. A contagem certa é **16**; os 12 são anteriores aos quatro medidores acrescentados depois. A matriz VU-INT-06 (144 combinações = 12 skins × 12 VUs) é atualizada quando a TASK-E02-07 tratar a integração visual'
       para = 'Este repositório e o pin rev.38 concordam em **16 medidores, todos selecionáveis**. A contagem herdada da rev.15 era 12; ela foi corrigida por M4 no contrato VU-INT e na matriz VU-INT-06 (192 combinações = 12 skins × 16 VUs).' }
)

$m4Aplicados = 0
$m4Falhas = @()
foreach ($e in $m4) {
    $p = Join-Path $RepoRoot $e.rel
    if (-not (Test-Path $p)) { $m4Falhas += "ausente: $($e.rel)"; continue }
    $txt = Read-Utf8 -Path $p
    $n = ([regex]::Matches($txt, [regex]::Escape($e.de))).Count
    if ($n -eq 0) {
        # Reexecucao: o `para` ja presente conta como aplicado.
        if ($txt.Contains($e.para)) { continue }
        $m4Falhas += "nao encontrado em $($e.rel): $($e.de.Substring(0, [Math]::Min(60, $e.de.Length)))"
        continue
    }
    if ($n -gt 1) {
        $m4Falhas += "ambiguo ($n ocorrencias) em $($e.rel): $($e.de.Substring(0, [Math]::Min(60, $e.de.Length)))"
        continue
    }
    if ($PSCmdlet.ShouldProcess($e.rel, 'M4: 12 -> 16 VUs')) {
        Write-Utf8 -Path $p -Text ($txt.Replace($e.de, $e.para))
    }
    $m4Aplicados++
}
Write-Feito "M4: $m4Aplicados ponto(s) aplicado(s) (12 -> 16 VUs); VU-INT-06 = 12 skins x 16 VUs = 192"
if ($m4Falhas) {
    $m4Falhas | ForEach-Object { Write-Aviso "M4: $_" }
    throw "M4 incompleto: $($m4Falhas.Count) ponto(s) nao casaram. Corrija os pares de/para antes de seguir."
}

# Varredura final: cada ocorrencia restante tem de ser historica ou citacao.
# Qualquer outra e contagem vigente que sobrou -- avisa e falha, porque a
# mensagem de commit deste passo afirma que M4 foi aplicado.
$m4Historico = @(
    @{ rel = 'docs/design/05-dsp-panel.md'; texto = '144 combinações de 12 skins × 12 VUs' },
    @{ rel = 'docs/product/05-decisions.md'; texto = 'encaixe universal para os 12 VUs aprovados' }
)
$m4Restantes = @()
# Sob -WhatIf nenhuma escrita aconteceu, entao o texto ainda tem os 12: a
# varredura acusaria o proprio -WhatIf. Ela so roda quando ha escrita real.
if (-not $WhatIfPreference) {
foreach ($rel in ($m4 | ForEach-Object { $_.rel } | Sort-Object -Unique)) {
    $p = Join-Path $RepoRoot $rel
    if (-not (Test-Path $p)) { continue }
    $linhas = (Read-Utf8 -Path $p) -split "`r?`n"
    for ($i = 0; $i -lt $linhas.Count; $i++) {
        if ($linhas[$i] -notmatch '12\s*(VUs?|VU meters?|SVGs?|modelos?)\b|\b144\b') { continue }
        $historico = $false
        foreach ($h in $m4Historico) {
            if ($h.rel -eq $rel -and $linhas[$i].Contains($h.texto)) { $historico = $true }
        }
        if ($historico) { continue }
        $m4Restantes += '{0}:{1}' -f $rel, ($i + 1)
    }
}
if ($m4Restantes.Count -gt 0) {
    $m4Restantes | ForEach-Object { Write-Aviso "M4: contagem vigente em 12 sobreviveu em $_" }
    throw "M4 incompleto: $($m4Restantes.Count) linha(s) ainda afirmam 12 VUs / 144."
}
Write-Feito 'M4: nenhuma contagem vigente em 12 sobrou (so historico e citacao do MVP)'
} else {
    Write-Feito 'M4: verificacao final pulada (WhatIf: nada foi escrito)'
}

# Fora do M4: as TASKs ainda citam 12 VUs e sao reescritas no passo 37. Fica
# visivel aqui para nao passar como pronto por engano.
$m4Tasks = git ls-files 'tasks/*.md' 'tasks/**/*.md' |
    Where-Object { (Test-Path (Join-Path $RepoRoot $_)) -and ((Read-Utf8 -Path (Join-Path $RepoRoot $_)) -match '\b12 VUs\b') }
if ($m4Tasks) { Write-Aviso "M4: TASKs com 12 VUs (reescritas no passo 37): $(@($m4Tasks) -join ', ')" }

# ------------------------------------------------------------------ 4. M6: Woodpecker
Write-Passo 'M6: removendo Woodpecker (CHANGELOG preservado)'

# A substituicao cega de "Woodpecker" por "GitHub Actions" foi DESCARTADA: ela
# renomeia caminhos (ADR-010-woodpecker-only-ci.md, chore-woodpecker-3-20-1) em
# links, e transforma identificadores de codigo em coisa que nao compila
# (fn github_actions_rejects_woodpecker -> "..._rejects_GitHub Actions").
# Woodpecker sai pelo arquivamento da ADR-010 e da evidencia (lista $arquivar
# acima) e pela remocao do .woodpecker/ (ja feito na ADR-014) -- nao por sed no
# texto. O que sobra no texto e registro historico e deve ficar: a ADR-014
# existe justamente para explicar por que a ADR-010 foi substituida, e apagar o
# motivo falsifica o registro.
#
# O que este bloco corrige: afirmacoes de que Woodpecker e o CI ATUAL.
$correcoesM6 = @(
    @{ rel = 'docs/engineering/04-ci-cd.md'; de = 'CI/CD somente por GitHub Actions'; ja = $true },
    @{ rel = 'docs/product/03-requirements.md'; de = 'nenhum workflow referencia Woodpecker'; ja = $true }
)

$totalM6 = 0
$arquivosM6 = 0
$pendentesM6 = @()
foreach ($c in $correcoesM6) {
    $p = Join-Path $RepoRoot $c.rel
    if (-not (Test-Path $p)) { continue }
    $txt = Read-Utf8 -Path $p
    if ($txt -notmatch '(?i)woodpecker') { continue }
    $arquivosM6++
    if ($c.ja) {
        # ja escrito no vocabulario do GitHub Actions; a mencao restante e historica
        Write-Feito "ja correto: $($c.rel)"
        continue
    }
    $pendentesM6 += $c.rel
}

# Varredura final: so REPORTAR. Nenhuma escrita, porque nao ha substituicao
# mecanica segura -- as ocorrencias restantes sao historicas.
$restantesM6 = git ls-files 'docs/*.md' 'tasks/*.md' 'tasks/**/*.md' 'SPEC.md' 'README.md' |
    Where-Object { $_ -notlike 'docs/evidence/*' -and $_ -notlike '_arquivo/*' -and $_ -ne 'CHANGELOG.md' } |
    Where-Object { (Test-Path (Join-Path $RepoRoot $_)) -and ((Read-Utf8 -Path (Join-Path $RepoRoot $_)) -match '(?i)woodpecker') }
$totalM6 = @($restantesM6).Count
Write-Feito "M6: $totalM6 arquivo(s) ainda citam Woodpecker (historico; ADR-010 arquivada)"
if ($pendentesM6) { Write-Aviso "corrigir a mao (afirmam CI atual): $($pendentesM6 -join ', ')" }
Write-Aviso 'NAO substituir "Woodpecker" por "GitHub Actions" no texto: quebra links e identificadores'

# xtask: o validador de pipeline Woodpecker sai
$validateRs = Join-Path $RepoRoot 'xtask\src\validate.rs'
if (Test-Path $validateRs) {
    $v = Read-Utf8 -Path $validateRs
    if ($v -match '(?i)woodpecker') {
        Write-Aviso 'xtask/src/validate.rs ainda menciona Woodpecker: remover a funcao inteira a mao'
    }
}

# ------------------------------------------------------------------ 5. JWT
Write-Passo 'removendo a regra com JWT de .claude/settings.local.json'

$settingsLocal = Join-Path $ExternoRoot '.claude\settings.local.json'
if (Test-Path $settingsLocal) {
    $json = Read-Utf8 -Path $settingsLocal | ConvertFrom-Json
    $removidas = 0
    foreach ($prop in @('permissions')) {
        if ($json.PSObject.Properties.Name -contains $prop) {
            foreach ($campo in @('allow', 'deny', 'ask')) {
                if ($json.$prop.PSObject.Properties.Name -contains $campo) {
                    $novo = @()
                    foreach ($regra in $json.$prop.$campo) {
                        if ($regra -match '(?i)woodpecker|WOODPECKER_TOKEN|eyJ') {
                            $removidas++
                            Write-Host "      removida: $($regra.Substring(0, [Math]::Min(60, $regra.Length)))..." -ForegroundColor DarkYellow
                        } else {
                            $novo += $regra
                        }
                    }
                    $json.$prop.$campo = $novo
                }
            }
        }
    }
    if ($removidas -gt 0 -and $PSCmdlet.ShouldProcess($settingsLocal, "remover $removidas regra(s) com JWT")) {
        Copy-Item $settingsLocal "$settingsLocal.bak-30" -Force
        Write-Utf8 -Path $settingsLocal -Text ($json | ConvertTo-Json -Depth 20)
        Write-Feito "removidas $removidas regra(s); backup em settings.local.json.bak-30"
    } else {
        Write-Feito 'nenhuma regra com JWT encontrada'
    }
    Write-Aviso 'REVOGAR o token no servico Woodpecker. Remover do arquivo nao invalida o token.'
} else {
    Write-Aviso "nao encontrado: $settingsLocal"
}

# ------------------------------------------------------------------ 6. .gitignore
Write-Passo 'corrigindo .gitignore'

$gitignore = Join-Path $RepoRoot '.gitignore'
# _arquivo/ NAO entra aqui: ele e versionado, porque o passo 38 apaga do disco o
# que ele guarda. Ignorar deixaria a exclusao sem volta num clone novo.
$necessarias = @('*.log', 'VALIDATION.json', '.cache/', '/target/')
$atual = if (Test-Path $gitignore) { Read-Utf8 -Path $gitignore } else { '' }
$faltando = $necessarias | Where-Object { $atual -notmatch [regex]::Escape($_) }

if ($faltando) {
    if ($PSCmdlet.ShouldProcess($gitignore, "adicionar: $($faltando -join ', ')")) {
        $bloco = @(
            '',
            '# passo 30',
            '# *.log NAO deve ser ignorado em docs/evidence: ver !docs/evidence/**',
            '!docs/evidence/**',
            'VALIDATION.json',
            '.cache/'
        ) + $faltando
        Append-Utf8 -Path $gitignore -Text ($bloco -join "`n")
        Write-Feito "adicionado: $($faltando -join ', ')"
    }
} else {
    Write-Feito '.gitignore ja completo'
}

# ------------------------------------------------------------------ 7. ancora quebrada
Write-Passo 'corrigindo ancora quebrada em tools/02-vynamp-converter.md'

$tools02 = Join-Path $RepoRoot 'docs\tools\02-vynamp-converter.md'
if (Test-Path $tools02) {
    $t = Read-Utf8 -Path $tools02
    $novo = $t -replace '#3-preservação-de-lotes-batching-atomic', '#3-batching-atomico'
    if ($novo -ne $t -and $PSCmdlet.ShouldProcess($tools02, 'corrigir ancora')) {
        Write-Utf8 -Path $tools02 -Text $novo -NoTrailingNewline
        Write-Feito 'ancora corrigida'
    } else {
        Write-Feito 'ancora ja correta ou arquivo antes do arquivamento'
    }
}

# ------------------------------------------------------------------ 8. M8 gate
Write-Passo 'M8: AUDIOSR e SOURCE_SEPARATION'

$adr015 = Test-Path (Join-Path $RepoRoot 'docs/architecture/adr/ADR-015-ai-restoration-future-phase.md')
$tools03 = Test-Path (Join-Path $RepoRoot 'docs/tools/03-vynamp-restore-ia.md')

if ($adr015 -and $tools03) {
    Write-Feito 'ADR-015 e tools/03 presentes: incorporacao concluida, os dois .md podem ser arquivados'
    foreach ($f in @('AUDIOSR-RUST-CPU.md', 'SOURCE_SEPARATION_MODELS.md')) {
        $src = Join-Path $ExternoRoot $f
        if (Test-Path $src) {
            $dstDir = Join-Path $ArquivoRoot 'raiz'
            if (-not (Test-Path $dstDir)) { New-Item -ItemType Directory -Path $dstDir -Force | Out-Null }
            if ($PSCmdlet.ShouldProcess($src, "mover para _arquivo/raiz/")) {
                Move-Item -Path $src -Destination (Join-Path $dstDir $f) -Force
                Add-Indice -Origem "raiz/$f" -Destino "_arquivo/raiz/$f" `
                    -Motivo "absorvido por ADR-015 + tools/03 em $(Get-Date -Format 'yyyy-MM-dd')"
                Write-Feito "arquivado: $f"
            }
        }
    }
} else {
    Write-Aviso 'ADR-015 ou tools/03 ausentes: o merge de chore/claude-specs-ia-futura ainda nao aconteceu'
    Write-Aviso 'AUDIOSR-RUST-CPU.md e SOURCE_SEPARATION_MODELS.md NAO foram tocados (M8 exige o merge antes)'
    if (-not $SkipM8Gate) {
        Write-Host ''
        Write-Host '    O passo 30 pode terminar sem M8, mas nao pule o merge depois.' -ForegroundColor Yellow
    }
}

# ------------------------------------------------------------------ 9. M11: heranca
Write-Passo 'M11: registrando o estado herdado'

$changelog = Join-Path $RepoRoot 'CHANGELOG.md'
if (Test-Path $changelog) {
    $ch = Read-Utf8 -Path $changelog
    if ($ch -notmatch '29e14e6') {
        $entrada = @(
            '',
            '## Estado herdado — reorganizacao em um repo por modulo',
            '',
            "Data: $(Get-Date -Format 'yyyy-MM-dd')",
            '',
            'Este repositorio substitui `jimmykedlyn/vynamp-mediaplayer`, que passa a ser',
            'arquivado (somente leitura). O ultimo commit herdado foi `29e14e6` em `develop`.',
            'Os 109 commits anteriores e os PRs #1-#39 permanecem legiveis no repositorio',
            'arquivado; citacoes a PR nas specs usam o nome completo quando ambiguas.',
            ''
        )
        if ($PSCmdlet.ShouldProcess($changelog, 'registrar estado herdado')) {
            Append-Utf8 -Path $changelog -Text ($entrada -join "`n")
            Write-Feito "registrado: SHA 29e14e6"
        }
    } else {
        Write-Feito 'ja registrado'
    }
}

# ------------------------------------------------------------------ 10. resumo
Write-Passo 'resumo'

Write-Host ''
Write-Host '    Movidos para _arquivo/:  ' -NoNewline
Write-Host $(if (Test-Path $IndexPath) { ((Read-Utf8 -Path $IndexPath) -split "`r?`n" | Where-Object { $_ -like '| 20*' }).Count } else { 'n/a (WhatIf)' })
Write-Host '    M4 aplicados:           ' -NoNewline
Write-Host "$m4Aplicados (12 -> 16 VUs; 192 combinacoes)"
Write-Host '    M6 mencoes restantes:   ' -NoNewline
Write-Host "$totalM6 arquivo(s) (historico; ADR-010 arquivada)"
Write-Host ''
Write-Host '    Verificacao:' -ForegroundColor Cyan
Write-Host '      git status --short'
Write-Host '      git diff --stat'
Write-Host '      grep -ri woodpecker docs   # so historico (ADR-010 arquivada, ADR-014) e CHANGELOG.md'
Write-Host '      grep -rn "12 VUs" docs     # zero no que afirma contagem vigente'
Write-Host ''
Write-Host '    Nada foi commitado. Revise o diff e commite com a mensagem do 30-limpeza.md.' -ForegroundColor Yellow
Write-Host '    ACOES MANUAIS: (1) revogar o JWT Woodpecker no servico' -ForegroundColor Yellow
Write-Host '                   (2) revisar D-MVP-04, VU-INT-06 e as TASKs E15-04/E19-01/E19-02/E19-05' -ForegroundColor Yellow
Write-Host ''

} finally {
    Pop-Location
}
