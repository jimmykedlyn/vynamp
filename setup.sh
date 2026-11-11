#!/bin/sh
# Liga os hooks locais deste clone e confirma que eles podem executar.
#
# `core.hooksPath` mora em `.git/config`, que nao e versionado: os hooks estao no
# repositorio, mas o git nao olha para eles ate alguem apontar. Este script faz
# essa ligacao e e idempotente -- rodar de novo nao custa nada.
#
# Rode depois de clonar, e de novo depois de `git submodule update --init` se os
# modulos reclamarem de hook ausente: cada repositorio de modulo tem o proprio
# `setup.sh`, e este aqui **nao** alcanca os submodulos.
#
# Uso: sh setup.sh
set -eu

root=$(git rev-parse --show-toplevel)
cd "$root"

git config core.hooksPath .githooks

# O bit de execucao viaja no indice, nao no conteudo: um clone feito num sistema
# que nao o preserva (ou um arquivo comitado como 100644) deixa o hook mudo, e o
# git pula hook sem permissao de execucao sem dizer nada. Reafirmar aqui e barato
# e transforma um silencio em erro visivel.
git update-index --add --chmod=+x -- .githooks/pre-commit .githooks/pre-push 2>/dev/null || true

# CRLF quebra o shebang: o kernel procura "/bin/sh\r" e nao acha. O
# `.gitattributes` ja forca `eol=lf` nestes caminhos, mas um clone anterior a
# essa linha pode ter ficado com CRLF em disco.
for hook in pre-commit pre-push; do
    if [ ! -f ".githooks/$hook" ]; then
        echo "aviso: .githooks/$hook nao existe neste clone." >&2
        continue
    fi
    # Detectar CR aqui e mais barato que o grep parece: neste ambiente o
    # `grep`/`awk` tratam o CR como terminador de linha e respondem que nao ha
    # nada, entao a checagem compara tamanho com e sem os CRs.
    if [ "$(tr -d '\r' < ".githooks/$hook" | wc -c)" != "$(wc -c < ".githooks/$hook")" ]; then
        echo "aviso: .githooks/$hook tem CRLF em disco; o shebang nao vai executar." >&2
        echo "       rode: git rm --cached .githooks/$hook && git checkout -- .githooks/$hook" >&2
    fi
    if [ ! -x ".githooks/$hook" ]; then
        echo "aviso: .githooks/$hook nao tem bit de execucao neste sistema de arquivos." >&2
        echo "       em Linux/macOS o git vai ignora-lo em silencio." >&2
    fi
done

echo "core.hooksPath = $(git config --get core.hooksPath)"
echo "Hooks ativos: pre-commit (guard-branch) e pre-push (cargo xtask ci docs)."
echo "Commit em main/develop ou em branch fora do Gitflow sera bloqueado."
