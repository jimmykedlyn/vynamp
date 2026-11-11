# Plataformas iOS

Versão: 1.1.0 · Data: 2026-11-11 · Estado: aprovada para a FASE 6.
Baseado em: [ADR-006](../adr/ADR-006-multiplataforma.md), [ADR-016-ios](../adr/ADR-016-ios.md), [design/05-mobile-tablet.md](../design/05-mobile-tablet.md), [design/06-painel-dsp.md](../design/06-painel-dsp.md).

## 1. Escopo e Decisões Arquiteturais

1. **Layout Adaptado (Touch-First):** O DSP clássico de desktop é adaptado para gestos iOS (sliders vertical de 44pt, bottom sheets, full-screen parametric editor). O mapeamento área por área está em [painel DSP §10](../design/06-painel-dsp.md#10-layout-no-toque-android-e-ios); os limites de ajuste de [painel §8.5](../design/06-painel-dsp.md#85-limites-do-ajuste) não mudam com o toque.
2. **Sem Módulos de Conversão e Restauração:** Os apps `apps/converter` e `apps/restore` **não** existem no build iOS. O player embarca apenas o decodificador, o motor de áudio e a interface SwiftUI. A decisão é do mantenedor, de 2026-11-03, registrada em [ADR-012 §1](../adr/ADR-012-integracao-shell.md#1-arquitetura-de-integração-por-plataforma) e em [D-MOB-01](../product/05-decisoes.md#decisões-de-mobile-2026-11-11).
3. **Pilha de Áudio (CoreAudio / AVAudioEngine / RemoteIO):** Uso obrigatório de CoreAudio em thread Real-Time isolada, com ring buffer SPSC sem alocação dinâmica.
4. **Sandboxing e Permissões:** Acesso restrito ao container do app e à biblioteca de mídia via `MediaPlayer` framework e arquivos permitidos pelo usuário.

## 2. Critérios de Aceite da Fase 6 (iOS)

| ID | Given | When | Then |
|---|---|---|---|
| `IOS-AC-1` | o binário compilado para `aarch64-apple-ios` | executado em simulador/device arm64 | inicialização sem violação de sandbox e integração de playback |
| `IOS-AC-2` | a reprodução ativa em background | o app entra em segundo plano | a sessão de áudio (`AVAudioSession`) mantém o playback sem interrupção |
| `IOS-AC-3` | o layout em viewport de iPhone SE (375x667) | renderizado pelo shell SwiftUI/Swift | controles de DSP e playlist acessíveis sem scroll horizontal |
| `IOS-AC-4` | o binário de release do app | o bundle é inspecionado | **não** existe target, extensão nem deep-link de `vynamp-restore` ou `vynamp-converter`; o app embarca só decodificador, motor e shell SwiftUI | §1.2; [ADR-012 §1](../adr/ADR-012-integracao-shell.md#1-arquitetura-de-integração-por-plataforma) |
| `IOS-AC-5` | o painel DSP aberto em iPhone SE | a árvore de views é medida | todo alvo de toque tem ≥ 44 pt, e 20/31/64 e o paramétrico abrem em tela cheia | [painel §10](../design/06-painel-dsp.md#10-layout-no-toque-android-e-ios) |

Nenhum destes critérios está verificado: não há hardware Apple, simulador nem conta de desenvolvedor neste ambiente. Ausência de prova é limitação registrada, não aprovação.
