# Aula 6 — Preservar a natureza e a procedência declarada das fontes

## Conceito

O Bibliotecário já entregava uma ficha com trecho, endereço e hash. Porém,
deixava no acervo algumas etiquetas importantes: a natureza da obra, a declaração
de execução e a autoria da ficha. Uma informação existente no dossiê desaparecia
na desserialização, antes de chegar à consulta.

Nosso objetivo nesta aula foi preservar quatro campos já usados nos exemplos,
sem transformar essas declarações em provas e sem criar todas as abstrações
futuras de observação, hipótese e experimento.

## Implementação

Em [validate_evidence.rs](../../src/bin/validate_evidence.rs), acrescentamos:

| Estrutura | Campo | Tipo |
| --- | --- | --- |
| Source | kind | Option<String> |
| Source | executed | Option<bool> |
| Source | git_commit | Option<String> |
| Fact | authorship | Option<String> |

Todos atravessam o caminho dossiê → estrutura Rust → seleção JSON.

`kind` preserva a categoria declarada, como rust_source, test_run ou pseudocode.
Ainda é texto livre: não há enumeração fechada nem validação de significado.
Isso permite estudar a preservação sem decidir agora todo o vocabulário futuro.

`executed` precisa distinguir três situações:

- `Some(true)`: o dossiê declara execução.
- `Some(false)`: o dossiê declara que não houve execução.
- `None`: informação ausente ou null no JSON.

Ausência não equivale a false. Também não inferimos test_run ou executed a
partir de um trecho que contenha a palavra “ok”. A String "false" não é o booleano
false e é rejeitada ao carregar esse campo.

O commit e a autoria também são declarações preservadas. O programa não confirma
que o arquivo pertence ao commit, nem que a autoria declarada está correta.
O campo metadata_scope da exportação explica esse limite.

No exemplo, SRC_TUPLE tem executed false porque representa a fonte de código,
não um registro de execução. Isso não significa que esse código nunca rodou.
TEST_VECTOR_1 tem executed true porque representa o registro de um teste.
Essa convenção não é ainda um modelo formal de execuções.

## Testes

Os testes adicionados nesta aula verificam:

1. Preservação do tipo declarado, inclusive uma categoria ainda desconhecida.
2. Ausência de inferência de tipo quando o campo não existe.
3. Distinção entre true, false e informação ausente.
4. Preservação da autoria e do commit declarados.
5. Rejeição de texto onde se espera um booleano.

Executamos `cargo test --offline --bin validate_evidence`: **37 testes passaram**.
Esse total inclui os testes das aulas anteriores. Também executamos duas
exportações reais e conferimos seus campos.

## Experimento

Na raiz do projeto, usando caminhos de saída ainda inexistentes:

```bash
cargo run --bin validate_evidence -- ai/experimentos/03-tuplas/evidencias.json --fact F_VECTOR_TEST_X --context 4 --output consulta-codigo.json
cargo run --bin validate_evidence -- ai/experimentos/03-tuplas/evidencias.json --fact F_VECTOR_TEST_PASSED --context 4 --output consulta-execucao.json
```

Os resultados produzidos durante a aula estão preservados em:

- [consulta-vector-x-metadados.json](../experimentos/03-tuplas/consulta-vector-x-metadados.json)
- [consulta-teste-vector-metadados.json](../experimentos/03-tuplas/consulta-teste-vector-metadados.json)

A primeira preserva rust_source e false; a segunda, test_run e true.
Ambas preservam a autoria manual e o commit declarado. As exportações anteriores
permanecem como artefatos das etapas em que foram geradas.

## Explicação e limites

Agora o trecho chega acompanhado de sua classificação declarada. Isso permite
que uma análise diferencie código de registro de teste sem depender somente do
nome da ficha ou do texto do trecho.

Não verificamos que o modelo usará esses campos corretamente. Não houve nova
consulta ao Qwen, medição de qualidade do LLM ou benchmark nesta aula.

Ainda não preservamos na seleção todos os metadados do dossiê: ID do conjunto,
questões abertas, estado completo da árvore de trabalho e limitações adicionais
continuam fora dela. Também não verificamos consistência entre categorias e
campos, autenticidade dos logs ou relação histórica com o commit. Estas duas
exportações não são pacotes completos de procedência ou reprodução experimental.

O ganho desta etapa é evitar a perda de quatro informações existentes, mantendo
explícita a diferença entre declaração e validação.

## Fechamento

A Aula 6 está concluída. Mantivemos o modelo e o renderer sem alterações e
fortalecemos o percurso das informações no Bibliotecário. Antes de tratar a
exportação como uma consulta completa, ainda precisaremos decidir como levar
identidade do dossiê e limitações relevantes, além da pergunta e dos critérios.
Esse trabalho fica para a próxima sessão.
