# ADR-002 — núcleo Rust e backends por capacidade

Versão: 0.8.0 · Data: 2026-11-03 · Estado: especificação proposta, implementação não iniciada.

Status: proposto; congelar após TASK-E01-01/TASK-E01-02. Parcialmente substituída por [ADR-011](ADR-011-pilha-propria.md) (decode, saída, resampling, tags).

Decisão: domain/engine/control em Rust, decode e output por adapters; RT callback restrita; caminho transparente inteiro separado de DSP float. Symphonia/CPAL candidatos, WASAPI avançado adapter dedicado se necessário. Alternativa: wrapper de playback de alto nível; custo de ocultar clock/gapless/negotiation. Consequência: mais trabalho de engine, mas critérios verificáveis; bibliotecas nativas admitidas com isolation/license.
