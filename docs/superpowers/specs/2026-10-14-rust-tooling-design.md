# Automação Rust — desenho aprovado em conversa

O usuário aprovou migrar a automação proposta para Rust com cargo xtask. Este desenho registra a decisão existente, sem alterar o produto, as plataformas ou as autorizações de publicação.

## Contrato

Remover dependência de Python do pacote, hooks e workflows. Um workspace contendo apenas xtask não significa player implementado. Rust instalado via rustup, versão real resolvida e pinada, Cargo.lock real. Git, gh, agentes e ferramentas nativas de assinatura continuam externos.

## Componentes

xtask contém CLI tipada, operações Git com claims atômicos, descoberta de plugins, importação GitHub privada, validação documental/JSON Schema/XML/YAML, CI, gates de hardware, instaladores/assinatura e releases. Usar crates para parsing e criptografia; subprocessos recebem argumentos separados, sem interpolação de shell. Segredos ficam em arquivos temporários privados e comandos sensíveis têm saída suprimida.

## Segurança e compatibilidade

Preservar exclusividade de tarefa, recusa de worktrees sujas/não integradas e hotfix integrado em main e develop. Não usar force push. Publicação somente draft após evidência real de cinco plataformas, assinatura e versão. No Windows usar executáveis nativos; Android executa o wrapper adequado ao host. Android mantém Kotlin/Compose para integração nativa; Rust hospeda o núcleo e automação de desenvolvimento.

## Aceite

cargo fmt/clippy/test e cargo xtask ci docs passam. Testes usam Git real temporário, verificam gates negativos, digest, schemas, detecção de código e plugins sem instalação falsa. ZIP contém configurações ocultas, Cargo.lock e nenhum cache/credencial. Instalação de agentes, CI remoto, assinatura e player continuam não verificados quando indisponíveis.
