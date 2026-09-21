# Aula 22 — Separar fonte documental de captura compartilhada

## Conceito

Continuamos do fim da Aula 21, sem pular a investigação das responsabilidades.
Duas fontes podem apontar para a mesma captura, mas cada fonte ainda precisa
corresponder à saída e às linhas que declara. Uma captura consistente não
aprova automaticamente uma fonte que a referencia.

| Responsabilidade | Conferências |
| --- | --- |
| Fonte documental | Tipo test_run, ausência de execution legado simultâneo, arquivo atual, hash e linhas copiados. |
| Ligação fonte–captura | Caminho da fonte corresponde à saída da captura; hash declarado coincide com o hash da saída. |
| Documento de captura | Bytes do registro correspondem ao hash esperado; run_id corresponde à ligação; formato, resultado, saída e associações passam na conferência da Aula 15. |

## Implementação

Refatoramos validate_evidence.rs, sem criar cache por captura:

- `check_capture_document` confere o registro e retorna `CheckedCapture`, com
  conteúdo do registro, relatório e caminho da saída.
- `check_source_capture_output` confere a ligação de uma fonte à saída desse
  documento conferido.
- `validate_capture_link` coordena essas responsabilidades e mantém o formato
  exportado. A conferência do arquivo/hash/linhas da fonte continua obrigatória.

O reaproveitamento por ID de fonte, limitado a selections_json, permanece igual.
A conferência completa do documento agora precede a comparação do caminho/hash
com a fonte. Com várias inconsistências simultâneas, a ordem do diagnóstico pode
mudar. Ainda há duas leituras do registro; não eliminamos essa repetição aqui.

## Teste e medição

As [previsões](../experimentos/13-separar-responsabilidades/previsoes.md) foram
registradas antes das execuções de comparação.

```bash
cargo test --offline --bin validate_evidence
cargo build --offline --release --bin validate_evidence
```

**74 testes passaram**, incluindo um novo teste: mesmo com CheckedCapture válido,
um caminho de fonte diferente ou hash incorreto é recusado.

Repetimos as cargas da Aula 21, com 100 fichas:

| Fontes | Conferências de captura | Bytes instrumentados | Exportação |
| --- | --- | --- | --- |
| 1 | 2 | 41.880 | Idêntica à Aula 21 |
| 10 | 20 | 418.800 | Idêntica à Aula 21 |
| 100 | 200 | 4.188.000 | Idêntica à Aula 21 |

A carga de ID incorreto retornou 1 e não criou exportação. Os resultados estão
em [resultados.json](../experimentos/13-separar-responsabilidades/resultados.json).
As capturas RUN_SEPARATE_1_1, RUN_SEPARATE_10_1, RUN_SEPARATE_100_1 e
RUN_SEPARATE_invalido_1 preservam os comandos completos, saída e arquivos
selecionados. Os destinos já existem; novas execuções exigem destinos novos.
Não medimos tempo nem demonstramos ganho nesta refatoração.

## Decisão para o próximo passo

Vale experimentar reutilizar a parte CheckedCapture entre fontes durante uma
mesma exportação, mantendo as verificações próprias de cada fonte. A identidade
proposta inclui **caminho absoluto do registro, SHA-256 esperado e run_id esperado**.
O caminho deve preservar a localização usada para resolver saida.bin; canonicalizar
um link simbólico do registro sem considerar essa base pode mudar seu significado.
Uma opção conservadora é não unificar aliases de caminhos, aceitando perder
reutilização em vez de presumir equivalência.

Caminho sozinho não distingue versões. Hash sozinho não distingue o diretório
em que as referências da saída são resolvidas. ID sozinho é uma declaração,
sem garantia de unicidade global. A chave combinada também não autentica nada.

Na futura reutilização, uma entrada só poderá ser guardada após conferência
bem-sucedida. A fonte deverá continuar tendo seu arquivo/hash/linhas e ligação
conferidos; não basta encontrar a captura no mapa. Novas exportações deverão
refazer as leituras. Falhas não serão transformadas em resultados válidos.

Arquivos podem mudar após a primeira leitura. Reutilização local não oferece
snapshot atômico nem detecção de toda alteração durante o intervalo. Esses
limites já existem no reaproveitamento por fonte e precisam permanecer explícitos.

## Fechamento e próxima aula

A **Aula 22 está concluída**: responsabilidades separadas, refatoração testada,
exportações preservadas, contagens mantidas e identidade proposta para o próximo
experimento. Ainda não implementamos o compartilhamento por captura.

Na **Aula 23 — Reaproveitar a captura entre fontes**, vamos implementar esse
escopo local, testar diferenças de identidade e fontes inválidas e repetir
contagens antes de concluir qualquer ganho. Continuamos fechando o Graph Engine
e depois a integração do LLM Engine via Ollama, sem pular etapas. Qwen continua
atual; DeepSeek é uma possibilidade futura sujeita a avaliação, sem troca feita.
As notas das aulas 10 e 17 permanecem 3/6, sem comprovação retroativa de RUN_VECTOR_1.
