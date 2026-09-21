# Aula 21 — Testar o reaproveitamento com várias fontes

## Conceito e previsão

Na Aula 20, muitas fichas da mesma fonte reutilizam uma conferência durante a
exportação. Agora mantemos 100 fichas e variamos o número de fontes documentais.
O [protocolo prévio](../experimentos/12-varias-fontes/protocolo.md) prevê duas
conferências por fonte: uma na validação inicial e uma durante a exportação.

As fontes sintéticas têm IDs diferentes, mas apontam para a mesma saída e
captura. Isso testa identidade e escopo do reaproveitamento, não 100 arquivos
físicos ou execuções independentes. As fichas repetem conteúdo; não são novas
evidências sobre o renderer.

## Implementação do experimento

Criamos cargas com 1, 10 e 100 fontes, distribuindo as 100 fichas igualmente.
Preservamos as cargas como `input-1.json`, `input-10.json` e `input-100.json`
em `experimentos/12-varias-fontes/`. A carga de uma fonte mantém os metadados
da Aula 18 para permitir comparação exata da exportação.

Não alteramos Rust. Executamos o binário release existente com
`BIBLIOTECARIO_METRICS=1`, selecionando todas as fichas e contexto 1. Os comandos
completos, hashes dos arquivos selecionados e resultados estão nos registros
`captura-1`, `captura-10`, `captura-100` e `captura-invalido`. Cada destino foi
criado uma única vez. Para repetir, use destinos novos; os pais devem existir.

A preparação e as assertions da sessão foram orquestradas com Python, sem
criar outro medidor ou implementação do Bibliotecário. O programa exercitado
continua sendo o binário Rust. Não houve medição de tempo nesta etapa.

## Testes e medição

Conferimos por assertions, para cada carga válida:

- Código 0, 100 seleções e correspondência de IDs de ficha e fonte.
- Status `capture_and_source_match` em todas as seleções.
- `capture_link`, `capture_validate`, `record_link_read` e
  `record_validate_read` iguais a 2 × fontes.
- `source_content_read` igual a 3 × fontes; `digest_calls` igual a 12 × fontes.
- Bytes instrumentados iguais a 41.880 × fontes.

| Fichas | Fontes documentais | Conferências de captura | Chamadas de hash | Bytes instrumentados |
| --- | --- | --- | --- | --- |
| 100 | 1 | 2 | 12 | 41.880 |
| 100 | 10 | 20 | 120 | 418.800 |
| 100 | 100 | 200 | 1.200 | 4.188.000 |

Todas as previsões corresponderam. A exportação de uma fonte manteve o SHA-256
da Aula 20. Para dez fontes, uma execução adicional sem métricas produziu bytes
idênticos e stderr vazio. As contagens estão em
[resultados.json](../experimentos/12-varias-fontes/resultados.json).

Criamos uma quarta carga, separada, com dez fontes e `run_id` incorreto na
última. O Bibliotecário retornou **1** com erro de identidade e não criou a
exportação. O capturador retornou 0 porque registrou corretamente essa falha.
Essa recusa ocorreu na validação inicial da CLI; não é um teste de alteração
concorrente após preencher o mapa da exportação.

Não repetimos os 73 testes unitários da Aula 20, pois não alteramos código.
Os testes desta aula foram as execuções da CLI e suas assertions. Nenhum teste
do renderer ou consulta ao Qwen foi executado.

## Explicação e fechamento

O reaproveitamento está limitado ao ID de fonte. Muitas fichas de uma fonte
reduzem conferências repetidas; muitas fontes com IDs distintos continuam
conferidas separadamente, mesmo apontando para a mesma captura.
Não podemos chamar essa multiplicação de fontes de carga com arquivos únicos.
Os bytes são os entregues à aplicação nos pontos instrumentados, não tráfego
físico de disco nem medida de latência. Não demonstramos novo ganho de tempo.

A **Aula 21 está concluída**: cargas ampliadas, previsões confirmadas, vínculos
preservados e ligação inválida recusada. Não houve mudança na política de cache,
no formato exportado ou nos registros históricos. As notas das aulas 10 e 17
permanecem 3/6; o envio ao Ollama continua manual. Não há comprovação retroativa
de RUN_VECTOR_1.

Na **Aula 22 — Separar fonte documental de captura compartilhada**, propomos
examinar quais verificações pertencem à fonte e quais pertencem à captura antes
de decidir se vale reutilizar também por registro. Caminho sozinho não será
tratado como identidade suficiente, e toda reutilização precisará de escopo
explícito diante de arquivos mutáveis. Ainda não implementamos essa mudança.
