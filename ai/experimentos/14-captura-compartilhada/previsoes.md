# Aula 23 — Previsões antes das medições

Reutilização somente na montagem da exportação, por caminho absoluto não
canonicalizado do registro, hash esperado e run_id esperado. A validação
inicial permanece separada e não compartilha resultados entre fontes.

Nas cargas anteriores, com S fontes e uma captura, prever:
- capture_link: 2S; source_content_read: 3S (não reduzir verificações de fonte).
- capture_validate, record_link_read e record_validate_read: S + 1.
- digest_calls: 6(S + 1).
- bytes instrumentados: 1.620S + 20.130(S + 1).
- Exportações idênticas às da Aula 22; carga de run_id inválido recusada.

Não medir tempo nesta etapa. Testar que linhas/saída de uma nova fonte continuam
conferidas após preencher o cache e que hash, ID ou caminho diferentes não
reutilizam o resultado anterior. Erros não devem criar entradas válidas.
Novas exportações devem reconstruir o mapa. Não prometer snapshot ou autenticação.
