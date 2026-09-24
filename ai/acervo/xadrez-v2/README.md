# Xadrez: serviço e interceptador

Edição 2, preservando a edição 1. As duas fontes pertencem à revisão
`18fdab8329b2f680c731a1929132caed67e33587` do xadrez-angular.
O serviço reutiliza o snapshot em `../xadrez-v1`; o interceptador foi copiado
byte a byte de `git show` da mesma revisão. `catalog.json` registra procedência,
hashes e seleção. Execute os comandos pela raiz do renderer.

## Leitura documentada

| Serviço | Condição no interceptador | Fatos |
| --- | --- | --- |
| GET `/api/init` | GET e URL terminando em `/api/init` | F_INIT, F_HANDLE_INIT |
| GET `/api/row/` + posição | GET e regex `/\/api\/row\/\w+$/` | F_POSITION, F_HANDLE_POSITION |
| PUT `/api/rows/jogada` | PUT e URL terminando em `/api/rows/jogada` | F_PLAY, F_HANDLE_PLAY |
| PUT `/api/rows/mover/` + posição | PUT e regex `/\/api\/rows\/mover\/\w+$/` | F_MOVE, F_HANDLE_MOVE |

Os pares acima são uma leitura manual, não relações inferidas pelo Graph Engine.
As strings de posição aceitas pelo serviço não têm restrição demonstrada nessa
fonte; as condições do interceptador usam `\w+`. Portanto, não concluímos
correspondência para qualquer string arbitrária.

Outros fatos selecionados mostram as chamadas `criarJogada`, `atualizaPosicoes`
e o encaminhamento final `next.handle`. O contexto é uma janela textual de duas
linhas, não o corpo completo dos ramos. O dossiê preserva as fontes completas.

Não conferimos registro do interceptor na aplicação, execução HTTP, interface,
segurança ou correção completa das regras de xadrez. O texto “Xeque mate” no
código, por exemplo, não comprova implementação correta dessa regra.

## Reproduzir

Esta é uma referência histórica de cadastro manual, fora da trilha atual do
[TESTME](../../../TESTME.md). Para reproduzir esta exportação, na raiz do renderer,
use um destino novo:

```sh
cargo run --locked --bin validate_evidence -- ai/acervo/xadrez-v2/dossier.json --fact F_INIT --fact F_HANDLE_INIT --fact F_POSITION --fact F_HANDLE_POSITION --fact F_PLAY --fact F_HANDLE_PLAY --fact F_MOVE --fact F_HANDLE_MOVE --fact F_PLAY_ACTION --fact F_MOVE_ACTION --fact F_FALLBACK --context 2 --question ai/acervo/xadrez-v2/question.txt --bundle ./xadrez-duas-fontes-bundle
```

A seleção contém 11 fatos em duas fontes. Os IDs ficam na ordem solicitada;
os contextos permanecem separados por fonte, unindo apenas janelas vizinhas
da mesma fonte. Nenhum banco ou LLM é necessário.

Uma futura avaliação LLM deve definir previamente critérios para verbos/rotas,
distinção entre sufixo e regex, citações de ambas as fontes e ausência de
alegações de execução. Não foi realizada avaliação semântica de LLM nesta etapa.
