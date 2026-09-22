# Aula 7 — Identidade, limites e uma consulta avaliável

## Objetivo

Exportar uma pergunta com evidências identificadas e questões abertas e avaliar
uma resposta com critérios definidos antes de recebê-la.

## Contexto e pré-requisitos

As aulas 5–6 prepararam seleção, contexto e metadados. Falta identificar o dossiê
e distinguir pedido de material consultado. Use Cargo e o editor; Ollama só é
necessário para a etapa manual. Execute os comandos na raiz do clone.

## Conceitos e implementação

Em [validate_evidence.rs](../../src/bin/validate_evidence.rs), `Evidence` contém
`id: Option<String>` e `unknowns: Option<Vec<String>>`. A exportação os transporta
como `evidence_id` e `evidence_unknowns`.

O ID do conjunto não é o ID da ficha. Também não há unicidade global: dois
arquivos podem declarar o mesmo ID. Ausência de ID permanece `null`.
As questões abertas pertencem ao dossiê inteiro e são copiadas sem filtro;
`null` e lista vazia são distintos e nenhum comprova ausência de lacunas.

`--question ARQUIVO` lê a pergunta e exige um destino de exportação. O resultado
separa `question`, `evidence` e `instructions`. Pergunta vazia é recusada.
Instruções orientam o modelo, mas não garantem obediência nem validam a semântica.

## Passo a passo

Leia [pergunta-vector-x.txt](../experimentos/03-tuplas/pergunta-vector-x.txt).
Na raiz, gere uma consulta nova; o comando lê pergunta, dossiê e fontes, grava
a exportação e pode compilar em `target`. Não envia HTTP:

```bash
cargo run --locked --bin validate_evidence -- ai/experimentos/03-tuplas/evidencias.json --fact F_VECTOR_TEST_X --context 4 --question ai/experimentos/03-tuplas/pergunta-vector-x.txt --output aula07-consulta.json
```

Abra `aula07-consulta.json` no editor. Confira a pergunta, F_VECTOR_TEST_X,
SRC_TUPLE, contexto e limites. TEST_VECTOR_1 existe no acervo, mas não foi
selecionado: existência no dossiê não significa inclusão na consulta.

Antes da resposta, registre uma rubrica nova baseada nos cinco critérios da
tabela abaixo, sem copiar as notas históricas. No PowerShell da raiz, abra uma
sessão nova do modelo; o comando gera texto, sem editar fontes:

```powershell
ollama run renderer-analyst
```

Envie apenas o conteúdo completo de `aula07-consulta.json`. Preserve a primeira
resposta em arquivo novo no editor; não substitua os artefatos anteriores.
A exportação atual omite o booleano em fonte Rust, enquanto a consulta histórica
abaixo ainda o continha. São entradas diferentes e exigem avaliações distintas.

## Avaliação histórica

A [consulta original](../experimentos/03-tuplas/consulta-vector-x-pergunta.json)
e a [avaliação](../experimentos/03-tuplas/avaliacao-consulta-vector-x.json)
preservam entrada, hash, resposta e justificativas. Um critério integralmente
atendido vale um ponto; parcial vale zero:

| Critério | Resultado histórico |
| --- | --- |
| C1: identificar argumentos 1.4, 8.9 e 5.1 | 1 |
| C2: descrever chamada sem presumir interior da macro | 0, parcial |
| C3: citar F_VECTOR_TEST_X e SRC_TUPLE | 0 |
| C4: reconhecer ausência de resultado e não alegar execução própria | 1 |
| C5: não promover metadados a prova nem extrapolar | 0 |

**Resultado: 2/5.** A resposta reconheceu a falta de resultado, mas interpretou
`executed=false` como prova de que o teste nunca rodou. Também atribuiu à macro
inicialização correta sem conhecer sua implementação e citou campos em vez de
IDs. Extensão da resposta não foi penalizada porque a pergunta não a limitava.
Uma possível influência do SYSTEM didático permaneceu hipótese.

## Validação e limites

Na raiz, execute os testes de preservação da pergunta, identidade e limites.
Cargo grava em `target`; os testes não consultam o modelo:

```bash
cargo test --locked --bin validate_evidence
```

A etapa original teve **43 testes aprovados**; uma pergunta vazia foi recusada
antes da exportação. Na árvore atual, verifique ausência de falhas, não o total
histórico. Se faltar uma fonte na resposta, examine primeiro a seleção enviada.

O hash identifica a consulta preservada, sem autenticar modelo, parâmetros ou
histórico da sessão. A nota é manual e não mede confiabilidade geral. Não houve
benchmark. Dizer que uma obra descreve código é diferente de dizer que esse
código nunca foi executado.

## Resultado da aula e próxima aula

A consulta distingue pergunta, evidências e instruções. A
[Aula 8](08-codigo-nao-e-execucao.md) resolve a ambiguidade do booleano,
separando fonte de código e registro identificado de execução.
