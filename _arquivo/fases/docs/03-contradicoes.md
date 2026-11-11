# As 13 contradições — status e ação

Levantadas na FASE 1 (§b). Cada linha tem o status, a decisão que a fecha e o passo
da FASE 3 que a executa.

| # | Contradição | Status | Decisão | Passo |
|---|---|---|---|---|
| C1 | Workspace lista `vynamp-audio`/`-domain`/`-app` e não `vynamp-engine` | **fechada** | M3 | 31 |
| C2 | `architecture/04` define 6 traits num só contrato | **fechada** | M3 | 31 |
| C3 | `vitrola` como repo privado × "não há biblioteca externa" | **fechada** | ADR-014 §4 + M10 | merge |
| C4 | "não existe TASK vitrola" × E19-03/E19-07 existem | **fechada** | dissolução na `feature/task-e19-03` | merge |
| C5 | Vinyl/Tube "só config" × "processamento real" | **fechada** | evidência: `advanced-dsp.js:74,128` | 31 |
| C6 | "12 skins × 12 VUs = 144" × 16 VUs | **fechada** | M4 | 31 |
| C7 | Encoders: AAC com ressalva × sem ressalva | **fechada** | `tools/02` vence | 31 |
| C8 | Resampler no núcleo × em `vynamp-encode` | **fechada** | M9 (`vynamp-dsp`) | 31 |
| C9 | CI "enquanto privado, 403" × ADR-014 público | **fechada** | M6 | 34 |
| C10 | Duas autoridades de agente | **fechada** | M5 | 32 |
| C11 | `STATE.md` marca ✅ onde o backlog diz `in_progress` | **fechada** | `STATE.md` gerado do backlog | 37 |
| C12 | README do decode anuncia metadata/CUE sem crate | **aberta** | vira TASK do `vynamp-decode` | — |
| C13 | `crates/`/`apps/` são só README num "workspace de 14 crates" | **fechada** | M3 + M1 | 30 |

**12 fechadas, 1 aberta.** A aberta (C12) não bloqueia a reorganização: é dívida do
repo satélite, registrada para quando o `vynamp-decode` ganhar a lib de metadados.

---

## Detalhe das que exigiram investigação

### C5 — Vinyl/Tube

Não era contradição de fato, era redação ambígua sobre dois objetos diferentes.

- `product/05-decisions.md:52` — `D-DSP-17` fala do **protótipo antigo** (rev.14/15) e
  diz que Vinyl e Tube só têm configuração.
- `product/05-decisions.md:77` — fala do **pin rev.38** (`vynamp-advanced37`) e afirma
  processamento real.

O código confirma a segunda leitura: `vynamp-mvp-reference/mvp/advanced-dsp.js:74,128`
tem `class Tube` e `class Vinyl` com filtros, halfband, decimação, ruído e latência
real (`TUBE_LATENCY=3`).

**Ação (31):** reescrever `D-DSP-17` deixando explícito que a limitação vale para a
rev.14/15 e não para o pin rev.38, mantendo a ressalva "sem validação acústica".

### C7 — Encoders

`tools/02-vynamp-converter.md` vence `audio/02-formats.md:39` por três motivos:
é mais novo, é específico do Converter, e registra em `:19` que o risco de patente do
AAC foi **avaliado e aceito pelo mantenedor** — não é omissão.

**Ação (31):** alinhar `audio/02` às palavras de `tools/02`. AAC sem ressalva;
MP3 = "quando o encoder estiver na build".

### C8 — Resampler

Não era contradição, era má redação. E o fato que muda o problema: **não existe
resampler em lugar nenhum hoje.** Varredura feita:

| Repo | Resultado |
|---|---|
| `vynamp-output` | só negociação de formato em `vynamp-output-core` |
| `vynamp-decode` | zero `.rs` mencionando `resampl` |

Também não há dither, downmix ou mapeamento de canais compartilhados. Não estamos
refatorando código duplicado — estamos decidindo onde ele nasce.

**Ação (M9):** nasce em `vynamp-dsp`, std-only, dependência opcional de `output` e
`encode` via feature `resample`, obrigatória no `engine`. A seta é sempre
`módulo → dsp`, nunca `lib → lib`.

### C11 — `STATE.md` × `backlog.json`

`STATE.md` está errado. Ele marca ✅ em E19-01/E19-02 e o parágrafo seguinte diz
"#34 resolvido, #36 MERGED" com worktrees descritas como "restos de PRs já mergeados".
O `backlog.json` diz `in_progress`. Como nenhuma das 191 TASKs tem critério de aceite,
**não há base para marcar ✅ em nada**.

**Ação (37):** corrigir `STATE.md` para refletir o backlog, e a partir daí
`STATE.md` é **gerado** do `backlog.json` — nunca editado à mão no status.

---

## C12 — a única aberta

`vynamp-decode/README.md` anuncia "containers, codecs, metadata e CUE", mas o
workspace tem 9 crates e nenhum de metadados ou de CUE.

Não bloqueia a reorganização porque é afirmação de um repo satélite sobre seu próprio
escopo. Vira TASK do `vynamp-decode` quando a lib de metadados entrar em pauta; até
lá, o README precisa dizer "metadata e CUE: planejado, sem crate".

---

## O que a FASE 2 não decidiu

Nada de conteúdo novo. As 13 contradições são de **fato** — qual documento está
errado — e a FASE 2 só registrou o veredito e o passo que o executa. Nenhuma
contradição nova foi encontrada ao gerar o mapa das 596 entradas.
