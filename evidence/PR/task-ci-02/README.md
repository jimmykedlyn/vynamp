# Evidência TASK-CI-02 — job `check-tasks` no CI do umbrella

Versão 0.1.0 · 2026-11-09 · Autor Claude. Verificação local; o job foi adicionado ao
`ci.yml` e passa a valer no CI Gate a partir do merge.

## O que existe agora

| Peça | Onde |
|---|---|
| `cargo xtask check-tasks` | `xtask/src/main.rs` → `validate::tasks(&root)` |
| Regra de backlog × Markdown | `task_disagreements()` em `xtask/src/validate.rs` |
| Job de CI | `.github/workflows/ci.yml`, job `tasks`, exigido pelo `gate` |
| Fixtures e testes | `mod tests` de `validate.rs`, 7 testes `check_tasks_*`/`acceptance_rows_*` |

O job `tasks` clona com `submodules: recursive` porque o backlog aponta a maioria das
TASKs para o repositório dono: sem os submódulos, o gate leria uma árvore vazia de
tasks e aprovaria por ausência.

## O que o gate reprova

Medido, com `cargo xtask check-tasks`:

```
$ cargo xtask check-tasks
{"errors":[],"result":"passed","scope":"Backlog and TASK Markdown only; no build, no hardware","tasks":193}
```

Antes das autorias deste passo o mesmo comando devolvia erro para cada TASK fora de
`planned` sem critério escrito — seis, e nomeadas por ID e coluna.

As regras, em ordem:

1. Toda TASK de produto tem `requirements` não vazio, começando por `REQ-`.
   `TASK-CI-*` é isenta: não cobre requisito de produto.
2. `acceptance_ready` é booleano.
3. `status != planned` com `acceptance_ready != true` reprova.
4. Exatamente um módulo dono, e ele tem de existir no backlog. `PENDENTE` conta como
   um nome — e por isso nome desconhecido é checado contra o conjunto real de módulos.
5. Dono no Markdown ≠ dono no backlog reprova.
6. Fora de `planned`, toda célula vazia ou `PENDENTE` reprova, nomeando TASK e coluna.
   Em `planned`, só a coluna `Then` é exigida: `Given`/`When` são autoria pendente.

## `AC-4` × `AC-5` — a contradição e como foi resolvida

`AC-4` manda o vermelho nomear **exatamente as cinco** `in_progress`; `AC-5` manda
ficar verde quando essas cinco estiverem escritas. Ambas não podem valer no mesmo
commit: no estado do passo 37 havia **seis** TASKs fora de `planned` sem critério
escrito — as cinco mais `TASK-E19-06`, que foi fechada executando-a naquele passo.

A regra aplicada é a que as duas descrevem de fato: reprovar toda TASK fora de
`planned` sem critério escrito. No estado do 37 ela nomeia as cinco **e** a sexta;
depois das autorias, fica verde. Nenhuma exceção foi introduzida no caminho, que é
o ponto do passo: "verde por exceção é o defeito que este passo corrige"
(`37-tasks.md` §8).

## Autorias feitas para chegar ao verde

| TASK | Repo | ACs autorados |
|---|---|---|
| `TASK-E01-02` | `libs/output` | 3 |
| `TASK-E02-11` | `apps/player` | 5 |
| `TASK-E11-01` | `vynamp` | 4 |
| `TASK-E15-03` | `vynamp` | 4 |
| `TASK-E19-01` | `vynamp` | 5 |
| `TASK-E19-02` | `vynamp` | 4 |
| `TASK-E19-06` | `vynamp` | 4 |

Em `TASK-E19-06` as células `Then` ficaram **intactas** mesmo dizendo "os sete repos"
onde hoje são dez: a coluna `Then` é o requisito aprovado, copiado literal do
backlog, e reescrevê-la é decisão do mantenedor, não de engenharia. A discrepância
está registrada nas notas de execução da TASK.
