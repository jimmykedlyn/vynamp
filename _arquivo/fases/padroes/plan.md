# TEMPLATE — plano de implementação

> Copie para `<repo>/specs/plans/<AAAA-MM-DD>-<slug>.md`.
> Plano é derivado da spec. Se o plano precisa de algo que a spec não diz, a spec muda
> primeiro — nunca o contrário.

---

# Plano — <o que será construído>

| Campo | Valor |
|---|---|
| Spec de origem | `specs/<modulo>.md` §X |
| TASK(s) coberta(s) | `TASK-E01-01` |
| Modelo executor | Opus (alto risco) / Sonnet (padrão) |
| Revisor | contexto limpo, nunca o executor |
| Branch | `feature/task-<id>-<slug>` |

## 1. Estado atual

O que existe hoje, com caminho e linha. Se não existe nada, diga "nada".

## 2. Decisões técnicas

Só o que a spec deixa em aberto. Cada decisão nova e reversível fica aqui; cada
decisão estrutural vira ADR.

| # | Decisão | Alternativa descartada | Por quê |
|---|---|---|---|

## 3. Passos

Cada passo é executável e verificável isoladamente. Nenhum passo tem "e depois
faz o resto".

1. `cargo xtask repo worktree task-<id>` → worktree em `repo/.worktrees/claude/task-<id>-<slug>`
2. **TDD — vermelho**: escrever o teste do critério `<MOD>-AC-01`, rodar, confirmar que falha.
3. **Verde**: implementação mínima que faz o teste passar.
4. **Refatorar**: com a suíte verde, limpar. `cargo fmt` + `cargo clippy -D warnings`.
5. Repetir 2–4 por critério de aceite.
6. Evidência em `docs/evidence/PR/<task>/`.

## 4. Arquivos tocados

| Arquivo | Ação | Motivo |
|---|---|---|

Nenhum arquivo fora desta lista é tocado sem atualizar o plano.

## 5. Verificação

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo machete
```

Resultado esperado de cada um, escrito **antes** de rodar. Se o resultado divergir
do esperado, é achado, não ruído.

## 6. Riscos

| Risco | Probabilidade | Impacto | Mitigação |
|---|---|---|---|

## 7. O que quebra se der errado

Efeitos colaterais, migração, dados, usuários. Se a resposta for "nada", escreva
"nada" — mas pense antes.
