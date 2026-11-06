# Pull request — Vynamp

## Resultado

Problema e comportamento final:

O mantenedor decidiu, em 2026-11-05, que os dois arquivos de skin da raiz (`SKIN-REQUIREMENTS-CODEX…` e `SKIN-REQUIREMENTS-GPT-MVP-OBRIGATORIO-v1.2`) **não são fonte de requisito**. A UI nativa se baseia no MVP web (`vynamp-mvp-reference`). O escopo original era o inverso — absorver as "lacunas" dos SKIN nas specs; com a decisão não há lacuna a herdar.

Este PR registra a decisão e limpa as citações que ainda davam autoridade aos dois documentos. Nenhuma alteração de código, dependência, workflow ou backlog. Contagens seguem **191 requisitos / 191 tarefas / 19 épicos**.

A decisão registrada em [`docs/product/05-decisions.md`](../../../product/05-decisions.md):

- **D-MVP-01** — o MVP web é a fonte única da UI nativa; os SKIN não são fonte de requisito.
- **D-MVP-02** — só herda o comportamento que o código do MVP prova; o que os SKIN afirmavam e o MVP não tem **não** entra por herança.
- **D-MVP-03** — nenhuma TASK, REQ ou marco nasce dos SKIN.
- **D-MVP-04** — onde o repositório e o MVP divergem **em número**, vale o MVP. Caso atual: VU-INT fala em 12 medidores (número da rev.15) e o pin rev.38 tem **16, todos selecionáveis**.

Duas tabelas acompanham: as divergências encerradas por esta decisão, e os seis comportamentos que os SKIN atribuíam ao produto e o MVP não tem.

Nota de propriedade factual: os SKIN eram **consumidores** deste repositório — seus próprios changelogs citavam `DSP-UX`, `VU-INT` e `LYR` como documentos canônicos daqui. Manter as duas fontes produzia duas especificações para o mesmo comportamento.

TASK / REQ / QA:

Nenhuma. Trabalho de spec, sem TASK id — mesmo padrão de `chore-specs-vynamp-restore` e `chore-specs-shell-integration`. Não altera requisito, TASK nem marco (D-MVP-03).

Plano Superpowers e evidência de execução:

[`docs/evidence/PR/chore-specs-skin-gaps/README.md`](README.md) — método de verificação, as oito afirmações conferidas uma a uma contra o código do pin, e as limitações declaradas.

## Verificação

Comandos, resultados, ambientes/fixtures e limitações:

| Comando | Resultado |
|---|---|
| `cargo xtask check-docs` | `result: passed`, `errors: []` — 350 arquivos Markdown, 191 requisitos, 191 tarefas, 19 épicos |
| `cargo xtask check-delivery` | `result: passed`, `errors: []` |
| `cargo fmt --all --check` | sem diff |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | `No issues found` |
| `cargo test --workspace --locked` | 52 + 34 testes, 0 falhas |

Método: oito afirmações atribuídas aos SKIN foram conferidas contra o código do MVP em `vynamp-mvp-reference@0d9860b` (rev.38), quatro delas por subagentes de verificação com estágio adversarial de refutação. O resultado por afirmação — inclusive as três que **não** existem no MVP ("0 VU = −18 dBFS ajustável", "slot de 28 DIP", "intensidade 0 = bypass da seção") e as que **divergem** (presets da Vitrola; atalho no inspector) — está na evidência.

Limitações declaradas:

- A verificação do MVP é **de código, não de comportamento em execução**: nenhum navegador foi aberto. "Existe" significa "o código implementa"; "não existe" significa "não há handler, constante nem CSS que o faça".
- Quatro das oito afirmações ficaram sem o estágio adversarial automatizado (workflow interrompido) e foram conferidas diretamente, com o mesmo critério.
- Medição de latência, CPU e qualidade de áudio no EQ continua não executada aqui.
- Este PR não altera visibilidade, proteções, assinatura nem plano — isso é da TASK-E19-06, por decisão da ADR-014.

O que este PR deliberadamente **não** faz:

- **Não reescreve a matriz VU-INT-06** (144 combinações = 12 skins × 12 VUs). Atualizá-la é trabalho da TASK-E02-07; D-MVP-04 registra a divergência sem tocar na matriz.
- **Não decide** adotar "0 VU ajustável" nem "slot de 28 DIP" — se desejados, é decisão nova.
- **Não altera a evidência histórica** `chore-dsp-eq-docs/summary.md`, que registra o que foi feito naquele PR quando os SKIN ainda existiam.

Revisão técnica (subagente Sonnet/Opus), commit revisado e achados:

- [ ] Worktree e owner próprios.
- [ ] Superpowers carregado; teste útil e verificação executados.
- [ ] Sucesso/erro/cancelamento e acessibilidade aplicáveis cobertos.
- [ ] Docs/contracts/notices atualizados; nenhuma credencial no diff.
- [ ] CI Gate (GitHub Actions) aprovado; hardware e release assinado continuam gates separados.

- [ ] Revisão técnica independente — pendente.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
