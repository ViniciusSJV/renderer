# Avaliação da segunda consulta de performance

**Quatro critérios aprovados e um parcial; nenhum reprovado.** A resposta
interpreta corretamente a medição. A ressalva está na descrição dos limites
da evidência. A avaliação global é parcial, não aprovação integral.

## Integridade e execução registrada

ZIP recebido: `ai/experimentos/RENDER_CPU_V2_29ef6dfc096641cea7d45e0926d9bd0d.zip`.
`conferir.py` executado: 37 verificações passaram, incluindo hashes, vínculos,
rubrica prévia, igualdade do prompt, resposta extraída e oito fontes locais.
Nenhum ID F_* inexistente apareceu na resposta. O ZIP original foi preservado;
nenhuma instrução ou código contido nele foi executado.

Os registros informam HTTP 200, estado completed e 12.289,124 ms de transporte.
A consulta contém 9.077 bytes; o servidor registra prompt_eval_count=3274,
eval_count=508 e done_reason=stop. Integridade interna não autentica a máquina
remota, a execução histórica do benchmark ou a correção semântica da resposta.

## Rubrica prévia RUBRIC_RENDER_MEASUREMENT_V2

| Critério | Resultado | Justificativa |
| --- | --- | --- |
| C1 — Referências | Aprovado | F_TIMER/SRC_CAMERA, F_PROTOCOL/SRC_SAMPLES e F_MEDIANS/SRC_STATS corretos. |
| C2 — Cronômetro | Aprovado | Identifica render e exclui clone preparatório e descarte do canvas. |
| C3 — Protocolo | Aprovado | 64x64, release, quatro threads, três warmups, cinco amostras por execução. |
| C4 — Resultado | Aprovado | 4.876.415 e 4.132.936 ns, 35 amostras por versão, redução de 15,25%. |
| C5 — Limites | Parcial | Distingue registros de autenticação, mas nega genericamente informações sobre execução/validação e introduz “versão 2 do código”. |

Formulação mais precisa para C5: existem registros históricos e recálculo
aritmético, mas não uma captura que autentique a execução do benchmark; a
segunda consulta não realizou nova medição nem criou nova versão do renderer.

Observação fora dos cinco critérios: a resposta tem 241 palavras na contagem
por espaços, acima das 200 solicitadas. Não alteramos retroativamente a rubrica
para criar um critério novo de aprovação.

`avaliacao.json` contém os trechos literais, justificativas, hashes e julgamento
pelo assistente. `conferencia.json` contém os resultados determinísticos.
`resposta.txt` preserva os bytes do texto retornado. Os registros originais
continuam com o estado de avaliação que tinham no momento da execução.

## Comparação e consequência para o Bibliotecário

Na primeira consulta o modelo inventou F_TIME_REDUCTION e confundiu teste de
regressão com benchmark. Nesta consulta menor, esses erros não apareceram.
Isso demonstra uma resposta melhor para a tarefa mais restrita; não comprova
truncamento anterior nem uma melhora geral do modelo. Seleção, janela, pergunta
e rubrica mudaram, portanto as contagens não são uma comparação controlada.

Temos agora um ciclo real com resultado examinado: seleção de evidências,
conferência pelo Graph Engine, resposta preservada pelo LLM Engine e avaliação
separada. O Bibliotecário deve conservar tanto o caso reprovado quanto o parcial
como exemplos de avaliação, sem promover resposta a fato automaticamente.

Próximo passo proposto: usar perguntas menores e critérios prévios nos próximos
experimentos; ampliar a cena de benchmark em CPU e manter medição e regressão
visual como fontes distintas. A extração do Bibliotecário ainda depende de
isolar as dependências de domínio e provar um segundo fluxo completo.

## Reproduzir a conferência

Na raiz do repositório:

```bash
python3 ai/experimentos/20-render-cpu/avaliacao-windows-02/conferir.py
```

Requer o ZIP recebido e as fontes locais da versão avaliada. Se as fontes
mudarem, a comparação local falhará; isso não implica corrupção do ZIP antigo.
