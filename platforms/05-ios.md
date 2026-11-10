# Plataformas iOS

Versão: 1.0.0 · Data: 2026-11-10 · Estado: aprovada para a FASE 6.
Baseado em: [ADR-006](../adr/ADR-006-multiplataforma.md), [ADR-016-ios](../adr/ADR-016-ios.md), [design/05-mobile-tablet.md](../design/05-mobile-tablet.md).

## 1. Escopo e Decisões Arquiteturais

1. **Layout Adaptado (Touch-First):** O DSP clássico de desktop é adaptado para gestos iOS (sliders vertical de 44pt, bottom sheets, full-screen parametric editor).
2. **Sem Módulos de Conversão e Restauração:** Os apps `apps/converter` e `apps/restore` **não** existem no build iOS. O player embarca apenas o decodificador, o motor de áudio e a interface SwiftUI.
3. **Pilha de Áudio (CoreAudio / AVAudioEngine / RemoteIO):** Uso obrigatório de CoreAudio em thread Real-Time isolada, com ring buffer SPSC sem alocação dinâmica.
4. **Sandboxing e Permissões:** Acesso restrito ao container do app e à biblioteca de mídia via `MediaPlayer` framework e arquivos permitidos pelo usuário.

## 2. Critérios de Aceite da Fase 6 (iOS)

| ID | Given | When | Then |
|---|---|---|---|
| `IOS-AC-1` | o binário compilado para `aarch64-apple-ios` | executado em simulador/device arm64 | inicialização sem violação de sandbox e integração de playback |
| `IOS-AC-2` | a reprodução ativa em background | o app entra em segundo plano | a sessão de áudio (`AVAudioSession`) mantém o playback sem interrupção |
| `IOS-AC-3` | o layout em viewport de iPhone SE (375x667) | renderizado pelo shell SwiftUI/Swift | controles de DSP e playlist acessíveis sem scroll horizontal |
