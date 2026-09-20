# Aula 12 — Observar a fronteira da tolerância

## Conceito: previsão antes da observação

Na Aula 11, lemos que a equivalência de f64 exige diferença absoluta menor
que EPSILON. Agora observamos quatro casos usando a macro real do renderer.
As [previsões](../experimentos/03-tuplas/previsoes-fronteira.md) foram gravadas
antes da execução e seu hash foi incluído no relatório.

Usamos zero como primeiro operando e o próprio EPSILON como fronteira.
Não presumimos que o literal decimal 0.00001 tenha representação binária exata.
Dividir ou multiplicar esse valor normal por dois fornece casos abaixo e acima
do limite, sem introduzir a subtração entre números próximos de outra magnitude.

## Implementação pequena

Criamos [equivalence_boundary.rs](../../tests/equivalence_boundary.rs), com
quatro testes de integração. Eles importam a macro, o trait Equivalence
(necessário à resolução do método usado pela macro) e EPSILON do renderer.
A implementação da macro e as fontes das aulas anteriores foram preservadas.

Os testes de rejeição usam `#[should_panic(expected = "asserting equality.")]`.
Cada corpo contém apenas a chamada em investigação. O harness exige pânico
com essa parte da mensagem; se a macro aceitar o par, o teste falha.
Assim, quatro testes aprovados não significam quatro pares aceitos.

## Teste e resultado

Executamos na raiz do repositório:

```bash
cargo test --offline --test equivalence_boundary -- --test-threads=1
```

| Par f64 | Previsão | Observação em RUN_EQUIVALENCE_BOUNDARY_1 |
| --- | --- | --- |
| 0.0, 0.0 | Aceitar | Teste aprovado sem pânico. |
| 0.0, EPSILON / 2.0 | Aceitar | Teste aprovado sem pânico. |
| 0.0, EPSILON | Rejeitar | Teste aprovado com pânico esperado. |
| 0.0, EPSILON * 2.0 | Rejeitar | Teste aprovado com pânico esperado. |

**4 testes passaram; 0 falharam; código de término 0.**
O [relatório](../experimentos/03-tuplas/teste-fronteira.txt) preserva comando,
diretório, início e fim UTC, versão do Rust, HEAD, estado Git, hashes coletados
antes da execução e saída combinada. A captura foi feita por um script pontual
nesta sessão; ainda não temos um capturador reutilizável no projeto.

TEST_BOUNDARY_1 identifica o documento e RUN_EQUIVALENCE_BOUNDARY_1 identifica
a execução catalogada. Os hashes registrados abrangem o teste, a implementação
da equivalência, lib.rs, Cargo.toml, Cargo.lock e as previsões. Isso não é uma
captura completa do ambiente, autenticação ou comprovação da execução antiga.

## Evidências e medição

Criamos [evidencias-fronteira.json](../experimentos/03-tuplas/evidencias-fronteira.json)
com duas fontes e oito fichas manuais: quatro sobre a presença dos testes no
código e quatro sobre os resultados registrados. Validamos com:

```bash
cargo run --offline --bin validate_evidence -- ai/experimentos/03-tuplas/evidencias-fronteira.json
```

O Bibliotecário conferiu **2 fontes e 8 fichas, com 0 referências inválidas**,
terminando com código 0. Os campos da execução correspondem ao cabeçalho.
Ele não julga as afirmações nem verifica automaticamente os hashes internos
do relatório contra os arquivos compilados.

A contagem de testes e resultados é a medição desta aula. `finished in 0.00s`
não mede o custo da macro. Não houve benchmark, execução da suíte completa,
repetição do teste do vetor ou nova consulta ao Qwen.

## Explicação e limites

Os resultados correspondem às quatro previsões: a diferença igual ao limite
é rejeitada, coerentemente com `<`, e um par de valores diferentes é aceito
abaixo do limite. Aceitação por tolerância não implica igualdade exata.
Não investigamos NaN, infinitos, outras magnitudes ou todas as entradas.
Esta execução não comprova retroativamente quais bytes produziram RUN_VECTOR_1.
A avaliação da Aula 10 permanece 3/6.

## Fechamento e próxima aula

A **Aula 12 está concluída**: previsão, implementação, execução identificada,
registro, validação e interpretação dos quatro casos foram realizados.

Na **Aula 13 — Captura de execução**, vamos transformar a captura pontual em
um procedimento reutilizável para registrar comando, saída, código de término
e ambiente. Definiremos o formato e o tratamento de falhas antes de implementar.
A associação mais completa entre código e execução será aprofundada na Aula 14.
