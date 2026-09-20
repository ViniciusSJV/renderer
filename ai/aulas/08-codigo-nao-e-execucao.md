# Aula 8 — Código não é execução

## Conceito

Na Aula 7, o Qwen interpretou `executed: false` numa fonte de código como
indicação de que o teste nunca havia sido executado. Nosso contrato permitia
essa confusão: um arquivo descreve um procedimento, mas pode participar de
muitas execuções. Um booleano na fonte não representa esse histórico.

Separamos três perguntas:

| Pergunta | Material necessário |
| --- | --- |
| O que está escrito no teste? | Fonte de código e trecho correspondente. |
| O que o relatório registra sobre uma execução? | Registro identificado daquela execução. |
| Os campos transcritos correspondem ao relatório? | Comparação explícita entre os campos e o cabeçalho. |

A terceira conferência não autentica o relatório nem reexecuta o procedimento.

## Implementação em etapas

Em [validate_evidence.rs](../../src/bin/validate_evidence.rs), deixamos de
exportar `executed` para `rust_source`. A leitura do campo legado continua
permitida. O texto `execution_scope` explica que a fonte de código não informa
se, quando ou com qual resultado o código foi executado.

Acrescentamos `ExecutionRecord`, com `id`, `command`, `started_at`, `finished_at`
e `exit_code`. `Source` contém `execution: Option<ExecutionRecord>`: fontes
antigas podem continuar sem esse registro. Os campos de um registro presente
são obrigatórios na desserialização. Horários ainda são Strings.

O [dossiê identificado](../experimentos/03-tuplas/evidencias-execucao-identificada.json)
transcreve os dados do relatório existente. `RUN_VECTOR_1` foi atribuído ao
catalogar; não houve nova execução do teste do vetor. A
[primeira exportação](../experimentos/03-tuplas/consulta-execucao-identificada.json)
preserva esses campos, mas foi produzida antes da conferência automática.

Depois implementamos `validate_execution_record`. Quando há `execution`, ela:

1. Exige `kind: test_run`.
2. Procura comando, início, fim e código de término no cabeçalho de `source.lines`.
3. Rejeita campos ausentes, repetidos ou diferentes dos valores transcritos.

Neste formato, o cabeçalho termina na primeira linha vazia. Textos semelhantes
na saída do comando não substituem campos do cabeçalho. A comparação é textual
exata; o inteiro `exit_code` é convertido em texto para ser comparado.
O ID atribuído ao catalogar não possui correspondente no relatório.

A CLI confere primeiro as versões das fontes, depois os registros de execução,
antes de exportar. `selection_json` também confere o registro recebido antes de
produzir seu resultado de validação, mesmo quando chamada diretamente.

## Comunicar o que foi conferido

A [nova exportação](../experimentos/03-tuplas/consulta-execucao-conferida.json)
inclui `source.execution_validation`:

```json
{
  "status": "matches_report_header",
  "compared_fields": ["command", "started_at", "finished_at", "exit_code"],
  "basis": "source.lines: cabeçalho anterior à primeira linha vazia; comparação textual exata."
}
```

Para `test_run` sem `execution`, o status é `no_execution_record` e a lista de
campos comparados é vazia. Isso expressa ausência da conferência, não ausência
de execução. Nesse caso, preservamos o booleano legado como declaração.
Com registro identificado, omitimos esse booleano. Fontes `rust_source` não
recebem `execution_validation`.

O cabeçalho pode ficar fora da janela de contexto exportada. A conferência usa
as linhas completas da fonte; a janela continua limitada pela ficha e pelo raio.
O resultado exportado descreve uma operação do Bibliotecário, não oferece ao
leitor uma autenticação independente do JSON.

## Testes e experimento

Na raiz do projeto:

```bash
cargo test --offline --bin validate_evidence
cargo run --offline --bin validate_evidence -- ai/experimentos/03-tuplas/evidencias-execucao-identificada.json --fact F_VECTOR_TEST_PASSED
```

A suíte passou de 47 testes na etapa inicial para 52 com a conferência do
cabeçalho e **53 com o resultado explícito na exportação**. Os casos incluem
divergências em cada campo, ausência, duplicação, textos na saída do comando,
compatibilidade com fontes legadas e rejeição de exportação inconsistente.

Um código de término não zero também passa quando corresponde ao cabeçalho:
validar a transcrição não significa que o comando registrado teve sucesso.

Pela CLI, o dossiê identificado passou. Uma cópia temporária com `exit_code`
alterado para 1 foi rejeitada com saída 1, sem criar a exportação. O dossiê
legado também passou. A exportação final foi gerada com:

```bash
cargo run --offline --bin validate_evidence -- ai/experimentos/03-tuplas/evidencias-execucao-identificada.json --fact F_VECTOR_TEST_PASSED --context 4 --output ai/experimentos/03-tuplas/consulta-execucao-conferida.json
```

Esse arquivo já existe: para repetir, escolha outro destino. Conferimos que
os campos da execução e a janela de contexto correspondem à exportação anterior.
Os artefatos anteriores foram preservados.

## Medição e limites

Os resultados medem verificações de correção nos casos exercitados. Não houve
benchmark, execução da suíte completa do renderer ou nova consulta ao Qwen.
Não demonstramos melhora das respostas do modelo.

Não verificamos autenticidade, validade ou ordem cronológica dos horários,
significado do comando, unicidade dos IDs de execução, nem coerência do código
de término com o resumo dos testes. A associação histórica entre a versão do
código e a execução também não foi comprovada automaticamente. O hash do
relatório e a concordância da transcrição não resolvem essas questões.

## Fechamento e Aula 9

A Aula 8 está concluída: código e registro de execução têm papéis distintos,
os campos transcritos são conferidos e a exportação comunica o alcance da regra.

Na **Aula 9 — Reunir evidências sem misturar seus papéis**, vamos selecionar
mais de uma ficha na mesma consulta: uma sobre o procedimento no código e outra
sobre o resultado registrado. Estudaremos como preservar as referências e os
limites de cada fonte e evitar repetir contexto compartilhado. Estarem juntas
na consulta não comprova que aquela versão do código produziu aquele resultado.
Começaremos pelo contrato da seleção múltipla, antes de implementá-la.
