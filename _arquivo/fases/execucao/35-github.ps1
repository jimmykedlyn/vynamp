<#
    35-github.ps1 - passo 35 da FASE 3

    Cria os 7 repositorios novos no GitHub, semeia cada um com o minimo que o CI
    exige, liga os 9 submodulos ao umbrella e - POR ULTIMO, sob confirmacao
    digitada - arquiva o vynamp-mediaplayer.

    Este e o primeiro passo da FASE 3 que toca a rede. Ele NAO mexe em
    visibilidade, protecao de branch, assinatura nem plano. Os 7 repos nascem
    privados e permanecem privados.

    O vynamp-mediaplayer e arquivado com 'gh repo archive', que e reversivel por
    'gh repo unarchive'. O script nao faz merge, nao cria release e nao assina.

    Uso:  pwsh -NoProfile -File .\35-github.ps1
          .\35-github.ps1 -WhatIf
          .\35-github.ps1 -SemArquivar

    Parametros:
      -Ambiente     AMBIENTE.ps1 da raiz externa (o bootstrap NAO vai para o
                    umbrella). So e verificado, nao e carregado: dot-source dele
                    mudaria TEMP e CARGO_HOME do processo.
      -SemArquivar  pula a secao 9; o vynamp-mediaplayer continua ativo.
      -SemGitHub    nao existe aqui: este passo toca a rede por desenho.

    Nenhum argumento recebe token. O script exige 'gh' ja autenticado e, se
    nao estiver, imprime a receita com Invoke-VynampAuthenticatedAgent.
