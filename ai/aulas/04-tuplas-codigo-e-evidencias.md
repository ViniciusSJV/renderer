# Aula 4 — Uma obra real: tuplas, código e evidências

## Conceito: seguir o livro com o Bibliotecário

Vamos acompanhar *The Ray Tracer Challenge* na ordem do livro, usando o código
já implementado como material de investigação. Começamos por tuplas, pontos e
vetores. Não precisamos reconstruir os fundamentos a cada exercício: o foco
é aprender a transformar o trabalho existente em evidências rastreáveis.

A câmera teria ligação com o exemplo anterior do mutex, mas o Bibliotecário
não depende dela. Nossa primeira obra real passou a ser `src/tuple.rs`.

Na representação do projeto, pontos têm w = 1 e vetores têm w = 0. Consultamos
os construtores e executamos separadamente seus testes de componentes; ambos
passaram. Depois concentramos o dossiê no construtor de vetores.

## Implementação: guardar uma edição da obra

O [dossiê de tuplas](../experimentos/03-tuplas/evidencias.json) contém uma cópia
integral das linhas de `src/tuple.rs`, identificada como SRC_TUPLE. A ficha
F_VECTOR_W afirma que `Tuple::vector` define w como 0.0 e aponta para a linha 18.

O conteúdo foi extraído do arquivo; a afirmação foi escrita manualmente. Extrair
texto não é gerar nem comprovar fatos automaticamente.

Registramos o caminho, o commit consultado e o SHA-256 dos bytes do arquivo.
Na criação do dossiê, conferimos que o conteúdo correspondia ao arquivo no commit.
Esse registro é uma fotografia: uma edição nova não atualiza a cópia antiga.

O Bibliotecário ganhou duas verificações, usando a dependência `sha2`:

1. Ler `path`, calcular o SHA-256 e comparar com o valor registrado.
2. Comparar as linhas do arquivo com a cópia em `lines`.

A segunda verificação impede que uma cópia alterada passe apenas por carregar
o hash correto do arquivo externo. Espaços e terminações de linha fazem parte
dos bytes: diferenças de hash não significam necessariamente mudanças de comportamento.

As fontes antigas sem path e sha256 continuam aceitas, com uma mensagem indicando
que não houve conferência externa. Os dois campos devem aparecer juntos quando
usados. Os caminhos são relativos à pasta de execução; usamos a raiz do projeto.
O programa ainda não verifica git_commit ou matches_commit, nem autentica a origem
do documento. Se o arquivo mudar, uma ficha pode continuar historicamente válida,
mas precisa ser reconferida para descrever a versão atual.

## Uma segunda obra: o registro do teste

Executamos:

```bash
cargo test --offline --lib tuple::tests_tuple::vector_does_fill_properties -- --exact
```

O [registro preservado](../experimentos/03-tuplas/teste-vector.txt) contém comando,
horários, diretório, commit, estado do Git, versão do Rust, hashes de arquivos,
saída combinada e código de término. Havia alterações locais: HEAD identifica
uma base, não todo o estado executado. O registro não é uma captura completa
do ambiente nem permite reconstruí-lo sozinho.

O resultado foi um teste aprovado, zero falhas e 200 testes filtrados.
A fonte TEST_VECTOR_1 guarda esse registro. F_VECTOR_TEST_PASSED aponta para a
linha que informa a aprovação nessa execução específica.

Temos agora duas obras com papéis diferentes:

| Obra | O que podemos consultar |
| --- | --- |
| SRC_TUPLE | O código que implementa e testa as tuplas. |
| TEST_VECTOR_1 | O resultado observado de uma execução selecionada. |

Conferir o hash do registro não reexecuta o teste nem comprova sua autoria.

## Teste com o Qwen: resultado não descreve o procedimento

Enviamos o construtor e o resultado do teste ao Qwen. A resposta identificou
w = 0, mas sua explicação afirmou indevidamente que x, y e z também eram zerados.
Também descreveu verificações internas do teste sem ter recebido seu corpo.

Pedimos revisão. O Qwen corrigiu a interpretação de x, y e z, reconheceu a falta
do corpo do teste, mas voltou a afirmar que as componentes estavam preenchidas
corretamente. A ressalva não removeu a conclusão sem suporte. Também não seguiu
o limite de frases e as referências solicitadas.

