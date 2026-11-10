# ADR-002 — núcleo Rust e backends por capacidade

Versão: 0.8.0 · Data: 2026-11-03 · Estado: especificação proposta, implementação não iniciada.

Status: **não congelado**; o gate declarado é a TASK-E01-01 (e a E01-02 já fechada em 2026-11-09). Parcialmente substituída por [ADR-011](ADR-011-pilha-propria.md) (decode, saída, resampling, tags).

Decisão: domain/engine/control em Rust, decode e output por adapters; RT callback restrita; caminho transparente inteiro separado de DSP float. Symphonia/CPAL candidatos, WASAPI avançado adapter dedicado se necessário. Alternativa: wrapper de playback de alto nível; custo de ocultar clock/gapless/negotiation. Consequência: mais trabalho de engine, mas critérios verificáveis; bibliotecas nativas admitidas com isolation/license.

## Registro do passo 43 — não congelado, com o motivo medido

Medido em 2026-11-09. O gate declarado na linha 5 desta ADR é a TASK-E01-01, que
mede a suíte de decodificação e **não** fecha o spike que o `deliverable` pede:

| O que o gate exige | Medido | Situação |
|---|---|---|
| backend candidato escolhido no ADR-002 e confrontado com o catálogo | `grep` por candidatos no `libs/decode`: o `Cargo.toml` **não tem Symphonia nem CPAL**, e `config/format-catalog.json` nomeia backend real em **só 2 das 68 entradas** (`flac-native`, `mp3`, ambos `vynamp-decode (ADR-011)`) | aberto |
| spike de decoder registry | não existe: `find . -name '*.rs' | grep -i registry` vazio | aberto |
| ADR-002 "com versões/features propostas" | a ADR tem uma página e nenhum número de versão nem lista de features | aberto |
| ADR-011 já substituiu parte desta ADR | `adr/ADR-011-pilha-propria.md:5`: "Substitui as partes de ADR-002 sobre decode, saída, resampling e tags" | decidido |

**Por que congelar agora seria falsificar.** A ADR decide "decode e output por
adapters", com Symphonia/CPAL como candidatos. A ADR-011, aceita pelo mantenedor em
2026-11-03, substituiu exatamente essas partes e o catálogo nomeia `vynamp-decode`
como backend. Congelar a decisão de candidatos de terceiro num documento que a
decisão posterior já derrubou criaria duas fontes de verdade para a mesma escolha —
que é o mesmo defeito que D-MVP-01 matou nos SKIN.

**Custo se esta leitura estiver errada:** se o mantenedor considerar que congelar
após E01-01 significa apenas "as decisões que a E01-01 não derrubou", então ADR-002
pode ser congelada **como resíduo do que ADR-011 não cobre** — ou seja, como
"domain/engine/control em RT, caminho transparente inteiro separado de DSP float",
sem os candidatos. Essa é uma emenda, não um congelamento do documento como está,
e ela é escrita em ADR-002, não aqui.

**Se o mantenedor quiser congelar assim:** a frase a trocar é o `Status` acima, e o
que sai da ADR é a menção a Symphonia/CPAL. O registro fica no commit do passo 43.
