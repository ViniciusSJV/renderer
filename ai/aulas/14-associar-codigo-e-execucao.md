# Aula 14 — Associação entre código e execução

## Objetivo

Capturar os testes de fronteira com hashes de cinco arquivos selecionados,
coletados antes e depois do comando, e delimitar o que essa associação prova.

## Contexto e pré-requisitos

Use Linux/Bash com as ferramentas da Aula 13 e os testes da Aula 12. Todos os
comandos partem da raiz. A captura nova será usada nas próximas aulas; preserve
seu diretório e mantenha os arquivos selecionados sem alterações entre etapas.
Ollama não é necessário.

## Conceitos

HEAD sozinho não identifica arquivos modificados. SHA-256 identifica os bytes
observados de cada arquivo em um momento:

| Comparação | Conclusão permitida |
| --- | --- |
| Hash antes igual ao posterior | Conteúdos observados nas duas leituras coincidem. |
| Hash diferente | Houve diferença de conteúdo entre as leituras. |
| Leitura posterior indisponível | A observação posterior ficou incompleta. |

Hashes iguais não demonstram imutabilidade durante todo o intervalo: o arquivo
pode mudar e voltar. Também não provam que o compilador leu esses bytes, quais
artefatos reutilizou ou se outras entradas eram relevantes.

## Implementação

Em [capture_execution.rs](../../src/bin/capture_execution.rs), `--source CAMINHO`
é repetível. O formato 2 inclui `path`, `resolved_path`, `before_sha256`,
`after_sha256` e `comparison` por fonte, além de tamanho e hash de `saida.bin`.
Sem `--source`, a lista fica vazia.

A falha de leitura anterior impede executar o comando; uma falha posterior
preserva o resultado e registra `unavailable`, hash nulo e diagnóstico.
`equal` e `different` exigem os hashes correspondentes. Os caminhos são resolvidos
a partir do diretório do comando. Leituras sequenciais e links simbólicos não
produzem snapshot atômico. Erro de gravação ou hash da saída impede o JSON final.

## Passo a passo

Na raiz Linux, execute os testes da extensão. Cargo escreve em `target`; fontes
modificadas ou removidas pelos casos de teste ficam em diretórios temporários:

```bash
cargo test --locked --bin capture_execution
```

A extensão teve **11 testes aprovados**, cobrindo mudanças antes/depois, fonte
ausente, argumentos, hashes conhecidos e preservação de código não zero.

Compile o capturador para invocá-lo diretamente. Isso cria ou atualiza o binário
em `target/debug`, sem executar os testes de fronteira:

```bash
cargo build --locked --bin capture_execution
```

Capture uma execução nova na raiz. O comando cria `aula14-fronteira`, registra
cinco fontes e executa os quatro testes. O destino deve estar ausente:

```bash
target/debug/capture_execution --id AULA14_FRONTEIRA_01 --destino aula14-fronteira --source tests/equivalence_boundary.rs --source src/equivalent.rs --source src/lib.rs --source Cargo.toml --source Cargo.lock -- cargo test --locked --test equivalence_boundary -- --test-threads=1
```

Abra `aula14-fronteira/execucao.json` e `saida.bin`. Observe quatro aprovações,
`result.exit_code` igual a 0 e cinco comparações. Se nada alterou os arquivos,
elas devem ser `equal`. Tamanho da saída, horários, diretórios e hashes variam.
O código 0 do capturador sozinho não confirma esses resultados: leia o registro.

## Comparação com a evidência histórica

[RUN_EQUIVALENCE_BOUNDARY_2](../experimentos/05-associacao/fronteira/execucao.json)
registrou cinco fontes iguais, nenhuma diferente ou indisponível, e saída de
540 bytes com quatro aprovações. Uma conferência independente registrada na etapa
recalculou tamanho e hashes, que correspondiam naquele momento.

Esses valores não são requisitos para a captura nova. O registro histórico
contém caminhos do ambiente de origem e hashes antigos de Cargo.toml/Cargo.lock;
portanto não é uma captura portátil a reconferir automaticamente em qualquer clone.
A captura local nova preserva a identidade da nova execução sem reescrever a antiga.

## Validação e limites

Se não houver JSON final, examine o erro e os arquivos parciais. Se uma fonte
aparecer como `different` ou `unavailable`, investigue o arquivo antes de declarar
associação completa. Não altere o registro para converter a comparação em `equal`.

A lista de cinco arquivos é deliberadamente parcial, não o inventário completo
de entradas do Cargo. Aprovação com pânico esperado continua sendo rejeição do
par. Não houve benchmark nem autenticação da execução, e RUN_VECTOR_1 não ganhou
comprovação retroativa.

## Resultado da aula e próxima aula

Uma execução local reúne comando, saída identificada e observações de fontes.
A [Aula 15](15-conferir-registro-captura.md) usa o Bibliotecário para conferir
esse registro e seus arquivos atuais.
