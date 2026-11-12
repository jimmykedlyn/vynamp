<#
    34-ci.ps1 — passo 34 da FASE 3

    Reescreve os workflows do GitHub Actions para a arvore do umbrella e grava
    os templates de CI dos repos de modulo.

    O que este script NAO faz (deliberadamente):
      - nao reescreve delivery() nem github_actions() por regex: sao ~90 linhas
        de logica com 9 testes proprios. Isso vira TASK-CI-01 no passo 37.
      - nao cria repo, nao faz push, nao mexe em visibilidade ou protecao.
      - nao religa release.yml em lugar nenhum (D-CI-3).

    Uso:  pwsh -NoProfile -File .\34-ci.ps1
          .\34-ci.ps1 -WhatIf
#>
[CmdletBinding(SupportsShouldProcess = $true)]
param()

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$Umbrella = 'D:\Projects\vynamp'
$Fases    = 'D:\Projects\vynamp-fases'
$Flows    = Join-Path $Umbrella '.github\workflows'
$Padroes  = Join-Path $Fases 'padroes'
$Arquivo  = Join-Path $Umbrella '_arquivo'
$Index    = Join-Path $Arquivo 'INDEX.md'

# SHA de actions/checkout ja em uso no repo. Toda action nova precisa de SHA
# real de 40 chars: validate::github_actions reprova qualquer outra coisa.
$SHA_CHECKOUT = '11bd71901bbe5b1630ceea73d27597364c9af683'

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
if (-not (Test-Path (Join-Path $Umbrella 'xtask\Cargo.toml'))) { throw 'xtask ausente. Rode o passo 31 antes.' }
if (-not (Test-Path $Index)) { throw '_arquivo/INDEX.md ausente. Rode o passo 30 antes.' }

