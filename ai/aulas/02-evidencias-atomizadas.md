# Aula 2 — Evidências pequenas e critérios explícitos

## Conceito: separar antes de explicar

Na primeira aula, o Qwen confundiu disputa pelo mesmo mutex com escrita no
mesmo pixel. Vamos reduzir o problema até que cada afirmação seja fácil de
conferir. Esse é nosso primeiro exercício de atomização da informação.

Considere duas tarefas: A escreve no pixel 10 e B no pixel 20. Ambas adquirem
o mesmo mutex M antes de escrever e o liberam depois. O exemplo é pseudocódigo;
não temos uma execução nem medições.

Há três perguntas diferentes: onde cada tarefa escreve, qual trava adquire
e o que acontece durante uma execução. As duas primeiras têm resposta no
exemplo. A terceira exige evidência adicional.

## Teste inicial: reduzir o contexto

Enviamos o seguinte exercício ao Qwen:

```text
Analise apenas este pseudocódigo. Ele não foi executado.

M é um único mutex que protege uma imagem inteira.
A imagem tem pelo menos 21 pixels.
As tarefas A e B podem executar concorrentemente.

Tarefa A:
    adquirir M
    escrever vermelho no pixel 10
    liberar M

Tarefa B:
    adquirir M
    escrever azul no pixel 20
    liberar M

Responda:
1. As tarefas escrevem no mesmo pixel?
2. Elas adquirem o mesmo mutex?
3. As duas escritas protegidas podem ocorrer simultaneamente?
4. Podemos afirmar que houve espera nesta execução?
5. Podemos quantificar o custo ou concluir que há um gargalo?

Use FACT para fatos do exemplo, INFERENCE para deduções com premissas
e HYPOTHESIS para possibilidades não verificadas.
Não proponha otimizações. Não invente execução ou medições.
```

O Qwen acertou as cinco conclusões principais: pixels distintos, mesmo mutex,
escritas protegidas sem simultaneidade, espera desconhecida e custo desconhecido.
Foram 5/5 nesse critério restrito. As justificativas ainda precisaram de revisão:

- A segunda tarefa só espera se tentar adquirir M enquanto ele está ocupado.
  Chegar depois não significa necessariamente esperar.
- Pixels distintos não descartam gargalo no mutex compartilhado. Faltam
  medições para concluir se a trava limita o desempenho.
- Reconhecer que faltam dados é uma conclusão sobre a evidência. Uma hipótese
  descreve uma possibilidade ainda não verificada, como a ocorrência de espera.

Acertar uma conclusão não basta: também precisamos conferir a justificativa.
Esse resultado descreve uma resposta, não a confiabilidade geral do modelo.

## Implementação: um pequeno conjunto de evidências

O arquivo [evidencias.json](../experimentos/02-mutex/evidencias.json) guarda
a fonte do exemplo e cinco afirmações extraídas manualmente dela.

Cada fato tem quatro campos:

| Campo | Para que serve |
| --- | --- |
| `id` | Permite citar uma afirmação sem repetir seu texto inteiro. |
| `statement` | Expressa uma afirmação pequena que podemos conferir. |
| `source_id` | Identifica de onde ela veio. |
| `line` | Indica a posição na lista `lines` da fonte, contando a partir de 1. |

Por exemplo, F1 aponta para a quarta linha de S1: A escreve no pixel 10.
F2 aponta para a quinta: B escreve no pixel 20. A conclusão de que os pixels
são distintos pode então citar F1 e F2.

Os fatos descrevem o pseudocódigo, não eventos observados numa execução.
O campo `executed: false` torna essa diferença explícita. A lista `unknowns`
registra perguntas que a fonte não resolve.

Ainda não precisamos de um banco de grafos. Já temos uma relação simples:
um fato aponta para sua fonte. Mais adiante, conclusões poderão apontar para
os fatos que as sustentam, e medições para a execução que as produziu.

## Teste: pedir uma explicação com referências

Em uma conversa nova com `renderer-analyst`, envie o prompt abaixo seguido
do conteúdo completo do JSON:

```text
O JSON abaixo descreve um exemplo de pseudocódigo, não uma execução.
Use os identificadores dos fatos para sustentar suas respostas.

1. As tarefas escrevem no mesmo pixel? Cite os fatos necessários.
2. As escritas protegidas podem ocorrer simultaneamente? Cite os fatos
   necessários e explicite a premissa sobre o comportamento de um mutex.
3. Há evidência de espera ou de gargalo? Explique os limites da fonte.

Separe fatos, inferências e hipóteses. Não invente identificadores.
Não proponha otimizações nem medições fictícias.
```

O acesso ao arquivo não é automático: precisamos fornecer seu conteúdo ao
Qwen. Nesta etapa, fazemos esse transporte manualmente.

## Medição: conferir conclusão, referência e premissa

Antes de ler a resposta, fixamos cinco critérios, valendo um ponto cada:

1. Conclui que os pixels são distintos e cita F1 e F2.
2. Conclui que as escritas protegidas não são simultâneas, usando F3, F4 e F5.
3. Explicita a exclusão mútua como premissa, sem fingir que foi medida.
4. Reconhece que não há execução registrada para determinar espera ou gargalo.
5. Usa somente identificadores existentes e referências que sustentam a frase.

Uma pontuação ajuda a comparar respostas, mas registraremos também os erros
de justificativa. Um JSON válido não garante fatos corretos; uma referência
existente não garante que ela sustente a conclusão. Nesta aula, a conferência
de significado continua sendo manual.

## Resultado do teste com JSON

O Qwen identificou os pixels distintos com F1 e F2 e explicou a exclusão mútua
com F3, F4 e F5. Também reconheceu que não havia evidência de espera ou gargalo.
Porém, afirmou que B só poderia adquirir M depois de A liberá-lo. Essa ordem
não está na fonte: B também poderia adquirir o mutex primeiro.

O resultado foi **4/5**, contando um ponto apenas para critérios integralmente
atendidos. O quinto ficou parcial: os identificadores existem, mas não
sustentam a ordem A antes de B acrescentada à explicação.

A formulação correta é condicional: se uma tarefa mantém M adquirido, a outra
precisa aguardar sua liberação para adquiri-lo. Não sabemos qual chega primeiro.

O modelo acrescentou hipóteses sobre intervalos curtos de posse da trava.
Não inventou medições, mas essas hipóteses eram desnecessárias para responder
às perguntas. Também falou em executar o pseudocódigo; para medir, precisaríamos
implementar e executar um programa equivalente.

Uma referência válida pode acompanhar uma dedução inválida. O JSON facilitou
localizar o problema, mas esta amostra não demonstra melhora de qualidade pelo
formato: o contexto e a pergunta também mudaram. Não medimos performance.

## Onde continuaremos

O próximo exercício será um validador de evidências em Rust. Ele verificará
identificadores únicos, fontes existentes e linhas dentro dos limites.
Testaremos também entradas com ID duplicado, fonte inexistente e linha inválida.
Conjuntos e mapas serão uma oportunidade para estudar custo de consulta e memória.
Esse programa ainda não foi implementado. Sua validação será estrutural;
a correção das deduções continuará exigindo análise do conteúdo.
