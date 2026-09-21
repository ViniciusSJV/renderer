# Aula 23 — Reaproveitar a captura entre fontes

## Conceito

Reutilizamos a conferência do documento de captura durante uma exportação,
mantendo as verificações de cada fonte. A validação inicial da CLI permanece
separada. Portanto, S fontes da mesma captura ainda provocam S conferências
iniciais, mas somente uma conferência completa na montagem da exportação.

## Implementação

Em validate_evidence.rs, `CaptureCache` guarda `CheckedCapture`, usando a chave
(caminho absoluto do registro, SHA-256 esperado, run_id esperado). Não resolvemos
links simbólicos para compor a chave: a localização declarada determina onde
procurar saida.bin. Não unificamos caminhos distintos com `..` ou aliases de
symlink; perder reutilização é preferível a presumir equivalência.

`validate_capture_link_cached` sempre confere tipo, arquivo, hash e linhas da
fonte e sua ligação à saída. Só a conferência do documento é reaproveitada.
A entrada é inserida após `check_capture_document` ter sucesso. Uma falha de
ligação posterior não aprova a fonte: interrompe a exportação. O registro
conferido não significa que todas as fontes que o referenciam sejam válidas.

O mapa de documentos e o mapa por ID de fonte ficam locais a `selections_json`.
O wrapper usado pela validação inicial cria um mapa novo por chamada. Não há
cache global, entre exportações ou processos. Cada nova exportação refaz as
leituras. O formato JSON permanece igual.

Essa política reutiliza uma observação anterior: não garante detectar mudança
posterior dos arquivos durante a mesma exportação. Não existe snapshot atômico,
autenticação nem prova das entradas do compilador. A chave identifica o registro
esperado; não transforma arquivos mutáveis em conteúdo imutável.

## Testes e previsões

```bash
cargo test --offline --bin validate_evidence
cargo build --offline --release --bin validate_evidence
```

**76 testes passaram**. Dois testes novos exercitam o cache preenchido:

- Uma segunda fonte válida compartilha a entrada; linhas adulteradas e arquivo
  de saída errado continuam recusados.
- Hash, run_id ou localização diferentes exigem conferência e falham quando
  inválidos, sem acrescentar entradas válidas ao mapa.

O teste anterior de isolamento entre exportações e fontes também passou.
As [previsões](../experimentos/14-captura-compartilhada/previsoes.md) foram
registradas antes das execuções de contagem.

## Medição

Repetimos as cargas de 100 fichas da Aula 22. Cada fonte ainda aponta para os
mesmos arquivos físicos; as cargas variam identidades documentais.

| Fontes | Conferências completas antes | Agora | Bytes instrumentados antes | Agora |
| --- | --- | --- | --- | --- |
| 1 | 2 | 2 | 41.880 | 41.880 |
| 10 | 20 | 11 | 418.800 | 237.630 |
| 100 | 200 | 101 | 4.188.000 | 2.195.130 |

Todas as previsões corresponderam: capture_validate e ambas as leituras do
registro ocorrem S + 1 vezes; digest_calls, 6(S + 1). capture_link continua em
2S e source_content_read em 3S. As verificações próprias das fontes não foram
eliminadas. Os bytes somam 1.620S + 20.130(S + 1) nestas cargas específicas.

As três exportações foram comparadas byte a byte com as da Aula 22 e são
idênticas. A carga com ID de execução incorreto continuou recusada, com código 1
e sem exportação. Nesse caso, a recusa ocorre na fase inicial da CLI; os testes
unitários adicionais exercitam diretamente o reaproveitamento com mapa preenchido.

[Resultados completos](../experimentos/14-captura-compartilhada/resultados.json),
exportações e capturas RUN_SHARED_CAPTURE_1_1, RUN_SHARED_CAPTURE_10_1,
RUN_SHARED_CAPTURE_100_1 e RUN_SHARED_CAPTURE_invalido_1 estão preservados na
mesma pasta. Os registros contêm comandos exatos, ambiente parcial e hashes.
Para repetir, use destinos novos e mantenha seus diretórios pais existentes.

## Explicação e fechamento

A **Aula 23 está concluída**: reaproveitamento local, identidade composta,
verificações de fonte preservadas, testes e contagens. Reduzimos trabalho lógico
observado nas cargas com várias fontes. Não medimos tempo nesta aula, portanto
não demonstramos um novo ganho de latência. Não fizemos benchmark do renderer.

A validação inicial ainda repete capturas entre fontes. Isso é um limite
conhecido, não um defeito que obrigue a continuar otimizando antes da integração.
Registros históricos permanecem intactos. As notas das aulas 10 e 17 continuam
3/6, e não há comprovação retroativa de RUN_VECTOR_1.

Na **Aula 24 — Contrato e critérios de conclusão dos engines**, vamos consolidar
o que o Graph Engine já entrega, listar lacunas e definir entradas, saídas e
falhas da integração com o LLM Engine. Depois implementaremos a comunicação
Rust–Ollama. O modelo atual permanece Qwen; uma futura troca por DeepSeek será
avaliada. O envio continua manual até implementar e validar essa integração.
