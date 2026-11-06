# Evidência — chore-specs-skin-gaps

Data: 2026-11-05 · Autor: Claude Opus 5.5 · Revisor: subagente de contexto limpo.

Registra **por que** este PR existe, o que foi verificado no código do MVP e o que **não** foi.

## Origem do trabalho

O pedido inicial era absorver nas specs o que os dois arquivos de skin da pasta raiz tinham e o repositório não tinha. No meio do trabalho o mantenedor decidiu o contrário, em duas mensagens:

> "SKIN-REQUIREMENTS-CODEX pode desconsiderar, o vynamp se baseará pelo mvp"
> "SKIN-REQUIREMENTS-GPT-MVP-OBRIGATORIO-v1.2 também desconsiderar"

Isso **zera o escopo de absorção**: não há lacunas a herdar, porque o MVP é a fonte. Sobra o que este PR faz — registrar a decisão e limpar as citações que ainda davam autoridade aos dois arquivos.

A decisão é factualmente segura: os SKIN eram **consumidores** declarados deste repositório, não origem. Seus próprios changelogs citavam `DSP-UX`, `VU-INT` e `LYR` como documentos canônicos daqui. Manter as duas fontes produzia duas especificações para o mesmo comportamento.

## Método: verificar antes de re-apontar

Re-apontar citação sem evidência trocaria um erro por outro. Oito afirmações atribuídas aos SKIN foram conferidas contra o código do MVP em `vynamp-mvp-reference@0d9860b` (rev.38). Quatro saíram de subagentes de verificação com estágio adversarial de refutação; quatro foram conferidas diretamente, depois de o workflow de verificação passar de 50 minutos sem convergir.

| Afirmação | Veredito no MVP | Evidência |
|---|---|---|
| Passo de teclado do EQ (0,5 dB, ±2 dB com Shift, Home/End, duplo clique) | **existe** | `mvp/advanced-eq.js` — `input[type=range]` com `step=.5`; `keydown` com Shift aplica ±2 dB com clamp em ±15; `dblclick` zera |
| Rótulo `[DEMO — sem processamento de áudio neste modo]` | **existe** | `mvp/advanced-eq.js`, `mvp/dsp-workspace.js` |
| Cinco modos de EQ, com 64 bandas gráficas reais | **existe** | `mvp/advanced-eq.js` — `modes=['10','20','31','64','parametric']`; 64 `input[type=range]` |
| Presets da Vitrola | **diverge** | Presets são **Sutil/Quente/Rústico**, reais, sem marcação `[proposal]`. "Subtle/Warm/Rustic" não existe no MVP |
| Local e atalhos da Vitrola | **parcial** | Atalhos na barra do EQ e na faixa de áudio; **não** no inspector da cadeia, como o SKIN pedia |
| Enable separado por seção na Vitrola | **parcial** | Existe bypass por módulo + botão único da categoria. **Não** existe "intensidade 0 = bypass da seção" nem API de host |
| "0 VU = −18 dBFS ajustável" | **não existe** | Nenhuma constante de calibração no código; escala vem de `scaleDB`/`scaleAngles` no `meta.json` |
| "Slot de 28 DIP" | **não existe** | Instrumento fluido: `#meter-stage` com `width:100%`/`height:auto` |

A contagem de VU também divergiu: o contrato VU-INT fala em 12 medidores (número da rev.15) e o pin rev.38 tem **16, todos selecionáveis**.

## Arquivos

| Arquivo | Mudança |
|---|---|
| [`docs/product/05-decisions.md`](../../../product/05-decisions.md) | 0.7.6. Nova seção "O MVP é a fonte única da UI": **D-MVP-01..04**, tabela "Comportamentos que os SKIN atribuíam ao produto e o MVP não tem", "Divergências encerradas". Quatro divergências antigas reescritas como encerradas |
| [`docs/audio/08-equalizers.md`](../../../audio/08-equalizers.md) | 0.7.4. Passo de teclado deixa de citar o SKIN e passa a citar o MVP com o que ele realmente faz; §14 deixa de ser contrato para o Codex e passa a citar o pin verificado |
| [`docs/audio/10-vitrola.md`](../../../audio/10-vitrola.md) | 0.7.3. Presets, local/atalhos, enable por seção e marco M3 substituem as citações aos SKIN por comportamento verificado no MVP |
| [`docs/design/05-dsp-panel.md`](../../../design/05-dsp-panel.md) | 0.7.3. §6 troca "convenções preservadas do SKIN" pela interação real do MVP; registra que 0 VU ajustável e slot de 28 DIP **não existem**, e a divergência 12 × 16 medidores |

**Nenhuma alteração de código, dependência, workflow ou backlog.** As contagens 191 requisitos / 191 tarefas / 19 épicos não mudam.

## O que foi deliberadamente **não** feito

- **Nenhuma reescrita da matriz VU-INT-06.** As 144 combinações (12 skins × 12 VUs) continuam no texto; atualizá-las é trabalho da TASK-E02-07, quando a integração visual for feita. D-MVP-04 registra a divergência sem reescrever a matriz.
- **Nenhum requisito novo.** D-MVP-03 fixa que nenhuma TASK, REQ ou marco nasce dos SKIN.
- **Nenhuma decisão sobre "0 VU ajustável" nem "slot de 28 DIP".** Ficam registrados como inexistentes no MVP; adotá-los seria decisão nova, não herança.
- **A evidência histórica `chore-dsp-eq-docs/summary.md` foi preservada.** Ela registra o que foi feito naquele PR, com os SKIN existindo; reescrevê-la falsificaria o histórico.

## Verificação

| Comando | Resultado |
|---|---|
| `cargo xtask check-docs` | `result: passed`, `errors: []` — 350 arquivos Markdown, 191 requisitos, 191 tarefas, 19 épicos |
| `cargo xtask check-delivery` | `result: passed`, `errors: []`, 3 arquivos de workflow |
| `cargo fmt --all --check` | sem diff |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | `No issues found` |
| `cargo test --workspace --locked` | 52 + 34 testes, 0 falhas |

Os dois arquivos de evidência foram indexados em `docs/INDEX.md` (o validador exige que todo `.md` esteja no índice).

## Limitações

- **A verificação do MVP é de código, não de comportamento em execução.** Nenhum navegador foi aberto, nenhuma interação foi exercitada. "Existe" aqui significa "o código do MVP implementa"; "não existe" significa "não há handler, constante nem CSS que o faça".
- **O workflow de verificação foi interrompido.** Quatro afirmações ficaram sem o estágio adversarial automatizado e foram conferidas diretamente, com o mesmo critério, mas sem o segundo par de olhos.
- **`[DEMO]` vs áudio real no EQ:** o pin declara `[REAL] … uma instância Web Audio` quando o nó está na rota viva. A *medição* de latência, CPU e qualidade continua não executada aqui.
- Este PR não altera visibilidade, proteções, assinatura nem plano — isso é da TASK-E19-06, por decisão do mantenedor na ADR-014.
