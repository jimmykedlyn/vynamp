<#
    37-tasks.ps1 - passo 37 da FASE 3

    Reescreve as TASKs no formato padroes/tasks.md, atribui CADA tarefa ao
    modulo dono (nao por epico) e distribui os arquivos nos repos. Cria o
    esqueleto das 23 specs de modulo com proposito, fronteira e fora de
    escopo ja preenchidos.

    Regras que este script respeita:
      - Nao inventa criterio de aceite. A coluna Then recebe o texto de
        'acceptance' do backlog LITERAL; Given/When ficam marcados PENDENTE.
      - acceptance_ready = false nas 191 de produto. Nenhuma TASK de produto
        e criada e nenhuma e promovida: a unica mudanca de lugar e a que a
        arvore normativa manda (134 saem do umbrella para o repo dono).
      - A fonte e o PROPRIO umbrella (tasks/backlog.json e tasks/items). Por
        isso o arquivamento roda ANTES da geracao: depois dela, tasks/items ja
        contem o texto novo e copiar seria guardar a reescrita como historico.
      - Nenhum git push, merge, tag ou PR. Nenhuma mudanca de visibilidade,
        protecao de branch, assinatura ou plano.

    Uso:  pwsh -NoProfile -File ./37-tasks.ps1
          ./37-tasks.ps1 -WhatIf
#>
[CmdletBinding(SupportsShouldProcess = $true)]
param(
    [string]$Raiz = 'D:/Projects/vynamp',
    [string]$Dono = 'jimmykedlyn',
    [string]$Backlog = 'D:/Projects/vynamp/tasks/backlog.json',
    [string]$ItensOrigem = 'D:/Projects/vynamp/tasks/items',
    [switch]$SemCommit
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
    param([string]$Path, [string]$Text)
    $dir = Split-Path $Path -Parent
    if ($dir -and -not (Test-Path $dir)) { New-Item -ItemType Directory -Path $dir -Force | Out-Null }
    if (-not $Text.EndsWith("`n")) { $Text = $Text + "`n" }
    [System.IO.File]::WriteAllText($Path, $Text, $Utf8SemBom)
}
function Add-Linha {
    # Idempotente pela coluna ORIGEM, que e a segunda celula da linha.
    # A primeira e a data: usa-la como chave fazia a primeira linha do dia engolir
    # todas as outras, e o indice ficava com 1 de 191 entradas sem avisar ninguem.
    param([string]$Path, [string]$Text)
    $dir = Split-Path $Path -Parent
    if ($dir -and -not (Test-Path $dir)) { New-Item -ItemType Directory -Path $dir -Force | Out-Null }
    $antigo = if (Test-Path $Path) { [System.IO.File]::ReadAllText($Path, [System.Text.Encoding]::UTF8) } else { '' }
    $chave = ($Text -split '\|')[2]
    if ($chave -and $chave.Trim() -and $antigo.Contains($chave.Trim())) { return }
    if ($antigo.Length -gt 0 -and -not $antigo.EndsWith("`n")) { $antigo = $antigo + "`n" }
    [System.IO.File]::WriteAllText($Path, $antigo + $Text + "`n", $Utf8SemBom)
}

function Linha-Indice {
    # SEIS colunas: e o formato real do _arquivo/INDEX.md desde o passo 30. O 30 e o 32
    # emitiam 5 e o cabecalho do arquivo tem 6 (a coluna Estado); uma linha com 5 celulas
    # ao lado de um cabecalho com 6 desalinha a tabela inteira na renderizacao.
    param([string]$Origem, [string]$Destino = '-', [string]$Motivo, [string]$Passo = '37', [string]$Estado = 'arquivado')
    $Motivo = $Motivo -replace '\|', '/'
    $Destino = $Destino -replace '\|', '/'
    return ('| {0} | `{1}` | `{2}` | {3} | {4} | {5} |' -f (Get-Date -Format 'yyyy-MM-dd'), $Origem, $Destino, $Motivo, $Passo, $Estado)
}

function Write-Passo { param($m) Write-Host "==> $m" -ForegroundColor Cyan }
function Write-Feito { param($m) Write-Host "    $m" -ForegroundColor DarkGray }
function Write-Aviso { param($m) Write-Host "    ! $m" -ForegroundColor Yellow }

function Invoke-Git {
    param([string[]]$Argumentos, [string]$Onde)
    $out = & git -C $Onde @Argumentos 2>&1
    if ($LASTEXITCODE -ne 0) {
        throw "git $($Argumentos -join ' ') falhou (exit $LASTEXITCODE) em ${Onde}:`n$(($out | Out-String).Trim())"
    }
    return (($out | Out-String).Trim())
}

# ---------------------------------------------------------------------------
# Catalogo de repos. 'vynamp' e o umbrella; os 8 demais sao modulos.
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

# Specs por modulo, de 01-arvore-final.md secao 2. O 'tipo' e 'lib' ou 'app'.
$Specs = @(
    @{ repo = 'vynamp-dsp';       tipo = 'lib'; arquivos = @('dsp') }
    @{ repo = 'vynamp-decode';    tipo = 'lib'; arquivos = @('decode', 'formatos', 'registro-capacidades') }
    @{ repo = 'vynamp-encode';    tipo = 'lib'; arquivos = @('encode', 'formatos') }
    @{ repo = 'vynamp-output';    tipo = 'lib'; arquivos = @('output', 'backends') }
    @{ repo = 'vynamp-engine';    tipo = 'lib'; arquivos = @('engine', 'pipeline', 'dsp', 'equalizadores', 'catalogo-dsp') }
    @{ repo = 'vynamp-player';    tipo = 'app'; arquivos = @('player', 'ui', 'biblioteca', 'streaming', 'temas', 'plugins') }
    @{ repo = 'vynamp-converter'; tipo = 'app'; arquivos = @('converter', 'lote') }
    @{ repo = 'vynamp-restore';   tipo = 'app'; arquivos = @('restore', 'ia') }
)

function Resolve-Checkout {
    param([string]$Nome, [string]$Caminho)
    if ($Nome -eq 'vynamp') {
        if (Test-Path (Join-Path $Raiz '.git')) { return $Raiz }
        return $null
    }
    $sub = Join-Path $Raiz $Caminho
    if (Test-Path (Join-Path $sub '.git')) { return $sub }
    $own = Join-Path 'D:/Projects' $Nome
    if (Test-Path (Join-Path $own '.git')) { return $own }
    return $null
}

# ---------------------------------------------------------------------------
# Atribuicao por tarefa. Fonte: 37-tasks.md secao 3. NAO derive de epico.
# Formato: epic = @{ modulo = @(numeros) }
# ---------------------------------------------------------------------------
$Atribuicao = [ordered]@{
    'E01' = [ordered]@{
        'vynamp-decode'    = @('01','04','07','08','09')
        'vynamp-output'    = @('02')
        'vynamp-engine'    = @('03','05','06','10')
    }
    'E02' = [ordered]@{ 'vynamp-player' = @('01') + (2..16 | ForEach-Object { '{0:d2}' -f $_ }) }
    'E03' = [ordered]@{ 'vynamp-player' = @('01') + (2..10 | ForEach-Object { '{0:d2}' -f $_ }) }
    'E04' = [ordered]@{
        'vynamp'        = @('10')
        'vynamp-output' = @('05','06','09')
        'vynamp-engine' = @('01','03','04','07','08')
        'vynamp-dsp'    = @('02')
    }
    'E05' = [ordered]@{ 'vynamp-player' = @('01') + (2..10 | ForEach-Object { '{0:d2}' -f $_ }) }
    'E06' = [ordered]@{ 'vynamp-player' = @('01') + (2..10 | ForEach-Object { '{0:d2}' -f $_ }) }
    'E07' = [ordered]@{ 'vynamp-player' = @('01') + (2..10 | ForEach-Object { '{0:d2}' -f $_ }) }
    'E08' = [ordered]@{
        'vynamp-player'    = @('03','04','05','06','07','08','09','10')
        'vynamp-engine'    = @('01')
        'vynamp-converter' = @('02')
    }
    'E09' = [ordered]@{ 'vynamp' = @('01') + (2..10 | ForEach-Object { '{0:d2}' -f $_ }) }
    'E10' = [ordered]@{ 'vynamp' = @('01') + (2..10 | ForEach-Object { '{0:d2}' -f $_ }) }
    'E11' = [ordered]@{ 'vynamp' = @('01') + (2..10 | ForEach-Object { '{0:d2}' -f $_ }) }
    'E12' = [ordered]@{
        'vynamp'        = @('03')
        'vynamp-player' = @('01','02','08','09')
        'vynamp-decode' = @('05','06')
        'vynamp-output' = @('04')
        'vynamp-engine' = @('07','10')
    }
    'E13' = [ordered]@{ 'vynamp-player' = @('01') + (2..10 | ForEach-Object { '{0:d2}' -f $_ }) }
    'E14' = [ordered]@{ 'vynamp' = @('01','02','03','04','05','06') }
    'E15' = [ordered]@{ 'vynamp' = @('01') + (2..10 | ForEach-Object { '{0:d2}' -f $_ }) }
    'E16' = [ordered]@{ 'vynamp-player' = @('01') + (2..10 | ForEach-Object { '{0:d2}' -f $_ }) }
    'E17' = [ordered]@{
        'vynamp'        = @('08','12','18','20')
        'vynamp-player' = @('06','09','10','11')
        'vynamp-decode' = @('01','02','03','04','05','13','14','15','16','17')
        'vynamp-output' = @('19')
        'vynamp-engine' = @('07')
    }
    'E18' = [ordered]@{ 'vynamp-converter' = @('02'); 'vynamp-restore' = @('01') }
    'E19' = [ordered]@{
        'vynamp'        = @('01','02','04','05','06')
        'vynamp-engine' = @('03','07')
    }
}

