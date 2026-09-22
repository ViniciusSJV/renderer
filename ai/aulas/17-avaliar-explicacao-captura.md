# Aula 17 — Avaliar a explicação da captura

## Objetivo

Avaliar se a explicação distingue fonte, execução, conferência e autenticação,
com critérios definidos antes da resposta.

## Contexto e pré-requisitos

Use `aula16-dossie.json` e o ambiente Linux das aulas 13–16 para exportar.
Para geração manual, use o Ollama configurado no Windows na Aula 1. A consulta
pode ser copiada como texto entre os ambientes; isso não transfere o serviço.
Todos os comandos abaixo partem da raiz do clone no ambiente indicado.

## Preparação e passo a passo

Leia a [pergunta](../experimentos/08-avaliacao-captura/pergunta.txt) e os
[critérios históricos](../experimentos/08-avaliacao-captura/criterios.json).
No editor, prepare uma rubrica nova com seis critérios: identidade, resultado e
pânico esperado, referências, alcance da conferência, autenticidade e limites
de correção/desempenho. Adapte os IDs esperados à captura local antes da resposta.
Cada critério integralmente atendido vale um ponto; parcial vale zero.

Na raiz Linux, gere a consulta nova. O comando lê e confere o dossiê local,
cria a exportação e pode compilar em `target`; não envia ao modelo:

```bash
cargo run --locked --bin validate_evidence -- aula16-dossie.json --fact F_BOUNDARY_RUN_RESULT --fact F_BOUNDARY_LIMIT_PANIC --context 1 --question ai/experimentos/08-avaliacao-captura/pergunta.txt --output aula17-consulta.json
```

Observe uma fonte, duas fichas e ausência de referências inválidas. Abra a
consulta e confira fonte, execução e resultado da conferência. Caso o destino
exista, use outro nome. Fonte divergente deve ser investigada antes de continuar.

No PowerShell da raiz Windows, abra uma sessão nova; o comando carrega o modelo
e permite geração, sem modificar as fontes:

```powershell
ollama run renderer-analyst
```

Envie o conteúdo completo de `aula17-consulta.json`, sem o gabarito. Preserve
a primeira resposta em documento novo, antes de pedir revisão. Registre cortes,
erros e configuração conhecida. Um caminho de arquivo não entrega seu conteúdo
ao modelo; contexto e saída configurados não garantem processamento integral.

## Avaliação histórica

A [consulta original](../experimentos/08-avaliacao-captura/consulta.json) usou
o dossiê histórico, não a captura local criada neste roteiro. A
[resposta preservada](../experimentos/08-avaliacao-captura/resposta.txt) recebeu
**3/6**, sem alteração posterior dos critérios:

| Critério | Pontos | Justificativa |
| --- | --- | --- |
| C1 — Identidades | 1 | Distingue fonte e execução. |
| C2 — Resultado/pânico | 0 | Nega que o relatório permita confirmar o pânico esperado relatado. |
| C3 — Referências | 0 | Omite os dois IDs das fichas. |
| C4 — Conferência/comando | 1 | Enumera conferências e distingue seus resultados. |
| C5 — Associação/limites | 0 | Omite ausência de validação semântica da ficha. |
| C6 — Generalização | 1 | Não conclui correção universal ou ganho. |

O relatório pode registrar aprovação com pânico esperado sem que o Bibliotecário
autentique a execução. A falta de autenticação não apaga a informação declarada.
`should panic ... ok` significa rejeição esperada, não aceitação do par.
Dizer que não existe nenhuma informação sobre execução também seria amplo demais:
há informações registradas, com limites de comprovação.

## Validação e limites

Confira a resposta inteira, inclusive contradições entre itens. Um resumo correto
não compensa uma extrapolação posterior. O hash do texto salvo identifica esse
arquivo, não os bytes da sessão manual. Na coleta histórica, configuração efetiva
e ausência de truncamento não foram confirmadas.

A nota da Aula 10 continua 3/6, mas igualdade numérica não significa qualidade
igual: perguntas e critérios diferem. Não houve benchmark, alteração de Rust
ou nova execução de testes na avaliação histórica. A consulta local deste roteiro
exige avaliação própria; ela não herda a nota da resposta antiga.

## Resultado da aula e próxima aula

A explicação passa a ser examinada com critérios sobre cada camada de evidência.
A [Aula 18](18-medir-custo-conferencia.md) mede o custo total da CLI antes de
atribuir desempenho a operações isoladas.
