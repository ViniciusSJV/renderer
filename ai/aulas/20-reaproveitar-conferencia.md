# Aula 20 — Reaproveitar a conferência dentro de uma exportação

## Objetivo

Entender o escopo do reaproveitamento por fonte e verificar que reduzir leituras
não dispensa equivalência de resultados nem define, sozinho, ganho de tempo.

## Contexto e pré-requisitos

Use Linux/Bash, Cargo e as cargas locais da Aula 18. Os comandos partem da raiz;
Ollama não participa. A Aula 19 mostrou trabalho repetido, mas não isolou seu
custo. O [protocolo](../experimentos/11-reuso-conferencia/protocolo.md) prevê
reutilizar resultados somente dentro da montagem de uma exportação.

## Implementação e conceitos

Em [validate_evidence.rs](../../src/bin/validate_evidence.rs), um `HashMap` local
em `selections_json` guarda a conferência por ID de fonte. IDs são validados como
únicos antes do mapa. `selection_with_capture` monta cada ficha, mantendo a
conferência da referência e da execução legada. O JSON do resultado é clonado:
a repetição do conteúdo exportado e seu custo não desaparecem.

Uma falha interrompe a exportação. Ao retornar, o mapa é descartado: não há
cache entre comandos. A validação inicial da CLI e a montagem da exportação
continuam separadas, resultando em duas conferências para uma fonte.

Reutilizar significa observar um resultado anterior dentro dessa montagem.
Não garante detectar alterações posteriores à leitura. Nenhuma das versões
oferece snapshot atômico. Fontes com IDs diferentes ainda exigem suas próprias
verificações; o compartilhamento do documento de captura virá na Aula 23.

## Passo a passo

Na raiz, execute a suíte para verificar o isolamento entre fontes/exportações.
Cargo usa `target` e os testes usam arquivos temporários:

```bash
cargo test --locked --bin validate_evidence
```

A implementação original teve **73 testes aprovados**, incluindo recusa de
ligação modificada numa nova exportação e de segunda fonte inválida.
A árvore atual contém as extensões posteriores.

Compile a versão atual em release, sem executar o ensaio:

```bash
cargo build --locked --release --bin validate_evidence --bin bench_evidence
```

Na raiz Linux, observe duas fichas da mesma fonte com instrumentação. Este
comando lê a carga existente e cria uma exportação nova:

```bash
BIBLIOTECARIO_METRICS=1 target/release/validate_evidence aula18-medicao/input-10.json --fact BENCH_0 --fact BENCH_1 --context 1 --output aula20-selecao.json
```

Observe duas seleções e duas conferências completas, sem multiplicar a conferência
por cada ficha. Os bytes dependem dos arquivos locais. Para repetir o ensaio
completo atual, sem ativar métricas, crie outro destino:

```bash
target/release/bench_evidence target/release/validate_evidence aula16-dossie.json aula20-medicao
```

Abra `aula20-medicao/results.json` e compare com a medição local da Aula 18.
Como os dois comandos deste roteiro usam a mesma implementação atual, diferenças
entre eles não demonstram o ganho histórico da otimização.

## Evidência histórica

Os [contadores](../experimentos/11-reuso-conferencia/contagens.json) passaram a
duas conferências, 12 chamadas de hash e 41.880 bytes nas três cargas de uma
fonte. Os hashes de exportação permaneceram iguais aos da Aula 18.

A comparação temporal usou um binário anterior preservado temporariamente e o
novo na mesma sessão, com três aquecimentos e 15 amostras por carga:

| Fichas | Mediana anterior (ms) | Mediana nova (ms) |
| --- | --- | --- |
| 1 | 1,569 | 1,592 |
| 10 | 3,619 | 2,044 |
| 100 | 26,513 | 5,997 |

As amostras estão em [antes](../experimentos/11-reuso-conferencia/antes/results.json)
e [depois](../experimentos/11-reuso-conferencia/depois/results.json).
Exportações das duas versões tinham hashes iguais. O executável anterior era
temporário e não foi versionado; não há binário anterior pronto para repetir
esta comparação no clone. Seus hashes e resultados continuam registrados.

## Validação e limites

Houve redução da mediana com 10 e 100 fichas naquela comparação; uma ficha não
melhorou. Não houve randomização entre versões, isolamento da máquina ou análise
de significância. As contagens não medem disco físico nem tempo isolado de hash.
Esses dados não garantem ganho em outra carga, ambiente ou no renderer.

Se a exportação falhar, examine identidade e hashes antes de atribuir o erro ao
cache. Destinos novos são obrigatórios. Capturas históricas não devem ser
reescritas para acompanhar o código atual.

## Resultado da aula e próxima aula

Reaproveitamento tem escopo explícito e evidência limitada de desempenho.
A [Aula 21](21-testar-varias-fontes.md) separa muitas fichas de uma fonte de
muitas fontes documentais ligadas à mesma captura.
