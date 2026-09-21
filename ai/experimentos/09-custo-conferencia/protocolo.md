# Protocolo anterior à medição — Aula 18

Objetivo: estabelecer uma linha de base da CLI de exportação do Bibliotecário,
sem otimização nesta aula. Hipótese: aumentar fichas selecionadas da mesma fonte
pode aumentar o custo, pois a implementação repete conferências por seleção.
Isso é uma hipótese sobre contribuição de custo, não uma conclusão causal.

- Binário release compilado antes das medições; invocação direta, sem Cargo.
- Cargas sintéticas de 1, 10 e 100 fichas com IDs distintos, copiando a primeira
  ficha do dossiê da Aula 16 e mantendo uma única fonte e captura. São repetições
  de conteúdo para medir escala, não novas evidências independentes.
- Exportar todas as fichas, raio de contexto 1, destino novo a cada processo.
- Três rodadas de aquecimento e 15 rodadas medidas por carga. Alternar a ordem
  das três cargas por rotação a cada rodada; registrar a ordem de cada amostra.
- Medir tempo de parede com Instant, de spawn até o término do processo. Inclui
  inicialização, leitura, hashes, serialização e escrita. Preparação das cargas,
  leitura/conferência do resultado e remoção da exportação ficam fora do tempo.
- stdout descartado, stderr preservado em erro. Qualquer comando não zero,
  exportação inválida ou resultado não determinístico interrompe o ensaio.
- Registrar amostras em nanossegundos, mínimo, mediana e máximo, bytes de entrada
  e saída e hashes. Não medir alocações, bytes efetivamente lidos, CPU, tokens,
  cache frio, custo isolado dos hashes ou tempo do renderer/Qwen.
- Ambiente compartilhado do Codespaces, sem isolamento de CPU ou limpeza de
  cache. Não executar outros testes em paralelo à medição. Aquecimento favorece
  caches; interferência externa ainda pode ocorrer.
- Uma ficha usa o formato individual; 10 e 100 usam exportação múltipla. Esse
  efeito de formato faz parte da CLI medida e limita comparações de escala.
- Hashes de fontes e executável serão registrados; não comprovam o processo de
  compilação. Este ensaio não compara versões nem demonstra ganho de desempenho.
