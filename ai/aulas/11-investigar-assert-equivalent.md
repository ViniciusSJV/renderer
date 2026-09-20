# Aula 11 — Investigar assert_equivalent!

## Conceito

Uma chamada mostra quais expressões são fornecidas à macro. Para entender a
comparação, precisamos seguir as dependências: macro → método
`not_equivalent` → implementação de `equivalent` para o tipo → `EPSILON`.
Essa cadeia fornece premissas sobre o código apresentado. Ela não estabelece
quais bytes foram compilados na execução histórica RUN_VECTOR_1.

## Primeiro passo: localizar a cadeia

Lemos `src/tuple.rs`, `src/equivalent.rs` e `src/lib.rs`:

| Evidência | Local | O que mostra |
| --- | --- | --- |
| F_TUPLE_X_F64 e F_TUPLE_Y_F64 | tuple.rs:6–7 | x e y têm tipo f64. |
| F_MACRO_OPERANDS | equivalent.rs:20 | O match vincula os operandos. |
| F_MACRO_CONDITION e F_MACRO_PANIC | equivalent.rs:22–23 | O ramo chama panic! quando not_equivalent retorna true. |
| F_NOT_EQUIVALENT | equivalent.rs:7 | O método padrão nega equivalent. |
| F_F64_EQUIVALENCE | equivalent.rs:11 | A implementação é para f64. |
| F_ABSOLUTE_TOLERANCE | equivalent.rs:13 | A comparação usa diferença absoluta e < EPSILON. |
| F_EPSILON_IMPORT | equivalent.rs:1 | EPSILON vem da raiz do crate. |
| F_EPSILON_VALUE | lib.rs:1 | O literal declarado é 0.00001, do tipo f64. |

A macro delega a comparação a métodos; a tolerância é da implementação para
f64. Não devemos atribuir essa mesma fórmula a qualquer tipo aceito pela macro.

## Segundo passo: registrar e conferir

Criamos [evidencias-macro.json](../experimentos/03-tuplas/evidencias-macro.json)
a partir de uma cópia do dossiê da execução identificada, preservando o original.
Acrescentamos duas fontes de código, com SHA-256 e linhas completas dos arquivos
atuais, e dez fichas manuais. Não atribuímos execução às fontes de código nem
commit histórico às novas cópias.

O Bibliotecário existente validou **4 fontes e 17 fichas, com 0 referências
inválidas**, e exportou as dez fichas novas com contexto de raio 4 para
[consulta-implementacao-macro.json](../experimentos/03-tuplas/consulta-implementacao-macro.json).
Esse arquivo é uma seleção de evidências, ainda sem pergunta para o Qwen.

Comando usado, a partir da raiz do repositório:

```bash
cargo run --offline --bin validate_evidence -- ai/experimentos/03-tuplas/evidencias-macro.json --fact F_TUPLE_X_F64 --fact F_TUPLE_Y_F64 --fact F_MACRO_OPERANDS --fact F_MACRO_CONDITION --fact F_MACRO_PANIC --fact F_NOT_EQUIVALENT --fact F_F64_EQUIVALENCE --fact F_ABSOLUTE_TOLERANCE --fact F_EPSILON_IMPORT --fact F_EPSILON_VALUE --context 4 --output ai/experimentos/03-tuplas/consulta-implementacao-macro.json
```

O destino já existe; para reproduzir a exportação, use outro caminho.
Para apenas validar o dossiê, omita todos os argumentos após seu caminho.
O comando terminou com código 0. A conferência verifica referências, hashes,
linhas e os campos já previstos do relatório antigo. A interpretação semântica
das fichas continua manual.

## Explicação e limites

Para x e y, a cadeia apresentada exige que a diferença absoluta calculada em
f64 seja estritamente menor que EPSILON. É uma tolerância absoluta: a fórmula
não divide a diferença pela magnitude dos operandos. Uma diferença calculada
igual a EPSILON não satisfaz `<`. A aceitação admite valores distintos dentro
da tolerância; portanto não permite concluir igualdade exata.

Essas conclusões vêm da leitura do código. Nesta aula não executamos novos
casos da macro. Não observamos experimentalmente a fronteira, valores especiais
ou efeitos de arredondamento. Esses pontos podem ser investigados com testes
específicos, mantendo previsão e resultado observado separados.

A implementação agora está disponível no novo dossiê, mas a consulta antiga
continua sem ela. A avaliação da Aula 10 permanece **3/6**, sem revisão retroativa.
RUN_VECTOR_1 ainda não tem associação histórica comprovada com o conjunto de
código apresentado. O hash das novas fontes identifica bytes atuais; não
recupera o ambiente da execução antiga.

Não alteramos Rust ou Modelfile, não repetimos o teste do vetor nem a suíte do
Bibliotecário, não enviamos nova consulta ao Qwen e não fizemos benchmark.
A medição desta etapa é a contagem de fontes, fichas e referências inválidas;
não é uma medida de desempenho ou de qualidade do modelo.

## Fechamento e próxima aula

A **Aula 11 está concluída**, com escopo de investigação estática: localizamos a
cadeia, registramos evidências e validamos os artefatos usando o Bibliotecário.

Na **Aula 12 — Observar a fronteira da tolerância**, a proposta é definir antes
os resultados esperados para pares f64 iguais, abaixo do limite, no limite e
acima dele. Usaremos zero como um dos operandos para tornar a fronteira mais
fácil de examinar. Depois criaremos testes pequenos da macro e registraremos
uma nova execução com identidade própria. Aprovar um teste que espera pânico
significa observar a rejeição esperada, não aceitar o par comparado. A nova
execução não comprovará retroativamente a origem de RUN_VECTOR_1.
