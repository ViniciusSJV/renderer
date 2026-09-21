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

Na [Aula 13](aulas/13-captura-de-execucao.md), criamos um capturador reutilizável
em Rust, com comando e argumentos, saída binária combinada, resultado e
ambiente parcial. **8 testes passaram** e três exemplos registraram sucesso,
código 7 e comando inexistente. O JSON novo ainda não é importado pelo
Bibliotecário. Não houve benchmark nem nova consulta ao Qwen.

Na [Aula 14](aulas/14-associar-codigo-e-execucao.md), acrescentamos `--source`
e hashes antes/depois ao capturador Rust, com formato versão 2 e hash da saída.
**11 testes passaram**. RUN_EQUIVALENCE_BOUNDARY_2 registrou **4 testes aprovados**
e **5 fontes com hashes iguais nas duas leituras**, sem comprovar as entradas
reais do compilador. Os registros anteriores foram preservados.

Na [Aula 15](aulas/15-conferir-registro-captura.md), o Bibliotecário ganhou
`--capture` para conferir registros versão 2, saída e associações de fontes.
**68 testes passaram**. A conferência de RUN_EQUIVALENCE_BOUNDARY_2 validou
540 bytes de saída e cinco arquivos atuais correspondentes aos hashes
posteriores. A conferência foi registrada em RUN_VALIDATE_CAPTURE_1.

Na [Aula 16](aulas/16-levar-conferencia-ao-dossie.md), ligamos uma fonte de
resultado à captura por caminho, SHA-256 e ID de execução. **72 testes passaram**.
O novo dossiê teve **1 fonte e 2 fichas, sem referências inválidas**, e sua
exportação inclui a conferência e os limites. Os registros anteriores foram
preservados; não houve benchmark nem nova consulta ao Qwen.

Na [Aula 17](aulas/17-avaliar-explicacao-captura.md), preparamos seis critérios
antes da resposta do Qwen. A resposta recebida manualmente foi preservada e
recebeu **3/6**: distinguiu IDs e conferência, mas interpretou mal o pânico
esperado, omitiu referências de fichas e atendeu parcialmente aos limites.
Não é comparação controlada com a Aula 10, cuja nota permanece **3/6**.

Na [Aula 18](aulas/18-medir-custo-conferencia.md), medimos a CLI release com
1, 10 e 100 fichas da mesma captura. O medidor Rust passou em **2 testes**.
Com 15 amostras por carga após aquecimento, as medianas foram **1,563 ms**,
**3,558 ms** e **22,687 ms**. São tempos totais da CLI, incluindo exportação;
não houve otimização, isolamento do custo de hashes ou ganho demonstrado.
Protocolo, cargas e amostras estão em `experimentos/09-custo-conferencia/`.

Na [Aula 19](aulas/19-investigar-conferencias-repetidas.md), instrumentamos as
conferências. **72 testes passaram**. Para 1, 10 e 100 fichas, observamos 2, 11
e 101 conferências de captura, com 41.880, 227.910 e 2.088.210 bytes nas leituras
instrumentadas. As exportações preservaram os hashes da Aula 18. Confirmamos
trabalho repetido, sem medir sua participação no tempo nem otimizar.

Na [Aula 20](aulas/20-reaproveitar-conferencia.md), reutilizamos a conferência
por fonte dentro de uma exportação, sem cache entre comandos. **73 testes
passaram**. As cargas de 1, 10 e 100 fichas passaram a duas conferências cada,
com exportações idênticas. Na comparação local, as medianas de 10 e 100 fichas
caíram de 3,619 e 26,513 ms para 2,044 e 5,997 ms; uma ficha não melhorou.
Esses resultados não garantem ganho em outros ambientes ou cargas.

Na [Aula 21](aulas/21-testar-varias-fontes.md), distribuímos 100 fichas entre
1, 10 e 100 fontes documentais da mesma captura. Observamos **2, 20 e 200
conferências**, confirmando o escopo por fonte. Uma ligação inválida foi recusada
sem exportação; os testes de identidade e equivalência de saída passaram.
Não alteramos Rust nem medimos tempo; as fontes não representam arquivos únicos.

Na [Aula 22](aulas/22-separar-fonte-e-captura.md), separamos a conferência do
documento de captura da ligação e validação da fonte. **74 testes passaram**.
As exportações e contagens das cargas da Aula 21 permaneceram iguais; a ligação
inválida continuou recusada. Não implementamos cache por captura nem medimos tempo.

Na [Aula 23](aulas/23-reaproveitar-captura-entre-fontes.md), reutilizamos o
documento de captura entre fontes dentro da exportação, preservando verificações
individuais. **76 testes passaram**. Com 1, 10 e 100 fontes, as conferências
completas passaram de 2, 20 e 200 para **2, 11 e 101**; exportações idênticas.
Não medimos tempo nem implementamos compartilhamento na validação inicial.

Na [Aula 24](aulas/24-contrato-dos-engines.md), documentamos o
[contrato v1 dos engines](contratos/engines-v1.md): responsabilidades, entradas,
saídas, falhas e 12 critérios de conclusão (3 implementados, 2 parciais e 7
pendentes). Não implementamos a integração nem executamos testes nesta aula.

## Ponto de retomada

A **Aula 24 está concluída** e a [Aula 25 — Primeira comunicação Rust–Ollama](aulas/25-primeira-comunicacao-ollama.md)
está **em andamento**. O preparador Rust `prepare_ollama` monta a requisição sem
enviar nem reconferir evidências; **3 testes passaram**. Seguimos com Windows
nativo: o usuário informou Rust/Cargo 1.98.1, Ollama 0.34.2 em execução local
e os modelos renderer-analyst:latest e qwen2.5-coder:14b disponíveis. O próximo
passo é levar o código do Codespaces para uma cópia no Windows. Depois
implementaremos o transporte HTTP e testaremos uma chamada real. O capturador
ainda é Unix; os engines ainda não estão fechados.

