# Previsões anteriores às contagens — Aula 19

Nas cargas da Aula 18 há uma fonte, com captura, e N fichas selecionadas.
Pelo fluxo da implementação, prevemos:

- capture_link: N + 1 (uma validação inicial e uma por ficha).
- capture_validate: N + 1.
- record_link_read e record_validate_read: N + 1 leituras completas cada.
- source_content_read: N + 2 (validação inicial da fonte, mais uma por ligação).
- digest_calls: 6 × (N + 1), pois a captura tem uma saída e cinco fontes.

Bytes são os entregues à aplicação, não bytes físicos transferidos pelo disco.
Contaremos somente os pontos instrumentados da conferência; leitura do dossiê,
pergunta/parecer, metadados do filesystem, JSON e escrita ficam fora dessa conta.
Cada categoria de leitura conta novamente bytes relidos. digest_chunks conta
blocos retornados, não chamadas completas de hash.

Execuções separadas com BIBLIOTECARIO_METRICS=1, binário release, cargas de
1, 10 e 100 fichas já preservadas. Conferir que a exportação tem o mesmo hash
obtido na Aula 18. Sem medição de tempo nem otimização nesta etapa.
