# Aula 20 — Reaproveitar a conferência dentro de uma exportação

## Conceito

A Aula 19 confirmou conferências repetidas da mesma captura. Nesta aula o
resultado é reaproveitado por ID de fonte somente dentro de `selections_json`.
Não há cache entre exportações ou processos. Os IDs são validados como únicos
antes de usar o mapa. Fontes distintas continuam tendo conferências distintas,
mesmo se apontarem para a mesma captura.

Esse escopo muda o momento das observações: as fichas posteriores reutilizam
uma leitura anterior da mesma montagem. Não detectamos necessariamente alterações
posteriores a ela. Não existe snapshot atômico, nem na versão anterior.
O limite já exportado de conferência local sem snapshot permanece válido.

## Implementação e previsão

O [protocolo prévio](../experimentos/11-reuso-conferencia/protocolo.md) prevê duas
conferências nas cargas de uma fonte: validação inicial da CLI e montagem da
exportação. Mantivemos essas duas fases separadas nesta etapa.

Em validate_evidence.rs, um HashMap local guarda o resultado por fonte. A função
`selection_with_capture` monta cada ficha com esse resultado, mantendo suas
conferências de referência e execução legada. O resultado JSON é clonado para
cada ficha; não eliminamos a repetição do conteúdo exportado nem seu custo.
Uma falha de conferência interrompe a exportação. Ao retornar, o mapa é descartado.

## Testes e contagens

```bash
cargo test --offline --bin validate_evidence
cargo build --offline --release --bin validate_evidence
```

**73 testes passaram**. O novo teste verifica que uma nova exportação refaz a
conferência e rejeita uma ligação modificada, e que uma segunda fonte com
ligação inválida não aproveita a conferência da primeira.

Executamos novamente as cargas da Aula 18 com BIBLIOTECARIO_METRICS=1. Em todas
elas observamos **2 capture_link, 2 capture_validate, 12 digest_calls e 41.880
bytes instrumentados**. A previsão foi conferida por assertions. Os hashes das
três exportações permaneceram iguais aos da Aula 18. Com 100 fichas, eram 101
conferências e 2.088.210 bytes na Aula 19.

Os [contadores e comandos](../experimentos/11-reuso-conferencia/contagens.json)
estão preservados. As contagens descrevem leituras lógicas, não tráfego físico
de disco. Não medimos alocações nem o tempo isolado dos hashes.

## Medição de tempo

Antes de recompilar, preservamos o binário release da Aula 19 em
`/tmp/validate-evidence-aula19`. Medimos esse binário e o novo na mesma sessão,
sequencialmente, com o medidor da Aula 18: três aquecimentos e 15 amostras por
carga. A variável de métricas não foi ativada nos ensaios de tempo.

| Fichas | Mediana anterior (ms) | Mediana nova (ms) |
| --- | --- | --- |
| 1 | 1,569 | 1,592 |
| 10 | 3,619 | 2,044 |
| 100 | 26,513 | 5,997 |

Amostras e comandos estão em [antes/results.json](../experimentos/11-reuso-conferencia/antes/results.json)
e [depois/results.json](../experimentos/11-reuso-conferencia/depois/results.json).
As execuções foram capturadas como RUN_BENCH_BEFORE_20_1 e RUN_BENCH_AFTER_20_1,
com código 0. Exportações das duas versões tiveram hashes iguais. O binário
anterior em /tmp é temporário, não um artefato versionado; seus hashes ficaram
registrados. Os destinos do ensaio já existem; novas execuções exigem destinos novos.

## Explicação e fechamento

Observamos redução do tempo mediano com 10 e 100 fichas neste ensaio. Para uma
ficha, não observamos redução. As contagens mostram objetivamente menos leituras;
a comparação de tempo não isola sua contribuição de todos os demais efeitos.
Não houve randomização entre versões, isolamento da máquina ou análise de
significância. O ganho observado não é uma garantia para outras cargas ou ambientes.

A **Aula 20 está concluída**: escopo definido, implementação, testes, contagens
e comparação local de tempo. Não reescrevemos capturas históricas. As notas das
aulas 10 e 17 permanecem 3/6; o envio ao Ollama continua manual, sem comprovação
retroativa de RUN_VECTOR_1.

Na **Aula 21 — Testar o reaproveitamento com várias fontes**, propomos ampliar
as cargas para distinguir muitas fichas da mesma fonte de muitas fontes, testar
limites e verificar se o resultado se sustenta antes de novas otimizações.
