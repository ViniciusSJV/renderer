# Registro da consulta manual-001

**Para executar uma consulta nova, siga somente o [TESTME da raiz](../../../TESTME.md).**
Este arquivo descreve a tentativa histórica, sem repetir o roteiro operacional.

## Preparação preservada

Em 24/09/2026, a edição foi conferida: 63 fontes, 908 símbolos e zero diagnósticos.
O primeiro trecho completo de `search.json` foi escolhido explicitamente:
`Camera::ray_from_pixel`, `src/camera.rs:75–89`, identificado como E1. Seus bytes
foram comparados ao snapshot. Demais resultados/grafo não entraram no envio.

- `prompt.txt` e `query-ollama.json`: mesmo texto UTF-8, 2582 bytes.
- `request-ollama.json`: prévia de `/api/generate`, modelo `renderer-analyst:latest`,
  `stream=false`. O cliente preservou o corpo efetivamente enviado em `ollama-01/`.
- `graph.json`, `search.json`, configurações e edição: material de origem preservado.

## Resultado preservado

`ollama-01` teve HTTP 200, `completed`, `done_reason=stop` e 12,223 s no cliente.
Hashes/tamanhos e correspondência entre prompt, query, retorno e texto foram
conferidos. Leia a [avaliação retrospectiva](EVALUATION-OLLAMA-01.md): localização
correta, explicação parcialmente correta, com esclarecimento da ordem `pixel - origin`.

O transporte não reconferiu fontes (`evidence_rechecked=false`); dossiê/seleção
automatizados estão nulos. Houve seleção descrita na query e conferência manual
na preparação. Não é bundle PreparedQuery nem fechamento dos engines.

`result.json` e a resposta original permanecem intactos. A avaliação posterior
está em documento separado; não substitui uma rubrica pré-registrada. Para repetir,
use outra pasta e outro request_id conforme o TESTME, sem sobrescrever esta tentativa.

# Iniciar ollama

## Na raiz do renderer:

```powershell
cd C:\Users\beatl\Documents\Projetos\renderer

ollama show renderer-analyst:latest --parameters
ollama show renderer-analyst:latest --modelfile
```

## Para aplicar o seu arquivo local ao modelo:

```powershell
ollama create renderer-analyst:latest -f ai/ollama/Modelfile
```

## Enviar pela API

O cliente existente faz o POST em /api/generate e preserva requisição, resposta e registros. Documentação da API

```powershell
cargo run --locked --bin send_ollama -- ai/consultas/manual-001/query-ollama.json http://localhost:11434/api/generate renderer-analyst:latest 300000 1048576 MANUAL_001_OLLAMA_01 ai/consultas/manual-001/ollama-01
```
O timeout é de cinco minutos. A pasta ollama-01 precisa ser nova.

## Ler o resultado

```powershell
Get-Content -Raw -Encoding UTF8 ai/consultas/manual-001/ollama-01/response.txt
Get-Content -Raw -Encoding UTF8 ai/consultas/manual-001/ollama-01/result.json
```