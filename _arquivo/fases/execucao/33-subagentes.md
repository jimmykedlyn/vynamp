# Passo 33 — subagentes

Define e instala as **definições de subagente** do Claude Code: quem orquestra, quem
implementa, quem checa, quem revisa. Sem isso, o fluxo do `AGENTS.md` §5 é intenção,
não execução.

---

## 1. O que existe hoje

Verificado na FASE 1: **nada**. Não há `.claude/agents/` em lugar nenhum, não há hooks
versionados, e `docs/engineering/01-agents-superpowers.md` descreve o fluxo em prosa
sem nenhuma definição executável. O `AGENTS.md` prometia "Opus implementa, Sonnet
revisa" e nenhuma dessas palavras virava configuração.

## 2. O que este passo cria

`.claude/agents/` na raiz do umbrella, com 4 definições:

| Arquivo | Modelo | Papel | Ferramentas |
|---|---|---|---|
| `orquestrador.md` | Opus | lê spec, escreve plano, decompõe em TASKs, decide modelo por tarefa | read-only + fragmentos de escrita no plano |
| `implementador.md` | Sonnet | TDD, escreve código e teste | tudo |
| `checador.md` | Haiku | roda fmt/clippy/test e devolve só passou/falhou | bash + read |
| `revisor.md` | Opus ou Sonnet | revisa em contexto limpo, nunca foi o implementador | read-only |

Alto risco — áudio/tempo real, concorrência, segurança, release — troca
`implementador` e `revisor` para Opus.

## 3. Regras que entram nas definições

1. **O revisor nunca é o implementador.** É a regra mais importante e a mais fácil de
   violar por conveniência. Define-se `model:` explicitamente e proíbe-se reuso de
   sessão.
2. **O checador devolve resumo, não dump.** "3 comandos, 2 verdes, 1 vermelho:
   `cargo clippy` falhou em `x.rs:42`" — não 400 linhas de log.
3. **O implementador não faz merge, tag nem push.** Herda as proibições do
   `AGENTS.md` §6, e a definição as repete porque subagente não lê o contrato por
   osmose.
4. **O orquestrador não implementa.** Se ele editar código, perde o ponto de vista
   de revisão.

## 4. Formato do arquivo

```markdown
---
name: implementador
description: Implementa uma TASK com TDD, numa worktree própria. Use quando a spec
  da TASK já está aprovada e o plano escrito.
model: sonnet
tools: Read, Write, Edit, Bash, Grep, Glob
---

Você implementa uma TASK do Vynamp. Leia `AGENTS.md` antes de qualquer coisa.

Fluxo obrigatório:
1. Ler a TASK e os critérios de aceite em Given/When/Then.
2. Escrever o teste, rodar, confirmar que falha.
3. Implementação mínima até o teste passar.
4. `cargo fmt` + `cargo clippy -D warnings` + `cargo test`.
5. Repetir por critério.

Proibido: `git push`, merge, tag, `--no-verify`, abrir PR.
Devolva: arquivos tocados, testes criados, comandos rodados e o resultado de cada um.
```

## 5. O que **não** faz

- Não instala hook nenhum. Hooks são decisão do mantenedor e não existem hoje.
- Não altera `~/.claude/` — só o `.claude/` do projeto.
- Não muda modelo da sessão principal.

## 6. Verificação

```bash
ls D:\Projects\vynamp\.claude\agents\
# checador.md  implementador.md  orquestrador.md  revisor.md

head -8 D:\Projects\vynamp\.claude\agents\revisor.md
# frontmatter com name, description, model, tools
```

Resultado esperado: 4 arquivos, cada um com frontmatter válido e `model:` explícito.

## 7. Commit

```
feat(agentes): define os 4 subagentes do fluxo SDD (passo 33)

- orquestrador (Opus), implementador (Sonnet), checador (Haiku), revisor (contexto limpo)
- revisor nunca e o implementador; alto risco sobe para Opus
- checador devolve resumo, nao dump de log
- proibicoes do AGENTS.md secao 6 repetidas em cada definicao
```

## 8. Risco

| Risco | Mitigação |
|---|---|
| `model:` com id inválido e a definição não carrega | o script valida o frontmatter e avisa |
| Custo: Opus em tarefas simples | a tabela de risco por épico define o modelo; nada de Opus por padrão |
| Subagente ignorando o contrato | as proibições são repetidas na própria definição, não só referenciadas |