Em vez de repetir a correção até obter uma resposta desejada, fornecemos a
informação ausente: o procedimento executado.

## Acrescentar as fichas do procedimento

Adicionamos cinco fichas, sem alterar o código do renderer:

| Ficha | Trecho registrado |
| --- | --- |
| F_VECTOR_TEST_INPUT | Construção com argumentos 1.4, 8.9 e 5.1. |
| F_VECTOR_TEST_X | Chamada de assert_equivalent! para x e 1.4. |
| F_VECTOR_TEST_Y | Chamada de assert_equivalent! para y e 8.9. |
| F_VECTOR_TEST_Z | Chamada de assert_equivalent! para z e 5.1. |
| F_VECTOR_TEST_W | Chamada de assert_equivalent! para w e 0. |

Cada ficha aponta para uma linha de SRC_TUPLE. O dossiê passou a conter duas
fontes e sete fichas. O validador conferiu hashes, cópias e referências sem erros.
Não repetimos o teste para adicionar essas fichas: usamos a execução preservada.

## Consulta com o corpo do teste

Preparamos [prompt-corpo-teste.txt](../experimentos/03-tuplas/prompt-corpo-teste.txt)
com o corpo do teste, as fichas, o resultado e três perguntas. A orientação foi
usar uma conversa nova, evitando depender das correções anteriores; não houve
captura automática do histórico para verificar essa condição.

Critérios definidos antes da resposta:

- Descrever argumentos e comparações corretamente.
- Separar leitura do código de resultado observado.
- Não generalizar para todas as entradas e operações.

O prompt também exigia citar IDs, responder em até quatro frases e não deduzir
o funcionamento de assert_equivalent! apenas pelo nome.

### Avaliação da resposta recebida

O Qwen descreveu corretamente os argumentos e comparações, identificou a execução
aprovada e negou que isso demonstrasse correção de todas as operações. Porém:

- Omitiu os IDs das fichas.
- Excedeu o limite de quatro frases.
- Falou em inicialização correta sem ter recebido a implementação da macro.
- Sugeriu testes mais abrangentes como caminho para demonstrar correção de todas
  as entradas; ampliar testes aumenta a evidência, mas não garante correção universal.

A avaliação é manual, baseada nas respostas trazidas à aula. Não houve captura
automática dessas conversas nem comparação controlada de modelos.

## Medição e limites

Ao acrescentar a conferência de versões, executamos os testes do binário:
**26 passaram**, incluindo arquivo alterado, cópia alterada e metadados incompletos.
Também executamos o dossiê com sete fichas: nenhuma referência inválida.
Esses números registram verificações de correção, não desempenho.

Não fizemos benchmarks do renderer ou do Qwen, não executamos a suíte completa
do renderer e não demonstramos que JSON melhora a qualidade das respostas.

O exercício mostra por que precisamos separar:

```text
Código → descrição do procedimento
Execução → resultado observado
Qwen → explicação sujeita a avaliação
```

## Decisão: fortalecer o Graph Engine primeiro

Queremos melhorar o Qwen, mas corrigir repetidamente uma resposta não demonstra
que o modelo acertará uma consulta nova. Antes de ajustar o SYSTEM, precisamos
de entradas reproduzíveis e critérios de avaliação estáveis.

Vamos manter o Modelfile atual e trabalhar no Bibliotecário:

1. Selecionar fichas por ID no programa.
2. Reunir seus trechos e referências a partir do dossiê validado.
3. Guardar o material de consulta e definir os critérios de avaliação.
4. Depois comparar ajustes do LLM Engine usando entradas controladas, incluindo
   exemplos não usados para orientar os ajustes.

Ainda não precisamos de um banco de grafos. Rust e JSON permitem explorar as
relações ficha → fonte → trecho → execução → parecer. O Qwen continuará fazendo
parte das aulas, com envio manual enquanto a integração não for implementada.

O próximo passo pequeno será selecionar uma ficha por ID e obter seu trecho
pelo programa. A geração de consultas virá a partir dessa seleção, mantendo
explícitos os limites do contexto fornecido.
