# docs: o MVP web é a fonte única da UI nativa; citações aos SKIN descartadas

## O que muda

O mantenedor decidiu, em 2026-11-05, que os dois arquivos de skin da raiz (`SKIN-REQUIREMENTS-CODEX…` e `SKIN-REQUIREMENTS-GPT-MVP-OBRIGATORIO-v1.2`) **não são fonte de requisito**. A UI nativa se baseia no MVP web (`vynamp-mvp-reference`). Este PR registra a decisão e limpa as citações que ainda davam autoridade aos dois documentos.

O escopo original era o inverso — absorver as "lacunas" dos SKIN nas specs. Com a decisão, não há lacunas a herdar: elas deixam de existir como requisito.

Nenhuma alteração de código, dependência, workflow ou backlog. Contagens seguem **191 requisitos / 191 tarefas / 19 épicos**.

## Decisão registrada

Nova seção em [`docs/product/05-decisions.md`](../../../product/05-decisions.md):

- **D-MVP-01** — o MVP web é a fonte única da UI nativa; os SKIN não são fonte de requisito.
- **D-MVP-02** — só herda o comportamento que o código do MVP prova; o que os SKIN afirmavam e o MVP não tem **não** entra por herança.
- **D-MVP-03** — nenhuma TASK, REQ ou marco nasce dos SKIN.
- **D-MVP-04** — onde o repositório e o MVP divergem **em número**, vale o MVP. Caso atual: VU-INT fala em 12 medidores (número da rev.15), o pin rev.38 tem **16, todos selecionáveis**.

Duas tabelas acompanham: as divergências encerradas por esta decisão, e os seis comportamentos que os SKIN atribuíam ao produto e o MVP não tem.

## Propriedade factual

Os SKIN eram **consumidores** deste repositório: seus próprios changelogs citavam `DSP-UX`, `VU-INT` e `LYR` como documentos canônicos daqui. Manter as duas fontes produzia duas especificações para o mesmo comportamento.

## Método: verificar antes de re-apontar

Oito afirmações atribuídas aos SKIN foram conferidas contra o código do MVP em `vynamp-mvp-reference@0d9860b` (rev.38), quatro delas por subagentes de verificação com estágio adversarial. Resultado por afirmação — inclusive as três que **não** existem no MVP ("0 VU = −18 dBFS ajustável", "slot de 28 DIP", "intensidade 0 = bypass da seção") e as que **divergem** (presets da Vitrola; atalho no inspector). Detalhe e caminhos dos arquivos em [`docs/evidence/PR/chore-specs-skin-gaps/README.md`](README.md).

## Arquivos

| Arquivo | Mudança |
|---|---|
| [`docs/product/05-decisions.md`](../../../product/05-decisions.md) | 0.7.6 — nova seção com D-MVP-01..04, tabelas de comportamento e divergências encerradas |
| [`docs/audio/08-equalizers.md`](../../../audio/08-equalizers.md) | 0.7.4 — passo de teclado e §14 passam a citar o comportamento verificado no MVP |
| [`docs/audio/10-vitrola.md`](../../../audio/10-vitrola.md) | 0.7.3 — presets, local/atalhos, enable por seção e marco M3 sem os SKIN |
| [`docs/design/05-dsp-panel.md`](../../../design/05-dsp-panel.md) | 0.7.3 — §6 vira a interação real do MVP; registra o que não existe e a divergência 12 × 16 |
| `docs/evidence/PR/chore-specs-skin-gaps/` | evidência deste PR |

## O que este PR não faz

- **Não reescreve a matriz VU-INT-06** (144 combinações = 12 skins × 12 VUs). Atualizá-la é trabalho da TASK-E02-07; D-MVP-04 registra a divergência sem tocar na matriz.
- **Não cria requisito, TASK nem marco** a partir dos SKIN.
- **Não decide** adotar "0 VU ajustável" nem "slot de 28 DIP" — se desejados, é decisão nova.
- **Não altera a evidência histórica** `chore-dsp-eq-docs/summary.md`, que registra o que foi feito naquele PR quando os SKIN ainda existiam.
- **Não toca visibilidade, proteções, assinatura nem plano** — isso é da TASK-E19-06, por decisão da ADR-014.

## Verificação

| Comando | Resultado |
|---|---|
| `cargo xtask check-docs` | `result: passed`, `errors: []` — 350 arquivos Markdown, 191 requisitos, 191 tarefas, 19 épicos |
| `cargo xtask check-delivery` | `result: passed`, `errors: []` |
| `cargo fmt --all --check` | sem diff |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | `No issues found` |
| `cargo test --workspace --locked` | 52 + 34 testes, 0 falhas |

## Limitações declaradas

- A verificação do MVP é **de código, não de comportamento em execução**: nenhum navegador foi aberto. "Existe" significa "o código implementa"; "não existe" significa "não há handler, constante nem CSS que o faça".
- Quatro das oito afirmações ficaram sem o estágio adversarial automatizado (workflow interrompido) e foram conferidas diretamente.
- Medição de latência, CPU e qualidade de áudio no EQ continua não executada aqui.

## Revisão

- [ ] Revisão técnica independente — pendente.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