#>
[CmdletBinding(SupportsShouldProcess = $true)]
param(
    [string]$Dono = 'jimmykedlyn',
    [string]$Raiz = 'D:\Projects\vynamp',
    # AMBIENTE.ps1 fica na raiz externa e NAO vai para o umbrella: e o wrapper
    # de bootstrap do vynamp-mediaplayer. O passo 38 NUNCA o apaga (e o unico
    # caminho de autenticacao hoje). Se nao achar, rode assim (GH_TOKEN so no ambiente):
    #   . (Join-Path 'D:/Projects/vynamp-media-player' 'AMBIENTE.ps1')
    #   Invoke-VynampAuthenticatedAgent -Action { pwsh -NoProfile -File ./35-github.ps1 }
    [string]$Ambiente = 'D:\Projects\vynamp-media-player\AMBIENTE.ps1',
    # Pula a secao 9 (arquivar o vynamp-mediaplayer). O arquivamento e
    # reversivel por 'gh repo unarchive', mas e uma mudanca de estado no
    # GitHub: por isso ele e o ultimo passo, com confirmacao digitada, e pode
    # ser adiado com esta flag.
    [switch]$SemArquivar
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

function Write-Passo { param($m) Write-Host "==> $m" -ForegroundColor Cyan }
function Write-Feito { param($m) Write-Host "    $m" -ForegroundColor DarkGray }
function Write-Aviso { param($m) Write-Host "    ! $m" -ForegroundColor Yellow }

# ---------------------------------------------------------------------------
# Catalogo. 8 repos de codigo (5 libs + 3 apps) + a referencia = 9 submodulos.
# O umbrella e o proprio repo do trabalho e nao se adiciona como submodule.
# ---------------------------------------------------------------------------
$Modulos = @(
    @{ nome = 'vynamp-dsp';       caminho = 'libs/dsp';       tipo = 'lib';    ci = 'ci-lib.yml' }
    @{ nome = 'vynamp-encode';    caminho = 'libs/encode';    tipo = 'lib';    ci = 'ci-lib.yml' }
    @{ nome = 'vynamp-engine';    caminho = 'libs/engine';    tipo = 'engine'; ci = 'ci-engine.yml' }
    @{ nome = 'vynamp-player';    caminho = 'apps/player';    tipo = 'app';    ci = 'ci-app.yml' }
    @{ nome = 'vynamp-converter'; caminho = 'apps/converter'; tipo = 'app';    ci = 'ci-app.yml' }
    @{ nome = 'vynamp-restore';   caminho = 'apps/restore';   tipo = 'app';    ci = 'ci-app.yml' }
)
$Existentes = @(
    @{ nome = 'vynamp-decode'; caminho = 'libs/decode'; url = "https://github.com/$Dono/vynamp-decode.git" }
    @{ nome = 'vynamp-output'; caminho = 'libs/output'; url = "https://github.com/$Dono/vynamp-output.git" }
)
$Referencia = @{
    nome   = 'vynamp-mvp-reference'
    caminho = 'reference/vynamp-mvp'
    url    = "https://github.com/$Dono/vynamp-mvp-reference.git"
    commit = '741316f'
}
$TagInicial = 'v0.1.0'
$Fases = 'D:\Projects\vynamp-fases'

$Descricoes = @{
    'vynamp-dsp'       = 'Primitivas de sinal: resampler, dither, mapeamento de canal, mixer. std-only, sem dominio.'
    'vynamp-encode'    = 'Codificacao de audio. Depende opcional de vynamp-dsp (feature resample).'
    'vynamp-engine'    = 'Integracao, dominio e cadeia DSP. Unico que conhece decode, encode, output e dsp.'
    'vynamp-player'    = 'Player Vynamp: shell de playback, integracao do engine, UI.'
    'vynamp-converter' = 'Conversor em lote de audio. Processo separado do player.'
    'vynamp-restore'   = 'Restauracao de audio. Processo separado, motor compartilhado com o engine.'
}

# ---------------------------------------------------------------------------
# Execucao. Todo comando git/gh roda por Invoke-Git / Invoke-Gh, que capturam a
# saida e falham com a mensagem real. Nenhum token e lido, impresso ou gravado:
# o GH_TOKEN vem do ambiente, dentro do bloco autenticado do AMBIENTE.ps1.
# O git autentica pelo credential helper global, que ja aponta para o gh.
# ---------------------------------------------------------------------------
# Push e fetch sao as operacoes de git que dependem da rede; as locais (add,
# commit, checkout, submodule add depois do clone) nao precisam de retry.
# Sem isto, uma queda de 20 s no push de main aborta o passo depois de tudo
# pronto -- medido duas vezes.
function Invoke-Git {
    param(
        [string[]]$Argumentos,
        [string]$Onde = $Raiz,
        [int]$Tentativas = 1
    )
    $out = $null
    for ($t = 1; $t -le $Tentativas; $t++) {
        $out = & git -C $Onde @Argumentos 2>&1
        $code = $LASTEXITCODE
        if ($code -eq 0) { break }
        $detalhe = ($out | Out-String)
        # Erro de transporte se repete; erro de conteudo/repositorio nao.
        $transporte = $detalhe -match 'Could not connect|Failed to connect|connectex|timed out|RPC failed|early EOF|remote end hung up'
        if (-not $transporte -or $t -eq $Tentativas) {
            throw "git $($Argumentos -join ' ') falhou (exit $code) em ${Onde}:`n$($detalhe.Trim())"
        }
        Start-Sleep -Seconds (8 * $t)
    }
    return ($out | Out-String).Trim()
}

function Invoke-Gh {
    param([string[]]$Argumentos)
    $out = & gh @Argumentos 2>&1
    $code = $LASTEXITCODE
    if ($code -ne 0) {
        $detalhe = ($out | Out-String).Trim()
        throw "gh $($Argumentos -join ' ') falhou (exit $code):`n$detalhe"
    }
    return ($out | Out-String).Trim()
}

# Diagnostica 403: repositorio inexistente vs. credencial sem permissao.
#
# A rede deste ambiente derruba conexoes com 'dial tcp ... connectex' de forma
# intermitente, e nao '403' nem '404' -- esses dois sao resposta, nao falha de
# transporte. Sem retry, uma queda de tres segundos no meio do passo deixa o
# script parado num estado valido mas incompleto (7 repos criados, 0 semeados),
# que e exatamente o que aconteceu na primeira execucao. As consultas sao de
# leitura, entao repetir e seguro.
function Invoke-GhComRetry {
    param([string[]]$Argumentos, [int]$Tentativas = 6)
    $out = $null
    for ($t = 1; $t -le $Tentativas; $t++) {
        $out = & gh @Argumentos 2>&1
        if ($LASTEXITCODE -eq 0) { return @{ Ok = $true; Saida = ($out | Out-String).Trim() } }
        $txt = ($out | Out-String)
        # 403/404 sao respostas definitivas: repetir nao muda o resultado.
        if ($txt -match '404|Not Found|403|Forbidden') { break }
        if ($t -lt $Tentativas) { Start-Sleep -Seconds (3 * $t) }
    }
    return @{ Ok = $false; Saida = ($out | Out-String).Trim() }
}

function Testa-RepoExiste {
    param([string]$Nome)
    $r = Invoke-GhComRetry @('api', "repos/$Dono/$Nome", '--jq', '.full_name')
    if ($r.Ok) { return $true }
    if ($r.Saida -match '404|Not Found') { return $false }
    if ($r.Saida -match '403') {
        throw "403 em repos/$Dono/$Nome. Nao e limite de plano: e credencial sem permissao ou org com restricao. Diagnostique a autenticacao antes de seguir."
    }
    throw "Falha ao consultar repos/$Dono/${Nome} apos 6 tentativas: $($r.Saida)"
}

function Testa-BranchExiste {
    param([string]$Nome, [string]$Branch)
    # Repo recem-criado e vazio responde 409 'Git Repository is empty': nao tem
    # branch nenhuma ainda, que e a mesma resposta que 'esta branch nao existe'.
    #
    # Nao basta devolver Ok: uma queda de rede depois dos retries daria 'false'
    # e o script re-semearia um repo que ja tem develop -- apagando o commit
    # com Cargo.lock e refazendo o push por cima. 'Nao sei' tem de ser distinto
    # de 'nao existe', e a unica resposta segura para 'nao sei' e parar.
    $r = Invoke-GhComRetry @('api', "repos/$Dono/$Nome/branches/$Branch", '--jq', '.name')
    if ($r.Ok) { return $true }
    if ($r.Saida -match '404|Not Found|409|Git Repository is empty') { return $false }
    throw "Falha ao consultar a branch $Branch de ${Nome} apos 6 tentativas; nao sei se ela existe, e assumir que nao existe re-semearia o repo. $($r.Saida)"
}

function Testa-TagExiste {
    param([string]$Nome, [string]$Tag)
    $r = Invoke-GhComRetry @('api', "repos/$Dono/$Nome/git/ref/tags/$Tag", '--jq', '.ref')
    if ($r.Ok) { return $true }
    if ($r.Saida -match '404|Not Found|409') { return $false }
    throw "Falha ao consultar a tag $Tag de ${Nome} apos 6 tentativas: $($r.Saida)"
}

# ---------------------------------------------------------------------------
# Pre-checagem
# ---------------------------------------------------------------------------
Write-Passo 'pre-checagem'

if (-not (Test-Path $Ambiente)) { throw "AMBIENTE.ps1 nao encontrado: $Ambiente" }

# O AMBIENTE.ps1 nao e carregado aqui de proposito: dot-source dele redefine
# TEMP, TMP e CARGO_HOME do processo inteiro, e este passo so precisa de 'gh'.
# Ele continua sendo o caminho de autenticacao -- a receita abaixo o usa.
# A checagem vem ANTES de criar repo: falhar no meio do passo 1 deixaria
# repos criados sem submodulo, que e o pior estado possivel.
if ($env:OS -eq 'Windows_NT') {
    $jaAutenticado = & gh auth status 2>&1
    if ($LASTEXITCODE -ne 0) {
        throw @'
gh sem credencial neste shell. Rode dentro do bloco autenticado:

    . D:\Projects\vynamp-media-player\AMBIENTE.ps1
    Invoke-VynampAuthenticatedAgent -Action { pwsh -NoProfile -File ./35-github.ps1 }

O token nunca vai em argumento nem em arquivo: ele e lido dentro do bloco e
devolvido ao valor anterior no finally.
'@
    }
    Write-Feito 'gh autenticado'
}

if (-not (Test-Path $Raiz)) { throw "Umbrella nao encontrado: $Raiz. Rode o passo 31 antes." }
if (-not (Test-Path (Join-Path $Raiz '.git'))) { throw "$Raiz nao e um repositorio git." }

$dirty = & git -C $Raiz status --short 2>&1
if ($dirty) {
    Write-Aviso 'arvore do umbrella nao esta limpa:'
    $dirty | ForEach-Object { Write-Host "      $_" }
    throw 'Commite o passo 34 antes.'
}
Write-Feito 'arvore limpa'

foreach ($m in $Modulos) {
    $ci = Join-Path $Fases "padroes\$($m.ci)"
    if (-not (Test-Path $ci)) { throw "Template de CI ausente: $ci (rode o passo 34)" }
}
Write-Feito 'templates de CI presentes'

# ---------------------------------------------------------------------------
# 1. Criar os 7 repos no GitHub, privados
# ---------------------------------------------------------------------------
Write-Passo '1. criando os 7 repositorios novos (privados)'

foreach ($m in $Modulos) {
    $nome = $m.nome
    if (Testa-RepoExiste -Nome $nome) {
        Write-Feito "$nome ja existe; mantido como esta"
        continue
    }
    if ($PSCmdlet.ShouldProcess("$Dono/$nome", 'criar repositorio privado')) {
        Invoke-Gh @('repo', 'create', "$Dono/$nome", '--private', '--description', $Descricoes[$nome]) | Out-Null
        Write-Feito "$nome criado (privado)"
    } else {
        Write-Feito "$nome seria criado (WhatIf)"
    }
}
Write-Host ''

# ---------------------------------------------------------------------------
# 2. Semear os 7 repos e publicar develop + main.
#    Nenhum pode ficar vazio: 'git submodule add' falha contra repo sem commit,
#    e 'cargo --locked' falha sem Cargo.lock.
# ---------------------------------------------------------------------------
Write-Passo '2. semeando os 7 repositorios e publicando develop e main'

$Area = Join-Path $env:TEMP "vynamp-seed-$PID"
if (Test-Path $Area) { Remove-Item $Area -Recurse -Force }
New-Item -ItemType Directory -Path $Area -Force | Out-Null

try {
    foreach ($m in $Modulos) {
        $nome = $m.nome

        if (Testa-BranchExiste -Nome $nome -Branch 'develop') {
            Write-Feito "$nome ja tem develop publicado; skip do seeding"
            continue
        }
        if (-not $PSCmdlet.ShouldProcess("$Dono/$nome", 'semear e publicar')) { continue }

        $work = Join-Path $Area $nome
        $crate = $nome.Replace('-', '_')
        $isBin = ($m.tipo -eq 'app')

        New-Item -ItemType Directory -Path $work -Force | Out-Null
        & git -C $work init -q -b develop 2>&1 | Out-Null
        & git -C $work config user.email 'agent@vynamp.local' 2>&1 | Out-Null
        & git -C $work config user.name 'Vynamp Agent' 2>&1 | Out-Null

        # Cabecalho identico ao do resto do projeto: edition 2024, rust-version 1.99.
        $linhas = @(
            '[package]'
            "name = `"$crate`""
            'version = "0.1.0"'
            'edition = "2024"'
            'rust-version = "1.99"'
            "description = `"$($Descricoes[$nome])`""
            'license = "MIT OR Apache-2.0"'
            'publish = false'
            ''
            '[dependencies]'
            ''
        )
        if ($isBin) {
            # App: binario com nome proprio. Sem [lib], o cargo nao infere crate.
            $linhas += @(
                '[[bin]]'
                "name = `"$crate`""
                "path = `"src/main.rs`""
                ''
            )
        } else {
            # Lib: sem [lib], o cargo procura src/lib.rs — mas a declaracao
            # explicita deixa o nome do crate visivel e imutavel.
            $linhas += @(
                '[lib]'
                "name = `"$crate`""
                "path = `"src/lib.rs`""
                ''
            )
        }
        Write-Utf8 -Path (Join-Path $work 'Cargo.toml') -Text ($linhas -join "`n")

        $srcDir = Join-Path $work 'src'
        New-Item -ItemType Directory -Path $srcDir -Force | Out-Null
        if ($isBin) {
            Write-Utf8 -Path (Join-Path $srcDir 'main.rs') -Text @"
fn main() {
    println!("$crate 0.1.0");
}
"@
        } else {
            Write-Utf8 -Path (Join-Path $srcDir 'lib.rs') -Text @"
//! $nome
//!
//! Especifico deste modulo: ver specs/ no proprio repo.

/// Versao do modulo. O contrato de release do umbrella fixa a tag.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
"@
        }

        Write-Utf8 -Path (Join-Path $work '.gitignore') -Text @"
/target
"@

        Write-Utf8 -Path (Join-Path $work 'AGENTS.md') -Text @"
# AGENTS.md

O contrato do agente e o do umbrella:

https://github.com/$Dono/vynamp/blob/develop/AGENTS.md

Especifico deste modulo:

- <regra local, se houver. Se ela contrariar o umbrella, corrija o umbrella.>
"@

        Write-Utf8 -Path (Join-Path $work 'README.md') -Text @"
# $nome

$($Descricoes[$nome])

Compila sozinho:

``````bash
cargo test
``````

O contrato do agente e o do umbrella: https://github.com/$Dono/vynamp/blob/develop/AGENTS.md
"@

        Write-Utf8 -Path (Join-Path $work 'rust-toolchain.toml') -Text @"
[toolchain]
channel = "1.99.0"
profile = "minimal"
components = ["rustfmt", "clippy"]
"@

        $ghDir = Join-Path $work '.github\workflows'
        New-Item -ItemType Directory -Path $ghDir -Force | Out-Null
        Copy-Item (Join-Path $Fases "padroes\$($m.ci)") (Join-Path $ghDir 'ci.yml') -Force

        # Cargo.lock: 'cargo --locked' falha sem ele, e o CI usa --locked.
        Push-Location $work
        try {
            & cargo generate-lockfile --offline 2>&1 | Out-Null
            if ($LASTEXITCODE -ne 0) {
                & cargo generate-lockfile 2>&1 | Out-Null
                if ($LASTEXITCODE -ne 0) { throw "cargo generate-lockfile falhou em $nome" }
            }
        } finally { Pop-Location }

        if (-not (Test-Path (Join-Path $work 'Cargo.lock'))) {
            throw "Cargo.lock nao foi gerado em $nome; o CI usa --locked e quebraria."
        }

        & git -C $work add -A 2>&1 | Out-Null
        & git -C $work commit -q -m "chore: estrutura inicial de $nome" 2>&1 | Out-Null

        # remote + push de develop e main
        & git -C $work remote add origin "https://github.com/$Dono/$nome.git" 2>&1 | Out-Null
        & git -C $work push -u origin develop 2>&1 | Out-Null
        if ($LASTEXITCODE -ne 0) { throw "push de develop falhou em $nome" }
        Write-Feito "${nome}: develop publicada"

        if (-not (Testa-BranchExiste -Nome $nome -Branch 'main')) {
            & git -C $work push origin 'develop:refs/heads/main' 2>&1 | Out-Null
            if ($LASTEXITCODE -ne 0) { throw "push de main falhou em $nome" }
            Write-Feito "${nome}: main criada a partir de develop"
        }

        # branch padrao = develop (a tabela de 04-repos-e-remotes.md §1).
        Invoke-Gh @('repo', 'edit', "$Dono/$nome", '--default-branch', 'develop') | Out-Null
        Write-Feito "${nome}: branch padrao = develop"

        $nArquivos = @(& git -C $work ls-files).Count
        Write-Feito "$nome semeado ($nArquivos arquivo(s))"
    }
} finally {
    if (Test-Path $Area) { Remove-Item $Area -Recurse -Force -ErrorAction SilentlyContinue }
}
Write-Host ''

# ---------------------------------------------------------------------------
# 3. O umbrella troca de remote: vynamp-mediaplayer -> vynamp
# ---------------------------------------------------------------------------
Write-Passo '3. apontando o origin do umbrella para jimmykedlyn/vynamp'

if (-not (Testa-RepoExiste -Nome 'vynamp')) {
    if ($PSCmdlet.ShouldProcess("$Dono/vynamp", 'criar repositorio privado do umbrella')) {
        Invoke-Gh @('repo', 'create', "$Dono/vynamp", '--private', '--description', 'Umbrella Vynamp: workspace, specs, ADRs, CI e submodulos.') | Out-Null
        Write-Feito 'vynamp criado (privado)'
    }
}

$origemAtual = (Invoke-Git @('remote', 'get-url', 'origin')).Trim()
Write-Feito "origin atual: $origemAtual"

if ($PSCmdlet.ShouldProcess("$Dono/vynamp", 'trocar o origin do umbrella')) {
    Invoke-Git @('remote', 'set-url', 'origin', "https://github.com/$Dono/vynamp.git") | Out-Null
    Write-Feito "origin agora e https://github.com/$Dono/vynamp.git"
}
Write-Host ''

# ---------------------------------------------------------------------------
# 4. Push do umbrella: develop e main
# ---------------------------------------------------------------------------
Write-Passo '4. publicando o umbrella'

if ($PSCmdlet.ShouldProcess($Raiz, 'push de develop e main para vynamp')) {
    Invoke-Git @('push', '-u', 'origin', 'develop') -Tentativas 6 | Out-Null
    Write-Feito 'develop publicada'

    if (-not (Testa-BranchExiste -Nome 'vynamp' -Branch 'main')) {
        Invoke-Git @('push', 'origin', 'develop:refs/heads/main') -Tentativas 6 | Out-Null
        Write-Feito 'main criada a partir de develop'
    }
    Invoke-Gh @('repo', 'edit', "$Dono/vynamp", '--default-branch', 'develop') | Out-Null
    Write-Feito 'vynamp: branch padrao = develop'
}
Write-Host ''

# ---------------------------------------------------------------------------
# 5. Tags v0.1.0 nos 8 repos de codigo
# ---------------------------------------------------------------------------
Write-Passo "5. criando a tag $TagInicial nos 8 repos de codigo"

$TodosCodigo = @()
foreach ($m in $Modulos)    { $TodosCodigo += $m.nome }
foreach ($m in $Existentes) { $TodosCodigo += $m.nome }

# Repos que nao podem receber tag ainda: sem 'develop' ou sem conteudo compilavel.
$SemTag = @{}

foreach ($nome in $TodosCodigo) {
    if (Testa-TagExiste -Nome $nome -Tag $TagInicial) {
        Write-Feito "${nome}: tag $TagInicial ja existe"
        continue
    }
    if (-not (Testa-BranchExiste -Nome $nome -Branch 'develop')) {
        Write-Aviso "$nome nao tem develop; a tag $TagInicial nao pode ser criada. Verifique o estado do repo."
        $SemTag[$nome] = 'sem branch develop'
        continue
    }
    # O develop do output tem 2 arquivos e nenhum Cargo.toml (D-GH-9). A tag
    # marcaria 'sem produto' num repo que nem compila — o pin fica em develop.
    $temCargo = $false
    $rArq = Invoke-GhComRetry @('api', "repos/$Dono/$nome/contents?ref=develop", '--jq', '.[].name')
    if (-not $rArq.Ok) {
        Write-Aviso "${nome}: nao foi possivel ler o conteudo de develop; a tag fica pendente."
        $SemTag[$nome] = 'conteudo ilegivel'
        continue
    }
    # `\r?$`: o gh devolve as linhas com CRLF no Windows, e com `$` sozinho a
    # comparacao falha para TODOS os repos -- foi o que aconteceu na primeira
    # execucao, com os 8 marcados como 'develop nao tem Cargo.toml' (o decode e
    # o output tem, e os 6 recem-semeados tambem). O `\r` do CRLF fica entre o
    # 'l' e o fim de linha, e `$` em modo multiline aceita antes do '\n', nao
    # antes do '\r'.
    $temCargo = ($rArq.Saida -match '(?m)^Cargo\.toml\r?$')
    if (-not $temCargo) {
        Write-Aviso "${nome}: develop nao tem Cargo.toml; a tag $TagInicial fica pendente e o submodule permanece em develop."
        $SemTag[$nome] = 'develop sem Cargo.toml'
        continue
    }
    if (-not $PSCmdlet.ShouldProcess("$Dono/$nome", "tag $TagInicial")) { continue }

    $rSha = Invoke-GhComRetry @('api', "repos/$Dono/$nome/commits/develop", '--jq', '.sha')
    if (-not $rSha.Ok) { throw "nao foi possivel ler o SHA de develop em ${nome}: $($rSha.Saida)" }
    $sha = $rSha.Saida
    Invoke-Gh @('api', '-X', 'POST', "repos/$Dono/$nome/git/refs", '-f', "ref=refs/tags/$TagInicial", '-f', "sha=$sha") | Out-Null
    Write-Feito "${nome}: $TagInicial -> $($sha.Substring(0,7))"
}
Write-Host ''

# ---------------------------------------------------------------------------
# 6. Submodulos: 8 repos de codigo (fixados na tag) + a referencia (commit)
# ---------------------------------------------------------------------------
Write-Passo '6. adicionando os 9 submodulos'

# O hook guard-branch recusa commit em develop/main, e este passo commita o
# .gitmodules e os ponteiros dos submodulos. Trabalhar na branch e o que o
# contrato do umbrella ja exige para todo passo da FASE 3; o commit entra por
# PR como os passos 30-34, e nao direto em develop.
$BranchPasso = 'chore/claude-fase3-passo-35'
$branchAtual = (Invoke-Git @('branch', '--show-current')).Trim()
if ($branchAtual -ne $BranchPasso) {
    if (Testa-BranchExiste -Nome 'vynamp' -Branch $BranchPasso) {
        Invoke-Git @('fetch', 'origin', $BranchPasso) | Out-Null
        Invoke-Git @('checkout', '-B', $BranchPasso, "origin/$BranchPasso") | Out-Null
    } else {
        Invoke-Git @('checkout', '-B', $BranchPasso) | Out-Null
    }
    Write-Feito "branch de trabalho: $BranchPasso"
} else {
    Write-Feito "ja na branch $BranchPasso"
}

$TodosSub = @()
foreach ($m in $Modulos)    { $TodosSub += @{ nome = $m.nome; caminho = $m.caminho; url = "https://github.com/$Dono/$($m.nome).git"; pin = $TagInicial } }
foreach ($m in $Existentes) { $TodosSub += @{ nome = $m.nome; caminho = $m.caminho; url = $m.url; pin = $TagInicial } }
$TodosSub += @{ nome = $Referencia.nome; caminho = $Referencia.caminho; url = $Referencia.url; pin = $Referencia.commit }

foreach ($s in $TodosSub) {
    $destino = Join-Path $Raiz $s.caminho
    if (Test-Path (Join-Path $destino '.git')) {
        Write-Feito "$($s.nome) ja e submódulo; skip"
        continue
    }
    # O passo 31 cria libs/<x>/LEIA-ME.md; 'git submodule add' exige diretorio vazio.
    if (Test-Path $destino) {
        $itens = @(Get-ChildItem -LiteralPath $destino -Force)
        if ($itens.Count -gt 0) {
            $soLeiaMe = ($itens.Count -eq 1) -and ($itens[0].Name -eq 'LEIA-ME.md')
            if (-not $soLeiaMe) {
                throw "O destino $($s.caminho) nao esta vazio (tem $($itens.Count) itens). 'git submodule add' exige diretorio vazio."
            }
            Remove-Item -LiteralPath $destino -Recurse -Force
        }
    }

    if (-not $PSCmdlet.ShouldProcess($s.caminho, "submodule add $($s.nome)")) { continue }

    # `submodule add` clona pela rede, entao tambem precisa de retry. Um clone
    # interrompido deixa o diretorio com um .git pela metade, e o proximo run
    # veria '.git existe' e pularia -- por isso o diretorio e limpo antes de
    # cada tentativa que nao seja a primeira.
    try {
        Invoke-Git @('submodule', 'add', '--force', $s.url, $s.caminho) -Tentativas 6 | Out-Null
    } catch {
        if (Test-Path $destino) { Remove-Item -LiteralPath $destino -Recurse -Force -ErrorAction SilentlyContinue }
        throw
    }
    Write-Feito "$($s.nome) adicionado em $($s.caminho)"
}
Write-Host ''

# ---------------------------------------------------------------------------
# 7. Fixar cada submodulo no seu pin (tag ou commit)
# ---------------------------------------------------------------------------
Write-Passo '7. fixando os submodulos nos pinos'

foreach ($s in $TodosSub) {
    $destino = Join-Path $Raiz $s.caminho
    if (-not (Test-Path (Join-Path $destino '.git'))) { continue }

    & git -C $destino fetch --tags --quiet origin 2>&1 | Out-Null

    # Repos sem tag (SemTag) ficam em develop, com aviso explicito.
    $usarTag = ($s.pin -eq $TagInicial)
    if ($usarTag -and $SemTag.ContainsKey($s.nome)) {
        $usarTag = $false
        Write-Aviso "$($s.nome) fica em develop (motivo: $($SemTag[$s.nome]))."
    }

    if ($usarTag) {
        & git -C $destino checkout --quiet $TagInicial 2>&1 | Out-Null
        if ($LASTEXITCODE -ne 0) { throw "checkout da tag $TagInicial falhou em $($s.nome)" }
        Write-Feito "$($s.nome): fixado em $TagInicial"
    } elseif ($s.pin -eq $TagInicial) {
        & git -C $destino checkout --quiet origin/develop 2>&1 | Out-Null
        if ($LASTEXITCODE -ne 0) { throw "checkout de develop falhou em $($s.nome)" }
        Write-Feito "$($s.nome): fixado em develop (pin por tag pendente)"
    } else {
        & git -C $destino checkout --quiet $s.pin 2>&1 | Out-Null
        if ($LASTEXITCODE -ne 0) { throw "checkout do commit $($s.pin) falhou em $($s.nome)" }
        Write-Feito "$($s.nome): fixado no commit $($s.pin)"
    }
}
Write-Host ''

# ---------------------------------------------------------------------------
# 8. Commitar e publicar os pinos
# ---------------------------------------------------------------------------
Write-Passo '8. publicando os pinos'

if ($PSCmdlet.ShouldProcess($Raiz, 'commit e push dos pinos')) {
    Invoke-Git @('add', '-A', 'libs', 'apps', 'reference') | Out-Null
    Invoke-Git @('add', '.gitmodules') | Out-Null
    Invoke-Git @('commit', '-q', '-m', "chore(repos): fixa os submodulos na $TagInicial (passo 35)") | Out-Null
    Invoke-Git @('push', '-u', 'origin', $BranchPasso) -Tentativas 6 | Out-Null
    Write-Feito "pinos publicados em $BranchPasso"
}
Write-Host ''

# ---------------------------------------------------------------------------
# 9. Arquivar o vynamp-mediaplayer - ultimo, e so com confirmacao digitada
# ---------------------------------------------------------------------------
Write-Passo '9. arquivando o vynamp-mediaplayer'

if ($SemArquivar) {
    Write-Aviso 'pulado (-SemArquivar): o vynamp-mediaplayer continua ativo'
} else {
    $arquivado = Invoke-Gh @('api', "repos/$Dono/vynamp-mediaplayer", '--jq', '.archived')
    if ($arquivado.Trim() -eq 'true') {
        Write-Feito 'vynamp-mediaplayer ja esta arquivado'
    } else {
        Write-Host ''
        Write-Host '    O vynamp-mediaplayer contem o historico herdado (109 commits, PRs #1-#39).' -ForegroundColor Yellow
        Write-Host '    Ele fica legivel e read-only. Reverter: gh repo unarchive.' -ForegroundColor Yellow
        Write-Host ''
        $resp = Read-Host '    Digite "pode arquivar" para arquivar o vynamp-mediaplayer'
        if ($resp -eq 'pode arquivar') {
            if ($PSCmdlet.ShouldProcess("$Dono/vynamp-mediaplayer", 'arquivar')) {
                Invoke-Gh @('repo', 'archive', "$Dono/vynamp-mediaplayer", '--yes') | Out-Null
                Write-Feito 'vynamp-mediaplayer arquivado'
            }
        } else {
            Write-Aviso 'Confirmacao nao dada: o vynamp-mediaplayer NAO foi arquivado.'
        }
    }
}
Write-Host ''

# ---------------------------------------------------------------------------
# Verificacao
# ---------------------------------------------------------------------------
Write-Passo 'verificacao'

$erros = @()

try {
    $lista = Invoke-Gh @('api', "users/$Dono/repos?per_page=100", '--jq', '.[] | select(.name|test("^(vynamp|vitrola)")) | .name')
    $nomes = @($lista -split "`n" | Where-Object { $_ })
    Write-Feito "$($nomes.Count) repos vynamp*/vitrola no GitHub"
    foreach ($esperado in @('vynamp', 'vynamp-dsp', 'vynamp-decode', 'vynamp-encode', 'vynamp-output', 'vynamp-engine', 'vynamp-player', 'vynamp-converter', 'vynamp-restore', 'vynamp-mvp-reference')) {
        if ($nomes -notcontains $esperado) { $erros += "repo ausente no GitHub: $esperado" }
    }
} catch {
    $erros += "nao foi possivel listar os repos: $($_.Exception.Message)"
}

$status = Invoke-Git @('submodule', 'status')
$linhas = @($status -split "`n" | Where-Object { $_ })
Write-Feito "submodule status: $($linhas.Count) entrada(s)"
if ($linhas.Count -ne 9) { $erros += "esperado 9 submodulos, encontrado $($linhas.Count)" }
foreach ($l in $linhas) {
    if ($l -match '^-') { $erros += "submodulo sujo/nao inicializado: $l" }
    if ($l -notmatch '\(v0\.1\.0\)|\(741316f\)') {
        # Repos com tag pendente ficam em develop: e um aviso, nao um erro.
        $nomeSub = (($l -split '\s+')[-1] -split '/')[-1]
        $marcado = $false
        foreach ($s in $TodosSub) {
            if ($s.caminho.Split('/')[-1] -eq $nomeSub -and $SemTag.ContainsKey($s.nome)) { $marcado = $true }
        }
        if ($marcado) {
            Write-Aviso "submodulo em develop (tag pendente): $l"
        } else {
            $erros += "submodulo nao fixado em tag/commit: $l"
        }
    }
}

foreach ($nome in $TodosCodigo) {
    if (-not (Testa-TagExiste -Nome $nome -Tag $TagInicial)) {
        if ($SemTag.ContainsKey($nome)) {
            Write-Aviso "tag $TagInicial pendente em $nome (motivo: $($SemTag[$nome]))"
        } else {
            $erros += "tag $TagInicial ausente em $nome"
        }
    }
}

foreach ($e in $erros) { Write-Aviso $e }
if ($erros.Count -gt 0) {
    Write-Host ''
    Write-Host '    Verificacao com pendencias. Nada foi excluido.' -ForegroundColor Yellow
} else {
    Write-Host ''
    Write-Host '    Verificacao do passo 35 passou.' -ForegroundColor Green
}

Write-Host ''
Write-Host '    Proximo: confira o diff e commite com a mensagem do 35-github.md.' -ForegroundColor Yellow
Write-Host ''
