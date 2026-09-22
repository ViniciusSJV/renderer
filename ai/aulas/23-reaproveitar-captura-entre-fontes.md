# Aula 23 — Reaproveitar a captura entre fontes

## Objetivo

Verificar o compartilhamento da conferência de uma captura durante a exportação,
sem eliminar as verificações individuais das fontes.

## Contexto e pré-requisitos

Use Linux/Bash, Cargo, Python 3 da Aula 16 e `aula21-resultados` da Aula 21.
Todos os comandos partem da raiz; Ollama não participa. A Aula 22 separou fonte,
ligação e documento. Agora S fontes da mesma captura exigem S conferências na
validação inicial, mas apenas uma conferência completa na exportação.

## Implementação e limites da identidade

Em [validate_evidence.rs](../../src/bin/validate_evidence.rs), `CaptureCache`
guarda `CheckedCapture` com a chave caminho absoluto, SHA-256 esperado e run_id.
Não resolve links simbólicos nem unifica aliases com `..`: a localização
declarada determina onde procurar a saída.

`validate_capture_link_cached` mantém tipo, arquivo, hash, linhas e ligação da
fonte. Só o documento é reaproveitado. A entrada é inserida após conferência
bem-sucedida; uma ligação posterior inválida interrompe a exportação.

Os mapas ficam locais a `selections_json`. A validação inicial usa um mapa novo
por chamada. Não há cache global, entre exportações ou processos. O formato
exportado permanece igual. Essa política reutiliza uma observação anterior,
sem snapshot atômico, autenticação ou prova das entradas do compilador.

## Passo a passo

Na raiz, execute a suíte. Cargo grava em `target`; as fixtures verificam identidade,
fontes inválidas e isolamento sem modificar evidências históricas:

```bash
cargo test --locked --bin validate_evidence
```

A etapa original teve **76 testes aprovados**. Dois casos novos verificaram
compartilhamento com fonte válida, recusa de linhas/saída incorretas e diferenças
de hash, run_id ou localização sem inserir entradas inválidas. A árvore atual
tem mais testes; o resultado relevante é ausência de falhas.

Leia os contadores da prática local da Aula 21 e compare suas exportações com
as contagens previstas para a versão atual. Este bloco, na raiz, somente lê
os logs e arquivos, sem reexecutar o validador ou gravar novos resultados:

```bash
python3 - <<'PY'
import json
from pathlib import Path

for sources in (1, 10, 100):
    stderr = Path(f'aula21-resultados/{sources}.stderr.txt').read_text(encoding='utf-8')
    reports = [line.split(' ', 1)[1] for line in stderr.splitlines()
               if line.startswith('BIBLIOTECARIO_METRICS ')]
    assert len(reports) == 1
    counts = json.loads(reports[0])['counts']
    assert counts['capture_validate']['calls'] == sources + 1
    assert counts['capture_link']['calls'] == 2 * sources
    assert counts['source_content_read']['calls'] == 3 * sources
    export = json.loads(Path(f'aula21-resultados/export-{sources}.json').read_text(encoding='utf-8'))
    assert len(export['selections']) == 100
    print(sources, counts['capture_validate']['calls'])
PY
```

Observe pares 1/2, 10/11 e 100/101. Se alguma asserção falhar, confira a versão
do executável release e os logs completos antes de atribuir causa. Um build
antigo não representa necessariamente a fonte atual.

## Evidência histórica

As [previsões](../experimentos/14-captura-compartilhada/previsoes.md) e os
[resultados](../experimentos/14-captura-compartilhada/resultados.json) registram
100 fichas distribuídas entre fontes da mesma captura:

| Fontes | Conferências antes | Depois | Bytes antes | Depois |
| --- | --- | --- | --- | --- |
| 1 | 2 | 2 | 41.880 | 41.880 |
| 10 | 20 | 11 | 418.800 | 237.630 |
| 100 | 200 | 101 | 4.188.000 | 2.195.130 |

As leituras do registro passaram a S + 1; chamadas de hash, 6(S + 1).
Ligação permaneceu 2S e leitura de conteúdo, 3S. Naqueles arquivos, os bytes
somavam 1.620S + 20.130(S + 1), não uma fórmula geral de tamanho.
As exportações foram comparadas byte a byte com as da Aula 22 e permaneceram
idênticas. A ligação inválida continuou recusada na fase inicial; testes unitários
exercitaram também o mapa já preenchido.

## Validação e conclusão permitida

Houve redução de trabalho lógico nas cargas com várias fontes. Não houve nova
medição de tempo; portanto a etapa não demonstrou ganho adicional de latência
nem desempenho do renderer. As verificações próprias da fonte não desapareceram.
A validação inicial ainda repete capturas; esse limite é conhecido.

A leitura dos logs atuais reproduz a regra atual, mas não uma comparação entre
binários históricos. As avaliações do modelo e os registros antigos permanecem
inalterados, sem comprovação retroativa de RUN_VECTOR_1.

## Resultado da aula e próxima aula

O reaproveitamento tem identidade, escopo e limites testados. A
[Aula 24](24-contrato-dos-engines.md) consolida entradas, saídas e critérios de
conclusão antes da comunicação Rust–Ollama.
