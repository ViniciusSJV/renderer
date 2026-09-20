# Laboratório de IA e performance

## O projeto

Este laboratório acompanha o ray tracer em Rust baseado em *The Ray Tracer
Challenge*, de Jamis Buck. O README principal registra capítulos 1–15
concluídos e 16–17 pendentes. Queremos aprender implementando, com atenção
especial a algoritmos e performance.

**Condensar fatos a partir de vapores de nuances.**

A arquitetura que estamos explorando divide responsabilidades:

- **Graph Engine — “Testa sem explicar”:** fatos, relações, contexto,
  evidências, testes, benchmarks e validação.
- **LLM Engine — “Explica sem interpretar”:** explica as evidências recebidas,
  distinguindo fatos, inferências e hipóteses.

O Codex é parceiro de desenvolvimento e estudo. O Qwen local, via Ollama,
é parte da arquitetura que estamos aprendendo a construir.

Cada aula segue conceito → implementação → teste → medição → explicação.
Ao estudar performance, priorizamos complexidade, estruturas de dados,
alocações, localidade de cache, SIMD quando aplicável e paralelismo.
Uma otimização só será chamada de mais rápida com evidência mensurável.

## Ambiente

O repositório está no Codespaces `silver guide`, em `/workspaces/renderer-local`.
O Ollama roda no Windows, com `qwen2.5-coder:14b`. O localhost desses ambientes
é diferente. Ainda não construímos uma integração entre o projeto remoto e o
Ollama local; os exemplos são enviados manualmente ao modelo.

## O que fizemos

Na [Aula 1](aulas/01-modelfile-ollama.md), criamos o
[Modelfile](ollama/Modelfile), com temperatura 0, semente 42, contexto de 4096
tokens e saída limitada a 1024 tokens. O SYSTEM orienta o domínio técnico e
a separação entre FACT, INFERENCE e HYPOTHESIS.

O arquivo foi copiado para o Windows e registrado com sucesso como
`renderer-analyst`. Para iniciar uma conversa no PowerShell:

```powershell
ollama run renderer-analyst
```

O modelo pode ser iniciado de qualquer pasta. Isso não lhe dá acesso automático
aos arquivos do projeto. Alterações no Modelfile precisam ser registradas
novamente com `ollama create` no ambiente do Ollama.

Nos primeiros testes, o Qwen seguiu os rótulos, mas presumiu uma representação
de cores, recomendou AtomicU32 sem conhecer o código e confundiu disputa pela
trava com escrita no mesmo pixel. Reconheceu algumas lacunas quando questionado,
mas manteve contradições. Configurar o comportamento não garante correção.

Na [Aula 2](aulas/02-evidencias-atomizadas.md), reduzimos o exemplo a duas
tarefas que escrevem em pixels distintos usando o mesmo mutex. A resposta
acertou as cinco conclusões principais, mas ainda apresentou justificativas
problemáticas: uma nota sobre conclusões não descreve a resposta inteira.

Criamos então [evidencias.json](experimentos/02-mutex/evidencias.json), com uma
fonte de pseudocódigo, cinco fatos identificados e uma lista de informações
desconhecidas. Cada fato aponta para uma linha da fonte. A estrutura do JSON,
a unicidade dos IDs e a existência das referências foram verificadas.

No teste com esse JSON, o Qwen recebeu **4/5** nos critérios definidos antes
da resposta. Citou fatos pertinentes, mas acrescentou uma ordem de execução
não fornecida: A antes de B. Uma referência existente pode acompanhar uma
dedução que ela não sustenta.

Essas avaliações são manuais e descrevem as respostas examinadas. Não provam
confiabilidade geral nem superioridade do JSON sobre texto livre. Nas aulas 1 e 2, não rodamos
testes Rust ou benchmarks do renderer e não medimos latência do Qwen.

Na [Aula 3](aulas/03-bibliotecario-em-rust.md), construímos o Bibliotecário em
[src/bin/validate_evidence.rs](../src/bin/validate_evidence.rs). Ele lê o JSON,
confere IDs únicos, encontra fontes e verifica linhas. Referências inválidas
resultam em código de saída 1. Ao fim da implementação, os 21 testes do binário
passaram; não fizemos benchmark nem executamos a suíte completa do renderer.

Também registramos um parecer do Qwen e implementamos a conferência de sua
associação à ficha e ao trecho. O julgamento permanece uma análise do modelo;
o programa não verifica sua correção semântica. A comunicação com o Ollama
continua manual. A aula documenta os comandos e os experimentos reproduzíveis.

