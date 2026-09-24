# Busca lexical e grafo estrutural — primeira entrega

O algoritmo e o grafo são implementados em `librarian-ingest`. O renderer fornece
o [vocabulário](../renderer-lexicon.json), as [perguntas de referência](questions.json)
e um comando consumidor. Não é necessário banco, Ollama ou DeepSeek.

## Consultar

Na raiz do renderer, com o checkout atualizado do Librarian em `../librarian`:

```powershell
cargo run --locked --bin catalog_sources -- search ai/acervo/renderer-auto-v1/edition "Onde a câmera calcula a direção dos raios?"
```

O comando usa o vocabulário do renderer, cinco resultados e profundidade 1.
Para explicitar vocabulário, número de resultados e profundidade:

```powershell
cargo run --locked --bin catalog_sources -- search ai/acervo/renderer-auto-v1/edition "Onde a câmera calcula a direção dos raios?" ai/acervo/renderer-lexicon.json 5 2
```

Esperado: primeiro resultado `impl Camera::ray_from_pixel`, `src/camera.rs:75–89`.
O JSON preservado em [camera-query.json](camera-query.json) registra essa segunda
consulta: pergunta, termos, hashes do catálogo/vocabulário, pontuação, motivos,
trechos e vizinhança do primeiro resultado. É uma seleção de candidatos, não uma
resposta explicativa nem o bundle `prepare_query`.

## Como o léxico funciona

Normaliza maiúsculas e acentos portugueses; separa snake_case, camelCase e siglas.
Remove palavras de ligação e aplica grupos explícitos de equivalência, incluindo
plurais e traduções (`raios/raio/ray/rays`, `direção/direction`). Não usa um modelo
ou lematizador geral: novas flexões e termos precisam de vocabulário explícito.

Cada grupo da pergunta conta uma vez. Um candidato precisa cobrir pelo menos 60%
dos grupos úteis. A pontuação soma 100 por grupo encontrado e o maior peso do campo
por grupo: nome 8, escopo 3, caminho 1, código 1. Testes marcados `#[test]` recebem
penalidade 20; desempates priorizam não testes, trechos menores e ID. Pontuação não
é probabilidade. `reasons` mostra termos expandidos e campos correspondentes.

Módulos e blocos impl não competem na busca textual, mas participam do grafo.
Trechos sobrepostos no mesmo arquivo são deduplicados após a ordenação. Há no
máximo 10 resultados, 4000 bytes UTF-8 por trecho e 16000 no total. Corte recebe
`excerpt_truncated=true`; hashes e linhas identificam o trecho completo.

Nenhum candidato gera `no_lexical_evidence`, sem resposta inventada. Isso não prova
ausência de implementação: pode ser uma limitação do vocabulário ou da busca.
Saída 0: candidatos sem diagnósticos; 2: nenhum candidato ou diagnósticos de
extração; 1: erro de entrada, leitura ou integridade.

## O grafo

Derivado dos snapshots já conferidos, com nós de arquivo, símbolo e ocorrência
sintática. Cada aresta guarda fonte/hash, intervalo em bytes/linhas e hash do trecho.

| Relação | O que afirma |
| --- | --- |
| `declares` | O arquivo contém a declaração. |
| `contains` | Um símbolo está dentro do intervalo de outro (ex.: método dentro de impl). |
| `type_reference` / `declared_return` | Um tipo/retorno está escrito naquele trecho. |
| `call_observed` / `method_call_observed` | A expressão de chamada está presente; destino não resolvido. |
| `local_binding` | A declaração da variável/padrão está presente. |
| `name_candidate` | Há declaração de tipo com o mesmo nome; pode haver vários candidatos. Não resolve imports, aliases ou tipos. |

Na pergunta da câmera, o método tem retorno escrito `Ray`, uma variável `direction`
e uma chamada escrita `Ray::new`. O `impl Camera` contém esse método e `render`;
`render` tem referência escrita a `World` e chamada `world.color_at(...)`. Isso
permite navegar por vínculos observados, sem afirmar resolução de tipos ou chamadas.
`SceneSpec` só aparece quando a busca ou relações observadas levarem até ele.
Não criamos uma ligação câmera–cena simplesmente pelo significado das palavras.

A expansão parte somente do primeiro resultado, percorre arestas nos dois sentidos
preservando sua direção original e aceita profundidade 0 a 3. A consulta limita a
48 nós e 96 arestas; `truncated` informa corte por esses limites. A profundidade é
registrada separadamente. Arquivos são folhas na navegação para evitar trazer todos
os símbolos de um arquivo. Relações por nome continuam rotuladas como candidatas.
Rótulos da vizinhança têm limite de 256 bytes UTF-8; `labels_truncated` informa a
quantidade cortada. O grafo completo preserva rótulos integrais.

Para exportar e conferir o grafo inteiro, escolha um arquivo de destino novo:

```powershell
cargo run --locked --bin catalog_sources -- graph ai/acervo/renderer-auto-v1/edition ./renderer-graph-new.json
cargo run --locked --bin catalog_sources -- verify-graph ai/acervo/renderer-auto-v1/edition ./renderer-graph-new.json
```

A exportação local produziu **8619 nós e 9800 arestas**. `verify-graph` reconstrói
o grafo dos snapshots e compara todos os registros; hashes não autenticam autoria.
O grafo completo usado na validação ficou em `target/renderer-syntax-graph-v1.json`;
o repositório preserva a consulta limitada e a avaliação, evitando duplicar o grafo.

## Avaliação e reprodução

A [validação local](validation.json) registra **34 testes aprovados no Librarian**
e **280 nos alvos selecionados do renderer**. O binário independente do Librarian
também gerou um acervo do próprio projeto (20 fontes, 240 símbolos, zero diagnósticos)
e recuperou `prepare_query` primeiro para a pergunta `prepare query`.

[evaluation.json](evaluation.json) registra quatro perguntas com o símbolo esperado
em primeiro lugar e duas sem evidência lexical. As seis passaram. Este conjunto foi
usado no desenvolvimento e ajuste; não é avaliação independente de generalização.

```powershell
cargo test --locked --bin catalog_sources
# Na raiz do Librarian:
cargo test --workspace --locked
```

Testes cobrem normalização, aliases, ausência de evidência, limites UTF-8, relações
ambíguas, chamadas não resolvidas, hashes das arestas, limites de navegação e rejeição
de grafo adulterado. Nenhuma chamada LLM ou benchmark de ray tracing foi realizada.

A edição consultada é histórica e permaneceu intacta. `verify edition .` pode
relatar divergências após mudanças nos comandos/testes do projeto; isso é esperado.
A busca confere os snapshots, não promete representar o código atual. Uma edição
nova pode ser gerada pelo comando `generate` sem substituir a anterior.

O próximo passo é montar um dossiê/bundle com a seleção e suas relações, preservando
a distinção entre sintaxe, candidatos de resolução e afirmações avaliadas.
