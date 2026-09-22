# Aula 5 — Seleção e contexto pelo Bibliotecário

## Objetivo

Selecionar uma ficha por ID e exportar sua referência com uma janela de linhas
vizinhas, preservando a diferença entre endereço exato e contexto.

## Contexto e pré-requisitos

Conclua a conferência do dossiê de tuplas da Aula 4. Use Rust/Cargo da Aula 3 e
um editor de JSON. Todos os comandos partem da raiz. Não é necessário Ollama.
O acervo pode ter muitas obras; uma pergunta costuma precisar de poucas fichas.
Selecionar uma ficha não dispensa validar o dossiê inteiro.

## Implementação e conceitos

Em [validate_evidence.rs](../../src/bin/validate_evidence.rs), `find_fact` compara
IDs e retorna `Some` ou `None`. Para F fichas, a busca faz até F comparações.
`selection_json` reúne afirmação, ID, caminho, hash, linha e trecho;
`serde_json::to_string_pretty` serializa esses dados em JSON legível.

`source.line` e `source.excerpt` mantêm a referência exata. `source.context`
guarda raio solicitado, início, fim e textos. `--context N` pede até N linhas
antes e depois; o padrão é 3 e zero inclui apenas a linha referenciada.

O cálculo, após validar a referência, usa índices Rust a partir de zero:

```rust
let index = fact.line - 1;
let start = index.saturating_sub(radius);
let end = fact.line.saturating_add(radius).min(source.lines.len());
```

Operações saturadas impedem subtração negativa e estouro. `start..end` exclui o
fim; no JSON, as linhas começam em 1 e `end_line` é inclusivo. A janela trata
texto, não a sintaxe do Rust: pode cortar uma função ou omitir uma macro relevante.

## Passo a passo

Primeiro consulte a ficha no terminal. Cargo pode gravar em `target`; o programa
somente lê o dossiê e suas fontes:

```bash
cargo run --locked --bin validate_evidence -- ai/experimentos/03-tuplas/evidencias.json --fact F_VECTOR_TEST_X
```

Observe a ficha solicitada e sua referência. Uma falha de hash ou de referência
impede usar a seleção como conferida; consulte o diagnóstico da Aula 4.

Agora exporte para um arquivo novo na raiz, com quatro linhas de cada lado.
Isso cria apenas a exportação, além dos artefatos usuais do Cargo:

```bash
cargo run --locked --bin validate_evidence -- ai/experimentos/03-tuplas/evidencias.json --fact F_VECTOR_TEST_X --context 4 --output aula05-consulta.json
```

Abra `aula05-consulta.json` no editor. A ficha deve permanecer F_VECTOR_TEST_X,
com linha e trecho exatos e limites da janela. O terminal é um resumo; a janela
completa está no JSON. Destino existente é recusado por `create_new(true)`;
para repetir, escolha outro nome, sem apagar o anterior. O diretório pai deve existir.

## Comparação com os artefatos históricos

| Artefato | Conteúdo registrado |
| --- | --- |
| [consulta-vector-x.json](../experimentos/03-tuplas/consulta-vector-x.json) | Exportação inicial apenas da linha 159. |
| [consulta-vector-x-contexto.json](../experimentos/03-tuplas/consulta-vector-x-contexto.json) | Raio 3, linhas 156–162. |
| [consulta-vector-x-contexto4.json](../experimentos/03-tuplas/consulta-vector-x-contexto4.json) | Raio 4, linhas 155–163. |

Neste trecho, raio 3 não inclui a chave final do teste; raio 4 inclui atributo,
assinatura, corpo e fechamento. Isso não garante completude em outros trechos.
A versão atual exporta também campos introduzidos depois; não exija igualdade
byte a byte com uma exportação de uma versão anterior.

## Validação

Na raiz, execute os testes de seleção, contexto e demais regras do validador.
Eles usam `target` e arquivos temporários; não chamam o modelo:

```bash
cargo test --locked --bin validate_evidence
```

A etapa original teve **32 testes aprovados**. Duas exportações com os mesmos
dados, parâmetros e implementação produziram bytes iguais, identificados por:

```text
d1dd27b0225e7cfea690e4cb62fc7fe257d8016f825584756856da17bc4d3e26
```

Esse hash é histórico, não o valor esperado da versão atual. As verificações
incluíram raio negativo, não numérico, ausente ou repetido e recusa de destino
existente, preservando seus bytes. Nem todos esses ensaios de CLI eram testes
permanentes. Não houve benchmark nem prova de repetibilidade do Qwen.

## Resultado da aula e próxima aula

Uma ficha agora pode ser transportada com contexto delimitado, sem copiar linhas
manualmente. Ela ainda não é uma consulta completa nem uma afirmação comprovada.
A [Aula 6](06-metadados-das-fontes.md) estuda a preservação da natureza e da
procedência declarada das fontes.
