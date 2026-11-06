# Pull request — Vynamp

## Resultado

Problema e comportamento final:

O PR #35 (mergeado em `efc3164`) registrou que o MVP web é a fonte única da UI e tirou as citações aos dois arquivos `SKIN-*` da cadeia documental. A revisão pós-merge achou **dois erros factuais** no que ele mergeou, mais três afirmações da mesma família e quatro contradições internas na própria evidência. Este PR corrige os nove — e, na rev. 2, mais dois arquivos onde a mesma afirmação falsa sobrevivia.

**Bloqueantes — texto que afirma o oposto do código do pin rev.38:**

1. **`docs/audio/10-vitrola.md` dizia que Vinyl e Tube "não têm processamento" no MVP.** Têm, e não é pouco: `class Vinyl` e `class Tube` em `mvp/advanced-dsp.js` com kernels próprios, montados como `AudioWorkletNode` (`vynamp-advanced37`) por `mvp/graph-audio.js`, e marcados `real=true` em `mvp/graph-model.js`. O cabeçalho e a §5 foram reescritos para separar três coisas que o texto anterior misturava: **existe** processamento, **não existe** validação acústica, e **não existe** nada em nativo.
2. **`docs/design/05-dsp-panel.md` mantinha "0 VU = −18 dBFS (−14/−20 opcionais)"** na tabela de medidores — três parágrafos abaixo do texto que declara essa convenção inexistente no MVP. A linha agora diz "Referência de 0 VU: **a definir**" e cita a escala real: cada medidor traz a sua em `scaleDB`/`scaleAngles` no `meta.json`, e os analógicos vão de −20 a +3 dB.

**A mesma família de erro, encontrada conferindo as linhas vizinhas:**

3. **§9 do painel DSP** listava Vinyl/Tube como "configuração preservada, sem processamento no grafo" e gate/expander/true peak/LUFS como "nós de configuração, indisponíveis para áudio". No pin, os quatro produzem áudio: `gate`/`expander` montam `vynamp-dynamics36`, o limiter tem lookahead e detector FIR 4×, o `loudness-meter` é o medidor de LUFS. A tabela foi corrigida e a lista "Pendente" passou a distinguir **áudio que existe** de **validação e certificação que não existem**.
4. **`08-equalizers.md` e `05-dsp-panel.md` ainda diziam que o protótipo "não foi verificado aqui"** — contradizendo o conteúdo verificado que os mesmos documentos passaram a ter no #35. Os dois cabeçalhos foram atualizados para citar o pin e apontar o que de fato ficou sem verificação (medição).

**Higiene da evidência do #35:**

5. **`docs/INDEX.md` tinha duas seções para a mesma decisão D-MVP.** Ficou uma, com o link do corpo do PR incorporado.
6. **A evidência se contradizia sobre quem revisou.** O campo "Revisor" dizia "subagente de contexto limpo" e o método dizia que quatro das oito afirmações "saíram de subagentes de verificação com estágio adversarial de refutação" — enquanto a própria seção "Limitações" dizia que o workflow foi abortado sem concluir e que o subagente revisor morreu por cota (HTTP 429). Agora diz o que aconteceu: **as oito foram conferidas diretamente pelo autor, sem segunda leitura independente.**
7. **A versão de `05-decisions.md` estava registrada como 0.7.6.** O arquivo já estava em **0.8.0** desde a E19-02 e este trabalho não sobe a versão. Idem para `10-vitrola.md`, que o #35 mergeou em 0.7.3 e esta correção leva a 0.7.4.

**Rev. 2 — a mesma afirmação falsa em mais quatro arquivos:**

8. **`docs/audio/09-dsp-catalog.md` dizia duas vezes que Vinyl e Tube são só configuração**: na tabela de famílias ("configuração completa preservada, sem processamento desses modelos; dependem de adaptadores") e na §5 ("Protótipo (relatado, rev. 15): só configuração, sem processamento"). O cabeçalho repetia a rev. 15 como estado atual. Os três pontos foram corrigidos e o cabeçalho passou a citar o pin rev.38.
9. **`docs/audio/03-dsp.md` listava "Vinyl/Tube só com configuração"** entre as capacidades por módulo, e o cabeçalho também tratava a rev. 15 como estado. Corrigidos.
10. **`docs/audio/03-dsp.md` §Grafo modular dizia que o protótipo liga áudio "com nós nativos do Web Audio, sem AudioWorklet"**. O pin rev.38 usa AudioWorklet em duas famílias de nó: `vynamp-advanced37` (bandas do EQ-64, limiter, loudness-meter, Vinyl, Tube) e `vynamp-dynamics36` (gate, expander), montados em `mvp/graph-audio.js`; o `registerProcessor` está em `mvp/advanced-dsp.js`. A frase foi corrigida e a rev. 15 marcada como histórico.
11. **`docs/product/05-decisions.md` mantinha "Vinyl e Tube só com configuração"** na linha D-DSP-17 (Saturação × Tube) — três linhas acima da entrada que o #35 já tinha corrigido. O `merge-tree` confirma que o #38 não toca esse texto.
12. **`VALIDATION.json` deixou de ser commitado neste PR.** O arquivo é saída de `check-docs`, não fonte; foi revertido com `git checkout --` e não entra no diff. A contagem 356 corresponde à árvore (nenhum `.md` novo nesta rodada).

