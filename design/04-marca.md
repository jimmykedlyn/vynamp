# Vynamp — marca C / Pulso e ícones multiplataforma

Versão 0.7.0 · Nome Vynamp e conceito C aprovados pelo usuário.

## Identidade canônica

Vynamp, grafia com V maiúsculo e restante minúsculo. Identificadores técnicos usam vynamp. Símbolo: V largo e arredondado com triângulo play separado no espaço interno. Marca própria; nenhuma compatibilidade com Winamp ou reutilização de seu raio, código ou assets.

Marca principal: símbolo âmbar #FFBF69 e wordmark branco #F0F2F5 sobre grafite #11131B. Em fundo claro, wordmark grafite e símbolo âmbar. Variação monocromática usa uma única cor sólida com alpha, preservando os vazios internos. Sem gradientes, sombras, brilho ou metal no master de produção. Pranchas raster são referência visual; o master vetorial ainda deverá ser reconstruído e revisado.

[Prancha aprovada](../assets/branding/approved-c-board.png), [símbolo transparente](../_arquivo/repo/assets/branding/reference/symbol-c-transparent.png), [tokens](../assets/branding/brand-tokens.json), [inventário](../assets/branding/asset-manifest.json). Conceitos A/B/D ficam somente como histórico, sem instalação como marca alternativa.

## Sistema de logo

Exportar horizontal principal, horizontal fundo claro, empilhada, símbolo isolado e monocromáticas clara/escura. Entregar SVG de paths, PNG transparente e respectivos SHA256. Wordmark não pode depender de uma fonte instalada na máquina; converter lettering aprovado em paths e registrar origem/licença quando utilizar fonte externa. A geometria do símbolo e o gap do play são comuns em todos os exports. Não esticar, inclinar, trocar o V por triângulo isolado nem encaixar lettering em ícones pequenos.

Respiro mínimo: 20% da altura do símbolo ao redor do lockup; em tile usar as margens nativas/safe zone mais restritivas. UI: símbolo de marca a partir de 24 DIP quando acompanhado de texto; abaixo disso usar versão ótica simplificada aprovada e verificada. Para 16 px remover detalhes de arredondamento sem perder o triângulo/gap; não aplicar a mesma rasterização reduzida indiscriminadamente a todos os tamanhos. Favicon e bandeja têm inspeção própria. Se o play/gap desaparecer, revisar desenho, não aprovar só porque exportou.

## Marca e interface

A identidade de marca não substitui os tokens semânticos do Classic Graphite. Verde-lima #B8F35A continua playback/tempo; ciano #66D9EF continua foco/ferramentas; âmbar também continua VU/edição. Preservar layouts clássico/biblioteca/mini/mobile e geometria definida. Splash/About/onboarding/instaladores usam Vynamp e C. Headers exibem o nome Vynamp; ícone da marca não deve se tornar um botão play sem semântica explícita.

O branco de marca #F0F2F5 não altera automaticamente text-primary da UI #F1F3F7. Logotipo decorativo é diferente de texto/botão funcional. Não usar âmbar sobre branco para controles essenciais sem contraste; escolher monocromático grafite. Aplicar critérios de contraste e acessibilidade ao uso real. Imagem com texto alternativo Vynamp quando identifica o app; símbolo repetido ao lado do nome é decorativo. Não depender só da cor.

## Exports por plataforma

| Destino | Entrega | Regras |
|---|---|---|
| Windows x64 | packaging/icons/vynamp.ico com 16/20/24/32/40/48/64/128/256 px | RGBA/transparência, sem tile forçado; versão pequena ótica. Embutir no PE/NSIS/atalhos, validar 100/125/150/200% DPI. |
| Linux x64 | packaging/icons/vynamp.svg e PNG 16/24/32/48/64/128/256/512/1024 px | SVG sanitizado de paths; desktop Icon=vynamp; hicolor por tamanho/scalable; AppImage e DEB usam mesma marca. |
| macOS Intel | packaging/icons/Vynamp.icns, iconset intermediário | Ícone com margens/tile da plataforma, 16/32/128/256/512 pt em 1x/2x; incluir 64 e 1024 px. Resources/Vynamp.icns; CFBundleIconFile=Vynamp. |
| macOS Apple Silicon | Exatamente o mesmo Vynamp.icns | ABI muda binário, não marca/arquivo; Universal2 tem uma única cópia do ícone. |
| Android arm64 | Adaptive background/foreground, monochrome e fallback mipmaps | Layers independentes; não rasterizar círculo/squircle como foreground. Fundo grafite completo; V/play âmbar dentro de safe zone. |
| Web futura / docs | favicon SVG + ICO 16/32/48; PNG 192/512 | Favicon simples; manifest normal/maskable separados, maskable com safe zone de sua especificação; sem publicar plataforma web antes M4. |
| Tray / menu bar / notificação | Glyphs claro/escuro ou template nativo | Alpha/monocromático sem wordmark e sem background tile; estados pausa/erro usam badges/semântica, não outra marca. |

