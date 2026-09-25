# Workflow completo: renderer, Librarian e Ollama

A única trilha executável está no [PLAYME](PLAYME.ps1). Este fluxograma mostra o fluxo completo entre o renderer, o Librarian e o Ollama, desde a seleção e extração das fontes até a preparação da consulta, envio ao modelo, preservação da resposta e validação final dos artefatos.


```mermaid
flowchart TD

    %% =========================
    %% RENDERER - CONFIGURAÇÃO
    %% =========================

    subgraph R1["RENDERER / PLAYME"]
        A["Escolhe e configura fontes"]
        A1["renderer-sources.json"]
        A2["renderer-lexicon.json"]

        A --> A1
        A --> A2
    end

    %% =========================
    %% LIBRARIAN
    %% =========================

    subgraph L["LIBRARIAN"]
        B["generate"]
        B1["Extrai fontes"]
        B2["Preserva snapshots"]
        B3["Extrai chunks / símbolos"]

        C["verify"]
        C1["Confere edição contra fontes"]

        D["graph"]
        D1["Deriva relações"]
        D2["Exporta graph.json"]

        V["verify-graph"]
        V1["Confere grafo contra edição"]

        S["search"]

        S1["Normaliza pergunta"]
        S2["Expande léxico"]
        S3["Ranqueia símbolos"]
        S4["Expande vizinhança do grafo"]

        B --> B1
        B --> B2
        B --> B3

        B --> C
        C --> C1

        C --> D
        D --> D1
        D --> D2

        D --> V
        V --> V1

        V --> S

        S --> S1
        S1 --> S2
        S2 --> S3
        S3 --> S4
    end

    A1 --> B
    A2 --> S

    %% =========================
    %% RENDERER - RESULTADOS
    %% =========================

    subgraph R2["RENDERER / PLAYME"]
        G["Inspeciona resultados"]

        G1["lexicon-trace.json"]
        G2["retrieved-excerpts.txt"]
        G3["graph-trace.json"]

        H["Seleciona hits[0] = E1"]

        H1["Localiza chunk"]
        H2["Localiza source"]
        H3["Lê snapshot preservado"]
        H4["Reconstrói E1"]

        X["Compara busca com snapshot"]

        X1["path"]
        X2["source SHA256"]
        X3["chunk SHA256"]
        X4["linhas"]
        X5["bytes / excerpt"]

        P["Cria query"]

        P1["question"]
        P2["evidence = E1"]
        P3["hashes"]
        P4["instructions"]

        Criterios["Define critérios prévios"]

        G --> G1
        G --> G2
        G --> G3

        G --> H

        H --> H1
        H1 --> H2
        H2 --> H3
        H3 --> H4

        H4 --> X

        X --> X1
        X --> X2
        X --> X3
        X --> X4
        X --> X5

        X --> P

        P --> P1
        P --> P2
        P --> P3
        P --> P4

        P --> Criterios
    end

    S4 --> G

    %% =========================
    %% PREPARE OLLAMA
    %% =========================

    subgraph PREP["prepare_ollama"]
        PO["Recebe query-ollama.json"]
        PO1["Monta corpo exato da API"]
        PO2["Gera request-ollama.json"]

        PO --> PO1
        PO1 --> PO2
    end

    Criterios --> PO

    %% =========================
    %% OLLAMA
    %% =========================

    subgraph O["OLLAMA"]
        OS["ollama serve"]

        OM["Modelo"]
        OM1["ollama pull Qwen"]
        OM2["Lê Modelfile"]
        OM3["ollama create renderer-analyst"]

        API["POST /api/generate"]
        Q["Qwen gera resposta"]

        OS --> OM
        OM --> OM1
        OM1 --> OM2
        OM2 --> OM3

        OM3 --> API
        API --> Q
    end

    PO2 --> API

    %% =========================
    %% SEND OLLAMA
    %% =========================

    subgraph SEND["send_ollama"]
        SO["Envia consulta"]

        SO1["Preserva query.json"]
        SO2["Preserva request.json"]
        SO3["Preserva response.bin"]
        SO4["Preserva response.txt"]
        SO5["Preserva result.json"]

        SO6["Calcula / registra SHA256"]

        SO --> SO1
        SO --> SO2
        SO --> SO3
        SO --> SO4
        SO --> SO5

        SO5 --> SO6
    end

    Q --> SO

    %% =========================
    %% PLAYME - FINALIZAÇÃO
    %% =========================

    subgraph R3["RENDERER / PLAYME"]
        K["Reconfere hashes"]

        K1["query SHA256"]
        K2["request SHA256"]
        K3["response SHA256"]
        K4["text SHA256"]

        EVAL["Cria evaluation.md"]

        PEND["Avaliação semântica<br/>PENDENTE"]

        K --> K1
        K --> K2
        K --> K3
        K --> K4

        K --> EVAL
        EVAL --> PEND
    end

    SO6 --> K
```

