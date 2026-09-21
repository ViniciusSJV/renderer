# Aula 19 — Investigar conferências repetidas

## Conceito e previsão

A Aula 18 mediu tempo total da CLI. Agora contamos trabalho lógico: chamadas
de conferência e bytes entregues à aplicação nas leituras instrumentadas.
Esses bytes não representam tráfego físico de disco: caches podem atender
leituras repetidas. Não estamos medindo renderização.

As [previsões](../experimentos/10-trabalho-repetido/previsoes.md) foram registradas
antes das execuções. Com uma fonte e N fichas selecionadas, previmos N + 1
conferências da captura: uma na validação inicial, outra para cada seleção.

## Implementação

Criamos [metrics.rs](../../src/bin/validate_evidence/metrics.rs) e instrumentamos
as leituras em validate_evidence.rs e capture.rs. Para ativar:

```bash
BIBLIOTECARIO_METRICS=1 target/release/validate_evidence ai/experimentos/09-custo-conferencia/baseline/input-1.json --fact BENCH_0 --context 1 --output /tmp/aula19-exemplo-novo.json
```

O destino deve ser novo. A linha `BIBLIOTECARIO_METRICS` vai para stderr com JSON;
a exportação das fichas permanece intacta. Sem a variável igual a 1, não há
contagem nem relatório, embora os pontos instrumentados ainda consultem a flag.

Categorias:

- `capture_link` e `capture_validate`: chamadas de conferência.
- `record_link_read` e `record_validate_read`: leituras completas do JSON.
- `source_content_read`: leitura do conteúdo da fonte documental.
- `digest_calls`: chamadas da função que lê e calcula hash de arquivo.
- `digest_chunks`: blocos e bytes retornados nessas leituras de hash.

Contadores de chamadas têm bytes zero; não representam leituras adicionais.
O total soma somente as quatro categorias de bytes, sem contar chamadas duas
vezes. Leituras de dossiê/pergunta/parecer, metadados, escrita, alocações e custo
de parsing não estão instrumentados. O relatório é emitido ao concluir com
sucesso; saídas por erro via process::exit não o emitem. Não usamos essas
contagens como diagnóstico completo de execuções interrompidas.

## Teste e medição

Os **72 testes do Bibliotecário passaram**. Depois compilamos release e rodamos
as três cargas preservadas da Aula 18, cada uma em um processo separado.
Conferimos por assertions todas as fórmulas previstas e hashes das exportações.
Um teste adicional pela CLI sem a variável confirmou stderr vazio e a mesma
exportação. Não cronometramos a versão instrumentada.

| Fichas | Conferências de captura | Chamadas de hash de arquivo | Bytes instrumentados |
| --- | --- | --- | --- |
| 1 | 2 | 12 | 41.880 |
| 10 | 11 | 66 | 227.910 |
| 100 | 101 | 606 | 2.088.210 |

Todas as previsões corresponderam às contagens. As três exportações tiveram
SHA-256 idêntico ao registrado na Aula 18. Os resultados completos estão em
[contagens.json](../experimentos/10-trabalho-repetido/contagens.json).
Cada carga tem uma captura própria, RUN_COUNTS_1_1, RUN_COUNTS_10_1 e
RUN_COUNTS_100_1, com comando exato, ambiente parcial, hashes e saída. Os hashes
selecionados incluem o executável e os arquivos de instrumentação.

## Explicação

Há trabalho repetido observado: para 100 fichas de uma única fonte, conferimos
a mesma captura 101 vezes. Cada uma das 101 conferências calcula hashes de
seis arquivos: saída e cinco fontes associadas. O JSON da captura também é
lido em dois pontos por ligação: identidade/hash e conferência interna.

A relação observada para bytes dessas cargas é **540 + (N + 1) × 20.670**.
Ela depende dos tamanhos desses arquivos e desse fluxo; não é uma fórmula
geral para qualquer dossiê. Não é possível atribuir uma porcentagem do tempo
da Aula 18 aos hashes com esses contadores. Não demonstramos que eles sejam
o maior gargalo e ainda não fizemos otimização.

## Fechamento e próxima aula

A **Aula 19 está concluída**: previsão, instrumentação, testes e contagens
confirmaram repetição, preservando os bytes exportados. Os registros históricos
foram mantidos; recompilar o binário pode causar divergência com o hash de
executáveis de capturas anteriores, sem refutar suas observações históricas.
As notas das aulas 10 e 17 permanecem 3/6; o envio ao Ollama continua manual.
Não há comprovação retroativa de RUN_VECTOR_1.

Na **Aula 20 — Reaproveitar a conferência dentro de uma exportação**, propomos
definir o escopo de reutilização por fonte e seus limites diante de arquivos
mutáveis. Depois implementaremos, conferiremos equivalência das exportações
e repetiremos contagens e medições antes de afirmar ganho. Não criaremos
confiança permanente em um arquivo apenas por ele ter sido conferido antes.
