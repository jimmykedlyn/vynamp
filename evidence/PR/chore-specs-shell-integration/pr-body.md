# Pull request — Vynamp

## Resultado

Problema e comportamento final:

O Vynamp precisa de integração profunda e previsível com o shell de cada sistema operacional suportado, e de uma forma de identificar visualmente arquivos de áudio que reflita a identidade do produto. Hoje não existe especificação normativa para o menu de contexto (só menções a verbos `Open`/`Enqueue`), não há decisão sobre como o menu moderno do Windows 11 seria suportado, e não há catálogo de ícones de arquivo.

Este PR formaliza, **em nível de especificação e assets** (nenhuma integração implementada):

- **ADR-012** — arquitetura de integração por plataforma, estrutura do submenu "Vynamp", batching atômico, sistema de pacotes de ícones e identidade das ferramentas auxiliares.
- **`docs/platforms/03-system-integration.md` v0.8.0** — contrato operacional completo: verbos canônicos e seus comandos CLI, manifesto do pacote esparso, contratos Linux/macOS/Android e a seção "Integração com o Sistema" das Preferências.
- **`assets/icons/`** — sistema de pacotes de ícones de arquivo com 3 famílias visuais (Documento, Disco, Sinal) para 17 formatos de áudio, manifesto, catálogo interativo e vetores canônicos da marca e das ferramentas.
- **TASK-E16-03/04/05/08** e `tasks/backlog.json` — entregas, contratos e critérios de aceite alinhados à nova arquitetura.

Decisões-chave registradas:

| Tema | Decisão |
|---|---|
| Windows 11 (menu moderno) | Pacote esparso MSIX assinado (identidade de pacote, sem virtualização) + servidor COM `IExplorerCommand`/`IExplorerCommandEnumerator` |
| Windows 10 / "Mostrar mais opções" | Registro clássico HKCU (`Software\Classes\Vynamp.<ext>\shell`) com `MUIVerb` + `SubCommands` |
| Instalação | NSIS per-user em `$LOCALAPPDATA\Programs\Vynamp`; pacote esparso registrado via `PackageManager.AddPackageByUriAsync` |
| Lote | `IShellItemArray` entregue em uma única ativação — sem múltiplos processos do player |
| Linux | Desktop Actions (`.desktop`) + ServiceMenu KIO (Dolphin), extensão Nautilus, Custom Actions (Thunar/Nemo), hierarquia `hicolor` |
| macOS | Quick Actions via `NSServices` + `CFBundleDocumentTypes`; pacotes de ícones aplicam-se à UI interna (bundle assinado é imutável) |
| Android | Intent Filters `ACTION_VIEW` / `ACTION_SEND` / `ACTION_SEND_MULTIPLE` com SAF e `takePersistableUriPermission` quando permitido |
| Ferramentas | **Vynamp Restore** e **Vynamp Converter** como executáveis desktop separados (`vynamp-restore`, `vynamp-converter`), conforme decisão do mantenedor de 2026-11-03 |
| Ícones | 3 famílias × 17 formatos; `.ico` multi-resolução (16–256px) no Windows; alteração do shell no Windows/Linux; in-app no macOS/Android |

Verbos padronizados do submenu "Vynamp": Reproduzir (padrão), Abrir em nova lista, Adicionar à lista, Adicionar à fila, Editar etiquetas, Converter, Restaurar com Vynamp Restore — mapeados para `--play`, `--new-playlist`, `--add-playlist`, `--enqueue`, `--edit-tags`, `vynamp-converter` e `vynamp-restore`.

TASK / REQ / QA:

- Specs e ativos: `ADR-012`, `docs/platforms/03-system-integration.md`, `assets/icons/*`, `docs/GLOSSARY.md`, `docs/INDEX.md`, `CHANGELOG.md`, `VALIDATION.json`.
- Tarefas realinhadas: `TASK-E16-03` (Windows), `TASK-E16-04` (Linux), `TASK-E16-05` (macOS), `TASK-E16-08` (portable/backup/limpeza) — e `tasks/backlog.json`.
- REQ: `REQ-E16-03`, `REQ-E16-04`, `REQ-E16-05`, `REQ-E16-08`.
- QA referenciados (não executados aqui — continuam sob `TASK-E16-07`): `QA-022`, `QA-023`, `QA-024`, `QA-031`.

Plano Superpowers e evidência de execução:

Plano em `docs/superpowers/plans/2026-11-05-specs-shell-integration.md` (6 tarefas: ADR-012 → spec de plataforma → sistema de ícones → tarefas/backlog → índices/glossário/changelog → validação e revisão). Worktree e branch próprios: `repo/.worktrees/claude/specs-shell-integration` em `chore/claude-specs-shell-integration`, base `chore/claude-specs-mvp-reference` (37653d3).

## Verificação

Comandos, resultados, ambientes/fixtures e limitações:

Executados localmente (Windows 11, Rust 1.99.0):

- `cargo xtask check-docs` → `result: passed`, 0 erros, 329 arquivos Markdown, 182 tarefas, 182 requisitos, 68 entradas de catálogo.
- `cargo xtask check-delivery` → `result: passed`, 0 erros.
- `cargo test -p xtask` → 85 testes aprovados, 0 falhas.
- Hook de pre-push (mesmos checks do CI: `xtask ci docs` + 51 testes de `xtask` + 34 testes de `tests/tooling.rs`) → aprovado.

Revisão técnica (subagente Sonnet/Opus), commit revisado e achados:

- Revisão por subagente de contexto limpo (Opus, alto risco por definir arquitetura de integração) sobre ADR-012, spec de plataforma, catálogo de ícones, tarefas/backlog, índice e glossário → **APROVADO**, sem achados que exigissem correção.
- Commit revisado: `7ba0635`.
- Nenhuma credencial no diff.

Limitações declaradas:

- **Nada foi implementado nem testado em sistema operacional.** Este PR é exclusivamente de especificação e assets.
- **CI Gate (Woodpecker) não verificado.** A rede do agente não alcançou `api.github.com` com estabilidade ao longo do trabalho; conforme instrução do mantenedor, o CI/CD foi deixado para depois. O CI Gate precisa ser conferido pelo mantenedor no SHA final antes do merge.
- **macOS e hardware de saída de áudio não verificados** (sem agente Mac nem hardware de teste neste ambiente) — marcados como não verificados.

- [x] Worktree e owner próprios.
- [x] Superpowers carregado; teste útil e verificação executados.
- [x] Sucesso/erro/cancelamento e acessibilidade aplicáveis cobertos (n/a — entrega documental).
- [x] Docs/contracts/notices atualizados; nenhuma credencial no diff.
- [ ] CI Gate (Woodpecker) aprovado; macOS/hardware não disponível marcado como não verificado.

---

Aprovação e merge são do mantenedor (merge commit pelo GitHub).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
