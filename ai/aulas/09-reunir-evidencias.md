# Aula 9 — Reunir evidências sem misturar seus papéis

## Objetivo

Selecionar várias fichas e compartilhar janelas de contexto sobrepostas ou
adjacentes, mantendo a referência individual e a identidade de cada fonte.

## Contexto e pré-requisitos

Use Cargo, o editor e o dossiê com execução identificada da Aula 8. Todos os
comandos partem da raiz. Ollama não é necessário. Reunir código e resultado
não comprova que aquela versão do código produziu aquela execução.

## Implementação: seleção múltipla

Em [validate_evidence.rs](../../src/bin/validate_evidence.rs), `--fact ID` pode
ser repetido. `select_facts` resolve os IDs na ordem solicitada e rejeita IDs
inexistentes ou repetidos. O dossiê inteiro continua sendo validado antes da
seleção. Uma ficha mantém o formato individual; várias geram `selections`.

A [exportação inicial](../experimentos/03-tuplas/consulta-codigo-e-resultado-inicial.json)
ainda repetia as janelas em cada seleção. Essa etapa intermediária teve 57 testes
aprovados. O formato seguinte compartilha as janelas em `contexts`.

## Conceitos: janelas e blocos

Cada `source.context` de uma seleção múltipla contém raio solicitado, início,
fim e `context_id`. O bloco compartilhado tem ID local, `source_id`, limites e
`lines`. Linha e trecho exatos permanecem em `source.line` e `source.excerpt`:
a repetição desse trecho curto é intencional.

`merge_ranges` ordena intervalos por fonte e une os que se sobrepõem ou encostam.
Uma lacuna inicia outro bloco; textos iguais de fontes diferentes não são unidos.
Para K janelas de uma fonte, ordenar custa O(K log K) comparações e a passagem
seguinte é linear. Isso não mede o custo total da exportação, que também faz
buscas e serialização.

As fichas ficam na ordem solicitada; fontes, na ordem da primeira seleção;
blocos de cada fonte, na ordem de linha. Uma janela individual pode ser menor
que seu bloco, mas seus limites são preservados. Não há análise da sintaxe Rust.

## Passo a passo

Na raiz, exporte x, y e resultado para um destino novo. O comando lê as fontes,
pode compilar em `target` e cria somente a nova exportação documental:

```bash
cargo run --locked --bin validate_evidence -- ai/experimentos/03-tuplas/evidencias-execucao-identificada.json --fact F_VECTOR_TEST_X --fact F_VECTOR_TEST_Y --fact F_VECTOR_TEST_PASSED --context 4 --output aula09-consulta.json
```

Abra `aula09-consulta.json` e siga cada `context_id` até o bloco correspondente.
Compare com o [artefato histórico](../experimentos/03-tuplas/consulta-contexto-compartilhado.json):

| Ficha | Janela individual | Bloco compartilhado |
| --- | --- | --- |
| F_VECTOR_TEST_X | SRC_TUPLE, 155–163 | CTX_1, 155–164 |
| F_VECTOR_TEST_Y | SRC_TUPLE, 156–164 | CTX_1, 155–164 |
| F_VECTOR_TEST_PASSED | TEST_VECTOR_1, 23–29 | CTX_2, 23–29 |

As janelas individuais somavam 25 linhas e os blocos somam 17. Essa contagem
não inclui excertos individuais, metadados, tamanho total do JSON ou tokens.

## Validação e problemas comuns

Na raiz, execute a suíte; ela grava artefatos em `target` e usa entradas de teste,
sem consultar o modelo:

```bash
cargo test --locked --bin validate_evidence
```

O formato final desta etapa teve **60 testes aprovados**: sobreposição, adjacência,
inclusão, lacunas, fontes distintas, ordem, limites extremos e preservação dos
papéis de código e execução. A seleção individual manteve seu formato.

Se um contexto parecer maior que o solicitado, compare primeiro a janela da
ficha com o bloco compartilhado. Se o comando rejeitar um ID, confira repetição
ou erro de grafia. Destinos existentes são recusados; use outro nome.

Uma repetição histórica com os mesmos parâmetros produziu bytes iguais.
`--question` também preservou seleções e blocos, mas a pergunta de teste não foi
enviada ao Qwen. IDs CTX valem apenas dentro da exportação, sem identidade global.
Não houve benchmark, contagem de tokens ou melhora demonstrada do modelo.

## Resultado da aula e próxima aula

As evidências podem ser reunidas com menos contexto repetido e sem misturar
fontes. A [Aula 10](10-avaliar-codigo-e-resultado.md) acrescenta uma pergunta
e critérios prévios para avaliar a explicação desse conjunto.
