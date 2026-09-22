# Consulta 02 — interpretação da medição

A saída de `ollama show` fornecida pelo usuário está preservada em
`configuracao-relatada-windows.txt`: num_ctx 4096, num_predict 1024,
seed 42, temperature 0. É um relato capturado após a primeira tentativa;
não é telemetria do contexto efetivamente usado naquela requisição.

A segunda consulta seleciona F_TIMER, F_PROTOCOL e F_MEDIANS do mesmo dossiê,
com contexto de seis linhas e pergunta limitada à medição. O modelo, endpoint
local e parâmetros declarados permanecem os mesmos; o cliente não envia
opções para substituí-los. Há cinco critérios prévios na nova rubrica.

A conferência local passou e a consulta exportada tem 9.077 bytes, contra
36.889 bytes da primeira consulta. Bytes não são tokens nem garantem que a
entrada caiba no contexto. Mudamos seleção, janela e pergunta: este é um teste
mais restrito de interpretação, não um experimento que isola a causa dos erros
anteriores. Uma resposta melhor não provará truncamento da consulta anterior.

## Windows

Extraia `renderer-consulta-02.zip` dentro da raiz já existente de
`renderer-cpu-dossie-windows` (onde está Cargo.toml). O ZIP acrescenta apenas
`ai/experimentos/20-render-cpu/consulta-02`; depende do pacote anterior.

Na raiz do projeto, na sessão PowerShell já autorizada a executar scripts:

```powershell
& .\ai\experimentos\20-render-cpu\consulta-02\executar-windows.ps1
```

O script recompila os executáveis, reconfere o dossiê e chama o Ollama. Após
sucesso, compacta o diretório da tentativa e mostra o caminho do ZIP para
retorno. Em falha, preserva os registros e informa seu diretório.
A primeira tentativa não é alterada. O script Windows não foi executado aqui.

Estado: pacote preparado e Graph Engine conferido localmente; execução real e
avaliação da segunda resposta pendentes. Não há nova otimização do renderer.
