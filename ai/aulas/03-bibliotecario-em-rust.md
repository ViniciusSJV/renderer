# Aula 3 — Construindo o Bibliotecário em Rust

## Conceito: localizar não é comprovar

Vamos imaginar o laboratório como uma biblioteca. A inspiração no Bibliotecário
de *Snow Crash* nos ajuda a dar nomes concretos às peças que estamos construindo:
queremos localizar informações, preservar suas origens e distinguir o que sabemos
do que ainda precisa ser investigado.

| Na biblioteca | No programa |
| --- | --- |
| Dossiê | Um documento JSON com fontes, fichas e questões abertas. |
| Obra | Uma `Source`: neste exemplo, um pseudocódigo. |
| Código da obra | `source.id`, como S1. |
| Trechos numerados | As entradas de `source.lines`. |
| Ficha | Um `Fact`, com uma afirmação e uma referência. |
| Código da ficha | `fact.id`, como F4. |
| Afirmação da ficha | `fact.statement`. |
| Endereço para consulta | `fact.source_id` e `fact.line`. |
| Nota de revisão | Um `Review`, com o parecer do Qwen. |

F4 aponta para a obra S1, trecho 5. O Bibliotecário pode conferir esse endereço
sem decidir se a afirmação da ficha é verdadeira. Essa separação é o fundamento
da aula: **uma referência válida pode acompanhar uma afirmação incorreta**.

O campo `line` conta entradas de `lines` a partir de 1. Não é a numeração física
do arquivo JSON mostrada no editor. Já os índices de um `Vec` começam em zero.

## Implementação: primeiro a ficha, depois o acervo

O programa está em [src/bin/validate_evidence.rs](../../src/bin/validate_evidence.rs).
Começamos construindo uma ficha diretamente na memória:

```rust
struct Fact {
    id: String,
    statement: String,
    source_id: String,
    line: usize,
}
```

`String` guarda o texto pertencente ao valor. `usize` representa um inteiro não
negativo, adequado para tamanhos e índices de coleções. Ele aceita zero, embora
nossa convenção de linhas não aceite: o tipo não substitui a regra de validação.

Depois criamos a obra:

```rust
struct Source {
    id: String,
    lines: Vec<String>,
}
```

`Vec<String>` é uma sequência de textos. `Vec<Source>` passa a ser nosso acervo.
Ter S1 escrito na ficha e na obra não cria uma ligação automática em Rust;
o programa precisa comparar os identificadores.

### Conferir o endereço

A função `validate_reference` recebe a ficha e uma obra por empréstimo, com
`&Fact` e `&Source`. Confere se o ID corresponde e se a linha está no intervalo:

```text
1 ≤ line ≤ quantidade de trechos
```

Só depois acessamos `source.lines[fact.line - 1]`. Isso evita tanto subtrair 1
de zero quanto acessar uma posição além do fim da lista.

A função devolve `Result<(), String>`: `Ok(())` indica que as verificações
passaram; `Err(message)` descreve o problema. O `main` decide como apresentar
a resposta. O Bibliotecário confere; a interface mostra o resultado.

### Procurar no acervo

`find_source` percorre as obras com um `for`, comparando cada ID com o solicitado.
Devolve `Some(source)` quando encontra e `None` quando chega ao fim sem sucesso.

A assinatura usa uma referência com lifetime `'a`: a obra devolvida continua
pertencendo ao acervo emprestado. A busca não copia seu conteúdo.

Para N obras, essa busca faz até N comparações de identificadores. Para F fichas,
repetir a busca pode exigir até F × N comparações. A comparação de textos também
tem um custo que depende do conteúdo; contar comparações não mede tempo real.

### Evitar códigos repetidos

Duas obras com código S1 tornam a referência ambígua. Duas fichas com código F4
fazem o mesmo com uma citação do modelo. Criamos `validate_source_ids` e
`validate_fact_ids`, cada uma com um `HashSet` de identificadores já vistos.

```rust
if !seen.insert(fact.id.as_str()) {
    return Err(format!("Fato duplicado: \"{}\".", fact.id));
}
```

`insert` devolve `true` para um valor novo e `false` para um valor já presente.
O `!` inverte essa resposta. `as_str()` empresta o texto, sem duplicar a String.
O conjunto ainda precisa de memória para sua própria estrutura.

A regra trata identidade: textos iguais com IDs diferentes são permitidos.
Ela também não detecta que uma obra ganhou uma nova edição. Para código real,
precisaremos considerar versões, como commits ou hashes de conteúdo.

## Do documento às estruturas Rust

Primeiro lemos o arquivo usando `fs::read_to_string`. Nesse momento, temos apenas
texto. Depois adicionamos `serde` com a opção `derive` e `serde_json` para
transformá-lo nas estruturas:

```rust
#[derive(Deserialize)]
struct Evidence {
    sources: Vec<Source>,
    facts: Vec<Fact>,
}
```

`serde_json::from_str` faz a desserialização. Campos exigidos pelas estruturas
precisam estar presentes e ter tipos compatíveis. Os campos adicionais, como
`kind`, `executed` e `unknowns`, permanecem no arquivo, mas não são carregados
pelas estruturas atuais. Não estamos validando todo o significado do dossiê.

Uma linha zero pode ser desserializada como `usize` e depois ser rejeitada pelo
validador. Ler, interpretar e validar são etapas distintas.

## Teste: entregar dossiês diferentes

Execute os comandos a partir da raiz do projeto no terminal do Codespaces:

