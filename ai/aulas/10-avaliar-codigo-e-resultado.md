# Aula 10 — Avaliar uma consulta com código e resultado

## Conceito e preparação

Definimos critérios antes de receber a resposta para evitar adaptar o gabarito
ao resultado. A consulta reúne procedimento e resultado registrado; precisamos
avaliar tanto a conclusão quanto as referências e as premissas usadas.

A [pergunta](../experimentos/03-tuplas/pergunta-codigo-e-resultado.txt) tem quatro
itens: chamadas visíveis e macro, execução registrada, alcance da conferência,
correção geral e desempenho. Geramos a
[consulta](../experimentos/03-tuplas/consulta-codigo-e-resultado-pergunta.json)
pelo Bibliotecário com F_VECTOR_TEST_X, F_VECTOR_TEST_Y e F_VECTOR_TEST_PASSED,
raio 4 e `--question`. Conferimos que as evidências correspondem ao artefato da
Aula 9 e que o texto da pergunta foi preservado.

A [rubrica](../experimentos/03-tuplas/avaliacao-codigo-e-resultado.json) foi
registrada separadamente, com seis critérios e o SHA-256 da consulta. A orientação
foi enviar apenas a consulta ao renderer-analyst numa conversa nova e preservar
a primeira resposta antes de pedir revisão. A resposta foi trazida manualmente
pelo usuário; modelo efetivo, conversa nova e ausência de truncamento não foram
confirmados. Não atribuímos essas condições como fatos da coleta.

## Avaliação da resposta recebida

A resposta completa e as justificativas estão na rubrica. Os critérios foram
preservados e o hash da consulta foi reconferido. Um critério integralmente
atendido vale um ponto; parcial vale zero. Avaliamos a resposta inteira.

| Critério prévio | Resultado | Pontos |
| --- | --- | --- |
| C1: chamadas visíveis sem promover valores esperados a comprovação | Acerta as chamadas, mas depois conclui inicialização correta. | 0 |
| C2: reconhecer a implementação ausente da macro | Reconhece a lacuna e não inventa mecanismo ou tolerância. | 1 |
| C3: resultado atribuído à execução identificada | Relata aprovação, mas usa TEST_VECTOR_1 no lugar de RUN_VECTOR_1 e omite o código 0. | 0 |
| C4: referências existentes que sustentam as afirmações | Cita IDs existentes, mas usa a aprovação para sustentar correção do construtor. | 0 |
| C5: alcance da conferência | Lista os quatro campos e nega autenticação ou comprovação histórica. | 1 |
| C6: limites de generalização e desempenho | Não afirma correção universal ou ganho; pede evidências adicionais. | 1 |

**Resultado: 3/6.** C1 e C4 examinam aspectos relacionados do mesmo excesso:
a conclusão e sua sustentação. Não são medições independentes. O ponto em C2
reconhece a lacuna explícita sobre a macro, sem validar a conclusão posterior.
O ponto em C5 reconhece a descrição correta do que o validador conferiu.

No item 4, a resposta diz que testes mais amplos seriam necessários para garantir
correção geral. Isso merece precisão: ampliar testes não garante por si só
correção para todas as entradas. Porém, necessário não significa suficiente;
não transformamos a frase numa alegação explícita de suficiência para retirar C6.
Não houve desconto por extensão ou pelos escapes Markdown nos IDs.

## O que o erro ensina

O modelo reconheceu duas lacunas: não conhece o interior da macro e não tem
comprovação da ligação histórica entre a versão de código e a execução. Mesmo
assim, concluiu que o construtor inicializa corretamente x e y porque o teste
passou. O rótulo INFERENCE não fornece as premissas ausentes.

Uma formulação sustentada seria:

> SRC_TUPLE mostra chamadas da macro para x e 1.4 e para y e 8.9
> (F_VECTOR_TEST_X e F_VECTOR_TEST_Y). TEST_VECTOR_1 registra aprovação do
> teste em RUN_VECTOR_1, com código de término 0 (F_VECTOR_TEST_PASSED).
> O material não fornece a implementação da macro nem comprova a associação
> histórica entre essa versão do código e a execução.

TEST_VECTOR_1 identifica o documento; RUN_VECTOR_1 identifica a execução
catalogada. A diferença permite falar de cada entidade sem trocar seus papéis.

## Medição e limites

Esta é uma avaliação manual de uma resposta, não uma taxa geral de acerto.
Pergunta, evidências e rubrica diferem da Aula 7: 3/6 e 2/5 não são medidas de
uma comparação controlada nem demonstram melhora causada pelo formato.

Não alteramos Rust ou Modelfile, não repetimos a suíte Rust, não executamos
novamente o teste do vetor e não fizemos benchmark. O trabalho desta aula foi
preparar a consulta, definir critérios e registrar a avaliação da resposta.

## Fechamento e próxima aula

A Aula 10 está concluída. Preservamos consulta, critérios prévios, resposta e
justificativas. Não ajustamos o gabarito nem orientamos revisões sucessivas para
substituir a resposta original.

Na Aula 11, a proposta é investigar uma premissa ausente: a implementação de
`assert_equivalent!`. Vamos localizar a macro, criar evidências sobre seu
comportamento e delimitar o que ela permite concluir. Isso não resolverá sozinho
a associação histórica entre código e execução; essa questão continuará explícita.
