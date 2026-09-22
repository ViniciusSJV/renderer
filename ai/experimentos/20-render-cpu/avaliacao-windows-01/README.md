# Avaliação da primeira consulta de performance no Windows

**Transporte concluído; resposta reprovada na rubrica.** São zero critérios
plenamente atendidos, três parciais e três reprovados. Isso não invalida a
medição do renderer: identifica falhas da explicação produzida pelo modelo.

## Integridade

Arquivo recebido: `ai/experimentos/resultado-render-cpu.zip`.
Tentativa: `RENDER_CPU_a06382b3472e484fa76066c72a779354`.

`conferir.py` executou 37 verificações: hashes da consulta, requisição, resposta,
texto, origem, rubrica e saída do validador; identidade/seleção; igualdade do
prompt com a consulta; igualdade da resposta extraída; correspondência do
modelo; comparação do dossiê e da rubrica com os arquivos locais e conferência
das oito fontes locais. A rubrica esperada não aparece no prompt.

Os registros são internamente consistentes: validador com exit code 0,
HTTP 200, resposta concluída, aproximadamente 20,48 s de transporte.
Isso não autentica o computador remoto nem a coleta histórica do benchmark.
O ZIP original é preservado, sem extrair ou executar conteúdo dele.

## Julgamento pela rubrica prévia

| Critério | Resultado | Evidência e problema |
| --- | --- | --- |
| C1 — Rastreabilidade | Reprovado | Inventou `F_TIME_REDUCTION` e atribuiu ganho de performance a `SRC_TEST`. |
| C2 — Mecanismo | Reprovado | Menciona remoção de cópias, mas não explica `&self` e reutilização do mundo. |
| C3 — Medição | Reprovado | Diz que o cronômetro mede o teste de regressão; omite os números e o protocolo fornecidos. |
| C4 — Correção/proveniência | Parcial | Reconhece digest quantizado e cena específica; confunde aprovação visual com ganho de tempo. |
| C5 — Limites | Parcial | Reconhece câmera dentro das esferas e variabilidade; omite tamanho e ausência de perfil CPU. |
| C6 — Próximo experimento | Parcial | Pede baseline e configuração fixa, mas não define regressão visual nem cena externa maior/perfil CPU. |

`avaliacao.json` preserva trechos literais e justificativas por critério, com
hashes da resposta, rubrica e ZIP. Trata-se de avaliação pelo assistente,
separada das verificações determinísticas de `conferencia.json`.
Não alteramos os registros originais que indicavam avaliação pendente.

## O que as fontes realmente sustentam

- F_MEDIANS/SRC_STATS: 35 amostras por versão, medianas de 4.876.415 e
  4.132.936 ns, redução observada de 15,25% na cena medida.
- F_TIMER/SRC_CAMERA: cronômetro em torno de `Camera::render`; clone preparatório
  e descarte do canvas fora da medição. Não mede o teste de regressão.
- F_REGRESSION_LOG/SRC_TEST: aprovação de um teste, sem evidência de velocidade.
- F_BORROW e F_RENDER: remoção de clones de World no caminho de render/shading,
  usando empréstimos. Não elimina todas as alocações do renderer.

## Hipótese a investigar antes de outra consulta

A consulta preservada tem 36.889 bytes. O servidor registrou
`prompt_eval_count=2050`, `eval_count=1024` e `done_reason=stop`.
O Modelfile do repositório declara `num_ctx 4096` e `num_predict 1024`;
isso não comprova a configuração do modelo instalado no Windows. Não é possível
converter bytes em tokens ou concluir truncamento apenas desses números.

Uma limitação de contexto/saída é hipótese, não causa demonstrada dos erros.
Para registrar a configuração do modelo instalado, no Windows:

```powershell
ollama show renderer-analyst:latest --modelfile
ollama show renderer-analyst:latest --parameters
```

Próximo experimento proposto: uma consulta menor, selecionando explicitamente
F_TIMER, F_PROTOCOL e F_MEDIANS, para perguntar apenas o escopo e o resultado da
medição. Definir a rubrica antes e preservar a primeira tentativa reprovada.
Não mudar simultaneamente modelo, contexto e pergunta e atribuir a melhora a
uma única causa. Isso testa a capacidade de usar evidência temporal antes de
solicitar uma análise ampla ou outra otimização.

## Reproduzir a conferência local

Na raiz do repositório:

```bash
python3 ai/experimentos/20-render-cpu/avaliacao-windows-01/conferir.py
```

O script exige o ZIP original e os arquivos das fontes sem alterações.
Alterações futuras nessas fontes fazem a comparação local falhar; não devem
ser confundidas com corrupção do ZIP histórico.
