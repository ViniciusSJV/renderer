# Aula 22 — Separar fonte documental de captura compartilhada

## Objetivo

Identificar quais conferências pertencem à fonte, à ligação e ao documento de
captura, preservando a recusa de fontes inválidas mesmo diante de captura válida.

## Contexto e pré-requisitos

Use o editor e Cargo da Aula 3 e os resultados da Aula 21. Os comandos partem
da raiz; não há dependência nova nem uso de Ollama. Duas fontes podem apontar
para uma captura, mas cada fonte ainda precisa corresponder à saída declarada.

## Conceitos e implementação

| Responsabilidade | Conferências |
| --- | --- |
| Fonte documental | `test_run`, ausência de execução legada simultânea, arquivo atual, hash e linhas. |
| Ligação fonte–captura | Caminho da fonte corresponde à saída e hash declarado coincide. |
| Documento de captura | Hash do registro, run_id, formato, resultado, saída e associações da Aula 15. |

Em [validate_evidence.rs](../../src/bin/validate_evidence.rs):

- `check_capture_document` retorna `CheckedCapture`, com registro, relatório e
  caminho da saída após conferência.
- `check_source_capture_output` compara a ligação da fonte com a saída conferida.
- `validate_capture_link` coordena o fluxo. Arquivo, hash e linhas da fonte
  continuam obrigatórios.

Na refatoração original, o reaproveitamento por fonte em `selections_json`
permaneceu igual, sem cache por captura. A conferência completa do documento
passou a preceder a comparação de caminho/hash com a fonte. Havendo vários erros,
a ordem do diagnóstico pode mudar. As duas leituras do registro ainda existiam.
A árvore atual já inclui o cache da Aula 23; use os artefatos para examinar a
refatoração isolada.

## Passo a passo e validação

No editor, siga as três funções acima e localize o teste
`checked_capture_does_not_approve_an_unrelated_source`. Ele recebe captura válida
e exige recusa de caminho ou hash incompatível da fonte.

Na raiz, execute esse teste especificamente. Cargo compila em `target`; a
fixture usa arquivos temporários, sem alterar o dossiê local ou histórico:

```bash
cargo test --locked --bin validate_evidence checked_capture_does_not_approve_an_unrelated_source
```

Observe um teste selecionado aprovado. Se nenhum teste for selecionado, confira
o nome e a versão do código antes de interpretar o resumo como aprovação.

Execute também a suíte do validador, na raiz, para verificar as demais regras:

```bash
cargo test --locked --bin validate_evidence
```

A refatoração original teve **74 testes aprovados**. A árvore atual inclui testes
posteriores. Os [resultados históricos](../experimentos/13-separar-responsabilidades/resultados.json),
precedidos pelas [previsões](../experimentos/13-separar-responsabilidades/previsoes.md),
preservaram a comparação com a Aula 21:

| Fontes | Conferências | Bytes instrumentados | Exportações |
| --- | --- | --- | --- |
| 1 | 2 | 41.880 | Idênticas às anteriores. |
| 10 | 20 | 418.800 | Idênticas às anteriores. |
| 100 | 200 | 4.188.000 | Idênticas às anteriores. |

A ligação inválida continuou recusada sem exportação. Não houve medição de tempo
nem ganho demonstrado nesta refatoração. O roteiro local da Aula 21 permite
exercitar as cargas com o código atual, cuja política já é diferente.

## Identidade proposta para reutilização

A próxima etapa identifica a captura por caminho absoluto do registro,
SHA-256 esperado e run_id esperado. Caminho sozinho não distingue versões;
hash sozinho não define onde resolver `saida.bin`; ID sozinho não é único
universalmente. A chave composta também não autentica a origem.

Resolver um link simbólico sem considerar a base da saída pode mudar seu
significado. A política conservadora mantém localizações distintas sem presumir
aliases equivalentes. Cada fonte continua sendo conferida, mesmo quando o
documento está no mapa. Somente documentos conferidos com sucesso podem entrar.

Arquivos podem mudar depois da leitura: reutilização local não oferece snapshot
atômico nem detecta todas as mudanças concorrentes. Novas exportações devem
refazer as leituras.

## Resultado da aula e próxima aula

As responsabilidades estão separadas e possuem um teste que protege a ligação.
A [Aula 23](23-reaproveitar-captura-entre-fontes.md) usa essa divisão para
compartilhar a conferência do documento entre fontes, mantendo verificações individuais.
