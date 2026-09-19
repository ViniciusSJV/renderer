# Aula 5 — Seleção e contexto pelo Bibliotecário

## Conceito

O acervo pode conter muitas obras, mas uma pergunta costuma precisar de poucas
fichas. Vamos selecionar uma ficha, conferir suas referências e exportar o
material consultado. Mantemos o Qwen com a configuração atual enquanto tornamos
suas entradas reproduzíveis.

Na biblioteca, isso corresponde a pedir uma ficha pelo código e receber uma
cópia do trecho marcado, acompanhada de linhas vizinhas e identificação da obra.

## Implementação

Em [validate_evidence.rs](../../src/bin/validate_evidence.rs), `find_fact` percorre
as fichas comparando IDs. Devolve `Some` com a referência encontrada ou `None`.
A busca faz até F comparações para F fichas. Antes da seleção, o programa ainda
valida o dossiê inteiro, incluindo versões disponíveis e referências; selecionar
uma ficha não elimina esse trabalho.

`selection_json` reúne a afirmação, ID, caminho, hash, linha e trecho. A chamada
`serde_json::to_string_pretty` transforma esses dados em JSON: serialização é o
caminho inverso da desserialização estudada anteriormente.

O arquivo de saída contém os dados selecionados; mensagens de execução ficam
no terminal. `create_new(true)` impede sobrescrever uma exportação existente.

## Contexto: uma janela de linhas

A linha exata continua em `source.line` e `source.excerpt`. As vizinhas ficam em
`source.context`, com raio solicitado, linha inicial, linha final e lista de textos.

O parâmetro `--context N` pede até N linhas antes e N depois. Sem ele, usamos 3.
Zero inclui apenas a linha referenciada. O início é limitado a zero nos índices
Rust e o fim à quantidade de linhas. Usamos operações saturadas para evitar
subtração negativa ou estouro na soma dos limites.

```rust
let index = fact.line - 1;
let start = index.saturating_sub(radius);
let end = fact.line.saturating_add(radius).min(source.lines.len());
```

A referência é validada antes dessa conta. `start..end` exclui o índice final;
no JSON, os números de linha começam em 1 e `end_line` é inclusivo.

O programa não interpreta a estrutura do Rust. Uma janela pode cortar uma
função, incluir parte de outra ou deixar de fora uma macro relevante. Mais
contexto não é automaticamente melhor, e não medimos seu efeito no Qwen.

## Execução

Na raiz do projeto, para consultar a ficha no terminal:

```bash
cargo run --bin validate_evidence -- ai/experimentos/03-tuplas/evidencias.json --fact F_VECTOR_TEST_X
```

Para exportar com quatro linhas de cada lado:

```bash
cargo run --bin validate_evidence -- ai/experimentos/03-tuplas/evidencias.json --fact F_VECTOR_TEST_X --context 4 --output consulta.json
```

Escolha um caminho ainda inexistente. O diretório de destino precisa existir.
`--context` e `--output` podem trocar de ordem depois de `--fact ID`. A janela
é incluída no JSON exportado; a apresentação no terminal continua resumida.
Os modos anteriores, com dossiê e parecer opcional, permanecem disponíveis.

## Experimentos preservados

| Arquivo | Conteúdo |
| --- | --- |
| [consulta-vector-x.json](../experimentos/03-tuplas/consulta-vector-x.json) | Exportação inicial, só a linha 159. |
| [consulta-vector-x-contexto.json](../experimentos/03-tuplas/consulta-vector-x-contexto.json) | Janela de raio 3, linhas 156–162. |
| [consulta-vector-x-contexto4.json](../experimentos/03-tuplas/consulta-vector-x-contexto4.json) | Janela de raio 4, linhas 155–163. |

A janela de raio 3 não inclui a chave final do teste. Neste exemplo, raio 4 inclui
atributo, assinatura, corpo e fechamento. Isso foi conferido neste trecho; não é
uma garantia geral da regra de seleção.

## Testes e medição

Os **32 testes do binário passaram** ao concluir a etapa. Eles incluem busca de
ficha, referência inválida, exportação, limites da janela e repetição da seleção.

Também executamos duas exportações com o mesmo dossiê, ficha e raio 4, em arquivos
temporários distintos. Os bytes foram iguais, com SHA-256:

```text
d1dd27b0225e7cfea690e4cb62fc7fe257d8016f825584756856da17bc4d3e26
```

Conferimos a rejeição de raio negativo, texto não numérico, valor ausente e opção
repetida. Uma tentativa de escrever sobre arquivo existente falhou e preservou
seus bytes. Esses testes do comando foram executados separadamente; não são todos
casos permanentes da suíte Rust.

A igualdade observada vale para os mesmos dados, parâmetros e implementação.
Não promete bytes idênticos entre versões futuras do programa ou bibliotecas.
Também não implica repetibilidade das respostas do Qwen. Não fizemos benchmark.

## Resultado e próximos passos

A Aula 5 está concluída: selecionamos uma ficha, exportamos sua referência e
contexto configurável e verificamos a repetibilidade da exportação.

O artefato ainda é material de consulta, não um prompt completo: falta a pergunta
e seu critério de avaliação. A próxima etapa poderá montar essa consulta a partir
do material exportado, sem copiar os trechos manualmente. Depois ampliaremos para
mais de uma ficha, mantendo as origens e evitando contexto repetido.

O Bibliotecário não comprovou o significado da afirmação. Ele preparou um pacote
de evidências identificadas, com limites explícitos, para que a análise seja
avaliada separadamente.
