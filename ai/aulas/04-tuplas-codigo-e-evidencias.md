# Aula 4 — Uma obra real: tuplas, código e evidências

## Objetivo

Relacionar uma afirmação sobre `Tuple::vector` a uma fonte identificada por hash
e distinguir essa leitura de um resultado de teste registrado.

## Contexto e pré-requisitos

Use o ambiente Rust e Cargo da Aula 3 e o editor, na raiz do clone.
As tuplas são o primeiro assunto do ray tracer estudado nesta sequência.
Não é necessário reconstruir o renderer: o código existente é o material de
investigação. Os exercícios usam a árvore atual; artefatos anteriores continuam
identificados como históricos.

## Conceitos e implementação

Em [src/tuple.rs](../../src/tuple.rs), pontos têm `w = 1` e vetores têm `w = 0`.
O [dossiê](../experimentos/03-tuplas/evidencias.json) copia as linhas desse
arquivo como SRC_TUPLE. F_VECTOR_W aponta para o construtor de vetores,
na linha 18 da cópia, e afirma que ele define `w` como `0.0`.

O texto foi extraído do arquivo e a ficha foi escrita manualmente. Extrair
linhas não é extrair automaticamente fatos verdadeiros.

Um SHA-256 identifica bytes. O validador usa `sha2`, já declarada no manifesto,
para comparar `path` e `sha256` com o arquivo atual. Depois compara a lista
`lines` com esse arquivo: um hash externo correto não aprova uma cópia adulterada.
Espaços e terminações de linha afetam os bytes sem necessariamente alterar o
comportamento do programa.

`path` e `sha256` devem aparecer juntos. Fontes sem ambos continuam aceitas sem
conferência externa. `git_commit` e `matches_commit` não são autenticados pelo
validador. Um commit identifica a base versionada, não todas as alterações locais.

## Passo a passo

Na raiz, execute apenas o teste dos componentes do vetor. Cargo grava artefatos
em `target`; não cria um relatório histórico nem consulta Ollama:

```bash
cargo test --locked --lib tuple::tests_tuple::vector_does_fill_properties -- --exact
```

Observe um teste aprovado. A quantidade de testes filtrados pode variar com a
versão. O registro original teve um aprovado e 200 filtrados; isso descreve
somente aquela seleção, não a suíte inteira.

Agora valide o dossiê, ainda na raiz. O comando lê arquivos e imprime a
conferência; não atualiza hashes nem exporta dados:

```bash
cargo run --locked --bin validate_evidence -- ai/experimentos/03-tuplas/evidencias.json
```

No material correspondente, são duas fontes e sete fichas sem referências
inválidas. Se o arquivo atual divergir, examine o diagnóstico e a cópia preservada.
Não reescreva um hash antigo para apresentar uma nova versão como a mesma fonte.

## Código, execução e explicação

O [relatório do vetor](../experimentos/03-tuplas/teste-vector.txt) é outra obra,
TEST_VECTOR_1. Ele registra comando, horários, ambiente parcial, hashes, saída e
código de término. F_VECTOR_TEST_PASSED aponta para a aprovação relatada.
Conferir seu hash não reexecuta o teste nem autentica quem o executou.

| Ficha | Informação no código ou relatório |
| --- | --- |
| F_VECTOR_W | Construtor define w como 0.0. |
| F_VECTOR_TEST_INPUT | Teste constrói vetor com 1.4, 8.9 e 5.1. |
| F_VECTOR_TEST_X | Chamada de `assert_equivalent!` para x e 1.4. |
| F_VECTOR_TEST_Y | Chamada para y e 8.9. |
| F_VECTOR_TEST_Z | Chamada para z e 5.1. |
| F_VECTOR_TEST_W | Chamada para w e 0. |
| F_VECTOR_TEST_PASSED | Aprovação registrada do teste selecionado. |

A chamada da macro mostra operandos, mas ainda não explica seu mecanismo.
Sua implementação será investigada na Aula 11. Um resultado aprovado não
substitui o procedimento que o produziu.

## Experimento de explicação

Leia [prompt-corpo-teste.txt](../experimentos/03-tuplas/prompt-corpo-teste.txt).
Para repetir a consulta manual, inicie o modelo no PowerShell da raiz; ele pode
gerar texto e carregar o modelo, mas não altera fontes:

```powershell
ollama run renderer-analyst
```

Envie o conteúdo completo do prompt em uma sessão nova. Antes de receber a
resposta, registre os critérios: descrever argumentos e comparações; separar
código e resultado; evitar generalização. O prompt também exige IDs, até quatro
frases e nenhuma dedução do funcionamento da macro somente pelo nome.

A avaliação histórica encontrou descrição correta dos argumentos, mas omissão
de IDs, excesso de frases e conclusão de inicialização correta sem a macro.
Consultas anteriores sem o corpo do teste também produziram a suposição errada
de que x, y e z eram zerados. A informação ausente foi então acrescentada;
revisões sucessivas não foram tratadas como prova de confiabilidade geral.

## Validação e limites

Na raiz, execute a suíte do validador. Ela grava artefatos de teste em `target`
e usa arquivos temporários; não modifica os dossiês:

```bash
cargo test --locked --bin validate_evidence
```

Na implementação desta etapa, **26 testes passaram**, incluindo arquivo alterado,
cópia adulterada e metadados incompletos. A árvore atual inclui testes posteriores.
Uma nova resposta deve receber avaliação própria, sem herdar a nota histórica.
Não houve benchmark nem demonstração de que JSON melhora a qualidade do modelo.

## Resultado da aula e próxima aula

Código descreve um procedimento; execução registra uma observação; o modelo
produz uma explicação sujeita a avaliação. A
[Aula 5](05-selecao-e-contexto.md) seleciona uma ficha por ID e exporta seu trecho
com contexto, fortalecendo a preparação das entradas antes de ajustar o modelo.