# ---------------------------------------------------------------------------
# Epicos que o AGENTS.md chama de alto risco, independente do campo 'risk'.
# ---------------------------------------------------------------------------
$AltoRisco = @('E01','E04','E12','E17-07','E17-08','E08-01','E11-02','E11-05',
               'E11-06','E11-07','E14-05','E15-06','E15-07','E15-08','E15-09',
               'E15-10','E19-06')

# ---------------------------------------------------------------------------
# Proposito, fronteira e fora de escopo de cada modulo, de 01-arvore-final.md
# secao 2. E o que o esqueleto de spec preenche; o resto fica para autoria.
# ---------------------------------------------------------------------------
$Fronteira = @{
    'vynamp-dsp' = @{
        proposito = 'Primitivas de processamento de amostras: resampling, dither, mapa de canais, mixer, ganho, fade e crossfade.'
        naoFaz    = 'Nao conhece arquivo, container, dispositivo, formato de midia nem qualquer tipo de dominio.'
        permitido = 'std; nenhuma dependencia de outro crate, nem opcional'
        proibido  = 'qualquer crate `vynamp-*`, inclusive o proprio engine; qualquer tipo de dominio'
        dependeDe = 'nada — esta lib e a base da pilha'
        dependidoPor = 'engine (obrigatorio); output e encode (opcionais, feature `resample`); restore'
        fora      = 'DSP musical (EQ, dynamics, reverb, tempo/pitch) — isso e do engine; formatos de arquivo — isso e do decode/encode'
    }
    'vynamp-decode' = @{
        proposito = 'Transformar bytes de arquivo de midia em amostras PCM, com precisao de bit e duracao corretas.'
        naoFaz    = 'Nao escreve tag, nao monta biblioteca, nao toca audio, nao converte taxa.'
        permitido = 'std; `vynamp-dsp` opcional atras de feature'
        proibido  = 'qualquer crate de dominio `vynamp-*`; qualquer tipo de dominio; escrever arquivo'
        dependeDe = '`vynamp-dsp` (opcional, feature `resample`)'
        dependidoPor = 'engine; encode (para transcodificar sem segundo parser)'
        fora      = 'metadata, CUE e capas (C12: a spec anunciou, o crate nao existe — fica registrado como limitacao); encoders; saida de audio'
    }
    'vynamp-encode' = @{
        proposito = 'Codificar PCM em arquivo de midia, com muxing e metadados de encoder corretos (priming/padding).'
        naoFaz    = 'Nao decodifica, nao toca audio, nao gerencia fila de conversao.'
        permitido = 'std; `vynamp-dsp` opcional atras de feature `resample`'
        proibido  = 'qualquer crate de dominio `vynamp-*`; qualquer tipo de dominio'
        dependeDe = '`vynamp-dsp` (opcional, feature `resample`)'
        dependidoPor = 'engine; converter'
        fora      = 'AAC entra sem ressalva; MP3 so quando o encoder estiver na build (C7). UI de conversao e fila persistida sao do converter'
    }
    'vynamp-output' = @{
        proposito = 'Entregar amostras ao dispositivo de audio com taxa, layout, latencia e exclusividade corretos.'
        naoFaz    = 'Nao decodifica, nao processa DSP musical, nao conhece arquivo.'
        permitido = 'std; `vynamp-dsp` opcional atras de feature `resample`'
        proibido  = 'qualquer crate de dominio `vynamp-*`; qualquer tipo de dominio (exceto `PathProperties`, que e deste modulo e so de primitivos)'
        dependeDe = '`vynamp-dsp` (opcional, feature `resample`)'
        dependidoPor = 'engine'
        fora      = 'Selecao de faixa, volume de aplicacao, EQ; casting e AirPlay sao do player (E12-09)'
    }
    'vynamp-engine' = @{
        proposito = 'Integrar decode, dsp, output e encode num grafo de reproducao com estado, comandos e grafo DSP.'
        naoFaz    = 'Nao tem UI, nao gerencia biblioteca, nao abre dispositivo por conta propria sem o output.'
        permitido = 'decode, encode, output, dsp (as quatro, obrigatorias)'
        proibido  = 'qualquer app; qualquer crate de dominio externo a estas quatro libs'
        dependeDe = 'decode, encode, output, dsp (as quatro)'
        dependidoPor = 'player, converter, restore'
        fora      = 'Vinyl/Tube e VST3 entram como modulos internos deste grafo (E19-07, E12-07). Jobs persistidos (E08-01) entram como infraestrutura nao-audio, decisao registrada'
    }
    'vynamp-player' = @{
        proposito = 'App desktop e Android: UI, biblioteca, streaming, temas, plugins e integracao com o sistema operacional.'
        naoFaz    = 'Nao converte em lote nem restaura audio — delega aos apps proprios.'
        permitido = 'vynamp-engine (unico crate do projeto); crates de UI e plataforma de terceiros'
        proibido  = 'acessar decode/output/encode/dsp direto; qualquer dependencia de converter ou restore'
        dependeDe = '`vynamp-engine` (unico crate do projeto)'
        dependidoPor = 'ninguem'
        fora      = 'Conversao em lote, restauracao por IA e servidor de streaming (E09): os dois primeiros sao apps separados, o terceiro nao tem repo'
    }
    'vynamp-converter' = @{
        proposito = 'App desktop de conversao em lote, com fila persistida e preview.'
        naoFaz    = 'Nao toca audio, nao restaura, nao gerencia biblioteca.'
        permitido = 'vynamp-engine, vynamp-encode'
        proibido  = 'acessar decode/output/dsp direto quando houver API no engine; depender do player ou do restore'
        dependeDe = '`vynamp-engine`, `vynamp-encode`'
        dependidoPor = 'ninguem'
        fora      = 'Codificadores proprios: a unica fonte e a lib `vynamp-encode`. Jobs persistidos vem do engine (E08-01)'
    }
    'vynamp-restore' = @{
        proposito = 'App desktop, em processo separado (ADR-013 §1), de restauracao de audio com diagnostico de defeitos.'
        naoFaz    = 'Nao converte em lote, nao toca audio, nao embute modelo de IA no instalador.'
        permitido = 'vynamp-engine, vynamp-dsp'
        proibido  = 'depender do player ou do converter; baixar modelo sem clique explicito do usuario'
        dependeDe = '`vynamp-engine`, `vynamp-dsp`'
        dependidoPor = 'ninguem'
        fora      = 'Modelos de IA: download opt-in com SHA-256 e selecao no painel (ADR-015); nenhum modelo no instalador'
    }
}

# ---------------------------------------------------------------------------
# Pre-checagem
# ---------------------------------------------------------------------------
Write-Passo 'pre-checagem'

if (-not (Test-Path $Raiz)) { throw "Umbrella nao encontrado: $Raiz. Rode os passos 31 e 35 antes." }
if (-not (Test-Path $Backlog)) { throw "backlog.json nao encontrado: $Backlog" }
if (-not (Test-Path $ItensOrigem)) { throw "tasks/items nao encontrado: $ItensOrigem" }

$dados = (Read-Utf8 $Backlog) | ConvertFrom-Json
$tarefas = @($dados.tasks)
Write-Feito "$($tarefas.Count) tarefa(s) lida(s) do backlog"

# ---------------------------------------------------------------------------
# 1. Valida a atribuicao antes de escrever qualquer coisa.
# ---------------------------------------------------------------------------
Write-Passo '1. validando a atribuicao por tarefa'

$porId = @{}
foreach ($t in $tarefas) { $porId[$t.id] = $t }

$mapaDono = @{}
$erros = @()

foreach ($epico in $Atribuicao.Keys) {
    $esperadas = @($tarefas | Where-Object { $_.epic -eq $epico })
    $atribuidas = @()
    foreach ($mod in $Atribuicao[$epico].Keys) {
        foreach ($num in $Atribuicao[$epico][$mod]) {
            $id = "TASK-$epico-$num"
            if (-not $porId.ContainsKey($id)) {
                $erros += "$id atribuida a $mod mas nao existe no backlog"
                continue
            }
            if ($mapaDono.ContainsKey($id)) {
                $erros += "$id atribuida duas vezes: $($mapaDono[$id]) e $mod"
                continue
            }
            $mapaDono[$id] = $mod
            $atribuidas += $id
        }
    }
    foreach ($t in $esperadas) {
        if (-not $mapaDono.ContainsKey($t.id)) { $erros += "$($t.id) do epic $epico ficou sem dono" }
    }
    if ($atribuidas.Count -ne $esperadas.Count) {
        $erros += "epic ${epico}: $($atribuidas.Count) atribuidas, $($esperadas.Count) no backlog"
    }
}

$semEpicoMapeado = @($tarefas | Where-Object { -not $Atribuicao.Contains($_.epic) })
foreach ($t in $semEpicoMapeado) { $erros += "epic $($t.epic) nao esta em `$Atribuicao ($($t.id))" }

if ($mapaDono.Count -ne 191) { $erros += "total atribuido = $($mapaDono.Count); esperado 191" }

if ($erros.Count -gt 0) {
    foreach ($e in $erros) { Write-Aviso $e }
    throw "atribuicao inconsistente ($($erros.Count) erro(s)). Nada foi escrito."
}

$contagem = @{}
foreach ($mod in $mapaDono.Values) {
    if (-not $contagem.ContainsKey($mod)) { $contagem[$mod] = 0 }
    $contagem[$mod]++
}
Write-Feito "191 tarefas atribuidas:"
foreach ($mod in ($contagem.Keys | Sort-Object)) { Write-Feito ("  {0,-18} {1}" -f $mod, $contagem[$mod]) }

