# Aula 18 — Medir o custo da conferência

## Conceito e previsão

Antes de otimizar, precisamos de uma linha de base. O
[protocolo](../experimentos/09-custo-conferencia/protocolo.md) foi escrito antes
da medição: exportar 1, 10 e 100 fichas da mesma fonte/captura, com IDs distintos.
São cópias sintéticas de uma afirmação, não evidências independentes.

A hipótese foi que repetir conferências por ficha pode contribuir para aumentar
o custo. A leitura da implementação mostra chamadas de `validate_capture_link`
na validação da fonte e na seleção de cada ficha. Isso não mede sua contribuição
no tempo total. Serialização, alocações, buscas e escrita também podem contribuir.

## Implementação

Criamos [bench_evidence.rs](../../src/bin/bench_evidence.rs), sem alterar o
validador. O medidor prepara três dossiês, executa o binário release diretamente
e usa `Instant` do início da criação do processo até seu término. A compilação
não entra no tempo. stdout é descartado e stderr é capturado para diagnóstico.

Cada carga recebe três aquecimentos e 15 amostras. A ordem é rotacionada por
rodada, sem ensaios paralelos iniciados pelo medidor. O ambiente continua
compartilhado; não isolamos CPU nem limpamos caches.

Após cada processo, fora do tempo medido, o medidor verifica JSON, quantidade
e IDs das seleções e status de conferência. Compara o hash da exportação com o
primeiro resultado da mesma carga. Remove somente a exportação temporária que
acabou de produzir. Dossiês e resultados brutos permanecem no destino.

Falha do comando, exportação inesperada ou mudança de bytes interrompe o ensaio.
O destino deve ser novo; `results.json` só é publicado ao concluir. Uma falha
pode deixar artefatos parciais. O JSON final registra comandos, ordem, amostras,
tamanhos, hashes de entrada/saída e do executável. O hash do executável foi
comparado antes/depois, sem comprovação do processo de compilação.

## Teste e execução

```bash
cargo test --offline --bin bench_evidence
cargo build --offline --release --bin validate_evidence --bin bench_evidence
```

**2 testes passaram**, conferindo geração das cargas e cálculo da mediana.
O ensaio real também verificou os resultados das 54 execuções: nove aquecimentos
e 45 medições. Os hashes de cada exportação permaneceram iguais nas repetições.
Não repetimos a suíte do validador, pois sua implementação não mudou nesta aula.

O comando do ensaio foi:

```bash
target/release/bench_evidence target/release/validate_evidence ai/experimentos/07-dossie-captura/evidencias.json ai/experimentos/09-custo-conferencia/baseline
```

Executamos esse comando pelo capturador em
[RUN_BENCH_EVIDENCE_BASELINE_1](../experimentos/09-custo-conferencia/captura/execucao.json),
que preservou ambiente parcial, argumentos, hashes de oito arquivos selecionados,
saída e código 0. O diretório pai deve existir; os destinos registrados já estão
ocupados. Use novos destinos e IDs para repetir.

Os [resultados brutos](../experimentos/09-custo-conferencia/baseline/results.json)
contêm todas as amostras em nanossegundos. Uma conferência posterior recalculou
as medianas e confirmou 15 amostras por carga e 45 registros de ordem.

## Medição

Valores de tempo em milissegundos, arredondados a três casas:

| Fichas | Entrada (bytes) | Exportação (bytes) | Mínimo | Mediana | Máximo |
| --- | --- | --- | --- | --- | --- |
| 1 | 1.558 | 2.798 | 1,512 | 1,563 | 1,616 |
| 10 | 3.395 | 29.461 | 3,494 | 3,558 | 3,699 |
| 100 | 21.846 | 288.401 | 22,358 | 22,687 | 26,033 |

Esses tamanhos são dos arquivos, não a quantidade total de bytes lidos pelo
processo. O tempo inclui inicialização, leitura, conferência, serialização e
escrita da CLI. Não inclui preparação das cargas nem análise posterior das
exportações. Não é custo isolado de SHA-256 ou da macro Rust.

## Explicação e limites

Neste ensaio, aumentar fichas coincidiu com aumento de tempo e tamanho da saída.
Ainda não sabemos quanto cabe a cada componente. Uma ficha usa formato individual;
as outras cargas usam formato múltiplo. Os caches foram aquecidos e o Codespaces
pode sofrer interferência externa. Três cargas e uma sessão não estabelecem uma
lei geral de complexidade nem desempenho em outras máquinas.

Não houve comparação entre versões nem otimização: **não demonstramos ganho de
performance**. Também não medimos alocações, CPU, tokens ou latência do Qwen.
Os binários em target são artefatos locais de build e não são versionados;
recompilá-los pode fazer seus hashes atuais divergirem da captura histórica.

## Fechamento e próxima aula

A **Aula 18 está concluída**: protocolo prévio, medidor Rust, testes, amostras
preservadas e interpretação limitada ao custo total observado. As avaliações
das aulas 10 e 17 permanecem **3/6**, sem nova consulta ao modelo. Nenhuma
comprovação retroativa foi atribuída a RUN_VECTOR_1.

Na **Aula 19 — Investigar conferências repetidas**, propomos contar chamadas e
bytes lidos para localizar trabalho repetido antes de escolher uma otimização.
Um eventual reaproveitamento deverá ter escopo e limites explícitos diante de
arquivos mutáveis. Continuamos depois com integração e avaliação e, mais adiante,
criação de cenas por linguagem natural. O envio ao Ollama permanece manual.
