# Tarefa — reescrever as specs com base nas libs próprias e no repositório do MVP

Leia antes: `AGENTS.md`, `STATE.md`, `ROADMAP-rodadas.md`. Pré-requisito: PR #24 (remoção do MVP do repo) integrado; se não estiver, pare e avise.

## Objetivo

1. **Áudio:** as specs passam a descrever a pilha própria (vynamp-decode, vynamp-output, vitrola) no lugar de crates de terceiros (`cpal`, `symphonia`, `rubato`, `lofty`).
2. **UI:** as specs passam a referenciar o MVP no repositório `vynamp-mvp-reference` (rev.38). Nenhum arquivo do MVP volta para este repositório.

## Fontes de verdade (ler, não inventar)

| Repositório | Caminho local | O que usar |
|---|---|---|
| vynamp-decode | `D:\Projects\vynamp-decode` | README, `docs/` (design e planos 01–05), API pública real em `develop` |
| vynamp-output | `D:\Projects\vynamp-output` | README, `docs/` (design aprovado 2026-10-25, planos 01–05), PR #1 |
| vitrola | `D:\Projects\vitrola` | README, `docs/superpowers/` (design aprovado) |
| vynamp-mvp-reference | `D:\Projects\vynamp-mvp-reference` | `README.md`, `mvp/` (rev.38, 16 medidores selecionáveis), `out/`, `tools/`; branch `history/chatgpt-mvp` só como histórico |

Use o commit de cada repositório como referência fixa (registre o SHA na spec). Onde uma lib ainda não tem a API (plano não executado), escreva "planejado no plano NN de <lib>", nunca uma API inventada.

## Inventário inicial (repo em `c091b27`)

- Terceiros: `cpal`, `symphonia`, `rubato` e `lofty` aparecem em `docs/architecture/02-stack.md`, `adr/ADR-002-audio-core.md`, `docs/audio/02-formats.md`, `04-outputs-hires.md`, `05-platform-backends.md`, `09-dsp-catalog.md`, `docs/product/02-reference-benchmark.md` e `05-decisions.md`. Use `git grep -n -i "cpal\|symphonia\|rubato\|lofty"` para a lista completa.
- MVP:
  - `docs/design/06-mvp-reference.md` ainda descreve a rev.37, com 9 medidores selecionáveis e o pacote dentro do repo.
  - TASK-E02-12..16, `tasks/backlog.json`, REQ-E02-11..16 em `docs/product/03-requirements.md` e QA-034 em `docs/quality/01-test-strategy.md` também se referem ao MVP.
  - Menções a `interactive/` aparecem em `docs/audio/03-dsp.md`, `08-equalizers.md`, `docs/product/05-decisions.md` e `docs/quality/05-dsp-test-matrix.md`.

## Regras

- Não reescreva a história: TASKs integradas (E02-11, E15-01..04, E11-01) e evidências de PR ficam como estão; só links quebrados viram texto.
- Troca de pilha de áudio = ADR nova que substitui as partes de ADR-002 (não editar a decisão antiga em silêncio). Atualize `02-stack.md`, GLOSSARY, INDEX e CHANGELOG, e incremente a versão de cada doc alterado.
- Lacuna que nenhuma lib própria cobre (ex.: resampler, metadados/tags antes do plano 04 do decode, backends iOS/Android) é listada numa tabela com a lib que vai cobri-la e o plano. Não escolha crate de terceiros nem crie lib nova sem perguntar ao usuário.
- Referência ao MVP: URL do repositório, SHA fixo e caminho dentro dele (`mvp/...` ou `out/...`). Não copie arquivos, SVGs ou JSON do MVP para cá.
- Separe em 2 PRs (serializados, pois mexem nos índices): (1) pilha de áudio; (2) referências ao MVP. Use worktree própria, revisão Sonnet de contexto limpo, `cargo xtask check-docs` e `check-delivery`, CI Gate verde e entrega com "aprove e faça o merge commit pelo GitHub". Não faça merge.

## Perguntar ao usuário antes de fechar o PR 2

- Como o CI (Woodpecker) e o harness E02-16 obtêm o pacote de referência que agora está em outro repositório privado: clone no SHA fixo com chave de deploy/token, ou artefato publicado. A decisão afeta segurança/credenciais.

## Pronto quando

- `git grep -i "cpal\|symphonia\|rubato\|lofty"` em `docs`/`tasks` só acha a ADR antiga, a nova ADR e o histórico.
- Nenhuma spec cita `assets/reference/mvp`, `interactive/` ou rev.37 como fonte atual.
- Toda TASK de UI pendente aponta para `vynamp-mvp-reference@<SHA>`.
- `STATE.md` atualizado (≤ 2 KB) com os dois PRs.
