# Contrato dos engines — versão 1

Estado atual: Aula 25 concluída; integração implementada e testada com servidor
simulado, execução integrada real ainda pendente (Aula 26). Este documento
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

## Evidência incremental da Aula 25 (aula ainda em andamento)

A tabela da Aula 24 acima permanece como fotografia do planejamento inicial.
Em 21/09/2026, o adaptador `send_ollama` passou em 12 testes locais; três testes
do preparador compartilhado também passaram. Um experimento simulado separado
preserva artefatos e medição em `ai/experimentos/16-cliente-http-simulado/`.

- L1: implementado e simulado em HTTP; chamada Rust real no Windows pendente.
- L2: consulta, corpo enviado, retorno, texto e configuração ligados por
  tentativa/hashes; origem do dossiê e seleção ainda nulas (integração pendente).
- L3: falhas, timeout, limite e gravação exercitados localmente; validar também
  a execução portátil no Windows. Erros de uso anteriores à tentativa vão a stderr.
- L4: seleção explícita do modelo implementada sem alteração do Graph Engine;
  troca real de modelo não realizada nem necessária nesta etapa.
- I1: testes locais do adaptador implementados e aprovados.
- I3: latência e tamanhos registrados no transporte simulado; ciclo integrado pendente.
- I4: CLI, artefatos e limites documentados na Aula 25; execução Windows pendente.
- G4 e I2 permanecem pendentes. Não há novo fechamento dos engines.

O adaptador desta aula não recebe credenciais e aceita apenas HTTP; não reconfere
fontes. `completed` exige resposta estruturada com done=true e done_reason=stop.
O marcador final de gravação não é autenticação ou snapshot atômico das fontes.

### Atualização — testes simulados no Windows

O usuário forneceu saída de cargo test --locked --bin send_ollama após aplicar
a correção do teste de porta fechada: **13 aprovados, 0 falhas e 1 ignorado**,
em 2,05 s. Os 3 testes do preparador também passaram anteriormente no Windows.
A falha inicial e a correção estão documentadas na Aula 25. Isso acrescenta
execução Windows à evidência de L3/I1; a chamada real Rust–Ollama de L1 continua
pendente. Não fecha G4/I2 nem transforma tempos de teste em latência do modelo.

### Atualização — chamada Rust real no Windows, tentativa 02

O usuário forneceu a saída de OLLAMA_WINDOWS_RUST_02: completed, com registro
em tentativa-ollama-rust-02/result.json. É evidência de L1 por relato de execução
do cliente real; o registro e a resposta ainda aguardam recebimento/exame.
A tentativa 01 ficou sem result.json e foi preservada como parcial. O envio usou
a consulta histórica da Aula 17, sem reconferência: G4/I2 continuam pendentes.
Avaliação semântica e medições da tentativa 02 ainda não examinadas. Sem fechamento
antecipado da Aula 25 ou dos engines.

Registro/texto da tentativa 02 foram posteriormente fornecidos pelo usuário:
HTTP 200, completed, 7682,6093 ms, 8739 bytes de requisição e 18171 de retorno.
Transcrições e avaliação retrospectiva da rubrica existente (3/6) estão no
experimento 17. Os originais ainda não foram recebidos para conferir os hashes.
L1 tem chamada real relatada e I3 tem medição de transporte real; G4/I2 continuam
pendentes por ausência de ciclo recém-conferido e avaliação previamente vinculada.

Originais da tentativa 02 recebidos em ZIP e preservados no experimento 17.
Hashes, tamanhos, IDs, coerência prepared/result, consulta/prompt e retorno/texto
conferidos com sucesso. L1 agora tem retorno bruto examinado; L2/I3 têm artefatos
de transporte real conferidos, mas origem/seleção seguem nulas. A avaliação do
original permanece 3/6 em documento separado. G4/I2 ainda exigem integração e
avaliação previamente vinculada; não declarar os engines fechados.

### Integração incremental — pacote de origem

validate_evidence --bundle gera consulta conferida e cópias de dossiê/pergunta
com hashes e parâmetros de seleção (experimento 18, 78 testes aprovados).
G4/L2 continuam parciais: ainda falta ligar essa exportação ao envio e à rubrica
prévia no mesmo fluxo. O pacote é evidência de uma exportação, não autorização
para promover uma consulta posteriormente carregada a evidência atual.
A captura histórica da Aula 17 é hoje recusada por divergência de Cargo.toml;
não alteramos seu hash registrado para contornar a validação.

## Revisão ao fechar a Aula 25 — 22/09/2026

A tabela original da Aula 24 e as atualizações intermediárias são históricas.
Agora explain_evidence chama a conferência/exportação antes do HTTP, liga origem,
seleção e rubrica prévia ao transporte e rejeita divergências sem enviar.
99 testes aprovados; experimento 19 preserva simulação e medidas. A versão final
extraída do transporte/coordenador ainda aguarda validação Windows.

| Critério | Evidência atual e pendência |
| --- | --- |
| G1–G3 | Mantidos; 78 testes do Bibliotecário passaram com fixtures isoladas e rejeição explícita de fonte alterada. |
| G4 | Fluxo implementado e simulado; pendente demonstração integrada real. |
| L1 | Chamada Rust real Windows recebida e conferida (versão anterior à extração final do módulo). |
| L2 | Origem/seleção/rubrica ligadas no fluxo simulado; falta aplicação real e conferência de seus artefatos. |
| L3 | Falhas de transporte e bloqueios de integração testados; prazo do subprocesso Graph ainda não implementado. |
| L4 | Modelo é configuração explícita, sem alteração do Graph Engine. |
| I1 | Testes locais de envio/falhas e integração aprovados. |
| I2 | Pendente: chamada histórica real teve avaliação retrospectiva; simulação não é geração real. |
| I3 | Medidas de transporte real e integração simulada registradas; falta medida da integração real. |
| I4 | CLI/configuração/limites documentados; reprodução integrada Windows pendente. |

Aula 25 concluída no escopo acordado de primeira comunicação. O fechamento v1
dos engines permanece pendente; Aula 26 executará a integração real e revisará
as lacunas. Não reinterpretar completed como nota semântica, autenticação ou
snapshot atômico. evidence_rechecked=true no coordenador significa conferência
nesta invocação antes do envio, com os limites do Bibliotecário preservados.
