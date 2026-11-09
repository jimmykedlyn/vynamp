# Evidência TASK-E19-06 — repositórios públicos e branches protegidas

Versão 0.1.0 · 2026-11-08 · Autor Claude. Estado remoto lido da API do GitHub em
2026-11-08; **não** é release, não cria tag e não muda código de produto.

## Escopo executado

Dez repositórios `jimmykedlyn/*` — o umbrella `vynamp`, as cinco bibliotecas
(`vynamp-dsp`, `vynamp-decode`, `vynamp-encode`, `vynamp-output`,
`vynamp-engine`), as três aplicações (`vynamp-player`, `vynamp-converter`,
`vynamp-restore`) e o repositório histórico `vynamp-mediaplayer` — ficaram
públicos, com `main` e `develop` protegidas, relato privado de vulnerabilidade
habilitado e description que descreve o repositório em vez de prometer abertura
futura.

| Item | Comando | Estado |
|---|---|---|
| Proteção em `main` e `develop` | `cargo xtask repo protect --repo <owner/name> --reviews 0` | aplicada nos 10 |
| PVR habilitado | `PUT repos/<owner>/<name>/private-vulnerability-reporting` | `enabled=true` nos 9 públicos sem arquivamento |
| Descriptions | `PATCH repos/<owner>/<name> -f description=...` | preenchidas e sem "abertura futura" |
| Wiki/projects | `PATCH ... -F has_wiki=false -F has_projects=false` | uniformes nos 9 módulos |
| Webhooks Woodpecker | `DELETE repos/<owner>/<name>/hooks/<id>` | `hooks=0` nos 9 |
| Arquivamento | `PATCH repos/<owner>/<name> -F archived=true` | `vynamp-mediaplayer`, `vitrola` |

## O que a API responde

`gh api repos/jimmykedlyn/<repo>/branches/<branch>/protection` nos 10
repositórios, em `main` **e** `develop` (20 leituras, todas iguais):

```
strict=true contexts=CI Gate admins=true force=false del=false
pr=true reviews=0 stale=true conv=true
```

`cargo xtask repo audit --repo jimmykedlyn/<repo>` nos 9 públicos devolve
`"pending": []` e `"protection": {"develop": "applied", "main": "applied"}`.

## Por que `--reviews 0`

Existe uma única conta colaboradora — verificado em
`repos/jimmykedlyn/vynamp/collaborators`, que devolve só o próprio mantenedor.
Com `enforce_admins=true` e `required_approving_review_count=1`, o autor não
consegue aprovar o próprio PR, e a conta única não tem quem aprove por ela: todo
PR ficaria permanentemente não mergeável. `engineering/03-gitflow.md` já
prescrevia `--reviews 0` para esse caso. `config/github.json` passou de
`review_count: 1` para `0`, coerente com o que foi aplicado.

## Por que `vynamp-mvp-reference` ficou de fora da proteção

`cargo xtask repo protect` exige uma `develop`, porque é o default branch que ele
impõe. O repositório de referência só tem `main` (e `history/chatgpt-mvp`) e não
tem workflow nenhum, então exigir `CI Gate` ali tornaria todo PR impossível de
mergear — a proteção seria um bloqueio, não uma garantia. Ele continua público e
com o PVR habilitado; o que ficou de fora é só a regra de branch.

## Correções aplicadas no caminho

1. **PVR estava desligado nos nove.** O recurso nasceu desabilitado e nenhum
   passo anterior o ligou; `private_vulnerability_reporting` respondia
   `enabled=false` para os nove. `AC-1` não estava atendido até esta TASK.
2. **A descrição de `vynamp-output` prometia abertura futura.** Era
   `"... sem dependências de terceiros (privada; abertura futura)"`, exatamente o
   que `AC-3` proíbe, e era a única das dez com descrição preenchida. Trocada
   pelas nove descrições derivadas dos READMEs dos módulos.
3. **`SECURITY.md`, `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md` e `README.md`
   mandavam o leitor abrir issue pública** porque "o repositório é privado e o
   canal entra no ar na TASK-E19-06", e as URLs de advisory apontavam para
   `vynamp-mediaplayer`, que agora está arquivado. Reapontadas para
   `jimmykedlyn/vynamp`.

## O que esta TASK não faz

- Não assina nada, não cria tag e não publica release.
- Não dota os seis repositórios de `LICENSE`, `SECURITY.md` e
  `CONTRIBUTING.md`: `vynamp-decode` e `vynamp-output` já os têm, os outros seis
  não. Isso é a TASK-E19-04 (D-GH-13), e é uma pendência real — os repositórios
  estão públicos sem licença na raiz.
- Não revoga o JWT do Woodpecker. Os dois hooks foram removidos
  (`702255365` em decode, `703016033` em output) e `hooks=0` confirma; o servidor
  e o token continuam existindo por decisão do mantenedor.