```bash
cargo run --bin validate_evidence
cargo run --bin validate_evidence -- ai/experimentos/02-mutex/evidencias-fonte-ausente.json
cargo run --bin validate_evidence -- ai/experimentos/02-mutex/evidencias-afirmacao-incorreta.json
```

O `--` separa os argumentos do Cargo dos argumentos do programa. Sem um caminho,
o programa usa o dossiê original. Os outros dois são cópias para experimentos,
não substituições do original.

| Dossiê | Alteração | Resultado observado |
| --- | --- | --- |
| [Original](../experimentos/02-mutex/evidencias.json) | Nenhuma | Cinco referências válidas; saída 0. |
| [Fonte ausente](../experimentos/02-mutex/evidencias-fonte-ausente.json) | F4 aponta para S9. | Uma referência inválida; saída 1. |
| [Afirmação incorreta](../experimentos/02-mutex/evidencias-afirmacao-incorreta.json) | F4 diz que B escreve no pixel 999. | Referências válidas; saída 0. |

A terceira entrada passa porque a obra e o trecho existem. Seu trecho fala em
pixel 20, mas o validador estrutural não interpreta a afirmação sobre pixel 999.

O programa continua verificando as fichas quando encontra uma referência
inválida. Conta os problemas e encerra com código 1 se encontrou algum. IDs
repetidos encerram a consulta antes da busca, pois tornam o catálogo ambíguo.
Erros de leitura e desserialização também resultam em código 1.

Código 0 significa sucesso nas verificações implementadas, não verdade dos fatos.

## O parecer do Qwen

Enviamos manualmente uma afirmação e seu trecho ao `renderer-analyst`, no Ollama
local do Windows. A primeira resposta misturou sustentação parcial, contradição
e insuficiência. Isso revelou também uma lacuna na pergunta: o trecho representa
uma operação específica ou toda a tarefa, incluindo operações não mostradas?

Refinamos o escopo:

```text
Avalie somente a operação explicitamente mostrada no trecho.

Afirmação F4:
Na operação de escrita mostrada em S1:5, a tarefa B escreve no pixel 999.

S1:5:
Tarefa B: adquirir M; escrever azul no pixel 20; liberar M.

Escolha um único resultado: SUSTENTA, CONTRADIZ ou INSUFICIENTE.
Justifique em uma frase, sem avaliar afirmações alternativas.
```

O Qwen respondeu `CONTRADIZ`, justificando a diferença entre 20 e 999. A resposta
atendeu a esse teste. Uma amostra não demonstra confiabilidade geral do modelo.

Guardamos a nota em [parecer-f4.json](../experimentos/02-mutex/parecer-f4.json),
com afirmação original, reformulação avaliada, trecho, prompt, resposta e julgamento.
A coleta foi manual; não capturamos automaticamente o histórico completo nem os
parâmetros efetivos da execução.

A nota preserva os dois textos porque o julgamento foi feito sobre a reformulação.
Não podemos atribuí-lo silenciosamente a qualquer frase parecida.

## Conferir a nota de revisão

O programa recebe o parecer como segundo argumento:

```bash
cargo run --bin validate_evidence -- ai/experimentos/02-mutex/evidencias-afirmacao-incorreta.json ai/experimentos/02-mutex/parecer-f4.json
```

`Review` e `ReviewSource` carregam os campos usados na conferência. A função
`validate_review` verifica IDs únicos no dossiê e depois:

1. Procura a ficha indicada.
2. Compara sua afirmação com `original_statement` do parecer.
3. Confere o ID da fonte e o número da linha.
4. Valida o endereço antes de acessar a lista.
5. Compara o trecho da fonte com a cópia guardada no parecer.

O operador `?`, usado nessas chamadas, devolve imediatamente um erro recebido;
se o resultado for `Ok`, a execução continua.

Com o dossiê da afirmação incorreta, P1 passou e o programa terminou com saída 0.
Com o dossiê original, o programa rejeitou P1: F4 tinha outro texto. A saída foi 1.

Essas comparações são textuais e exatas. Elas não verificam a equivalência de
significado entre a afirmação original e sua reformulação, a correção do julgamento,
a autoria da resposta ou a versão histórica dos documentos. O campo `evidence_file`
é descritivo nesta implementação; o programa confere o conteúdo do dossiê recebido,
não a identidade do caminho registrado na nota.

## Medição e explicação do resultado

Ao concluir a implementação, executamos:

```bash
cargo test --bin validate_evidence
```

**Resultado observado: 21 testes passaram.** Eles cobrem referências, IDs,
carregamento do JSON e associação dos pareceres. Também executamos o programa
com os dossiês e conferimos seus códigos de saída, inclusive com casos temporários
contendo múltiplos erros. Esses experimentos temporários não são novos testes
permanentes da suíte.

A quantidade de testes não prova ausência de defeitos. Não medimos desempenho,
não executamos a suíte completa do renderer nesta etapa e não alteramos seu
algoritmo de renderização.

O ciclo que construímos é:

```text
Dossiê → validação estrutural → seleção de afirmação e trecho
                                      ↓
                              Qwen, por envio manual
                                      ↓
                          parecer registrado em arquivo
                                      ↓
                         conferência das correspondências
```

Essa é uma primeira peça da arquitetura. Ainda não há extração automática de
fatos, banco de grafos nem chamada ao Ollama pelo programa Rust.

## Próxima aula: uma obra real

Vamos começar por `src/camera.rs`: ler o arquivo, observar uma operação e criar
uma ficha manual com sua referência. Antes de medir ou propor otimizações,
precisamos entender o trecho e o contexto que sustenta a afirmação.

Manteremos o ritmo: conceito, uma implementação pequena, teste e explicação.
