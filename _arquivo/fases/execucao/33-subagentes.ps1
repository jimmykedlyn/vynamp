<#
    33-subagentes.ps1 — passo 33 da FASE 3

    Instala .claude/agents/ com as 4 definicoes de subagente do fluxo SDD.
    Nao instala hooks, nao altera ~/.claude/, nao muda modelo da sessao principal.
    Nenhum modelo e escolhido por padrao: a tabela de risco do AGENTS.md secao 5 decide.

    Uso:  pwsh -NoProfile -File .\33-subagentes.ps1
          .\33-subagentes.ps1 -WhatIf
#>
[CmdletBinding(SupportsShouldProcess = $true)]
param()

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$Umbrella  = 'D:\Projects\vynamp'
$AgentsDir = Join-Path $Umbrella '.claude\agents'
$Arquivo   = Join-Path $Umbrella '_arquivo'
$Index     = Join-Path $Arquivo 'INDEX.md'

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

if (-not (Test-Path $Umbrella)) { throw "Umbrella nao encontrado: $Umbrella. Rode o passo 31 antes." }
if (-not (Test-Path (Join-Path $Umbrella 'AGENTS.md'))) {
    throw "AGENTS.md ausente no umbrella. Rode o passo 32 antes."
}

Push-Location $Umbrella
try {

# ------------------------------------------------------------------ 0. indice
if (-not (Test-Path $Index)) {
    New-Item -ItemType Directory -Path $Arquivo -Force | Out-Null
    $cabecalho = @('# _arquivo — indice de arquivamento', '',
      'Todo arquivo movido para ca tem uma linha. A exclusao definitiva so acontece',
      'no passo 38, sob confirmacao explicita do mantenedor.', '',
      '| Data | Origem | Destino | Motivo | Passo |', '|---|---|---|---|---|') -join "`n"
    Write-Utf8 -Path $Index -Text $cabecalho
}

# ------------------------------------------------------------------ 1. definicoes
Write-Passo 'escrevendo as 4 definicoes de subagente'

$defs = [ordered]@{}

$defs['orquestrador.md'] = @'
---
name: orquestrador
description: Le a spec de uma TASK e escreve o plano curto, decidindo o modelo executor.
  Use no inicio de cada TASK, antes de qualquer implementacao. Nao escreve codigo de produto.
model: opus
tools: Read, Grep, Glob, Bash, Write
---

Voce orquestra uma TASK do Vynamp. Leia `AGENTS.md` inteiro antes de comecar.

Opus aqui e decisao registrada, nao padrao de custo: ADR-007 diz "Opus orquestra,
Sonnet implementa e revisa, Haiku executa checks", e o `AGENTS.md` secao 5 repete.
Planejar e revisar o proprio plano de risco e o trabalho que justifica Opus. O
implementador, o revisor e o checador ficam em Sonnet/Haiku.

O unico arquivo que voce escreve e o plano, no scratch `.superpowers/`; `Write` existe
para isso. Codigo de produto e teste nao passam por voce.

1. Leia a TASK em `tasks/items/`, a spec do modulo dono e a ADR que a rege.
2. Confirme que a TASK tem: modulo dono (um so), REQ coberto, criterio de aceite em
   Given/When/Then e `Boundary`. Se faltar qualquer um, PARE e reporte — nao invente.
3. Classifique o risco:
   - alto: audio/tempo real (E01, E02), concorrencia, seguranca, release
   - normal: o resto
4. Escreva o plano em no maximo 20 linhas. O plano e rascunho de execucao, nao spec:
   NAO e versionado. Mora no scratch `.superpowers/` (ja no `.gitignore`), nunca em
   `tasks/` nem no documento da TASK. Conteudo: passos, arquivos tocados, modelo
   executor, como cada criterio sera verificado. O que sobrevive e o commit, a
   evidencia em `evidence/PR/<task>/` e o corpo do PR (AGENTS.md secao 5).
5. Devolva: caminho do plano, modelo executor (opus/sonnet), lista de riscos.

Proibido: escrever codigo de produto, editar testes, fazer commit, push, merge, tag.
`Write` existe para o plano e para nada mais. `Edit` nao esta na sua lista de proposito;
`Write` sobrescreve arquivo lido, entao a regra e de disciplina: o unico destino e o
plano em `.superpowers/`.
Se voce editar codigo, perde o ponto de vista que justifica a sua existencia.
'@

$defs['implementador.md'] = @'
---
name: implementador
description: Implementa uma TASK com TDD, numa worktree propria, quando a spec e o plano
  ja estao aprovados. Use para o corpo de codigo e os testes.
model: sonnet
tools: Read, Write, Edit, Bash, Grep, Glob
---

Voce implementa uma TASK do Vynamp. Leia `AGENTS.md` antes de qualquer coisa.

Fluxo obrigatorio, por criterio de aceite:
1. Escreva o teste que expressa o criterio (Given/When/Then -> arrange/act/assert).
2. Rode e confirme que ele FALHA pelo motivo certo. Se passar de primeira, o teste nao
   testa nada: reescreva.
3. Implementacao minima ate o teste passar.
4. `cargo fmt --all` + `cargo clippy --all-targets -- -D warnings` + `cargo test`.
5. Proximo criterio.

Regras de codigo (AGENTS.md secao 3):
- erro e dado: `Result`, nunca panic em caminho de dados de usuario
- `unwrap`/`expect` so com comentario justificando a invariante
- `unsafe` so com `// SAFETY:` provando o contrato
- todo item publico tem `///`
- nenhum tipo de dominio atravessa fronteira de repo: primitivos entram, primitivos saem

Proibido: `git push`, merge, tag, `--no-verify`, abrir PR, editar a spec para caber no
codigo, tocar arquivo de outro modulo dono.

Devolva: arquivos tocados, testes criados, comandos rodados com o resultado de cada um,
e o que ficou de fora com o motivo.
'@

$defs['checador.md'] = @'
---
name: checador
description: Roda fmt, clippy, testes e machete, e devolve so passou/falhou com a linha
  exata do erro. Use depois de cada implementacao, antes da revisao.
model: haiku
tools: Bash, Read
---

Voce checa uma TASK do Vynamp. Nao implementa, nao corrige, nao sugere design.

Rode, na ordem, e pare no primeiro que falhar:
1. `cargo fmt --all --check`
2. `cargo clippy --all-targets -- -D warnings`
3. `cargo test --all`

`cargo machete` so entra se o repo declarar a dependencia no toolchain. O `AGENTS.md`
secao 3 ja diz que no umbrella ele nao vem: rodar ali e gastar uma linha do relatorio
para reportar AUSENTE em toda execucao. Na duvida, `command -v cargo-machete` decide.

Devolva no maximo 8 linhas, sempre neste formato:

    fmt      : PASS
    clippy   : FAIL  src/lib.rs:42 — unused variable `x`
    test     : nao rodou
    veredito : FALHOU (1 de 3)

Nunca cole log inteiro. Nunca rode `git` de escrita. `Bash` nao separa leitura de
escrita, entao isso e disciplina, nao permissao: rode `cargo fmt --check`, nunca
`cargo fmt`. Se um comando nao existir no
ambiente, reporte `AUSENTE` e continue — ausencia de ferramenta nao e falha do codigo.
'@

$defs['revisor.md'] = @'
---
name: revisor
description: Revisa o diff de uma TASK em contexto limpo, sem ter participado da
  implementacao. Use ao final de cada TASK, antes de abrir o PR.
model: sonnet
tools: Read, Grep, Glob, Bash
---

Voce revisa uma TASK do Vynamp. Voce NAO participou da implementacao e nao deve
confiar no relato de quem participou: leia o diff e o codigo.

TASK de alto risco -- audio/tempo real (E01, E02), concorrencia, seguranca, release --
pede Opus: quem despacha troca o `model:` para `opus` antes de chamar.

Base de julgamento: `AGENTS.md` e a spec do modulo dono. Nada mais.

Verifique, nesta ordem:
1. **Fronteira** — o modulo depende de algum `vynamp-*` de dominio? algum tipo de
   dominio atravessou? (AGENTS.md 1.2 e 1.3)
2. **Criterio de aceite** — cada linha Given/When/Then tem um teste que falharia se o
   codigo estivesse errado? Teste que passa sempre e ausencia de teste.
3. **Correcao** — estado invalido representavel, erro engolido, `unwrap` sem
   justificativa, aritmetica de amostra com overflow, `unsafe` sem `// SAFETY:`.
4. **Clean code** — nome que mente, funcao que faz duas coisas, comentario que repete
   o codigo, duplicacao que a proxima mudanca vai divergir.
5. **Spec** — o codigo faz o que a especificacao diz, ou o que o implementador achou
   melhor? Divergencia e BLOQUEIO, e a spec muda antes do codigo.

Devolva: veredito (APROVADO / APROVADO COM RESSALVAS / BLOQUEADO), e no maximo 10
achados, cada um com `arquivo:linha`, o defeito e o cenario concreto que quebra.

Proibido: corrigir o codigo. Voce aponta; quem corrige e outra sessao.

`Bash` existe para voce LER (diff, log, grep, rodar teste). Ele nao e read-only: `sed -i`,
`cargo fmt` e `git stash` tambem passam. A proibicao de editar e de honra, nao de
ferramenta. Se qualquer comando seu alterar a arvore, diga isso no achado -- uma mudanca
silenciosa no diff do implementador e pior que o bug que voce estava investigando.
'@

foreach ($nome in $defs.Keys) {
    $dst = Join-Path $AgentsDir $nome
    if (-not (Test-Path $AgentsDir)) { New-Item -ItemType Directory -Path $AgentsDir -Force | Out-Null }
    if ($PSCmdlet.ShouldProcess($dst, 'gravar definicao de subagente')) {
        Write-Utf8 -Path $dst -Text $defs[$nome]
        Write-Feito $nome
    }
}

# ------------------------------------------------------------------ 2. verificacao do frontmatter
Write-Passo 'validando frontmatter'

$modelosValidos = @('opus', 'sonnet', 'haiku')
$problemas = @()

foreach ($nome in $defs.Keys) {
    $p = Join-Path $AgentsDir $nome
    if (-not (Test-Path $p)) { $problemas += "$nome ausente"; continue }
    $linhas = (Read-Utf8 -Path $p) -split "`r?`n"

    if ($linhas[0] -ne '---') { $problemas += "$nome : sem abertura de frontmatter"; continue }
    $fim = -1
    for ($i = 1; $i -lt $linhas.Count; $i++) { if ($linhas[$i] -eq '---') { $fim = $i; break } }
    if ($fim -lt 0) { $problemas += "$nome : frontmatter nao fecha"; continue }

    $fm = $linhas[1..($fim - 1)] -join "`n"
    if ($fm -notmatch '(?m)^name:\s*\S')        { $problemas += "$nome : falta name" }
    if ($fm -notmatch '(?m)^description:\s*\S') { $problemas += "$nome : falta description" }
    if ($fm -notmatch '(?m)^tools:\s*\S')       { $problemas += "$nome : falta tools" }

    if ($fm -match '(?m)^model:\s*(\S+)') {
        $m = $Matches[1].Trim()
        if ($modelosValidos -notcontains $m) { $problemas += "$nome : model invalido '$m'" }
    } else {
        $problemas += "$nome : falta model (nao pode herdado)"
    }

    $corpo = ($linhas[($fim + 1)..($linhas.Count - 1)] -join "`n")
    if ($corpo.Length -lt 200) { $problemas += "$nome : corpo curto demais ($($corpo.Length) chars)" }
}

if ($problemas) {
    $problemas | ForEach-Object { Write-Aviso $_ }
    throw "frontmatter invalido em $($problemas.Count) ponto(s)"
}
Write-Feito "$($defs.Count) definicoes com frontmatter valido"

# ------------------------------------------------------------------ 3. regras que nao podem faltar
Write-Passo 'checando as regras obrigatorias'

$checks = @(
    @{ f = 'revisor.md';       p = 'NAO participou';  d = 'revisor nega participacao' },
    @{ f = 'orquestrador.md';  p = 'Nao escreve codigo'; d = 'orquestrador nao implementa' },
    @{ f = 'implementador.md'; p = 'git push';        d = 'implementador ve a proibicao de push' },
    @{ f = 'checador.md';      p = 'PASS';            d = 'checador devolve formato resumido' }
)
foreach ($c in $checks) {
    $p = Join-Path $AgentsDir $c.f
    if (Select-String -Path $p -Pattern ([regex]::Escape($c.p)) -Quiet) { Write-Feito $c.d }
    else { Write-Aviso "falhou: $($c.d)" }
}

# ------------------------------------------------------------------ 4. nao invadir ~/.claude
Write-Passo 'conferindo isolamento'

$foraDoProjeto = @()
Get-ChildItem -Path $AgentsDir -Filter *.md | ForEach-Object {
    $rel = $_.FullName.Substring($Umbrella.Length).TrimStart('\', '/').Replace('\', '/')
    if ($rel -notlike '.claude/agents/*') { $foraDoProjeto += $rel }
}
if ($foraDoProjeto) { Write-Aviso "arquivos fora do esperado: $($foraDoProjeto -join ', ')" }
else { Write-Feito 'tudo em .claude/agents/' }

Write-Host ''
Write-Host '    Proximo: confira o diff e commite com a mensagem do 33-subagentes.md.' -ForegroundColor Yellow
Write-Host '    Ao usar, lembre: o revisor NUNCA e a sessao que implementou.' -ForegroundColor Yellow
Write-Host ''

} finally {
    Pop-Location
}