Na [Aula 4](aulas/04-tuplas-codigo-e-evidencias.md), seguimos o livro a partir
de `src/tuple.rs`, criando uma cópia identificada por hash e fichas manuais.
O Bibliotecário agora compara arquivos atuais, hashes e linhas copiadas.
Registramos uma execução do teste do vetor e reunimos duas fontes e sete fichas.
Os 26 testes do validador passaram ao implementar essa etapa; não houve benchmark.

As consultas ao Qwen mostraram acertos, mas também omissão de referências e
conclusões além das evidências. A aula registra a avaliação e seus limites.

Na [Aula 5](aulas/05-selecao-e-contexto.md), adicionamos seleção por ID e
exportação de uma ficha com contexto configurável por `--context`. Os 32 testes
do binário passaram; duas exportações com os mesmos dados e parâmetros tiveram
bytes idênticos. A aula registra comandos, artefatos e limites dessa observação.

Na [Aula 6](aulas/06-metadados-das-fontes.md), preservamos `kind`, `executed`,
`git_commit` e `authorship` na leitura e exportação. Campos ausentes permanecem
sem informação; declarações não são promovidas a comprovações. Os 37 testes do
binário passaram e duas exportações reais tiveram seus metadados conferidos.

Na [Aula 7](aulas/07-identidade-limites-e-consulta.md), preservamos identidade
e questões abertas do dossiê e montamos a consulta com `--question`. Os 43 testes
do binário passaram. Uma resposta do Qwen recebeu 2/5 em critérios definidos
previamente; consulta, resposta e avaliação estão preservadas.

Na [Aula 8](aulas/08-codigo-nao-e-execucao.md), separamos fonte de código de
registro de execução. As novas exportações de `rust_source` omitem `executed`;
`ExecutionRecord` identifica uma execução e seus campos são comparados ao
cabeçalho do relatório. A exportação informa quais campos foram conferidos e
os limites dessa comparação. Os **53 testes do Bibliotecário passaram**.
As consultas anteriores foram preservadas; não houve nova consulta ao Qwen
nem benchmark.

Na [Aula 9](aulas/09-reunir-evidencias.md), passamos a selecionar várias fichas
com `--fact ID` repetido e compartilhar janelas de contexto sobrepostas ou
adjacentes da mesma fonte. As referências individuais e os limites de cada
janela foram preservados. Os **60 testes do Bibliotecário passaram**. No exemplo
com três fichas, 25 linhas de contexto nas janelas individuais passaram a 17
nos blocos compartilhados; não medimos tokens, desempenho ou qualidade do Qwen.

Na [Aula 10](aulas/10-avaliar-codigo-e-resultado.md), preparamos uma consulta
com código e resultado e seis critérios anteriores à resposta. A resposta
recebida manualmente obteve **3/6**: reconheceu limites da macro e da conferência,
mas inferiu inicialização correta sem suporte suficiente e confundiu o ID da
fonte com o da execução. Consulta, critérios, resposta e justificativas estão
preservados na [avaliação](experimentos/03-tuplas/avaliacao-codigo-e-resultado.json).
Não houve alteração de Rust ou Modelfile, nova execução de testes ou benchmark.

Na [Aula 11](aulas/11-investigar-assert-equivalent.md), investigamos a cadeia
`assert_equivalent!` → `not_equivalent` → `equivalent` para f64 → `EPSILON`.
Criamos dez fichas e duas fontes novas em um dossiê separado. O Bibliotecário
validou **4 fontes e 17 fichas, sem referências inválidas**. A leitura mostra
uma tolerância absoluta estrita de `0.00001`; não houve novos testes da macro,
consulta ao Qwen ou benchmark. Os artefatos anteriores foram preservados.

Na [Aula 12](aulas/12-observar-fronteira-tolerancia.md), registramos previsões
e executamos quatro testes de integração da macro: pares iguais e abaixo de
EPSILON foram aceitos; no limite e acima dele houve o pânico esperado.
**4 testes passaram**, com código 0, em RUN_EQUIVALENCE_BOUNDARY_1.
O novo dossiê contém **2 fontes e 8 fichas, sem referências inválidas**.
Não houve benchmark nem nova consulta ao Qwen.

## Ponto de retomada

A **Aula 12 — Observar a fronteira da tolerância está concluída**.
As previsões, o relatório e as fichas estão vinculados na aula. A execução nova
não comprova a associação histórica com RUN_VECTOR_1. A nota da Aula 10
permanece **3/6**.

Na **Aula 13 — Captura de execução**, vamos transformar a captura pontual em
um procedimento reutilizável para registrar comando, saída, código de término
e ambiente, incluindo o tratamento de falhas. A associação entre código e
execução será aprofundada na Aula 14. O envio ao Ollama continua manual.

Seguimos a sequência acordada: Bibliotecário e evidências, integração e avaliação,
performance e, depois, criação de cenas por linguagem natural. Ainda não há
banco de grafos nem extração automática de afirmações.
