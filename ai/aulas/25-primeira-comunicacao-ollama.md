# Aula 25 — Primeira comunicação Rust–Ollama

## Objetivo

Enviar uma consulta pelo cliente Rust, localizar a resposta e os registros da
tentativa e distinguir sucesso do transporte de correção da explicação.

O estado alcançado inclui uma chamada real registrada no Windows e um coordenador
integrado testado com servidor simulado. A execução real do coordenador e sua
avaliação previamente vinculada continuam como próxima etapa, na Aula 26.

## Contexto e arquitetura

Ollama executa o modelo local e oferece a interface de geração. Ele não substitui
o Graph Engine nem fornece ao modelo acesso automático ao repositório.
A consulta contém pergunta, evidências e instruções; o adaptador coloca o texto
completo em `prompt`, informa `model` e solicita `stream: false`.

| Componente | Responsabilidade |
| --- | --- |
| `src/bin/validate_evidence.rs` | Confere dossiê/fontes e exporta a consulta. |
| `src/bin/prepare_ollama.rs` | Prepara o corpo JSON sem enviar ou reconferir fontes. |
| `src/bin/send_ollama.rs` | Envia a consulta e preserva a tentativa, sem reconferência própria. |
| `src/bin/ollama_common/mod.rs` | Valida a estrutura básica e constrói a requisição. |
| `src/bin/ollama_common/client.rs` | Transporte compartilhado e classificação do retorno. |
| `src/bin/explain_evidence.rs` | Coordena conferência, exportação e envio com origem e rubrica prévia. |

O cliente usa `reqwest` bloqueante, declarado em `Cargo.toml`; Cargo obtém essa
dependência. Uma invocação faz uma tentativa sequencial. O adaptador aceita
somente HTTP e caminho `/api/generate`, sem credenciais, query ou fragmento.
Não há proxy automático, redirecionamento ou repetição automática. Não são
informadas opções `system` ou `options`: valores efetivos do modelo não são inferidos.

## Pré-requisitos

O passo a passo principal é para **PowerShell no Windows**, com clone completo
e Ollama na mesma máquina. Assim, `127.0.0.1:11434` aponta para o servidor correto.
Um terminal Linux remoto não alcança por esse endereço o Ollama do Windows.
Não há ponte automática nem instrução para expor o serviço à rede.

Use a preparação de Git/clone da Aula 1 e Rust/Cargo da Aula 3. Na raiz, confirme
os arquivos necessários à versão atual; este bloco apenas lê a árvore:

```powershell
@('Cargo.toml', 'Cargo.lock', 'src/bin/prepare_ollama.rs', 'src/bin/send_ollama.rs', 'src/bin/ollama_common/mod.rs', 'src/bin/ollama_common/client.rs', 'src/bin/validate_evidence.rs', 'src/bin/validate_evidence/bundle.rs', 'src/bin/explain_evidence.rs') | ForEach-Object { if (-not (Test-Path -LiteralPath $_ -PathType Leaf)) { throw "Arquivo ausente: $_" } }
```

Ausência indica edição incompleta para esta aula. Alterações ainda locais não
chegam por clone do remoto. Não use pacotes parciais antigos de transferência
como substitutos de uma árvore coerente com manifesto e lockfile.

## Preparação do Rust

Na raiz, verifique o compilador, sem modificar arquivos:

```powershell
rustc --version
```

Confira Cargo separadamente:

```powershell
cargo --version
```

Os registros anteriores citam Rust/Cargo 1.98.1. Não existe versão mínima
validada no manifesto. Se faltarem, conclua a instalação de Rust com rustup;
para o alvo Windows MSVC, instale também ferramentas C++/MSVC e SDK quando o
diagnóstico apontar ausência de `link.exe`. Reabra o terminal e repita as versões.
O repositório não preserva um instalador ou bootstrap testado; a validação
operacional é a compilação e os testes seguintes.

Obtenha dependências respeitando o lockfile. O comando usa rede e grava cache,
sem atualizar intencionalmente o lockfile:

```powershell
cargo fetch --locked
```

Observe término sem erro. Não acrescente `--offline` antes de preencher o cache.
Compile os binários do fluxo no mesmo perfil, na raiz; isso grava em `target/debug`:

```powershell
cargo build --locked --bin validate_evidence --bin prepare_ollama --bin send_ollama --bin explain_evidence
```

Não use compilação indiscriminada de todos os binários no Windows: o capturador
continua Unix. O coordenador procura o validador ao lado de seu executável.

Teste os componentes, ainda sem precisar do Ollama. Os testes abrem servidores
TCP locais simulados e escrevem somente artefatos de build e fixtures temporárias:

