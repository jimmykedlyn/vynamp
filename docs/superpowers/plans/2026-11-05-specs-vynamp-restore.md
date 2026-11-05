# Plano de Especificação — PR4: Vynamp Restore (e preparação do Converter)

Data: 2026-11-05 · Autor: Claude Opus 5.5 (Orquestrador) · Alvo: PR4 (`chore/claude-specs-vynamp-restore`)

## Objetivo

Formalizar a especificação normativa do **Vynamp Restore**, ferramenta desktop dedicada de análise acústica e restauração de áudio, e deixar o **Vynamp Converter** preparado (ADR e escopo de tarefa) para o PR5. Nenhuma implementação é feita aqui — é entrega documental, de ativos e de contracts de backlog.

## Fontes de verdade (ler, não inventar)

| Fonte | Caminho | Uso |
|---|---|---|
| MVP web rev.38 | `D:\Projects\vynamp-mvp-reference\mvp` @ `0d9860b` | Inventário factual das ferramentas de restauração (algoritmos, faixas de parâmetros, diagnósticos, presets, limites) |
| ADR-012 | `docs/architecture/adr/ADR-012-shell-integration.md` | Decisão já tomada: Restore e Converter são executáveis separados, ações de menu de contexto, badge de ícone |
| `docs/platforms/03-system-integration.md` v0.8.0 | repo | Contratos CLI e de registro já normativos |
| Pilha de áudio própria | ADR-011 | `vynamp-decode`/`vynamp-output` como base; Restore não usa crates de terceiros |

## Decisões do mantenedor que este PR implementa

1. **Restore e Converter são executáveis desktop separados** (`vynamp-restore`, `vynamp-converter`) sobre as mesmas libs do player, com escopo maior que o MVP (decisão de 2026-11-03, registrada em ADR-012 §3).
2. **Épico novo E18** — "Ferramentas de arquivo" — para Restore e Converter, por serem apps com identidade, ícone, release e contrato próprios, não utilitários internos do player.
3. **Restore completo + Converter preparado** neste PR (decisão do mantenedor nesta sessão).

## Tarefas do Plano

### Tarefa 1: ADR-013 — Vynamp Restore como ferramenta desktop dedicada
- Criar `docs/architecture/adr/ADR-013-vynamp-restore.md`.
- Cobrir: por que processo separado (isolamento de falha, tempo real do player intocado, ciclo de release próprio, uso em máquina sem o player instalado); fronteira do app (`vynamp-restore <paths...>`, sem dependência de instância do player rodando); o motor de restauração como biblioteca interna compartilhada (o app é a casca, o motor é reaproveitável); formato de saída WAV; preservação de metadados; o que **não** é Restore (separação de voz por IA, reconstrução de master, upscaling hi-res); decisões herdadas do MVP que são corrigidas na versão nativa (o "de-crackle" realmente executado no MVP é o modelo AR, não o módulo mid/side; o denoise Wiener é inalcançável no MVP; `kind: overlay` é declarada mas não implementada no MVP).

### Tarefa 2: Spec normativa `docs/tools/01-vynamp-restore.md`
- Criar a spec canônica da ferramenta, com:
  - Contrato de CLI e de invocação (paths, flags, exit codes).
  - Motor de restauração: cadeia de DSP normativa e ordem fixa.
  - Cada ferramenta: algoritmo, faixa de parâmetro, valor padrão, com os números reais do MVP.
  - Diagnósticos: lista de detectores, severidade, correção sugerida.
  - Regiões: esquema, limite, não sobreposição, tipos (`between`/`overlay`).
  - Comparação A/B: modos, métricas, pré-roll.
  - Lote: limite, concorrência, cancelamento, relatório agregado.
  - Metadados: extração, preservação, re-embedding, formato de saída.
  - Presets: built-in + custom, validação.
  - Formato de projeto `.vynamp-restore` (JSON versionado).
  - Limites e guardas (duração, canais, memória, tamanho de projeto).
  - Matriz de o que está disponível em cada plataforma.

### Tarefa 3: Spec preparatória `docs/tools/02-vynamp-converter.md`
- Criar a spec canônica do Converter já com escopo fechado (delimitadores de PR5), com:
  - Fronteira do app, CLI, contrato de fila de jobs.
  - Matriz de formatos de entrada/saída e o papel da lib `vynamp-encode` (a criar no PR5).
  - Regras de qualidade (lossless vs lossy, AAC sem ressalva — risco de patente aceito e registrado).
  - Preservação de metadados, escritas atômicas e validação pós-escrita.

### Tarefa 4: Épico E18 no backlog
- Criar `tasks/epics/E18.md`.
- Criar `tasks/items/TASK-E18-01.md` (Restore) e `tasks/items/TASK-E18-02.md` (Converter).
- Adicionar REQ correspondentes em `docs/product/03-requirements.md` (REQ-E18-01, REQ-E18-02).
- Adicionar as tarefas em `tasks/backlog.json` com paridade exata de `deliverable`/`acceptance` (string verbatim nos arquivos MD).
- Referenciar QA existentes (QA-015, QA-016, QA-023, QA-024, QA-031) — sem inventar IDs de QA novos.

### Tarefa 5: Atualização de índice, glossário, changelog e ícones
- `docs/INDEX.md`: indexar os novos documentos, o épico E18 e as novas TASKs.
- `docs/GLOSSARY.md`: termos novos (motor de restauração, projeto de restauração, defeito, severidade, região, cadeia de restauração).
- `CHANGELOG.md`: entrada 0.8.0.
- `docs/product/06-feature-parity.md`: linha de Restore/Converter na tabela de utilitários.
- `assets/icons/manifest.json`: já contém as ferramentas — confirmar coerência do badge e do executável.
- `VALIDATION.json`: contagem de Markdown.

### Tarefa 6: Validação e revisão
- `cargo xtask check-docs` e `cargo xtask check-delivery`.
- `cargo test -p xtask`.
- Revisão por subagente de contexto limpo (Opus — alto risco: define arquitetura de uma ferramenta de áudio).
- Commit, push e PR para `develop`.

## Riscos e limites declarados

- Nenhuma integração foi executada em sistema operacional; a validação é documental e estrutural.
- CI Gate (Woodpecker) não verificado — o mantenedor confere no SHA final.
- macOS não verificado (sem agente Mac); hardware de saída de áudio não testado.
- O inventário do MVP é a fonte factual dos números; onde o MVP é incoerente (módulos inalcançáveis), a spec nativa corrige e declara a divergência.