ICNS: icon_16x16, 16@2x, 32, 32@2x, 128, 128@2x, 256, 256@2x, 512, 512@2x. Apple usa o mesmo desenho para ambas arquiteturas. Menu bar macOS usa template com alpha, inspeção 18 pt/36 px. Windows/Linux tray: 16/20/24/32 px conforme shell/DPI real. Não reutilizar o ícone grande reduzido se perder legibilidade.

## Android

Canvas adaptive 108×108 dp; foreground transparente e background opaco separados. Todos os elementos essenciais do V/play devem ficar dentro do círculo central de 66 dp de diâmetro, centro 54/54; validar máscaras circle, squircle, rounded square e recortes OEM. Margin extra para animação/parallax; sem pré-máscara nem sombra embutida. Monochrome reproduz a mesma geometria com uma única máscara alpha, cor escolhida pelo sistema.

Recursos planejados: drawable/vynamp_foreground.xml, drawable/vynamp_monochrome.xml, drawable/vynamp_notification.xml, values/colors.xml com vynamp_background; mipmap-anydpi-v26/ic_launcher.xml e ic_launcher_round.xml; v33 inclui monochrome. Fallback launcher PNG mdpi48/hdpi72/xhdpi96/xxhdpi144/xxxhdpi192 px. Manifest usa @mipmap/ic_launcher e @mipmap/ic_launcher_round. Notificação foreground service: símbolo branco/alpha 24 dp próprio, sem o tile colorido. Ícone Google Play 512×512 opaco, export independente; recursos de loja não substituem recursos runtime.

## Naming e integração

Desktop package vynamp-desktop, bin vynamp; bundles/appId com.jimmykedlyn.vynamp, WIT vynamp:plugin; temas .vyntheme, plugins .vynplug, diagnóstico .vyndiag. Pasta distribuída vynamp-mediaplayer. GitHub jimmykedlyn/vynamp-mediaplayer, renomeado segundo usuário; acesso/proteções ainda não verificados neste ambiente.

Git claims locais mantêm namespace histórico legado (`auralis/claims`) para exclusividade entre sessões de tooling 0.4 e 0.5. É metadado privado do Git, não nome de produto nem compatibilidade com Winamp. Não mover/remover claims ativos para mudar a marca. Não supor dados de app lançado: ainda não existe app; futuros paths de configuração/installer usam Vynamp/vynamp desde a primeira versão.

## Gates, tarefas e evidência

TASK-E02-02 incorpora marca nas shells/UI; TASK-E06-01 governa tokens/temas; TASK-E15-06/07/08/09 integram ícones nos instaladores Windows/Linux/macOS/Android; TASK-E14-06 valida o conjunto multiplataforma. Entregas nativas permanecem planejadas; aprovar referência PNG não aprova app, vetor, instalador ou comportamento em sistema operacional.

Manifest de assets exige caminho relativo, hash SHA256, dimensões, status reference/pending/exported/verified e plataforma. Branding exports gate só passa após conferir todos os arquivos nativos e hashes, ambos agentes revisarem e inspeção real por tamanho/tema/máscara. Em config/project.json branding_exports_ready=false por enquanto; preflight recusa flags alteradas sem arquivos/manifests íntegros. application_ready/release_enabled continuam false.

QA: transparência/alpha; pixels isolados/fringing; desenho e gap em16/20/24/32; ausência de dependências externas emSVG; ícones cortados/máscaras; contraste claro/escuro/high contrast; DPI; launchers/shortcuts/taskbar/menu bar/tray; About/splash nome correto; Android themed/notification; Intel/Silicon arte idêntica. Revisão humana além de checks de arquivo/hash. Brand escolhida não equivale a registro de marca/domínio.

## Contrato do receipt de exportação

[Exemplo pendente](../packaging/icons/export-manifest.example.json) define campos; não é evidência de execução. Após exports/QA, criar packaging/icons/export-manifest.json com conceito C, source_digest de cargo xtask release digest, reviewers claude, status verified por plataforma/report/hash e files path/sha256. Atualizar status do asset-manifest a verified somente com essa evidência. O receipt é excluído do digest para evitar auto-referência; native files/tokens/assets são incluídos.

Gates atuais exigem ICO e ICNS modernos com frames PNG decodificáveis nos tamanhos nativos, SVG restrito a geometria/path sem refs externas e PNGs Linux/Android com dimensões explícitas. ICO/ICNS só com prefixo/header ou arquivos com extensão falsa são recusados. Codificação ICNS PNG-backed deve ser produzida/verificada no spike macOS; outputs RLE/JP2 antigos exigem decoder adicional e regressões antes aceitação. Preview PNG ainda não constitui export.
