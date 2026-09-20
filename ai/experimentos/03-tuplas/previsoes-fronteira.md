# Previsões da Aula 12

Registradas antes da execução dos novos testes. Base: leitura da cadeia em
src/equivalent.rs e da constante em src/lib.rs, estudada na Aula 11.

| Teste | Par f64 | Previsão para a macro |
| --- | --- | --- |
| accepts_equal_values | 0.0, 0.0 | Retornar sem pânico. |
| accepts_difference_below_epsilon | 0.0, EPSILON / 2.0 | Retornar sem pânico. |
| rejects_difference_at_epsilon | 0.0, EPSILON | Pânico com mensagem contendo `asserting equality.`. |
| rejects_difference_above_epsilon | 0.0, EPSILON * 2.0 | Pânico com mensagem contendo `asserting equality.`. |

Esperamos quatro testes aprovados: nos dois últimos, aprovação significa
observar o pânico esperado. A fronteira é o valor f64 armazenado em EPSILON.
Não são testes de todas as entradas, valores especiais ou desempenho.
