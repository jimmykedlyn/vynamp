<#
    32-agentes.ps1 — passo 32 da FASE 3

    Consolida os dois AGENTS.md num contrato unico (M5).
    Antes de arquivar os antigos, imprime as linhas que NAO foram carregadas no novo,
    para revisao manual. Sem push, sem merge, sem tag.

    Uso:  pwsh -NoProfile -File .\32-agentes.ps1
          .\32-agentes.ps1 -WhatIf
#>
[CmdletBinding(SupportsShouldProcess = $true)]
param()

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$Umbrella    = 'D:\Projects\vynamp'
$Fases       = 'D:\Projects\vynamp-fases'
$RaizExterna = 'D:\Projects\vynamp-media-player'
$ArquivoRoot = Join-Path $Umbrella '_arquivo'
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

if (-not (Test-Path $Umbrella)) { throw "Umbrella nao encontrado: $Umbrella" }

function Add-Indice {
    # Idempotente pela chave (Origem + Destino + Passo + Estado), SEM a data: com a
    # data na chave, reexecutar em outro dia duplicava a linha.
    # O INDEX tem 6 colunas (Data|Origem|Destino|Motivo|Passo|Estado) desde 9b344cc.
    # Gravar 5 desalinhava a tabela e a chave nunca casava.
    param([string]$Origem, [string]$Destino, [string]$Motivo, [string]$Passo = '32', [string]$Estado = 'arquivado')
    $Motivo = $Motivo -replace '\|', '/'
    $data = Get-Date -Format 'yyyy-MM-dd'
    $linhaSemData = "| `{0}` | `{1}` |" -f $Origem, $Destino
    $existe = (Read-Utf8 -Path $IndexPath) -split "`r?`n" | Where-Object {
        $_.Contains($linhaSemData) -and $_.EndsWith("| $Passo | $Estado |")
    }
    if ($existe) { return }
    $linha = '| {0} | `{1}` | `{2}` | {3} | {4} | {5} |' -f $data, $Origem, $Destino, $Motivo, $Passo, $Estado
    Append-Utf8 -Path $IndexPath -Text $linha
}

function Move-ParaArquivo {
    # git mv para um destino que ja existe falha com "destination exists" (exit 128); o
    # fallback antigo (Move-Item -Force) entao sobrescrevia o arquivo arquivado com o
    # conteudo novo -- o oposto do que o INDEX registra. Aqui o destino existente e
    # preservado e a divergencia e reportada, nao resolvida em silencio.
    param([string]$Origem, [string]$Destino)
    $dstDir = Split-Path $Destino -Parent
    if (-not (Test-Path $dstDir)) { New-Item -ItemType Directory -Path $dstDir -Force | Out-Null }
    if (Test-Path $Destino) {
        $hOrigem = (Get-FileHash -Path $Origem -Algorithm SHA256).Hash
        $hDestino = (Get-FileHash -Path $Destino -Algorithm SHA256).Hash
        if ($hOrigem -eq $hDestino) {
            Remove-Item -Path $Origem -Force
            Write-Feito "destino ja existe e e identico; origem removida: $Destino"
        } else {
            Write-Aviso "destino ja existe e DIVERGE; nada foi sobrescrito: $Destino"
            Write-Aviso "origem preservada: $Origem"
        }
        return
    }
    git mv -- $Origem $Destino 2>$null
    if ($LASTEXITCODE -ne 0) { Move-Item -Path $Origem -Destination $Destino -Force }
}