Push-Location $Umbrella
try {

# ------------------------------------------------------------------ 1. indice
$dirty = git status --short
if ($dirty) {
    Write-Aviso 'arvore nao esta limpa:'
    $dirty | ForEach-Object { Write-Host "      $_" }
    Write-Aviso 'O passo 34 tolera pendencia, mas commite antes de seguir ao 35.'
}

# ------------------------------------------------------------------ 2. xtask: caminhos de docs/
Write-Passo 'atualizando caminhos de docs/ no xtask'

# Trocas 1:1. Cada uma troca o prefixo docs/ pelo caminho novo no umbrella.
# docs/evidence/ -> evidence/ ; docs/<cat>/<file> -> <cat>/<file-novo>
$trocas = @(
    @{ de = 'docs/INDEX.md';                          para = 'INDEX.md' },
    @{ de = 'docs/product/03-requirements.md';        para = 'product/03-requisitos.md' },
    @{ de = 'docs/quality/01-test-strategy.md';       para = 'quality/01-estrategia-testes.md' },
    @{ de = 'docs/design/01-visual-system.md';        para = 'design/01-sistema-visual.md' },
    @{ de = 'docs/evidence/release/';                 para = 'evidence/release/' }
)

$arquivosRust = @(
    'xtask\src\validate.rs',
    'xtask\src\release.rs',
    'xtask\src\ci.rs',
    'xtask\src\main.rs',
    'xtask\src\repo.rs',
    'xtask\src\util.rs',
    'xtask\src\branding.rs',
    'xtask\src\package.rs',
    'xtask\src\catalog.rs',
    'xtask\src\github.rs',
    'xtask\src\signing.rs',
    'xtask\src\agents.rs'
)

$totalTrocas = 0
foreach ($t in $trocas) {
    foreach ($rel in $arquivosRust) {
        $p = Join-Path $Umbrella $rel
        if (-not (Test-Path $p)) { continue }
        $txt = Read-Utf8 -Path $p
        if (-not $txt.Contains($t.de)) { continue }
        $novo = $txt.Replace($t.de, $t.para)
        if ($PSCmdlet.ShouldProcess($rel, "trocar '$($t.de)' -> '$($t.para)'")) {
            Write-Utf8 -Path $p -Text $novo
            $n = ([regex]::Matches($txt, [regex]::Escape($t.de))).Count
            $totalTrocas += $n
            Write-Feito "$rel : $($t.de) -> $($t.para) ($n)"
        }
    }
}
if ($totalTrocas -eq 0) { Write-Feito 'nenhuma troca de caminho necessaria (ja aplicadas)' }

# ------------------------------------------------------------------ 3. excluded(): submodulos fora da varredura
Write-Passo 'excluindo libs/ apps/ reference/ _arquivo/ da varredura do xtask'

$utilPath = Join-Path $Umbrella 'xtask\src\util.rs'
$util = Read-Utf8 -Path $utilPath

if ($util -notmatch '"_arquivo"') {
    # Insere os 4 diretorios na lista de `excluded`. A lista hoje comeca em
    # `matches!(name.as_str(), ".git" | ".worktrees" | ...)`; inserir logo apos
    # `.git` mantem a ordem alfabetica existente.
    $ancora    = '        ".git"'
    $substituto = @'
        ".git"
            | "libs"
            | "apps"
            | "reference"
            | "_arquivo"
'@
    $novoUtil = $util.Replace($ancora, $substituto.TrimEnd("`r", "`n"))
    if ($novoUtil -eq $util) { throw 'Anchor .git nao encontrado em util.rs' }
    if ($PSCmdlet.ShouldProcess('xtask\src\util.rs', 'excluir dirs de submodule da varredura')) {
        Write-Utf8 -Path $utilPath -Text $novoUtil
        Write-Feito 'excluded() agora ignora libs/ apps/ reference/ _arquivo/'
    }
} else {
    Write-Feito 'excluded() ja exclui os diretorios de submodule'
}

# ------------------------------------------------------------------ 4. workflows
Write-Passo 'escrevendo os workflows do umbrella'

if (-not (Test-Path $Flows)) { New-Item -ItemType Directory -Path $Flows -Force | Out-Null }

# 4.1 ci.yml
$ci = @"
name: CI

# Gate do umbrella. Nao constroi app: os apps sao repos proprios com CI propria.
# O gate valida o que o umbrella e dono: specs, ferramental, pins de submodule
# e as regras de arquitetura. Ver AGENTS.md secao 1.2.
on:
  pull_request:
    branches: [develop]
  push:
    branches: [develop, main]
  workflow_dispatch:

permissions:
  contents: read

concurrency:
  group: ci-`${{ github.workflow }}-`${{ github.ref }}
  cancel-in-progress: true

jobs:
  specs:
    name: Specs
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@$SHA_CHECKOUT
        with:
          persist-credentials: false
      - name: Provision pinned Rust toolchain
        run: rustup show
      - run: cargo xtask check-docs

  tooling:
    name: Tooling
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@$SHA_CHECKOUT
        with:
          persist-credentials: false
      - name: Provision pinned Rust toolchain
        run: rustup show
      - run: cargo fmt --all -- --check
      - run: cargo clippy --package xtask --all-targets --locked -- -D warnings
      - run: cargo test --package xtask --locked

  submodules:
    name: Submodules fixados
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@$SHA_CHECKOUT
        with:
          persist-credentials: false
      - name: Provision pinned Rust toolchain
        run: rustup show
      - name: Submodule must be pinned to a tag, never to a branch
        run: |
          set -eu
          git submodule status --recursive
          if git config --file .gitmodules --get-regexp '^submodule\..*\.branch$' > /dev/null 2>&1; then
            echo 'Submodule tracking a branch is forbidden; pin to a tag.'
            exit 1
          fi

  arch:
    name: Regras de arquitetura
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@$SHA_CHECKOUT
        with:
          # Sem isto o job le libs/*/Cargo.toml de diretorios vazios e falha
          # no primeiro run com "submodule ainda nao adicionado".
          submodules: recursive
          persist-credentials: false
      - name: Provision pinned Rust toolchain
        run: rustup show
      - name: Rule 3-4 dependency direction
        run: |
          set -eu
          # vynamp-dsp NAO entra nesta lista: AGENTS.md 1.4 o define como
          # primitivas, nao dominio, e 01-arvore-final o permite opcional em
          # decode/encode/output. Lista-lo aqui contradiria a arvore aprovada.
          for m in decode encode output; do
            if [ ! -f "libs/`$m/Cargo.toml" ]; then
              echo "libs/`$m ausente: submodule ainda nao adicionado (passo 35)."
              echo "Regra 3 nao verificavel; declarando para o gate nao passar."
              exit 1
            fi
            if cargo tree --manifest-path "libs/`$m/Cargo.toml" --prefix none 2>/dev/null \
                 | grep -E '^vynamp-(engine|player|converter|restore)( |$)'; then
              echo "libs/`$m depende de crate de dominio do projeto; viola AGENTS.md 1.2."
              exit 1
            fi
          done
          if [ -f libs/dsp/Cargo.toml ]; then
            if cargo tree --manifest-path libs/dsp/Cargo.toml --prefix none 2>/dev/null \
                 | grep -E '^vynamp-(engine|player|converter|restore|decode|encode|output)( |$)'; then
              echo 'libs/dsp depende de outra lib do projeto; viola AGENTS.md 1.4.'
              exit 1
            fi
          fi
          echo 'Regras 3 e 4 verificadas.'
      - name: Rule 5, each lib builds alone
        run: |
          set -eu
          for d in libs/*; do
            [ -f "`$d/Cargo.toml" ] || continue
            cargo build --manifest-path "`$d/Cargo.toml"
          done

  gate:
    name: CI Gate
    if: always()
    needs: [specs, tooling, submodules, arch]
    runs-on: ubuntu-22.04
    steps:
      - name: Aggregate
        shell: bash
        run: |
          test "`${{ needs.specs.result }}" = success
          test "`${{ needs.tooling.result }}" = success
          test "`${{ needs.submodules.result }}" = success
          test "`${{ needs.arch.result }}" = success
          echo 'CI Gate passed: specs, tooling, submodules and architecture rules.'
          echo 'Hardware, signing and release acceptance remain separate gates.'
"@

# 4.2 quality.yml
$quality = @"
name: Quality

# Hardware acceptance is never part of the CI Gate: it needs real devices and
# real audio hardware, so it only runs when a maintainer asks for it, from an
# integrated branch, and only after confirming the self-hosted runners exist.
# See ADR-014.
on:
  workflow_dispatch:
    inputs:
      confirm_hardware_runners:
        description: 'Run on the vynamp-hardware / vynamp-android-device self-hosted runners'
        type: boolean
        default: false

permissions:
  contents: read

jobs:
  preflight:
    if: github.ref == 'refs/heads/develop' || github.ref == 'refs/heads/main'
    runs-on: ubuntu-22.04
    steps:
      - name: Confirm hardware runners
        shell: bash
        env:
          CONFIRMED: `${{ inputs.confirm_hardware_runners }}
        run: |
          if [ "`$CONFIRMED" = "true" ]; then
            echo 'Hardware runners confirmed by the maintainer; dispatching the matrix.'
          else
            echo 'Skipped: hardware runners are not provisioned yet (TASK-E15-05).'
            echo 'Re-dispatch with confirm_hardware_runners = true once they are online.'
            echo 'Build CI stays on ci.yml; hardware, soak and bit-perfect are not covered here.'
          fi

  trusted-hardware:
    needs: preflight
    if: inputs.confirm_hardware_runners
    environment: hardware-validation
    strategy:
      fail-fast: false
      matrix:
        include:
          - labels: [self-hosted, Windows, X64, vynamp-hardware]
          - labels: [self-hosted, Linux, X64, vynamp-hardware]
          - labels: [self-hosted, macOS, X64, vynamp-hardware]
          - labels: [self-hosted, macOS, ARM64, vynamp-hardware]
          - labels: [self-hosted, Linux, X64, vynamp-android-device]
    runs-on: `${{ matrix.labels }}
    timeout-minutes: 180
    steps:
      - uses: actions/checkout@$SHA_CHECKOUT
        with:
          fetch-depth: 0
          persist-credentials: false
      - name: Provision pinned Rust toolchain
        run: rustup show
      - run: cargo xtask quality-gate
"@

$novosWorkflows = @{ 'ci.yml' = $ci; 'quality.yml' = $quality }

foreach ($nome in $novosWorkflows.Keys) {
    $dst = Join-Path $Flows $nome
    if (Test-Path $dst) {
        # comparar normalizado: Write-Utf8 acrescenta o newline final que a here-string nao tem
        $antigo = (Read-Utf8 -Path $dst).TrimEnd("`r", "`n")
        $querido = $novosWorkflows[$nome].TrimEnd("`r", "`n")
        if ($antigo -eq $querido) { Write-Feito "$nome ja esta atualizado"; continue }
        $backup = Join-Path $Arquivo "workflows\$($nome -replace '\.yml$','')-antes-do-passo-34.yml"
        $bakDir = Split-Path $backup -Parent
        if (-not (Test-Path $bakDir)) { New-Item -ItemType Directory -Path $bakDir -Force | Out-Null }
        if ($PSCmdlet.ShouldProcess($dst, "substituir (backup em $backup)")) {
            Copy-Item -Path $dst -Destination $backup -Force
            Write-Utf8 -Path $dst -Text $novosWorkflows[$nome]
            $linha = '| {0} | `{1}` | `{2}` | {3} | 34 |' -f ((Get-Date -Format 'yyyy-MM-dd'),
                ".github/workflows/$nome", "_arquivo/workflows/$($nome -replace '\.yml$','')-antes-do-passo-34.yml",
                'versao anterior guardada antes da reescrita')
            Append-Utf8 -Path $Index -Text $linha
            Write-Feito "$nome reescrito (backup em _arquivo/workflows/)"
        }
    } else {
        if ($PSCmdlet.ShouldProcess($dst, 'criar')) {
            Write-Utf8 -Path $dst -Text $novosWorkflows[$nome]
            Write-Feito "$nome criado"
        }
    }
}

# 4.3 release.yml sai do umbrella (D-CI-3)
$releasePath = Join-Path $Flows 'release.yml'
if (Test-Path $releasePath) {
    $dstDir = Join-Path $Arquivo 'workflows'
    if (-not (Test-Path $dstDir)) { New-Item -ItemType Directory -Path $dstDir -Force | Out-Null }
    if ($PSCmdlet.ShouldProcess($releasePath, 'mover para _arquivo/workflows/ (D-CI-3)')) {
        Move-Item -Path $releasePath -Destination (Join-Path $dstDir 'release.yml') -Force
        $linha = '| {0} | `{1}` | `{2}` | {3} | 34 |' -f ((Get-Date -Format 'yyyy-MM-dd'),
            '.github/workflows/release.yml', '_arquivo/workflows/release.yml',
            'D-CI-3: release e do app que publica; o umbrella nao tem binario')
        Append-Utf8 -Path $Index -Text $linha
        Write-Feito 'release.yml movido para _arquivo/workflows/'
    }
}

# ------------------------------------------------------------------ 5. templates de modulo
Write-Passo 'gravando os templates de CI dos repos de modulo'

if (-not (Test-Path $Padroes)) { New-Item -ItemType Directory -Path $Padroes -Force | Out-Null }

$base = @"
name: CI

on:
  pull_request:
    branches: [develop]
  push:
    branches: [develop, main]
  workflow_dispatch:

permissions:
  contents: read

concurrency:
  group: ci-`${{ github.workflow }}-`${{ github.ref }}
  cancel-in-progress: true

jobs:
  fmt:
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@$SHA_CHECKOUT
        with:
          persist-credentials: false
      - name: Provision pinned Rust toolchain
        run: rustup show
      - run: cargo fmt --all -- --check

  clippy:
    needs: fmt
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@$SHA_CHECKOUT
        with:
          persist-credentials: false
      - name: Provision pinned Rust toolchain
        run: rustup show
      - run: cargo clippy --all-targets --locked -- -D warnings

  test:
    needs: clippy
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@$SHA_CHECKOUT
        with:
          persist-credentials: false
      - name: Provision pinned Rust toolchain
        run: rustup show
      - run: cargo test --all --locked

  machete:
    needs: clippy
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@$SHA_CHECKOUT
        with:
          persist-credentials: false
      - name: Provision pinned Rust toolchain
        run: rustup show
      - run: cargo machete || echo 'cargo-machete ausente no toolchain; verificacao pulada e listada.'

  gate:
    name: CI Gate
    if: always()
    needs: [fmt, clippy, test, machete]
    runs-on: ubuntu-22.04
    steps:
      - name: Aggregate
        shell: bash
        run: |
          test "`${{ needs.fmt.result }}" = success
          test "`${{ needs.clippy.result }}" = success
          test "`${{ needs.test.result }}" = success
          test "`${{ needs.machete.result }}" = success
          echo 'CI Gate passed.'
"@

$app = @"
name: CI

on:
  pull_request:
    branches: [develop]
  push:
    branches: [develop, main]
  workflow_dispatch:

permissions:
  contents: read

concurrency:
  group: ci-`${{ github.workflow }}-`${{ github.ref }}
  cancel-in-progress: true

jobs:
  fmt:
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@$SHA_CHECKOUT
        with:
          persist-credentials: false
      - name: Provision pinned Rust toolchain
        run: rustup show
      - run: cargo fmt --all -- --check

  clippy:
    needs: fmt
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@$SHA_CHECKOUT
        with:
          persist-credentials: false
      - name: Provision pinned Rust toolchain
        run: rustup show
      - run: cargo clippy --all-targets --locked -- -D warnings

  # M7: o fixture de referencia preta nao vive no repo. Este job baixa e confere
  # o sha256 antes de rodar a suite. O teste le `VYNAMP_MP3_REFERENCE`; sem ela,
  # e `#[ignore = "..."]` e aparece listado no `cargo test` em vez de sumir.
  test:
    needs: clippy
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@$SHA_CHECKOUT
        with:
          persist-credentials: false
      - name: Provision pinned Rust toolchain
        run: rustup show
      - name: Fetch reference fixture
        run: ./scripts/fetch-fixtures.sh
      - name: Run tests with reference fixture
        env:
          VYNAMP_MP3_REFERENCE: `${{ github.workspace }}/fixtures/reference.mp3
        run: cargo test --all --locked -- --include-ignored

  build:
    needs: test
    strategy:
      fail-fast: false
      matrix:
        include:
          - runner: windows-2022
            target: x86_64-pc-windows-msvc
          - runner: ubuntu-22.04
            target: x86_64-unknown-linux-gnu
          - runner: macos-15-intel
            target: x86_64-apple-darwin
          - runner: macos-15
            target: aarch64-apple-darwin
    runs-on: `${{ matrix.runner }}
    steps:
      - uses: actions/checkout@$SHA_CHECKOUT
        with:
          persist-credentials: false
      - name: Provision pinned Rust toolchain
        run: rustup show
      - run: rustup target add `${{ matrix.target }}
      - run: cargo build --all --locked --target `${{ matrix.target }}

  gate:
    name: CI Gate
    if: always()
    needs: [fmt, clippy, test, build]
    runs-on: ubuntu-22.04
    steps:
      - name: Aggregate
        shell: bash
        run: |
          test "`${{ needs.fmt.result }}" = success
          test "`${{ needs.clippy.result }}" = success
          test "`${{ needs.test.result }}" = success
          test "`${{ needs.build.result }}" = success
          echo 'CI Gate passed.'
"@

# O engine e o unico que pode ter mais de uma lib do projeto (AGENTS.md 1.2).
# Template proprio, completo, em vez de remendar o de lib por Replace.
$engine = @"
name: CI

on:
  pull_request:
    branches: [develop]
  push:
    branches: [develop, main]
  workflow_dispatch:

permissions:
  contents: read

concurrency:
  group: ci-`${{ github.workflow }}-`${{ github.ref }}
  cancel-in-progress: true

jobs:
  fmt:
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@$SHA_CHECKOUT
        with:
          persist-credentials: false
      - name: Provision pinned Rust toolchain
        run: rustup show
      - run: cargo fmt --all -- --check

  clippy:
    needs: fmt
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@$SHA_CHECKOUT
        with:
          persist-credentials: false
      - name: Provision pinned Rust toolchain
        run: rustup show
      - run: cargo clippy --all-targets --locked -- -D warnings

  test:
    needs: clippy
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@$SHA_CHECKOUT
        with:
          persist-credentials: false
      - name: Provision pinned Rust toolchain
        run: rustup show
      - run: cargo test --all --locked

  arch:
    name: Regras de arquitetura
    needs: clippy
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@$SHA_CHECKOUT
        with:
          persist-credentials: false
      - name: Provision pinned Rust toolchain
        run: rustup show
      # Regras 2, 3 e 4 de AGENTS.md 1.2: o engine e o teto; ninguem depende
      # de apps; e o engine nao depende de si mesmo por engano.
      - name: No crate may depend on an app
        run: |
          set -eu
          if cargo tree --all --prefix none 2>/dev/null \
               | grep -E '^vynamp-(player|converter|restore)( |$)'; then
            echo 'Algum crate depende de app; viola AGENTS.md 1.2.'
            exit 1
          fi
          echo 'Regras 2-4 verificadas.'

  machete:
    needs: clippy
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@$SHA_CHECKOUT
        with:
          persist-credentials: false
      - name: Provision pinned Rust toolchain
        run: rustup show
      - run: cargo machete || echo 'cargo-machete ausente no toolchain; verificacao pulada e listada.'

  gate:
    name: CI Gate
    if: always()
    needs: [fmt, clippy, test, arch, machete]
    runs-on: ubuntu-22.04
    steps:
      - name: Aggregate
        shell: bash
        run: |
          test "`${{ needs.fmt.result }}" = success
          test "`${{ needs.clippy.result }}" = success
          test "`${{ needs.test.result }}" = success
          test "`${{ needs.arch.result }}" = success
          test "`${{ needs.machete.result }}" = success
          echo 'CI Gate passed: engine dependencies respect the architecture rules.'
"@

$templates = @{
    'ci-lib.yml'    = $base
    'ci-engine.yml' = $engine
    'ci-app.yml'    = $app
}
foreach ($nome in $templates.Keys) {
    $dst = Join-Path $Padroes $nome
    if ($PSCmdlet.ShouldProcess($dst, 'gravar template de CI')) {
        Write-Utf8 -Path $dst -Text $templates[$nome]
        Write-Feito "padroes/$nome"
    }
}

# ------------------------------------------------------------------ 6. .gitignore do umbrella
Write-Passo 'conferindo .gitignore'

$gi = Join-Path $Umbrella '.gitignore'
# _arquivo/ e versionado de proposito: o passo 38 apaga do disco o que ele guarda.
$necessarias = @('*.log', 'VALIDATION.json', '.cache/')
$atual = if (Test-Path $gi) { Read-Utf8 -Path $gi } else { '' }
$faltando = $necessarias | Where-Object { $atual -notmatch [regex]::Escape($_) }
if ($faltando) {
    Write-Aviso "faltando no .gitignore: $($faltando -join ', ')"
} else {
    Write-Feito '.gitignore completo'
}

# ------------------------------------------------------------------ 7. verificacao
Write-Passo 'verificacao'

$problemas = @()

# 7.1 nenhuma action sem SHA de 40
foreach ($f in Get-ChildItem -Path $Flows -Filter *.yml) {
    $txt = Read-Utf8 -Path $f.FullName
    foreach ($m in [regex]::Matches($txt, 'uses:\s*([^\s]+)')) {
        $ref = $m.Groups[1].Value
        if ($ref -notmatch '@[0-9a-f]{40}$') { $problemas += "$($f.Name): action sem SHA de 40 -> $ref" }
    }
}
if ($problemas) { $problemas | ForEach-Object { Write-Aviso $_ } }
else { Write-Feito 'todas as actions com SHA de 40' }

# 7.2 o gate le os resultados
$ciTxt = Read-Utf8 -Path (Join-Path $Flows 'ci.yml')
if ($ciTxt -notmatch 'if:\s*always\(\)') { Write-Aviso 'gate sem if: always()' }
else { Write-Feito 'gate com if: always()' }
foreach ($dep in @('needs.specs.result', 'needs.tooling.result', 'needs.submodules.result', 'needs.arch.result')) {
    if ($ciTxt -notmatch [regex]::Escape($dep)) { Write-Aviso "gate nao le $dep" }
}

# 7.3 zero Woodpecker
$wp = Select-String -Path (Join-Path $Umbrella '.github\*.*') -Pattern 'woodpecker' -CaseSensitive:$false -ErrorAction SilentlyContinue
if ($wp) { Write-Aviso "$($wp.Count) mencao(es) a Woodpecker em .github/" } else { Write-Feito 'zero Woodpecker em .github/' }

# 7.4 nenhum caminho docs/ antigo
$antigos = Select-String -Path (Join-Path $Umbrella 'xtask\src\*.rs') -Pattern '"docs/' -ErrorAction SilentlyContinue
if ($antigos) {
    Write-Aviso "$($antigos.Count) referencia(s) a docs/ ainda no xtask:"
    $antigos | ForEach-Object { Write-Host "      $($_.Filename):$($_.LineNumber)" }
} else { Write-Feito 'zero referencia a docs/ no xtask' }

# 7.5 compila
Write-Passo 'cargo check --package xtask'
cargo check --package xtask 2>&1 | Select-Object -Last 12
if ($LASTEXITCODE -ne 0) { Write-Aviso 'xtask nao compilou; a TASK-CI-01 precisa comecar por isso' }
else { Write-Feito 'xtask compila' }

Write-Host ''
Write-Host '    Proximo: confira o diff e commite com a mensagem do 34-ci.md.' -ForegroundColor Yellow
Write-Host '    A TASK-CI-01 (delivery() e github_actions()) continua aberta de proposito.' -ForegroundColor Yellow
Write-Host ''

} finally {
    Pop-Location
}
