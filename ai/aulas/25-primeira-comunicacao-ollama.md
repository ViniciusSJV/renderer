# Aula 25 — Primeira comunicação Rust–Ollama

## Conceito e estado atual

A aula está **em andamento**, seguindo com Rust e Ollama nativos no Windows.
Em 21/09/2026, o usuário trouxe saídas do PowerShell com Rust/Cargo 1.98.1 e
Ollama 0.34.2. Após iniciar `ollama serve`, o servidor informou escuta em
127.0.0.1:11434 e `ollama list` mostrou renderer-analyst:latest e
qwen2.5-coder:14b. São observações fornecidas pelo usuário, não acesso direto
ao Windows pelo assistente. O repositório ainda está no Codespaces.
Não fizemos uma chamada de geração pelo cliente Rust.

## Primeiro passo independente: preparar a requisição

Consultamos a [documentação oficial de geração](https://docs.ollama.com/api/generate).
O corpo de POST /api/generate recebe model e prompt. Usaremos stream=false para
pedir resposta única. O adaptador de transporte e o tratamento do retorno ainda
não foram implementados.

Criamos [prepare_ollama.rs](../../src/bin/prepare_ollama.rs), que lê a consulta
como UTF-8, confere sua estrutura básica e prepara model, prompt e stream.
O texto original completo é o valor de prompt, preservando espaços e quebras
de linha após decodificar o JSON da requisição. Não definimos system ou options
neste primeiro passo: não sobrescrevemos explicitamente a configuração do modelo
nem afirmamos conhecer seus valores efetivos.

Esse comando **não reconfere evidências e não envia requisições**. Uma consulta
histórica não vira uma consulta recém-conferida por passar nele. O ciclo integrado
terá de reexportar a consulta e registrar sua origem antes do envio.

## Como usar sem o assistente

Na raiz do repositório, com uma consulta já preparada:

```text
cargo run --offline --bin prepare_ollama -- CAMINHO_DA_CONSULTA renderer-analyst DESTINO_NOVO.json
```

Substitua os caminhos; o diretório pai do destino precisa existir. O modo offline
requer dependências já disponíveis localmente. Em uma instalação nova, pode ser
necessário obtê-las antes. O nome renderer-analyst deve existir no Ollama escolhido
na etapa de envio; preparar o arquivo não verifica a instalação do modelo.
O comando recusa destino existente, retorna 0 na preparação concluída e 1 em erro.
Uma falha de gravação pode deixar arquivo parcial; nunca o trate como requisição
pronta sem verificar o término do comando. Não há registro de tentativa HTTP aqui.

## Testes e limites

**3 testes passaram** com cargo test --offline --bin prepare_ollama: preservação
do prompt/determinismo, modelo configurável e rejeição de entrada inválida.
Não medimos tokens, latência, qualidade do modelo ou desempenho de rede. O código
não usa APIs Unix, mas ainda não foi compilado/testado no Windows nesta sessão.
O capturador antigo permanece Unix; preparar a requisição não o torna portátil.

## Ponto de retomada

Transferir o código atualizado para uma cópia no Windows; depois continuar
com o cliente HTTP, testes locais e primeira chamada real. Para preparar a
requisição, compilar especificamente `--bin prepare_ollama`: o capturador Unix
ainda precisa de adaptação antes de compilar todos os binários no Windows. Não marcar L1 ou I2 do contrato como atendidos. Não houve
troca do Qwen por DeepSeek. O envio ao Ollama ainda é manual.
