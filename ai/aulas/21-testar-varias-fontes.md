# Aula 21 — Testar o reaproveitamento com várias fontes

## Objetivo

Distribuir 100 fichas entre 1, 10 e 100 fontes documentais e verificar identidade,
escopo de reutilização e recusa de ligação inválida.

## Contexto e pré-requisitos

Use Linux/Bash, Python 3 da Aula 16, `aula16-dossie.json` e o validador release
da Aula 20. Execute tudo na raiz. Não há dependências Python adicionais ou Ollama.
As fontes sintéticas têm IDs diferentes, mas apontam para a mesma saída/captura:
isso não representa 100 arquivos únicos nem execuções independentes.

## Previsão histórica e versão atual

O [protocolo original](../experimentos/12-varias-fontes/protocolo.md) esperava
duas conferências por fonte: fase inicial e exportação. A versão atual já
compartilha documentos de captura na exportação, como explicado na Aula 23;
portanto, novas execuções terão contagens diferentes das da tabela histórica.

## Passo a passo: preparar cargas locais

Na raiz, execute o bloco para criar `aula21-cargas` com três cargas válidas e
uma inválida. Ele apenas lê o dossiê local e escreve cópias sintéticas novas;
recusa diretório existente. Não executa o validador:

```bash
python3 - <<'PY'
import copy
import json
from pathlib import Path

base = json.loads(Path('aula16-dossie.json').read_text(encoding='utf-8'))
folder = Path('aula21-cargas')
folder.mkdir()
for count in (1, 10, 100):
    value = copy.deepcopy(base)
    value['id'] = f'AULA21_SYNTHETIC_{count}'
    value['sources'] = []
    value['facts'] = []
    for i in range(count):
        source = copy.deepcopy(base['sources'][0])
        source['id'] = f'SYNTHETIC_SOURCE_{i}'
        value['sources'].append(source)
    for i in range(100):
        fact = copy.deepcopy(base['facts'][0])
        fact['id'] = f'BENCH_{i}'
        fact['source_id'] = value['sources'][i % count]['id']
        value['facts'].append(fact)
    (folder / f'input-{count}.json').write_text(
        json.dumps(value, ensure_ascii=False, indent=2), encoding='utf-8')
    if count == 10:
        invalid = copy.deepcopy(value)
        invalid['sources'][-1]['capture']['run_id'] = 'INVALID_RUN_ID'
        (folder / 'input-invalido.json').write_text(
            json.dumps(invalid, ensure_ascii=False, indent=2), encoding='utf-8')
print('Quatro cargas criadas em aula21-cargas')
PY
```

Abra as cargas e confira IDs diferentes, conteúdo compartilhado e 100 fichas.
A carga inválida altera somente uma ligação em sua cópia.

Agora execute todas as cargas com seleção explícita das 100 fichas. Na raiz,
o bloco cria `aula21-resultados`, exportações e logs; preserva argumentos e códigos:

```bash
python3 - <<'PY'
import json
import os
import subprocess
from pathlib import Path

folder = Path('aula21-resultados')
folder.mkdir()
results = []
for case in ('1', '10', '100', 'invalido'):
    destination = folder / f'export-{case}.json'
    command = ['target/release/validate_evidence',
               f'aula21-cargas/input-{case}.json']
    for i in range(100):
        command += ['--fact', f'BENCH_{i}']
    command += ['--context', '1', '--output', str(destination)]
    run = subprocess.run(command, capture_output=True, text=True,
                         env={**os.environ, 'BIBLIOTECARIO_METRICS': '1'})
    (folder / f'{case}.stdout.txt').write_text(run.stdout, encoding='utf-8')
    (folder / f'{case}.stderr.txt').write_text(run.stderr, encoding='utf-8')
    results.append({'case': case, 'argv': command, 'exit_code': run.returncode})
    if case == 'invalido':
        assert run.returncode == 1 and not destination.exists()
    else:
        assert run.returncode == 0, run.stderr
        selections = json.loads(destination.read_text(encoding='utf-8'))['selections']
        assert len(selections) == 100
        for i, selection in enumerate(selections):
            assert selection['fact_id'] == f'BENCH_{i}'
            assert selection['source']['id'] == f'SYNTHETIC_SOURCE_{i % int(case)}'
            assert selection['source']['capture_validation']['status'] == 'capture_and_source_match'
    print(case, run.returncode)
(folder / 'results.json').write_text(json.dumps(results, indent=2), encoding='utf-8')
PY
```

Observe código 0 nas três cargas e 1 na inválida, sem sua exportação. Leia os
contadores nos arquivos stderr. Na versão atual são 2, 11 e 101 conferências
completas; a investigação histórica abaixo usava outra política.

## Evidência histórica e validação

Os [resultados preservados](../experimentos/12-varias-fontes/resultados.json)
registraram:

| Fichas | Fontes | Conferências | Chamadas de hash | Bytes instrumentados |
| --- | --- | --- | --- | --- |
| 100 | 1 | 2 | 12 | 41.880 |
| 100 | 10 | 20 | 120 | 418.800 |
| 100 | 100 | 200 | 1.200 | 4.188.000 |

As contagens de ligação e leituras de registro eram 2 × fontes; leitura do
conteúdo, 3 × fontes. A carga de uma fonte preservou o hash anterior, e uma
execução sem métricas teve bytes iguais e stderr vazio. A ligação inválida foi
recusada na validação inicial, não após preencher o mapa de exportação.
O capturador histórico retornou 0 porque registrou corretamente a falha do filho.

## Problemas comuns e limites

Se uma asserção falhar, leia o stderr da carga antes de interpretar resultados.
Uma fonte local que mudou invalida também suas cópias sintéticas. Não há medição
de tempo nesta aula, e bytes lógicos não representam disco físico. O experimento
original não alterou Rust nem repetiu a suíte unitária: exercitou a CLI.

## Resultado da aula e próxima aula

A quantidade de fontes documentais é uma variável diferente da quantidade de
fichas. A [Aula 22](22-separar-fonte-e-captura.md) separa as responsabilidades
da fonte e do documento de captura antes de compartilhar sua conferência.
