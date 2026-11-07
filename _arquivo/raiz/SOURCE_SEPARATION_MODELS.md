# Vynamp Restore — Source Separation Models

> Documento de referência para a implementação nativa em Rust (Tract + ONNX).
> O usuário poderá escolher qual modelo baixar; nenhum modelo é embutido no instalador.

## Arquitetura de inferência

- **Runtime:** [Tract](https://github.com/sonos/tract) — runtime leve em Rust para ONNX/TensorFlow, otimizações SIMD, sem dependências externas.
- **Formato:** ONNX (exportação única do PyTorch; o modelo é carregado diretamente pelo binário).
- **Fallback:** CPU multithreaded (rayon). WebGPU é opcional para quem tem hardware compatível.
- **Integração:** o kernel DSP atual (`restore-dsp`) permanece como pré/pós-processamento; o modelo atua apenas na máscara espectral das regiões marcadas como `overlay`.

## Modelos recomendados

| Modelo | Tamanho | Stems | Qualidade | Uso recomendado |
|---|---|---|---|---|
| **MDX-Net B (Kim Inst)** | ~30 MB | vocais / instrumental | ★★★★☆ | Padrão — rápido, leve, excelente para remoção de voz |
| **Kim_Vocal_2** | ~50 MB | vocais / instrumental | ★★★★☆ | Alternativa ao MDX-Net B com assinatura tonal diferente |
| **Demucs v3 (mdx_extra)** | ~80 MB | 4 stems (vocais, bateria, baixo, outro) | ★★★★☆ | Quando precisa de stems individuais, não só vocal |
| **htdemucs-ft (vocals)** | ~100 MB | vocais / instrumental | ★★★★★ | Alta fidelidade — recomendado para audiófilos |
| **htdemucs-ft (4 stems)** | ~316 MB | 4 stems | ★★★★★ | Qualidade máxima; baixar apenas se necessário |

## Fluxo do usuário

1. **Primeira abertura do painel Restore:** a região `overlay` aparece com aviso "modelo de separação não instalado".
2. **Seletor de modelo:** dropdown com os modelos acima, mostrando tamanho e descrição. Botão "Baixar" ao lado de cada um.
3. **Download:** arquivo `.onnx` salvo em `%APPDATA%/vynamp/models/` (ou equivalente em macOS/Linux). Progresso visível.
4. **Cache:** uma vez baixado, o modelo permanece disponível offline. Usuário pode remover pelo painel de preferências.
5. **Inferência:** ao criar uma região `overlay` e clicar em "Separar", o Tract carrega o modelo escolhido e produz a máscara espectral aplicada em tempo real.

## Por que não embutir?

- Os modelos de alta qualidade (`htdemucs-ft`) chegam a **316 MB** — inviável para distribuir no instalador.
- A maioria dos usuários só precisa de remoção de voz (~30–50 MB); forçar o download de 4 stems seria desperdício.
- Respeita a conexão do usuário: modelos são baixados sob demanda.

## Referências

- [StemSplitio/htdemucs-ft-vocals-onnx (Hugging Face)](https://huggingface.co/StemSplitio/htdemucs-ft-vocals-onnx)
- [StemSplitio/htdemucs-ft-bass-onnx (Hugging Face)](https://huggingface.co/StemSplitio/htdemucs-ft-bass-onnx)
- [tract — runtime Rust para ONNX](https://github.com/sonos/tract)
- [candle — framework ML minimalista da Hugging Face](https://github.com/huggingface/candle)
- [Building Sentence Transformers in Rust with Burn, ONNX Runtime, and Candle](https://www.libhunt.com/posts/1455954-building-sentence-transformers-in-rust-a-practical-guide-with-burn-onnx-runtime-and-candle)

## MVP atual (referência de UX)

O Vynamp Restore em JavaScript serve apenas como referência de interação:
- Marcação de regiões `overlay` por arraste na waveform
- Preview A/B (original / tratado / só removido)
- Parâmetros de ganho e fade nas bordas
- Undo/redo e persistência em JSON

A implementação do modelo de separação fica **exclusivamente** para a fase Rust nativa. Nenhuma lógica de IA deve ser portada para o MVP JS.