```powershell
cargo test --locked --bin validate_evidence --bin prepare_ollama --bin send_ollama --bin explain_evidence
```

Na árvore desta revisão, são **99 aprovados**: 78 do validador, 3 do preparador,
13 do transporte e 5 do coordenador; dois gravadores ficam ignorados por padrão.
A execução final foi verificada em Linux. O cliente anterior à extração final
teve testes e chamada real no Windows; isso não substitui testar esta versão
nesse sistema. Os testes integrados esperam `target/debug/validate_evidence`;
`CARGO_TARGET_DIR` personalizado não está tratado nessa fixture.

## Instalar e verificar Ollama

Na raiz Windows, verifique a instalação, sem geração:

```powershell
ollama --version
```

Se o comando não existir, instale o aplicativo Ollama para Windows pelo
procedimento do fornecedor e reabra o PowerShell. **Limitação verificável:** as
fontes registram uso de Ollama 0.34.2 já instalado, mas não preservam instalador,
passos de instalação ou requisitos de hardware medidos. Não há script de
instalação do Ollama no repositório nem instalação Linux/WSL validada nesta
sequência. A instalação externa precisa terminar antes dos comandos seguintes;
repita a consulta de versão para verificar sua disponibilidade.

Consulte o serviço por uma operação de leitura, sem carregar uma geração:

```powershell
Invoke-RestMethod -Uri 'http://127.0.0.1:11434/api/tags' -Method Get -TimeoutSec 10 | ConvertTo-Json -Depth 6
```

Observe um JSON de modelos. Uma lista vazia indica serviço disponível, mas não
modelo pronto. Se a conexão for recusada, abra outra janela do PowerShell e
inicie o servidor, de qualquer diretório:

```powershell
ollama serve
```

Mantenha essa janela aberta e repita a consulta à API na primeira. Se houver
mensagem de porta ocupada, consulte a API antes de iniciar outra instância:
porta ocupada sozinha não identifica Ollama. Esse problema ocorreu na sequência
histórica e a API confirmou um servidor já disponível.

## Disponibilizar o modelo

Na primeira janela, confira os modelos locais; o comando apenas lista:

```powershell
ollama list
```

A base é `qwen2.5-coder:14b`; a configuração do projeto é `renderer-analyst`.
Se a base faltar, obtenha-a. O comando usa rede e grava o modelo no armazenamento
do Ollama; tempo e espaço variam e não foram validados nesta revisão:

```powershell
ollama pull qwen2.5-coder:14b
```

Repita a listagem e confirme o nome. Falta de memória, espaço ou disponibilidade
do modelo remoto deve ser resolvida antes da geração; não substitua o modelo
silenciosamente e atribua a ele o resultado histórico.

Na raiz, registre o Modelfile, criando ou atualizando a configuração no Ollama:

```powershell
ollama create renderer-analyst -f ai/ollama/Modelfile
```

Confirme a conclusão e a presença de `renderer-analyst:latest` na lista. O
Modelfile usa temperatura 0, semente 42, contexto 4096 e geração até 1024 tokens.
Esses parâmetros não treinam pesos nem garantem verdade ou ausência de truncamento.

## Preparar uma consulta reproduzível

Esta prática usa o [dossiê do preparador](../experimentos/18-origem-consulta/evidencias.json),
cuja fonte é `src/bin/ollama_common/mod.rs`. Ele pergunta sobre a exigência de
modelo explícito, sem depender da captura histórica que hoje diverge em Cargo.toml.
Leia a [pergunta](../experimentos/18-origem-consulta/pergunta.txt) e a
[rubrica planejada](../experimentos/19-integracao-simulada/rubrica-proxima-consulta.json)
antes de receber a resposta. São três critérios, diferentes dos seis da chamada
histórica; não compare suas notas como o mesmo experimento.

Na raiz Windows, confira e exporte a consulta para um arquivo novo. O comando
lê dossiê/fontes e cria a exportação, sem enviar HTTP:

```powershell
cargo run --locked --bin validate_evidence -- ai/experimentos/18-origem-consulta/evidencias.json --fact F_MODEL_EMPTY_CHECK --context 1 --question ai/experimentos/18-origem-consulta/pergunta.txt --output aula25-consulta.json
```

Observe uma fonte, uma ficha e ausência de referências inválidas. Se houver
hash divergente, compare a fonte e a cópia antes de prosseguir. Não reescreva
o dossiê histórico para fingir correspondência.

Para observar a transformação em requisição, crie outro arquivo novo. Este
comando não acessa o serviço nem reconfere evidências:

```powershell
cargo run --locked --bin prepare_ollama -- aula25-consulta.json renderer-analyst:latest aula25-requisicao.json
```

