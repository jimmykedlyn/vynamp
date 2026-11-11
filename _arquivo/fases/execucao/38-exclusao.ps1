<#
    38-exclusao.ps1 — passo 38 da FASE 3

    O UNICO passo da FASE 3 que apaga do disco. Remove a raiz externa antiga
    (build, worktrees, conteudo nao migrado, cache descartavel) e o
    vynamp-chatgpt-arquivo. Nada mais.

    O que NAO apaga, por construcao:
      - D:\Projects\vynamp (o umbrella) e qualquer outro repo D:\Projects\vynamp*
      - AMBIENTE.ps1 na raiz externa: e o unico caminho de autenticacao hoje
      - .cache\cargo (CARGO_HOME) e .cache\import-completed.json (recibo do INICIAR)
      - .cache\tools\ffmpeg e .cache\flac-test-files quando uma variavel do
        AMBIENTE.ps1 aponta para eles (fixtures de dev)
      - vitrola, vynamp-mvp-reference, vynamp-decode, vynamp-output, vynamp-fases
      - _vynamp-recuperacao-2026-11-06 (nao esta em nenhuma lista: fica)

    Cada caminho removido ganha uma linha em _arquivo/INDEX.md do umbrella, com
    Destino '-' e passo 38. O indice e versionado: um clone novo sabe o que sumiu.

    Nao faz push, merge, tag, branch nem gh. Nao commita: deixa o INDEX.md
    modificado para o mantenedor revisar e commitar.

    Uso:  pwsh -NoProfile -File .\38-exclusao.ps1
          .\38-exclusao.ps1 -WhatIf          # mostra tudo, nao apaga nada
          .\38-exclusao.ps1 -SemConfirmacao  # so para -WhatIf; sem a pergunta
