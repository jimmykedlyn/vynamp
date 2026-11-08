# Vynamp

Player de audio multiplataforma em Rust, com ferramentas de conversao e restauracao.
O projeto e dividido em **um repositorio por modulo**, cada um com gitflow, spec e CI
proprios.

## Estado

Spec avancada; o codigo real existe em `vynamp-decode` e `vynamp-output`, em
branches de feature. As specs de modulo ainda nao publicadas ficam em
`reference/` (`audio/` e `tools/`), congeladas, e o passo correspondente do
plano de migracao as leva para o repositorio de cada modulo.

## Estrutura

| Caminho | Repo | Tipo |
|---|---|---|
| `libs/dsp` | `vynamp-dsp` | primitivas de sinal, sem dominio |
| `libs/decode` | `vynamp-decode` | decodificacao para PCM |
| `libs/encode` | `vynamp-encode` | codificacao |
| `libs/output` | `vynamp-output` | saida para device |
| `libs/engine` | `vynamp-engine` | integracao, dominio, DSP |
| `apps/player` | `vynamp-player` | player |
| `apps/converter` | `vynamp-converter` | conversor em lote |
| `apps/restore` | `vynamp-restore` | restauracao |
| `reference/vynamp-mvp` | `vynamp-mvp-reference` | referencia read-only |

Este repositorio contem as specs que atravessam modulos (`architecture/`, `product/`,
`design/`, ...), as ADRs, as TASKs cross-modulo e a ferramentaria (`xtask`).

## Direcao de dependencia

```
apps -> engine -> { decode, output, encode, dsp }
```

`decode`, `encode` e `output` nao dependem de nenhum crate `vynamp-*` de dominio.
Cada lib compila sozinha: `git clone` + `cargo test`, sem este repositorio.

## Comecar

Leia [AGENTS.md](AGENTS.md) — o contrato do agente, e tambem o melhor mapa da
arquitetura. Depois [SPEC.md](SPEC.md) e [tasks/NEXT.md](tasks/NEXT.md).

## Licenca

MIT ou Apache-2.0, a escolher por quem usa.
