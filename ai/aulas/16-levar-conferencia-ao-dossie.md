# Aula 16 — Levar a conferência ao dossiê

## Objetivo

Ligar duas fichas a uma captura local e exportar o resultado da conferência,
preservando as identidades de fonte documental e execução.

## Contexto e pré-requisitos

Use Linux/Bash, Cargo e a captura `aula14-fronteira`, conferida na Aula 15.
Todos os comandos partem da raiz. A prática cria um novo dossiê; os registros
históricos permanecem intactos. Ollama ainda não participa.

O bloco auxiliar abaixo usa somente a biblioteca padrão do Python para montar
JSON e calcular hashes, como apoio documental, sem substituir o validador Rust.
Confira a ferramenta na raiz; o comando somente imprime sua versão:

```bash
python3 --version
```

Se ausente, instale Python 3 pelo gerenciador de pacotes da distribuição e
repita a verificação. O repositório não registra um instalador validado; não são
necessários pacotes via pip. Python será usado também para preparar as cargas
sintéticas da Aula 21. O programa do laboratório continua sendo Rust.

## Conceitos e implementação

Uma ficha aponta para uma fonte; uma fonte `test_run` pode conter `capture`,
com `path`, `sha256` do registro e `run_id`. O caminho da fonte aponta para a
saída, enquanto o caminho de `capture` aponta para o JSON da execução.
Os caminhos do dossiê são relativos à raiz de execução; `saida.bin` dentro da
captura é relativo ao diretório do registro.

O Bibliotecário exige tipo `test_run`, ausência de `execution` legado simultâneo,
correspondência de arquivo/hash/linhas, identidade/hash da captura e ligação à
saída correta. Depois aplica a conferência da Aula 15. Erros impedem exportação.
`source.capture_validation` contém resultado, identidade, itens conferidos e
limites. Essa fonte não exporta o booleano legado nem a conferência do cabeçalho.

## Passo a passo: construir o dossiê local

O bloco seguinte lê a captura nova, exige quatro testes aprovados e localiza as
duas linhas de interesse. Ele transporta afirmações previamente definidas para
um exemplo específico; não é um extrator geral de fatos. Cria somente
`aula16-dossie.json`, recusando sobrescrita. Execute na raiz Linux:

```bash
python3 - <<'PY'
import hashlib
import json
from pathlib import Path

record_path = Path('aula14-fronteira/execucao.json')
record_bytes = record_path.read_bytes()
record = json.loads(record_bytes)
output_path = record_path.parent / 'saida.bin'
output = output_path.read_bytes()
lines = output.decode('utf-8').splitlines()
assert record['result']['status'] == 'exited'
assert record['result']['exit_code'] == 0
assert record['output']['sha256'] == hashlib.sha256(output).hexdigest()

def locate(fragment):
    positions = [i + 1 for i, line in enumerate(lines) if fragment in line]
    assert len(positions) == 1, (fragment, positions)
    return positions[0]

result_line = locate('test result: ok. 4 passed; 0 failed;')
panic_line = locate('test rejects_difference_at_epsilon - should panic ... ok')
dossier = {
    'id': 'AULA16_DOSSIE_LOCAL',
    'unknowns': ['Não autentica execução nem comprova bytes compilados.',
                 'Não valida semanticamente as afirmações.'],
    'sources': [{
        'id': 'TEST_BOUNDARY_LOCAL', 'kind': 'test_run',
        'path': output_path.as_posix(),
        'sha256': hashlib.sha256(output).hexdigest(), 'lines': lines,
        'capture': {'path': record_path.as_posix(),
                    'sha256': hashlib.sha256(record_bytes).hexdigest(),
                    'run_id': record['run_id']}
    }],
    'facts': [
        {'id': 'F_BOUNDARY_RUN_RESULT', 'source_id': 'TEST_BOUNDARY_LOCAL',
         'line': result_line, 'authorship': 'manual',
         'statement': 'O relatório registra quatro testes aprovados e zero falhas.'},
        {'id': 'F_BOUNDARY_LIMIT_PANIC', 'source_id': 'TEST_BOUNDARY_LOCAL',
         'line': panic_line, 'authorship': 'manual',
         'statement': 'O teste no limite foi aprovado esperando pânico.'}
    ]
}
with Path('aula16-dossie.json').open('x', encoding='utf-8') as destination:
    json.dump(dossier, destination, ensure_ascii=False, indent=2)
    destination.write('\n')
print('Dossiê criado: aula16-dossie.json')
PY
```

Abra o arquivo e leia as duas afirmações junto às linhas. Se uma asserção falhar,
examine a saída do teste; mudanças de formato exigem revisar a localização antes
de produzir o dossiê. Não remova verificações para afirmar um resultado ausente.

Na raiz, valide e exporte as duas fichas. O comando lê as fontes, cria
`aula16-selecao.json` e pode compilar em `target`:

```bash
cargo run --locked --bin validate_evidence -- aula16-dossie.json --fact F_BOUNDARY_RUN_RESULT --fact F_BOUNDARY_LIMIT_PANIC --context 1 --output aula16-selecao.json
```

Observe uma fonte, duas fichas, zero referências inválidas e
`capture_and_source_match` nas seleções. O ID da fonte local difere do `run_id`.
Use outro nome se o destino existir.

## Validação e evidência histórica

Na raiz, execute os testes do validador; usam `target` e fixtures temporárias:

```bash
cargo test --locked --bin validate_evidence
```

A etapa original teve **72 testes aprovados**, incluindo ligação errada,
ID/hash incorretos, cópia adulterada e tipos incompatíveis. A versão atual usa
fixtures isoladas para não depender da permanência da árvore histórica.

O [dossiê original](../experimentos/07-dossie-captura/evidencias.json) e sua
[seleção](../experimentos/07-dossie-captura/selecao.json) tinham uma fonte e duas
fichas, conferidas naquele ambiente. Hoje, a mudança de Cargo.toml impede
reconferir a captura antiga na árvore atual. Isso não refuta o registro nem
justifica mudar seu hash; o exercício acima produz evidência nova.

## Resultado, limites e próxima aula

A ligação local pode ser exportada, mas não autentica execução, não comprova
compilação e não valida semanticamente as fichas. Não há snapshot atômico nem
benchmark nesta etapa. A [Aula 17](17-avaliar-explicacao-captura.md) avalia se o
modelo respeita esses limites ao explicar a consulta.
