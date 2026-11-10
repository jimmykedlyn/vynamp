# ADR-016: Suporte iOS

Versão: 0.1.0 · Data: 2026-11-10 · Estado: proposto, sujeito a aprovação pelo mantenedor.
Status: proposto; adotado pela FASE 6 do roadmap após aprovação do mantenedor.
Decisão: iOS (arm64 e Apple Silicon) é um alvo de expansão e qualidade para M3/M4, compartilhando o núcleo Rust (domain, decode, dsp, output, engine) via AAudio/CoreAudio. O shell é SwiftUI/ObjC++ com MediaPlayback e CoreAudio (AVAudioEngine ou RemoteIO). O acesso a arquivos utiliza Core File Provider e APIs de documentos.

Consequências: requisito de Apple Developer Program para assinatura e distribuição App Store; suporte de testes em dispositivos físicos iOS; validação de áudio de baixa latência e background playback.
