# Vynamp Restore — restauração por IA (pós-1.0, planejada e não prometida)

Versão 0.1.0 · Data: 2026-11-05 · Estado: **especificação de fase futura; nada implementado, nada prometido, nada embutido.**

Documento canônico das duas capacidades de restauração por aprendizado de máquina que o produto **considera** para depois da 1.0: **super-resolução de banda** (AudioSR) e **separação de fontes** (MDX-Net / Demucs). Complementa [spec do Restore](01-vynamp-restore.md), [ADR-013](../../adr/ADR-013-restore.md), [ADR-011](../../adr/ADR-011-pilha-propria.md) e [privacy/licensing](../../security/02-privacidade-licencas.md).

## 1. Estado e relação com a 1.0

### 1.1 O que este documento **não** faz

Este documento **não** altera o escopo da 1.0. A [ADR-013 §4](../../adr/ADR-013-restore.md#4-correção-das-incoerências-do-mvp) permanece integralmente válida:

| Item | Estado na 1.0 | Efeito deste documento |
|---|---|---|
| Região `overlay` | **Removida.** A 1.0 tem apenas regiões de ganho com fade (`kind: between`) | Nenhum. A separação de fontes, se um dia entrar, entra por decisão nova e com o `kind` reaberto explicitamente |
| Separação de voz por IA | "Decisão futura e não é prometida" | **Continua não prometida.** Este documento descreve a decisão — custos, contratos, riscos — sem tomá-la |
| Upscaling de hi-res / aumento real de resolução | Fora de escopo: extensão de banda é **síntese**, não recuperação | Mantido. A super-resolução é síntese de banda alta e será rotulada assim, nunca como "recuperação" |
| Denoise espectral (Wiener) | Fora de escopo na 1.0 | Nenhum. Não é capacidade de IA neste documento |

### 1.2 Por que existe

Duas capacidades foram especificadas antes da ADR-013 fechar o escopo: `AUDIOSR-RUST-CPU.md` e `SOURCE_SEPARATION_MODELS.md`. Ambas descreviam pipelines concretos com escolha de runtime, modelo, download e critérios de aceite, e nenhuma tinha representação no repositório — nem spec, nem requisito, nem ADR, nem fase de roadmap. O risco real disso não é a falta da capacidade: é a **especificação órfã**, que existe em nível de detalhe de implementação e não é normativa, então ninguém sabe se o produto promete ou não.

Este documento resolve a orfandade sem promover o escopo: registra o que já foi pensado, sob que condições poderia ser admitido, e o que o bloqueia.

### 1.3 Condições para sair de "planejada"

Nenhuma destas condições está satisfeita hoje. Enquanto qualquer uma faltar, a capacidade **não** entra em milestone nem em release:

1. **Licença dos pesos conferida** — redistribuição e uso comercial, por escrito, para cada modelo listado em §4. É bloqueante de release (§8.2).
2. **Spike concluído** — um bloco de 10 s rodando no runtime escolhido, com tempo e RAM medidos (§9).
3. **Orçamento de download aceito** — o maior modelo passa de 300 MB; a política de download explícito (§6) precisa ser aceita como parte da UX.
4. **Decisão registrada em [05-decisions](../../product/05-decisoes.md)** com ID próprio, como toda decisão de escopo.
5. **Milestone atribuído** — hoje não há item de ML em nenhuma fase de [01-roadmap](../../delivery/01-roadmap.md).

## 2. Alinhamento com a pilha própria (ADR-011)

A [ADR-011](../../adr/ADR-011-pilha-propria.md) decidiu não usar terceiros no caminho de áudio: `vynamp-decode` e `vynamp-output` existem para substituir `symphonia`, `cpal`, `rubato` e `lofty`.

Uma capacidade de IA **não** é exceção a essa decisão, e também não a viola:

| Camada | Decisão | Por quê |
|---|---|---|
| **Caminho de áudio em tempo real** | Continua 100% próprio. Nenhum runtime de ML entra na callback | A ADR-011 é sobre determinismo e controle de latência no playback. Inferência não tem lugar ali |
| **Job offline** | Pode usar runtime de terceiros (`candle`, `ort` ou `tract`), atrás de feature Cargo | É processamento em lote, fora da reprodução, com resultado auditável. A troca fica registrada em ADR nova, não presumida |
| **Dependência transitiva** | Um build sem as features de ML **não** tem nenhuma dependência de ML | Mantém o player instalável e auditável na 1.0 e depois |

**Consequência obrigatória:** habilitar IA exige uma ADR nova que emende a ADR-011 com uma exceção nomeada e datada. Este documento **não** é essa ADR; ele é a entrada dela.

## 3. CPU primeiro, GPU opcional

O requisito mínimo é CPU. GPU é aceleração, nunca pré-requisito.

| Backend | Disponibilidade | Papel |
|---|---|---|
| **CPU** | Sempre; é o padrão | Referência de corretude. Todos os testes de paridade rodam nela |
| CUDA (NVIDIA) | Opcional | Aceleração em jobs longos |
| Metal (macOS) | Opcional | Idem, em Apple Silicon |
| DirectML / Vulkan / wgpu | Opcional, a avaliar | Sem compromisso |

Regras de seleção, todas obrigatórias quando a capacidade existir:

- **Automático** usa a GPU só se ela for detectada **e** passar num teste rápido de memória e paridade. Caso contrário, CPU.
- **Fallback** — falha de GPU em qualquer ponto (driver, VRAM, operador sem suporte) volta à CPU, avisa e continua do bloco onde parou. Nunca reinicia o job do zero em silêncio.
- **Precisão** — fp32 na CPU; fp16 na GPU é opcional e precisa passar no mesmo critério de escuta e A/B.
- **Determinismo** — bit-exato só no mesmo backend e na mesma máquina. Entre CPU e GPU vale tolerância de paridade declarada.
- **Teto de VRAM** configurável, com a mesma semântica do teto de RAM do job (§9).

## 4. Modelos e o que cada um entrega

Nenhum modelo é embutido no instalador. Todos são baixados a pedido (§6).

### 4.1 Super-resolução de banda

| Item | Valor |
|---|---|
| Modelo | [AudioSR](https://github.com/haoheliu/versatile_audio_super_resolution) — super-resolução versátil por difusão latente |
| Entrada / saída | Áudio limitado em banda (corte entre 2 e 16 kHz) → 48 kHz |
| Variantes | `basic` (música) e `speech`, baixadas individualmente; o usuário pode ter nenhuma, uma ou as duas |
| Tamanho | ~1–2 GB por variante (estimativa; confirmar antes de exibir na UI) |
| Licença dos pesos | **A conferir. Bloqueante para release** (§8.2) |

Componentes a portar, na ordem do pipeline:

| # | Componente | Função |
|---|---|---|
| 1 | Pré-processamento | Resample para 48 kHz, STFT e mel com os mesmos parâmetros do original, com teste de paridade |
| 2 | Encoder VAE | Mel de baixa resolução → latente de condicionamento |
| 3 | UNet de difusão latente | Remoção de ruído condicionada |
| 4 | Sampler DDIM | ~50 passos por padrão, configurável; menos passos = mais rápido e pior |
| 5 | Decoder VAE | Latente → mel |
| 6 | Vocoder (estilo HiFi-GAN) | Mel → forma de onda em 48 kHz |

### 4.2 Separação de fontes

| Modelo | Tamanho | Stems | Uso recomendado |
|---|---|---|---|
| MDX-Net B (Kim Inst) | ~30 MB | vocais / instrumental | Padrão — rápido, leve, bom para remoção de voz |
| Kim_Vocal_2 | ~50 MB | vocais / instrumental | Alternativa, assinatura tonal diferente |
| Demucs v3 (`mdx_extra`) | ~80 MB | 4 stems (vocais, bateria, baixo, outro) | Quando precisa de stems individuais |
| htdemucs-ft (vocals) | ~100 MB | vocais / instrumental | Alta fidelidade |
| htdemucs-ft (4 stems) | ~316 MB | 4 stems | Qualidade máxima; baixar só se necessário |

A separação age **apenas na máscara espectral** das regiões marcadas. O kernel DSP de restauração continua como pré e pós-processamento; o modelo não substitui a cadeia de §3 da [spec do Restore](01-vynamp-restore.md).

**Dependência de escopo:** separar por região exige o `kind: overlay` que a 1.0 removeu. Admitir separação significa reabrir esse tipo por decisão explícita — não por extensão silenciosa.

### 4.3 Runtime

**Primeira opção:** [`candle`](https://github.com/huggingface/candle) com pesos em `safetensors`.
**Alternativa:** [`ort`](https://github.com/pykeio/ort), ONNX Runtime, com componentes exportados para ONNX.
**Terceira:** [`tract`](https://github.com/sonos/tract) — só se suportar os operadores do UNet de difusão. Avaliar no spike.

Paralelismo com `rayon` e threads intra-op do runtime, com número de threads configurável.

## 5. Por que não embutir

- O maior modelo passa de **300 MB**; o de super-resolução chega a ~2 GB. Inviável no instalador.
- A maioria dos usuários precisa só de remoção de voz (~30–50 MB). Baixar 4 stems para todos seria desperdício de banda e disco.
- Download sob demanda respeita a conexão e o espaço do usuário, e mantém o pacote base pequeno.

## 6. Ativação, download e remoção (opt-in de ponta a ponta)

O recurso vem **desligado**. Nada relacionado a ele é carregado, e o player e o Restore DSP funcionam integralmente sem ele.

| Passo | Regra |
|---|---|
| **Ativar** | Chave em Preferências → Restore. Ativar **não** baixa nada |
| **Escolher** | Lista de modelos com tamanho, fonte e licença visíveis **antes** do botão |
| **Baixar** | Só por clique explícito, depois de ver tamanho/fonte/licença. Pode ser adiado, cancelado ou retomado |
| **Verificar** | Checksum SHA-256 obrigatório após o download. Falha descarta o arquivo e informa |
| **Guardar** | `%APPDATA%/vynamp/models/` (ou equivalente por SO), disponível offline depois |
| **Instalar offline** | O usuário pode apontar uma pasta com pesos já convertidos, sem passar pelo download; o checksum é validado do mesmo jeito |
| **Remover** | Botão explícito que libera o disco. Desativar a chave **não** apaga automaticamente — o app pergunta antes |
| **Rede** | Nenhuma conexão além do download pedido. Sem telemetria. Sem download automático, nunca |

A política de imagem e artefatos de [privacy/licensing](../../security/02-privacidade-licencas.md) é estendida por este documento: pesos de modelo são artefatos de terceiros e entram no inventário de dependências, com checksum e origem registrados.

## 7. Processamento

### 7.1 Super-resolução

1. **Diagnóstico:** o detector `band_limit` (já normativo, [spec §4.2](01-vynamp-restore.md)) estima a frequência de corte. Sem `band_limit`, a ferramenta fica desabilitada com o aviso "áudio já tem banda completa".
2. **Segmentação:** blocos de ~10 s (tamanho de entrada do modelo), sobreposição de ~1 s, crossfade *equal-power*. Cada canal é processado separadamente.
3. **Inferência:** cada bloco passa pelo pipeline de §4.1. A seed é fixa e configurável, para resultado determinístico.
4. **Substituição só da banda alta.** O conteúdo gerado é usado **apenas acima do corte**. Abaixo do corte fica o áudio original, com crossover linear-phase e transição de ~1 kHz em torno do corte. Isso impede que o modelo altere timbre, fase ou dinâmica da parte que já é fiel.
5. **Pós:** casar o ganho da banda gerada com a inclinação espectral logo abaixo do corte, depois true-peak limiter.
6. **Saída:** arquivo novo em 48 kHz, 24-bit ou float. O original **nunca** é sobrescrito, como já é regra na 1.0.

### 7.2 Separação de fontes

1. Região marcada define o intervalo e o modelo escolhido.
2. O modelo produz a máscara espectral do intervalo.
3. A máscara é aplicada pelo kernel DSP existente; o resultado entra na cadeia normal de restauração.
4. Monitor A/B/C da 1.0 (`original` / `treated` / `removed`) continua sendo a forma de inspeção — a separação não cria um quarto modo.

## 8. Requisitos e risco legal

### 8.1 Requisitos

| ID | Requisito |
|---|---|
| RSAI-01 | Recurso desligado por padrão; nenhum byte de modelo é baixado numa instalação limpa |
| RSAI-02 | Download só por clique explícito, depois de exibir tamanho, fonte e licença |
| RSAI-03 | Checksum SHA-256 verificado; falha descarta e informa |
| RSAI-04 | Nenhuma dependência de ML em build sem as features de IA |
| RSAI-05 | CPU é o padrão e o requisito mínimo; GPU é opcional com fallback automático para CPU |
| RSAI-06 | Falha de GPU no meio do job retoma do bloco onde parou, sem reiniciar |
| RSAI-07 | Teto de RAM e de VRAM configuráveis; exceder aborta com erro claro |
| RSAI-08 | Cancelamento a qualquer momento, sem deixar arquivo parcial |
| RSAI-09 | Super-resolução substitui apenas acima do corte, com crossover linear-phase |
| RSAI-10 | Saída sempre em arquivo novo; a origem nunca é sobrescrita |
| RSAI-11 | Remover o modelo é ação explícita; desativar a chave não apaga sozinho |
| RSAI-12 | Instalação manual de pesos por pasta do usuário, com o mesmo checksum |
| RSAI-13 | Determinismo bit-exato na mesma máquina, backend e seed |
| RSAI-14 | Licença de cada peso e do código-fonte conferida antes de qualquer release que as distribua |

### 8.2 O bloqueio legal, explicitado

**Pesos de modelo são obra de terceiros.** Distribuí-los sem licença clara é o mesmo erro que a [política de privacidade e licenças](../../security/02-privacidade-licencas.md) já proíbe para codecs: a licença do código do produto não concede direito sobre obra de terceiros embutida nele.

Regras, todas bloqueantes:

- A licença dos pesos de **cada** modelo listado em §4 é conferida antes de o modelo ser oferecido na UI.
- Se os pesos não puderem ser redistribuídos, o usuário baixa da fonte oficial e converte localmente — e o app diz isso antes do download.
- `release preflight` continua exigindo `THIRD_PARTY_NOTICES` e SBOM. Um modelo baixado pelo usuário **não** entra no instalador, então não muda essa exigência; um modelo embutido mudaria, e por isso nenhum é embutido.

## 9. Desempenho (estimativa — medir)

- **CPU:** a estimativa é de vários minutos por música com 50 passos DDIM. É ordem de grandeza; o valor real sai do spike.
- **RAM:** modelo carregado mais ativações, provavelmente 3–6 GB. Exige teto configurável e aborto com erro claro.
- Por isso a operação é **job em segundo plano**, com progresso por bloco, ETA, cancelamento, prioridade baixa de thread e retomada do bloco.

O job reutiliza o gerenciador de jobs e o contrato de cancelamento já normativos na 1.0 ([spec §6](01-vynamp-restore.md)); o que ele **acrescenta** é a granularidade por bloco e a retomada após falha de backend.

## 10. Critérios de aceite (quando a capacidade for admitida)

Nenhum destes critérios é exigível hoje — servem para a fase em que a decisão for tomada.

| Critério | Verificação |
|---|---|
| **Paridade de componentes** | Em 3 clipes de referência, mel do pré-processamento e saída de cada componente (VAE, UNet em 1 passo, vocoder) batem com a implementação de referência, com tolerância declarada |
| **Efeito restrito** | Abaixo do corte, a diferença em relação ao original é ≈ 0, só pelo crossover |
| **Determinismo** | Mesma entrada e mesma seed produzem arquivo bit-exato na mesma máquina |
| **Recursos** | RAM e VRAM dentro do teto; cancelamento libera memória e threads em < 1 s |
| **Opt-in** | Numa instalação limpa, nenhum byte de modelo é baixado e nenhuma dependência de ML é carregada |
| **GPU** | Com GPU disponível, a saída fica dentro da tolerância de paridade em relação à CPU; falha simulada cai para a CPU sem perder progresso |
| **Escuta** | A/B cego, original × DSP × IA, em ≥ 3 faixas com perda |

Métricas subjetivas de "qualidade" (notas por estrela, alvos de energia em faixa nomeada) **não** entram como critério normativo: são estimativas de outro documento e não foram medidas neste repositório.

## 11. Riscos

| Risco | Mitigação |
|---|---|
| **Tempo de CPU** pode inviabilizar álbuns inteiros | Reduzir passos DDIM, processar em lote, agendar para período ocioso |
| **RAM** insuficiente em máquinas de 8 GB | Blocos menores, execução sequencial, teto configurável |
| **Alucinação** — a banda inventada pode soar artificial ou chiada | Substituição só acima do corte, mais A/B obrigatório |
| **Port** — operadores do UNet podem faltar em `candle`/`tract` | Plano B é `ort`; o spike decide |
| **Licença** — pesos podem não ser redistribuíveis | Download da fonte oficial e conversão local; §8.2 |
| **Peso da dependência** — um runtime de ML é uma árvore de dependências grande | Features Cargo opcionais; build padrão sem nenhuma dependência de ML |

## 12. Fora de escopo, mesmo na fase futura

- Exigir GPU. CPU é o piso, sempre.
- Tempo real ou aplicação durante a reprodução.
- Treinar ou fazer fine-tuning de modelo.
- Outros modelos de super-resolução (AP-BWE, NU-Wave e similares) — candidatos, não compromisso.
- Denoise espectral (Wiener), que a ADR-013 mantém fora de escopo por razão própria.
- Prometer qualquer um destes itens no instalador, no README ou na UI antes de §1.3 estar satisfeito.

## 13. Ordem de implementação, se admitida

1. **Spike:** carregar os pesos convertidos no runtime candidato, rodar 1 bloco de 10 s, medir tempo e RAM. Decide entre `candle`, `ort` e `tract`.
2. Pré-processamento e vocoder, com testes de paridade.
3. VAE, UNet e DDIM, com teste de paridade de 1 passo.
4. Segmentação, crossfade e substituição de banda alta.
5. Job em segundo plano: progresso, cancelamento, tetos e retomada.
6. UI no Restore: chave de ativação, download opcional e A/B.
7. Backends de GPU opcionais (CUDA primeiro), com fallback para CPU.

A separação de fontes vem **depois** de a super-resolução estar estável, porque depende de reabrir `kind: overlay` — uma mudança de escopo própria.

## 14. Limitações deste documento

- **Nada foi medido.** Nenhum modelo foi baixado, nenhum runtime foi exercitado, nenhum spike foi executado. Tempo, RAM e qualidade são estimativas herdadas das especificações originais.
- **Nenhuma licença de peso foi conferida.** §8.2 é regra de processo, não resultado.
- **Nenhuma ADR emenda a ADR-011.** A exceção para runtime de terceiros em job offline está proposta aqui, não decidida.
- **Nenhum item de roadmap ou milestone.** Este documento não cria fase, requisito numerado em [03-requirements](../../product/03-requisitos.md) nem TASK em [backlog](../../tasks/backlog.json).
