# Avaliação retrospectiva — MANUAL_001_OLLAMA_01

Avaliada em 24/09/2026 a partir dos originais em `ollama-01/`, do envelope
`query-ollama.json` e da edição preservada. Nenhuma nova geração LLM foi feita.
Os critérios abaixo foram aplicados depois da resposta: não são uma rubrica
pré-registrada e não demonstram confiabilidade geral do modelo.

## Resultado

**Transporte concluído e artefatos consistentes; explicação parcialmente correta,
com correção necessária na descrição da direção.** A localização responde à pergunta.
Não aceitar automaticamente a explicação inteira como fato do acervo.

| Critério | Avaliação | Evidência e implicação |
| --- | --- | --- |
| Localizar e citar | Aprovado | Cita `ray_from_pixel`, `Camera`, `src/camera.rs:75–89` e E1, compatíveis com a seleção enviada. |
| Descrever a operação | Parcial | “subtração do ponto de origem ... pelo ponto ... correspondente ao pixel” é ambíguo e pode ser lido como `origin - pixel`. O código é explicitamente `(pixel - origin).normalize()`. |
| Distinguir observação e inferência | Parcial | Subtração e normalização são operações observáveis no trecho, embora colocadas em INFERENCE. A interpretação geométrica deve separar-se dessa observação. |
| Hipótese pertinente e testável | Parcial | Rotula performance como hipótese e não inventa ganho. “Transformações simples vs. complexas” não define protocolo e E1 não fornece o corpo de `inverse()` para justificar diferença de custo. A pergunta não exigia hipótese de performance. |
| Não inventar execução | Aprovado | Não afirma que executou testes ou benchmarks. A lacuna deve ser formulada como “não foram fornecidas medições neste contexto”; não demonstra ausência no projeto. |

Resposta corrigida, redigida nesta avaliação (não gerada novamente pelo modelo):

> A direção é calculada em `Camera::ray_from_pixel`, no arquivo `src/camera.rs`,
> linhas 75–89 (E1). O método aplica a transformação inversa da câmera ao ponto
> correspondente ao pixel e ao ponto `(0, 0, 0)`, obtendo `pixel` e `origin`.
> Em seguida calcula `direction = (pixel - origin).normalize()` e retorna
> `Ray::new(origin, direction)`. Subtrai a origem do ponto do pixel, nessa ordem.
> O trecho fornecido não contém medições que sustentem conclusões sobre desempenho.

## Conferência dos artefatos

Foram conferidos SHA-256 de query, request, retorno bruto e texto contra
`ollama-01/result.json`, e os tamanhos de query/request/retorno. O prompt da
requisição é idêntico à query preservada; o texto é idêntico ao campo `response`
do retorno bruto. Os hashes de `search.json` e do manifesto correspondem aos
registrados no envelope. `Catalog::load`/`verify` reconferiu a edição: 63 fontes,
908 símbolos e zero diagnósticos. Esses checks não autenticam a máquina/execução.

| Campo | Registro observado |
| --- | --- |
| Modelo solicitado/retornado | `renderer-analyst:latest` |
| HTTP / estado | 200 / `completed` |
| Conclusão do provedor | `done=true`, `done_reason=stop` |
| Tempo do cliente HTTP | 12.223,1222 ms (12,223 s) |
| Query / request / retorno | 2582 / 2800 / 10448 bytes |
| `prompt_eval_count` / `eval_count` | 1637 / 322, segundo o provedor |
| Tempo total / carregamento / geração do provedor | 11,9045028 / 5,9191461 / 5,284452 s |

Tempo HTTP não é benchmark do ray tracer nem tempo puro de geração. Contagens de
tokens e `done` não provam semanticamente o processamento integral das evidências.
`explicit_options={}` significa que o cliente não enviou overrides; não comprova
parâmetros efetivos nem o digest do modelo instalado. O Modelfile local não foi
preservado como configuração efetiva do servidor nesta tentativa.

`evidence_rechecked=false` é correto: o transporte não reconfere fontes. Houve
conferência na preparação e outra nesta avaliação, etapas separadas do envio.
`dossier_origin=null` e `selection=null` indicam ausência do vínculo automatizado
de dossiê/seleção no registro do cliente. A seleção manual está descrita dentro da
query (E1 e hashes); não é um bundle `PreparedQuery`.

`semantic_evaluation=pending` permanece no `result.json` original. Este documento
é a avaliação posterior vinculada pelo ID e hash da resposta; não reescreve o
registro de transporte para aparentar avaliação anterior.

- SHA-256 da query: `53f1322f2dd3b4800eadec2903dcf8b1a42a51d6ec875a0e92cba6685341ed99`.
- SHA-256 do texto avaliado: `04dbaf9b0899ed33c0f1b7db7eb4735482934432349744875679cbef645a2052`.
- SHA-256 do retorno bruto: `7336d64872e5fac9ba2f9822310f8d587842adf126ffd47ecc6a9b3086bc8d44`.

Próxima tentativa: definir critérios antes da geração; solicitar fórmula explícita
e resposta focada na pergunta. Manter qualquer nova resposta e avaliação em diretório
próprio, sem substituir esta tentativa nem promover hipóteses a fatos.
