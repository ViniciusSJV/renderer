# Aula 12 — Observar a fronteira da tolerância

## Objetivo

Executar quatro testes da macro real e interpretar corretamente a aprovação de
um teste que espera pânico.

## Contexto e pré-requisitos

A Aula 11 identificou a regra de diferença absoluta estritamente menor que
EPSILON para `f64`. Use Cargo e o editor na raiz do clone. Não é necessário
Ollama. A previsão deve anteceder a leitura do resultado.

## Conceitos e implementação

Leia [previsoes-fronteira.md](../experimentos/03-tuplas/previsoes-fronteira.md)
e [tests/equivalence_boundary.rs](../../tests/equivalence_boundary.rs).
Os quatro testes importam a macro, o trait `Equivalence` para resolver o método
usado por ela e a constante `EPSILON`.

Zero é o primeiro operando. Não é necessário presumir que o decimal 0.00001
tenha representação binária exata: o próprio valor de EPSILON é a fronteira,
e sua metade e seu dobro definem os outros casos.

Os testes de rejeição usam `#[should_panic(expected = "asserting equality.")]`.
O harness exige um pânico contendo essa mensagem. Se a macro aceitar o par, o
teste falha. Portanto, quatro testes aprovados não significam quatro pares aceitos.

## Passo a passo

Na raiz, execute somente os testes de integração da fronteira. Cargo compila e
grava em `target`; não altera os registros históricos nem cria uma captura:

```bash
cargo test --locked --test equivalence_boundary -- --test-threads=1
```

O segundo `--` encaminha opções ao harness; uma thread torna a execução dos
casos sequencial. Observe quatro aprovações, incluindo dois pânicos esperados:

| Par | Previsão | Registro RUN_EQUIVALENCE_BOUNDARY_1 |
| --- | --- | --- |
| 0.0, 0.0 | Aceitar | Aprovação sem pânico. |
| 0.0, EPSILON / 2.0 | Aceitar | Aprovação sem pânico. |
| 0.0, EPSILON | Rejeitar | Aprovação com pânico esperado. |
| 0.0, EPSILON * 2.0 | Rejeitar | Aprovação com pânico esperado. |

O [relatório histórico](../experimentos/03-tuplas/teste-fronteira.txt) registra
**4 aprovados, 0 falhas e código 0**, comando, horários, Rust, Git e hashes
anteriores do teste, fontes, manifesto, lockfile e previsões. Ele foi produzido
por captura pontual, cujo script não é um procedimento reutilizável versionado.
Uma execução nova no terminal não é a execução antiga com o mesmo ID.

Na raiz, confira o dossiê histórico. Isso lê fontes e relatório, sem reexecutar
seu comando ou criar exportação:

```bash
cargo run --locked --bin validate_evidence -- ai/experimentos/03-tuplas/evidencias-fronteira.json
```

O material correspondente contém duas fontes e oito fichas: quatro sobre código
e quatro sobre resultados. A conferência histórica teve zero referências
inválidas. Se uma fonte atual mudar, examine a divergência; não atualize o hash
histórico para fazê-la passar.

## Validação e problemas comuns

TEST_BOUNDARY_1 identifica o documento; RUN_EQUIVALENCE_BOUNDARY_1, a execução.
O validador compara os campos transcritos ao cabeçalho, mas não verifica todos
os hashes internos do relatório contra arquivos compilados.

Se aparecer `should panic ... ok`, leia como rejeição esperada observada.
Se aparecer falha, examine qual caso divergiu da previsão antes de modificar o
teste. `finished in 0.00s` é uma apresentação arredondada do harness, não uma
medição do custo da macro.

## Resultado e limites

Os quatro casos são coerentes com `< EPSILON` e mostram que tolerância pode
aceitar valores distintos. Não cobrem todas as entradas, NaN, infinitos ou outras
magnitudes. Não comprovam retroativamente os bytes que produziram RUN_VECTOR_1.
Não houve benchmark ou nova avaliação do Qwen; a nota da Aula 10 permanece 3/6.

## Próxima aula

A [Aula 13](13-captura-de-execucao.md) apresenta um capturador reutilizável,
para preservar comando, saída e resultado de novas execuções.
