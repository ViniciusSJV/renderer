# Aula 8 — Código não é execução

## Objetivo

Representar uma execução identificada e conferir os campos transcritos de um
relatório, sem tratar código-fonte como registro de execução.

## Contexto e pré-requisitos

Na Aula 7, um booleano na fonte permitiu confundir natureza do documento com
histórico do código. Use o ambiente Cargo da Aula 3, os conceitos de metadados
e o dossiê de tuplas. Os comandos partem da raiz; não precisam de Ollama.

## Conceitos

| Pergunta | Evidência necessária |
| --- | --- |
| O que o teste descreve? | Fonte de código e trecho. |
| O que um relatório registra? | Documento daquela execução. |
| Os dados transcritos correspondem ao relatório? | Comparação com seu cabeçalho. |

Conferir uma transcrição não autentica o relatório nem reexecuta o procedimento.
TEST_VECTOR_1 identifica uma fonte documental; RUN_VECTOR_1 identifica a execução
catalogada. O segundo ID foi atribuído ao relatório existente, sem novo teste.

## Implementação

Em [validate_evidence.rs](../../src/bin/validate_evidence.rs), exportações de
`rust_source` omitem `executed`, embora aceitem a leitura do campo legado.
`execution_scope` explica que o código não informa quando ou com qual resultado
foi executado.

`Source` pode conter `execution: Option<ExecutionRecord>`. Quando presente,
`ExecutionRecord` exige `id`, `command`, `started_at`, `finished_at` e `exit_code`.
Horários continuam strings; sua presença não valida a cronologia.

`validate_execution_record` exige `kind: test_run`, procura comando, início,
fim e código no cabeçalho e rejeita ausências, repetições ou divergências.
O cabeçalho termina na primeira linha vazia. Texto semelhante no corpo da saída
não substitui o cabeçalho. A comparação é textual exata; o ID catalogado não tem
correspondente no relatório antigo.

A CLI confere versões e execução antes de exportar. A seleção também confere o
registro ao ser chamada diretamente. O campo `source.execution_validation`
lista status, base e campos comparados:

```json
{
  "status": "matches_report_header",
  "compared_fields": ["command", "started_at", "finished_at", "exit_code"],
  "basis": "source.lines: cabeçalho anterior à primeira linha vazia; comparação textual exata."
}
```

Sem registro, `test_run` recebe `no_execution_record` e lista vazia, preservando
o booleano legado como declaração. Com registro, o booleano é omitido. O cabeçalho
pode estar fora da janela exportada: a conferência usa a fonte completa.

## Passo a passo

Na raiz, consulte o resultado identificado. O comando lê o dossiê e fontes,
imprime diagnósticos e pode compilar em `target`; não reexecuta o teste registrado:

```bash
cargo run --locked --bin validate_evidence -- ai/experimentos/03-tuplas/evidencias-execucao-identificada.json --fact F_VECTOR_TEST_PASSED
```

Observe ficha, fonte e execução como identidades diferentes. Havendo divergência
de arquivo, a recusa atual não altera a observação histórica.

Exporte para um destino novo, ainda na raiz. Isso cria `aula08-consulta.json`:

```bash
cargo run --locked --bin validate_evidence -- ai/experimentos/03-tuplas/evidencias-execucao-identificada.json --fact F_VECTOR_TEST_PASSED --context 4 --output aula08-consulta.json
```

No editor, confira `execution_validation` e sua lista de quatro campos. Compare
com a [exportação histórica conferida](../experimentos/03-tuplas/consulta-execucao-conferida.json)
e a [anterior à conferência](../experimentos/03-tuplas/consulta-execucao-identificada.json).
O destino novo evita a recusa de sobrescrita; não altere os artefatos históricos.

## Validação e limites

Na raiz, execute os testes. Eles usam `target` e entradas temporárias para casos
inválidos, sem gerar nova resposta do modelo:

```bash
cargo test --locked --bin validate_evidence
```

A evolução histórica passou por 47 e 52 testes, chegando a **53 aprovados** com
o resultado explícito da conferência. Os casos cobrem cada campo divergente,
ausência, duplicação e texto fora do cabeçalho. Uma transcrição com código não
zero passa se corresponder ao relatório: conferência não é aprovação do comando.
Uma cópia com código alterado foi recusada sem criar exportação.

Não são verificados autenticidade, cronologia, unicidade global de execuções,
significado do comando ou coerência entre código de saída e resumo dos testes.
A associação histórica entre código e execução também não foi comprovada.
Não houve benchmark nem consulta ao Qwen nesta etapa.

## Resultado da aula e próxima aula

Código e execução têm papéis separados, e a exportação informa o alcance da
conferência. A [Aula 9](09-reunir-evidencias.md) reúne várias fichas preservando
suas origens e compartilhando somente contexto compatível.
