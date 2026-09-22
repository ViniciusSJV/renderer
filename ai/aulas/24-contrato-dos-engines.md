# Aula 24 — Contrato e critérios de conclusão dos engines

## Objetivo

Definir responsabilidades e critérios de aceitação para o ciclo de explicação,
separando evidência conferida, resposta recebida e resposta avaliada.

## Contexto e pré-requisitos

Leia as aulas 1–23 e use o editor para consultar código, Modelfile e
[contrato dos engines](../contratos/engines-v1.md). Não há instalação ou execução
nova necessária nesta aula. O contrato contém a matriz original da Aula 24 e
atualizações posteriores; não confunda as duas situações.

## Conceitos: três resultados distintos

```text
Dossiê → conferência → consulta → Ollama/modelo → resposta preservada → avaliação
```

O **Graph Engine — “Testa sem explicar”** organiza fontes, fichas, contexto,
execuções e relações, aplicando verificações determinísticas. Uma referência
válida não comprova a afirmação da ficha.

O **LLM Engine — “Explica sem interpretar”** recebe evidências e produz uma
explicação que deve distinguir fato, inferência e hipótese. O transporte preserva
a resposta, mas não a transforma em evidência de correção semântica.
A avaliação manual aplica critérios definidos antes da resposta e examina
referências, premissas e contradições.

## Implementação documental

O contrato define uma consulta com `question`, `evidence` e `instructions` e
um registro de tentativa separado. A tentativa deve ligar identidade, hashes,
origem do dossiê, seleção, endpoint, modelo, limites, horários e resposta.
O modelo é configurável; Qwen é o usado nos registros. DeepSeek permanece
hipótese de avaliação futura, sem superioridade demonstrada ou troca realizada.

A topologia precisa ser explícita: cliente e Ollama no mesmo Windows podem
usar o endereço de loopback. `localhost` de um ambiente remoto identifica esse
ambiente, não o Windows. O contrato não estabelece uma ponte de rede automática.

## Passo a passo: revisar o contrato

No editor, percorra o contrato e associe cada regra ao código ou ao teste que
já a sustenta. Essa inspeção não modifica arquivos nem chama o modelo:

1. Confira a seleção em `query_json` e a distinção entre consulta e instruções.
2. Relacione as verificações das aulas 3–23 às fontes e aos testes existentes.
3. Examine a matriz original e identifique o que ainda exigia um cliente HTTP.
4. Separe uma exigência documentada de uma implementação e de uma execução observada.

As quatro situações centrais são: evidência inválida bloqueia envio; falha de
comunicação não vira resposta avaliada; resposta recebida pode estar incompleta
ou errada; status numa consulta antiga não equivale a reconferência atual.
Não há execução automática de sugestões do modelo.

## Critérios de aceitação na etapa original

| ID | Exigência | Estado ao final da Aula 24 |
| --- | --- | --- |
| G1 | Recusar referências, hashes e linhas inválidas. | Implementado. |
| G2 | Conferir captura versão 2 e ligação ao dossiê. | Implementado. |
| G3 | Preservar IDs, contexto e limites, com reaproveitamento local. | Implementado. |
| G4 | Conferir antes de enviar e registrar origem exata. | Pendente. |
| L1 | Cliente Rust com endpoint e modelo configuráveis. | Pendente. |
| L2 | Ligar entrada, resposta e configuração por identidade/hash. | Parcial. |
| L3 | Testar falhas, timeout, limites e gravação. | Pendente. |
| L4 | Configurar modelo sem alterar Graph Engine. | Pendente no cliente. |
| I1 | Testar envio e falhas com servidor simulado. | Pendente. |
| I2 | Executar ciclo real e avaliar com rubrica prévia. | Pendente. |
| I3 | Registrar latência e tamanhos com escopo. | Pendente no cliente. |
| I4 | Documentar reprodução, configuração e limites. | Parcial. |

São **12 critérios: 3 implementados, 2 parciais e 7 pendentes**. A contagem não
representa porcentagem de esforço. G1–G3 se apoiavam nos testes até a Aula 23;
esta aula de contrato não executou novos testes, benchmark ou geração.

## Validação e limites

Fechar os engines exige evidência para cada critério e registro dos limites
restantes. Não exige nota perfeita do Qwen: erros precisam ser preservados e
identificados. Uma resposta correta isolada também não encerra o contrato.

Banco de grafos, extração automática de fatos, otimização autônoma e geração de
cenas não são requisitos dessa versão. Os objetivos posteriores registrados são
performance do renderer em CPU e cenas por linguagem natural, após validar os
engines. O contrato não é especificação externa da API Ollama.

## Resultado da aula e próxima aula

O ciclo possui requisitos examináveis. A
[Aula 25](25-primeira-comunicacao-ollama.md) apresenta o preparador, o cliente HTTP,
a primeira comunicação real registrada e o coordenador testado por simulação,
sem confundir esses avanços com fechamento de todos os critérios.
