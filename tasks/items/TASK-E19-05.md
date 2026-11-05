# TASK-E19-05 — CI e README bilíngue nas libs Rust e no MVP de referência

Versão 0.8.0 · Estado planejado; CI configurado não equivale a CI executado nos runners.

Fase M1 / P1 · Owner engineering-delivery · Esforço inicial 2–6 dias úteis; reestimar.

## Dependências

- [TASK-E19-01](TASK-E19-01.md) — Migrar o CI do repo principal para GitHub Actions

## Entrega concreta

GitHub Actions com gate CI Gate em vynamp-decode e vynamp-output, CI próprio em Node no vynamp-mvp-reference, e README bilíngue EN/PT nos três repos, com Woodpecker removido.

Cada repo é dono do seu CI. `vynamp-decode` e `vynamp-output` são Rust: workflow com fmt, clippy `-D warnings`, test e `cargo doc` com `RUSTDOCFLAGS="-D warnings"` (o output mantém o `cargo doc`), actions fixadas por SHA, toolchain pinado por `rustup` e job `gate` publicando o contexto `CI Gate`. `vynamp-mvp-reference` é **Node**, não Rust: `npm ci`, `npm run check-svg` e `npm run verify`, encadeando o servidor local (`node tools/serve.cjs mvp 8766`) que o `verify` exige. Nenhum deles tem `cargo xtask`, então a branch protection é aplicada pela API do GitHub, não pelo comando do repo principal.

Os READMEs das libs hoje são em inglês e dizem "private repository" — viram bilíngues EN/PT no mesmo arquivo, com o bloco PT acrescentado. O do MVP de referência é honesto sobre ser **referência visual**, não o produto: o player real é `vynamp-mediaplayer`.

Escopo: `D:\Projects\vynamp-decode` e `D:\Projects\vynamp-output` (`.github/workflows/ci.yml`, `.woodpecker/` removido, `README.md`, `SECURITY.md`, `CONTRIBUTING.md`, `LEGAL.md`); `D:\Projects\vynamp-mvp-reference` (`.github/workflows/ci.yml`, `README.md`, `LICENSE`, `SECURITY.md`, `CONTRIBUTING.md`, `LEGAL.md`).

Boundary: não altera código de runtime das libs; não promete cobertura de CI idêntica à do player (as libs não têm `apps/desktop`, então `xtask ci core` não se aplica); não abre os repositórios ao público — a abertura é TASK-E19-06.

## Critérios de aceite

Cada repo publica o contexto CI Gate e reprova falha ou cancelamento; o CI do MVP roda serve e verify encadeados; os READMEs são bilíngues; nenhum workflow referencia Woodpecker.

Detalhamento: `grep -rn "woodpecker" .github/` retorna vazio nos três repos; `gh workflow list --repo jimmykedlyn/vynamp-decode` mostra `ci.yml`; o job `gate` do MVP encadeia o servidor antes do `verify` e falha se o `verify` falhar; `README.md` de cada repo tem seção English e seção Português; nenhum README diz "private repository" nem "(privada; abertura futura)".

[Spec](../../docs/engineering/08-folder-structure.md) · REQ-E19-05 em [matriz](../../docs/product/03-requirements.md).

## Evidência

QA-018, QA-034; registrar commit/digest, comando, ambiente/OS/ABI, outputs e limitações. Instalação de plugin/repo privado/assinatura requer ambiente real; não substituir por mocks/documentos. Quebrar em slices pequenas antes execução, implementar sucesso/erro/cancelamento, revisar pelo outro agente e preservar resultados no PR/docs/evidence.
