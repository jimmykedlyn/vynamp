# reference

Referencia read-only, fora do workspace e do build.

- `vynamp-mvp/` -> vynamp-mvp-reference (pin 741316f, rev.38) — submodulo, passo 35
- `audio/` e `tools/` — specs dos modulos de audio e das ferramentas, congeladas

`audio/` e `tools/` vieram de `docs/audio` e `docs/tools` no passo 31. Nao sao
dominio do umbrella: viram spec de `libs/*` e `apps/*` quando cada repositorio
de modulo for criado, e o arquivo correspondente sai daqui naquele passo. Ate
la ficam aqui porque 247 links vivos de `architecture/`, `product/`, `quality/`
e `tasks/` apontam para eles, e `_arquivo/` nao pode receber link.

Nunca editar. Se a referencia divergir da spec, a spec registra a divergencia e o motivo.