Push-Location $Umbrella
try {

# ------------------------------------------------------------------ 0. pre-checagem
Write-Passo 'pre-checagem'

if (-not (Test-Path $IndexPath)) {
    New-Item -ItemType Directory -Path $ArquivoRoot -Force | Out-Null
    $cabecalho = @(
        '# _arquivo — indice de arquivamento', '',
        '| Data | Origem | Destino | Motivo | Passo |', '|---|---|---|---|---|'
    ) -join "`n"
    Write-Utf8 -Path $IndexPath -Text $cabecalho
}

$dirty = git status --short
if ($dirty) {
    Write-Aviso 'arvore nao esta limpa; o passo 31 precisa estar commitado:'
    $dirty | ForEach-Object { Write-Host "      $_" }
    throw 'Commite o passo 31 antes.'
}
Write-Feito 'arvore limpa'

# ------------------------------------------------------------------ 1. AGENTS.md
Write-Passo 'instalando AGENTS.md'

$novoAgents = Join-Path $Fases 'agentes\AGENTS.md'
if (-not (Test-Path $novoAgents)) { throw "Template nao encontrado: $novoAgents" }

$dstAgents = Join-Path $Umbrella 'AGENTS.md'
if (Test-Path $dstAgents) {
    # compara antes de sobrescrever: mostra termos do antigo ausentes no novo
    Write-Passo 'comparando com o AGENTS.md existente'
    $antigo = Read-Utf8 -Path $dstAgents
    $novo   = Read-Utf8 -Path $novoAgents

    $termosAntigos = [regex]::Matches($antigo, '[A-Za-zÀ-ÿ][A-Za-zÀ-ÿ\-]{4,}') |
        ForEach-Object { $_.Value.ToLower() } | Sort-Object -Unique
    $novoLower = $novo.ToLower()
    $perdidos = $termosAntigos | Where-Object { -not $novoLower.Contains($_) }

    if ($perdidos) {
        Write-Aviso "$($perdidos.Count) termo(s) do AGENTS.md antigo nao aparecem no novo (revisar):"
        ($perdidos | Select-Object -First 40) | ForEach-Object { Write-Host "      $_" -ForegroundColor DarkYellow }
        if ($perdidos.Count -gt 40) { Write-Host "      ... e mais $($perdidos.Count - 40)" -ForegroundColor DarkYellow }
        Write-Host ''
        Write-Host '    Compare a mao:' -ForegroundColor Yellow
        Write-Host "      $dstAgents"
        Write-Host "      $novoAgents"
        Write-Host ''

        # Lista e para registro, nao pergunta. O diff por regex de palavra solta
        # acusa ~190 termos quase sempre (o antigo era prosa densa, o novo e
        # normativo) -- um prompt aqui trava qualquer execucao nao interativa e
        # nunca responde "sim" por um motivo bom. A comparacao de verdade e o
        # diff do commit, que entra no corpo do PR.
        Write-Warning 'O antigo vai para _arquivo/repo/; o diff esta no commit. Nada foi apagado.'
    }

    if ($PSCmdlet.ShouldProcess($dstAgents, 'substituir AGENTS.md')) {
        Move-ParaArquivo -Origem 'AGENTS.md' -Destino (Join-Path $ArquivoRoot 'repo/AGENTS.md')
        Add-Indice -Origem 'AGENTS.md' -Destino '_arquivo/repo/AGENTS.md' -Motivo 'M5: consolidado no contrato unico'
    }
}

if ($PSCmdlet.ShouldProcess($dstAgents, 'gravar contrato unico')) {
    Copy-Item $novoAgents $dstAgents -Force
    Write-Feito "AGENTS.md instalado ($([Math]::Round((Get-Item $dstAgents).Length / 1KB, 1)) KB)"
}

# ------------------------------------------------------------------ 2. CLAUDE.md
Write-Passo 'instalando CLAUDE.md'

$novoClaude = Join-Path $Fases 'agentes\CLAUDE.md'
$dstClaude  = Join-Path $Umbrella 'CLAUDE.md'

if (Test-Path $dstClaude) {
    if ($PSCmdlet.ShouldProcess($dstClaude, 'substituir CLAUDE.md')) {
        Move-ParaArquivo -Origem 'CLAUDE.md' -Destino (Join-Path $ArquivoRoot 'repo/CLAUDE.md')
        Add-Indice -Origem 'CLAUDE.md' -Destino '_arquivo/repo/CLAUDE.md' -Motivo 'M5: CLAUDE.md passa a ser so @AGENTS.md'
    }
}
if ($PSCmdlet.ShouldProcess($dstClaude, 'gravar CLAUDE.md de import')) {
    Copy-Item $novoClaude $dstClaude -Force
    Write-Feito 'CLAUDE.md instalado (4 linhas: @AGENTS.md)'
}

# ------------------------------------------------------------------ 3. raiz externa
Write-Passo 'arquivando a segunda autoridade (raiz externa)'

$agentsRaiz = Join-Path $RaizExterna 'AGENTS.md'
if (Test-Path $agentsRaiz) {
    if ($PSCmdlet.ShouldProcess($agentsRaiz, 'mover para _arquivo/raiz/')) {
        Move-ParaArquivo -Origem $agentsRaiz -Destino (Join-Path $ArquivoRoot 'raiz/AGENTS.md')
        Add-Indice -Origem 'raiz/AGENTS.md' -Destino '_arquivo/raiz/AGENTS.md' -Motivo 'M5: segunda autoridade conflitante, absorvida'
        Write-Feito 'AGENTS.md da raiz externa arquivado'
    }
} else {
    Write-Feito 'AGENTS.md da raiz externa ja nao existe'
}

# O bootstrap da raiz externa apontava para o AGENTS.md que acabou de sair de la.
# Sem esta correcao, a proxima sessao aberta por ABRIR-CLAUDE.ps1 recebe um caminho
# morto e o agente comeca sem contrato.
$promptBootstrap = Join-Path $RaizExterna 'PROMPT-CLAUDE.txt'
$agentsUmbrella = Join-Path $Umbrella 'AGENTS.md'
if ((Test-Path $promptBootstrap) -and ((Read-Utf8 -Path $promptBootstrap) -notmatch [regex]::Escape($agentsUmbrella))) {
    $texto = @(
        "Leia $agentsUmbrella e $RaizExterna\STATE.md. Voce e o unico autor e revisor do Vynamp.",
        'Retome a proxima acao do STATE.md seguindo o contrato do umbrella',
        '(Opus orquestra; Sonnet implementa/revisa; Haiku roda checks). Sem merge/release/force push.'
    ) -join ' '
    if ($PSCmdlet.ShouldProcess($promptBootstrap, 'apontar bootstrap para o contrato do umbrella')) {
        Write-Utf8 -Path $promptBootstrap -Text $texto
        Write-Feito "PROMPT-CLAUDE.txt aponta para $agentsUmbrella"
    }
}

# ------------------------------------------------------------------ 4. AGENTS.md de modulo
Write-Passo 'preparando AGENTS.md de modulo (aplicado no passo 35)'

$moduloAgents = @'
# AGENTS.md

O contrato do agente e o do umbrella:

https://github.com/jimmykedlyn/vynamp/blob/develop/AGENTS.md

Especifico deste modulo:

- <regra local, se houver. Se ela contrariar o umbrella, corrija o umbrella.>
'@
$padrao = Join-Path $Fases 'padroes\AGENTS-modulo.md'
if ($PSCmdlet.ShouldProcess($padrao, 'gravar padrao de AGENTS.md de modulo')) {
    Write-Utf8 -Path $padrao -Text $moduloAgents
    Write-Feito "padrao salvo em $padrao (usado pelo passo 35)"
}

# ------------------------------------------------------------------ 5. verificacao
Write-Passo 'verificacao'

$ok = $true
if (-not (Test-Path $dstAgents)) { Write-Aviso 'AGENTS.md ausente'; $ok = $false }
if (-not (Test-Path $dstClaude)) { Write-Aviso 'CLAUDE.md ausente'; $ok = $false }

if ($ok) {
    # Sob -WhatIf os destinos ainda sao os antigos: verificar o template, nao o alvo.
    $alvoAgents = if ($WhatIfPreference) { $novoAgents } else { $dstAgents }
    $alvoClaude = if ($WhatIfPreference) { $novoClaude } else { $dstClaude }

    $secoes = (Select-String -Path $alvoAgents -Pattern '^## ').Count
    $linhasClaude = (((Read-Utf8 -Path $alvoClaude).TrimEnd("`r", "`n")) -split "`r?`n").Count
    Write-Feito "AGENTS.md: $secoes secao(oes)"
    Write-Feito "CLAUDE.md: $linhasClaude linha(s)"
    if ($secoes -lt 6) { Write-Aviso 'esperado 6 secoes' }
    if ($linhasClaude -gt 6) { Write-Aviso 'CLAUDE.md deveria ter ~5 linhas' }
    $importa = Select-String -Path $alvoClaude -Pattern '@AGENTS\.md' -Quiet
    if ($importa) { Write-Feito 'CLAUDE.md importa @AGENTS.md' } else { Write-Aviso 'CLAUDE.md sem @AGENTS.md' }
}

Write-Host ''
Write-Host '    Proximo: confira o diff e commite com a mensagem do 32-agentes.md.' -ForegroundColor Yellow
Write-Host ''

} finally {
    Pop-Location
}
