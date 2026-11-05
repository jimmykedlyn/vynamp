# ADR-013 — Vynamp Restore como ferramenta desktop dedicada

Versão: 0.1.1 · Data: 2026-11-05 · Estado: decisão do mantenedor; especificação normativa da ferramenta de restauração.

Status: aceito pelo mantenedor em 2026-11-05. Complementa [ADR-012](ADR-012-shell-integration.md) §3 e §5 (ferramentas auxiliares), [ADR-011](ADR-011-own-audio-stack.md) (pilha de áudio própria) e [ADR-008](ADR-008-vynamp-brand.md) (identidade). Orienta [TASK-E18-01](../../../tasks/items/TASK-E18-01.md).

## Contexto

O protótipo web de referência (MVP rev.38, repositório `vynamp-mvp-reference` @ `0d9860b`) traz uma ferramenta de restauração acústica embutida no reprodutor: um painel de workspace com monitor ao vivo (Original / Tratado / Só removido), ferramentas ajustáveis (DC, zumbido, gate, de-click, de-crackle, anti-saturação, remoção de artefatos MP3, extensão de agudos, de-rumble), regiões com ganho e fade, diagnóstico automático de defeitos, comparação em cortina, presets, exportação e processamento em lote.

Ao levar esse conjunto para o produto nativo, surgem cinco problemas que a referência não resolve:

1. **Escopo.** O MVP trata a restauração como uma aba dentro do player. A decisão do mantenedor em 2026-11-03 ([ADR-012](ADR-012-shell-integration.md) §3) é que **Restore e Converter sejam executáveis desktop separados**, com escopo funcional maior que o MVP. O motivo declarado é impedir que trabalho pesado de restauração afete a estabilidade e o tempo real da engine de playback.
2. **Corpus do MVP.** A referência tem 17 dos 17 formatos de áudio suportados pelo produto, mas o motor de restauração do MVP trabalha só com **dois canais** e trava acima de **96 MiB de PCM** (`restore.js` recusa o arquivo). Um utilitário desktop de restauração é usado justamente em coleções grandes e masterizações de hi-res.
3. **Incoerências do MVP.** A referência contém quatro módulos que a própria referência não exercita: o de-crackle mid/side (`restore-decrackle.js`) é definido mas nunca chamado — o que de fato roda é um reparo AR/L-SAR em `restore-dsp.js`; o denoise Wiener existe no código mas é descartado pelo modelo de estado e não tem controle na UI; o tipo de região `overlay` é oferecido na interface e declarado como pendente ("separação de voz e reconstrução por IA ainda não estão implementadas"); e o detector de cliques (`restore-click-detector.js`) produz `report.clickDetection` no worker sem que nenhum consumidor da UI leia o campo. Uma spec nativa que apenas transcrevesse o MVP herdaria essas incoerências.
4. **Limites de escrita.** O MVP exporta **apenas WAV float32 de 32 bits**, escrito à mão em memória, e regrava metadados ID3/LIST-INFO num WAV recém-gerado. Um arquivo de origem FLAC, ALAC, AIFF ou WMA perde o contêiner e os metadados originais; nada é preservado do ponto de vista do formato de saída.
5. **Reprodutibilidade e auditoria.** A restauração altera o áudio. Um procedimento de restauração precisa ser **repetível**, **inspecionável** (qual detector disparou, com que severidade, qual parâmetro foi mudado) e **não destrutivo por padrão** (a fonte nunca é sobrescrita sem confirmação explícita).

## Decisão

### 1. Processo separado, motor compartilhado

O **Vynamp Restore** é um executável desktop próprio (`vynamp-restore`), distribuído no mesmo instalador e nos mesmos artefatos de release do player, mas com **processo, janela e ciclo de vida próprios**.

O motor de restauração (análise, cadeia de DSP, diagnósticos, comparação) é **biblioteca interna do workspace Rust**, não código do app: o app é a casca, e o motor fica testável e reutilizável. O player **não** embute o motor de restauração na 1.0 — restauração é trabalho de fundo, não de reprodução.

Consequência direta: o player em execução **não é requisito** para o Restore funcionar. A ferramenta opera sobre arquivos.

### 2. Fronteira do aplicativo

```
vynamp-restore [opções] <caminhos...>
```

- Sem caminhos: abre vazio, aguardando arrastar ou o seletor de arquivos.
- Com caminhos: os arquivos entram na fila de análise, na ordem recebida, preservando a ordem do lote atômico entregado pelo shell ([ADR-012](ADR-012-shell-integration.md) §3).
- Chamado a partir do menu de contexto (`vynamp-restore <paths...>`) ou do atalho de aplicação: mesmo contrato, mesmo comportamento.
- **Uma instância por usuário** para o app, com IPC local autenticado, para que uma segunda chamada traga arquivos para a janela aberta em vez de abrir outra. O requisito de unicidade de owner por perfil já é normativo em [TASK-E16-02](../../../tasks/items/TASK-E16-02.md).

