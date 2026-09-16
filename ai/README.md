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

## Ponto de retomada

O exemplo do mutex está concluído para esta etapa. A próxima aula usará
`src/camera.rs` como primeira obra real: carregar o arquivo e criar uma ficha
manual sobre uma operação, conferindo o trecho e seu contexto.

Continuaremos ponto a ponto, com a analogia do Bibliotecário: obras, fichas,
trechos e notas de revisão. Ainda faltam versionamento das fontes, extração
automática e integração com o Ollama; cada passo será estudado separadamente.