# Fluxo de evidências do PLAYME: fontes, grafo, busca e consulta ao Ollama

```mermaid
flowchart TD

    PLAYME["PLAYME.ps1<br/>Execução completa da consulta"]

    %% =========================================================
    %% CONFIGURAÇÃO / ENTRADA
    %% =========================================================

    subgraph INPUT["1. Entrada e configuração"]
        SOURCESCFG["sources-config.json<br/>Quais fontes do projeto serão analisadas"]
        LEXICON["lexicon.json<br/>Vocabulário e expansões"]
        QUESTION["question.txt<br/>Pergunta original"]
    end

    PLAYME --> SOURCESCFG
    PLAYME --> LEXICON
    PLAYME --> QUESTION

    %% =========================================================
    %% EDIÇÃO DO LIBRARIAN
    %% =========================================================

    subgraph EDITION["2. edition/ — evidências preservadas pelo Librarian"]

        subgraph SNAP["snapshots/"]
            SNAPSHOT["Snapshots<br/>Bytes originais das fontes<br/>endereçados por SHA-256"]
        end

        SOURCES["sources.jsonl<br/>Source<br/>path, sha256, byte_len, language"]

        SYMBOLS["symbols.jsonl<br/>Symbol<br/>name, qualified_name,<br/>kind, chunk_id"]

        CHUNKS["chunks.jsonl<br/>Chunk<br/>start/end byte,<br/>start/end line, sha256"]

        DIAG["diagnostics.json<br/>Falhas ou limitações<br/>da extração"]

        MANIFEST["manifest.json<br/>Versão do schema/extractor,<br/>hashes e contagens"]
    end

    SOURCESCFG -->|"generate"| SOURCES

    SOURCES --> SNAPSHOT
    SOURCES --> SYMBOLS
    SOURCES --> CHUNKS

    SYMBOLS -->|"chunk_id"| CHUNKS
    CHUNKS -->|"source_id"| SOURCES

    SOURCES --> MANIFEST
    SYMBOLS --> MANIFEST
    CHUNKS --> MANIFEST
    DIAG --> MANIFEST

    SNAPSHOT -->|"fonte verificável"| SOURCES
    SNAPSHOT -->|"re-extração / conferência"| SYMBOLS
    SNAPSHOT -->|"re-extração / conferência"| CHUNKS

    %% =========================================================
    %% GRAFO COMPLETO
    %% =========================================================

    subgraph GRAPH["3. Grafo estrutural completo"]

        GRAPHJSON["graph.json<br/>Nodes + Edges + Evidence"]

        NODE_SOURCE["Node: Source<br/>ex.: src/camera.rs"]
        NODE_SYMBOL["Node: Symbol<br/>ex.: impl Camera::ray_from_pixel"]
        NODE_OCC["Node: Occurrence<br/>ex.: call_observed,<br/>type_reference"]

        EDGE_DECL["Edge: declares"]
        EDGE_CONTAINS["Edge: contains"]
        EDGE_CALL["Edge: call_observed"]
        EDGE_TYPE["Edge: type_reference"]
        EDGE_CAND["Edge: name_candidate"]

        EVIDENCE["Evidence da aresta<br/>path, source_sha256,<br/>bytes, linhas,<br/>excerpt_sha256"]
    end

    SOURCES --> NODE_SOURCE
    SYMBOLS --> NODE_SYMBOL
    CHUNKS --> EVIDENCE
    SNAPSHOT --> EVIDENCE

    NODE_SOURCE --> EDGE_DECL
    EDGE_DECL --> NODE_SYMBOL

    NODE_SYMBOL --> EDGE_CONTAINS
    EDGE_CONTAINS --> NODE_SYMBOL

    NODE_SYMBOL --> EDGE_CALL
    EDGE_CALL --> NODE_OCC

    NODE_SYMBOL --> EDGE_TYPE
    EDGE_TYPE --> NODE_OCC

    NODE_OCC --> EDGE_CAND
    EDGE_CAND --> NODE_SYMBOL

    NODE_SOURCE --> GRAPHJSON
    NODE_SYMBOL --> GRAPHJSON
    NODE_OCC --> GRAPHJSON
    EDGE_DECL --> GRAPHJSON
    EDGE_CONTAINS --> GRAPHJSON
    EDGE_CALL --> GRAPHJSON
    EDGE_TYPE --> GRAPHJSON
    EDGE_CAND --> GRAPHJSON
    EVIDENCE --> GRAPHJSON

    %% =========================================================
    %% BUSCA
    %% =========================================================

    subgraph SEARCH["4. Busca e recuperação"]

        SEARCHJSON["search.json<br/>Resultado completo da busca"]

        LEXTRACE["lexicon-trace.json<br/>Termos normalizados,<br/>expansões e matches"]

        EXCERPTS["retrieved-excerpts.txt<br/>Trechos candidatos encontrados"]

        GRAPHTRACE["graph-trace.json<br/>Subgrafo recuperado<br/>depth / max_nodes / max_edges"]

        E1["E1<br/>hits[0]<br/>Primeiro resultado completo"]
    end

    QUESTION --> SEARCHJSON
    LEXICON --> SEARCHJSON

    SYMBOLS --> SEARCHJSON
    CHUNKS --> SEARCHJSON

    GRAPHJSON -->|"expansão de vizinhança"| SEARCHJSON

    SEARCHJSON --> LEXTRACE
    SEARCHJSON --> EXCERPTS
    SEARCHJSON --> GRAPHTRACE

    SEARCHJSON --> E1

    GRAPHJSON -.->|"grafo completo"| GRAPHTRACE
    GRAPHTRACE -.->|"somente subgrafo da busca"| E1

    %% =========================================================
    %% VERIFICAÇÃO DE E1
    %% =========================================================

    subgraph VERIFY["5. Conferência da evidência selecionada"]

        REBUILD["Reconstruir E1<br/>a partir do snapshot"]

        CHECK["Comparar<br/>path<br/>linhas<br/>source SHA-256<br/>chunk SHA-256<br/>bytes"]
    end

    E1 --> REBUILD
    CHUNKS --> REBUILD
    SOURCES --> REBUILD
    SNAPSHOT --> REBUILD

    REBUILD --> CHECK

    %% =========================================================
    %% QUERY
    %% =========================================================

    subgraph QUERY["6. Preparação da consulta ao LLM"]

        QUERYJSON["query-ollama.json<br/>question + evidence E1<br/>+ hashes + instructions"]

        PROMPT["prompt.txt<br/>Prompt preservado"]

        CRITERIA["criteria-before.md<br/>Critérios definidos<br/>antes da resposta"]
    end

    QUESTION --> QUERYJSON
    CHECK --> QUERYJSON

    QUERYJSON --> PROMPT
    PLAYME --> CRITERIA

    %% =========================================================
    %% OLLAMA
    %% =========================================================

    subgraph OLLAMA["7. Ollama"]

        SERVER["server.json<br/>Servidor / endpoint / PID"]

        RESPONSE["Resposta do modelo<br/>renderer-analyst / Qwen"]
    end

    PLAYME --> SERVER
    QUERYJSON --> RESPONSE
    SERVER --> RESPONSE

    %% =========================================================
    %% EXECUÇÃO / AUDITORIA
    %% =========================================================

    subgraph AUDIT["8. Auditoria da execução"]

        COMMANDS["commands.jsonl<br/>Comandos executados,<br/>exit codes e timestamps"]

        LOGS["logs/<br/>stdout / stderr"]

        PLAYLOG["playme.log<br/>Log geral da execução"]

        RUN["run.json<br/>Estado final da execução"]

        EVALUATION["Avaliação semântica<br/>PENDENTE"]
    end

    PLAYME --> COMMANDS
    PLAYME --> LOGS
    PLAYME --> PLAYLOG

    RESPONSE --> RUN
    CRITERIA --> EVALUATION
    RESPONSE --> EVALUATION

    RUN --> EVALUATION
```
