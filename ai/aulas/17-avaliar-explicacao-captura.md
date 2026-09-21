# Aula 17 — Avaliar a explicação da captura

## Conceito e preparação

A conferência local agora acompanha as fichas exportadas. Vamos observar se o
Qwen explica o resultado sem confundir fonte, execução, conferência e autenticação.
Os critérios foram escritos antes de gerar a consulta e antes de receber resposta.

Preparamos a [pergunta](../experimentos/08-avaliacao-captura/pergunta.txt) e
[seis critérios](../experimentos/08-avaliacao-captura/criterios.json): identidade,
resultado/pânico esperado, referências, alcance da conferência, autenticidade e
limites de correção/desempenho. Cada critério integralmente atendido vale um
ponto; parcial vale zero. Contradições na resposta inteira contam na análise.

## Geração e conferência

```bash
cargo run --offline --bin validate_evidence -- ai/experimentos/07-dossie-captura/evidencias.json --fact F_BOUNDARY_RUN_RESULT --fact F_BOUNDARY_LIMIT_PANIC --context 1 --question ai/experimentos/08-avaliacao-captura/pergunta.txt --output ai/experimentos/08-avaliacao-captura/consulta.json
```

O comando terminou com código 0, **1 fonte e 2 fichas, zero referências inválidas**.
A consulta foi interpretada como JSON e seu SHA-256 foi registrado na rubrica.
O destino já existe; use outro caminho para nova exportação. Não alteramos Rust,
não repetimos testes nem fizemos benchmark. Não medimos tokens ou latência.

## Passo manual

1. No Windows, inicie uma conversa nova com `ollama run renderer-analyst`.
2. Envie o conteúdo completo de [consulta.json](../experimentos/08-avaliacao-captura/consulta.json).
   Não envie os critérios, que contêm o gabarito.
3. Preserve a primeira resposta completa, sem pedir correção antes da avaliação.
4. Traga a resposta para esta conversa. Informe se houve corte, erro ou mudança
   na configuração do modelo.

O modelo não recebe o arquivo por conhecer seu caminho no Codespaces. É preciso
transferir o conteúdo manualmente. O contexto e a saída configurados anteriormente
não garantem recebimento integral; não confirmamos a configuração efetiva atual.

## Resposta e avaliação

Recebemos manualmente a [resposta completa](../experimentos/08-avaliacao-captura/resposta.txt).
Preservamos o texto recebido com quebras de linha e escapes Markdown; seu hash
identifica o arquivo salvo, não os bytes de transporte da sessão. O hash da
consulta preparada foi reconferido e correspondeu ao registrado.

A [rubrica preenchida](../experimentos/08-avaliacao-captura/criterios.json)
resultou em **3/6**, aplicando um ponto por critério integralmente atendido:

| Critério | Pontos | Motivo |
| --- | --- | --- |
| C1 — Identidades | 1 | Distingue fonte e execução. |
| C2 — Resultado e pânico | 0 | Nega que o relatório permita confirmar o pânico esperado relatado. |
| C3 — Referências | 0 | Omite os dois IDs de fichas exigidos. |
| C4 — Conferência versus comando | 1 | Enumera conferências e separa o resultado do comando da consistência do registro. |
| C5 — Associação e limites | 0 | Parcial: acerta hashes e limites de autenticação, mas omite a ausência de validação semântica da ficha. |
| C6 — Generalização | 1 | Não conclui correção universal nem ganho de desempenho. |

O ponto central é separar duas frases: o relatório registra aprovação com
pânico esperado; o Bibliotecário não autentica que a execução ocorreu. A segunda
não invalida a leitura da primeira. `should panic ... ok` não significa que o
par foi aceito pela macro.

No item 3, “não fornece informações sobre a execução ou a compilação” é amplo
demais: existem informações declaradas, sem a comprovação pretendida. No item 4,
“não fornece ... resultados esperados” ignora o pânico esperado apresentado.
Essas nuances estão registradas na avaliação; a nota não pretende resumir toda
a qualidade da resposta. Não alteramos os critérios depois de recebê-la.

## Fechamento e próxima aula

**A Aula 17 está concluída:** pergunta e critérios prévios, consulta conferida,
resposta preservada e avaliação manual com justificativas. Não há confirmação
de configuração efetiva nem ausência de truncamento de contexto. Não houve
benchmark, alteração de Rust ou nova execução de testes nesta avaliação.

A nota da Aula 10 permanece **3/6**. Igualdade numérica não indica desempenho
igual: tarefas e rubricas diferem, sem comparação controlada.

Na **Aula 18 — Medir o custo da conferência**, propomos definir cargas e métricas
para medir o Bibliotecário antes de otimizar: tempo, volume de dados e efeito de
repetir fichas ligadas à mesma captura. A avaliação identificou limites do modelo;
nenhum ganho de performance foi demonstrado. O envio ao Ollama continua manual,
e a criação de cenas por linguagem natural permanece posterior a essa sequência.
