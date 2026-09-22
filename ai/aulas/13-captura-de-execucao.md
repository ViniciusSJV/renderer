# Aula 13 — Captura de execução

## Objetivo

Registrar uma execução em arquivos e distinguir sucesso do capturador de sucesso
do comando capturado.

## Pré-requisitos e preparação do ambiente

Use o clone e Rust/Cargo da Aula 3 em **Linux com terminal Bash**. O capturador
usa APIs Unix, sockets locais, `date`, `uname`, Rust, Cargo e Git. O código não
compila como capturador nativo Windows. Um ambiente Linux remoto pode executar
esta etapa; não precisa acessar o Ollama do Windows.

Na raiz, confira os utilitários usados para horário e identificação do sistema.
São consultas de leitura, sem modificar arquivos:

```bash
date -u
```

A saída deve apresentar data/hora UTC. Confira o sistema:

```bash
uname -r
```

Deve aparecer a versão do kernel. `date` é necessário para concluir a captura;
sondagens como `uname`, Rust, Cargo e Git registram também indisponibilidade.
Se faltarem utilitários, instale os pacotes correspondentes da distribuição.
Não há bootstrap Linux nem configuração WSL/Dev Container validada no repositório.

## Conceitos e implementação

[src/bin/capture_execution.rs](../../src/bin/capture_execution.rs) cria um
diretório novo por execução, com `execucao.json` e `saida.bin`. O JSON registra
identidade, argumentos, diretório, horários, ambiente parcial e resultado.
A saída combina bytes de stdout e stderr, inclusive bytes não UTF-8. Não preserva
a identidade dos canais nem garante a ordem lógica de mensagens em buffers.

O comando é uma lista de argumentos, sem interpretação automática por shell.
A entrada padrão fica fechada: use comandos sem interação. A biblioteca padrão
e `serde_json` já estão disponíveis no projeto.

| Resultado do comando | `status` | `exit_code` |
| --- | --- | --- |
| Término normal | `exited` | Código, inclusive não zero. |
| Falha ao iniciar | `start_failed` | `null`; diagnóstico em `error`. |
| Término por sinal | `signaled` | `null`; número em `signal`. |

O capturador retorna **0 quando consegue registrar**, mesmo que o comando falhe,
e **2 em erros tratados de captura ou argumentos**. Automação de testes deve
examinar `result`, não apenas o código do capturador.

A saída é gravada progressivamente e sincronizada. O JSON temporário é renomeado
para `execucao.json` por último. Sem esse arquivo final, o registro é parcial.
Destinos existentes são recusados antes de executar o comando. Não há timeout
nem controle completo de descendentes; uma saída mantida aberta pode bloquear
até depois do término do processo principal.

## Passo a passo

Na raiz Linux, teste o capturador. Cargo escreve em `target` e os testes usam
diretórios temporários; Ollama não participa:

```bash
cargo test --locked --bin capture_execution
```

O formato inicial teve **8 testes aprovados**. O código atual inclui a extensão
da Aula 14 e seus testes; não exija a contagem antiga.

Faça uma captura nova de uma consulta à versão do compilador. O diretório pai
é a própria raiz; não crie `aula13-captura` antes. O comando cria a pasta e os
dois arquivos, além dos artefatos de compilação:

```bash
cargo run --locked --bin capture_execution -- --id AULA13_RUST_01 --destino aula13-captura -- rustc --version
```

O primeiro `--` separa Cargo do programa; o segundo, opções da captura do comando.
Abra `aula13-captura/execucao.json` no editor: com compilador disponível, observe
`exited` e código 0. `saida.bin` deve conter a versão, cujo texto varia.
A versão atual gera formato 2, mesmo sem fontes; a primeira implementação gerava
formato 1. Não atribua uma execução nova ao formato antigo.

## Evidências históricas

| Registro | Resultado | Bytes | Código do capturador |
| --- | --- | --- | --- |
| [Sucesso](../experimentos/04-captura/rust-sucesso/execucao.json) | `exited`, 0 | 19 | 0 |
| [Falha](../experimentos/04-captura/rust-falha/execucao.json) | `exited`, 7 | 17 | 0 |
| [Inexistente](../experimentos/04-captura/rust-inexistente/execucao.json) | `start_failed`, null | 0 | 0 |

Os testes também exercitam sinal POSIX, saída binária, destino existente, falha
de publicação e argumentos sem shell. Registros do protótipo Python nas pastas
sem prefixo `rust-` não foram reatribuídos ao capturador Rust.

## Validação e limites

Saída vazia não significa sucesso; `null` não equivale a zero. Uma pasta parcial
não deve ser reutilizada: examine seus arquivos e escolha destino e ID novos.
O código 0 informa conclusão do registro, sem autenticação, snapshot atômico ou
garantia completa contra queda de energia.

O formato inicial não associava hashes de fontes nem era importado pelo
Bibliotecário. HEAD não identifica alterações locais nem comprova compilação.
Não houve benchmark ou consulta ao modelo nesta etapa.

## Resultado da aula e próxima aula

Comando, saída e resultado podem ser preservados para inspeção. A
[Aula 14](14-associar-codigo-e-execucao.md) acrescenta observações de hashes de
fontes antes e depois de uma nova execução.