#>
[CmdletBinding(SupportsShouldProcess = $true)]
param(
    [string]$ExternoRoot = 'D:\Projects\vynamp-media-player',
    [string]$Raiz        = 'D:\Projects\vynamp',
    [string]$ChatGpt     = 'D:\Projects\vynamp-chatgpt-arquivo',
    [string]$Smoke       = 'D:\Projects\_smoke37',
    [switch]$SemConfirmacao
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

function Write-Passo { param($m) Write-Host "==> $m" -ForegroundColor Cyan }
function Write-Feito { param($m) Write-Host "    $m" -ForegroundColor DarkGray }
function Write-Aviso { param($m) Write-Host "    ! $m" -ForegroundColor Yellow }

# ---------------------------------------------------------------------------
# Encoding. Alvo: PowerShell 7+ (pwsh). Os helpers garantem UTF-8 SEM BOM na
# saida, que e o que os parsers de YAML do GitHub e o frontmatter exigem.
# ---------------------------------------------------------------------------
$Utf8SemBom = New-Object System.Text.UTF8Encoding($false)

# Linhas do AMBIENTE.ps1 externo, quando ele existe. O portao das fixtures le
# daqui alem da variavel do processo: ver o comentario em $cacheDescartavel.
$ambientePs1 = Join-Path $ExternoRoot 'AMBIENTE.ps1'
$ambiente = $null
if (Test-Path -LiteralPath $ambientePs1) {
    $ambiente = @([System.IO.File]::ReadAllLines($ambientePs1, [System.Text.Encoding]::UTF8))
}

function Read-Utf8 {
    param([string]$Path)
    return [System.IO.File]::ReadAllText($Path, [System.Text.Encoding]::UTF8)
}
function Append-Utf8 {
    param([string]$Path, [string]$Text)
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

function Pega-Tamanho {
    param([string]$Path)
    if (-not (Test-Path -LiteralPath $Path)) { return $null }
    $item = Get-Item -LiteralPath $Path -Force
    if ($item.PSIsContainer) {
        # Set-StrictMode: Measure-Object sem entrada nao devolve .Sum. Contar
        # antes evita o erro e devolve 0 para pasta vazia.
        $arquivos = @(Get-ChildItem -LiteralPath $Path -Recurse -File -Force -ErrorAction SilentlyContinue)
        $bytes = if ($arquivos.Count -eq 0) { 0 } else { ($arquivos | Measure-Object -Property Length -Sum).Sum }
    } else {
        $bytes = $item.Length
    }
    if ($bytes -ge 1GB) { return ('{0:N1} GB' -f ($bytes / 1GB)) }
    if ($bytes -ge 1MB) { return ('{0:N0} MB' -f ($bytes / 1MB)) }
    if ($bytes -ge 1KB) { return ('{0:N0} KB' -f ($bytes / 1KB)) }
    return ('{0} B' -f $bytes)
}

# ---------------------------------------------------------------------------
# Caminhos fixos
# ---------------------------------------------------------------------------
$IndexPath = Join-Path $Raiz '_arquivo\INDEX.md'

# Nada e apagado aqui, em nenhuma hipotese. A lista existe para a checagem 2.
$protegidos = @(
    (Join-Path $ExternoRoot 'AMBIENTE.ps1'),
    (Join-Path $ExternoRoot '.cache\cargo'),
    (Join-Path $ExternoRoot '.cache\import-completed.json'),
    (Join-Path $ExternoRoot 'INICIAR.ps1'),
    (Join-Path $ExternoRoot 'ABRIR-CLAUDE.ps1'),
    (Join-Path $ExternoRoot 'PROMPT-CLAUDE.txt'),
    (Join-Path $ExternoRoot 'LEIA-ME.md'),
    (Join-Path $ExternoRoot 'AGENTS.md'),
    (Join-Path $ExternoRoot 'STATE.md'),
    (Join-Path $ExternoRoot '.claude'),
    $Raiz,
    (Join-Path $Raiz '.git')
) | ForEach-Object { [System.IO.Path]::GetFullPath($_) }

# Fixtures que um script de dev usa hoje. So entram na exclusao quando a
# variavel do AMBIENTE.ps1 NAO aponta para dentro delas.
$fixtures = [ordered]@{
    'tools'           = 'FFMPEG'
    'flac-test-files' = 'VYNAMP_FLAC_TEST_FILES'
}

# Conteudo descartavel de .cache (B). cargo/ e import-completed.json NAO estao.
$cacheDescartavel = @(
    'tmp', 'bootstrap-build', 'e15-01-verification-20261015', 'task-e15-01-driver',
    'woodpecker', 'doctor.txt', 'pr-task-e15-01-eac3221.md', 'wpw', 'tests',
    'tools', 'flac-test-files'
)

# ---------------------------------------------------------------------------
# 0. pre-checagem
# ---------------------------------------------------------------------------
Write-Passo 'pre-checagem'

if (-not (Test-Path -LiteralPath $Raiz)) {
    throw "Umbrella nao encontrado: $Raiz. Rode os passos 31 e 35 antes deste."
}
if (-not (Test-Path -LiteralPath (Join-Path $Raiz '.git'))) {
    throw "$Raiz nao e repo git. O passo 38 pressupoe o umbrella criado e com historico."
}
if (-not (Test-Path -LiteralPath $IndexPath)) {
    throw "INDICE ausente: $IndexPath. Sem ele nada e apagado: sem registro nao ha volta."
}
Write-Feito "umbrella: $Raiz"
Write-Feito "indice: $IndexPath"

# O grupo C apaga a casca do repo antigo. So entra se o _arquivo de la estiver
# VAZIO e o .git nao existir (foi movido no 31) ou apontar para o vynamp.
$arquivoAntigo = Join-Path $ExternoRoot 'repo\_arquivo'
$gitAntigo     = Join-Path $ExternoRoot 'repo\.git'
$cLiberado     = $true
$cMotivo       = @()

if (Test-Path -LiteralPath $arquivoAntigo) {
    $sobrou = @(Get-ChildItem -LiteralPath $arquivoAntigo -Recurse -File -Force -ErrorAction SilentlyContinue)
    if ($sobrou.Count -gt 0) {
        $cLiberado = $false
        $cMotivo  += "$($sobrou.Count) arquivo(s) em repo\_arquivo: conteudo nao migrado"
    }
}
if (Test-Path -LiteralPath $gitAntigo) {
    $origin = (& git -C (Join-Path $ExternoRoot 'repo') remote get-url origin 2>$null)
    $origin = ([string]$origin).Trim()
    if ($origin -notmatch 'github\.com[:/]jimmykedlyn/vynamp(\.git)?$') {
        $cLiberado = $false
        $cMotivo  += "repo\.git presente e origin='$origin' (esperado .../vynamp)"
    }
}
if ($cLiberado) { Write-Feito 'grupo C liberado: casca do repo antigo sem conteudo vivo' }
else {
    Write-Aviso 'grupo C RECUSADO:'
    $cMotivo | ForEach-Object { Write-Aviso "  $_" }
    Write-Aviso '  Nada de repo\ sera apagado alem do que os grupos A/B ja levam.'
}

# ---------------------------------------------------------------------------
# 1. montar os grupos com o que existe de fato
# ---------------------------------------------------------------------------
Write-Passo 'lendo o disco'

function New-Entrada {
    param([string]$Path, [string]$Motivo, [string]$Chave)
    $full = [System.IO.Path]::GetFullPath($Path)
    $tam = Pega-Tamanho -Path $full
    return [pscustomobject]@{
        Path   = $full
        Motivo = $Motivo
        Chave  = $Chave
        Tamanho = $tam
        Existe = ($null -ne $tam)
    }
}

# Cada grupo e sempre um array: com Set-StrictMode, $null.Count e erro, e um
# pipeline vazio devolve $null. Reexecutar depois de apagar tudo cairia nisso.
$grupoA = @(
    @(
        (New-Entrada (Join-Path $ExternoRoot 'repo\target')     'build do workspace antigo (3,1 GB)' 'repo\target'),
        (New-Entrada (Join-Path $ExternoRoot 'repo\.worktrees') 'worktrees das branches ja mergeadas (11 GB)' 'repo\.worktrees'),
        (New-Entrada $Smoke                                     'teste de smoke do passo 37' '_smoke37')
    ) | Where-Object { $_.Existe -and -not $_.Path.StartsWith([System.IO.Path]::GetFullPath($Raiz) + '\', [StringComparison]::OrdinalIgnoreCase) }
)

$grupoB = @(
    (New-Entrada (Join-Path $ExternoRoot 'archive') 'historico do fluxo antigo, nao migrado' 'repo\archive'),
    (New-Entrada (Join-Path $ExternoRoot 'logs')    'residuo de log; o AMBIENTE.ps1 recria vazia' 'repo\logs'),
    (New-Entrada $ChatGpt                          'M10: fora das specs novas; nao e git' 'vynamp-chatgpt-arquivo')
)
foreach ($c in $cacheDescartavel) {
    $p = Join-Path $ExternoRoot ".cache\$c"
    if (-not (Test-Path -LiteralPath $p)) { continue }

    # fixture em uso hoje? entao fica.
    #
    # A consulta tem DUAS fontes, e as duas contam:
    #
    #  1. a variavel de ambiente do processo -- funciona para quem carregou o
    #     AMBIENTE.ps1 antes;
    #  2. o TEXTO do AMBIENTE.ps1, que e a fonte que sobrevive. `$env:FFMPEG` e
    #     atribuido na linha 65 daquele arquivo a partir de `$PSScriptRoot`;
    #     ele nao existe como variavel persistente do usuario, entao um run
    #     isolado do passo 38 lia `$null` e classificava `.cache\tools` como
    #     descartavel. Os tres `tools/fixtures/generate*.ps1` do decode fazem
    #     `. D:\Projects\vynamp-media-player\AMBIENTE.ps1` e usam exatamente
    #     esse ffmpeg: apagar quebra um teste que passa hoje, e e justamente o
    #     que a tabela do passo pede para pular.
    if ($fixtures.Contains($c)) {
        $var = $fixtures[$c]
        $aponta = [Environment]::GetEnvironmentVariable($var)
        $emUso = $false
        if ($aponta -and [System.IO.Path]::GetFullPath($aponta).StartsWith([System.IO.Path]::GetFullPath($p), [StringComparison]::OrdinalIgnoreCase)) {
            $emUso = $true
        }
        if (-not $emUso -and $null -ne $ambiente) {
            # Basta a atribuicao citar a pasta: reconstruir a expressao
            # PowerShell aqui seria pior que nao checar -- erra calado.
            $fragmento = ".cache\$c"
            foreach ($linha in $ambiente) {
                if ($linha -match ('\$env:' + [regex]::Escape($var) + '\s*=') -and
                    $linha.IndexOf($fragmento, [StringComparison]::OrdinalIgnoreCase) -ge 0) {
                    $emUso = $true
                    break
                }
            }
        }
        if ($emUso) {
            Write-Aviso "${var} aponta para .cache\${c} (processo ou AMBIENTE.ps1): preservado (fixture de dev)"
            continue
        }
    }
    $grupoB += New-Entrada $p "cache descartavel do bootstrap ($c)" ".cache\$c"
}
$grupoB = @($grupoB | Where-Object { $_.Existe })

# C: a casca do repo antigo, e so ela. target/.worktrees ja saem em A.
$grupoC = @()
if ($cLiberado -and (Test-Path -LiteralPath (Join-Path $ExternoRoot 'repo'))) {
    $cripto = New-Entrada (Join-Path $ExternoRoot 'repo') 'casca do repo antigo; historico migrado para o umbrella (passo 35)' 'repo'
    if ($cripto.Existe) { $grupoC = @($cripto) }
}
$grupoB = @($grupoB)

# ---------------------------------------------------------------------------
# 2. mostrar e confirmar
# ---------------------------------------------------------------------------
Write-Passo 'resumo do que sera apagado'

$total = 0
foreach ($g in @(@{ n = 'A'; l = $grupoA }, @{ n = 'B'; l = $grupoB }, @{ n = 'C'; l = $grupoC })) {
    if ($g.l.Count -eq 0) { continue }
    Write-Host "    Grupo $($g.n):" -ForegroundColor White
    foreach ($e in $g.l) {
        Write-Host ("      {0,-14} {1}" -f $e.Tamanho, $e.Path)
        Write-Host ("      {0,-14} {1}" -f '', $e.Motivo) -ForegroundColor DarkGray
        $total++
    }
}
if ($total -eq 0) {
    Write-Feito 'nada a apagar: o disco ja esta limpo'
    return
}
Write-Host ''
Write-Host '    Preservado de proposito:' -ForegroundColor Cyan
Write-Host '      AMBIENTE.ps1 (autenticacao), .cache\cargo (CARGO_HOME),' -ForegroundColor DarkGray
Write-Host '      .cache\import-completed.json, o umbrella e os repos vynamp*,' -ForegroundColor DarkGray
Write-Host '      vitrola, vynamp-fases, _vynamp-recuperacao-2026-11-06' -ForegroundColor DarkGray
Write-Host ''

if (-not $SemConfirmacao) {
    $resp = Read-Host '    Digite "pode excluir" para prosseguir'
    if ($resp -ne 'pode excluir') {
        Write-Aviso 'Confirmacao nao dada. NADA foi apagado.'
        return
    }
} else {
    Write-Aviso 'pergunta de confirmacao pulada (-SemConfirmacao)'
}

# ---------------------------------------------------------------------------
# 3. porta de seguranca: nada sai sem passar por aqui
# ---------------------------------------------------------------------------
function Test-PodeApagar {
    param([string]$Path, [string[]]$Lista)

    $full = [System.IO.Path]::GetFullPath($Path)

    # 1. dentro da raiz externa, ou uma das duas excecoes explicitas. As duas
    #    sao irmas da raiz externa, nao filhas: por isso a comparacao e exata,
    #    nunca por prefixo -- um prefixo errado apagaria D:\Projects inteiro.
    $naExterna = $full.StartsWith([System.IO.Path]::GetFullPath($ExternoRoot) + '\', [StringComparison]::OrdinalIgnoreCase)
    $excecoes  = @([System.IO.Path]::GetFullPath($Smoke), [System.IO.Path]::GetFullPath($ChatGpt))
    $eExcecao  = $excecoes -contains $full
    if (-not ($naExterna -or $eExcecao)) {
        throw "RECUSADO (fora da raiz externa e das excecoes): $full"
    }

    # 2. esta na lista do grupo e nao e protegido nem ancestor de protegido
    if ($Lista -notcontains $full) { throw "RECUSADO (nao esta na lista do grupo): $full" }
    foreach ($p in $protegidos) {
        if ($full.Equals($p, [StringComparison]::OrdinalIgnoreCase)) {
            throw "RECUSADO (caminho protegido): $full"
        }
        if ($p.StartsWith($full + '\', [StringComparison]::OrdinalIgnoreCase)) {
            throw "RECUSADO (ancestor de caminho protegido '$p'): $full"
        }
    }

    # 3. nunca ancestor do umbrella nem de outro repo D:\Projects\vynamp*
    #
    #    O filtro de irmao exclui os proprios alvos deste passo. `vynamp*` casa
    #    `vynamp-chatgpt-arquivo`, que e justamente o alvo do grupo B: sem a
    #    exclusao a checagem 3 recusava o proprio caminho que a checagem 1
    #    autoriza, e o grupo B parava no primeiro item do .cache -- com A e
    #    parte de B ja apagados. As excecoes explicitas (1) continuam sendo a
    #    unica porta para fora da raiz externa; aqui so se impede que elas
    #    virem "repo vivo" pelo padrao de nome.
    $alvos = @([System.IO.Path]::GetFullPath($Smoke), [System.IO.Path]::GetFullPath($ChatGpt))
    foreach ($irmao in @($Raiz) + @(Get-ChildItem -LiteralPath 'D:\Projects' -Directory -Force -ErrorAction SilentlyContinue |
                                     Where-Object { ($_.Name -like 'vynamp*' -or $_.Name -eq 'vitrola') -and ($alvos -notcontains $_.FullName) } |
                                     ForEach-Object { $_.FullName })) {
        $irmaoFull = [System.IO.Path]::GetFullPath($irmao)
        if ($irmaoFull.Equals($full, [StringComparison]::OrdinalIgnoreCase)) {
            throw "RECUSADO (e um repo vivo): $full"
        }
        if ($irmaoFull.StartsWith($full + '\', [StringComparison]::OrdinalIgnoreCase)) {
            throw "RECUSADO (ancestor de '$irmaoFull'): $full"
        }
    }
    return $true
}

function Add-Indice {
    # 5 colunas, formato do passo 30. Idempotente por (Data, Origem, Passo).
    param([string]$Origem, [string]$Motivo)
    $Motivo = $Motivo -replace '\|', '/'
    $data = Get-Date -Format 'yyyy-MM-dd'
    if (Test-Path -LiteralPath $IndexPath) {
        $chave = "| {0} | `{1}` | - |" -f $data, $Origem
        $existe = (Read-Utf8 -Path $IndexPath) -split "`r?`n" |
            Where-Object { $_.StartsWith($chave) -and $_.EndsWith('| 38 |') }
        if ($existe) { return }
    }
    $linha = '| {0} | `{1}` | - | {2} | 38 |' -f $data, $Origem, $Motivo
    Append-Utf8 -Path $IndexPath -Text $linha
}

# ---------------------------------------------------------------------------
# 4. grupos A e B
# ---------------------------------------------------------------------------
function Invoke-Grupo {
    param([string]$Nome, [object[]]$Itens)

    if ($Itens.Count -eq 0) { return 0 }
    Write-Passo "grupo $Nome"

    $lista = @($Itens | ForEach-Object { [System.IO.Path]::GetFullPath($_.Path) })
    $apagados = 0

    foreach ($e in $Itens) {
        $full = [System.IO.Path]::GetFullPath($e.Path)
        Test-PodeApagar -Path $full -Lista $lista | Out-Null

        if (-not $PSCmdlet.ShouldProcess($full, "apagar ($($e.Tamanho))")) { continue }

        try {
            Remove-Item -LiteralPath $full -Recurse -Force -ErrorAction Stop
        } catch {
            Write-Aviso "falhou: $full - $($_.Exception.Message)"
            Write-Aviso '  (arquivo em uso? feche o programa e rode de novo; o passo e idempotente)'
            continue
        }
        Add-Indice -Origem $e.Chave -Motivo $e.Motivo
        Write-Feito "apagado: $($e.Chave) ($($e.Tamanho))"
        $apagados++
    }
    return $apagados
}

$nA = Invoke-Grupo -Nome 'A' -Itens $grupoA
$nB = Invoke-Grupo -Nome 'B' -Itens $grupoB

# C e separado: e o unico grupo que apaga uma raiz de repo, e depende do
# _arquivo do umbrella ja conter tudo. Confere de novo, no momento do uso.
$nC = 0
if ($grupoC.Count -gt 0) {
    $linhasUmbrella = @((Read-Utf8 -Path $IndexPath) -split "`r?`n" | Where-Object { $_ -like '| 20*' })
    $temArquivamentos = @($linhasUmbrella | Where-Object { $_ -notmatch '\| 38 \|$' }).Count
    if ($temArquivamentos -lt 1) {
        Write-Aviso "INDICE sem arquivamentos anteriores ($($linhasUmbrella.Count) linha total): grupo C recusado."
        Write-Aviso 'Apagar o repo sem a copia indexada seria apagar sem motivo.'
    } else {
        $nC = Invoke-Grupo -Nome 'C' -Itens $grupoC
    }
} else {
    Write-Feito 'nada a apagar em C'
}

# ---------------------------------------------------------------------------
# 5. relatorio
# ---------------------------------------------------------------------------
Write-Passo 'relatorio'

Write-Host ''
Write-Host '    Apagados: ' -NoNewline
Write-Host "A=$nA B=$nB C=$nC" -ForegroundColor White
Write-Host ''
Write-Host '    Verificacao:' -ForegroundColor Cyan
Write-Host '      Test-Path D:\Projects\vynamp-media-player\repo\target        # False'
Write-Host '      Test-Path D:\Projects\vynamp-chatgpt-arquivo                 # False'
Write-Host '      Test-Path D:\Projects\vynamp-media-player\AMBIENTE.ps1       # True'
Write-Host '      Test-Path D:\Projects\vynamp-media-player\.cache\cargo       # True'
Write-Host '      Test-Path D:\Projects\vynamp-media-player\.cache\import-completed.json  # True'
Write-Host '      Test-Path D:\Projects\vynamp                                 # True'
Write-Host '      git -C D:\Projects\vynamp status --short                     # so _arquivo/INDEX.md'
Write-Host ''

$restou = @()
foreach ($d in @('repo', 'archive')) {
    $p = Join-Path $ExternoRoot $d
    if (Test-Path -LiteralPath $p) { $restou += $d }
}
if ($restou.Count -gt 0) {
    Write-Aviso "ainda no disco em $ExternoRoot`: $($restou -join ', ')"
    Write-Aviso 'Se for so build, rode de novo; se for conteudo, decida antes de apagar.'
}

Write-Host '    NADA foi commitado. Revise o INDEX.md e commite no umbrella:' -ForegroundColor Yellow
Write-Host '      git -C D:\Projects\vynamp diff -- _arquivo/INDEX.md'
Write-Host '      git -C D:\Projects\vynamp add _arquivo/INDEX.md'
Write-Host '      git -C D:\Projects\vynamp commit -m "chore(limpeza): exclusao definitiva da raiz externa (passo 38)"'
Write-Host ''
