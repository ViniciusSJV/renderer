# Aula 13 — Captura de execução

## Conceito

Uma captura válida pode registrar um comando que falhou. Se o comando termina
com código 7, guardar a saída e o código 7 é o comportamento correto do
capturador. Isso é diferente de não conseguir gravar o registro.

Antes da implementação, definimos um diretório novo por execução, contendo
`execucao.json` e `saida.bin`. O JSON registra versão do formato, identidade,
argumentos, diretório, início e fim UTC, ambiente e resultado. A saída preserva
os bytes combinados de stdout e stderr, inclusive bytes que não são UTF-8.
Não preserva a identidade de cada canal nem garante a ordem lógica entre
mensagens que o próprio programa tenha armazenado em buffers.

## Implementação

O [capturador](../../src/bin/capture_execution.rs) é um binário Rust do projeto,
com a biblioteca padrão e serde_json já disponível, sem novas dependências.
Esta versão é para Unix (o Codespaces usa Linux): um par de sockets locais
reúne stdout e stderr antes da cópia para o arquivo. Usa `date -u` para UTC
com resolução de segundos; falha nessa coleta impede concluir o registro. O comando é uma lista de argumentos;
não há interpretação automática por shell. A entrada padrão fica fechada
(`Stdio::null()`): esta versão é para comandos sem interação.

O ambiente inclui sistema operacional, versão do sistema, arquitetura,
Rust, Cargo, HEAD e estado Git. As sondagens de Rust, Cargo e Git registram
disponibilidade, saída, erro e código quando houver. O ambiente é observado
antes do comando, depois da reserva do destino; o estado Git pode incluir a
pasta reservada e alterações locais. Não copiamos todas as variáveis de ambiente.

O campo `argv` contém o comando realmente passado ao processo. `result` distingue:

| Situação | status | exit_code | Informação adicional |
| --- | --- | --- | --- |
| Término normal, com sucesso ou erro | exited | Código do processo | — |
| Não foi possível iniciar | start_failed | null | error |
| Término por sinal em POSIX | signaled | null | signal |

O capturador retorna **0 quando o registro foi concluído**, inclusive para
`start_failed` ou código não zero do comando. Retorna **2 para erros de captura
tratados ou argumentos inválidos**. Quem automatizar uma aprovação de testes
precisa ler `result`: o código do capturador sozinho não aprova o comando.

O destino deve ser novo e seu diretório pai deve existir. Um destino existente
é recusado antes de executar comandos. A saída é gravada progressivamente,
sem acumulá-la inteira na memória. Após fechar e sincronizar a saída, gravamos
e sincronizamos um JSON temporário, renomeado para `execucao.json` ao final.
Se uma etapa falhar, arquivos parciais podem permanecer: sem o JSON final,
não há registro concluído. Uma nova tentativa deve usar outro destino.

Essa publicação evita apresentar um JSON parcialmente escrito como concluído;
não oferece garantia completa contra queda de energia, adulteração ou mudanças
concorrentes. Não há timeout nem controle de árvores de processos nesta versão;
comandos que não terminam ou descendentes que mantêm a saída aberta podem
manter a captura em espera. Interrupções do capturador não têm um relatório
final garantido.

## Teste

Na raiz do repositório:

```bash
cargo test --offline --bin capture_execution
```

**8 testes passaram**: saída binária combinada e JSON, código não zero, comando
inexistente, sinal POSIX, destino existente sem executar, falha de publicação
sem registro final, separação dos códigos pela CLI e argumentos sem
interpretação automática por shell.
Os testes usam diretórios temporários. Não repetimos as suítes já concluídas
do Bibliotecário e da fronteira; seus arquivos Rust não foram alterados.

## Medição e exemplos registrados

Executamos três exemplos independentes, com identidades novas:

| Execução | Resultado | Bytes de saída | Código do capturador |
| --- | --- | --- | --- |
| RUN_CAPTURE_RUST_SUCCESS_1 | exited, código 0 | 19 | 0 |
| RUN_CAPTURE_RUST_FAILURE_1 | exited, código 7 | 17 | 0 |
| RUN_CAPTURE_RUST_MISSING_1 | start_failed, código null | 0 | 0 |

Registros: [sucesso](../experimentos/04-captura/rust-sucesso/execucao.json),
[falha](../experimentos/04-captura/rust-falha/execucao.json) e
[comando inexistente](../experimentos/04-captura/rust-inexistente/execucao.json).
Cada pasta contém também `saida.bin`. Conferimos os tamanhos registrados contra
os arquivos. O diagnóstico de início malsucedido fica no JSON, pois não houve
processo filho produzindo saída. Essas contagens não são um benchmark.

Para experimentar uma nova captura, usando um destino ainda inexistente:

```bash
cargo run --offline --bin capture_execution -- --id RUN_EXEMPLO_2 --destino ai/experimentos/04-captura/exemplo-2 -- printf 'ola\n'
```

O primeiro `--` separa as opções do Cargo; o segundo separa as opções do
capturador dos argumentos do comando. Os comandos
exatos dos três exemplos preservados estão no campo `argv` de cada JSON.

Os exemplos originais nas pastas `sucesso`, `falha` e `inexistente` foram
produzidos pelo protótipo Python e permanecem preservados como registros
históricos. O procedimento e os testes Python foram substituídos pelo binário
Rust; não reatribuímos as execuções antigas à implementação nova.

## Explicação e limites

Saída vazia não significa sucesso. Código diferente de zero não significa que
a captura falhou. `null` não é zero: nos exemplos, significa que não existe um
código de término normal a registrar. Essa separação é o aprendizado central.

O novo JSON ainda não é importado automaticamente pelo Bibliotecário. Seu
formato anterior continua conferindo referências, hashes, linhas e quatro
campos de cabeçalho. Não alteramos esse contrato nesta aula.

Os registros são observações locais, sem autenticação. HEAD não identifica
sozinho os arquivos modificados nem comprova os bytes compilados. Não há hash
ou associação automática com fontes neste formato inicial. Não reescrevemos
RUN_EQUIVALENCE_BOUNDARY_1 nem atribuímos comprovação retroativa a RUN_VECTOR_1.
A avaliação da Aula 10 permanece **3/6**; não houve consulta ao Qwen e o envio
ao Ollama continua manual. A revisão local das aulas 6–12 foi preservada.

## Fechamento e próxima aula

A **Aula 13 está concluída**: definimos o contrato antes de implementar,
criamos o procedimento reutilizável, testamos falhas e registramos exemplos.

Na **Aula 14 — Associação entre código e execução**, vamos estudar como
identificar os arquivos relacionados a uma nova execução e conferir essa
associação, distinguindo hashes observados de prova sobre os bytes compilados.
Continuamos na sequência Bibliotecário e evidências, integração e avaliação,
performance e, depois, cenas por linguagem natural.
