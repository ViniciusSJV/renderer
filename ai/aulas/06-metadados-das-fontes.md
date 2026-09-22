# Aula 6 — Preservar a natureza e a procedência declarada das fontes

## Objetivo

Verificar o transporte de metadados do dossiê à exportação sem transformar
classificação, autoria ou commit declarados em comprovação.

## Contexto e pré-requisitos

Use o ambiente da Aula 3 e a seleção da Aula 5. O problema desta etapa é a perda
de etiquetas durante a desserialização: o arquivo continha informações que as
estruturas iniciais não carregavam. Não é necessário instalar outra dependência
nem executar Ollama. Todos os comandos partem da raiz.

## Conceitos e implementação

Em [validate_evidence.rs](../../src/bin/validate_evidence.rs), os campos opcionais
permitem distinguir informação fornecida de informação ausente:

| Estrutura | Campo | Tipo |
| --- | --- | --- |
| `Source` | `kind` | `Option<String>` |
| `Source` | `executed` | `Option<bool>` |
| `Source` | `git_commit` | `Option<String>` |
| `Fact` | `authorship` | `Option<String>` |

`kind` é texto livre, como `rust_source`, `test_run` ou `pseudocode`, não uma
categoria semanticamente validada. `Some(true)`, `Some(false)` e `None` são
três situações distintas. Ausência ou `null` não equivalem a falso; a string
`"false"` não é o booleano `false` e é recusada na leitura.

O código não deduz execução pela palavra “ok” no trecho. Commit e autoria são
preservados, mas não autenticados. `metadata_scope` comunica esse limite.

## Evolução que precisa permanecer visível

Na etapa original, SRC_TUPLE exportava `executed: false` por ser fonte de código;
TEST_VECTOR_1 exportava `executed: true` por ser registro de teste. Essa convenção
não descrevia adequadamente as várias execuções possíveis de um mesmo código.

**Na versão atual, `rust_source` já omite `executed` na exportação**, mudança
explicada na Aula 8. A leitura do campo legado continua possível. Para observar
a versão antiga, consulte os artefatos preservados; não altere o programa atual
para obter a saída antiga.

## Passo a passo

Na raiz, exporte a ficha de código para um destino novo. O comando lê dossiê e
fontes, cria a exportação e pode compilar em `target`:

```bash
cargo run --locked --bin validate_evidence -- ai/experimentos/03-tuplas/evidencias.json --fact F_VECTOR_TEST_X --context 4 --output aula06-codigo.json
```

No editor, confira `kind`, `git_commit`, autoria e os textos de escopo. Não
espere o booleano de execução na fonte Rust da versão atual.

Exporte a ficha do resultado, ainda na raiz, para outro arquivo novo:

```bash
cargo run --locked --bin validate_evidence -- ai/experimentos/03-tuplas/evidencias.json --fact F_VECTOR_TEST_PASSED --context 4 --output aula06-execucao.json
```

Observe `test_run` e a declaração legada de execução. O dossiê deste comando
não é a versão posterior com execução identificada. Compare com os históricos
[de código](../experimentos/03-tuplas/consulta-vector-x-metadados.json) e
[de teste](../experimentos/03-tuplas/consulta-teste-vector-metadados.json).

## Validação e problemas comuns

Execute na raiz a suíte que verifica tipos, valores ausentes e preservação dos
metadados. Ela usa artefatos em `target` e arquivos temporários:

```bash
cargo test --locked --bin validate_evidence
```

A implementação original teve **37 testes aprovados** e duas exportações
conferidas. O total atual inclui aulas posteriores. Se o JSON rejeitar um campo,
confira primeiro seu tipo. Se o destino existir, use outro nome; se o hash da
fonte divergir, trate a diferença como na Aula 4.

Preservar informações não garante que o modelo as interprete corretamente.
Não houve nova consulta ao Qwen ou benchmark nesta etapa. As exportações antigas
não constituem pacotes completos de reprodução, nem prova do vínculo com o commit.

## Resultado da aula e próxima aula

Metadados acompanham as evidências com limites explícitos. A
[Aula 7](07-identidade-limites-e-consulta.md) acrescenta identidade do dossiê,
questões abertas e a pergunta separada das evidências.
