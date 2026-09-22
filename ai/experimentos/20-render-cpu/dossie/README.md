# Dossiê de performance para Graph Engine e LLM Engine

Este pacote aplica o fluxo existente ao experimento de remoção de clones de
World. Não muda o núcleo genérico e não apresenta o LLM como autor da otimização.

## Estado observado

- 11 fichas com linhas, fontes e SHA-256 em `evidencias.json`.
- `conferir_amostras.py` executado: 35 amostras por versão, medianas
  4.876.415 e 4.132.936 ns; redução de 15,24642591%.
- Teste `render_preserves_complete_reflective_scene` reexecutado: um aprovado;
  saída em `regressao.txt`. Não é uma captura autenticada versão 2.
- Graph Engine (`validate_evidence`) executado com sucesso; consulta e origem
  preservadas em `pacote-local`, diagnóstico em `conferencia-local.txt`.
- Rubrica com seis critérios escrita antes de qualquer resposta.
- Chamada real ao Ollama e avaliação: **pendentes de execução no Windows**.

A exportação local não vincula a rubrica. O coordenador `explain_evidence`
preserva a rubrica antes da conferência/envio no Windows. Os critérios esperados
não são enviados no prompt. Não enviar diretamente a consulta antiga:
o script usa o coordenador para reconferir as fontes antes da chamada.

## Executar no Windows

Pré-requisitos: Rust/Cargo com toolchain Windows funcional; Ollama em execução
na mesma máquina, com `renderer-analyst:latest` disponível. A instalação do
modelo não é feita por este pacote.

Extraia o ZIP em um diretório novo. Ele contém uma cópia dos fontes atuais,
Cargo.toml/Cargo.lock, testes e arquivos de `ai`, preservando os bytes usados
nos hashes. Não inclui `.git`, `target`, credenciais ou instalação do Ollama.
Não substitua esses fontes por um clone antigo: há alterações locais ainda
não commitadas. Conversão de LF para CRLF também altera os hashes.

Na raiz extraída, execute no PowerShell:

```powershell
ollama list
& .\ai\experimentos\20-render-cpu\dossie\executar-windows.ps1
```

Para selecionar outro modelo já instalado:

```powershell
& .\ai\experimentos\20-render-cpu\dossie\executar-windows.ps1 -Model "nome:tag"
```

O script compila os dois executáveis necessários e cria um diretório
`RENDER_CPU_<id>` novo a cada tentativa. Não executa sugestões do modelo.
Examine o diretório `attempt` e seus registros mesmo quando houver falha.
Endpoint padrão: `http://127.0.0.1:11434/api/generate`, timeout 120 segundos.

O script PowerShell foi revisado, mas não executado em Windows nesta etapa.
Compilação dos executáveis e conferência do pacote foram realizadas em Linux.

## Retorno e avaliação

Traga o diretório `RENDER_CPU_<id>` completo após a execução. A avaliação deve
usar a rubrica preservada na tentativa, com um resultado por critério,
trecho da resposta que sustenta o julgamento e explicação de lacunas ou erros.
Transporte bem-sucedido não significa resposta correta. Não preencher uma
avaliação como aprovada antes de ler a resposta real.

## Limites da evidência

Os hashes e as linhas conferem os arquivos atuais. A coleta histórica do
benchmark não ganhou proveniência de execução retroativa. O recálculo é uma
conferência aritmética. A aprovação do teste cobre uma cena e cores quantizadas,
não equivalência universal. A hipótese do próximo hotspot exige novo experimento.
