# ADR-015 — Restauração por IA admitida apenas como fase opt-in pós-1.0

Versão: 0.1.0 · Data: 2026-11-05 · Estado: decisão do mantenedor; normativa sobre escopo, não sobre implementação.

Status: aceito pelo mantenedor em 2026-11-05. **Não substitui nem emenda** [ADR-013](ADR-013-restore.md) nem [ADR-011](ADR-011-pilha-propria.md); acrescenta a elas a condição sob a qual uma capacidade de aprendizado de máquina poderia um dia entrar. Orienta a especificação [Restore — restauração por IA](../reference/tools/03-vynamp-restore-ia.md).

## Contexto

Duas especificações de restauração por aprendizado de máquina existiam fora do repositório e sem representação normativa:

1. **`AUDIOSR-RUST-CPU.md`** — super-resolução de banda por difusão latente (AudioSR), em Rust, CPU como piso, GPU opcional, sem runtime Python, com pipeline completo (pré-processamento, VAE, UNet, DDIM, vocoder), pesos de 1–2 GB, substituição só da banda alta e critérios de paridade.
2. **`SOURCE_SEPARATION_MODELS.md`** — separação de fontes por modelos ONNX (MDX-Net, Demucs), de ~30 MB a ~316 MB, com download sob demanda e máscara espectral aplicada a regiões marcadas.

Nenhuma das duas era mencionável pelo produto: não havia requisito, ADR, fase de roadmap nem TASK. Isso produz um estado pior que a ausência — a **especificação órfã**, detalhada em nível de implementação e desprovida de status, que pode ser lida tanto como "prometido e atrasado" quanto como "descartado".

Ao mesmo tempo, a [ADR-013](ADR-013-restore.md) §4 declarou explicitamente:

- a região `overlay` foi **removida** da 1.0 (restam apenas regiões de ganho com fade, `kind: between`);
- *"separação de voz por IA é decisão futura e não é prometida"*;
- o upscaling de hi-res é **síntese**, não recuperação.

Incorporar as duas especificações como se fossem escopo corrente contradiria uma ADR aceita. Descartá-las perderia trabalho de análise técnica real — escolha de runtime, ordem de componentes, política de download, riscos medidos. A decisão do mantenedor em 2026-11-05 foi a terceira via: **registrar a capacidade como fase futura, sem tocar no escopo da 1.0**.

## Decisão

### 1. A capacidade é planejada, não prometida

Super-resolução de banda e separação de fontes ficam **admitidas apenas como fase pós-1.0**, documentadas em [Restore — restauração por IA](../reference/tools/03-vynamp-restore-ia.md).

Nada muda no escopo da 1.0: `kind: overlay` continua removido, o upscaling de hi-res continua síntese, e a frase "decisão futura e não é prometida" da ADR-013 §4 permanece verdadeira. Esta ADR descreve a **decisão sobre a decisão** — custos, condições, bloqueios — sem tomá-la.

### 2. Cinco condições bloqueantes, nenhuma satisfeita

A capacidade não entra em milestone nem em release enquanto qualquer uma faltar:

| # | Condição | Verificação |
|---|---|---|
| 1 | Licença de cada peso conferida (redistribuição e uso comercial) | §4 |
| 2 | Spike de 1 bloco de 10 s no runtime escolhido, com tempo e RAM medidos | §5 |
| 3 | Política de download explícito aceita como parte da UX | §3 |
| 4 | Decisão de escopo registrada em [05-decisions](../product/05-decisoes.md) | D-IA-01 |
| 5 | Milestone atribuído em [01-roadmap](../delivery/01-roadmap.md) | hoje: nenhum item de ML |

### 3. Opt-in de ponta a ponta

Desligado por padrão. Ativar a chave **não** baixa nada; o download é clique explícito, depois de o usuário ver tamanho, fonte e licença, com checksum SHA-256 verificado. Nenhum download automático, nunca. Desativar a chave não apaga pesos automaticamente — remover é ação explícita. Instalação manual por pasta do usuário é suportada, com o mesmo checksum.

Consequência de engenharia: **um build sem as features de IA não tem nenhuma dependência de ML**. O player e o Restore DSP da 1.0 permanecem instaláveis, auditáveis e pequenos.

### 4. Pesos não são embutidos; a licença bloqueia release

Nenhum modelo entra no instalador. O maior passa de 300 MB e o de super-resolução chega a ~2 GB; a maioria dos usuários precisa só de remoção de voz (~30–50 MB). Além do custo de pacote, há o problema legal: **pesos são obra de terceiros**, e a licença do código do produto não concede direito sobre eles — a mesma regra que [privacidade e licenças](../security/02-privacidade-licencas.md) já aplica a codecs.

Enquanto a licença de um modelo não estiver conferida por escrito, ele não é oferecido na UI. Se não for redistribuível, o usuário baixa da fonte oficial e converte localmente, e o app diz isso antes do download.