# ---------------------------------------------------------------------------
# Localiza os checkouts. Sem o repo, a tarefa nao pode ir para o lugar certo.
# ---------------------------------------------------------------------------
$checkouts = @{}
foreach ($m in $Modulos) {
    $p = Resolve-Checkout -Nome $m.nome -Caminho $m.caminho
    if ($null -eq $p) { $erros += "$($m.nome): checkout nao encontrado (submodulo em $Raiz nem D:/Projects/$($m.nome))" }
    else { $checkouts[$m.nome] = $p }
}
$umbrella = Resolve-Checkout -Nome 'vynamp' -Caminho $null
if ($null -eq $umbrella) { $erros += "umbrella: $Raiz nao e um repo git" }
else { $checkouts['vynamp'] = $umbrella }

if ($erros.Count -gt 0) {
    foreach ($e in $erros) { Write-Aviso $e }
    throw "repos ausentes. Rode o passo 35 antes. Nada foi escrito."
}
Write-Feito "$($checkouts.Count) checkout(s) localizado(s)"
Write-Host ''

# ---------------------------------------------------------------------------
# 7. Arquiva os arquivos que SAEM do umbrella. Nada e apagado: vai para
#    _arquivo/repo/tasks/items/ com destino e motivo em _arquivo/INDEX.md.
#
#    ATENCAO a ordem: esta etapa tem de rodar ANTES da geracao (secao 3). A
#    fonte agora e o PROPRIO umbrella -- o parametro -ItensOrigem aponta para
#    D:/Projects/vynamp/tasks/items --, entao arquivar depois de gerar copiaria
#    o texto NOVO e perderia o original que o _arquivo existe para guardar. Por
#    isso a DEFINICAO fica aqui, acima das outras funcoes operacionais, e o
#    chamador a invoca no passo 1.5, antes de qualquer escrita.
#
#    Arquivam-se os 134 que mudam de repo. As 57 tarefas do umbrella FICAM no
#    lugar: nao sao historico, continuam sendo a fonte viva.
# ---------------------------------------------------------------------------
function Invoke-Arquivamento {
    param([string]$ItensOrigem, [string]$Umbrella, [hashtable]$MapaDono)

    Write-Passo '1.5 arquivando os tasks/items que saem do umbrella'

    $arquivo = Join-Path $Umbrella '_arquivo'
    $indice = Join-Path $arquivo 'INDEX.md'
    $arquivados = 0

    # O passo 30 cria o INDEX.md com cabecalho. Se ele nao existir, o 37 o cria no
    # MESMO formato: SEIS colunas, contando Estado. Um indice com 5 celulas ao lado
    # de um cabecalho com 6 desalinha a tabela na renderizacao.
    if (-not (Test-Path $indice)) {
        $cabecalho = @(
            '# _arquivo — indice de arquivamento', '',
            'Todo arquivo movido para ca tem uma linha. A exclusao definitiva so acontece',
            'no passo 38, sob confirmacao explicita do mantenedor.', '',
            '| Data | Origem | Destino | Motivo | Passo | Estado |',
            '|---|---|---|---|---|---|'
        ) -join "`n"
        New-Item -ItemType Directory -Path $arquivo -Force | Out-Null
        [System.IO.File]::WriteAllText($indice, $cabecalho + "`n", $Utf8SemBom)
        Write-Aviso 'INDEX.md ausente; criado no formato do passo 30'
    }

    $originais = @(Get-ChildItem -Path $ItensOrigem -Filter 'TASK-*.md' -File)
    Write-Feito "$($originais.Count) arquivo(s) encontrado(s) em tasks/items"

    # Os que ficam: dono 'vynamp'. Os que saem: todos os demais.
    $saem = @($originais | Where-Object {
            $MapaDono.ContainsKey($_.BaseName) -and $MapaDono[$_.BaseName] -ne 'vynamp'
        })
    Write-Feito "$($saem.Count) saem do umbrella; $($originais.Count - $saem.Count) ficam"

    foreach ($o in $saem) {
        $mod = $MapaDono[$o.BaseName]
        $motivo = "reescrito no formato padroes/tasks.md e movido para $mod (passo 37)"
        $destinoRel = "repo/tasks/items/$($o.Name)"

        if (-not $PSCmdlet.ShouldProcess($o.FullName, 'arquivar com destino e motivo')) { continue }

        # Copia para o _arquivo do umbrella. Copiar, e nao mover: o original so
        # desaparece de tasks/items quando o commit troca o conteudo do repo.
        $destino = Join-Path $arquivo $destinoRel
        $dir = Split-Path $destino -Parent
        if (-not (Test-Path $dir)) { New-Item -ItemType Directory -Path $dir -Force | Out-Null }
        Copy-Item -Path $o.FullName -Destination $destino -Force
        Add-Linha -Path $indice -Text (Linha-Indice -Origem "tasks/items/$($o.Name)" -Destino $destinoRel -Motivo $motivo -Estado 'arquivado')
        $arquivados++
    }
    Write-Feito "$arquivados arquivo(s) copiado(s) para _arquivo e indexado(s)"

    # NAO se arquivam tasks/epics: a fonte e o proprio umbrella e a secao 5 os
    # reescreve aqui mesmo. Copiar depois de reescrever guardaria o texto novo
    # sob o rotulo de 'superseded', que e o oposto do que a linha diz.
    return $arquivados
}

# ---------------------------------------------------------------------------
# 1.5 Arquivamento. ANTES de qualquer escrita: a fonte e o umbrella, e depois
#     da secao 3 tasks/items ja contem o texto reescrito (ver o comentario da
#     funcao na secao 7).
# ---------------------------------------------------------------------------
$arquivados = Invoke-Arquivamento -ItensOrigem $ItensOrigem -Umbrella $umbrella -MapaDono $mapaDono
Write-Host ''

# ---------------------------------------------------------------------------
# 2. Esqueleto das specs de modulo. Vem ANTES das TASKs: a TASK aponta para
#    specs/<m>.md secao 5, e o link precisa existir para check-docs passar.
# ---------------------------------------------------------------------------
Write-Passo '2. criando o esqueleto das specs de modulo'

$dataHoje = (Get-Date).ToString('yyyy-MM-dd')
$specsCriadas = 0
$specsPuladas = 0

foreach ($s in $Specs) {
    $repo = $s.repo
    $f = $Fronteira[$repo]
    $caminho = Join-Path $checkouts[$repo] 'specs'

    foreach ($nome in $s.arquivos) {
        $arq = Join-Path $caminho "$nome.md"
        if (Test-Path $arq) { Write-Aviso "$repo/specs/$nome.md ja existe; pulado"; $specsPuladas++; continue }

        # Identificador de modulo para os IDs: dsp, decode, engine, player...
        $mod = $repo -replace '^vynamp-', ''
        $modId = ($mod.ToUpper() -replace '-', '_')

        $fronteiraEspecifica = if ($nome -eq $s.arquivos[0]) {
            @"
| Permitido | Proibido |
|---|---|
| $($f.permitido) | $($f.proibido) |
"@
        } else {
            @"
| Permitido | Proibido |
|---|---|
| herda a fronteira de ``specs/$($s.arquivos[0]).md`` | herda as proibicoes de ``specs/$($s.arquivos[0]).md`` |
"@
        }

        $txt = @"
# Spec — $repo · $nome

| Campo | Valor |
|---|---|
| Módulo | ``$repo`` |
| Crate(s) | a definir na autoria |
| Tipo | $($s.tipo) |
| Versão da spec | 0.1.0 |
| Data | $dataHoje |
| Estado | esqueleto — §3 a §10 pendentes de autoria |
| Dono | a definir |
| Depende de | $($f.dependeDe) |
| É dependido por | $($f.dependidoPor) |

> **Esqueleto gerado pelo passo 37.** §1, §2 e §7 foram decididos em
> ``01-arvore-final.md`` §2 e valem como norma. §3 a §6 e §8 a §10 exigem projeto de
> API e **não** foram escritos: inventá-los aqui seria fabricar a autoridade que o SDD
> exige. Estado só passa a ``aprovada`` com §5 preenchida.

## 1. Propósito

$($f.proposito)

**Não faz:** $($f.naoFaz)

## 2. Fronteira

$fronteiraEspecifica

## 3. API pública

PENDENTE — projeto de API. Só o que estiver aqui é contrato.

## 4. Requisitos

PENDENTE — cada requisito com ID estável ``$modId-REQ-nn`` e REQ de produto,
rastreável até uma TASK e um teste.

## 5. Critérios de aceite

PENDENTE — Given/When/Then, coerente de fora. Enquanto esta seção estiver vazia,
nenhuma TASK deste módulo pode sair de ``planned``.

## 6. Erros e cancelamento

PENDENTE — erro é dado, não ``panic!``. Cancelamento é explícito, não timeout.

## 7. Fora de escopo

$($f.fora)

## 8. Dependências

Depende de: $($f.dependeDe)

É dependido por: $($f.dependidoPor)

## 9. Testes e fixtures

PENDENTE — oráculo numérico onde couber. Teste que depende de arquivo grande vive
atrás de ``#[ignore]`` com motivo escrito e variável de ambiente documentada.

## 10. Limitações conhecidas

PENDENTE — limitação declarada não reprova a entrega; limitação omitida reprova.
"@

        if ($PSCmdlet.ShouldProcess("$repo/specs/$nome.md", 'criar esqueleto de spec')) {
            Write-Utf8 -Path $arq -Text $txt
            $specsCriadas++
        }
    }
}
Write-Feito "$specsCriadas spec(s) criada(s), $specsPuladas pulada(s)"

# specs/README.md por repo de modulo
foreach ($s in $Specs) {
    $repo = $s.repo
    $mod = $repo -replace '^vynamp-', ''
    $arq = Join-Path (Join-Path $checkouts[$repo] 'specs') 'README.md'
    if (Test-Path $arq) { continue }
    $linhas = @()
    foreach ($nome in $s.arquivos) { $linhas += "- [``$nome.md``]($nome.md)" }
    $txt = @"
# specs — $repo

Specs deste módulo. Normativas: o código segue a spec, não o contrário.

$($linhas -join "`n")

O estado de cada uma está no cabeçalho. ``esqueleto`` não autoriza implementação.
"@
    if ($PSCmdlet.ShouldProcess("$repo/specs/README.md", 'criar indice de specs')) {
        Write-Utf8 -Path $arq -Text $txt
    }
}
Write-Host ''

