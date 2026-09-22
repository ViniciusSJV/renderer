# Aula 19 — Investigar conferências repetidas

## Objetivo

Observar contadores de chamadas e bytes instrumentados e distinguir trabalho
lógico, tráfego físico de disco e tempo total.

## Contexto e pré-requisitos

Use Linux/Bash, a compilação release e `aula18-medicao/input-1.json` da Aula 18.
Todos os comandos partem da raiz. Não há dependência nova nem uso de Ollama.
O ensaio anterior mediu a CLI inteira; isso não identifica sozinho o custo de
cada componente.

## Implementação e conceitos

[src/bin/validate_evidence/metrics.rs](../../src/bin/validate_evidence/metrics.rs)
conta pontos instrumentados no validador e em seu módulo de captura. A variável
`BIBLIOTECARIO_METRICS=1` ativa o relatório em stderr; o JSON exportado não recebe
esses contadores. Sem valor 1, não há relatório, embora os pontos consultem a flag.

| Categoria | O que conta |
| --- | --- |
| `capture_link`, `capture_validate` | Chamadas de conferência. |
| `record_link_read`, `record_validate_read` | Leituras do JSON da captura. |
| `source_content_read` | Leitura da fonte documental. |
| `digest_calls` | Chamadas de hash de arquivo. |
| `digest_chunks` | Blocos e bytes entregues nas leituras de hash. |

Contadores de chamadas têm bytes zero, sem duplicar leituras no total. Leituras
de dossiê/pergunta/parecer, metadados, escrita, parsing e alocações não estão
inteiramente instrumentados. Bytes lógicos podem vir de caches e não equivalem
a tráfego físico. O relatório é emitido no sucesso; erros com `process::exit`
podem não produzi-lo.

## Passo a passo

Na raiz, rode a carga de uma ficha com instrumentação. A variável vale apenas
para este processo; o comando lê fontes e cria `aula19-selecao.json`:

```bash
BIBLIOTECARIO_METRICS=1 target/release/validate_evidence aula18-medicao/input-1.json --fact BENCH_0 --context 1 --output aula19-selecao.json
```

Observe a linha de métricas no terminal e a seleção no arquivo. Com a versão
atual e uma fonte ligada à captura, são duas conferências completas: fase inicial
e exportação. Os bytes dependem da captura local. Não espere os tamanhos históricos.

Execute a suíte na raiz para conferir as regras que sustentam o fluxo. Cargo
grava em `target` e os testes usam fixtures; não fazem geração de modelo:

```bash
cargo test --locked --bin validate_evidence
```

A versão instrumentada original passou em **72 testes**. Na árvore atual o
total é maior; procure falhas, não equivalência da contagem de testes.

## Previsão e observação históricas

As [previsões](../experimentos/10-trabalho-repetido/previsoes.md) esperavam N + 1
conferências para N fichas de uma fonte. Os
[registros de contagem](../experimentos/10-trabalho-repetido/contagens.json)
confirmaram isso na versão anterior ao reaproveitamento:

| Fichas | Conferências | Chamadas de hash | Bytes instrumentados |
| --- | --- | --- | --- |
| 1 | 2 | 12 | 41.880 |
| 10 | 11 | 66 | 227.910 |
| 100 | 101 | 606 | 2.088.210 |

Nessa carga, cada conferência calculava hashes de saída e cinco fontes, e o
registro era lido em dois pontos. A expressão observada para bytes foi
540 + (N + 1) × 20.670, específica daqueles arquivos. As exportações preservaram
os hashes da Aula 18; uma execução sem métricas também preservou a saída.

## Validação e limites

A árvore atual já reutiliza conferências: selecionar muitas fichas não deve
ser apresentado como reprodução do N + 1 histórico. Uma nova medição descreve
a implementação atual e exige registrar essa diferença.

Se não aparecerem métricas, confira valor da variável e sucesso da CLI antes
de interpretar ausência como zero operações. Uma captura divergente impede
exportação. Esses contadores confirmam repetição, mas não atribuem a ela uma
porcentagem do tempo nem demonstram o maior gargalo. Não houve cronômetro nesta
etapa histórica.

## Resultado da aula e próxima aula

Trabalho repetido tem evidência própria, separada de latência. A
[Aula 20](20-reaproveitar-conferencia.md) delimita o reaproveitamento por fonte
e examina uma comparação histórica de tempo e equivalência.
