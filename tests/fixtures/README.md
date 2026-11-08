# Paths e contrato de fixtures

Estado em 2026-10-18: paths definidos pela TASK-E11-01; nenhum corpus de áudio, rede, UI ou hardware entregue aqui.

Todos os paths abaixo são relativos à raiz do repositório, independentes da pasta de checkout/worktree e do sistema operacional. Pastas de corpus são criadas junto das primeiras fixtures reais; esta tabela reserva seus destinos, sem arquivos vazios que simulem testes.

| Path canônico | Conteúdo / owner futuro |
|---|---|
| `tests/fixtures/audio/` | PCM, arquivos codificados e metadata; tarefas de engine/codecs e QA numerical |
| `tests/fixtures/network/` | Respostas HTTP/ICY/HLS e cenários adversariais, sem credenciais/serviços externos |
| `tests/fixtures/library/` | Bases de teste/migrations/backup/recovery sem dados pessoais; inclui QA-012 do player quando implementado |
| `tests/fixtures/packages/` | Pacotes próprios de temas/plugins e entradas adversariais |
| `tests/fixtures/ui/` | Referências de UI próprias com host/DPI/tema identificados |
| `tests/fixtures/manifests/` | Inventários de cada corpus, adicionados com seus arquivos reais |

## Registro obrigatório antes de incluir corpus

Cada inventário deve identificar ID estável, path relativo à raiz do repo, tamanho, SHA-256 dos bytes, origem/autor, licença ou permissão de redistribuição e transformação/gerador com versão e comando reproduzível. Incluir formato/perfil/parâmetros pertinentes, TASK/REQ/QA consumidor e resultado esperado com tolerância e fonte independente do oráculo. Artefatos binários usam bytes estáveis; a tarefa que adicionar o corpus deve definir atributos Git adequados e verificar os hashes em seus testes/CI.

Não incluir corpus sem permissão, dados pessoais, tokens, código/assets Winamp ou material proprietário não autorizado. Fixtures sintéticas identificam seu gerador e limites; não viram prova de compatibilidade universal. O formato de inventário/schema/validador é entregue junto do primeiro corpus real; não há manifesto fictício vazio marcado como aprovado nesta tarefa.

## Fixtures temporárias do tooling

`xtask/tests/tooling.rs` usa `tempfile` e repositórios Git reais em caminhos com espaços. No Windows preparado, `AMBIENTE.ps1` direciona TMP/TEMP para `D:\Projects\vynamp-media-player\.cache\tmp`; cada teste recebe diretório único e remove o contexto Git herdado nos subprocessos. Esses repositórios gerados não são corpus versionado e não dependem de nomes temporários fixos.

Builds usam `<worktree>/target/xtask-driver` e `<worktree>/target/xtask-ci`; scratch em `.superpowers` é ignored. Resultados permanentes ficam em `evidence/PR/<task>/`, com comando/ambiente/SHA/hashes reais. Capturas de hardware incluem device/driver/ABI; nenhum path ou teste de tooling substitui execução do player em hardware.

Referências: [estratégia QA](../../quality/01-estrategia-testes.md), [contrato de testes](../../engineering/05-testes.md), [ADR-009](../../adr/ADR-009-baseline.md).
