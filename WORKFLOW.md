# Workflow: renderer como consumidor do Librarian

O fluxo, os artefatos e o formato do perfil estão no
[WORKFLOW do Librarian](../librarian/WORKFLOW.md). O renderer não executa etapas
próprias: `PLAYME.cmd`/`PLAYME.ps1` apenas chamam
`../librarian/tools/playme/PLAYME.ps1` com [playme.json](playme.json).

| O que o renderer mantém | Arquivo |
| --- | --- |
| Perfil da execução | [playme.json](playme.json) |
| Fontes do acervo | [ai/acervo/renderer-sources.json](ai/acervo/renderer-sources.json) |
| Vocabulário português → código | [ai/acervo/renderer-lexicon.json](ai/acervo/renderer-lexicon.json) |
| Modelo local | [ai/ollama/Modelfile](ai/ollama/Modelfile) (`renderer-analyst`) |
| Política de explicação | [ai/playme/instructions.json](ai/playme/instructions.json) |
| Critérios prévios | [ai/playme/criteria-before.md](ai/playme/criteria-before.md) |
| Consultas geradas | `ai/consultas/` (ignorada pelo Git; use `git add -f` para preservar uma avaliada) |

As consultas anteriores foram removidas da árvore em 4158ecb e continuam no
histórico. A [primeira tentativa manual avaliada](https://github.com/ViniciusSJV/renderer/blob/a6e56ba19213d879f5de6e3e07f8a75ef84fe569/ai/consultas/manual-001/EVALUATION-OLLAMA-01.md)
teve HTTP 200 e hashes conferidos, com avaliação retrospectiva parcialmente correta.
