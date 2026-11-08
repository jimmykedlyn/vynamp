# Security Policy / Política de Segurança

Versão 0.9.0 · 2026-11-05 · Estado: política vigente para o repositório; o player ainda não está implementado.

## Reporting a vulnerability / Reportar uma vulnerabilidade

**Please do not open a public issue for a security problem.**
**Não abra issue pública para um problema de segurança.**

Use **GitHub Security Advisories** on this repository. The report is private by
default: only the maintainer and you can see it.

Use **GitHub Security Advisories** neste repositório. O relatório é privado por
padrão: só o mantenedor e você o veem.

- New advisory / Nova consulta: <https://github.com/jimmykedlyn/vynamp-mediaplayer/security/advisories/new>
- Existing advisories / Consultas publicadas: <https://github.com/jimmykedlyn/vynamp-mediaplayer/security/advisories>

**Not open yet.** The repository is private and private vulnerability reporting
is not enabled on it, so this channel goes live in TASK-E19-06, when the
repository becomes public and the feature is switched on. Until then the
advisory URLs above return 404 for anyone who is not the maintainer, and the
"Report a vulnerability" button does not appear. If you have something to report
before that, open a minimal public issue that asks for a private channel and
contains no details of the problem.

**Ainda não está aberto.** O repositório é privado e o recurso de relato privado
de vulnerabilidade não está habilitado nele, então este canal entra no ar na
TASK-E19-06, quando o repositório ficar público e o recurso for ligado. Até lá
as URLs de advisory acima retornam 404 para quem não é o mantenedor, e o botão
"Report a vulnerability" não aparece. Se você tiver algo a relatar antes disso,
abra uma issue pública mínima pedindo um canal privado, sem nenhum detalhe do
problema.

This is the only security channel. There is no security e-mail address, and
none should be invented. It is the channel decided as D-GOV-01 in
[`product/05-decisoes.md`](product/05-decisoes.md).

Este é o único canal de segurança. Não existe e-mail de segurança, e nenhum
deve ser inventado. É o canal decidido como D-GOV-01 em
[`product/05-decisoes.md`](product/05-decisoes.md).

### What to include / O que incluir

Version or commit, platform and architecture, the smallest licensed input that
reproduces it, the observed and expected result, and the impact you believe it
has. Do not attach copyrighted music, personal data or third-party secrets.

Versão ou commit, plataforma e arquitetura, a menor entrada licenciada que
reproduz o problema, o resultado observado e o esperado, e o impacto que você
atribui. Não anexe música protegida, dados pessoais nem segredos de terceiros.

### What to expect / O que esperar

This is a single-maintainer project with no paid support and **no committed
response SLA**. Reports are read as capacity allows, and the maintainer confirms
receipt when possible. No advisory is published without telling the reporter
first. A report that requires an unreleased component (most of the player today)
may stay open without a fix until that component exists.

Projeto de mantenedor único, sem suporte pago e **sem SLA de resposta
comprometido**. Os relatórios são lidos conforme a capacidade, e o mantenedor
confirma o recebimento quando possível. Nenhum advisory é publicado sem avisar
quem relatou antes. Um relatório que dependa de componente não lançado (a maior
parte do player hoje) pode ficar aberto sem correção até esse componente
existir.

## Supported versions / Versões suportadas

| Version | Supported |
|---|---|
| `main` | yes — once tagged releases exist / sim, quando existirem releases com tag |
| `develop` | best effort / melhor esforço, sem promessa |

No version has been released yet: there is no supported binary, and no security
fix is promised for a build that does not exist.

Nenhuma versão foi lançada: não há binário suportado e nenhuma correção de
segurança é prometida para um build que não existe.

## Scope and priorities / Escopo e prioridades

Priority components, in the order the product will exist: parsers and format
metadata, native codec helpers, packages and capabilities, local IPC, the
updater, web auth/ingest. Triage records version, minimal licensed input,
impact and mitigation; local reproductions carry no personal content. A
dependency advisory needs reachability analysis — neither ignored nor called an
exploit without proof.

Componentes prioritários, na ordem em que o produto existir: parsers e
metadados de formato, helpers nativos de codec, pacotes e capabilities, IPC
local, updater, autenticação/ingestão web. A triagem registra versão, entrada
mínima licenciada, impacto e mitigação; reproduções locais não carregam
conteúdo pessoal. Aviso de dependência exige análise de alcançabilidade — nem
ignorado, nem tratado como exploit sem prova.

## Out of scope / Fora de escopo

Reports that depend on a build the project does not ship; issues in third-party
codecs, platform SDKs or FFmpeg builds configured by whoever distributes them;
denial of service from a file the user chose to open, unless it also corrupts
data or escapes the sandbox the specification requires.

Relatórios que dependem de um build que o projeto não entrega; problemas em
codecs de terceiros, SDKs de plataforma ou builds de FFmpeg configurados por
quem distribui; negação de serviço a partir de um arquivo que o usuário escolheu
abrir, a menos que também corrompa dados ou escape do sandbox que a
especificação exige.

## See also / Ver também

- [Threat model / Modelo de ameaças](security/01-threat-model.md)
- [Privacy and licensing / Privacidade e licenciamento](security/02-privacidade-licencas.md)
- [Legal notes / Notas legais](LEGAL.md)
- [CI and gates / CI e gates](engineering/04-ci.md)

Nothing is declared "secure" merely because it is written in Rust.
Nada é declarado "seguro" só por ser escrito em Rust.