# ---------------------------------------------------------------------------
# 3. Gera as 191 TASKs. A coluna Then e o 'acceptance' do backlog, literal.
# ---------------------------------------------------------------------------
Write-Passo '3. gerando as TASKs'

# Inverso de dependencies: 'Bloqueia'
$bloqueia = @{}
foreach ($t in $tarefas) {
    foreach ($d in $t.dependencies) {
        if (-not $bloqueia.ContainsKey($d)) { $bloqueia[$d] = @() }
        $bloqueia[$d] += $t.id
    }
}

# Linha de Butdo: 'vynamp' e o umbrella (repo sem sufixo de modulo)
function Get-RepoDe { param([string]$mod) return $(if ($mod -eq 'vynamp') { 'vynamp' } else { $mod }) }

$DoD = @'
- [ ] Todos os critérios de aceite com evidência
- [ ] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` limpos
- [ ] `cargo test` verde; testes ignorados listados com motivo
- [ ] Spec do módulo atualizada se a implementação revelou algo novo
- [ ] `backlog.json` com o estado correto
- [ ] Revisão feita por contexto limpo, não pelo implementador
- [ ] PR aberto com template preenchido; CI Gate verde no SHA final
'@

$geradas = 0
$porRepo = @{}

foreach ($t in ($tarefas | Sort-Object id)) {
    $mod = $mapaDono[$t.id]
    $repo = Get-RepoDe $mod
    $raizRepo = $checkouts[$repo]
    $arq = Join-Path (Join-Path (Join-Path $raizRepo 'tasks') 'items') "$($t.id).md"

    # Then: 'acceptance' literal, separado em ';' -> uma linha por clausula.
    # NAO reescrevemos o texto. Se a clausula tiver ';' interno, ela continua unica.
    $clausulas = @($t.acceptance -split ';\s*' | Where-Object { $_.Trim() })
    $linhasAC = @()
    for ($i = 0; $i -lt $clausulas.Count; $i++) {
        $n = $i + 1
        $linhasAC += "| ``$($t.id)-AC-$n`` | PENDENTE | PENDENTE | $($clausulas[$i].Trim()) |"
    }

    $deps = if (@($t.dependencies).Count -gt 0) { (@($t.dependencies) | ForEach-Object { "``$_``" }) -join ', ' } else { 'nenhuma' }
    $bloq = if ($bloqueia.ContainsKey($t.id)) { (@($bloqueia[$t.id]) | ForEach-Object { "``$_``" }) -join ', ' } else { 'nenhuma' }

    $altoRisco = $AltoRisco -contains $t.epic -or $AltoRisco -contains $t.id
    $modelo = if ($altoRisco) { 'Opus — áudio/RT, concorrência, segurança ou release' } else { 'Sonnet' }

    $modSpec = $mod -replace '^vynamp-', ''
    $specPrincipal = if ($Fronteira.ContainsKey($mod)) { @($Specs | Where-Object { $_.repo -eq $mod } | Select-Object -First 1).arquivos[0] } else { $null }
    $critSpec = if ($specPrincipal) { "pendente — ``specs/$specPrincipal.md`` §5" } else { 'n/a — TASK transversal do umbrella; o critério vive na própria TASK' }

    $req = (@($t.requirements) | ForEach-Object { "``$_``" }) -join ', '

    $txt = @"
# $($t.id) — $($t.title)

> Gerado pelo passo 37 a partir de ``tasks/backlog.json``. Os campos de estado vêm do
> backlog e não devem ser editados aqui. Critério de aceite: **PENDENTE** de autoria —
> a coluna ``Then`` é o requisito aprovado, copiado literal; ``Given`` e ``When`` ainda
> não existem.

| Campo | Valor |
|---|---|
| ID | ``$($t.id)`` |
| Módulo dono | ``$mod`` |
| Repo | ``$Dono/$repo`` |
| REQ coberto | $req |
| Critérios da spec | $critSpec |
| Fase | $($t.phase) |
| Prioridade | $($t.priority) |
| Estado | $($t.status) |
| Depende de | $deps |
| Bloqueia | $bloq |
| Modelo executor | $modelo |
| Esforço | $($t.estimate_days.min)–$($t.estimate_days.max) dias úteis |
| Risco | $($t.risk) |

## Entrega

$($t.deliverable)

## Boundary

<!-- PENDENTE: o que esta TASK não faz, mesmo parecendo relacionado. -->

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (``cargo xtask repo worktree task-$($t.id.ToLower() -replace '^task-','')-claude-<slug>``)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
$($linhasAC -join "`n")

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | ``cargo test`` no repo ``$repo`` | verde no CI Gate |
| Oráculo independente | PENDENTE | comparação numérica, não "parece certo" |
| Hardware | PENDENTE | ausência é limitação registrada, não aprovação |

## DoD — pronto para review

$DoD
## Notas de execução

Preenchido durante a execução, não antes.
"@

    if ($PSCmdlet.ShouldProcess("$repo/tasks/items/$($t.id).md", 'gerar TASK')) {
        Write-Utf8 -Path $arq -Text $txt
    }
    $geradas++
    if (-not $porRepo.ContainsKey($repo)) { $porRepo[$repo] = 0 }
    $porRepo[$repo]++
}
Write-Feito "$geradas TASK(s) de produto gerada(s)"

# As que sairam do umbrella: o arquivo novo esta no repo dono e o antigo ja esta
# em _arquivo. Deixar a copia velha em tasks/items faria o gate ler o documento
# errado -- o check-docs exige indice para todo .md e o backlog aponta para o
# repo dono, entao a copia orfa apareceria como arquivo sem indice.
$removidas = 0
foreach ($t in $tarefas) {
    if ($mapaDono[$t.id] -eq 'vynamp') { continue }
    $velho = Join-Path $ItensOrigem "$($t.id).md"
    if (-not (Test-Path $velho)) { continue }
    if (-not $PSCmdlet.ShouldProcess("tasks/items/$($t.id).md", 'remover do umbrella (copia em _arquivo)')) { continue }
    Remove-Item -Path $velho -Force
    $removidas++
}
Write-Feito "$removidas TASK(s) removida(s) do umbrella; $((@($tarefas | Where-Object { $mapaDono[$_.id] -eq 'vynamp' })).Count) ficam"
Write-Host ''

# ---------------------------------------------------------------------------
# 4. As duas TASKs de engenharia (D-37-13). Escritas a mao, nao derivadas:
#    TASK-CI-01 foi prometida em 34-ci.md secao 7; TASK-CI-02 e o portao de
#    acceptance_ready. Ambas com criterio em Given/When/Then de verdade.
# ---------------------------------------------------------------------------
Write-Passo '4. emitindo TASK-CI-01 e TASK-CI-02'

$Ci01 = @'
# TASK-CI-01 — Reescrever delivery() e github_actions() para a árvore do umbrella

> Originada de `34-ci.md` §7: "Isso vira **TASK-CI-01** no passo 37". Enquanto ela
> não fechar, `cargo xtask check-delivery` fica vermelho por motivo conhecido.

| Campo | Valor |
|---|---|
| ID | `TASK-CI-01` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | n/a — engenharia, não requisito de produto |
| Critérios da spec | n/a — TASK transversal do umbrella |
| Fase | M1 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | passo 34 (concluído) |
| Bloqueia | nenhuma |
| Modelo executor | Sonnet |
| Esforço | 2–4 dias úteis |
| Risco | médio |

## Entrega

`xtask/src/validate.rs` com `delivery()` e `github_actions()` reescritos para a árvore
do umbrella: validam spec, `xtask`, pins de submódulo e regras de arquitetura, e nada
de empacotamento. `cargo xtask check-delivery` verde com os submódulos do passo 31.

## Boundary

Não reescreve o job de CI (isso é `34-ci.ps1`) nem cria `check-tasks` (TASK-CI-02).
Não toca `check-docs`, que já foi corrigido no passo 34.

## Antes de começar

- [ ] Passo 34 executado e `cargo xtask check-docs` verde
- [ ] Worktree própria criada
- [ ] `cargo test --package xtask` verde antes da mudança (baseline)

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-CI-01-AC-1` | o umbrella sem `docs/` nem `packaging/` | `cargo xtask check-delivery` roda | valida spec, `xtask`, pins de submódulo e regras de arquitetura; nenhuma checagem de empacotamento |
| `TASK-CI-01-AC-2` | os fixtures de `validate.rs:587-730` reescritos | `cargo test --package xtask` roda | os 3 testes de workflow passam com o `ci.yml` do passo 34 |
| `TASK-CI-01-AC-3` | os submódulos do passo 31 no estado semeado | `cargo xtask check-delivery` roda | verde, sem falso positivo de pin |
| `TASK-CI-01-AC-4` | um workflow com `uses:` sem SHA | `check-delivery` roda | reprova e nomeia o arquivo e a linha |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test --package xtask` | verde no CI Gate |
| Gate real | `cargo xtask check-delivery` | verde |
| Oráculo independente | fixture adversária com `uses:` sem SHA | reprovação com arquivo e linha |

## DoD — pronto para review