O envio continua manual até a integração funcionar. Qwen é o modelo atual;
DeepSeek permanece uma possibilidade sujeita a avaliação. As notas das aulas
10 e 17 permanecem **3/6**. Seguimos os engines primeiro, sem comprovação
retroativa de RUN_VECTOR_1 ou promessa de snapshot atômico.

## Roteiro acordado — engines antes dos objetivos

Manter o desenvolvimento em Rust e o **Ollama como meio de execução do modelo**.
O modelo atual é o Qwen; o usuário pode optar por substituí-lo por **DeepSeek
via Ollama**, buscando melhor desempenho em tarefas de Rust e análise de
performance. Essa vantagem é uma hipótese a avaliar com tarefas, critérios e
configurações registrados, não um resultado já demonstrado pelo laboratório.
Não houve troca de modelo. Não introduzir outros serviços de IA como dependência.

O Graph Engine organiza e confere evidências; o LLM Engine consulta o modelo
configurado no Ollama e preserva suas explicações como respostas sujeitas a
avaliação. Planejar a integração com seleção explícita do modelo, registrando
sua identificação e configuração em cada consulta para permitir uma futura
troca sem reescrever o Graph Engine. O modelo não substitui validação
determinística, testes ou benchmarks.

Seguimos conceito → implementação → teste → medição → explicação. Não refazer
etapas concluídas. Atualizar o ponto de retomada e apresentar a próxima aula
a cada fechamento.

| Etapa | Trabalho previsto | Condição para avançar |
| --- | --- | --- |
| Aula 22 — concluída | Separar verificações da fonte documental e da captura compartilhada. | Responsabilidades separadas, identidade e limites propostos; 74 testes passaram. |
| Aula 23 — concluída | Reaproveitar a captura entre fontes na mesma exportação. | 76 testes passaram; exportações preservadas e contagens reduzidas. |
| Aula 24 — concluída | Consolidar contrato e critérios de conclusão dos engines. | Contrato v1 documentado, integração ainda pendente. |
| Aula 25 | Primeira comunicação Rust–Ollama. | Configuração explícita, adaptador testado e chamada registrada. |
| Aulas seguintes — Graph Engine | Concluir as etapas decorrentes dessa investigação e explicitar contrato, lacunas e critérios de conclusão dos engines. | Conferências e limites testados; contrato e critérios verificáveis documentados. |
| Integração com Ollama | Implementar comunicação Rust → Ollama → modelo configurado (Qwen inicialmente), considerando Codespaces e Windows. | Consulta e resposta reais registradas; conexão e modelo explícitos. |
| Validação integrada | Testar o ciclo completo e falhas de comunicação, resposta e validação. | Evidências conferidas → consulta → resposta preservada → avaliação reproduzível. |
| Aulas seguintes, se necessárias | Fechar as lacunas encontradas no Graph Engine e no LLM Engine. | Critérios de conclusão atendidos e limites registrados. |
| Após fechar os dois engines | Aplicar o conjunto ao desempenho do Ray Tracer em CPU. | Profiling, testes e benchmarks sustentam cada melhoria proposta. |
| Depois, seguindo a sequência acordada | Criar cenas por linguagem natural. | Descrição → modelo via Ollama → estrutura validada → objetos do renderer → renderização. |

A numeração é uma previsão de escopo: podemos dividir aulas para aprender com
calma. **A Aula 25 não marca uma mudança automática para os objetivos**; a
transição depende do fechamento dos dois engines, mesmo que exija mais aulas.

### O que significa fechar os engines

Fechar significa ter uma versão funcional, integrada, testada e documentada para
o escopo acordado, com limites conhecidos. Não significa resolver toda evolução
futura nem garantir que o modelo sempre explique corretamente. Detalharemos
os critérios e casos de aceitação ao longo das próximas aulas, preservando
a continuidade da Aula 21, a partir desta base:

- **Graph Engine:** registrar e conferir fontes, fichas, execuções e suas
  relações; selecionar/exportar evidências com identidade e limites; detectar
  inconsistências previstas e demonstrar o comportamento com testes.
- **LLM Engine:** enviar consultas ao modelo configurado no Ollama, preservar entradas,
  respostas e configuração informada, tratar falhas e respostas incompletas,
  e avaliar explicações por critérios definidos antes das respostas.
- **Integração:** executar o ciclo completo em casos reais e em casos de falha,
  distinguindo conferência, resultado do comando e interpretação do modelo;
  registrar medições pertinentes sem afirmar ganhos não demonstrados.

Ainda não há banco de grafos nem extração automática de afirmações. A necessidade
dessas capacidades deve decorrer do contrato; o nome Graph Engine não exige,
por si só, acrescentá-las antes de avançar. A integração automática com o Ollama
continua pendente: o envio foi manual até a Aula 21.

### Objetivos finais preservados

1. **Otimizar a renderização em CPU, sem RTX/GPU:** escolher cargas reais, fazer
   profiling, identificar gargalos, implementar melhorias e comparar correção e
   desempenho. Os ganhos medidos até aqui são do Bibliotecário, não do renderer.
2. **Criar cenas por linguagem natural:** o modelo via Ollama produz uma descrição estruturada;
   o programa valida estrutura e regras do domínio antes de construir World,
   câmera, objetos, materiais e luzes e renderizar. Esse fluxo ainda não existe.

Podemos trabalhar em partes desses objetivos durante as aulas dos engines para
validá-los com exemplos reais. Isso não altera a prioridade: primeiro fechar
os engines, depois concentrar o desenvolvimento nos objetivos finais.
