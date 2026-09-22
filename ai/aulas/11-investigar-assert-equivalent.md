# Aula 11 — Investigar assert_equivalent!

## Objetivo

Seguir a cadeia de dependências da macro e delimitar o significado da comparação
numérica, sem confundir leitura do código com execução observada.

## Contexto e pré-requisitos

A Aula 10 mostrou que a chamada de uma macro não fornece sua implementação.
Use Cargo, o editor e o dossiê de tuplas das aulas anteriores. Não é necessário
Ollama nem outra dependência. Os comandos partem da raiz.

## Conceitos e investigação

Abra `src/tuple.rs`, `src/equivalent.rs` e `src/lib.rs` no editor. Siga a cadeia:
`assert_equivalent!` → `not_equivalent` → `equivalent` para `f64` → `EPSILON`.

| Evidência | Trecho registrado | O que mostra |
| --- | --- | --- |
| F_TUPLE_X_F64 e F_TUPLE_Y_F64 | `src/tuple.rs`, linhas 6–7 | Tipos de x e y. |
| F_MACRO_OPERANDS | `src/equivalent.rs`, linha 20 | Vinculação dos operandos com `match`. |
| F_MACRO_CONDITION e F_MACRO_PANIC | `src/equivalent.rs`, linhas 22–23 | Pânico quando `not_equivalent` é verdadeiro. |
| F_NOT_EQUIVALENT | `src/equivalent.rs`, linha 7 | Negação de `equivalent`. |
| F_F64_EQUIVALENCE | `src/equivalent.rs`, linha 11 | Implementação específica para f64. |
| F_ABSOLUTE_TOLERANCE | `src/equivalent.rs`, linha 13 | Diferença absoluta estritamente menor que EPSILON. |
| F_EPSILON_IMPORT | `src/equivalent.rs`, linha 1 | Constante importada da raiz do crate. |
| F_EPSILON_VALUE | `src/lib.rs`, linha 1 | Valor declarado 0.00001. |

As linhas são as registradas no dossiê; em versões alteradas, procure os símbolos.
A macro delega a comparação a métodos: a fórmula para `f64` não deve ser atribuída
a qualquer tipo aceito pela macro. Tolerância absoluta não divide a diferença
pela magnitude dos operandos. Valores distintos podem ser aceitos; aceitação
não significa igualdade exata.

## Implementação documental

[evidencias-macro.json](../experimentos/03-tuplas/evidencias-macro.json) preserva
as fontes anteriores e acrescenta duas fontes de código e dez fichas manuais,
com hashes e linhas completas. Nenhuma execução ou origem histórica foi atribuída
a essas novas cópias. A consulta antiga continua sem a macro.

## Passo a passo

Na raiz, confira o dossiê. O comando lê suas fontes e pode compilar em `target`,
sem exportar ou executar os casos da macro:

```bash
cargo run --locked --bin validate_evidence -- ai/experimentos/03-tuplas/evidencias-macro.json
```

Com os arquivos correspondentes, observe quatro fontes e 17 fichas sem referências
inválidas. Uma divergência atual precisa ser examinada, não apagada do histórico.

Exporte as dez fichas para um destino novo na raiz. Isso cria uma seleção sem
pergunta e sem chamada ao modelo:

```bash
cargo run --locked --bin validate_evidence -- ai/experimentos/03-tuplas/evidencias-macro.json --fact F_TUPLE_X_F64 --fact F_TUPLE_Y_F64 --fact F_MACRO_OPERANDS --fact F_MACRO_CONDITION --fact F_MACRO_PANIC --fact F_NOT_EQUIVALENT --fact F_F64_EQUIVALENCE --fact F_ABSOLUTE_TOLERANCE --fact F_EPSILON_IMPORT --fact F_EPSILON_VALUE --context 4 --output aula11-macro.json
```

Abra o resultado e confira que a cadeia pode ser percorrida pelas referências.
Compare com [consulta-implementacao-macro.json](../experimentos/03-tuplas/consulta-implementacao-macro.json).
Se o destino existir, escolha outro nome; não substitua a seleção anterior.

## Validação e limites

A validação histórica terminou com código 0, quatro fontes e 17 fichas.
Isso confere referências, hashes, cópias e campos previstos do relatório antigo;
o significado das fichas continua sendo analisado manualmente.

Pelo código, uma diferença calculada igual a EPSILON não satisfaz `< EPSILON`.
Esta aula ainda não observa a fronteira em execução, nem investiga NaN, infinitos
ou arredondamento em outras magnitudes. O hash atual não recupera as entradas
do compilador em RUN_VECTOR_1. A nota da Aula 10 permanece 3/6.

Não houve alteração de Rust, novo teste da macro, consulta ao Qwen ou benchmark
na investigação histórica. Contar fontes e fichas não mede desempenho.

## Resultado da aula e próxima aula

A premissa da comparação está documentada. A
[Aula 12](12-observar-fronteira-tolerancia.md) registra previsões e testa valores
iguais, abaixo, no limite e acima da tolerância.