- [ ] Todos os critérios de aceite com evidência
- [ ] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` limpos
- [ ] `cargo test` verde; testes ignorados listados com motivo
- [ ] Spec do módulo atualizada se a implementação revelou algo novo
- [ ] `backlog.json` com o estado correto
- [ ] Revisão feita por contexto limpo, não pelo implementador
- [ ] PR aberto com template preenchido; CI Gate verde no SHA final

## Notas de execução

Preenchido durante a execução, não antes.
'@

$Ci02 = @'
# TASK-CI-02 — Adicionar o job check-tasks ao CI do umbrella

> Criada pelo passo 37 (D-37-13). É o que dá consequência mecânica a
> `acceptance_ready: false`: sem ela o campo é decoração e o SDD volta a ser
> declaração.

| Campo | Valor |
|---|---|
| ID | `TASK-CI-02` |
| Módulo dono | `vynamp` |
| Repo | `jimmykedlyn/vynamp` |
| REQ coberto | n/a — engenharia, não requisito de produto |
| Critérios da spec | n/a — TASK transversal do umbrella |
| Fase | M1 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | passo 37 (esta TASK é emitida por ele) |
| Bloqueia | nenhuma |
| Modelo executor | Sonnet |
| Esforço | 1–2 dias úteis |
| Risco | médio |

## Entrega

Job `check-tasks` no `.github/workflows/ci.yml` do umbrella e o comando `cargo xtask check-tasks` que o alimenta: valida que toda TASK de produto tem `Module dono` de um único módulo, REQ, critério em Given/When/Then sem `PENDENTE`, e que nenhuma TASK está fora de `planned` com `acceptance_ready: false`. Entra em `CI Gate`.

## Boundary

Não escreve critério de aceite, não reescreve TASK, não decide dono. Só reprova o que
não satisfaz a regra e **nomeia** cada infração.

## Antes de começar

- [ ] Passo 37 executado e as 193 TASKs no lugar
- [ ] Worktree própria criada
- [ ] `cargo xtask check-docs` verde

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-CI-02-AC-1` | uma TASK de produto com `Given` = PENDENTE | `cargo xtask check-tasks` roda | reprova e imprime o ID da TASK e a coluna que falta |
| `TASK-CI-02-AC-2` | uma TASK com `status: in_progress` e `acceptance_ready: false` no backlog | `check-tasks` roda | reprova e lista o ID |
| `TASK-CI-02-AC-3` | uma TASK com dois módulos donos ou com módulo inexistente | `check-tasks` roda | reprova e nomeia o arquivo |
| `TASK-CI-02-AC-4` | o repositório no estado do passo 37 | `check-tasks` roda | reprova **nomeando as 5 `in_progress`** e nenhuma outra — o vermelho esperado, documentado em `37-tasks.md` §8 |
| `TASK-CI-02-AC-5` | as 5 `in_progress` com critério escrito e `acceptance_ready: true` | `check-tasks` roda | verde |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test --package xtask` com 4 fixtures adversárias | verde no CI Gate |
| Gate real | `cargo xtask check-tasks` | reprova nomeando exatamente as 5 de AC-4 |
| CI | job `check-tasks` no `ci.yml` | presente e exigido pelo `CI Gate` |

## DoD — pronto para review

- [ ] Todos os critérios de aceite com evidência
- [ ] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` limpos
- [ ] `cargo test` verde; testes ignorados listados com motivo
- [ ] Spec do módulo atualizada se a implementação revelou algo novo
- [ ] `backlog.json` com o estado correto
- [ ] Revisão feita por contexto limpo, não pelo implementador
- [ ] PR aberto com template preenchido; CI Gate verde no SHA final

## Notas de execução

Preenchido durante a execução, não antes.
'@

foreach ($par in @(@{ id = 'TASK-CI-01'; txt = $Ci01 }, @{ id = 'TASK-CI-02'; txt = $Ci02 })) {
    $arq = Join-Path (Join-Path (Join-Path $checkouts['vynamp'] 'tasks') 'items') "$($par.id).md"
    if ($PSCmdlet.ShouldProcess("tasks/items/$($par.id).md", 'emitir TASK de engenharia')) {
        Write-Utf8 -Path $arq -Text $par.txt
    }
    Write-Feito "$($par.id): emitida"
}
$porRepo['vynamp'] = $porRepo['vynamp'] + 2
Write-Host ''

# ---------------------------------------------------------------------------
# 5. Indices: epicos no umbrella, README por repo. D-37-11: o indice de epico
#    fica SO no umbrella; duplicar criaria versoes divergentes.
# ---------------------------------------------------------------------------
Write-Passo '5. escrevendo indices'

$epicos = @($tarefas | ForEach-Object { $_.epic } | Sort-Object -Unique)

foreach ($epico in $epicos) {
    $doEpico = @($tarefas | Where-Object { $_.epic -eq $epico } | Sort-Object id)
    $linhas = @()
    foreach ($t in $doEpico) {
        $mod = $mapaDono[$t.id]
        # O ID e o titulo viram link para onde o arquivo ESTA. Antes do passo 37
        # todo ID era um link relativo para ../items/; depois dele 134 vivem no
        # repo dono, e o link relativo apontaria para um arquivo que nao existe
        # mais. Como o .gitmodules e um submodulo do mesmo dono, o caminho
        # relativo ../libs/<m>/tasks/items/... resolve nos dois: na estacao
        # (submodulo populado) e no runner (checkout com submodules: recursive).
        $alvo = if ($mod -eq 'vynamp') { "../items/$($t.id).md" }
                 else { "../../$(($Modulos | Where-Object { $_.nome -eq $mod }).caminho)/tasks/items/$($t.id).md" }
        $linhas += "| [``$($t.id)``]($alvo) | $($t.title) | ``$mod`` | $($t.phase) | $($t.status) |"
    }
    $txt = @"
# Épico $epico

$($doEpico.Count) tarefa(s). O status é o do ``tasks/backlog.json`` do umbrella; esta
tabela é um índice e não uma fonte.

| ID | Título | Módulo dono | Fase | Estado |
|---|---|---|---|---|
$($linhas -join "`n")

## Onde cada TASK vive

| Módulo dono | Repo | Pasta |
|---|---|---|
$((($doEpico | ForEach-Object { $mapaDono[$_.id] } | Sort-Object -Unique) | ForEach-Object {
    $r = if ($_ -eq 'vynamp') { 'vynamp' } else { $_ }
    "| ``$_`` | ``$Dono/$r`` | ``tasks/items/`` |"
}) -join "`n")
"@
    $arq = Join-Path (Join-Path $checkouts['vynamp'] 'tasks/epics') "$epico.md"
    if ($PSCmdlet.ShouldProcess("tasks/epics/$epico.md", 'escrever indice de epico')) {
        Write-Utf8 -Path $arq -Text $txt
    }
}
Write-Feito "$($epicos.Count) indice(s) de epico no umbrella"

# ---------------------------------------------------------------------------
# 5.2 Indices e links do umbrella. Sair do umbrella tem DUAS consequencias que
#     o indice de epico nao cobre:
#
#       a) o INDEX.md lista todo .md do umbrella (check-docs exige), e 134 dos
#          que ele lista deixam de existir aqui;
#       b) outros documentos apontam para tasks/items/TASK-*.md por link
#          relativo -- 207 na matriz de requisitos, mais ADRs, design, evidence
#          e referencia. `check-docs` verifica cada link com exists() estrito,
#          entao o link quebrado reprova o gate.
#
#     A escolha e reescrever, nao afrouxar o gate: com `submodules: recursive`
#     no job `specs` o caminho novo resolve tanto na estacao quanto no runner,
#     e o gate continua provando que o link leva a algum lugar.
# ---------------------------------------------------------------------------

function Get-CaminhoModulo {
    param([string]$mod)
    $m = @($Modulos | Where-Object { $_.nome -eq $mod })
    if ($m.Count -eq 0) { return $null }
    return $m[0].caminho
}

# Onde CADA TASK esta depois deste passo, relativo a raiz do umbrella.
$alvoTask = @{}
foreach ($t in $tarefas) {
    $mod = $mapaDono[$t.id]
    $alvoTask[$t.id] = if ($mod -eq 'vynamp') { "tasks/items/$($t.id).md" }
    else { "$(Get-CaminhoModulo $mod)/tasks/items/$($t.id).md" }
}
foreach ($id in @('TASK-CI-01', 'TASK-CI-02')) { $alvoTask[$id] = "tasks/items/$id.md" }

# (a) INDEX.md: sai a linha de toda TASK que deixou de viver aqui.
$indiceUmbrella = Join-Path $umbrella 'INDEX.md'
$indiceTexto = Read-Utf8 $indiceUmbrella
$linhasIndice = @($indiceTexto -split "`n")
$podadas = 0
$mantidas = @()
foreach ($l in $linhasIndice) {
    $mm = [regex]::Match($l, '^- \[TASK-[A-Z0-9-]+[^\]]*\]\(tasks/items/(TASK-[A-Z0-9-]+)\.md\)')
    if ($mm.Success -and $alvoTask.ContainsKey($mm.Groups[1].Value) -and
        $alvoTask[$mm.Groups[1].Value] -ne "tasks/items/$($mm.Groups[1].Value).md") {
        $podadas++
        continue
    }
    $mantidas += $l
}
if ($podadas -gt 0) {
    $novoIndice = ($mantidas -join "`n")
    # Cada entrada e separada por uma linha em branco. Tirar a entrada sem
    # recomprimir deixaria pilhas de linhas em branco onde havia 134 seguidas.
    $novoIndice = [regex]::Replace($novoIndice, '(\r?\n){3,}', "`n`n")
    # A variavel carrega o texto NOVO aconteca o que acontecer com a escrita. A
    # proxima etapa (as TASKs de engenharia) precisa do indice ja podado, e
    # reler do disco sob -WhatIf devolveria o antigo -- o indice ganharia as duas
    # linhas por cima das 134 que o ensaio fingiu remover.
    $indiceTexto = $novoIndice
    if ($PSCmdlet.ShouldProcess('INDEX.md', 'remover as linhas das TASKs que sairam do umbrella')) {
        Write-Utf8 -Path $indiceUmbrella -Text $indiceTexto
    }
}
Write-Feito "$podadas linha(s) removida(s) do INDEX.md"

