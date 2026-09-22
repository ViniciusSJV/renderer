# Aula 26 — Separar o núcleo do Bibliotecário dos adapters do domínio

## Objetivo

Fechar a etapa de arquitetura do Bibliotecário: separar o núcleo genérico das
responsabilidades específicas do renderer, mantendo a validação e a exportação
estáveis e sem forçar uma extração imediata para outro repositório.

Esta aula conclui a revisão do ciclo de integração com Ollama e passa para o
próximo passo operacional: tornar o núcleo reutilizável e deixar o renderer como
um adaptador de domínio, não como o centro da lógica do Bibliotecário.

## Contexto e arquitetura

A partir da Aula 24, o contrato dos engines separa:

- Graph Engine: valida origem, seleção, contexto, hashes e integridade da
  consulta antes de qualquer envio.
- LLM Engine: preserva a resposta e não eleva a explicação recebida ao status de
  evidência verdadeira.
- Bibliotecário: organiza fatos, evidências, contexto, avaliação e exportação em
  um fluxo reutilizável.

A evolução desta aula toma o caminho seguinte:

```text
núcleo genérico
    ├─ fonte
    ├─ evidência
    ├─ fato
    ├─ seleção
    ├─ validação
    └─ exportação

        ↓ adaptadores do domínio
    renderer
    chess
    futuro: dominio-x
```

A regra central é simples: o núcleo não sabe que o projeto é um ray tracer. Ele
só conhece estruturas universais de trabalho de evidência. O renderer, por sua
vez, conhece câmeras, raios, esferas, materiais e cenas. Essa separação reduz
acoplamento e permite que o mesmo fluxo de conferência e exportação sirva a
outros domínios.

## Conceitos

### Núcleo genérico

O núcleo deve definir estruturas estáveis para:

- `SourceRef`: referência a um documento ou trecho de código
- `Fact`: afirmação atômica, com identidade, origem e linha
- `Selection`: ordem da seleção e contexto de janela
- `ValidationOutcome`: resultado da conferência com motivo

Essas estruturas não devem depender de `sphere`, `camera`, `canvas`, `triangle`
ou qualquer outra classe do renderer.

### Adaptador do renderer

O adaptador converte o domínio específico em um fato genérico do Bibliotecário.
Exemplo de relação:

```rust
RendererArtifact {
    id: "F-1",
    statement: "ray hits sphere",
    source_id: "source-1",
    line: 12,
}
```

fica equivalente a:

```rust
BibliotecarioFact {
    id: "F-1",
    statement: "ray hits sphere",
    source_id: "source-1",
    line: 12,
}
```

O ponto é que o adaptador faz a tradução, e não o núcleo do Bibliotecário.

### Adaptador de outro domínio

Uma segunda prova de reutilização é necessária antes de qualquer extração para
um repositório separado. O exemplo mais direto e didático é um domínio de xadrez
ou um outro domínio documental. Isso mostra que o núcleo foi projetado para o
problema de *evidência e seleção*, e não para o renderer em si.

## Implementação praticada

A estrutura proposta foi criada internamente no repositório atual, sem mover os
arquivos para um segundo projeto. O primeiro passo foi:

1. manter a árvore do renderer intacta;
2. criar o módulo de núcleo `bibliotecario`;
3. criar o módulo `adapters` com submódulos específicos;
4. testá-lo primeiro com o renderer e depois com um segundo domínio.

O contrato de prova foi de duas partes:

- converter um artefato do renderer para `BibliotecarioFact`;
- converter um artefato de xadrez para a mesma estrutura;
- verificar que a seleção preserva ordem e contexto configurable.

Esse passo é importante porque mostra que o problema central é estrutural,
reutilizável e verificável, e não uma dependência acidental do código do ray tracer.

## Validação

A validação desta aula foi executada pela suíte relevante do projeto:

```bash
cd /workspaces/renderer-local && cargo test bibliotecario -- --nocapture
```

Resultado observado: 3 testes do núcleo passaram, 0 falharam.

Também foi validado o impacto no projeto atual:

```bash
cd /workspaces/renderer-local && cargo test --lib
```

Resultado observado: 203 testes passaram, 0 falharam.

Esses dados são a prova de que a separação inicial foi feita sem regressão da
biblioteca do renderer.

## Limites e regra de extração

A simples criação de módulos internos não significa que o projeto deva ser
extraído imediatamente para outro repositório. A regra correta é esta:

- primeiro: isolar o núcleo genérico;
- segundo: provar reutilização em um segundo domínio;
- terceiro: somente então considerar extração para outro projeto.

Essa regra evita a armadilha de “extrair cedo” e criar um repositório sem
prova de que o núcleo realmente é genérico.

Além disso, o JSON exportado continua sendo o contrato de transição. O núcleo
não substitui o renderer, nem o renderer substitui o núcleo. O formato estável
permite evoluir cada camada sem quebrar a outra.

## Resultado da aula

O fechamento do pipeline Alice/Graph Engine/LLM Engine foi validado na prática;
agora a arquitetura do Bibliotecário avança para a segunda etapa: o núcleo
passa a ser explícito e reutilizável, e o renderer fica como um adaptador de
entrada para esse núcleo.

O ponto relevante não é a criação de um novo repositório, e sim a capacidade de
reusar a mesma lógica para mais de um domínio sem acoplar o problema à renderização.

## Prática seguinte

A próxima etapa é expandir esse núcleo para a estrutura completa do fluxo de
trabalho do Bibliotecário:

- `source`
- `evidence`
- `facts`
- `selection`
- `validation`
- `export`

Depois, o código de adapters pode crescer de forma ordenada e o caso do
renderer continua como prova inicial, enquanto um segundo domínio (como xadrez)
confirma a intenção de reutilização.
