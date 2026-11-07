# Vynamp Restore — Super-resolução por IA (AudioSR em Rust, CPU por padrão, GPU opcional)

> Especificação para o produto nativo. **Status: documentado, não implementado.**
> No MVP web (rev.49) já existem os caminhos DSP em tempo real: extensão de banda
> (`bwe`/`bweCutoff`) e remoção de artefatos MP3 (`deartifact`). Este documento
> define a opção **IA**, que vem depois.

## Decisão

- **Modelo:** [AudioSR](https://github.com/haoheliu/versatile_audio_super_resolution).
  É uma super-resolução de áudio "versátil" por difusão latente: entra áudio
  limitado em banda (2–16 kHz de corte) e sai áudio em 48 kHz.
- **Linguagem:** Rust. Não há Python em tempo de execução.
- **Hardware:** **CPU é o padrão e o requisito mínimo.** A GPU é uma aceleração opcional (ver "Backends"). Tudo precisa funcionar sem GPU.
- **Modo:** offline (job em segundo plano / exportação), nunca em tempo real.
- **Opcional de ponta a ponta:** o recurso vem **desligado**. O usuário decide se quer ativá-lo, e o download do modelo nunca é automático (ver "Ativação e download").

## Ativação e download (opt-in)

- **Recurso desligado por padrão.** Em Preferências → Restore há a chave "Super-resolução por IA".
  - Desligada: a ferramenta não aparece no Restore, ou aparece só como aviso "desativada", e nada relacionado a ela é carregado.
  - O player e o Restore DSP (`bwe`/`deartifact`) funcionam integralmente sem ela.
- **Download opcional e explícito.** Ativar a chave **não** baixa nada.
  - O download só começa quando o usuário clica em "Baixar modelo", depois de ver o tamanho, a fonte e a licença.
  - Pode ser adiado, cancelado ou retomado.
- **Escolha de variante:** `basic` (música) e `speech` são baixadas individualmente. O usuário pode ter nenhuma, uma ou as duas.
- **Remoção:** botão "Remover modelo" que libera o disco. Desativar a chave **não** apaga o modelo automaticamente; o app pergunta antes.
- **Instalação manual (offline):** o usuário pode apontar uma pasta com os pesos já convertidos, sem passar pelo download do app. O checksum é validado do mesmo jeito.
- **Sem telemetria e sem rede** além do download pedido pelo usuário.
- **Build:** o suporte inteiro fica atrás da feature Cargo `audiosr` (e `audiosr-cuda`/`audiosr-metal`/`audiosr-wgpu` para GPU). Um build sem essas features não tem nenhuma dependência de ML.

## Backends (CPU padrão, GPU opcional)

| Backend | Quando | Observação |
|---|---|---|
| **CPU** | Sempre disponível; é o padrão | Referência de corretude; todos os testes de paridade rodam nele |
| **CUDA** (NVIDIA) | Opcional | `candle` CUDA ou `ort` com CUDA EP |
| **Metal** (macOS) | Opcional | `candle` Metal ou `ort` com CoreML EP |
| **DirectML / Vulkan / wgpu** | Opcional, avaliar | `ort` DirectML no Windows; wgpu é experimental |

- **Seleção:** Preferências → "Dispositivo de processamento", com as opções Automático / CPU / GPU (lista das detectadas).
  - Automático usa a GPU se ela for detectada **e** passar num teste rápido de memória e paridade. Caso contrário, usa a CPU.
- **Fallback:** se a GPU falhar em qualquer ponto (driver, VRAM insuficiente, operador sem suporte), o job volta à CPU automaticamente, avisa o usuário e continua do bloco onde parou.
- **Precisão:** fp32 na CPU. Na GPU, fp16 é opcional (padrão fp32) e precisa passar no mesmo critério de escuta e A/B.
- **Determinismo:** só é garantido bit-exato no mesmo backend e na mesma máquina. Entre CPU e GPU vale a tolerância de paridade.
- **Teto de VRAM** configurável, igual ao teto de RAM.

## Runtime

- **Primeira opção:** [`candle`](https://github.com/huggingface/candle), com pesos em `safetensors`.
- **Alternativa:** [`ort`](https://github.com/pykeio/ort), o ONNX Runtime em CPU, com os componentes exportados para ONNX.
  - [Tract](https://github.com/sonos/tract) é o runtime já previsto em `SOURCE_SEPARATION_MODELS.md`.
  - Usar o Tract aqui só se ele suportar os operadores do UNet de difusão. Avaliar no spike.
- **Paralelismo:** usar `rayon` e os threads intra-op do runtime. O número de threads deve ser configurável.

### Componentes a portar

| # | Componente | Função |
|---|---|---|
| 1 | Pré-processamento | Resample para 48 kHz, STFT e mel conforme a configuração do AudioSR (mesmos parâmetros, com teste de paridade) |
| 2 | Encoder VAE | Mel do áudio de baixa resolução → latente de condicionamento |
| 3 | UNet de difusão latente | Remoção de ruído condicionada |
| 4 | Sampler DDIM | Por padrão ~50 passos, configurável (menos passos = mais rápido e pior) |
| 5 | Decoder VAE | Latente → mel |
| 6 | Vocoder (estilo HiFi-GAN) | Mel → forma de onda em 48 kHz |

### Pesos

- Vêm dos checkpoints oficiais do AudioSR no Hugging Face (`basic` e `speech`). O tamanho estimado é de ~1–2 GB, a confirmar.
- São convertidos uma única vez, fora do app, para `safetensors` ou ONNX, por script de conversão versionado.
  - O app só carrega o arquivo convertido.
- **Não** são embutidos no instalador.
  - Download opcional, só a pedido do usuário (ver "Ativação e download"), para `%APPDATA%/vynamp/models/audiosr/`.
  - O checksum SHA-256 é verificado.
- A licença dos pesos e do código deve ser conferida antes de distribuir. Isso é bloqueante para o release.

## Processamento

1. **Diagnóstico:** o detector `band_limit` (em `restore-diagnostics`) estima a frequência de corte (ex.: Track07 → 15,8 kHz).
   - Sem `band_limit`, a ferramenta fica desabilitada com o aviso "áudio já tem banda completa".
2. **Segmentação:** blocos de ~10 s (o tamanho de entrada do modelo), com sobreposição de ~1 s e crossfade equal-power. Cada canal é processado separadamente.
3. **Inferência:** cada bloco passa pelo pipeline acima. A seed é fixa e configurável, para que o resultado seja determinístico.
4. **Substituição só da banda alta.** O conteúdo gerado é usado **apenas acima do corte**.
   - Abaixo do corte fica o áudio original, com crossover linear-phase e transição de ~1 kHz em torno do corte.
   - Isso evita que o modelo altere timbre, fase ou dinâmica da parte que já é fiel.
5. **Pós:** casar o ganho da banda gerada com a inclinação espectral logo abaixo do corte, depois passar por um true-peak limiter.
6. **Saída:** arquivo novo em 48 kHz, 24-bit ou float. O original nunca é sobrescrito.

## Desempenho esperado (estimativa — medir)

- Em CPU desktop, a estimativa é de **vários minutos por música** com 50 passos DDIM. É ordem de grandeza: o valor real precisa ser medido no spike.
- RAM: modelo carregado mais ativações, provavelmente 3–6 GB. Precisa de um teto configurável e de abortar com erro claro quando ele é excedido.
- Por isso a operação é um **job em segundo plano**, com:
  - progresso por bloco e ETA;
  - cancelamento a qualquer momento, sem deixar arquivo parcial;
  - prioridade baixa de thread, para não travar a reprodução.

## Integração no Restore

- Nova ferramenta **"Super-resolução (IA)"** na seção de agudos, ao lado de "Extensão de banda".
  - Rótulo: "offline — gera um novo arquivo".
- O caminho em tempo real continua sendo o DSP (`bwe` + `deartifact`). A IA é a opção "qualidade máxima" de exportação.
- O preview faz A/B entre original, DSP `bwe` e IA em um trecho de 10 s, processado sob demanda.
- Se o recurso estiver desligado, nada aparece além de um link para Preferências.
- Se o recurso estiver ligado mas o modelo não estiver instalado, a ferramenta mostra o tamanho e o botão "Baixar". Nada é baixado sem esse clique.
- O dispositivo em uso (CPU ou nome da GPU) aparece no painel do job.

## Critérios de aceite

- **Paridade:** em 3 clipes de referência, o mel do pré-processamento e a saída de cada componente (VAE, UNet em 1 passo, vocoder) batem com a implementação Python original, com tolerância a definir (ex.: erro relativo < 1e-3).
- **Track07 (corte em 15,8 kHz):**
  - a energia em 17–20 kHz fica ≥ ao resultado do DSP `bwe` 0.5 (−29,6 dB);
  - as métricas de crackle (k6) não pioram em relação à entrada restaurada (2,57);
  - abaixo de 15 kHz a diferença em relação ao original é ≈ 0, só pelo crossover.
- **Determinismo:** mesma entrada e mesma seed produzem o mesmo arquivo (bit-exato na mesma máquina).
- **Recursos:** a RAM fica dentro do teto configurado, e o cancelamento libera memória e threads em < 1 s.
- **Opt-in:** numa instalação limpa, nenhum byte de modelo é baixado e nenhuma dependência de ML é carregada. O player e o Restore DSP passam em todos os testes com o recurso desligado.
- **GPU:** quando disponível, a saída fica dentro da tolerância de paridade em relação à CPU. Uma falha simulada da GPU no meio do job cai na CPU sem perder o progresso.
- **Escuta:** A/B cego, original × DSP × IA, em ≥ 3 faixas MP3 128–192 kbps.

## Riscos

- **Tempo de CPU:** pode ser inviável para álbuns inteiros. Mitigar reduzindo os passos DDIM (25) e processando em lote, à noite.
- **RAM:** em máquinas com 8 GB pode faltar memória. Mitigar com blocos menores e execução sequencial.
- **Alucinação:** a banda alta inventada pode soar artificial ou chiada. A mitigação é a substituição só acima do corte, mais o A/B obrigatório.
- **Port:** os operadores do UNet podem faltar no `candle`/`tract`. O plano B é o `ort`.
- **Licença:** se os pesos não puderem ser redistribuídos, o usuário baixa da fonte oficial e converte localmente.

## Fora do escopo do MVP

- Exigir GPU: ela é sempre opcional. O MVP entrega CPU, e um backend de GPU (CUDA primeiro) entra na fase 7 abaixo.
- Tempo real ou aplicar durante a reprodução.
- Treinar ou fazer fine-tuning do modelo.
- Outros modelos de super-resolução (AP-BWE, NU-Wave etc.). Ficam como candidatos futuros, caso o AudioSR se mostre lento demais.

## Ordem de implementação sugerida

1. **Spike:** carregar os pesos convertidos no `candle`, rodar 1 bloco de 10 s e medir tempo e RAM. Isso decide entre `candle`, `ort` e `tract`.
2. Pré-processamento e vocoder, com testes de paridade.
3. VAE, UNet e DDIM, com teste de paridade de 1 passo.
4. Segmentação, crossfade e substituição de banda alta.
5. Job em segundo plano (progresso, cancelamento, teto de RAM).
6. UI no Restore, chave de ativação, download opcional e A/B.
7. Backends de GPU opcionais (CUDA, depois Metal/DirectML), com fallback para CPU.