# (b) INDEX.md ganha as duas TASKs de engenharia, que nenhum passo anterior
#     podia listar porque ainda nao existiam.
foreach ($ci in @(
        @{ id = 'TASK-CI-01'; titulo = 'Reescrever delivery() e github_actions() para a árvore do umbrella' },
        @{ id = 'TASK-CI-02'; titulo = 'Adicionar o job check-tasks ao CI do umbrella' })) {
    if ($indiceTexto.Contains("tasks/items/$($ci.id).md")) { continue }
    $nova = "- [$($ci.id) — $($ci.titulo)](tasks/items/$($ci.id).md) — tasks/items/$($ci.id).md"
    # Depois da ultima TASK-* do indice, que e onde o bloco termina.
    $mUlt = [regex]::Matches($indiceTexto, '(?m)^- \[TASK-[A-Z0-9-]+[^\]]*\]\(tasks/items/TASK-[A-Z0-9-]+\.md\)[^\n]*$')
    if ($mUlt.Count -eq 0) { throw "INDEX.md sem bloco de TASK; nao sei onde inserir $($ci.id)" }
    $fim = $mUlt[$mUlt.Count - 1].Index + $mUlt[$mUlt.Count - 1].Length
    $indiceTexto = $indiceTexto.Insert($fim, "`n`n$nova")
}
if ($PSCmdlet.ShouldProcess('INDEX.md', 'acrescentar TASK-CI-01 e TASK-CI-02')) {
    Write-Utf8 -Path $indiceUmbrella -Text $indiceTexto
}
# O texto que o INDEX.md TERA. A verificacao le daqui e nao do disco: sob -WhatIf
# o disco ainda tem o indice antigo, e medir o antigo reprovaria o ensaio por nao
# ter feito exatamente o que o -WhatIf mandou nao fazer.
$indiceFinal = $indiceTexto

