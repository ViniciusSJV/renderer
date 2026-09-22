# Experimento 19 — conferência → consulta → transporte, com rubrica prévia

Data do fechamento documental: 22/09/2026. Servidor HTTP simulado no Codespaces;
nenhum Ollama/modelo real executado neste experimento.

## Implementação

`explain_evidence CONFIG.json DIRETORIO_NOVO` reserva uma pasta exclusiva, valida
configuração/rubrica e preserva a rubrica antes da conferência. Executa o binário
validate_evidence irmão (mesmo diretório/perfil) com --bundle, sem shell. Registra
stdout/stderr, resultado e duração da conferência. Se ela falhar, não chama HTTP.

Após conferir os hashes do pacote e a seleção, chama o transporte compartilhado
em ollama_common/client.rs. send_ollama conserva sua CLI histórica e continua
registrando evidence_rechecked=false. Somente o coordenador usa a ligação da
conferência executada nesta invocação. O transporte verifica novamente os bytes
da consulta contra o hash antes de montar a requisição. Origem e seleção passam
a constar em prepared.json/result.json; integration contém hashes da origem e
da rubrica. Rubrica não é enviada no prompt.

O critério de rubrica usa JSON estrito: id e criteria não vazios; cada critério
com id único, description e expected. Campos como score/results/response são
recusados: usar uma rubrica ainda não preenchida, não uma avaliação histórica.
Isso verifica estrutura, não qualidade dos critérios nem sua autoria/autenticidade.

## Comandos reproduzíveis (Codespaces)

```bash
cargo build --locked --offline --bin validate_evidence --bin explain_evidence --bin send_ollama
cargo test --locked --offline --bin explain_evidence --bin send_ollama --bin prepare_ollama --bin validate_evidence
OLLAMA_INTEGRATION_RECORD_DIR=ai/experimentos/19-integracao-simulada/execucao-02 cargo test --locked --offline --bin explain_evidence record_lesson_pipeline -- --ignored --nocapture
```

O gravador exige pasta nova e autorização de sockets locais no ambiente. O teste
integrado procura o validador em target/debug; a CLI real resolve o executável
irmão de current_exe. Compilar os dois no mesmo perfil antes de usar. Testes com
CARGO_TARGET_DIR customizado ainda não estão tratados nessa fixture.

## Evidência e medição

Verificação final: **78 testes do Bibliotecário, 13 do transporte, 3 do preparador
e 5 da integração aprovados** (99 no total). Dois gravadores ignorados por padrão;
o gravador integrado foi executado separadamente e passou.

Casos integrados: exportação real com servidor simulado; rubrica presente antes
do POST e gabarito ausente do prompt; fonte alterada bloqueia HTTP (socket sem
conexão); rubrica preenchida/duplicada/vazia rejeitada; consulta alterada após
exportação rejeitada antes de criar a requisição. Os testes anteriores continuam
cobrindo timeout, limites, falhas do provedor e persistência.

Uma amostra debug preservada em execucao-01: 12,212222 ms no coordenador,
4,012230 ms na conferência como processo e 1,948818 ms no transporte. Corpo
enviado: 1796 bytes; retorno: 71 bytes. As durações têm escopos distintos: o
coordenador inclui I/O e etapas; conferência inclui iniciar o processo; transporte
inclui construir cliente, enviar, ler e classificar. Sem compilação nesses tempos.
Não são benchmarks nem resultados de qualidade/performance de modelo real.

execucao-01 contém cópias byte a byte da fixture temporária do teste. Caminhos
absolutos e porta são os observados; o diretório temporário original foi removido.
Não executar seu config.json para repetir: use o gravador para criar nova fixture.
A resposta F/S é fixa; não representa avaliação semântica aprovada de um modelo.

## Arquivos e falhas

Na pasta da tentativa: started.json; config.json/rubric.json; validation.stdout,
validation.stderr, validation.json; bundle/; ready.json; transport/; result.json.
O resultado final liga o resultado de transporte por SHA-256. Os caminhos de
integration são relativos a transport; os arquivos internos da origem ficam em
bundle. Falhas antes do envio produzem input_rejected/sent=false quando possível.
Erro de I/O no coordenador interrompe com código 1 e pode deixar registro parcial.
Uma falha do validador (inclusive sua exportação) é input_rejected, código 2, sem
HTTP; seu diagnóstico está nos logs. 0 significa completed com registro final;
2 significa estado registrado diferente de completed. Não sobrescreve destinos.

Timeout e limite configurados aplicam-se ao HTTP. O subprocesso validador ainda
não tem timeout/limite de stdout próprios. Sem retry, snapshot atômico, execução
autenticada, prova das entradas do compilador ou validação semântica automática.
Arquivos podem mudar depois da conferência; o escopo é antes do envio nesta
invocação. Não há aceitação automática de pacotes antigos para marcar rechecagem.

A chamada integrada real no Windows e sua avaliação com esta rubrica vinculada
ficam para a Aula 26. Os engines ainda não estão fechados.