**Duas autocontradições introduzidas no #35 e corrigidas aqui:** os cabeçalhos de `03-dsp.md` e `09-dsp-catalog.md` diziam que *tudo* abaixo era "evidência relatada, não verificada neste repositório", enquanto o mesmo documento passou a conter linhas verificadas no código do pin. Agora a regra é explícita: onde a linha cita o pin, é verificada; medição continua não executada.

TASK / REQ / QA:

Nenhuma. Spec-only, sem TASK id — mesmo padrão de `chore-specs-vynamp-restore` e `chore-specs-skin-gaps`. Não altera requisito, TASK nem marco; as contagens seguem **191 requisitos / 191 tarefas / 19 épicos**.

Plano Superpowers e evidência de execução:

[`docs/evidence/PR/chore-specs-skin-gaps/README.md`](../chore-specs-skin-gaps/README.md) — a evidência do trabalho original, agora em rev. 2, com os erros corrigidos registrados e as limitações declaradas sem contradição.

## Verificação

Comandos, resultados, ambientes/fixtures e limitações:

| Comando | Resultado |
|---|---|
| `cargo xtask check-docs` | `result: passed`, `errors: []` — 356 arquivos Markdown, 191 requisitos, 191 tarefas, 19 épicos |
| `cargo xtask check-delivery` | `result: passed`, `errors: []`, 3 arquivos de workflow |
| `cargo fmt --all --check` | sem diff |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | `No issues found` |
| `cargo test --workspace --locked` | 52 + 34 testes, 0 falhas |

`VALIDATION.json` não entra no diff: é saída de `check-docs`, não fonte. Depois dos checks ele volta a `git checkout -- VALIDATION.json`. A contagem 356 corresponde à árvore; nenhum `.md` novo foi criado nesta rodada.

Base de cada afirmação: leitura direta de `vynamp-mvp-reference@0d9860b` via `git show` — `mvp/advanced-dsp.js`, `mvp/graph-audio.js`, `mvp/graph-model.js`, `mvp/graph-editor.js`, `mvp/player.js`, `mvp/advanced-eq.js` e `out/svg/meters/*/meta.json`.

Limitações declaradas:

- **A verificação é de código, não de comportamento em execução.** Nenhum navegador foi aberto. "Existe" aqui significa "o código do pin implementa"; "não existe" significa "não há handler, constante nem CSS que o faça". Processamento existente **não** é processamento validado.
- **Nenhuma destas afirmações teve segunda leitura independente.** A revisão por subagente não rodou (cota do plano, HTTP 429) e o workflow de verificação foi abortado antes do estágio adversarial. Estes dois erros foram achados por releitura do autor, não por revisão — que é exatamente por que a rev. 1 os mergeou.
- **Não foi aberto navegador para conferir o selo "CONFIG." × áudio real** nos módulos do grafo. A conclusão vem do código (`real=true` e a montagem do `AudioWorkletNode`), não da UI renderizada.
- **A rev. 1 deste PR ainda deixava a afirmação falsa em dois arquivos** (`09-dsp-catalog.md` e `03-dsp.md`). Foi encontrada na rev. 2, por varredura de texto sobre toda a árvore — não por revisão. É a segunda vez que o mesmo erro passa por falta de segunda leitura.
- **Este PR conflita com o #38 (E19-03)** em `docs/INDEX.md` e `docs/audio/10-vitrola.md`, confirmado por `git merge-tree`. A resolução não é mecânica: o #38 numera o design como §8.1–8.9 e move a antiga §5 para §8; o #36 aponta para §5. Quem entrar depois reconcilia as duas numerações.
- Este PR não altera visibilidade, proteções, assinatura nem plano — isso é da TASK-E19-06, por decisão da ADR-014.

Revisão técnica (subagente Sonnet/Opus), commit revisado e achados:

- [x] Worktree e owner próprios.
- [x] Superpowers carregado; verificação executada.
- [x] Sucesso/erro/cancelamento e acessibilidade aplicáveis cobertos — não se aplica: PR de documentação, sem comportamento de runtime.
- [x] Docs/contracts/notices atualizados; nenhuma credencial no diff.
- [ ] CI Gate (GitHub Actions) aprovado; hardware e release assinado continuam gates separados.

**Revisão independente: não realizada.** Mesma causa da rev. 1 — subagente revisor indisponível por limite de cota do plano. Quem approve está aprovando verificação direta do autor, sem segundo par de olhos. O histórico da rev. 1 mostra que essa é uma limitação real, não formal.

Commit revisado: `703da0c`.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
