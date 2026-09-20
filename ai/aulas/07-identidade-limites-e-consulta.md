# Aula 7 — Identidade, limites e uma consulta avaliável

## Conceito

O Bibliotecário precisa entregar não só uma ficha e um trecho, mas também
identificar o dossiê e preservar suas limitações. A pergunta deve ficar separada
das evidências. Antes de receber a análise, precisamos saber como a avaliaremos.

## Implementação

Em [validate_evidence.rs](../../src/bin/validate_evidence.rs), Evidence ganhou
`id: Option<String>` e `unknowns: Option<Vec<String>>`. A seleção exporta esses
campos como evidence_id e evidence_unknowns.

O ID distingue a identidade declarada do conjunto da identidade local da ficha.
Ainda não há verificação de unicidade entre dossiês: dois arquivos podem declarar
o mesmo ID. Nos exemplos antigos sem ID, a exportação mantém null.

As questões abertas são copiadas sem reescrita ou filtro de relevância. Seu escopo
é o dossiê inteiro, não apenas a ficha escolhida. Campo ausente resulta em null;
lista explicitamente vazia resulta em []. Nenhum dos dois comprova ausência de
lacunas. Essas observações também não substituem as fontes a que se referem.

## Separar o pedido das páginas consultadas

Criamos [pergunta-vector-x.txt](../experimentos/03-tuplas/pergunta-vector-x.txt)
e a opção `--question ARQUIVO`. Com ela, a exportação contém três blocos:

- question: texto do pedido, preservado como foi lido;
- evidence: seleção validada, com contexto e metadados;
- instructions: orientações para usar o material e explicitar seus limites.

O programa exige --output nesse modo e rejeita perguntas vazias. Ele monta a
consulta, mas não chama o Ollama. As orientações não garantem obediência do modelo
nem tornam o significado das evidências automaticamente validado.

Na raiz do projeto, usando um destino ainda inexistente:

```bash
cargo run --bin validate_evidence -- ai/experimentos/03-tuplas/evidencias.json --fact F_VECTOR_TEST_X --context 4 --question ai/experimentos/03-tuplas/pergunta-vector-x.txt --output consulta.json
```

A consulta usada está em
[consulta-vector-x-pergunta.json](../experimentos/03-tuplas/consulta-vector-x-pergunta.json).
Ela inclui o corpo do teste, mas não o registro TEST_VECTOR_1. O fato de esse
registro existir no acervo não permite presumir que foi enviado ao Qwen.

## Teste: definir critérios antes da resposta

Guardamos a rubrica em
[avaliacao-consulta-vector-x.json](../experimentos/03-tuplas/avaliacao-consulta-vector-x.json),
com o SHA-256 da consulta. O gabarito ficou separado do material enviado ao modelo.
A orientação foi usar uma conversa nova; não houve captura automática da sessão.

Cada critério integralmente atendido vale um ponto; parcial vale zero:

| Critério | Resultado da resposta recebida |
| --- | --- |
| C1: identificar os argumentos 1.4, 8.9 e 5.1 | Atendido: 1 |
| C2: descrever a chamada sem presumir funcionamento interno da macro | Parcial: 0 |
| C3: citar F_VECTOR_TEST_X e SRC_TUPLE | Não atendido: 0 |
| C4: reconhecer falta de resultado e não alegar execução própria | Atendido: 1 |
| C5: não transformar metadados em prova nem extrapolar | Não atendido: 0 |

**Resultado: 2/5.** A resposta completa e as justificativas foram preservadas na
avaliação. A consulta e os critérios anteriores à resposta não foram modificados.

O Qwen identificou os argumentos e a chamada, mas atribuiu à macro a verificação
de inicialização correta sem conhecer sua implementação. Citou nomes de campos,
não os IDs pedidos. Reconheceu a ausência de resultados, mas afirmou que
executed=false indicava que o teste não havia sido executado. Avaliamos esse
último erro em C5; ele não anula o reconhecimento, em C4, de que não recebeu um
resultado de aprovação nem alegou ter executado o teste.

Também tratou os valores esperados nas asserções como comportamento do construtor,
cuja implementação não estava na consulta. A resposta se expandiu para uma aula
e propostas de medição; não descontamos pontos por extensão, pois este pedido
não estabelecia limite de frases. Uma possível influência do SYSTEM didático
permanece hipótese, não resultado de comparação controlada.

## Medição e explicação

Os **43 testes do binário passaram** ao concluir a implementação. Conferimos
que a pergunta e as limitações foram preservadas na exportação e que uma pergunta
vazia foi rejeitada antes de criar o arquivo de saída. Não houve benchmark.

A pontuação do Qwen é uma avaliação manual de uma resposta, não uma taxa geral de
acerto do modelo. O hash identifica a consulta preservada; não autentica a execução
remota, a versão do modelo ou o histórico usado.

A aula revelou um problema no nosso contrato: `executed` é ambíguo quando ligado
a uma fonte de código. O arquivo representa código; seu histórico pode conter
muitas execuções. Um booleano na obra não representa adequadamente esse histórico.
Preservamos o campo sem perda, mas isso não garante clareza do significado.

Na biblioteca, é diferente dizer “esta obra é um procedimento” e dizer “esse
procedimento nunca foi executado”. A ausência de um relatório na seleção não
comprova a segunda afirmação.

## Fechamento e retomada

A Aula 7 está concluída. Construímos uma consulta com identidade e limites,
definimos critérios antes da resposta e registramos os erros observados.

O próximo passo será esclarecer o significado dos metadados de execução, mantendo
separados fonte de código e registro de uma execução. Não reescreveremos a consulta
já avaliada para esconder a ambiguidade; ela servirá como referência histórica.
Não precisamos criar todas as abstrações de uma vez nem ajustar o Qwen até que
repita o gabarito. O Modelfile e o renderer permaneceram sem alterações nesta aula.
