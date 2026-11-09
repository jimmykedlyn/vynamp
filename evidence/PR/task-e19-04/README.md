# Evidência TASK-E19-04 — specs re-apontadas e repos próprios

Versão 0.1.0 · 2026-11-09 · Autor Claude. Estado remoto lido da API do GitHub.
Este documento não cria tag, não muda código de produto e não declara nenhuma
capacidade implementada: os três módulos continuam em `Status: specification`.

## Escopo executado

| Item | Antes | Depois |
|---|---|---|
| `reference/tools/02-vynamp-converter.md` | citava `vynamp-encode` como "repositório privado", sem URL | cita as URLs públicas de `vynamp-encode` e `vynamp-converter` |
| `reference/tools/01-vynamp-restore.md` | nenhuma referência ao repo do módulo | cita a URL pública de `vynamp-restore` |
| `specs/README.md` dos três módulos | declarava "Normativas: o código segue a spec" — o repo de módulo se apresentava como fonte | declara que a fonte é o repositório principal e aponta o documento canônico |
| cabeçalho de `specs/*.md` dos três módulos | não dizia onde a norma vive | aponta `specs/README.md` e diz que este arquivo não é a norma |
| raiz dos três repos | sem LICENSE, SECURITY.md, CONTRIBUTING.md | `LICENSE`, `LICENSE-MIT`, `LICENSE-APACHE`, `SECURITY.md`, `CONTRIBUTING.md` |
| `README.md` de encode, converter e restore | sem o conteúdo exigido pelo AC-3 | bilíngue EN/PT, com `Status: specification` |

## AC-1 — as specs apontam para URLs reais

URLs citadas e lidas na API REST do GitHub em 2026-11-09:

```
$ gh api repos/jimmykedlyn/vynamp-encode
jimmykedlyn/vynamp-encode vis=public private=false default=develop

$ gh api repos/jimmykedlyn/vynamp-converter
jimmykedlyn/vynamp-converter vis=public private=false default=develop

$ gh api repos/jimmykedlyn/vynamp-restore
jimmykedlyn/vynamp-restore vis=public private=false default=develop
```

Nenhuma URL residual aponta `vynamp-fases` — o diretório `D:\Projects\vynamp-fases`
é material de planejamento local e **não** é repositório (`GET repos/jimmykedlyn/vynamp-fases`
devolve 404). Por isso a referência a `01-arvore-final.md` nos esqueletos é textual,
sem link.

## AC-2 — a fonte normativa continua única

A norma dos três módulos vive em `reference/tools/` no repositório principal:
`01-vynamp-restore.md`, `02-vynamp-converter.md` e `03-vynamp-restore-ia.md`. Os
`specs/` dos repos de módulo são esqueleto — §1, §2 e §7 são o recorte aprovado do
passo 37 e §3 a §6, §8 a §10 estão `PENDENTE` de autoria. Nenhuma afirmação
normativa dos esqueletos existe só ali: cada uma repete o decidido em
`01-arvore-final.md` §2 ou na spec canônica correspondente.

O `encode` não tem documento canônico próprio: sua norma está em
`02-vynamp-converter.md` §1 (motor de codificação) — é o que o `specs/README.md`
dele aponta agora.

## AC-3 — os três repos existem, públicos, com README bilíngue, LICENSE e SECURITY.md

Conteúdo da raiz lido por `gh api repos/jimmykedlyn/<repo>/contents/`:

```
vynamp-encode:    README.md specs src Cargo.toml Cargo.lock ...   + LICENSE SECURITY.md CONTRIBUTING.md
vynamp-converter: README.md specs tasks src Cargo.toml ...        + LICENSE SECURITY.md CONTRIBUTING.md
vynamp-restore:   README.md specs tasks src Cargo.toml ...        + LICENSE SECURITY.md CONTRIBUTING.md
```

A coluna de governança entra com este PR; `visibility=public` é o que a API já
responde acima. O README de cada um abre em inglês com
`**English** · [Português](#português--<slug>)` e repete o conteúdo em português.

## O que esta TASK não fez

- Não autorou §3 a §10 de nenhuma spec de módulo.
- Não moveu texto normativo para os repos de módulo.
- Não implementou encoder, conversor nem restaurador.
- Não mudou visibilidade nem branch protection (é a TASK-E19-06, executada).
