# Experimento 16 — transporte HTTP simulado

Data: 21/09/2026. Ambiente: Codespaces/Linux, build debug. Nenhum modelo foi
executado. O servidor simulado recebe POST /api/generate e devolve JSON fixo.
A consulta é sintética, criada em diretório temporário pelo teste; a cópia exata
está em tentativa-01/query.json. Seu caminho temporário original já não existe.
O modelo solicitado e o reportado são nomes sintéticos diferentes de propósito.

## Protocolo reproduzível

Na raiz do repositório, com dependências disponíveis:

```bash
cargo test --locked --offline --bin send_ollama --bin prepare_ollama
OLLAMA_SIMULATION_DIR=ai/experimentos/16-cliente-http-simulado/tentativa-02 cargo test --locked --offline --bin send_ollama record_lesson_simulation -- --ignored --nocapture
```

Usar sempre diretório novo; o pai deve existir. O segundo comando é Bash,
para reproduzir no Codespaces, não uma instrução de execução Windows.
O sandbox precisa permitir portas TCP em 127.0.0.1. O servidor escolhe uma porta
livre; endpoint e horários naturalmente variam. Não sobrescrever tentativa-01.

## Resultado observado

- Suíte: 12 testes do cliente aprovados, 1 gravador ignorado por padrão;
  3 testes do preparador aprovados. Duração reportada da suíte do cliente: 0,84 s.
- Gravador executado separadamente: 1 teste aprovado, 12 filtrados.
- Tentativa SIMULATED_HTTP_1: HTTP 200, completed, avaliação semântica pendente.
- Latência: 1,551635 ms. Consulta: 67 bytes. Requisição: 149 bytes. Retorno: 120 bytes.
- Request_id, configurações, hashes e arquivos: [resultado](tentativa-01/result.json).

A latência usa Instant, da construção do cliente até terminar a leitura e
classificação do corpo. Exclui preparação/gravação dos artefatos e inicialização
do servidor simulado. Tamanhos são dos corpos/consulta, não tráfego TCP total.
Uma única amostra não caracteriza distribuição, desempenho do Ollama ou ganho.
O servidor não processa o prompt e não testa tokens, qualidade ou truncamento
interno do modelo. Os resultados não alteram avaliações históricas.

## Falhas e correção durante o desenvolvimento

A primeira execução isolada não pôde abrir sockets no sandbox. Ao permitir
localhost, o caso de timeout durante o corpo revelou classificação incorreta
como transport_failed. O io::Error encapsula o erro reqwest: a inspeção passou
a incluir esse erro interno, e os dois casos de timeout passaram. Uma falha de
gravar response.bin após o envio também foi simulada; result.json permaneceu ausente.

Os arquivos desta pasta registram a tentativa sintética bem-sucedida; os outros
casos usam pastas temporárias removidas ao encerrar cada teste. Não são capturas
autenticadas e não usam o capturador Unix. A execução Windows permanece pendente.
