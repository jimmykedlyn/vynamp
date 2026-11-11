# TEMPLATE — ADR

> Arquivo: `<repo>/specs/adr/ADR-<NNN>-<slug>.md` no repo afetado, ou no umbrella
> quando a decisão atravessa módulos.
> Numeração é **global e única** no umbrella; repos de módulo referenciam a ADR do
> umbrella em vez de criar homônima.

---

# ADR-<NNN> — <decisão em uma frase imperativa>

| Campo | Valor |
|---|---|
| Status | proposta / **aceita** / substituída por ADR-NNN / obsoleta |
| Data | AAAA-MM-DD |
| Decisor | mantenedor |
| Escopo | umbrella / `vynamp-<nome>` |
| Substitui | ADR-NNN (ou "—") |
| É substituída por | ADR-NNN (ou "—") |
| Afeta | specs, TASKs, código |

## Contexto

O que obriga uma decisão agora. Fatos, não opiniões. Se a decisão existe porque
alguém preferiu assim, escreva isso — preferência declarada é contexto válido,
preferência escondida não.

## Decisão

Uma frase no imperativo, seguida do detalhamento necessário para executar.

## Alternativas consideradas

| Alternativa | Por que foi descartada |
|---|---|

Se não houve alternativa real, diga "não havia alternativa viável porque ...".

## Consequências

**Boas.** O que fica mais simples, mais seguro, mais barato.

**Ruins.** O que fica mais difícil. Toda decisão estrutural tem custo; uma ADR que
só lista benefícios está incompleta e será reaberta.

**Irreversível?** Se reverter exigir migração, perda de dados ou reescrita de
histórico, escreva aqui e diga o que é preciso para reverter.

## Execução

Quais TASKs executam esta decisão. Sem isso a ADR é declaração, não norma.

| TASK | O que faz |
|---|---|

## Verificação

Como se confirma, depois, que a decisão foi realmente aplicada. Uma ADR aceita e
não executada é dívida documentada, não decisão.

---

## Regras de ciclo de vida

- ADR **nunca** é editada para mudar a decisão. Muda-se o Status e cria-se a nova.
- Ao substituir, coloque o banner no topo do arquivo antigo:
  > ⚠️ **Substituída por [ADR-NNN](ADR-NNN-slug.md)** — o corpo abaixo é histórico.
- ADR aceita sem TASK de execução não entra no fluxo; fica como `proposta`.
- Só o mantenedor muda `proposta` → `aceita`.