Abra o resultado no editor: `prompt` deve conter o texto integral da consulta,
`model` deve ser explícito e `stream` deve ser falso. O preparador retorna 0
quando conclui e 1 em erro. Destino existente é recusado; falha de gravação pode
deixar arquivo parcial. Essa inspeção não é uma tentativa HTTP.

## Primeira requisição pelo cliente Rust

Na raiz Windows, envie **a consulta original**, não o corpo preparado. O comando
cria `aula25-tentativa-01`, preserva entrada/configuração e faz uma geração real.
Não crie a pasta antes. O timeout é 120000 ms e o limite de resposta é 1048576 bytes:

```powershell
cargo run --locked --bin send_ollama -- aula25-consulta.json http://127.0.0.1:11434/api/generate renderer-analyst:latest 120000 1048576 AULA25_HTTP_01 aula25-tentativa-01
```

Pode não haver progresso impresso enquanto o cliente aguarda a resposta única.
O timeout aplica-se ao HTTP, não à compilação. Em sucesso, observe `completed`
e indicação de `result.json`; isso ainda deixa a avaliação semântica pendente.

Imediatamente depois, consulte o código do processo; este comando apenas imprime:

```powershell
Write-Output $LASTEXITCODE
```

A CLI retorna 0 para `completed` com registro final, 2 para outro estado registrado
e 1 para erro de uso/gravação. Cargo também pode falhar antes de iniciar o cliente.
Em erro, examine a tentativa antes de decidir repetir com outro ID e diretório.

## Ler a resposta e validar o transporte

Na raiz, leia o registro sem gerar outra resposta:

```powershell
Get-Content -LiteralPath aula25-tentativa-01/result.json -Raw -Encoding UTF8
```

Confira estado, status HTTP, modelo solicitado, duração, tamanhos, hashes e
`response_body_complete`. Leia o texto extraído, se o registro indicar sua existência:

```powershell
Get-Content -LiteralPath aula25-tentativa-01/response.txt -Raw -Encoding UTF8
```

O corpo original está em `response.bin`; o texto extraído corresponde ao campo
`response` do JSON quando a estrutura permite. `query.json`, `request.json` e
`prepared.json` preservam entrada e configuração antes do POST. `started.json`
marca a reserva; `result.json` é publicado por último. Sem resultado final,
a tentativa pode estar parcial e não há garantia de que nenhum envio ocorreu.

Para conferir o hash do corpo preservado, execute a leitura na raiz e compare
com `response_sha256` do resultado:

```powershell
Get-FileHash -LiteralPath aula25-tentativa-01/response.bin -Algorithm SHA256
```

Maiúsculas na representação hexadecimal não mudam o hash. Compare também a
consulta e o prompt pelo editor. Hash correspondente identifica bytes; não
comprova que o modelo processou todo o contexto sem truncamento interno.

| Estado | Diagnóstico e alcance |
| --- | --- |
| `input_rejected` | Entrada/configuração recusada; sem envio nessa tentativa. |
| `transport_failed` | Falha de conexão ou leitura; prefixo recebido pode estar preservado. |
| `timed_out` | Prazo esgotado; não prova que o servidor parou a geração. |
| `provider_failed` | HTTP não 2xx ou campo `error` do provedor. |
| `invalid_response` | JSON inválido ou campos essenciais incompatíveis. |
| `incomplete_response` | Limite de bytes, `done=false` ou razão de término diferente de `stop`. |
| `completed` | Estrutura aceita, `done=true` e `done_reason=stop`; sem julgamento semântico. |

O limite preserva um prefixo e descarta um byte adicional usado para detectar
excesso. Não é limite de tokens nem de toda a memória do processo. Nome reportado
pode diferir do solicitado por alias; examine o corpo bruto. Uma resposta pode
estar estruturalmente completa e tecnicamente errada.

## Evidências históricas e avaliação

A primeira chamada manual registrada no [experimento 15](../experimentos/15-ollama-windows/)
usou PowerShell após o preparador. O retorno transcrito declarou 15,649 s no
servidor, incluindo 5,823 s de carga, 2998 tokens de entrada e 505 gerados.
Não houve medição completa no cliente nem conferência dos bytes da requisição.

A chamada Rust do [experimento 17](../experimentos/17-cliente-rust-windows/originais/)
preservou originais com HTTP 200, `completed`, consulta de 8203 bytes, requisição
de 8739 e retorno de 18171. O cliente mediu **7,683 s**; o servidor declarou
7,6739318 s, 2998 tokens de prompt e 450 gerados. Hashes, tamanhos, identidade,
consulta/prompt e resposta/texto foram conferidos. Não é comparação controlada
com o envio manual nem medição do renderer.

