# Aula 21 — Previsões antes da execução

Manter 100 fichas sintéticas e distribuir igualmente entre S = 1, 10 e 100
fontes com IDs únicos, copiadas da fonte da Aula 16. Todas apontam para os
mesmos arquivos e captura; isso testa separação por identidade documental,
não escala de arquivos físicos distintos ou execuções independentes.

Previsões para a CLI completa:
- capture_link = capture_validate = 2S, somando validação inicial e exportação.
- source_content_read = 3S.
- record_link_read = record_validate_read = 2S.
- digest_calls = 12S.
- bytes instrumentados = 41.880S, pois as fontes repetem os mesmos arquivos.

Conferir 100 seleções, vínculos por ID e status de captura, sem cronometragem.
Para S=1, exigir hash igual à exportação de 100 fichas da Aula 20. Para S=10
comparar hashes com métricas ligadas e desligadas. Criar separadamente uma carga
inválida cuja última fonte tem run_id incorreto: deve falhar sem criar exportação.
Não modificar registros históricos, implementar otimização ou inferir desempenho.