`release preflight` continua exigindo `THIRD_PARTY_NOTICES` e SBOM: um modelo baixado pelo usuário não entra no instalador, então não muda essa exigência — e por isso nenhum modelo é embutido.

### 5. CPU é o piso; GPU é aceleração com fallback

O requisito mínimo é CPU. GPU (CUDA, Metal, e DirectML/Vulkan a avaliar) é opcional e só é usada se detectada **e** aprovada num teste rápido de memória e paridade; caso contrário, CPU. Falha de GPU no meio do job volta à CPU, avisa e **retoma do bloco onde parou** — nunca reinicia em silêncio. Determinismo bit-exato vale por backend e máquina; entre CPU e GPU vale tolerância de paridade declarada.

### 6. A pilha própria (ADR-011) vale no tempo real, não no job offline

A [ADR-011](ADR-011-pilha-propria.md) decidiu não usar terceiros no caminho de áudio. Esta ADR interpreta o alcance:

- **Caminho de reprodução:** 100% próprio, sem nenhum runtime de ML. Vale integralmente.
- **Job offline:** pode usar runtime de terceiros (`candle`, `ort` ou `tract`), atrás de feature Cargo, porque é processamento em lote fora da reprodução, com resultado auditável.
- **Habilitar IA exige ADR nova** que emende a ADR-011 com a exceção nomeada e datada. Esta ADR **não** é essa emenda: é a entrada dela.

### 7. Correção, não recuperação — o rótulo é normativo

A super-resolução **sintetiza** banda alta; não recupera informação perdida. O produto não pode chamá-la de "recuperação", "aumento real de resolução" ou "restauração do original". A substituição ocorre **apenas acima do corte** estimado pelo detector `band_limit`; abaixo do corte o áudio é o original, com crossover linear-phase.

### 8. Escrita continua não destrutiva

Saída sempre em arquivo novo, em 48 kHz. A origem nunca é sobrescrita — regra que já é da 1.0 e que a IA não flexibiliza.

### 9. Sem número de requisito na matriz nem TASK no backlog

Os IDs `RSAI-01..14` da spec de IA são locais ao documento, como os `LYR-*` são (D-LYR-01). Não entram em [03-requirements](../product/03-requisitos.md) nem em `tasks/backlog.json` — a matriz conta requisitos de escopo corrente, e promover a capacidade a requisito numerado seria prometê-la. Quando a decisão de entrar em milestone for tomada, os REQs e TASKs nascem nesse PR.

## Alternativas Consideradas

1. **Absorver AudioSR e separação de fontes como escopo da 1.0.**
   - *Rejeitado pelo mantenedor em 2026-11-05.* Contradiria a ADR-013 §4 aceita, exigiria reabrir `kind: overlay`, e colocaria no caminho crítico da 1.0 capacidades que dependem de licença de pesos não conferida e de spike não executado.
2. **Descartar as duas especificações e não registrar nada.**
   - *Rejeitado.* Perderia análise técnica real (runtime, ordem de componentes, política de download, riscos) e não resolveria a orfandade — o arquivo continuaria existindo fora do repositório, com a mesma ambiguidade de status.
3. **Criar TASK-E19-08/09 no backlog para as duas capacidades.**
   - *Rejeitado.* Uma TASK é compromisso de execução; criá-la agora prometeria o que §1.3 ainda bloqueia, e alteraria as contagens da matriz (191 requisitos / 19 épicos) para trabalho que não entrou em fase.
4. **Embutir um modelo pequeno (~30 MB) no instalador para reduzir atrito.**
   - *Rejeitado.* Baixaria o atrito de UX ao custo de redistribuir obra de terceiros, com `THIRD_PARTY_NOTICES` e SBOM no caminho de release. O download explícito é mais barato do que a dúvida legal.

## Consequências

- Novo documento normativo [Restore — restauração por IA](../reference/tools/03-vynamp-restore-ia.md), marcado como **planejado e não prometido**, com requisitos locais `RSAI-01..14`.
- [ADR-013](ADR-013-restore.md) §4 e [spec do Restore §1.2](../reference/tools/01-vynamp-restore.md) ganham remissão a esta ADR: a exclusão da 1.0 permanece, e passa a existir um destino declarado para a decisão futura.
- [ADR-011](ADR-011-pilha-propria.md) ganha a nota de que runtime de ML em job offline exigirá emenda própria.
- [Privacidade e licenças](../security/02-privacidade-licencas.md) passa a tratar pesos de modelo como artefatos de terceiros no inventário de dependências.
- Novas decisões registradas: D-IA-01 (fase futura, não 1.0), D-IA-02 (opt-in e download explícito), D-IA-03 (CPU piso, GPU com fallback), D-IA-04 (runtime de terceiros só em job offline, por emenda à ADR-011), D-IA-05 (licença de pesos bloqueia release).
- **Nenhum código, nenhuma dependência, nenhum artefato.** O build da 1.0 é idêntico depois desta ADR.
- Fica explícito o que antes era implícito: se a capacidade nunca for admitida, isso é uma decisão tomada, não um esquecimento.
