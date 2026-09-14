# Aula 1 — Entendendo o Modelfile do Ollama

## O que vamos construir

Nosso ray tracer em Rust, baseado em *The Ray Tracer Challenge*, de Jamis
Buck, também será um laboratório de algoritmos, performance e IA local.
Nesta aula, vamos preparar o Qwen para participar desse laboratório: receber
informações sobre o projeto, explicar o que elas sustentam e reconhecer
o que ainda precisa ser testado.

Princípio: **Condensar fatos a partir de vapores de nuances.**

- Graph Engine — “Testa sem explicar”: futura camada de fatos, relações,
  contexto, evidências, testes, benchmarks e validação.
- LLM Engine — “Explica sem interpretar”: explica evidências e identifica
  deduções sem apresentá-las como observações verificadas.

O Codex participa como parceiro de desenvolvimento e estudo. O Qwen, rodando
no Ollama, é o modelo local com o qual faremos os experimentos.

Formato das aulas: conceito → implementação → teste → medição → explicação.
Prioridade de performance: complexidade, estruturas de dados, alocações,
localidade de cache, SIMD quando aplicável e paralelismo.
Nenhuma melhoria de velocidade deve ser afirmada sem evidência mensurável.

## Antes de começar: dois ambientes

O projeto está no GitHub Codespaces `silver guide`, em
`/workspaces/renderer-local`. Já o Ollama com `qwen2.5-coder:14b` roda no
Windows. Essa separação explica um detalhe importante da prática:
`localhost` no Codespaces aponta para o ambiente remoto, não para o Windows.
Por isso, criaremos o arquivo no projeto e levaremos uma cópia ao Windows
para registrá-lo no Ollama.

O pacote `renderer` usa Rust 2021, com
Rayon, itertools e png, testes dentro dos módulos, exemplos em `src/bin`,
um carregador OBJ e construção de BVH em `src/bin/cap15.rs`.
Em `src/camera.rs`, a renderização usa Rayon, chama `world.clone()` por pixel
e adquire um mutex para escrever a cor. Essas operações nos dão boas perguntas
para estudar performance.
Para descobrir quanto custam, precisaremos de benchmarks; nesta aula,
vamos trabalhar apenas com a configuração e as respostas do modelo.

## Conceito: uma receita para executar o modelo

Um Modelfile descreve o modelo base, parâmetros de execução e instruções de
comportamento no Ollama. Nossa configuração não treina os pesos do modelo.

| Instrução | Papel no experimento |
| --- | --- |
| `FROM qwen2.5-coder:14b` | Seleciona a base. |
| `PARAMETER temperature 0` | Reduz variabilidade da geração; não garante verdade. |
| `PARAMETER seed 42` | Registra uma semente; repetibilidade precisa ser observada. |
| `PARAMETER num_ctx 4096` | Define a janela de contexto em tokens. |
| `PARAMETER num_predict 1024` | Limita os tokens gerados na resposta. |
| `SYSTEM` | Orienta domínio técnico, categorias de afirmações e método de estudo. |

Tokens são unidades de texto, não necessariamente palavras. O contexto precisa
acomodar instruções, entrada, histórico e geração. Estes valores são condições
iniciais, não parâmetros demonstrados como ótimos para o hardware.

Categorias adotadas:

- **FACT:** afirmação sustentada por evidência identificável; relatos precisam
  ser atribuídos, sem fingir verificação independente.
- **INFERENCE:** conclusão derivada de fatos com premissas e limites explícitos.
- **HYPOTHESIS:** proposição ainda dependente de validação, acompanhada de uma
  forma de testá-la.

O SYSTEM orienta o comportamento, mas não garante cumprimento. Um rótulo FACT
não valida a frase. O Modelfile também não dá acesso automático ao repositório.

## Implementação: criando o renderer-analyst

A configuração fica em [ai/ollama/Modelfile](../ollama/Modelfile).
O `SYSTEM` reúne nosso foco em Rust e performance, a ordem de investigação
e as regras para distinguir evidências de suposições.

Com uma cópia do arquivo na pasta Downloads do Windows, executamos no
PowerShell:

```powershell
ollama create renderer-analyst -f "C:\Users\beatl\Downloads\Modelfile"
```

O comando terminou em `writing manifest` e `success`: o modelo configurado
foi registrado. As mensagens `using existing layer` indicam o reaproveitamento
de camadas. Estamos configurando como executar o Qwen, sem treinar seus pesos.

Agora precisamos descobrir como ele responde às nossas perguntas.