Essa chamada usou a consulta histórica da Aula 17, sem reconferência. A
[avaliação separada](../experimentos/17-cliente-rust-windows/avaliacao.json)
aplicou retrospectivamente a rubrica existente e marcou **3/6**: C1/C4/C6
atendidos; explicação insuficiente do pânico esperado, omissão de fichas e limites
impediram os demais pontos. Não houve rubrica previamente vinculada à tentativa.
As notas das aulas 10 e 17 permanecem inalteradas.

Na prática nova, aplique os três critérios preparados à resposta inteira e
registre justificativas em arquivo separado. Não altere o resultado de transporte
para fazê-lo representar a avaliação. `send_ollama` registra
`evidence_rechecked=false`: a exportação anterior foi uma etapa separada.

## Integração disponível e teste sem modelo real

`validate_evidence --bundle` cria um pacote com consulta, dossiê, pergunta e
`origin.json`, com hashes e seleção. O
[experimento 18](../experimentos/18-origem-consulta/) verificou igualdade com a
exportação simples, recusas e uma amostra debug de 228,051 ms para 2201 bytes
de consulta. O pacote sozinho não reconfere evidências no momento de um envio futuro.

`explain_evidence` preserva rubrica antes da resposta, executa o validador irmão,
confere o pacote e os bytes da consulta e chama o transporte compartilhado.
Falha de conferência bloqueia HTTP. Rubrica preenchida ou estruturalmente inválida
é recusada; o gabarito não entra no prompt. Configuração real planejada:
[config-windows-planejado.json](../experimentos/19-integracao-simulada/config-windows-planejado.json).

Para reproduzir somente a simulação em **Linux/Bash**, depois da compilação
inicial no mesmo perfil, execute na raiz. O comando cria um diretório novo de
registro e usa resposta fixa de servidor de teste, sem Ollama:

```bash
OLLAMA_INTEGRATION_RECORD_DIR=aula25-simulacao cargo test --locked --bin explain_evidence record_lesson_pipeline -- --ignored --nocapture
```

Observe o teste aprovado e `aula25-simulacao/attempt/result.json`. O retorno
fica em `aula25-simulacao/attempt/transport/response.txt`. O teste verifica
rubrica antes do POST, origem, seleção e transporte. Outros testes da suíte recusam fonte alterada,
consulta modificada e rubrica preenchida. O servidor precisa poder abrir sockets
locais. Não use uma configuração preservada de fixture temporária para fazer
nova chamada: ela contém caminhos e porta daquela execução.

O [experimento 19](../experimentos/19-integracao-simulada/) registrou uma amostra
simulada: 12,212 ms no coordenador, 4,012 ms na conferência e 1,949 ms no HTTP,
1796 bytes enviados e 71 recebidos. O [experimento 16](../experimentos/16-cliente-http-simulado/)
registrou transporte isolado de 1,552 ms, 149 bytes enviados e 120 recebidos.
São amostras debug sem geração real, não benchmarks do Qwen.

## Problemas comuns

- **Destino existente:** é uma proteção contra sobrescrita. Leia os arquivos;
  preserve a tentativa e use ID/diretório novos quando decidir repetir.
- **Ausência de `result.json`:** registro parcial; não conclua sucesso nem ausência
  de envio. A tentativa histórica 01 teve essa situação.
- **Porta fechada ou timeout:** confira primeiro a API de modelos. Um teste Windows
  mostrou timeout antes da recusa esperada; a classificação passou a ter teste
  determinístico separado do ensaio de rede, que aceita a falha realmente observada.
- **Hash divergente:** examine a fonte. A adição da dependência HTTP mudou
  Cargo.toml e invalidou a reconferência da captura antiga, sem refutar sua história.
- **Validador não encontrado pelo coordenador:** compile ambos no mesmo perfil.
  O timeout configurado cobre HTTP; o subprocesso validador ainda não tem prazo próprio.

## Resultado da aula e próxima aula

O laboratório dispõe de cliente, registros de transporte, chamada real histórica
e integração simulada. Isso prova capacidades específicas nos casos exercitados,
sem autenticar execuções, garantir semântica das fichas, correção das respostas,
snapshot atômico ou bytes efetivamente usados pelo compilador.

A etapa registrada para a **Aula 26** é executar o coordenador real no Windows
com a configuração atual e rubrica prévia, preservar e avaliar a resposta e
revisar G4/L2/I2/I4 antes de fechar os engines. A versão final compartilhada e o
coordenador ainda não têm execução Windows comprovada pelos registros finais.
Os objetivos de otimização CPU e cenas por texto continuam posteriores a essa
validação; nenhum deles é concluído por uma resposta do Ollama.