Ver o contrato completo em [spec da ferramenta](../../tools/01-vynamp-restore.md).

### 3. Motor de restauração: cadeia fixa e auditável

O motor é uma **cadeia de estágios em ordem fixa**, applied sobre o sinal decodificado. A ordem é normativa e não configurável pelo usuário (ver [spec §3](../../tools/01-vynamp-restore.md#3-cadeia-de-restauração)):

1. Reparo de crepitação (de-crackle, modelo AR) — 2 passagens.
2. Filtro passa-baixa de fase linear (63 taps, Kaiser) — condicionado ao de-crackle.
3. Remoção de artefatos de compressão (STFT, supressão de *birdies* e pré-eco).
4. Extensão de banda (síntese de alta faixa a partir da faixa-fonte).
5. De-click (substituição de amostra por média de vizinhos).
6. Remoção de offset DC.
7. De-rumble (passa-altas Butterworth 2ª ordem).
8. Notches de zumbido (50/60 Hz e harmônicos).
9. Gate de ruído.
10. Anti-saturação.
11. Mistura de regiões e ganho por região.

A cadeia é **documentada e versionada**; cada alteração de ordem é uma nova versão de motor e exige migração de projetos.

### 4. Correção das incoerências do MVP

A spec nativa **não** reproduz quatro incoerências do MVP:

| Coerência do MVP | Situação no MVP | Decisão nativa |
|---|---|---|
| De-crackle | `restore-decrackle.js` (mid/side) é definido e nunca chamado; o que roda é o reparo AR de `restore-dsp.js` | Mantém **um** mecanismo: o reparo AR/L-SAR, especificado com ordem, passes e latência. O módulo mid/side não é portado. |
| Denoise | Implementado (Wiener), mas descartado pelo modelo de estado e sem controle na UI | **Declarado fora de escopo na 1.0.** Sem controle, sem persistência, sem promessa. A redução de chiado espectral é separada de gate e não é prometida. |
| Região `overlay` | Oferecida na UI e marcada como pendente ("separação de voz e reconstrução por IA pendentes") | **Removida.** A 1.0 tem apenas regiões de ganho com fade (`between`). Separação de voz por IA é decisão futura e não é prometida. |
| Detector de cliques | `restore-click-detector.js` roda no worker e escreve `report.clickDetection`, mas nenhum consumidor da UI lê esse campo: o resultado é produzido e descartado | **Não portado como motor separado.** A detecção de cliques vive no detector `click` do diagnóstico (§6 da spec), que já emite severidade, evidência e correção sugerida no mesmo relatório; não há segundo relatório paralelo. |
| Presets embutidos | Os 5 presets do MVP (`restore-presets.js`) baseiam a ação principal em `denoise: 0.25–0.5`, que não existe na 1.0 (fora de escopo), sem `deartifact`/`bwe`/`declip` | **Redefinidos na 1.0.** Cada preset de built-in recebe um bloco `tools` explícito com as ferramentas e valores da 1.0; a ação principal sai de `denoise` para as ferramentas existentes (de-crackle, de-click, gate, DC, rumble, zumbido). O aviso de perda de agudos acima de 8 kHz aciona em filtro de zumbido ativo ou passa-baixa agressivo, não em "redução de ruído alta" do denoise descartado. |

Toda divergência em relação ao MVP é registrada nesta ADR e na spec, para que a paridade de recursos seja auditável e não implícita.

**Destino da "decisão futura".** A linha da região `overlay` deixou de ser um beco sem saída: a decisão sobre restauração por IA foi registrada em [ADR-015](ADR-015-ai-restoration-future-phase.md) e especificada em [Restore — restauração por IA](../../tools/03-vynamp-restore-ia.md), como **fase opt-in pós-1.0, planejada e não prometida**. Nada aqui muda: `overlay` continua removido, o denoise continua fora de escopo, e a 1.0 continua sem nenhuma capacidade de aprendizado de máquina.

### 5. Formato de saída e preservação de metadados

- **A fonte nunca é sobrescrita.** O Restore grava em arquivo novo; sobrescrever a fonte exige confirmação explícita e propõe nome com sufixo.
- **Escrita atômica**: arquivo temporário no **mesmo volume**, `fsync`, validação pós-escrita no temporário (releitura e decodificação do contêiner, taxa, canais e duração) e, só então, `rename` sobre o destino. Em caso de falha de validação, o temporário é removido e a origem permanece intacta; não deixa saída parcial nem toca a fonte.
- **O contêiner de saída é escolhido pelo usuário**, entre os formatos de saída comprovados pelo produto: FLAC, WAV (PCM 16/24/32 bits e float32), AIFF e ALAC. O **WAV float32** do MVP é mantido como opção, mas deixa de ser o único.
- **Metadados são preservados** da origem para a saída: tags, capa e campos técnicos, reescritos no contêiner de destino sem perda de campos conhecidos e sem descartar campos desconhecidos.
- **A saída é validada após a escrita**: o **temporário** é relido e decodificado; contêiner, taxa, canais e duração precisam coincidir com o esperado. Uma saída que falha na validação tem o temporário removido e a origem reportada como intacta.

### 6. Diagnóstico é o produto, não um extra

O diagnóstico automático é parte central da ferramenta, não um botão acessório. O contrato:

- Cada defeito tem **tipo**, **severidade** (`mild` / `moderate` / `severe`), **evidência** (valores numéricos medidos) e **correção sugerida** (patch de parâmetro ou recusa explícita).
- **Nenhuma correção é aplicada sem previsualização**: o motor propõe, o usuário confirma. O botão "aplicar correções recomendadas" aplica apenas defeitos de severidade `moderate` ou `severe` **com correção aplicável**, e cada aplicação é reversível.
- Defeitos **sem correção aplicável** são reportados e **não** recebem correção inventada — a ausência de correção é informação, não falha.
- Detecção e correção são **reproduzíveis**: o mesmo arquivo com a mesma versão de motor produz o mesmo relatório.

### 7. Limites declarados

| Limite | Valor | Razão |
|---|---|---|
| Duração por arquivo | 24 h | Alinhado ao modelo do MVP e à janela de edição |
| Canais | até 32 (mono a 7.1.4 preservando o layout do contêiner) | Supera o limite de 2 canais do MVP; o layout é do contêiner, não re-mapeado silenciosamente |
| Memória de trabalho | decodificação em blocos, com teto configurável (padrão 512 MiB) | Arquivos longos não podem exigir RAM equivalente ao tamanho decodificado |
| Regiões por projeto | 128 | Herdado do MVP; suficiente e previsível |
| Profundidade de undo/redo | 60 passos | Herdado do MVP |
| Tamanho de projeto | 256 KiB | Herdado do MVP; projetos são estado, não mídia |

### 8. Identidade

Conforme [ADR-008](ADR-008-vynamp-brand.md) e [ADR-012](ADR-012-shell-integration.md) §5: símbolo C / Pulso com **badge de restauração** em ciano `#66D9EF`. Ícone próprio (`vynamp-restore.ico` / `tools/vynamp-restore.svg`), já presente em `assets/icons/`.

## Alternativas Consideradas

1. **Manter a restauração como aba do player (como no MVP).**
   - *Rejeitado pelo mantenedor em 2026-11-03.* Uma sessão longa de restauração disputa CPU com a reprodução, mistura ciclo de vida do player com o da ferramenta e impede o uso do Restore em máquina onde o player não está instalado.
2. **Manter a exportação apenas em WAV float32 (como no MVP).**
   - *Rejeitado.* Perder o contêiner de origem é lossiness que o produto não pode anunciar como restauração. A escolha de contêiner é do usuário e a preservação de metadados é regra.
3. **Portar os módulos inativos do MVP (de-crackle mid/side, denoise Wiener, região `overlay`, detector de cliques).**
   - *Rejeitado.* Portar código que a própria referência não exercita significa depurar comportamento nunca observado em uso real e prometer capacidades sem evidência. O motor nativo começa pelo que o MVP efetivamente executa.
4. **Fazer do Restore um plugin/extensão do player.**
   - *Rejeitado.* Extensões do Vynamp são agentes de terceiros com permissões restritas ([ADR-004](ADR-004-extensions.md)); a ferramenta é produto próprio, com identidade, release e garantia próprias.

## Consequências

- Novo documento normativo `docs/tools/01-vynamp-restore.md` com a spec completa da ferramenta, e `docs/tools/02-vynamp-converter.md` com a spec preparatória do Converter.
- Novo épico E18 (`tasks/epics/E18.md`) com `TASK-E18-01` (Restore) e `TASK-E18-02` (Converter), e os requisitos `REQ-E18-01` / `REQ-E18-02`.
- O motor de restauração passa a ser **biblioteca interna com contrato de teste próprio**, com fixtures de referência por defeito (saturação, zumbido 50/60 Hz, cliques, crepitação, chiado, ronco, DC, corte de banda).
- O diagnóstico passa a ter **formato de relatório estruturado** (não texto), para ser gravado em log, exibido na UI e exportado em evidência.
- A paridade de recursos com o MVP é medida contra a spec nativa, e as quatro divergências declaradas em §4 são a linha de base do que o MVP não entrega.
