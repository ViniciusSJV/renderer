# Aula 22 — Previsões antes da conferência

A refatoração separa a conferência do documento de captura e a ligação da fonte
à saída. Não implementa compartilhamento entre fontes nesta aula.

Nas cargas válidas da Aula 21, esperamos os mesmos hashes de exportação e as
mesmas contagens: para 1, 10 e 100 fontes, 2, 20 e 200 capture_validate e
41.880, 418.800 e 4.188.000 bytes instrumentados. A carga com ID de execução
incorreto deve continuar recusada sem exportação.

Não medir tempo nem afirmar ganho. A ordem de diagnóstico pode mudar quando
há mais de uma inconsistência: a conferência completa da captura passa a
preceder a comparação de sua saída com a fonte documental.
