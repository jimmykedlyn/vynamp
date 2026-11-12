# TEMPLATE — spec de módulo

> Copie este arquivo para `<repo>/specs/<modulo>.md` e preencha.
> Toda spec vive **no repositório do módulo dono**, nunca no umbrella.
> O umbrella só guarda `specs/architecture/` (visão entre módulos) e as ADRs.

---

# Spec — <nome do módulo>

| Campo | Valor |
|---|---|
| Módulo | `vynamp-<nome>` |
| Crate(s) | `<crate-folha>`, ... |
| Tipo | lib / app |
| Versão da spec | 0.1.0 |
| Data | AAAA-MM-DD |
| Estado | rascunho / aprovada / implementada |
| Dono | <papel funcional> |
| Depende de | `vynamp-dsp` (opcional), ... |
| É dependido por | `vynamp-engine`, ... |

## 1. Propósito

Uma frase: o que este módulo faz e o que ele **não** faz.
O "não faz" é normativo e vale tanto quanto o "faz".

## 2. Fronteira

| Permitido | Proibido |
|---|---|
| std, `vynamp-dsp` (se declarado) | qualquer crate `vynamp-*` de domínio |
| receber e devolver primitivos | expor/exigir tipos de domínio (`SampleRate`, `MediaTime`, ...) |
| ser usado direto por `cargo add` | exigir o workspace do umbrella para compilar |

Se este módulo for o `engine` (único que pode depender de outras libs), declare aqui
explicitamente a lista e justifique cada uma.

## 3. API pública

Só o que está aqui é contrato. Tudo o mais é interno e pode mudar sem aviso.

```rust
// Assinaturas reais, com tipos primitivos.
pub fn <funcao>(<param>: &[f32], <param>: u32) -> Result<Vec<f32>, Error>;
```

| Item | Garantia | Desde |
|---|---|---|
| `<funcao>` | ... | 0.1.0 |

## 4. Requisitos

Cada requisito tem ID estável e é rastreável até uma TASK e um teste.

| ID | Requisito | Prioridade | REQ de produto |
|---|---|---|---|
| `<MOD>-REQ-01` | ... | P0 | REQ-E01-01 |

## 5. Critérios de aceite

Formato obrigatório: **Given / When / Then**, um por requisito.
Se a condição não pode ser observada de fora, não é critério.

| ID | Given | When | Then |
|---|---|---|---|
| `<MOD>-AC-01` | ... | ... | ... |

## 6. Erros e cancelamento

| Variante | Quando ocorre | O que o chamador deve fazer |
|---|---|---|
| ... | ... | ... |

Erro é dado, não `panic!`. Cancelamento é explícito, não timeout silencioso.

## 7. Fora de escopo

Lista explícita do que este módulo **não** entrega nesta versão, para impedir
que a spec cresça sem decisão.

- ...

## 8. Dependências externas

| Crate | Versão | Licença | Por quê |
|---|---|---|---|

`unsafe` só com justificativa escrita; `cargo machete` limpo antes de cada release.

## 9. Verificação

| Item | Comando | Evidência |
|---|---|---|
| Teste unitário | `cargo test -p <crate>` | ... |
| Referência externa | ... | fixture versionado |
| CI | GitHub Actions Gate | link do run no PR |

Teste de referência que depende de arquivo grande vive atrás de `#[ignore]` com o
motivo escrito e variável de ambiente documentada — nunca some em silêncio.

## 10. Limitações conhecidas

O que a spec promete e ainda não está provado. Limitação declarada não reprova a
entrega; limitação omitida reprova.
