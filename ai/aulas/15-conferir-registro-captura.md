# Aula 15 — Conferir o registro de captura

## Objetivo

Conferir formato, saída e associações de uma captura versão 2, distinguindo
consistência do registro de sucesso do comando registrado.

## Contexto e pré-requisitos

Use Linux/Bash, Cargo e a pasta `aula14-fronteira` produzida na Aula 14.
Todos os comandos partem da raiz. Não é necessário Ollama. Um comando com
código 7 pode ter sido corretamente registrado; uma saída adulterada impede
concluir a conferência, mesmo se o JSON declarar código 0.

## Implementação

O ponto de entrada é [validate_evidence.rs](../../src/bin/validate_evidence.rs);
o [módulo capture](../../src/bin/validate_evidence/capture.rs) trata o formato 2
separadamente de fichas e relatórios legados.

O modo `--capture` exige versão 2, ID e comando não vazios, diretório absoluto,
datas presentes e objeto de ambiente não vazio. Isso não valida cronologia,
veracidade das versões ou conteúdo das sondagens. Campos adicionais são ignorados.

O resultado distingue término normal com código não negativo, falha de início
com código nulo e erro, ou sinal positivo com código nulo. Combinações incompatíveis
são recusadas. A saída deve declarar `saida.bin` e `stdout+stderr`; o caminho é
resolvido junto ao JSON. Tamanho e SHA-256 são recalculados em blocos.

| Associação | Conferência |
| --- | --- |
| `equal` | Hashes antes/depois iguais e arquivo atual correspondente ao posterior. |
| `different` | Hashes distintos e arquivo atual correspondente ao posterior. |
| `unavailable` | Hash posterior ausente e erro não vazio; arquivo atual não conferido. |

O programa compara também `cwd` + `path` com `resolved_path`. Não reconstrói o
conteúdo anterior nem comprova uma indisponibilidade histórica. Sem fontes,
conferir a saída não cria associação a código.

## Passo a passo

Na raiz, confira a captura local da Aula 14. O comando somente lê os arquivos
e imprime o relatório, além de eventual compilação em `target`:

```bash
cargo run --locked --bin validate_evidence -- --capture aula14-fronteira/execucao.json
```

Com a captura concluída e as fontes correspondentes, observe término 0 e cinco
associações conferidas. Erro retorna 1. O comando contido na captura não é executado.

Para registrar a própria conferência, compile o validador. O comando atualiza
`target/debug/validate_evidence` e não modifica as evidências:

```bash
cargo build --locked --bin validate_evidence
```

Use o capturador já compilado na Aula 14, na raiz Linux. Este comando cria uma
pasta nova e observa duas fontes do validador durante a conferência:

```bash
target/debug/capture_execution --id AULA15_CONFERENCIA_01 --destino aula15-conferencia --source src/bin/validate_evidence.rs --source src/bin/validate_evidence/capture.rs -- target/debug/validate_evidence --capture aula14-fronteira/execucao.json
```

Abra `aula15-conferencia/execucao.json` e examine o resultado do validador, não
apenas o código do capturador. A saída fica em `aula15-conferencia/saida.bin`.
As duas fontes observadas não comprovam os bytes que produziram o executável.

## Validação

Na raiz, execute os testes. Eles gravam artefatos de compilação e usam arquivos
temporários para adulterações, sem modificar capturas históricas:

```bash
cargo test --locked --bin validate_evidence
```

A etapa original teve **68 testes aprovados**, incluindo oito novos sobre
saída adulterada, tamanho, fonte alterada/ausente, comparações, versão, caminhos,
hashes e estados de término. A versão atual inclui testes posteriores.

A [conferência histórica](../experimentos/06-conferencia/fronteira/execucao.json)
registrou 540 bytes de saída correspondente e cinco fontes iguais ao hash
posterior. Esses números pertencem a RUN_EQUIVALENCE_BOUNDARY_2 naquele ambiente,
não são a saída exigida da captura local nova.

## Problemas comuns e limites

Se um arquivo atual divergir, identifique qual caminho e hash foram comparados.
O registro pode continuar sendo evidência histórica, mas já não corresponde à
árvore atual. Capturas antigas contêm caminhos de origem: mover o clone não
atualiza esses endereços. Use uma nova captura local, sem reescrever o histórico.

Conferência local não é autenticação nem proteção contra adulteração conjunta
do JSON e arquivos. Não comprova entradas do compilador. Não houve benchmark
ou consulta ao modelo nesta etapa.

## Resultado da aula e próxima aula

O Bibliotecário confere registros versão 2 com limites explícitos. A
[Aula 16](16-levar-conferencia-ao-dossie.md) liga essa captura às fichas de um
dossiê e leva o resultado da conferência à exportação.
