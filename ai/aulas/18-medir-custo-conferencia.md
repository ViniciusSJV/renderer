# Aula 18 — Medir o custo da conferência

## Objetivo

Medir o tempo total de exportação de 1, 10 e 100 fichas ligadas à mesma captura,
preservando amostras e distinguindo custo observado de hipótese causal.

## Contexto e pré-requisitos

Use Linux/Bash, Cargo e `aula16-dossie.json`, com sua captura ainda conferível.
Todos os comandos partem da raiz. Ollama não participa. O
[protocolo histórico](../experimentos/09-custo-conferencia/protocolo.md) definiu
cargas antes da medição: são cópias sintéticas de uma ficha, não fatos independentes.

A versão inicial repetia conferências por ficha. A árvore atual já inclui as
reutilizações das aulas 20 e 23; medir agora produz uma linha de base da versão
atual, não recria o algoritmo anterior nem seus tempos.

## Conceitos e implementação

[src/bin/bench_evidence.rs](../../src/bin/bench_evidence.rs) prepara as cargas e
mede, com `Instant`, da criação do processo validador ao seu término. Compilação
fica fora do tempo. stdout é descartado; stderr é guardado para diagnóstico.

Cada carga recebe três aquecimentos e 15 amostras. A ordem é rotacionada por
rodada, sem ensaios paralelos iniciados pelo medidor. CPU e caches não são isolados.
Fora do intervalo medido, o programa confere JSON, quantidade e IDs de fichas,
status de captura e igualdade dos hashes das exportações da mesma carga.

O medidor remove apenas exportações temporárias produzidas por ele e preserva
entradas e resultados brutos. O destino é novo; falhas podem deixar arquivos
parciais e impedem publicar `results.json`. O executável também tem hash
comparado antes/depois, sem comprovação de suas entradas de compilação.

## Passo a passo

Na raiz, teste o medidor. Cargo grava em `target`; não inicia o benchmark completo:

```bash
cargo test --locked --bin bench_evidence
```

Observe os dois testes de geração de cargas e cálculo da mediana aprovados.
Compile os dois binários em release; isso grava em `target/release`, sem medir:

```bash
cargo build --locked --release --bin validate_evidence --bin bench_evidence
```

Agora execute o ensaio com o dossiê local. O comando cria `aula18-medicao`,
prepara cargas e realiza 54 processos: nove aquecimentos e 45 medições:

```bash
target/release/bench_evidence target/release/validate_evidence aula16-dossie.json aula18-medicao
```

Abra `aula18-medicao/results.json` no editor. Verifique 15 amostras por carga,
45 itens de ordem, tamanhos, hashes e resumo. Se a captura divergir, o medidor
interrompe; não use uma execução incompleta como resultado. Destino existente
exige outro nome, não sobrescrita.

## Evidência histórica

Os [resultados originais](../experimentos/09-custo-conferencia/baseline/results.json)
e a [captura do ensaio](../experimentos/09-custo-conferencia/captura/execucao.json)
preservam a versão medida antes das otimizações:

| Fichas | Entrada (bytes) | Exportação (bytes) | Mínimo (ms) | Mediana (ms) | Máximo (ms) |
| --- | --- | --- | --- | --- | --- |
| 1 | 1.558 | 2.798 | 1,512 | 1,563 | 1,616 |
| 10 | 3.395 | 29.461 | 3,494 | 3,558 | 3,699 |
| 100 | 21.846 | 288.401 | 22,358 | 22,687 | 26,033 |

São valores registrados, não resultados esperados da repetição atual.
O tempo inclui processo, leitura, conferência, serialização e escrita. Não inclui
preparação das cargas ou conferência posterior. Uma ficha usa formato individual;
as demais usam formato múltiplo. Tamanho do arquivo não é total de bytes lidos.

## Validação e limites

Compare primeiro correção e identidade das exportações, depois tempos.
O ensaio histórico verificou as 54 execuções e a estabilidade de bytes por carga.
Aumento de tempo com fichas não isola hashes, alocações, buscas ou escrita.
Não estabelece uma lei geral de complexidade nem mede o renderer ou o modelo.

Não houve comparação de versões ou otimização nesta etapa histórica. Caches
aquecidos e interferência do ambiente limitam a conclusão. Binários de `target`
não são versionados; recompilá-los não preserva o hash de executáveis antigos.

## Resultado da aula e próxima aula

O custo total tem um protocolo e amostras examináveis. A
[Aula 19](19-investigar-conferencias-repetidas.md) acrescenta contadores de
operações e bytes para investigar trabalho repetido sem confundi-lo com tempo.