Depois do registro, a conversa pode ser iniciada de qualquer pasta:

```powershell
ollama run renderer-analyst
```

A pasta Downloads só serviu para localizar o arquivo de entrada do create.
Alterações futuras no arquivo precisam ser registradas novamente no Ollama.

## Teste 1 — Resposta espontânea

Começamos com uma pergunta que convida a uma conclusão precipitada:

```text
Um renderer Rust usa Rayon para processar pixels e adquire um Mutex
compartilhado para gravar a cor de cada pixel. Não forneci código,
tempos de execução ou resultados de benchmark.

Remover esse Mutex tornará o renderer mais rápido?

Organize sua análise em FACT, INFERENCE e HYPOTHESIS.
Proponha um teste e uma medição, sem apresentar resultados inventados.
```

A resposta seguiu as categorias e não inventou tempos. Porém, não atribuiu
explicitamente de onde veio a descrição e misturou possibilidade de overhead
com FACT. Sugeriu AtomicU32 sem conhecer a representação das cores. Propôs
benchmark, mas não detalhou equivalência das imagens e controles suficientes.

## Teste 2 — Revisão guiada

Na mesma conversa, pedimos ao Qwen que examine as próprias suposições:

```text
Revise sua resposta anterior.

1. Você inspecionou o código ou apenas recebeu minha descrição?
2. Você conhece a representação das cores para recomendar AtomicU32?
3. A disputa pelo mesmo Mutex exige que as threads escrevam no mesmo pixel?
4. Se cada tarefa tiver acesso exclusivo a pixels distintos, operações
   atômicas seriam necessárias? Explicite as condições.
5. Como comparar as versões preservando correção e condições equivalentes?

Separe FACT, INFERENCE e HYPOTHESIS. Reconheça as lacunas da resposta
anterior e não invente evidências.
```

Dessa vez, o Qwen reconheceu que não leu código nem executou testes e
admitiu supor cores de 32 bits. Ainda associou disputa pela trava à mesma
região de memória. Disse que acesso exclusivo dispensa atômicos, mas voltou
a recomendar AtomicU32 no plano de teste. Não demonstrou correção da proposta.

## Medição: avaliando as respostas

Neste primeiro exercício, avaliamos a qualidade das duas respostas:
identificação das evidências, coerência e cuidado com as conclusões.
Ainda não medimos latência, quantidade de tokens ou velocidade do renderer.

| Critério | Teste 1 | Teste 2 |
| --- | --- | --- |
| Reconhecer origem e limites da evidência | Parcial | Reconheceu ausência de inspeção |
| Separar categorias corretamente | Parcial | Ainda classificou suposição como inferência |
| Evitar ganho de velocidade inventado | Atendeu | Atendeu |
| Propor comparação com correção e controles explícitos | Parcial | Parcial |
| Distinguir disputa pela trava de disputa pelo pixel | Insuficiente | Erro persistente |

Essas amostras não demonstram confiabilidade geral nem permitem comparar
modelos ou configurações. O segundo teste contém orientação adicional e não
é uma repetição controlada do primeiro.

## O que aprendemos com o resultado

Pixels diferentes podem disputar o mesmo mutex. A exclusão ocorre sobre a
trava compartilhada, mesmo quando as posições a modificar não coincidem.
Investigar regiões sem sobreposição com acesso exclusivo é uma alternativa
conceitual; ela não possui ganho de desempenho demonstrado nesta aula.
Também é necessário coordenar a conclusão das tarefas antes de consumir a
imagem e considerar quaisquer outros acessos durante o processamento.

Trocar a representação de cores exige verificar precisão e equivalência.
Atômicos não são sinônimo de acesso sem custo e não devem ser recomendados
automaticamente quando a divisão de propriedade pode resolver os acessos.

Uma comparação futura deve preservar cena, resolução, threads e compilação
entre versões, verificar resultados, repetir execuções e delimitar se o tempo
inclui renderização ou gravação. Ganhos sustentariam conclusões apenas nas
condições medidas.

## Próxima aula

Na Aula 2, vamos reduzir o problema a duas tarefas que escrevem em pixels
diferentes usando o mesmo mutex. Antes de consultar o Qwen, definiremos
os critérios de uma boa resposta. Assim poderemos avaliar uma questão
por vez e entender melhor o erro antes de ajustar o `SYSTEM`.

## Referências consultadas na aula

- [Modelfile Reference — Ollama](https://docs.ollama.com/modelfile)
- [API de geração — Ollama](https://docs.ollama.com/api/generate)