# (b) de novo, agora os links de prosa. Varre so o que o `doc_files()` do xtask
#     varre: nada dentro de submodulo (o link la e do repo dono), nada de
#     _arquivo (arquivo congelado) nem .claude (configuracao).
$prefixosSub = @($Modulos | ForEach-Object { $_.caminho }) + @('reference/vynamp-mvp')
# Caminhos que o `files()`/`excluded()` do xtask descarta: o gate nunca le esses
# .md, entao reescrever link dentro deles seria trabalho invisivel -- e em
# _arquivo, dano: e o registro congelado do que foi arquivado.
$prefixosFora = @('_arquivo', '.claude', '.worktrees', '.superpowers', 'target', 'dist', 'node_modules', '.git')
$linksReescritos = 0
# Percorre a arvore a mao em vez de -Recurse: o -Recurse desce em target/ e
# node_modules ANTES de qualquer filtro, e num workspace Rust isso e dezenas de
# milhares de entradas para descartar depois. Aqui a poda acontece no diretorio.
function Get-MdDoUmbrella {
    param([string]$Dir, [string]$Raiz, [string[]]$Fora, [string[]]$Sub)
    $out = @()
    foreach ($f in @(Get-ChildItem -Path $Dir -Filter '*.md' -File -ErrorAction SilentlyContinue)) { $out += $f }
    foreach ($d in @(Get-ChildItem -Path $Dir -Directory -ErrorAction SilentlyContinue)) {
        $rel = $d.FullName.Substring($Raiz.Length).TrimStart('\', '/') -replace '\\', '/'
        if ($Fora -contains $d.Name) { continue }
        if ($Sub -contains $rel) { continue }
        $out += Get-MdDoUmbrella -Dir $d.FullName -Raiz $Raiz -Fora $Fora -Sub $Sub
    }
    return $out
}
$arquivosLinks = @(Get-MdDoUmbrella -Dir $umbrella -Raiz $umbrella -Fora $prefixosFora -Sub $prefixosSub)

$rxLink = [regex]'\]\(([^)]+)\)'
$textoFinal = @{}
foreach ($arq in $arquivosLinks) {
    $texto = Read-Utf8 $arq.FullName
    $antigo = $texto
    $dirDoArq = Split-Path $arq.FullName -Parent
    $texto = $rxLink.Replace($texto, [System.Text.RegularExpressions.MatchEvaluator] {
            param($m)
            $link = $m.Groups[1].Value
            # Só o alvo que termina em tasks/items/<ID>.md e de TASK que mudou de
            # casa. Um link para o ID que fica continua valendo como esta.
            $mm = [regex]::Match($link, '^.*?tasks/items/(TASK-[A-Z0-9-]+)\.md(#.*)?$')
            if (-not $mm.Success) { return $m.Value }
            $id = $mm.Groups[1].Value
            if (-not $alvoTask.ContainsKey($id)) { return $m.Value }
            if ($alvoTask[$id] -eq "tasks/items/$id.md") { return $m.Value }
            $abs = Join-Path $umbrella $alvoTask[$id]
            $novoRel = ([System.IO.Path]::GetRelativePath($dirDoArq, $abs) -replace '\\', '/')
            $ancora = if ($mm.Groups[2].Success) { $mm.Groups[2].Value } else { '' }
            return "]($novoRel$ancora)"
        })
    $textoFinal[$arq.FullName] = $texto
    if ($texto -ne $antigo) {
        if ($PSCmdlet.ShouldProcess($arq.Name, 'reapontar links de TASK para o repo dono')) {
            Write-Utf8 -Path $arq.FullName -Text $texto
        }
        $linksReescritos++
    }
}
Write-Feito "$linksReescritos arquivo(s) com link reapontado"

foreach ($repo in ($porRepo.Keys | Sort-Object)) {
    $raizRepo = $checkouts[$repo]
    $minhas = @($tarefas | Where-Object { (Get-RepoDe $mapaDono[$_.id]) -eq $repo } | Sort-Object id)
    $linhas = @()
    foreach ($t in $minhas) {
        $linhas += "| ``$($t.id)`` | $($t.title) | $($mapaDono[$t.id]) | $($t.status) |"
    }
    $extra = ''
    if ($repo -eq 'vynamp') {
        $linhas += "| ``TASK-CI-01`` | Reescrever delivery() e github_actions() para a árvore do umbrella | ``vynamp`` | planned |"
        $linhas += "| ``TASK-CI-02`` | Adicionar o job check-tasks ao CI do umbrella | ``vynamp`` | planned |"
        $extra = "`nAs duas últimas são TASKs de engenharia, emitidas pelo passo 37 (D-37-13) e não`npresentes no backlog de produto.`n"
    }
    $txt = @"
# tasks — $repo

$($minhas.Count)tarefa(s) do backlog de produto com este repo como dono.$extra

| ID | Título | Módulo dono | Estado |
|---|---|---|---|
$($linhas -join "`n")

O status oficial é o ``tasks/backlog.json`` do umbrella. O índice de épico fica lá
também: uma cópia por repo divergiria.
"@
    $arq = Join-Path (Join-Path $raizRepo 'tasks') 'README.md'
    if ($PSCmdlet.ShouldProcess("$repo/tasks/README.md", 'escrever indice de tarefas')) {
        Write-Utf8 -Path $arq -Text $txt
    }
}
Write-Feito "$($porRepo.Count) indice(s) tasks/README.md"
Write-Host ''

# ---------------------------------------------------------------------------
# 6. backlog.json com module, repo e acceptance_ready. Fonte unica de status.
# ---------------------------------------------------------------------------
Write-Passo '6. atualizando backlog.json'

$novasTarefas = @()
foreach ($t in $tarefas) {
    $mod = $mapaDono[$t.id]
    $repo = Get-RepoDe $mod
    $novo = [ordered]@{}
    foreach ($p in $t.PSObject.Properties) {
        $novo[$p.Name] = $p.Value
        # Campos novos entram logo depois de 'spec' para o JSON ficar legivel
        if ($p.Name -eq 'spec') {
            $novo['module'] = $mod
            $novo['repo'] = $repo
            $novo['acceptance_ready'] = $false
        }
    }
    if (-not $novo.Contains('module')) {
        $novo['module'] = $mod
        $novo['repo'] = $repo
        $novo['acceptance_ready'] = $false
    }
    $novasTarefas += [pscustomobject]$novo
}

$novasTarefas += [pscustomobject][ordered]@{
    id = 'TASK-CI-01'; epic = 'CI'; title = 'Reescrever delivery() e github_actions() para a árvore do umbrella'
    phase = 'M1'; priority = 'P0'; status = 'planned'; owner_role = 'engineering-delivery'
    requirements = @(); dependencies = @(); spec = 'xtask/src/validate.rs'
    module = 'vynamp'; repo = 'vynamp'; acceptance_ready = $true
    # title/deliverable entram LITERAIS no Markdown: mesma grafia, mesmo acento.
    # acceptance e comparado clausula a clausula (separador ';'), e cada clausula tem de
    # aparecer no Markdown -- por isso as tres frases abaixo sao trechos que existem nos
    # criterios de aceite do $Ci01.
    deliverable = 'validam spec, `xtask`, pins de submódulo e regras de arquitetura'
    acceptance = 'valida spec, `xtask`, pins de submódulo e regras de arquitetura; nenhuma checagem de empacotamento; os 3 testes de workflow passam com o `ci.yml` do passo 34'
    qa = @(); estimate_days = [pscustomobject]@{ min = 2; max = 4 }; risk = 'médio'
}
$novasTarefas += [pscustomobject][ordered]@{
    id = 'TASK-CI-02'; epic = 'CI'; title = 'Adicionar o job check-tasks ao CI do umbrella'
    phase = 'M1'; priority = 'P0'; status = 'planned'; owner_role = 'engineering-delivery'
    requirements = @(); dependencies = @('TASK-CI-01'); spec = 'xtask/src/validate.rs'
    module = 'vynamp'; repo = 'vynamp'; acceptance_ready = $true
    deliverable = 'Job `check-tasks` no `.github/workflows/ci.yml` do umbrella e o comando `cargo xtask check-tasks` que o alimenta'
    acceptance = 'reprova e imprime o ID da TASK e a coluna que falta; reprova e lista o ID; reprova e nomeia o arquivo'
    qa = @(); estimate_days = [pscustomobject]@{ min = 1; max = 2 }; risk = 'médio'
}

# O backlog vive no umbrella.
$backlogDestino = Join-Path (Join-Path $checkouts['vynamp'] 'tasks') 'backlog.json'
$json = [pscustomobject][ordered]@{
    # schema_version/spec_version vem do backlog anterior e NAO podem sumir: o gate
    # compara `spec_version` com `revision` de config/format-catalog.json e publica o
    # valor no relatorio de check-docs. Sem o campo a comparacao vira null x "0.7.0".
    schema_version = 1
    spec_version = '0.7.0'
    version = '2.0.0'
    generated_by = 'passo 37 (37-tasks.ps1)'
    source = 'backlog.json anterior, 191 tarefas, com module/repo/acceptance_ready acrescentados'
    note = 'Fonte unica de status. O Markdown em tasks/items nao carrega estado confiavel: os campos vem daqui.'
    counts = [pscustomobject][ordered]@{
        product = 191
        engineering = 2
        total = 193
        acceptance_ready = 2
    }
    tasks = $novasTarefas
}
$jsonTexto = $json | ConvertTo-Json -Depth 8

if ($PSCmdlet.ShouldProcess('tasks/backlog.json', 'reescrever com module, repo e acceptance_ready')) {
    Write-Utf8 -Path $backlogDestino -Text $jsonTexto
}
Write-Feito "backlog reescrito com $($novasTarefas.Count) entradas (191 produto + 2 engenharia)"
Write-Host ''

# ---------------------------------------------------------------------------
# 8. Verificacao. Falha em qualquer desvio ANTES de commitar.
# ---------------------------------------------------------------------------
Write-Passo '8. verificacao'

$falhas = @()

# 8.1 Total de arquivos TASK-*.md nos repos, fora de _arquivo
#
# No ensaio (-WhatIf) os arquivos nao foram escritos: contar o disco contaria o
# estado ANTERIOR, e o passo apareceria como reprovado por nao ter feito o que
# o ShouldProcess proibiu. O que se verifica ali e o PLANO -- 191 de produto
# mais as 2 de engenharia --, que e o que o run real vai gravar.
$ensaio = $WhatIfPreference
$todas = @()
if ($ensaio) {
    $esperadoTotal = $tarefas.Count + 2
    if ($esperadoTotal -ne 193) { $falhas += "plano tem $esperadoTotal TASK(s); esperado 193" }
    else { Write-Feito "193 TASKs no plano (ensaio)" }
} else {
    foreach ($repo in $checkouts.Keys) {
        $p = Join-Path (Join-Path $checkouts[$repo] 'tasks') 'items'
        if (Test-Path $p) { $todas += @(Get-ChildItem -Path $p -Filter 'TASK-*.md' -File) }
    }
    if ($todas.Count -ne 193) { $falhas += "total de TASKs escritas = $($todas.Count); esperado 193 (191 produto + 2 engenharia)" }
    else { Write-Feito "193 TASKs escritas" }
}

# 8.2 Nenhum ID duplicado
$dups = @($todas | Group-Object BaseName | Where-Object Count -gt 1)
if ($dups.Count -gt 0) {
    foreach ($d in $dups) { $falhas += "TASK duplicada em dois repos: $($d.Name)" }
} else { Write-Feito 'nenhum ID duplicado' }

# 8.3 Toda TASK cita Repo
$semRepo = @($todas | Where-Object { (Read-Utf8 $_.FullName) -notmatch '\| Repo \| `' })
if ($semRepo.Count -gt 0) {
    foreach ($s in $semRepo) { $falhas += "sem campo Repo: $($s.BaseName)" }
} else { Write-Feito 'toda TASK cita o repo' }

# 8.4 Then == clausula do backlog (o criterio nao foi reescrito)
$divergentes = @()
foreach ($t in $(if ($ensaio) { @() } else { $tarefas })) {
    $mod = $mapaDono[$t.id]
    $repo = Get-RepoDe $mod
    $arq = Join-Path (Join-Path (Join-Path $checkouts[$repo] 'tasks') 'items') "$($t.id).md"
    if (-not (Test-Path $arq)) { $divergentes += "$($t.id): arquivo ausente"; continue }
    $txt = Read-Utf8 $arq
    foreach ($c in @($t.acceptance -split ';\s*' | Where-Object { $_.Trim() })) {
        if (-not $txt.Contains($c.Trim())) { $divergentes += "$($t.id): clausula ausente no Then -> $($c.Trim())" }
    }
}
if ($divergentes.Count -gt 0) {
    foreach ($d in $divergentes) { $falhas += "criterio reescrito: $d" }
} elseif ($ensaio) { Write-Feito 'Then identico ao backlog por construcao (ensaio; a comparacao roda no run real)' }
else { Write-Feito 'Then identico ao acceptance do backlog (nenhum criterio inventado)' }

# 8.5 acceptance_ready falso nao coexiste com estado avancado
#
# Le o arquivo quando ele ja foi reescrito e, sob -WhatIf -- onde o
# ShouldProcess pulou a escrita e medir o disco mediria o backlog ANTIGO --, o
# objeto que este run WIRIA escrever. Sem isso o ensaio reprova por um campo
# que o proprio passo ainda nao gravou, e um ensaio que so falha no fim nao
# ensina nada sobre o que o passo faz.
$novo = (Read-Utf8 (Join-Path (Join-Path $checkouts['vynamp'] 'tasks') 'backlog.json')) | ConvertFrom-Json
$tasksVerif = if (@($novo.tasks)[0].PSObject.Properties['acceptance_ready']) { @($novo.tasks) } else { @($novasTarefas) }
$conflito = @($tasksVerif | Where-Object { $_.status -ne 'planned' -and -not $_.acceptance_ready })
if ($conflito.Count -gt 0) {
    $ids = ($conflito | ForEach-Object { $_.id }) -join ', '
    Write-Aviso "acceptance_ready=false em estado avancado (esperado neste passo; check-tasks reprova ate a spec ter secao 5):"
    Write-Aviso "  $ids"
}

# 8.6 Contagem por repo confere com a tabela
$esperadoPorRepo = @{ 'vynamp' = 57 + 2; 'vynamp-player' = 92; 'vynamp-decode' = 17
                      'vynamp-output' = 6; 'vynamp-engine' = 15; 'vynamp-dsp' = 1
                      'vynamp-converter' = 2; 'vynamp-restore' = 1; 'vynamp-encode' = 0 }
$realPorRepo = @{}
foreach ($repo in $checkouts.Keys) { $realPorRepo[$repo] = 0 }
foreach ($t in $tasksVerif) {
    $r = $t.repo
    if (-not $realPorRepo.ContainsKey($r)) { $realPorRepo[$r] = 0 }
    $realPorRepo[$r]++
}
foreach ($r in $esperadoPorRepo.Keys) {
    $real = if ($realPorRepo.ContainsKey($r)) { $realPorRepo[$r] } else { 0 }
    if ($real -ne $esperadoPorRepo[$r]) { $falhas += "$r : $real TASKs; esperado $($esperadoPorRepo[$r])" }
}
if ($falhas.Count -eq 0) { Write-Feito 'contagem por repo confere com a tabela de 37-tasks.md secao 3' }

# 8.7 As 23 specs existem com secao 2 preenchida
$specsEsperadas = ($Specs | ForEach-Object { $_.arquivos.Count } | Measure-Object -Sum).Sum
$specsReais = 0
foreach ($s in $(if ($ensaio) { @() } else { $Specs })) {
    $p = Join-Path $checkouts[$s.repo] 'specs'
    if (-not (Test-Path $p)) { continue }
    foreach ($nome in $s.arquivos) {
        $arq = Join-Path $p "$nome.md"
        if (-not (Test-Path $arq)) { $falhas += "spec ausente: $($s.repo)/specs/$nome.md"; continue }
        $txt = Read-Utf8 $arq
        if ($txt -notmatch '(?m)^## 2\. Fronteira') { $falhas += "spec sem secao 2: $($s.repo)/specs/$nome.md" }
        if ($txt -match 'PENDENTE[\s\S]{0,40}## 2\.') { $falhas += "secao 2 com PENDENTE: $($s.repo)/specs/$nome.md" }
        $specsReais++
    }
}
if ($ensaio) { Write-Feito "$specsEsperadas specs de modulo no plano (ensaio)" }
elseif ($specsReais -ne $specsEsperadas) { $falhas += "specs = $specsReais; esperado $specsEsperadas" }
else { Write-Feito "$specsReais specs de modulo com fronteira preenchida" }

# 8.8 Nenhum documento do umbrella aponta para uma TASK que mudou de casa.
#    E o unico item que o proprio script consegue provar sem rodar o xtask:
#    check-docs reprovaria no primeiro Missing link, mas so depois do commit.
$sobras = @()
foreach ($arq in $arquivosLinks) {
    $txt = $textoFinal[$arq.FullName]
    $dirDoArq = Split-Path $arq.FullName -Parent
    # Resolve o alvo e compara com o arquivo real. Comparar o texto do link
    # marcaria como sobra o link do indice de epico (../../libs/decode/tasks/
    # items/...), que contem 'tasks/items/' mas ja aponta para o lugar certo.
    foreach ($m in [regex]::Matches($txt, '\]\(([^)]*?tasks/items/(TASK-[A-Z0-9-]+)\.md)(#[^)]*)?\)')) {
        $id = $m.Groups[2].Value
        if (-not $alvoTask.ContainsKey($id)) { continue }
        $esperado = [System.IO.Path]::GetFullPath((Join-Path $umbrella $alvoTask[$id]))
        $achado = [System.IO.Path]::GetFullPath((Join-Path $dirDoArq $m.Groups[1].Value))
        if ($achado -ne $esperado) {
            $rel = $arq.FullName.Substring($umbrella.Length).TrimStart('\', '/')
            $sobras += "$rel -> $($m.Groups[1].Value)"
        }
    }
}
if ($sobras.Count -gt 0) {
    foreach ($s in $sobras) { $falhas += "link nao reapontado: $s" }
} else { Write-Feito "nenhum link do umbrella aponta para TASK que saiu ($($arquivosLinks.Count) arquivo(s) varrido(s))" }

# 8.9 O INDEX.md nao lista mais as TASKs que sairam, e lista as duas novas
$idx = $indiceFinal
$orfas = @()
foreach ($t in $tarefas) {
    if ($alvoTask[$t.id] -eq "tasks/items/$($t.id).md") { continue }
    if ($idx.Contains("](tasks/items/$($t.id).md)")) { $orfas += $t.id }
}
if ($orfas.Count -gt 0) { $falhas += "INDEX.md ainda lista $($orfas.Count) TASK(s) que sairam: $($orfas -join ', ')" }
foreach ($ci in @('TASK-CI-01', 'TASK-CI-02')) {
    if (-not $idx.Contains("](tasks/items/$ci.md)")) { $falhas += "INDEX.md sem $ci" }
}
if ($orfas.Count -eq 0) { Write-Feito 'INDEX.md podado e com as duas TASKs de engenharia' }

# 8.10 O que o GATE le do backlog continua la, e as duas TASKs de engenharia
#      concordam com o proprio Markdown.
#      8.4 so cobre as 191 de produto, porque le $tarefas (o backlog antigo). As duas
#      de engenharia nascem neste passo e nunca passavam por comparacao nenhuma --
#      foi assim que TASK-CI-01 ficou com `arvore` no backlog e `árvore` no Markdown.
#      `check-docs` compara title/deliverable literais e acceptance clausula a clausula;
#      repetir a comparacao aqui e o que impede o passo de commitar um backlog que o
#      proprio gate reprova -- e o gate rodava DEPOIS do commit.
$verifBacklog = if ($ensaio) { $novasTarefas } else { @((Get-Content -Raw -Encoding utf8 $backlogDestino | ConvertFrom-Json).tasks) }
$cabecalho = if ($ensaio) { $json } else { Get-Content -Raw -Encoding utf8 $backlogDestino | ConvertFrom-Json }
if (-not $cabecalho.PSObject.Properties['spec_version'] -or -not $cabecalho.spec_version) {
    $falhas += 'backlog.json sem spec_version: check-docs compara com config/format-catalog.json'
}
$catalogoRev = (Get-Content -Raw -Encoding utf8 (Join-Path $checkouts['vynamp'] 'config/format-catalog.json') | ConvertFrom-Json).revision
if ($cabecalho.spec_version -ne $catalogoRev) {
    $falhas += "spec_version ($($cabecalho.spec_version)) difere da revision do catalogo ($catalogoRev)"
}
foreach ($ci in @('TASK-CI-01', 'TASK-CI-02')) {
    $entrada = @($verifBacklog | Where-Object { $_.id -eq $ci })[0]
    $arqCi = Join-Path (Join-Path (Join-Path $checkouts['vynamp'] 'tasks') 'items') "$ci.md"
    $txtCi = if (Test-Path $arqCi) { Read-Utf8 $arqCi } elseif ($ensaio) { if ($ci -eq 'TASK-CI-01') { $Ci01 } else { $Ci02 } } else { '' }
    foreach ($campo in @('title', 'deliverable')) {
        if (-not $txtCi.Contains($entrada.$campo)) {
            $falhas += "$ci $campo do backlog nao aparece no Markdown (o gate reprova)"
        }
    }
    foreach ($c in @($entrada.acceptance -split ';\s*' | Where-Object { $_.Trim() })) {
        if (-not $txtCi.Contains($c.Trim())) {
            $falhas += "$ci clausula do acceptance nao aparece no Markdown -> $($c.Trim())"
        }
    }
}
if ($cabecalho.spec_version -eq $catalogoRev -and $falhas.Count -eq 0) {
    Write-Feito "backlog com spec_version $catalogoRev e as duas TASKs de engenharia iguais ao Markdown"
}

Write-Host ''
if ($falhas.Count -gt 0) {
    foreach ($f in $falhas) { Write-Aviso $f }
    throw "verificacao falhou ($($falhas.Count) item(ns)). Nada foi commitado."
}
Write-Host '    Verificacao do passo 37 passou.' -ForegroundColor Green
Write-Host ''

# ---------------------------------------------------------------------------
# 9. Commit por repo, em branch propria, SEM push.
#    O hook do passo 36 bloqueia commit em main/develop; a branch e criada aqui.
# ---------------------------------------------------------------------------
if ($SemCommit) {
    Write-Aviso 'commit pulado (-SemCommit)'
    Write-Host ''
    return
}

Write-Passo '9. commitando em cada repo (local, sem push)'

$branch = 'chore/claude-tasks-37'
$commitados = @()

# Itera pelos checkouts, nao por $porRepo: um repo sem TASK de produto (encode)
# ainda recebe specs/ e tasks/README.md e precisa do commit. Quem decide e o
# 'git status', nao a contagem de tarefas.
foreach ($repo in ($checkouts.Keys | Sort-Object)) {
    $onde = $checkouts[$repo]

    $atual = (Invoke-Git @('branch', '--show-current') -Onde $onde)
    if ($atual -ne $branch) {
        $existe = (& git -C $onde rev-parse --verify --quiet "refs/heads/$branch") 2>$null
        if ($existe) {
            if (-not $PSCmdlet.ShouldProcess($repo, "trocar para a branch existente $branch")) { continue }
            Invoke-Git @('switch', $branch) -Onde $onde | Out-Null
        } else {
            if (-not $PSCmdlet.ShouldProcess($repo, "criar branch $branch")) { continue }
            Invoke-Git @('switch', '-c', $branch) -Onde $onde | Out-Null
        }
        Write-Feito "$repo`: branch $branch"
    }

    $sujo = (Invoke-Git @('status', '--short') -Onde $onde)
    if (-not $sujo) { Write-Feito "$repo`: nada a commitar"; continue }

    if (-not $PSCmdlet.ShouldProcess($repo, 'commitar as TASKs e as specs')) { continue }

    Invoke-Git @('add', '-A') -Onde $onde | Out-Null

    $n = if ($porRepo.ContainsKey($repo)) { $porRepo[$repo] } else { 0 }
    $corpo = if ($repo -eq 'vynamp') {
@"
chore(tasks): reescreve as TASKs do umbrella e consolida o backlog (passo 37)

- 57 TASKs transversais com dono vynamp (D-37-2)
- TASK-CI-01 (rework de delivery/github_actions) e TASK-CI-02 (check-tasks)
  emitidas aqui (D-37-13); total 193
- backlog.json ganha module, repo e acceptance_ready
- indices de epico ficam no umbrella (D-37-11)
- nenhuma TASK de produto criada, arquivada ou promovida (D-37-6, D-37-7, D-37-12)
"@
    } else {
@"
chore(tasks): reescreve as TASKs de $repo no formato padroes/tasks.md (passo 37)

- atribuicao por tarefa, nao por epico (D-37-1)
- $n TASK(s) de produto com este repo como dono
- Then copiado literal do backlog; Given/When marcados para autoria (D-37-3)
- specs/ esqueleto com proposito, fronteira e fora de escopo (D-37-4)
- acceptance_ready = false; nenhuma TASK sai de planned sem criterio
"@
    }

    Invoke-Git @('commit', '-q', '-m', $corpo.Trim()) -Onde $onde | Out-Null
    $sha = (Invoke-Git @('rev-parse', '--short', 'HEAD') -Onde $onde)
    Write-Feito "$repo`: commit $sha em $branch (sem push)"
    $commitados += @{ repo = $repo; sha = $sha; branch = $branch }
}
Write-Host ''

# ---------------------------------------------------------------------------
# Relatorio final
# ---------------------------------------------------------------------------
Write-Passo 'relatorio'

Write-Host '    TASKs por repo:' -ForegroundColor DarkGray
foreach ($r in ($esperadoPorRepo.Keys | Sort-Object)) {
    $real = if ($realPorRepo.ContainsKey($r)) { $realPorRepo[$r] } else { 0 }
    Write-Host ("      {0,-18} {1,3}" -f $r, $real) -ForegroundColor DarkGray
}

Write-Host ''
Write-Host '    Commits locais (sem push):' -ForegroundColor DarkGray
if ($commitados.Count -eq 0) { Write-Host '      nenhum' -ForegroundColor DarkGray }
foreach ($c in $commitados) { Write-Host ("      {0,-18} {1}  {2}" -f $c.repo, $c.sha, $c.branch) -ForegroundColor DarkGray }

Write-Host ''
Write-Host '    Pendencias que ficam abertas com este passo:' -ForegroundColor Yellow
Write-Host '      - 191 TASKs com Given/When PENDENTE e acceptance_ready = false' -ForegroundColor Yellow
Write-Host '      - 23 specs com secao 3 a 10 PENDENTE (so 1, 2 e 7 escritas)' -ForegroundColor Yellow
Write-Host '      - vynamp-encode e vynamp-dsp sem TASK de produto (D-37-6, pendencia P1)' -ForegroundColor Yellow
Write-Host '      - E09 sem repo (D-37-5, pendencia P2)' -ForegroundColor Yellow
Write-Host '      - TASK-CI-01 e TASK-CI-02 emitidas mas nao implementadas' -ForegroundColor Yellow
Write-Host ''
Write-Host '    Nada foi publicado. Nenhum merge, tag, PR ou mudanca de visibilidade.' -ForegroundColor Yellow
Write-Host ''
