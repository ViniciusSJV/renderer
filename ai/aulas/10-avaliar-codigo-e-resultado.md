# Aula 10 — Avaliar uma consulta com código e resultado

## Objetivo

Preparar uma consulta com código e resultado e aplicar uma rubrica que examine
conclusões, referências e premissas, sem adaptar os critérios à resposta.

## Contexto e pré-requisitos

A Aula 9 reuniu fichas de procedimento e resultado. Use o mesmo dossiê, Cargo,
o editor e, para nova geração manual, o Ollama da Aula 1. Todos os comandos
partem da raiz. Nenhuma implementação Rust nova é necessária.

## Preparação e passo a passo

Leia [pergunta-codigo-e-resultado.txt](../experimentos/03-tuplas/pergunta-codigo-e-resultado.txt).
Ela pergunta sobre chamadas e macro, execução, alcance da conferência e limites
de correção e desempenho. Gere uma consulta nova; o comando lê as entradas,
cria a exportação e pode compilar em `target`, sem enviar ao modelo:

```bash
cargo run --locked --bin validate_evidence -- ai/experimentos/03-tuplas/evidencias-execucao-identificada.json --fact F_VECTOR_TEST_X --fact F_VECTOR_TEST_Y --fact F_VECTOR_TEST_PASSED --context 4 --question ai/experimentos/03-tuplas/pergunta-codigo-e-resultado.txt --output aula10-consulta.json
```

Abra o arquivo e confira as três fichas e o texto da pergunta. Um destino
existente exige outro nome; divergência nas fontes impede tratar a exportação
como conferida. A [consulta histórica](../experimentos/03-tuplas/consulta-codigo-e-resultado-pergunta.json)
permanece disponível para leitura independente de uma nova geração.

Antes da resposta, copie somente os critérios da tabela seguinte para uma
rubrica nova no editor. Mantenha o gabarito separado da consulta. No PowerShell,
inicie uma sessão do modelo; o comando gera texto sem modificar o projeto:

```powershell
ollama run renderer-analyst
```

Envie o conteúdo completo de `aula10-consulta.json`. Salve a primeira resposta
sem solicitar correção antes da avaliação. Registre configuração conhecida e
qualquer corte observado; condições não verificadas devem permanecer desconhecidas.

## Avaliação histórica

A [rubrica preenchida](../experimentos/03-tuplas/avaliacao-codigo-e-resultado.json)
contém a resposta, o SHA-256 da consulta e as justificativas. Cada critério
integralmente atendido vale um ponto; parcial vale zero:

| Critério prévio | Resultado histórico | Pontos |
| --- | --- | --- |
| C1: chamadas sem promover valores esperados a comprovação | Acerta chamadas, mas conclui inicialização correta. | 0 |
| C2: reconhecer implementação ausente da macro | Reconhece a lacuna, sem inventar mecanismo ou tolerância. | 1 |
| C3: atribuir resultado à execução identificada | Troca RUN_VECTOR_1 por TEST_VECTOR_1 e omite código 0. | 0 |
| C4: referências que sustentam a afirmação | Usa aprovação para sustentar correção do construtor. | 0 |
| C5: alcance da conferência | Lista quatro campos e nega autenticação ou comprovação histórica. | 1 |
| C6: limites de generalização e desempenho | Não afirma correção universal ou ganho. | 1 |

**Resultado: 3/6.** C1 e C4 examinam aspectos relacionados, não medições
independentes. C2 reconhece uma lacuna, sem validar a conclusão posterior.
A menção a testes mais amplos como necessários não foi interpretada como
suficiência para garantir correção universal. Não houve desconto por extensão
ou escapes Markdown.

## Conceito central: evidência não é conclusão

O modelo reconheceu ausência da macro e falta de associação histórica comprovada,
mas concluiu inicialização correta porque o teste passou. O rótulo INFERENCE
não supre essas premissas. Uma formulação sustentada é:

> SRC_TUPLE mostra chamadas para x e 1.4 e para y e 8.9, referidas por
> F_VECTOR_TEST_X e F_VECTOR_TEST_Y. TEST_VECTOR_1 registra aprovação em
> RUN_VECTOR_1, com código 0, referida por F_VECTOR_TEST_PASSED. O material
> não fornece a macro nem comprova a associação histórica entre código e execução.

## Validação e limites

Confronte cada frase da resposta com as fichas efetivamente enviadas. Um ID
existente pode não sustentar a conclusão. Fonte documental e execução têm IDs
diferentes; a avaliação deve conferir ambos.

A coleta histórica foi manual: modelo efetivo, sessão nova e ausência de
truncamento não foram confirmados. A nota não é taxa geral de acerto e não é
comparável causalmente aos 2/5 da Aula 7, que usou outra pergunta e rubrica.
Nesta etapa histórica não houve alteração de Rust/Modelfile, nova execução do
teste do vetor, suíte Rust ou benchmark.

## Resultado da aula e próxima aula

Consulta, critérios e resposta podem ser analisados separadamente. A
[Aula 11](11-investigar-assert-equivalent.md) investiga a implementação de
`assert_equivalent!`, uma premissa ausente desta consulta, sem revisar sua nota
retroativamente.
