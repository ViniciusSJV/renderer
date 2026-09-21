# Aula 15 — Conferir o registro de captura

## Conceito

Conferir um registro é diferente de aprovar o comando registrado. Um código
7, uma mudança de fonte ou uma indisponibilidade posterior podem estar
corretamente descritos. Já uma saída cujo hash diverge do registro impede
concluir a conferência.

Antes de implementar, definimos três responsabilidades: estrutura básica da
versão 2, integridade da saída e coerência das associações de fontes. A leitura
atual das fontes será comparada com os hashes posteriores registrados; ela
não recupera os arquivos históricos nem comprova compilação.

## Implementação

O Bibliotecário ganhou um modo explícito:

```bash
cargo run --offline --bin validate_evidence -- --capture ai/experimentos/05-associacao/fronteira/execucao.json
```

O ponto de entrada continua em [validate_evidence.rs](../../src/bin/validate_evidence.rs).
A lógica nova fica no [módulo capture](../../src/bin/validate_evidence/capture.rs),
separada das fichas e dos cabeçalhos antigos. Não criamos um terceiro binário.

O modo aceita somente `schema_version: 2`; não converte os registros da Aula 13.
Confere campos obrigatórios, ID e comando não vazios, cwd absoluto, presença de
datas e objeto de ambiente não vazio. Datas e ambiente recebem apenas essa
conferência básica: formato cronológico, veracidade das versões e conteúdo das
sondagens não são verificados. Campos adicionais são ignorados pelo Serde.

O resultado deve distinguir término normal com código não negativo, falha de
início com código null e erro, ou sinal positivo com código null. Campos
incompatíveis de sinal/erro são recusados.

A saída deve declarar `saida.bin` e `stdout+stderr`. O caminho é resolvido junto
ao JSON, independentemente do cwd do comando. Recalculamos tamanho e SHA-256
em blocos, sem carregar toda a saída na memória. O JSON é carregado inteiro.

Para cada fonte, conferimos a associação `cwd` + `path` com `resolved_path`,
o formato dos hashes e a declaração:

| Declaração | Conferência |
| --- | --- |
| equal | Hashes antes/depois iguais; arquivo atual corresponde ao posterior. |
| different | Hashes antes/depois diferentes; arquivo atual corresponde ao posterior. |
| unavailable | Hash posterior ausente e erro não vazio; arquivo atual não conferido. |

Uma indisponibilidade histórica permanece declarada, não comprovada. O modo
não reconstrói o hash anterior de uma fonte modificada. Sem fontes, a contagem
conferida é zero; conferir a saída não cria associação a código.

O Bibliotecário retorna **0 se essa conferência passar** e **1 se houver erro**,
inclusive arquivo atual ausente ou divergente. Divergência atual não refuta a
execução histórica: informa que o material disponível não corresponde mais ao
hash posterior. Não há execução do comando contido no registro.

## Testes

```bash
cargo test --offline --bin validate_evidence
```

**68 testes passaram: 60 anteriores e 8 novos.** Os novos cobrem código não
zero, adulteração da saída mantendo o tamanho, tamanho incorreto, fonte atual
alterada/ausente, comparação inconsistente, diferença corretamente declarada,
indisponibilidade, versão não suportada, caminhos, hashes e estados de término.
Os testes de adulteração usam arquivos temporários.

Também exercitamos a CLI: versão 1 e uma cópia temporária da saída com um byte
alterado retornaram **1**. Nenhum artefato histórico foi modificado.

## Medição e evidência

A conferência real de RUN_EQUIVALENCE_BOUNDARY_2 retornou **0**:

- Saída de **540 bytes** com tamanho e SHA-256 correspondentes.
- **5** associações equal, **0** different, **0** unavailable.
- **5** arquivos atuais correspondentes aos hashes posteriores.

Preservamos uma captura dessa conferência em
[RUN_VALIDATE_CAPTURE_1](../experimentos/06-conferencia/fronteira/execucao.json),
com a [saída do Bibliotecário](../experimentos/06-conferencia/fronteira/saida.bin).
Ela foi produzida após compilar o validador, pelo comando:

```bash
target/debug/capture_execution --id RUN_VALIDATE_CAPTURE_1 --destino ai/experimentos/06-conferencia/fronteira --source src/bin/validate_evidence.rs --source src/bin/validate_evidence/capture.rs -- target/debug/validate_evidence --capture ai/experimentos/05-associacao/fronteira/execucao.json
```

O diretório pai deve existir. O destino acima já existe; use outro destino e
ID em nova execução. A captura identifica duas fontes do validador por hashes;
isso também não prova quais bytes produziram o binário utilizado.
Não repetimos os testes de fronteira, não fizemos benchmark nem consulta ao Qwen.

## Explicação e fechamento

A **Aula 15 está concluída**: o Bibliotecário lê o formato novo, confere saída e
associações, distingue erros e informa seus limites. Ainda não vinculamos esse
registro a fichas de um dossiê nem o incluímos na exportação ao modelo.

O programa confere consistência local. Não autentica o registro, não comprova
os bytes compilados e não impede adulteração conjunta do JSON e dos arquivos.
Não há comprovação retroativa de RUN_VECTOR_1. A avaliação da Aula 10 permanece
**3/6** e o envio ao Ollama continua manual.

Na **Aula 16 — Levar a conferência ao dossiê**, a proposta é ligar uma captura
às evidências e exportar precisamente o que foi conferido, preservando limites
e a distinção entre IDs de fonte e execução. Depois seguimos integração e
avaliação, performance e criação de cenas por linguagem natural.
