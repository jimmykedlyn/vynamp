# Git por SSH no Windows, sem pedir senha

Passo a passo para o `git` falar com o GitHub por SSH sem pedir login nem passphrase a
cada operação. Feito nesta máquina em 2026-11-24. Vale para uma máquina Windows nova
ou para trocar uma chave cuja passphrase se perdeu.

O `gh` continua autenticado pelo keyring (`gh auth login`, ver
[08-reinicio-windows.md](08-reinicio-windows.md) §2); este guia é só do transporte do
`git`. O `.gitmodules` continua em HTTPS, porque o CI clona por ele: a troca para SSH é
só na configuração local de cada clone.

## 1. Diagnóstico

Antes de mexer, saber de onde vem o pedido de login:

```powershell
gh auth status
git config --show-origin --get-regexp 'credential|url\.'
git remote -v
Get-Service ssh-agent | Select-Object Status,StartType
```

- `credential.https://github.com.helper` apontando para `gh auth git-credential`, com
  uma linha `helper` vazia antes, quer dizer que o HTTPS já está resolvido pelo `gh`;
  o pedido não vem dele.
- `ssh-agent` com `StartType Disabled` quer dizer que a passphrase da chave é pedida a
  cada `git fetch`/`push`.
- `Host key verification failed` quer dizer que o `known_hosts` não tem o `github.com`.

## 2. Chave

Se a passphrase da chave antiga se perdeu, ela não tem conserto: arquivar e gerar outra.
A chave antiga não é apagada; vai para uma pasta de arquivo com o motivo:

```powershell
$arq = "$HOME\.ssh\arquivo-AAAA-MM-DD"
New-Item -ItemType Directory -Force $arq | Out-Null
Move-Item $HOME\.ssh\id_ed25519, $HOME\.ssh\id_ed25519.pub $arq
Set-Content "$arq\INDEX.md" 'id_ed25519: passphrase perdida; substituida em AAAA-MM-DD.'
```

Gerar a nova com o `ssh-keygen` do OpenSSH do Windows. A passphrase é digitada por quem
usa a máquina, nunca passada em argumento:

```powershell
C:\Windows\System32\OpenSSH\ssh-keygen.exe -t ed25519 -a 100 -C <usuario>@github-AAAA-MM-DD -f $HOME\.ssh\id_ed25519
```

## 3. Cadastrar no GitHub

O `gh` precisa do escopo `admin:public_key` (abre o navegador uma vez, com código de
dispositivo):

```powershell
gh auth refresh -h github.com -s admin:public_key
gh ssh-key add $HOME\.ssh\id_ed25519.pub --title "<maquina>-AAAA-MM-DD" --type authentication
gh ssh-key list
```

Sem o `gh`, colar o `.pub` em <https://github.com/settings/keys>.

Remover a chave antiga da conta só depois de confirmar que é ela (comparar o miolo do
`.pub` arquivado com a lista) e que nenhuma outra máquina a usa:

```powershell
gh ssh-key list
gh ssh-key delete <id> --yes
```

## 4. Host do GitHub no `known_hosts`

Conferir as chaves do servidor contra as que o GitHub publica na API antes de gravá-las
(evita aceitar às cegas o primeiro servidor que responder):

```sh
gh api meta --jq '.ssh_keys[]' | sort > /tmp/gh_meta.txt
ssh-keyscan -t ed25519,ecdsa,rsa github.com 2>/dev/null | grep -v '^#' | cut -d' ' -f2- | sort > /tmp/gh_scan.txt
diff /tmp/gh_meta.txt /tmp/gh_scan.txt && sed 's/^/github.com /' /tmp/gh_meta.txt >> ~/.ssh/known_hosts
```

Fazer backup do `known_hosts` antes. Se o `diff` mostrar diferença, parar: não gravar.

## 5. Agente

O `ssh-agent` do Windows guarda a chave já destravada e sobrevive a reinício. Ligar o
serviço exige PowerShell **como administrador**:

```powershell
Get-Service ssh-agent | Set-Service -StartupType Automatic -PassThru | Start-Service
```

Carregar a chave, num PowerShell normal (pede a passphrase uma única vez):

```powershell
C:\Windows\System32\OpenSSH\ssh-add.exe $HOME\.ssh\id_ed25519
C:\Windows\System32\OpenSSH\ssh-add.exe -l
```

O Git for Windows traz o próprio `ssh`, que **não** fala com esse agente. Apontar o `git`
para o OpenSSH do Windows:

```powershell
git config --global core.sshCommand C:/Windows/System32/OpenSSH/ssh.exe
```

Efeito colateral: vale para todo host SSH que o `git` usar, não só o GitHub.

## 6. Trocar os remotes para SSH

No umbrella, nos submódulos e na URL local de cada submódulo. O `.gitmodules` não muda:

```sh
cd /d/Projects/vynamp
git remote set-url origin git@github.com:jimmykedlyn/vynamp.git
git submodule foreach --quiet 'git remote set-url origin "$(git remote get-url origin | sed -E "s#^https://github.com/#git@github.com:#")"'
for k in $(git config --get-regexp '^submodule\..*\.url$' | awk '{print $1}'); do
  git config "$k" "$(git config "$k" | sed -E 's#^https://github.com/#git@github.com:#')"
done
```

As worktrees compartilham o `origin` do umbrella. Outros clones (fora de
`D:\Projects\vynamp`) precisam da mesma troca no próprio remote.

## 7. Conferir

```powershell
C:\Windows\System32\OpenSSH\ssh.exe -T git@github.com
git ls-remote origin HEAD
git -C libs/engine ls-remote origin HEAD
```

O primeiro responde `Hi <usuario>! You've successfully authenticated` e sai com código 1
(é o normal do GitHub). Os outros dois devolvem um SHA sem pedir nada.

## Problemas conhecidos

- **`Set-Service ... Acesso negado`**: o PowerShell não está como administrador.
- **Passphrase pedida a cada comando**: `core.sshCommand` não aponta para o OpenSSH do
  Windows, ou a chave não está no agente (`ssh-add -l` vazio).
- **`ssh-keygen` sobrescrevendo a chave**: se perguntar `Overwrite (y/n)?`, responder `n`
  e conferir qual chave está cadastrada antes.
- **WSL, terminal elevado ou outro usuário do Windows**: têm `~/.ssh`, agente e
  `.gitconfig` próprios; repetir o guia em cada um.
