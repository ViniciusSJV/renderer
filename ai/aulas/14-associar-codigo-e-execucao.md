# Aula 14 — Associação entre código e execução

## Conceito: identificar bytes não é comprovar compilação

A Aula 13 registra uma execução. Agora queremos responder: quais arquivos
selecionados foram observados junto dessa execução? HEAD sozinho não basta,
porque a árvore de trabalho pode conter alterações não commitadas.

Um SHA-256 identifica o conteúdo observado de um arquivo. Nossa proposta é
coletar esse hash em dois momentos: antes de iniciar o comando e depois de
seu término. O registro ligará essas observações ao mesmo `run_id`.

| Observação | O que permite afirmar |
| --- | --- |
| Hash antes igual ao hash depois | Os conteúdos observados nos dois momentos coincidem. |
| Hash antes diferente do hash depois | Foi observada uma mudança de conteúdo entre as leituras. |
| Arquivo não pôde ser lido depois | A conferência desse arquivo ficou incompleta. |

Hashes iguais não demonstram que o arquivo ficou intacto durante todo o
intervalo: ele pode mudar e voltar. Também não demonstram que o compilador
leu aquele arquivo, que não reutilizou um artefato ou que não havia outras
entradas relevantes. Não estamos autenticando a captura.

## Implementação

Implementamos a extensão em `src/bin/capture_execution.rs`:

1. Acrescentamos `--source CAMINHO` repetível ao capturador Rust, resolvido a
   partir do diretório de execução, e registrar os caminhos selecionados.
2. Coletamos SHA-256 antes de iniciar o comando. Se uma fonte selecionada não
   puder ser lida, recusar a execução, sem apresentar associação completa.
3. Coletamos novamente depois e registrar cada resultado: igual, diferente ou
   indisponível. Falha nessa leitura não deve apagar o resultado do comando.
4. Identificamos também os bytes de `saida.bin` por SHA-256. Essa conferência
   detecta divergência em relação ao hash registrado; não impede que alguém
   altere conjuntamente o arquivo e o JSON.
5. O formato agora usa `schema_version: 2`. Os registros da versão 1 foram
   preservados. Sem `--source`, a lista `sources` fica vazia, sem associação
   a fontes declarada.

Para a fronteira, a seleção inicial incluiu o teste de integração,
`src/equivalent.rs`, `src/lib.rs`, `Cargo.toml` e `Cargo.lock`. Essa lista é
deliberadamente parcial: não é o inventário completo de entradas do Cargo.
A conferência pelo Bibliotecário será tratada explicitamente; o JSON da
Aula 13 ainda não é importado pelo validador atual.

## Formato e tratamento de falhas

Cada item de `sources` contém `path` (seleção original), `resolved_path`
(caminho absoluto a partir de cwd), `before_sha256`, `after_sha256` e
`comparison`: `equal`, `different` ou `unavailable`. Na indisponibilidade
posterior, `after_sha256` é null e `after_error` informa a causa.
Os caminhos das fontes são relativos ao cwd do comando; o destino continua
relativo ao diretório em que o capturador foi chamado.

As leituras são sequenciais, não uma fotografia atômica do conjunto. Links
simbólicos são seguidos na leitura; os hashes descrevem os bytes encontrados.
Uma falha antes do comando deixa no máximo uma captura parcial, sem JSON final.
Uma falha de leitura de fonte depois não apaga o resultado do comando. Falha
na gravação ou no hash da saída impede publicar o JSON final. O código 0 do
capturador continua significando registro concluído, mesmo com diferenças ou
fontes indisponíveis depois; consulte `result` e `sources` separadamente.

## Teste

```bash
cargo test --offline --bin capture_execution
```

**11 testes passaram.** Além dos oito da Aula 13, testamos fontes iguais,
modificadas e removidas depois (preservando inclusive código 7 do comando),
fonte ausente antes impedindo a execução e a CLI com `--source` repetido e
caminhos relativos a um cwd diferente. Os hashes de `abc` e da saída vazia
foram comparados a valores SHA-256 conhecidos. Os arquivos modificados pelos
testes ficam em diretórios temporários.

## Nova execução e medição

Após os testes, compilamos o capturador e executamos:

```bash
cargo build --offline --bin capture_execution
target/debug/capture_execution --id RUN_EQUIVALENCE_BOUNDARY_2 --destino ai/experimentos/05-associacao/fronteira --source tests/equivalence_boundary.rs --source src/equivalent.rs --source src/lib.rs --source Cargo.toml --source Cargo.lock -- cargo test --offline --test equivalence_boundary -- --test-threads=1
```

O diretório pai deve existir. O destino acima já está ocupado: para reproduzir,
use um novo destino e ID.

O [registro](../experimentos/05-associacao/fronteira/execucao.json) contém
**5 fontes com hashes iguais antes/depois**, **0 diferentes** e **0 indisponíveis**.
A [saída](../experimentos/05-associacao/fronteira/saida.bin) contém **540 bytes**,
com **4 testes aprovados**, código de término **0**. Os testes no limite e acima
de EPSILON continuam esperando pânico: aprovados significa rejeição esperada.

Uma conferência independente com hashlib recalculou o hash e o tamanho da
saída e os hashes atuais das cinco fontes; todos corresponderam ao registro.
Essa conferência da sessão ainda não é uma função do Bibliotecário. As contagens
não são benchmark nem medida de custo da macro. Não repetimos a suíte do
Bibliotecário: sua implementação não mudou.

## Explicação, fechamento e próxima aula

A **Aula 14 está concluída** no escopo de associação por observação: um mesmo
registro reúne comando, resultado, saída identificada e hashes das fontes
selecionadas em dois momentos. Não comprovamos as entradas efetivas do
compilador, não autenticamos a execução e não capturamos todas as dependências.
Nenhum resultado foi atribuído retroativamente a RUN_VECTOR_1 ou
RUN_EQUIVALENCE_BOUNDARY_1. A avaliação da Aula 10 permanece **3/6** e o envio
ao Ollama continua manual.

Na **Aula 15 — Conferir o registro de captura**, a proposta é integrar a leitura
do formato novo ao Bibliotecário, conferir saída e associações declaradas,
testando divergências sem confundir integridade com autenticação. Esse trabalho
ainda está pendente. Depois seguimos integração e avaliação, performance e
criação de cenas por linguagem natural.
