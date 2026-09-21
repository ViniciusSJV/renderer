# Protocolo prévio — Aula 20

Reutilizar a conferência por ID de fonte somente dentro de selections_json.
IDs de fontes são validados como únicos antes do reaproveitamento. Preservar
uma conferência inicial da CLI, separada da montagem da exportação. Previsão:
para as cargas de uma fonte da Aula 18, duas conferências independentemente de N.
Não manter cache global, entre exportações ou entre comandos. Não prometer
snapshot atômico nem detecção de alterações posteriores à leitura reutilizada.

Conferir testes e hashes das exportações. Repetir as três cargas com métricas
ativadas; prever 2 capture_link, 2 capture_validate, 12 digest_calls e 41.880
bytes instrumentados por carga. Depois medir sem métricas, usando o medidor
inalterado da Aula 18 (3 aquecimentos e 15 amostras por carga).

Preservar o binário release anterior em /tmp antes de recompilar e medir anterior
e novo nesta sessão, sequencialmente. Não há randomização entre versões nem
isolamento do ambiente: tratar diferenças de tempo como observações locais,
sem atribuir causalidade exclusiva. Não comparar apenas sessões históricas.
