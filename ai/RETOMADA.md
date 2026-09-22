# Ponto de retomada — 22/09/2026

## Pedido para o próximo chat

Leia este arquivo e os relatórios vinculados antes de alterar código.
Continuar a otimização do renderer em CPU com cenas maiores, usando o ciclo
Bibliotecário → Graph Engine → LLM Engine → avaliação. O usuário encerrou a
sessão e pediu salvar este ponto e commitar tudo. Não há pedido de push.

## Objetivo e ordem acordada

1. Medir e otimizar o ray tracer em CPU, sem GPU/RTX.
2. Manter evidências de código, testes e benchmark distintas; conferir antes de
   consultar o LLM e avaliar a resposta pela rubrica definida previamente.
3. Completar SceneSpec → validação → construção antes de gerar cenas por linguagem natural.
4. Isolar o Bibliotecário dentro do repo, provar o fluxo completo em outro domínio
   e só então extrair para um projeto GitHub independente.

Trabalhar em passos: conceito → implementação mínima → regressão → medição
antes/depois → explicação. Salvar a baseline antes de cada otimização.

## Código e resultado de CPU

- `src/bibliotecario.rs`, `src/core/`, `src/adapters/` e `src/extractors/`:
  protótipo de fatos, fontes, seleção, validação e exportação; adapters renderer/xadrez.
- `src/scene.rs`: schema com validação parcial; ainda sem construção validada do World.
- `src/camera.rs` e `src/bin/bench_render.rs`: benchmark de Camera::render;
  clone preparatório e descarte do canvas fora do cronômetro, sem I/O medido.
- `src/world.rs`: color_at, shade_hit, is_shadowed, reflected_color e
  refracted_color recebem &self. Removidos clones de World por pixel e no shading.
- Regressão: digest da imagem 24×16 com reflexão/transparência, cores quantizadas
  por EPSILON. Não é prova de equivalência universal ou bit a bit.
- Medição release, quatro threads, 64×64: 35 amostras por versão, ordem alternada,
  três warmups/cinco amostras por execução. Medianas 4.876.415 → 4.132.936 ns:
  redução observada de 15,25%, limitada a essa cena.
- Cena de benchmark atual tem câmera dentro das esferas. Ainda não há perfil CPU
  que comprove hotspot dominante. Mutex por pixel, par_bridge e inversão por raio
  são candidatos a investigar, não conclusões.

Leia [experimento CPU](experimentos/20-render-cpu/README.md), `comparison.json`
e `optimization.patch` nessa pasta. O patch permite recuperar a versão anterior
em cópia isolada, preservando o harness. Não depender dos binários em /tmp.

## Ciclo real de evidências já exercitado

Executáveis existentes:
- validate_evidence: Graph Engine, conferência e exportação;
- explain_evidence: coordenador que preserva rubrica, reconfere e envia;
- ollama_common/client.rs: transporte do LLM Engine;
- prepare_ollama/send_ollama: preparação/envio separados, sem reconferência própria no envio.

O Ollama está no Windows do usuário, não neste workspace remoto.
Modelo informado: renderer-analyst:latest. Saída de ollama show preservada em
`experimentos/20-render-cpu/consulta-02/configuracao-relatada-windows.txt`:
num_ctx=4096, num_predict=1024, seed=42, temperature=0.
Não inferir que localhost aqui alcança o Windows.

### Consulta 1

ZIP original: `experimentos/resultado-render-cpu.zip`.
Dossiê: `experimentos/20-render-cpu/dossie/`.
Avaliação: [avaliacao-windows-01](experimentos/20-render-cpu/avaliacao-windows-01/README.md).
37 verificações de integridade passaram; resposta com três critérios reprovados
 e três parciais. Inventou F_TIME_REDUCTION e confundiu regressão com performance.

### Consulta 2 — último trabalho concluído

ZIP original: `experimentos/RENDER_CPU_V2_29ef6dfc096641cea7d45e0926d9bd0d.zip`.
Preparação: `experimentos/20-render-cpu/consulta-02/`.
Avaliação: [avaliacao-windows-02](experimentos/20-render-cpu/avaliacao-windows-02/README.md).
37 verificações passaram. Quatro critérios aprovados, um parcial, nenhum reprovado.
Acertou referências, cronômetro, protocolo e números. Ressalva: negou genericamente
informações sobre execução/validação, embora existam registros e recálculo;
a ausência é de autenticação da execução histórica. Excedeu também as 200 palavras
pedidas (241 por espaços), anotado fora da rubrica, sem criar critério retroativo.

Consulta menor: 9.077 bytes contra 36.889 anteriormente. Seleção, janela, pergunta
e rubrica mudaram. Não concluir truncamento anterior ou superioridade geral com
essa comparação. Os dois ZIPs foram avaliados; não solicitar seu envio novamente.

`conferencia.json` é verificação determinística; `avaliacao.json` é julgamento
posterior pelo assistente. Não reescrever os registros originais com status pending.
Os scripts de conferência comparam fontes atuais: alterações futuras podem fazê-los
falhar sem significar corrupção dos registros históricos. Preservar os snapshots.

## Pendências reais de arquitetura

- BibliotecarioFact ainda conhece adapters pelas funções from_*; core só reexporta.
- Extractors apenas constroem fatos fornecidos; não extraem evidência automaticamente.
- EvidenceBundle não confere todos os vínculos fonte/fato/seleção e limites de linhas.
- SceneSpec não confere tipos suportados, transformações finitas/invertíveis ou FOV < pi.
- Xadrez só prova conversão; falta fluxo completo no segundo domínio.
- Os engines existentes nos executáveis não estão integralmente migrados ao núcleo novo.

A Aula 26 documenta a direção, mas não deve ser tomada como prova de isolamento
completo ou de todos os critérios de arquitetura concluídos.

## Próxima ação recomendada

Preparar cena externa maior de benchmark (e depois caso com grupos/reflexão),
preservando a cena antiga. Definir regressão visual e baseline release com threads
fixas antes de alterar o render. Medir/investigar o custo dominante e realizar
uma alteração por experimento. Usar perguntas pequenas ao LLM com rubrica prévia;
respostas sugerem hipóteses, testes e medições decidem correção/desempenho.
Não iniciar nova otimização nem extração de repo automaticamente só por ler esta nota.

## Verificações de referência

Suíte completa executada nesta sessão: 327 aprovados, zero falhas, dois ignorados;
biblioteca: 211 aprovados. Testes HTTP locais precisam da permissão de cargo test
fora do sandbox. O capturador Unix não compila nativamente no Windows; compilar
apenas os executáveis indicados pelos scripts de consulta naquele sistema.

```bash
cargo test --lib
python3 ai/experimentos/20-render-cpu/dossie/conferir_amostras.py
python3 ai/experimentos/20-render-cpu/avaliacao-windows-01/conferir.py
python3 ai/experimentos/20-render-cpu/avaliacao-windows-02/conferir.py
cargo build --release --bin bench_render
RAYON_NUM_THREADS=4 target/release/bench_render
```

Não comparar medições novas diretamente com debug ou tratar variação ambiental
como ganho. Hashes verificam correspondência de bytes, não autenticam execução.
