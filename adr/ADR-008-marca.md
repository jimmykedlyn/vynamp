# ADR-008 — Vynamp e identidade C / Pulso

Versão 0.7.0 · Decisão do usuário aceita; exports nativos pendentes.

## Decisão

Nome do produto Vynamp. Conceito C — Pulso (V com play), âmbar/grafite/branco. Mesma geometria em Windows/Linux/macOS Intel/macOS Apple Silicon/Android; composição nativa por plataforma. Paleta semântica e layout Classic Graphite preservados. [Guia normativo](../design/04-marca.md).

## Integração

Repositório informado renomeado para jimmykedlyn/vynamp-mediaplayer. Atualizar configuração/links/identificadores/templates e root ZIP para vynamp-mediaplayer. Sem alterar origin remotamente ou executar push. Schemas/WIT/pacotes próprios anteriores não foram lançados; usar namespace vynamp e extensões .vyntheme/.vynplug/.vyndiag. IDs TASK/REQ/QA existentes permanecem estáveis.

Claims Git mantêm legado (`auralis/claims`) por segurança entre sessões de tooling antigas/novas; nunca duplicar owner por mudar a marca. Logs/evidências anteriores preservados como históricos e não contam como execução da versão atual.

## Evidência e consequências

PNG gerado original incluído com SHA e dimensões; prancha/reference aprovadas não são export SVG/ICO/ICNS/Android compiled. Reconstituição vetorial/wordmark/licenças/exportinspeção integram tarefas UI/temas/instaladores existentes. branding_exports_ready=false, app/release flags false até gates reais. Não alegar exclusividade jurídica ou execução em todas plataformas a partir da prancha.
