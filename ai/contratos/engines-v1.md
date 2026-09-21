# Contrato dos engines — versão 1

Estado: desenho da Aula 24; integração ainda não implementada. Este documento
é um contrato interno, não uma especificação da API HTTP do Ollama. O mapeamento
para essa API será verificado ao implementar o cliente.

## Escopo e responsabilidades

A versão 1 entrega um ciclo de explicação de evidências: dossiê → conferência →
consulta → modelo via Ollama → resposta preservada → avaliação manual.

O Graph Engine registra relações e confere identidade, referências, conteúdo e
associações de execução. O LLM Engine transporta uma consulta ao modelo e preserva
sua resposta, sem promovê-la a fato. A avaliação julga a resposta inteira por
critérios prévios. Não há execução automática de comandos sugeridos pelo modelo.
Banco de grafos, extração automática de fatos, geração de cenas e otimização
autônoma de código não são requisitos desta versão.

## Entrada e ligação entre artefatos

A consulta existente mantém question, evidence e instructions. Ela não será
reescrita retroativamente para acompanhar este contrato. O cliente registrará
um envelope separado, versionado, com:

- request_id próprio, caminho e SHA-256 dos bytes da consulta;
- identificação/hash do dossiê usado e parâmetros da seleção (fichas, contexto,
  pergunta), para registrar a origem da consulta;
- endereço configurado do Ollama, modelo solicitado e opções explicitamente
  solicitadas; valores desconhecidos ou defaults efetivos não serão inventados;
- horários, resultado do envio e referências aos arquivos da resposta.

A verificação deve anteceder o envio no ciclo integrado. Uma consulta histórica
pode conter conferências antigas: ler seu campo de status não é reconferir seus
arquivos. Uma reexportação atual deve gerar nova consulta, preservando a anterior.
O hash identifica bytes; não autentica a sessão nem prova recebimento integral.

O cliente não deve truncar ou alterar silenciosamente o texto exportado. A
construção da requisição será determinística para os mesmos bytes e configuração,
sem promessa de resposta determinística do modelo. Registrar requisição exata,
resposta bruta recebida e texto extraído, permitindo comparar o que foi enviado
com a consulta original. Não registrar segredos de autenticação nos artefatos.

## Modelo e execução

Usar Ollama, inicialmente o modelo Qwen já empregado no laboratório. O nome
registrado renderer-analyst e o modelo base qwen2.5-coder:14b têm papéis distintos;
a configuração deve indicar qual nome será chamado. Uma futura troca por
DeepSeek não muda o contrato do Graph Engine. Registrar modelo/configuração em
cada tentativa; não inferir superioridade a partir do nome do modelo.

Codespaces e Windows são ambientes distintos: localhost em um não identifica
a máquina do outro. Antes da primeira chamada real, definir onde o cliente Rust
executará e como alcançará o Ollama. Não mudar a exposição de rede automaticamente.
A integração deverá funcionar com configuração explícita, não endereço embutido.

## Estados e falhas

| Estado | Significado e ação |
| --- | --- |
| input_rejected | Consulta/configuração inválida ou conferência falhou; não enviar. |
| transport_failed | Não foi possível obter resposta por erro de conexão/transporte; preservar diagnóstico. |
| timed_out | Prazo esgotado; não afirmar que o servidor deixou de processar a solicitação. |
| provider_failed | Ollama retornou erro; preservar código/mensagem disponíveis. |
| invalid_response | Retorno não corresponde ao formato esperado; preservar bytes recebidos. |
| incomplete_response | Resposta não concluída ou interrompida por limite conhecido; preservar conteúdo parcial. |
| completed | Resposta estruturalmente válida e concluída segundo os sinais disponíveis; avaliação semântica ainda pendente. |

O adaptador deve mapear os sinais reais da API para esses estados sem inventar
informações. Definir timeout e limite de bytes da resposta na configuração.
Não tentar novamente silenciosamente: uma nova tentativa recebe identidade
própria. Erro de gravação impede marcar a tentativa como registro concluído;
artefatos parciais devem ser distinguíveis. Destinos existentes não serão
sobrescritos. Não reutilizar o código de saída do capturador como significado
implícito de sucesso do cliente; documentar e testar a CLI nova.

## Resposta e avaliação

Preservar a primeira resposta antes de pedir revisão. Vinculá-la a request_id,
consulta e modelo/configuração registrados. IDs de resposta, execução do renderer,
fonte e ficha são distintos. Não atribuir resposta nova a uma avaliação antiga.

Critérios são definidos antes da resposta. Resultado por critério, justificativas,
contradições e nota só são preenchidos após recebê-la. Uma avaliação incompleta
permanece pendente; completed no transporte não altera isso. Referências a fichas
podem ser conferidas mecanicamente sem comprovar que sustentam a afirmação.

## Critérios de aceitação e lacunas atuais

| ID | Critério de conclusão v1 | Situação na Aula 24 |
| --- | --- | --- |
| G1 | Referências, hashes e linhas inválidos são recusados. | Implementado, testes existentes. |
| G2 | Captura versão 2 e ligação ao dossiê são conferidas com limites. | Implementado, testes existentes. |
| G3 | Exportação preserva IDs, contexto e limites; reaproveitamento fica local. | Implementado, testes existentes. |
| G4 | Ciclo de envio usa consulta recém-conferida e registra sua origem exata. | Pendente de integração. |
| L1 | Cliente Rust alcança Ollama com endereço e modelo configuráveis. | Pendente. |
| L2 | Consulta, requisição, resposta e configurações ficam vinculadas por identidade/hash. | Parcial: consulta/avaliação manual existem; tentativa automática pendente. |
| L3 | Estados de falha, timeout, limites e gravação são testados. | Pendente. |
| L4 | Seleção do modelo não exige alterar o Graph Engine. | Pendente no cliente; troca real de modelo não é obrigatória para fechar v1. |
| I1 | Testes locais cobrem envio, falhas e respostas simuladas, sem depender de geração real. | Pendente. |
| I2 | Ao menos uma consulta real ao Qwen via Ollama percorre o ciclo e é avaliada com rubrica prévia. | Pendente: consultas anteriores foram manuais. |
| I3 | Latência de parede e tamanhos de entrada/saída são registrados, com escopo explícito. | Pendente para o cliente integrado. |
| I4 | Comando reproduzível, configuração e limites são documentados. | Parcial: documentação do fluxo atual existe. |

São 12 critérios: 3 implementados, 2 parciais e 7 pendentes. Essa contagem é
planejamento, não porcentagem de esforço concluído. A aula não repetiu testes.
G1–G3 se apoiam nas execuções registradas até a Aula 23 (76 testes do validador).

Fechar v1 exige evidência para cada linha, uma revisão conjunta das lacunas e
registro dos limites restantes. Não exige nota perfeita do Qwen: uma resposta
incorreta deve ser preservada e identificada, não ocultada para concluir o ciclo.
Resultados anteriores de 3/6 permanecem históricos, sem comparação controlada.
