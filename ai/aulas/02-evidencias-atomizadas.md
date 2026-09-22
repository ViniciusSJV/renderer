# Aula 2 — Evidências pequenas e critérios explícitos

## Objetivo

Atomizar um exemplo de concorrência em afirmações verificáveis e avaliar
conclusão, referência e premissa separadamente.

## Contexto e pré-requisitos

A Aula 1 mostrou que instruções ao modelo não impedem extrapolações.
Use o clone, o editor e o `renderer-analyst` configurados naquela aula.
Rust ainda não é necessário: a validação deste exercício é manual.

## Conceitos

Atomização é separar informações para que cada afirmação tenha um escopo e uma
origem examináveis. Considere duas tarefas: A escreve no pixel 10 e B no pixel
20. Ambas adquirem o mesmo mutex M antes de escrever e o liberam depois.
O exemplo é pseudocódigo, não uma execução observada.

Há três perguntas distintas: onde cada tarefa escreve, qual trava adquire e o
que aconteceu numa execução. O texto resolve as duas primeiras; a terceira
exige um registro adicional. Exclusão mútua é uma premissa sobre o mecanismo,
não uma medição de espera neste exemplo.

## Implementação: fonte e afirmações

Abra [evidencias.json](../experimentos/02-mutex/evidencias.json). Ele contém uma
fonte de pseudocódigo, cinco fatos manuais e questões desconhecidas.

| Campo do fato | Responsabilidade |
| --- | --- |
| `id` | Identificação para citação, como F1. |
| `statement` | Afirmação pequena a conferir. |
| `source_id` | Fonte de origem, como S1. |
| `line` | Posição na lista `lines`, contando de 1. |

F1 aponta para a quarta entrada de S1, sobre o pixel 10; F2 aponta para a
quinta, sobre o pixel 20. A numeração é da lista, não das linhas físicas do JSON.
`executed: false` declara a natureza não executada do exemplo; `unknowns`
registra o que não pode ser resolvido com essa fonte.

Um JSON organiza dados, mas não comprova seu significado. O relacionamento
fato → fonte → trecho já permite rastrear afirmações sem banco de grafos.

## Passo a passo

Na raiz do clone Windows, abra a interface do modelo. O comando carrega o
modelo e permite enviar texto; não altera os arquivos do projeto:

```powershell
ollama run renderer-analyst
```

Envie o exercício abaixo na interface interativa. Antes de ler a resposta,
registre os cinco itens como critérios em um documento novo no editor:

```text
Analise apenas este pseudocódigo. Ele não foi executado.
M é um único mutex que protege uma imagem com pelo menos 21 pixels.
As tarefas A e B podem executar concorrentemente.
Tarefa A: adquirir M; escrever vermelho no pixel 10; liberar M.
Tarefa B: adquirir M; escrever azul no pixel 20; liberar M.
1. As tarefas escrevem no mesmo pixel?
2. Elas adquirem o mesmo mutex?
3. As duas escritas protegidas podem ocorrer simultaneamente?
4. Há evidência de espera em uma execução?
5. É possível quantificar custo ou concluir que há gargalo?
Separe FACT, INFERENCE e HYPOTHESIS. Não invente execução ou medições.
```

A avaliação histórica marcou 5/5 nas conclusões principais, mas encontrou
justificativas problemáticas. Chegar depois não implica esperar: a trava pode
já estar livre. Pixels diferentes também não descartam gargalo no mesmo mutex.

Em uma nova sessão do modelo, envie o pedido seguinte e o conteúdo completo de
`ai/experimentos/02-mutex/evidencias.json`, copiado pelo editor. O nome do arquivo
sozinho não transfere seus dados:

```text
O JSON abaixo descreve pseudocódigo, não uma execução.
1. As tarefas escrevem no mesmo pixel? Cite os fatos necessários.
2. As escritas protegidas podem ocorrer simultaneamente? Cite os fatos
   necessários e explicite a premissa sobre mutex.
3. Há evidência de espera ou gargalo? Explique os limites da fonte.
Separe fatos, inferências e hipóteses. Não invente identificadores,
otimizações, execuções ou medições.
```

## Validação

Defina estes critérios antes de receber a nova resposta; cada item integralmente
atendido vale um ponto. Preserve também as justificativas:

1. Pixels distintos, citando F1 e F2.
2. Escritas protegidas sem simultaneidade, usando F3, F4 e F5.
3. Exclusão mútua explicitada como premissa, sem alegar medição.
4. Ausência de registro para determinar espera ou gargalo.
5. Somente IDs existentes e referências que sustentem a frase inteira.

A resposta histórica recebeu **4/5**: citou fatos pertinentes, mas acrescentou
uma ordem A antes de B que a fonte não fornece. B poderia adquirir M primeiro.
A formulação sustentada é condicional: enquanto uma tarefa mantém M adquirido,
a outra precisa aguardar sua liberação para adquiri-lo.

Referência válida não significa sustentação semântica. Hipóteses desnecessárias
sobre duração da trava não devem ocupar o lugar de informação ausente.

## Problemas comuns e limites

Se a resposta disser que leu um arquivo sem receber seu conteúdo, confira a
entrada realmente enviada. Se citar um ID, procure a ficha e leia o trecho:
existência e pertinência são verificações diferentes.

As notas descrevem respostas específicas, não confiabilidade geral nem vantagem
causal do JSON sobre texto livre: pergunta e contexto também mudaram. Não houve
testes Rust, benchmark ou medição de latência nesta etapa.

## Resultado da aula e próxima aula

O exemplo possui cinco afirmações com endereços e uma avaliação explícita.
A [Aula 3](03-bibliotecario-em-rust.md) apresenta Rust e o Bibliotecário, que
confere esses endereços sem julgar automaticamente o significado das frases.
